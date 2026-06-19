use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use http::HeaderMap;
use http::HeaderValue;
use ontocode_api::ModelsClient;
use ontocode_api::RequestTelemetry;
use ontocode_api::ReqwestTransport;
use ontocode_api::TransportError;
use ontocode_api::auth_header_telemetry;
use ontocode_api::map_api_error;
use ontocode_feedback::FeedbackRequestTags;
use ontocode_feedback::emit_feedback_request_tags_with_auth_env;
use ontocode_login::AuthEnvTelemetry;
use ontocode_login::AuthManager;
use ontocode_login::CodexAuth;
use ontocode_login::collect_auth_env_telemetry;
use ontocode_login::default_client::build_reqwest_client;
use ontocode_model_provider_info::ModelProviderInfo;
use ontocode_models_manager::manager::ModelsEndpointClient;
use ontocode_otel::TelemetryAuthMode;
use ontocode_protocol::error::CodexErr;
use ontocode_protocol::error::Result as CoreResult;
use ontocode_protocol::openai_models::ModelInfo;
use ontocode_response_debug_context::extract_response_debug_context;
use ontocode_response_debug_context::telemetry_transport_error_message;
use serde::Deserialize;
use serde_json::json;
use tokio::time::timeout;

use crate::auth::resolve_provider_auth;

const MODELS_REFRESH_TIMEOUT: Duration = Duration::from_secs(5);
const MODELS_ENDPOINT: &str = "/models";
const ANTHROPIC_DEFAULT_BASE_URL: &str = "https://api.anthropic.com/v1";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const GEMINI_DEFAULT_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

/// Provider-owned OpenAI-compatible `/models` endpoint.
#[derive(Debug)]
pub(crate) struct OpenAiModelsEndpoint {
    provider_info: ModelProviderInfo,
    auth_manager: Option<Arc<AuthManager>>,
}

/// Provider-owned Anthropic model-list endpoint.
#[derive(Debug)]
pub(crate) struct AnthropicModelsEndpoint {
    provider_info: ModelProviderInfo,
    api_key_override: Option<String>,
}

/// Provider-owned Gemini model-list endpoint.
#[derive(Debug)]
pub(crate) struct GeminiModelsEndpoint {
    provider_info: ModelProviderInfo,
    api_key_override: Option<String>,
}

impl OpenAiModelsEndpoint {
    pub(crate) fn new(
        provider_info: ModelProviderInfo,
        auth_manager: Option<Arc<AuthManager>>,
    ) -> Self {
        Self {
            provider_info,
            auth_manager,
        }
    }

    async fn auth(&self) -> Option<CodexAuth> {
        match self.auth_manager.as_ref() {
            Some(auth_manager) => auth_manager.auth().await,
            None => None,
        }
    }

    fn auth_env(&self) -> AuthEnvTelemetry {
        let codex_api_key_env_enabled = self
            .auth_manager
            .as_ref()
            .is_some_and(|auth_manager| auth_manager.codex_api_key_env_enabled());
        collect_auth_env_telemetry(&self.provider_info, codex_api_key_env_enabled)
    }
}

impl AnthropicModelsEndpoint {
    pub(crate) fn new(provider_info: ModelProviderInfo) -> Self {
        Self {
            provider_info,
            api_key_override: None,
        }
    }

    #[cfg(test)]
    fn new_for_testing(provider_info: ModelProviderInfo, api_key: impl Into<String>) -> Self {
        Self {
            provider_info,
            api_key_override: Some(api_key.into()),
        }
    }
}

impl GeminiModelsEndpoint {
    pub(crate) fn new(provider_info: ModelProviderInfo) -> Self {
        Self {
            provider_info,
            api_key_override: None,
        }
    }

    #[cfg(test)]
    fn new_for_testing(provider_info: ModelProviderInfo, api_key: impl Into<String>) -> Self {
        Self {
            provider_info,
            api_key_override: Some(api_key.into()),
        }
    }
}

#[async_trait]
impl ModelsEndpointClient for OpenAiModelsEndpoint {
    fn has_command_auth(&self) -> bool {
        self.provider_info.has_command_auth()
    }

    async fn uses_codex_backend(&self) -> bool {
        self.auth()
            .await
            .as_ref()
            .is_some_and(CodexAuth::uses_codex_backend)
    }

