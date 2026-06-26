use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

use ontocode_api::Provider;
use ontocode_api::SharedAuthProvider;
use ontocode_login::AuthManager;
use ontocode_login::CodexAuth;
use ontocode_model_provider_info::ModelProviderInfo;
use ontocode_models_manager::manager::NativeModelsManager;
use ontocode_models_manager::manager::OpenAiModelsManager;
use ontocode_models_manager::manager::SharedModelsManager;
use ontocode_models_manager::manager::StaticModelsManager;
use ontocode_models_manager::native_provider_catalogs;
use ontocode_protocol::account::ProviderAccount;
use ontocode_protocol::openai_models::ModelsResponse;

use crate::amazon_bedrock::AmazonBedrockModelProvider;
use crate::auth::auth_manager_for_provider;
use crate::auth::resolve_provider_auth_with_manager;
use crate::descriptor::ProviderDescriptor;
use crate::descriptor::ProviderEngine;
use crate::models_endpoint::AnthropicModelsEndpoint;
use crate::models_endpoint::GeminiModelsEndpoint;
use crate::models_endpoint::OpenAiModelsEndpoint;

/// Optional provider-backed features that Codex may expose at runtime.
///
/// These capabilities are a provider-owned upper bound. Callers can disable
/// more functionality through normal config, but should not expose a feature
/// that the active provider marks unsupported here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderCapabilities {
    pub namespace_tools: bool,
    pub image_generation: bool,
    pub web_search: bool,
    pub requires_openai_auth: bool,
    pub supports_models_route_probe: bool,
}

impl Default for ProviderCapabilities {
    fn default() -> Self {
        Self {
            namespace_tools: true,
            image_generation: true,
            web_search: true,
            requires_openai_auth: true,
            supports_models_route_probe: true,
        }
    }
}

/// Current app-visible account state for a model provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderAccountState {
    pub account: Option<ProviderAccount>,
    pub requires_openai_auth: bool,
}

/// Error returned when a provider cannot construct its app-visible account state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderAccountError {
    MissingChatgptAccountDetails,
}

impl fmt::Display for ProviderAccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingChatgptAccountDetails => {
                write!(
                    f,
                    "email and plan type are required for chatgpt authentication"
                )
            }
        }
    }
}

impl std::error::Error for ProviderAccountError {}

pub type ProviderAccountResult = std::result::Result<ProviderAccountState, ProviderAccountError>;

/// Default model used for automatic approval review when a provider does not
/// require a backend-specific model ID.
pub const DEFAULT_APPROVAL_REVIEW_PREFERRED_MODEL: &str = "codex-auto-review";

/// Internal runtime protocol used by the core model client.
///
/// This is intentionally separate from public `WireApi` configuration so native
/// built-in providers can be staged without adding new config/schema values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderRuntimeEngine {
    OpenAiResponses,
    AmazonBedrockResponses,
    AnthropicMessages,
    GeminiGenerateContent,
    GitHubCopilot,
}

impl fmt::Display for ProviderRuntimeEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenAiResponses => write!(f, "openai-responses"),
            Self::AmazonBedrockResponses => write!(f, "amazon-bedrock-responses"),
            Self::AnthropicMessages => write!(f, "anthropic-messages"),
            Self::GeminiGenerateContent => write!(f, "gemini-generate-content"),
            Self::GitHubCopilot => write!(f, "github-copilot"),
        }
    }
}

impl ProviderRuntimeEngine {
    pub(crate) fn from_provider_engine(engine: ProviderEngine) -> Self {
        match engine {
            ProviderEngine::OpenAiResponses => Self::OpenAiResponses,
            ProviderEngine::AmazonBedrockResponses => Self::AmazonBedrockResponses,
            ProviderEngine::AnthropicMessages => Self::AnthropicMessages,
            ProviderEngine::GeminiGenerateContent => Self::GeminiGenerateContent,
            ProviderEngine::GitHubCopilot => Self::GitHubCopilot,
        }
    }
}

/// Runtime provider abstraction used by model execution.
///
/// Implementations own provider-specific behavior for a model backend. The
/// `ModelProviderInfo` returned by `info` is the serialized/configured provider
/// metadata used by the default OpenAI-compatible implementation.
#[async_trait::async_trait]
pub trait ModelProvider: fmt::Debug + Send + Sync {
    /// Returns the configured provider metadata.
    fn info(&self) -> &ModelProviderInfo;

    /// Returns the provider id used for auth and runtime routing.
    fn provider_id(&self) -> &str {
        self.info().name.as_str()
    }

    /// Returns the internal runtime protocol used to execute model requests.
    fn runtime_engine(&self) -> ProviderRuntimeEngine {
        ProviderRuntimeEngine::OpenAiResponses
    }

    /// Returns the provider-owned capability upper bounds.
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::default()
    }

    /// Returns the preferred model used for automatic approval review.
    ///
    /// Providers that require backend-specific model IDs should override this.
    fn approval_review_preferred_model(&self) -> &'static str {
        DEFAULT_APPROVAL_REVIEW_PREFERRED_MODEL
    }

    /// Returns whether requests made through this provider should include attestation.
    fn supports_attestation(&self) -> bool {
        false
    }

    /// Returns the provider-scoped auth manager, when this provider uses one.
    ///
    /// TODO(celia-oai): Make auth manager access internal to this crate so callers
    /// resolve provider-specific auth only through `ModelProvider`. We first need
    /// to think through whether Codex should have a unified provider-specific auth
    /// manager throughout the codebase; that is a larger refactor than this change.
    fn auth_manager(&self) -> Option<Arc<AuthManager>>;

    /// Returns the current provider-scoped auth value, if one is configured.
    async fn auth(&self) -> Option<CodexAuth>;

    /// Returns the current app-visible account state for this provider.
    fn account_state(&self) -> ProviderAccountResult;

    /// Returns provider configuration adapted for the API client.
    async fn api_provider(&self) -> ontocode_protocol::error::Result<Provider> {
        let auth = self.auth().await;
        self.info()
            .to_api_provider(auth.as_ref().map(CodexAuth::auth_mode))
    }

    /// Returns the provider base URL that will be used at request time.
    async fn runtime_base_url(&self) -> ontocode_protocol::error::Result<Option<String>> {
        Ok(self.info().base_url.clone())
    }

    /// Returns the auth provider used to attach request credentials.
    async fn api_auth(&self) -> ontocode_protocol::error::Result<SharedAuthProvider> {
        let auth = self.auth().await;
        resolve_provider_auth_with_manager(
            self.auth_manager().as_deref(),
            auth.as_ref(),
            self.provider_id(),
            self.info(),
        )
        .await
    }

    /// Creates the model manager implementation appropriate for this provider.
    fn models_manager(
        &self,
        codex_home: PathBuf,
        config_model_catalog: Option<ModelsResponse>,
    ) -> SharedModelsManager;
}

/// Shared runtime model provider handle.
pub type SharedModelProvider = Arc<dyn ModelProvider>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProviderKind {
    Configured,
    AmazonBedrock,
}

impl ProviderKind {
    fn for_provider(provider_info: &ModelProviderInfo) -> Self {
        Self::for_descriptor(ProviderDescriptor::for_provider(provider_info))
    }

    fn for_descriptor(descriptor: ProviderDescriptor) -> Self {
        match descriptor.engine() {
            ProviderEngine::OpenAiResponses => Self::Configured,
            ProviderEngine::AmazonBedrockResponses => Self::AmazonBedrock,
            ProviderEngine::AnthropicMessages
            | ProviderEngine::GeminiGenerateContent
            | ProviderEngine::GitHubCopilot => Self::Configured,
        }
    }

    fn create_provider(
        self,
        provider_id: String,
        provider_info: ModelProviderInfo,
        auth_manager: Option<Arc<AuthManager>>,
    ) -> SharedModelProvider {
        match self {
            Self::Configured => Arc::new(ConfiguredModelProvider::new(
                provider_id,
                provider_info,
                auth_manager,
            )),
            Self::AmazonBedrock => Arc::new(AmazonBedrockModelProvider::new(provider_info)),
        }
    }
}