    async fn list_models(
        &self,
        client_version: &str,
    ) -> CoreResult<(Vec<ModelInfo>, Option<String>)> {
        let _timer =
            ontocode_otel::start_global_timer("codex.remote_models.fetch_update.duration_ms", &[]);
        let auth = self.auth().await;
        let auth_mode = auth.as_ref().map(CodexAuth::auth_mode);
        let api_provider = self.provider_info.to_api_provider(auth_mode)?;
        let api_auth = resolve_provider_auth(auth.as_ref(), &self.provider_info)?;
        let transport = ReqwestTransport::new(build_reqwest_client());
        let auth_telemetry = auth_header_telemetry(api_auth.as_ref());
        let request_telemetry: Arc<dyn RequestTelemetry> = Arc::new(ModelsRequestTelemetry {
            auth_mode: auth_mode.map(|mode| TelemetryAuthMode::from(mode).to_string()),
            auth_header_attached: auth_telemetry.attached,
            auth_header_name: auth_telemetry.name,
            auth_env: self.auth_env(),
        });
        let client = ModelsClient::new(transport, api_provider, api_auth)
            .with_telemetry(Some(request_telemetry));

        timeout(
            MODELS_REFRESH_TIMEOUT,
            client.list_models(client_version, HeaderMap::new()),
        )
        .await
        .map_err(|_| CodexErr::Timeout)?
        .map_err(map_api_error)
    }
}

#[async_trait]
impl ModelsEndpointClient for AnthropicModelsEndpoint {
    fn has_command_auth(&self) -> bool {
        false
    }

    async fn uses_codex_backend(&self) -> bool {
        false
    }

    async fn list_models(
        &self,
        _client_version: &str,
    ) -> CoreResult<(Vec<ModelInfo>, Option<String>)> {
        let api_key = provider_api_key(
            &self.provider_info,
            self.api_key_override.as_deref(),
            "Claude",
        )?;
        let url = provider_models_url(&self.provider_info, ANTHROPIC_DEFAULT_BASE_URL)?;
        let mut headers = HeaderMap::new();
        add_configured_headers(&self.provider_info, &mut headers);
        headers.insert(
            "anthropic-version",
            HeaderValue::from_static(ANTHROPIC_VERSION),
        );
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&api_key).map_err(|err| {
                CodexErr::UnsupportedOperation(format!("invalid Claude API key: {err}"))
            })?,
        );
        let response = timeout(
            MODELS_REFRESH_TIMEOUT,
            build_reqwest_client().get(url).headers(headers).send(),
        )
        .await
        .map_err(|_| CodexErr::Timeout)?
        .map_err(|err| {
            CodexErr::UnsupportedOperation(format!("Claude model list request failed: {err}"))
        })?;
        let status = response.status();
        if !status.is_success() {
            return Err(CodexErr::UnsupportedOperation(format!(
                "Claude model list request failed with status {status}"
            )));
        }
        let body: AnthropicModelsResponse = response.json().await.map_err(|err| {
            CodexErr::UnsupportedOperation(format!("failed to parse Claude model list: {err}"))
        })?;
        let mut models = Vec::new();
        for (priority, model) in body.data.into_iter().enumerate() {
            models.push(native_model_info(
                model.id.as_str(),
                model.display_name.as_deref().unwrap_or(model.id.as_str()),
                "Anthropic Claude model through the native Messages API.",
                Some(200_000),
                priority as i32,
            )?);
        }
        Ok((models, None))
    }
}

#[async_trait]
impl ModelsEndpointClient for GeminiModelsEndpoint {
    fn has_command_auth(&self) -> bool {
        false
    }

    async fn uses_codex_backend(&self) -> bool {
        false
    }