/// Creates the default runtime model provider for configured provider metadata.
pub fn create_model_provider(
    provider_info: ModelProviderInfo,
    auth_manager: Option<Arc<AuthManager>>,
) -> SharedModelProvider {
    let provider_id = provider_info.name.clone();
    create_model_provider_with_id(provider_id.as_str(), provider_info, auth_manager)
}

pub(crate) fn create_model_provider_with_id(
    provider_id: &str,
    provider_info: ModelProviderInfo,
    auth_manager: Option<Arc<AuthManager>>,
) -> SharedModelProvider {
    ProviderKind::for_provider(&provider_info).create_provider(
        provider_id.to_string(),
        provider_info,
        auth_manager,
    )
}

/// Runtime model provider backed by configured `ModelProviderInfo`.
#[derive(Clone, Debug)]
struct ConfiguredModelProvider {
    provider_id: String,
    info: ModelProviderInfo,
    auth_manager: Option<Arc<AuthManager>>,
}

impl ConfiguredModelProvider {
    fn new(
        provider_id: String,
        provider_info: ModelProviderInfo,
        auth_manager: Option<Arc<AuthManager>>,
    ) -> Self {
        let auth_manager = auth_manager_for_provider(auth_manager, &provider_info);
        Self {
            provider_id,
            info: provider_info,
            auth_manager,
        }
    }
}

#[async_trait::async_trait]
impl ModelProvider for ConfiguredModelProvider {
    fn info(&self) -> &ModelProviderInfo {
        &self.info
    }

    fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    fn runtime_engine(&self) -> ProviderRuntimeEngine {
        ProviderRuntimeEngine::from_provider_engine(
            ProviderDescriptor::for_provider(&self.info).engine(),
        )
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderDescriptor::for_provider(&self.info).capabilities()
    }

    fn auth_manager(&self) -> Option<Arc<AuthManager>> {
        self.auth_manager.clone()
    }

    fn supports_attestation(&self) -> bool {
        self.auth_manager
            .as_ref()
            .and_then(|auth_manager| auth_manager.auth_cached())
            .is_some_and(|auth| auth.is_chatgpt_auth())
    }

    async fn auth(&self) -> Option<CodexAuth> {
        match self.auth_manager.as_ref() {
            Some(auth_manager) => auth_manager.auth().await,
            None => None,
        }
    }

    fn account_state(&self) -> ProviderAccountResult {
        let account = if self.info.requires_openai_auth {
            self.auth_manager
                .as_ref()
                .and_then(|auth_manager| {
                    let auth = auth_manager.auth_cached()?;
                    if auth_manager.refresh_failure_for_auth(&auth).is_some() {
                        return None;
                    }
                    Some(auth)
                })
                .map(|auth| match &auth {
                    CodexAuth::ApiKey(_) => Ok(ProviderAccount::ApiKey),
                    CodexAuth::Chatgpt(_)
                    | CodexAuth::ChatgptAuthTokens(_)
                    | CodexAuth::AgentIdentity(_) => {
                        let email = auth.get_account_email();
                        let plan_type = auth.account_plan_type();

                        match (email, plan_type) {
                            (Some(email), Some(plan_type)) => {
                                Ok(ProviderAccount::Chatgpt { email, plan_type })
                            }
                            _ => Err(ProviderAccountError::MissingChatgptAccountDetails),
                        }
                    }
                })
                .transpose()?
        } else {
            None
        };

        Ok(ProviderAccountState {
            account,
            requires_openai_auth: self.info.requires_openai_auth,
        })
    }

    fn models_manager(
        &self,
        codex_home: PathBuf,
        config_model_catalog: Option<ModelsResponse>,
    ) -> SharedModelsManager {
        match config_model_catalog {
            Some(model_catalog) => Arc::new(StaticModelsManager::new(
                self.auth_manager.clone(),
                model_catalog,
            )),
            None => match self.runtime_engine() {
                ProviderRuntimeEngine::AnthropicMessages => Arc::new(NativeModelsManager::new(
                    self.auth_manager.clone(),
                    native_provider_catalogs::anthropic_models_response(),
                    Arc::new(AnthropicModelsEndpoint::new(self.info.clone())),
                )),
                ProviderRuntimeEngine::GeminiGenerateContent => Arc::new(NativeModelsManager::new(
                    self.auth_manager.clone(),
                    native_provider_catalogs::gemini_models_response(),
                    Arc::new(GeminiModelsEndpoint::new(self.info.clone())),
                )),
                ProviderRuntimeEngine::GitHubCopilot => Arc::new(StaticModelsManager::new(
                    self.auth_manager.clone(),
                    native_provider_catalogs::github_copilot_models_response(),
                )),
                ProviderRuntimeEngine::OpenAiResponses => {
                    let endpoint = Arc::new(OpenAiModelsEndpoint::new(
                        self.info.clone(),
                        self.auth_manager.clone(),
                    ));
                    Arc::new(OpenAiModelsManager::new(
                        codex_home,
                        endpoint,
                        self.auth_manager.clone(),
                    ))
                }
                ProviderRuntimeEngine::AmazonBedrockResponses => unreachable!(
                    "Amazon Bedrock providers are handled by AmazonBedrockModelProvider"
                ),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::num::NonZeroU64;
    use std::sync::Mutex;
    use std::sync::OnceLock;

    use ontocode_login::AuthCredentialsStoreMode;
    use ontocode_login::auth::upsert_provider_oauth_credential;
    use ontocode_model_provider_info::ModelProviderAwsAuthInfo;
    use ontocode_model_provider_info::WireApi;
    use ontocode_models_manager::manager::RefreshStrategy;
    use ontocode_protocol::config_types::ModelProviderAuthInfo;
    use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
    use ontocode_protocol::openai_models::ModelInfo;
    use ontocode_protocol::openai_models::ModelsResponse;
    use ontocode_provider_auth::ProviderOAuthCredential;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::header_regex;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    use super::*;

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

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

    fn test_codex_home() -> std::path::PathBuf {
        std::env::temp_dir().join(format!("codex-model-provider-test-{}", std::process::id()))
    }

    fn provider_for(base_url: String) -> ModelProviderInfo {
        ModelProviderInfo {
            name: "mock".into(),
            base_url: Some(base_url),
            env_key: None,
            env_key_instructions: None,
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::Responses,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(0),
            stream_max_retries: Some(0),
            stream_idle_timeout_ms: Some(5_000),
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    fn remote_model(slug: &str) -> ModelInfo {
        serde_json::from_value(json!({
            "slug": slug,
            "display_name": slug,
            "description": null,
            "default_reasoning_level": "medium",
            "supported_reasoning_levels": [],
            "shell_type": "shell_command",
            "visibility": "list",
            "supported_in_api": true,
            "priority": 0,
            "upgrade": null,
            "base_instructions": "base instructions",
            "supports_reasoning_summaries": false,
            "support_verbosity": false,
            "default_verbosity": null,
            "apply_patch_tool_type": null,
            "truncation_policy": {"mode": "bytes", "limit": 10_000},
            "supports_parallel_tool_calls": false,
            "supports_image_detail_original": false,
            "context_window": 272_000,
            "max_context_window": 272_000,
            "experimental_supported_tools": [],
        }))
        .expect("valid model")
    }

    fn native_provider_info_with_command_auth(
        name: &str,
        base_url: &str,
        env_key: Option<&str>,
    ) -> ModelProviderInfo {
        ModelProviderInfo {
            name: name.to_string(),
            base_url: Some(base_url.to_string()),
            env_key: env_key.map(str::to_string),
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
            ..Default::default()
        }
    }

    async fn catalog_slugs(manager: SharedModelsManager) -> Vec<String> {
        manager
            .raw_model_catalog(RefreshStrategy::Online)
            .await
            .models
            .into_iter()
            .map(|model| model.slug)
            .collect()
    }

    #[test]
    fn provider_kind_classifies_openai_provider_as_configured() {
        assert_eq!(
            ProviderKind::for_provider(&ModelProviderInfo::create_openai_provider(
                /*base_url*/ None
            )),
            ProviderKind::Configured
        );
    }

    #[test]
    fn provider_kind_classifies_amazon_bedrock_provider() {
        assert_eq!(
            ProviderKind::for_provider(&ModelProviderInfo::create_amazon_bedrock_provider(
                /*aws*/ None
            )),
            ProviderKind::AmazonBedrock
        );
    }

    #[test]
    fn provider_kind_classifies_copilot_provider_as_configured_runtime() {
        assert_eq!(
            ProviderKind::for_provider(&ModelProviderInfo {
                name: "GitHub Copilot".to_string(),
                requires_openai_auth: false,
                ..Default::default()
            }),
            ProviderKind::Configured
        );
    }

    #[test]
    fn provider_kind_classifies_anthropic_descriptor_as_configured_runtime() {
        assert_eq!(
            ProviderKind::for_provider(&ModelProviderInfo {
                name: "Anthropic".to_string(),
                base_url: Some("https://api.anthropic.com/v1".to_string()),
                env_key: Some("ANTHROPIC_API_KEY".to_string()),
                requires_openai_auth: false,
                ..Default::default()
            }),
            ProviderKind::Configured
        );
    }

    #[test]
    fn provider_kind_classifies_gemini_descriptor_as_configured_runtime() {
        assert_eq!(
            ProviderKind::for_provider(&ModelProviderInfo {
                name: "Gemini".to_string(),
                base_url: Some("https://generativelanguage.googleapis.com/v1beta".to_string()),
                env_key: Some("GEMINI_API_KEY".to_string()),
                requires_openai_auth: false,
                ..Default::default()
            }),
            ProviderKind::Configured
        );
    }

    #[test]
    fn configured_provider_uses_default_capabilities() {
        let provider = create_model_provider(
            ModelProviderInfo::create_openai_provider(/*base_url*/ None),
            /*auth_manager*/ None,
        );

        assert_eq!(provider.capabilities(), ProviderCapabilities::default());
    }

    #[test]
    fn configured_provider_uses_openai_responses_runtime_engine() {
        let provider = create_model_provider(
            ModelProviderInfo::create_openai_provider(/*base_url*/ None),
            /*auth_manager*/ None,
        );

        assert_eq!(
            provider.runtime_engine(),
            ProviderRuntimeEngine::OpenAiResponses
        );
    }

    #[test]
    fn configured_gemini_provider_uses_gemini_generate_content_runtime_engine() {
        let provider = create_model_provider(
            ModelProviderInfo {
                name: "Gemini".to_string(),
                base_url: Some("https://generativelanguage.googleapis.com/v1beta".to_string()),
                env_key: Some("GEMINI_API_KEY".to_string()),
                requires_openai_auth: false,
                ..Default::default()
            },
            /*auth_manager*/ None,
        );

        assert_eq!(
            provider.runtime_engine(),
            ProviderRuntimeEngine::GeminiGenerateContent
        );
    }

    #[test]
    fn configured_provider_capabilities_follow_auth_requirement() {
        let provider = create_model_provider(
            ModelProviderInfo {
                requires_openai_auth: false,
                ..ModelProviderInfo::create_openai_provider(/*base_url*/ None)
            },
            /*auth_manager*/ None,
        );

        assert_eq!(
            provider.capabilities(),
            ProviderCapabilities {
                requires_openai_auth: false,
                ..ProviderCapabilities::default()
            }
        );
    }

    #[test]
    fn configured_azure_provider_disables_models_route_probe() {
        let provider = create_model_provider(
            ModelProviderInfo {
                name: "azure".to_string(),
                base_url: Some("https://example.openai.azure.com/openai/v1".to_string()),
                ..ModelProviderInfo::create_openai_provider(/*base_url*/ None)
            },
            /*auth_manager*/ None,
        );

        assert_eq!(
            provider.capabilities(),
            ProviderCapabilities {
                supports_models_route_probe: false,
                ..ProviderCapabilities::default()
            }
        );
    }

    #[test]
    fn configured_provider_uses_default_approval_review_preferred_model() {
        let provider = create_model_provider(
            ModelProviderInfo::create_openai_provider(/*base_url*/ None),
            /*auth_manager*/ None,
        );

        assert_eq!(
            provider.approval_review_preferred_model(),
            DEFAULT_APPROVAL_REVIEW_PREFERRED_MODEL
        );
    }

    #[tokio::test]
    async fn configured_provider_runtime_base_url_uses_configured_base_url() {
        let provider = create_model_provider(
            provider_for("https://example.test/v1".to_string()),
            /*auth_manager*/ None,
        );

        assert_eq!(
            provider
                .runtime_base_url()
                .await
                .expect("runtime base URL should resolve"),
            Some("https://example.test/v1".to_string())
        );
    }

    #[test]
    fn create_model_provider_builds_command_auth_manager_without_base_manager() {
        let provider = create_model_provider(
            provider_info_with_command_auth(),
            /*auth_manager*/ None,
        );

        let auth_manager = provider
            .auth_manager()
            .expect("command auth provider should have an auth manager");

        assert!(auth_manager.has_external_auth());
    }

    #[test]
    fn create_model_provider_does_not_use_openai_auth_manager_for_amazon_bedrock_provider() {
        let provider = create_model_provider(
            ModelProviderInfo::create_amazon_bedrock_provider(Some(ModelProviderAwsAuthInfo {
                profile: Some("codex-bedrock".to_string()),
                region: None,
            })),
            Some(AuthManager::from_auth_for_testing(CodexAuth::from_api_key(
                "openai-api-key",
            ))),
        );

        assert!(provider.auth_manager().is_none());
    }

    #[tokio::test]
    async fn provider_api_auth_uses_env_key_and_configured_bearer_token_without_oauth_profiles() {
        let env_key = "NATIVE_TEST_API_KEY";
        {
            let _env_lock = env_lock().lock().expect("env lock");
            let previous = env::var(env_key).ok();
            unsafe {
                env::set_var(env_key, "env-key-token");
            }

            let env_key_auth = crate::auth::resolve_provider_auth(
                /*auth*/ None,
                &ModelProviderInfo {
                    name: "Anthropic".to_string(),
                    base_url: Some("https://api.anthropic.com/v1".to_string()),
                    env_key: Some(env_key.to_string()),
                    requires_openai_auth: false,
                    ..Default::default()
                },
            )
            .expect("env key auth should resolve");
            let mut env_key_headers = http::HeaderMap::new();
            env_key_auth.add_auth_headers(&mut env_key_headers);

            assert_eq!(
                env_key_headers
                    .get(http::header::AUTHORIZATION)
                    .and_then(|value| value.to_str().ok()),
                Some("Bearer env-key-token")
            );
            assert!(env_key_headers.get("ChatGPT-Account-ID").is_none());

            match previous {
                Some(value) => unsafe {
                    env::set_var(env_key, value);
                },
                None => unsafe {
                    env::remove_var(env_key);
                },
            }
        }

        let configured_bearer_provider = create_model_provider(
            ModelProviderInfo {
                name: "Anthropic".to_string(),
                base_url: Some("https://api.anthropic.com/v1".to_string()),
                experimental_bearer_token: Some("configured-bearer-token".to_string()),
                requires_openai_auth: false,
                ..Default::default()
            },
            /*auth_manager*/ None,
        );
        let configured_bearer_auth = configured_bearer_provider
            .api_auth()
            .await
            .expect("configured bearer auth should resolve");
        let mut configured_bearer_headers = http::HeaderMap::new();
        configured_bearer_auth.add_auth_headers(&mut configured_bearer_headers);

        assert_eq!(
            configured_bearer_headers
                .get(http::header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok()),
            Some("Bearer configured-bearer-token")
        );
        assert!(
            configured_bearer_headers
                .get("ChatGPT-Account-ID")
                .is_none()
        );
    }

    #[test]
    fn gemini_cli_identity_keeps_provider_id_and_display_name_separate() {
        let provider = create_model_provider_with_id(
            "gemini-cli",
            ModelProviderInfo::create_gemini_cli_provider(),
            /*auth_manager*/ None,
        );

        assert_eq!(provider.provider_id(), "gemini-cli");
        assert_eq!(provider.info().name, "Gemini CLI");
    }

    #[tokio::test]
    async fn configured_provider_api_auth_uses_provider_id_alias_for_oauth_lookup() {
        let codex_home = std::env::temp_dir().join(format!(
            "ontocode-model-provider-auth-alias-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        std::fs::create_dir_all(&codex_home).expect("temp dir should be created");
        upsert_provider_oauth_credential(
            codex_home.as_path(),
            AuthCredentialsStoreMode::File,
            ProviderOAuthCredential::new(
                "gemini-cli",
                ProviderCredentialSourceKind::ExternalImport,
                "gemini-cli:workspace-1",
                "gemini-cli-access-token",
            ),
        )
        .expect("provider oauth credential should persist");
        let auth_manager = AuthManager::from_auth_for_testing_with_home(
            CodexAuth::from_api_key("openai-api-key"),
            codex_home,
        );
        let provider = create_model_provider_with_id(
            "gemini-cli",
            ModelProviderInfo::create_gemini_provider(),
            Some(auth_manager),
        );

        let auth = provider
            .api_auth()
            .await
            .expect("provider auth should resolve");
        let mut headers = http::HeaderMap::new();
        auth.add_auth_headers(&mut headers);

        assert_eq!(
            headers
                .get(http::header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok()),
            Some("Bearer gemini-cli-access-token")
        );
    }

    #[test]
    fn amazon_bedrock_provider_uses_bedrock_runtime_engine() {
        let provider = create_model_provider(
            ModelProviderInfo::create_amazon_bedrock_provider(/*aws*/ None),
            /*auth_manager*/ None,
        );

        assert_eq!(
            provider.runtime_engine(),
            ProviderRuntimeEngine::AmazonBedrockResponses
        );
    }

    #[test]
    fn openai_provider_returns_unauthenticated_openai_account_state() {
        let provider = create_model_provider(
            ModelProviderInfo::create_openai_provider(/*base_url*/ None),
            /*auth_manager*/ None,
        );

        assert_eq!(
            provider.account_state(),
            Ok(ProviderAccountState {
                account: None,
                requires_openai_auth: true,
            })
        );
    }

    #[test]
    fn openai_provider_returns_api_key_account_state() {
        let provider = create_model_provider(
            ModelProviderInfo::create_openai_provider(/*base_url*/ None),
            Some(AuthManager::from_auth_for_testing(CodexAuth::from_api_key(
                "openai-api-key",
            ))),
        );

        assert_eq!(
            provider.account_state(),
            Ok(ProviderAccountState {
                account: Some(ProviderAccount::ApiKey),
                requires_openai_auth: true,
            })
        );
    }

    #[test]
    fn custom_non_openai_provider_returns_no_account_state() {
        let provider = create_model_provider(
            ModelProviderInfo {
                name: "Custom".to_string(),
                base_url: Some("http://localhost:1234/v1".to_string()),
                wire_api: WireApi::Responses,
                requires_openai_auth: false,
                ..Default::default()
            },
            /*auth_manager*/ None,
        );

        assert_eq!(
            provider.account_state(),
            Ok(ProviderAccountState {
                account: None,
                requires_openai_auth: false,
            })
        );
    }

    #[test]
    fn amazon_bedrock_provider_returns_bedrock_account_state() {
        let provider = create_model_provider(
            ModelProviderInfo::create_amazon_bedrock_provider(/*aws*/ None),
            /*auth_manager*/ None,
        );

        assert_eq!(
            provider.account_state(),
            Ok(ProviderAccountState {
                account: Some(ProviderAccount::AmazonBedrock),
                requires_openai_auth: false,
            })
        );
    }

    #[tokio::test]
    async fn amazon_bedrock_provider_creates_static_models_manager() {
        let provider = create_model_provider(
            ModelProviderInfo::create_amazon_bedrock_provider(/*aws*/ None),
            /*auth_manager*/ None,
        );
        let manager =
            provider.models_manager(test_codex_home(), /*config_model_catalog*/ None);

        let catalog = manager.raw_model_catalog(RefreshStrategy::Online).await;
        let model_ids = catalog
            .models
            .iter()
            .map(|model| model.slug.as_str())
            .collect::<Vec<_>>();

        assert_eq!(model_ids, vec!["openai.gpt-5.5", "openai.gpt-5.4"]);

        let default_model = manager
            .list_models(RefreshStrategy::Online)
            .await
            .into_iter()
            .find(|preset| preset.is_default)
            .expect("Bedrock catalog should have a default model");

        assert_eq!(default_model.model, "openai.gpt-5.5");
    }

    #[tokio::test]
    async fn configured_native_providers_create_static_models_managers() {
        let cases = [
            (
                native_provider_info_with_command_auth(
                    "Anthropic",
                    "https://api.anthropic.com/v1",
                    Some("ANTHROPIC_API_KEY"),
                ),
                vec!["claude-sonnet-4-5".to_string()],
            ),
            (
                native_provider_info_with_command_auth(
                    "Gemini",
                    "https://generativelanguage.googleapis.com/v1beta",
                    Some("GEMINI_API_KEY"),
                ),
                vec!["gemini-2.5-pro".to_string()],
            ),
            (
                native_provider_info_with_command_auth(
                    "GitHub Copilot",
                    "https://api.githubcopilot.com",
                    /*env_key*/ None,
                ),
                vec!["gpt-4o-copilot".to_string()],
            ),
        ];

        for (provider_info, expected_slugs) in cases {
            let provider = create_model_provider(provider_info, /*auth_manager*/ None);
            let manager =
                provider.models_manager(test_codex_home(), /*config_model_catalog*/ None);

            assert_eq!(catalog_slugs(manager).await, expected_slugs);
        }
    }

    #[tokio::test]
    async fn configured_native_provider_catalog_json_takes_precedence() {
        let provider = create_model_provider(
            native_provider_info_with_command_auth(
                "Gemini",
                "https://generativelanguage.googleapis.com/v1beta",
                Some("GEMINI_API_KEY"),
            ),
            /*auth_manager*/ None,
        );
        let manager = provider.models_manager(
            test_codex_home(),
            Some(ModelsResponse {
                models: vec![remote_model("configured-gemini-model")],
            }),
        );

        assert_eq!(
            catalog_slugs(manager).await,
            vec!["configured-gemini-model".to_string()]
        );
    }

    #[tokio::test]
    async fn configured_bedrock_catalog_only_allows_default_service_tier() {
        let configured_model = ontocode_models_manager::bundled_models_response()
            .expect("bundled models should parse")
            .models
            .into_iter()
            .find(|model| model.slug == "gpt-5.5")
            .expect("bundled models should include GPT-5.5");
        assert!(!configured_model.additional_speed_tiers.is_empty());
        assert!(!configured_model.service_tiers.is_empty());

        let provider = create_model_provider(
            ModelProviderInfo::create_amazon_bedrock_provider(/*aws*/ None),
            /*auth_manager*/ None,
        );
        let manager = provider.models_manager(
            test_codex_home(),
            Some(ModelsResponse {
                models: vec![configured_model],
            }),
        );

        let catalog = manager.raw_model_catalog(RefreshStrategy::Online).await;

        assert_eq!(catalog.models.len(), 1);
        assert_eq!(catalog.models[0].slug, "gpt-5.5");
        assert_eq!(
            catalog.models[0].additional_speed_tiers,
            Vec::<String>::new()
        );
        assert_eq!(catalog.models[0].service_tiers, Vec::new());
        assert_eq!(catalog.models[0].default_service_tier, None);
    }

    #[tokio::test]
    async fn configured_provider_models_manager_uses_provider_bearer_token() {
        let server = MockServer::start().await;
        let remote_models = vec![remote_model("provider-model")];

        Mock::given(method("GET"))
            .and(path("/models"))
            .and(header_regex("Authorization", "Bearer provider-token"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/json")
                    .set_body_json(ModelsResponse {
                        models: remote_models.clone(),
                    }),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut provider_info = provider_for(server.uri());
        provider_info.experimental_bearer_token = Some("provider-token".to_string());
        let provider = create_model_provider(
            provider_info,
            Some(AuthManager::from_auth_for_testing(
                CodexAuth::create_dummy_chatgpt_auth_for_testing(),
            )),
        );

        let manager =
            provider.models_manager(test_codex_home(), /*config_model_catalog*/ None);
        let catalog = manager.raw_model_catalog(RefreshStrategy::Online).await;

        assert!(
            catalog
                .models
                .iter()
                .any(|model| model.slug == "provider-model")
        );
    }
}