    async fn list_models(
        &self,
        _client_version: &str,
    ) -> CoreResult<(Vec<ModelInfo>, Option<String>)> {
        let api_key = provider_api_key(
            &self.provider_info,
            self.api_key_override.as_deref(),
            "Gemini",
        )?;
        let url = provider_models_url(&self.provider_info, GEMINI_DEFAULT_BASE_URL)?;
        let mut headers = HeaderMap::new();
        add_configured_headers(&self.provider_info, &mut headers);
        headers.insert(
            "x-goog-api-key",
            HeaderValue::from_str(&api_key).map_err(|err| {
                CodexErr::UnsupportedOperation(format!("invalid Gemini API key: {err}"))
            })?,
        );
        let response = timeout(
            MODELS_REFRESH_TIMEOUT,
            build_reqwest_client().get(url).headers(headers).send(),
        )
        .await
        .map_err(|_| CodexErr::Timeout)?
        .map_err(|err| {
            CodexErr::UnsupportedOperation(format!("Gemini model list request failed: {err}"))
        })?;
        let status = response.status();
        if !status.is_success() {
            return Err(CodexErr::UnsupportedOperation(format!(
                "Gemini model list request failed with status {status}"
            )));
        }
        let body: GeminiModelsResponse = response.json().await.map_err(|err| {
            CodexErr::UnsupportedOperation(format!("failed to parse Gemini model list: {err}"))
        })?;
        let mut models = Vec::new();
        for (priority, model) in body
            .models
            .into_iter()
            .filter(|model| {
                model
                    .supported_generation_methods
                    .iter()
                    .any(|method| method == "generateContent")
            })
            .enumerate()
        {
            let slug = model
                .name
                .strip_prefix("models/")
                .unwrap_or(model.name.as_str());
            models.push(native_model_info(
                slug,
                model.display_name.as_deref().unwrap_or(slug),
                model
                    .description
                    .as_deref()
                    .unwrap_or("Google Gemini model through the native GenerateContent API."),
                model.input_token_limit,
                priority as i32,
            )?);
        }
        Ok((models, None))
    }
}

#[derive(Debug, Deserialize)]
struct AnthropicModelsResponse {
    data: Vec<AnthropicModel>,
}

#[derive(Debug, Deserialize)]
struct AnthropicModel {
    id: String,
    display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiModelsResponse {
    models: Vec<GeminiModel>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiModel {
    name: String,
    display_name: Option<String>,
    description: Option<String>,
    input_token_limit: Option<i64>,
    #[serde(default)]
    supported_generation_methods: Vec<String>,
}

fn provider_api_key(
    provider_info: &ModelProviderInfo,
    api_key_override: Option<&str>,
    provider_name: &str,
) -> CoreResult<String> {
    api_key_override
        .map(str::to_string)
        .or_else(|| {
            provider_info
                .env_key
                .as_deref()
                .and_then(|env_key| std::env::var(env_key).ok())
        })
        .filter(|api_key| !api_key.trim().is_empty())
        .ok_or_else(|| {
            CodexErr::UnsupportedOperation(format!(
                "{provider_name} model discovery requires provider env_key API-key auth"
            ))
        })
}

fn provider_models_url(
    provider_info: &ModelProviderInfo,
    default_base_url: &str,
) -> CoreResult<String> {
    let base_url = provider_info
        .base_url
        .as_deref()
        .unwrap_or(default_base_url)
        .trim_end_matches('/');
    let mut url = url::Url::parse(&format!("{base_url}{MODELS_ENDPOINT}")).map_err(|err| {
        CodexErr::UnsupportedOperation(format!("invalid model discovery base_url: {err}"))
    })?;
    if let Some(query_params) = &provider_info.query_params {
        url.query_pairs_mut().extend_pairs(query_params);
    }
    Ok(url.to_string())
}

fn add_configured_headers(provider_info: &ModelProviderInfo, headers: &mut HeaderMap) {
    if let Some(extra) = &provider_info.http_headers {
        for (name, val) in extra {
            if let Ok(name) = http::HeaderName::try_from(name)
                && let Ok(val) = HeaderValue::from_str(val)
            {
                headers.insert(name, val);
            }
        }
    }
    if let Some(env_headers) = &provider_info.env_http_headers {
        for (name, env_var) in env_headers {
            if let Ok(val) = std::env::var(env_var)
                && !val.trim().is_empty()
                && let Ok(name) = http::HeaderName::try_from(name)
                && let Ok(val) = HeaderValue::from_str(&val)
            {
                headers.insert(name, val);
            }
        }
    }
}

fn native_model_info(
    slug: &str,
    display_name: &str,
    description: &str,
    context_window: Option<i64>,
    priority: i32,
) -> CoreResult<ModelInfo> {
    serde_json::from_value(json!({
        "slug": slug,
        "display_name": display_name,
        "description": description,
        "default_reasoning_level": null,
        "supported_reasoning_levels": [],
        "shell_type": "shell_command",
        "visibility": "list",
        "supported_in_api": true,
        "priority": priority,
        "upgrade": null,
        "base_instructions": "",
        "supports_reasoning_summaries": false,
        "support_verbosity": false,
        "default_verbosity": null,
        "apply_patch_tool_type": null,
        "truncation_policy": {"mode": "bytes", "limit": 10_000},
        "supports_parallel_tool_calls": false,
        "supports_image_detail_original": false,
        "context_window": context_window,
        "max_context_window": context_window,
        "experimental_supported_tools": [],
        "input_modalities": ["text"],
        "supports_search_tool": false
    }))
    .map_err(|err| {
        CodexErr::UnsupportedOperation(format!("failed to build native model metadata: {err}"))
    })
}

#[derive(Clone)]
struct ModelsRequestTelemetry {
    auth_mode: Option<String>,
    auth_header_attached: bool,
    auth_header_name: Option<&'static str>,
    auth_env: AuthEnvTelemetry,
}

impl RequestTelemetry for ModelsRequestTelemetry {
    fn on_request(
        &self,
        attempt: u64,
        status: Option<http::StatusCode>,
        error: Option<&TransportError>,
        duration: Duration,
    ) {
        let success = status.is_some_and(|code| code.is_success()) && error.is_none();
        let error_message = error.map(telemetry_transport_error_message);
        let response_debug = error
            .map(extract_response_debug_context)
            .unwrap_or_default();
        let status = status.map(|status| status.as_u16());
        tracing::event!(
            target: "ontocode_otel.log_only",
            tracing::Level::INFO,
            event.name = "codex.api_request",
            duration_ms = %duration.as_millis(),
            http.response.status_code = status,
            success = success,
            error.message = error_message.as_deref(),
            attempt = attempt,
            endpoint = MODELS_ENDPOINT,
            auth.header_attached = self.auth_header_attached,
            auth.header_name = self.auth_header_name,
            auth.env_openai_api_key_present = self.auth_env.openai_api_key_env_present,
            auth.env_codex_api_key_present = self.auth_env.codex_api_key_env_present,
            auth.env_codex_api_key_enabled = self.auth_env.codex_api_key_env_enabled,
            auth.env_provider_key_name = self.auth_env.provider_env_key_name.as_deref(),
            auth.env_provider_key_present = self.auth_env.provider_env_key_present,
            auth.env_refresh_token_url_override_present = self.auth_env.refresh_token_url_override_present,
            auth.request_id = response_debug.request_id.as_deref(),
            auth.cf_ray = response_debug.cf_ray.as_deref(),
            auth.error = response_debug.auth_error.as_deref(),
            auth.error_code = response_debug.auth_error_code.as_deref(),
            auth.mode = self.auth_mode.as_deref(),
        );
        tracing::event!(
            target: "ontocode_otel.trace_safe",
            tracing::Level::INFO,
            event.name = "codex.api_request",
            duration_ms = %duration.as_millis(),
            http.response.status_code = status,
            success = success,
            error.message = error_message.as_deref(),
            attempt = attempt,
            endpoint = MODELS_ENDPOINT,
            auth.header_attached = self.auth_header_attached,
            auth.header_name = self.auth_header_name,
            auth.env_openai_api_key_present = self.auth_env.openai_api_key_env_present,
            auth.env_codex_api_key_present = self.auth_env.codex_api_key_env_present,
            auth.env_codex_api_key_enabled = self.auth_env.codex_api_key_env_enabled,
            auth.env_provider_key_name = self.auth_env.provider_env_key_name.as_deref(),
            auth.env_provider_key_present = self.auth_env.provider_env_key_present,
            auth.env_refresh_token_url_override_present = self.auth_env.refresh_token_url_override_present,
            auth.request_id = response_debug.request_id.as_deref(),
            auth.cf_ray = response_debug.cf_ray.as_deref(),
            auth.error = response_debug.auth_error.as_deref(),
            auth.error_code = response_debug.auth_error_code.as_deref(),
            auth.mode = self.auth_mode.as_deref(),
        );
        emit_feedback_request_tags_with_auth_env(
            &FeedbackRequestTags {
                endpoint: MODELS_ENDPOINT,
                auth_header_attached: self.auth_header_attached,
                auth_header_name: self.auth_header_name,
                auth_mode: self.auth_mode.as_deref(),
                auth_retry_after_unauthorized: None,
                auth_recovery_mode: None,
                auth_recovery_phase: None,
                auth_connection_reused: None,
                auth_request_id: response_debug.request_id.as_deref(),
                auth_cf_ray: response_debug.cf_ray.as_deref(),
                auth_error: response_debug.auth_error.as_deref(),
                auth_error_code: response_debug.auth_error_code.as_deref(),
                auth_recovery_followup_success: None,
                auth_recovery_followup_status: None,
            },
            &self.auth_env,
        );
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use super::*;
    use ontocode_protocol::config_types::ModelProviderAuthInfo;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::header;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    fn provider_info_with_command_auth() -> ModelProviderInfo {
        ModelProviderInfo {
            auth: Some(ModelProviderAuthInfo {
                command: "print-token".to_string(),
                args: Vec::new(),
                timeout_ms: NonZeroU64::new(5_000).expect("timeout should be non-zero"),
                refresh_interval_ms: 300_000,
                cwd: std::env::current_dir()
                    .expect("current dir should be available")
                    .try_into()
                    .expect("current dir should be absolute"),
            }),
            requires_openai_auth: false,
            ..ModelProviderInfo::create_openai_provider(/*base_url*/ None)
        }
    }

    fn native_provider_info(base_url: String) -> ModelProviderInfo {
        ModelProviderInfo {
            name: "native".to_string(),
            base_url: Some(base_url),
            env_key: Some("NATIVE_TEST_API_KEY".to_string()),
            requires_openai_auth: false,
            ..Default::default()
        }
    }

    #[test]
    fn command_auth_provider_reports_command_auth_without_cached_auth() {
        let endpoint = OpenAiModelsEndpoint::new(
            provider_info_with_command_auth(),
            /*auth_manager*/ None,
        );

        assert!(endpoint.has_command_auth());
    }

    #[test]
    fn provider_without_command_auth_reports_no_command_auth() {
        let endpoint = OpenAiModelsEndpoint::new(
            ModelProviderInfo::create_openai_provider(/*base_url*/ None),
            /*auth_manager*/ None,
        );

        assert!(!endpoint.has_command_auth());
    }

    #[tokio::test]
    async fn anthropic_models_endpoint_lists_models_with_provider_headers() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/models"))
            .and(header("anthropic-version", ANTHROPIC_VERSION))
            .and(header("x-api-key", "anthropic-test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [
                    {
                        "id": "claude-dynamic-test",
                        "display_name": "Claude Dynamic Test"
                    }
                ]
            })))
            .expect(1)
            .mount(&server)
            .await;
        let endpoint = AnthropicModelsEndpoint::new_for_testing(
            native_provider_info(server.uri()),
            "anthropic-test-key",
        );

        let (models, etag) = endpoint
            .list_models("test-client")
            .await
            .expect("model list should succeed");

        assert_eq!(etag, None);
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].slug, "claude-dynamic-test");
        assert_eq!(models[0].display_name, "Claude Dynamic Test");
    }

    #[tokio::test]
    async fn gemini_models_endpoint_lists_generate_content_models_only() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/models"))
            .and(header("x-goog-api-key", "gemini-test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "models": [
                    {
                        "name": "models/gemini-dynamic-test",
                        "displayName": "Gemini Dynamic Test",
                        "description": "dynamic Gemini model",
                        "inputTokenLimit": 123456,
                        "supportedGenerationMethods": ["generateContent"]
                    },
                    {
                        "name": "models/gemini-embed-test",
                        "displayName": "Gemini Embed Test",
                        "supportedGenerationMethods": ["embedContent"]
                    }
                ]
            })))
            .expect(1)
            .mount(&server)
            .await;
        let endpoint = GeminiModelsEndpoint::new_for_testing(
            native_provider_info(server.uri()),
            "gemini-test-key",
        );

        let (models, etag) = endpoint
            .list_models("test-client")
            .await
            .expect("model list should succeed");

        assert_eq!(etag, None);
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].slug, "gemini-dynamic-test");
        assert_eq!(models[0].display_name, "Gemini Dynamic Test");
        assert_eq!(models[0].context_window, Some(123456));
    }

    #[tokio::test]
    async fn native_model_discovery_errors_do_not_include_secret_values() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/models"))
            .respond_with(
                ResponseTemplate::new(401).set_body_string("secret-token-should-not-appear"),
            )
            .expect(1)
            .mount(&server)
            .await;
        let endpoint = GeminiModelsEndpoint::new_for_testing(
            native_provider_info(server.uri()),
            "gemini-secret-key",
        );

        let err = endpoint
            .list_models("test-client")
            .await
            .expect_err("model list should fail");
        let message = err.to_string();

        assert!(!message.contains("gemini-secret-key"));
        assert!(!message.contains("secret-token-should-not-appear"));
        assert!(message.contains("Gemini model list request failed with status"));
    }
}
