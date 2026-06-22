use std::sync::Arc;

use http::HeaderMap;
use http::HeaderValue;
use ontocode_agent_identity::AgentIdentityKey;
use ontocode_agent_identity::AgentTaskAuthorizationTarget;
use ontocode_agent_identity::authorization_header_for_agent_task;
use ontocode_api::AuthProvider;
use ontocode_api::SharedAuthProvider;
use ontocode_login::AuthManager;
use ontocode_login::CodexAuth;
use ontocode_model_provider_info::ModelProviderInfo;

use crate::bearer_auth_provider::BearerAuthProvider;

#[derive(Clone, Debug)]
struct AgentIdentityAuthProvider {
    auth: ontocode_login::auth::AgentIdentityAuth,
}

impl AuthProvider for AgentIdentityAuthProvider {
    fn add_auth_headers(&self, headers: &mut HeaderMap) {
        let record = self.auth.record();
        let header_value = authorization_header_for_agent_task(
            AgentIdentityKey {
                agent_runtime_id: &record.agent_runtime_id,
                private_key_pkcs8_base64: &record.agent_private_key,
            },
            AgentTaskAuthorizationTarget {
                agent_runtime_id: &record.agent_runtime_id,
                task_id: self.auth.process_task_id(),
            },
        )
        .map_err(std::io::Error::other);

        if let Ok(header_value) = header_value
            && let Ok(header) = HeaderValue::from_str(&header_value)
        {
            let _ = headers.insert(http::header::AUTHORIZATION, header);
        }

        if let Ok(header) = HeaderValue::from_str(self.auth.account_id()) {
            let _ = headers.insert("ChatGPT-Account-ID", header);
        }

        if self.auth.is_fedramp_account() {
            let _ = headers.insert("X-OpenAI-Fedramp", HeaderValue::from_static("true"));
        }
    }
}

// Some providers are meant to send no auth headers. Examples include local OSS
// providers and custom test providers with `requires_openai_auth = false`.
#[derive(Clone, Debug)]
struct UnauthenticatedAuthProvider;

impl AuthProvider for UnauthenticatedAuthProvider {
    fn add_auth_headers(&self, _headers: &mut HeaderMap) {}
}

pub fn unauthenticated_auth_provider() -> SharedAuthProvider {
    Arc::new(UnauthenticatedAuthProvider)
}

/// Returns the provider-scoped auth manager when this provider uses command-backed auth.
///
/// Providers without custom auth continue using the caller-supplied base manager, when present.
pub(crate) fn auth_manager_for_provider(
    auth_manager: Option<Arc<AuthManager>>,
    provider: &ModelProviderInfo,
) -> Option<Arc<AuthManager>> {
    match provider.auth.clone() {
        Some(config) => Some(AuthManager::external_bearer_only(config)),
        None => auth_manager,
    }
}

pub(crate) fn resolve_provider_auth(
    auth: Option<&CodexAuth>,
    provider: &ModelProviderInfo,
) -> ontocode_protocol::error::Result<SharedAuthProvider> {
    if let Some(auth) = bearer_auth_for_provider(provider)? {
        return Ok(Arc::new(auth));
    }

    Ok(match auth {
        Some(auth) => auth_provider_from_auth(auth),
        None => unauthenticated_auth_provider(),
    })
}

pub(crate) async fn resolve_provider_auth_with_manager(
    auth_manager: Option<&AuthManager>,
    auth: Option<&CodexAuth>,
    provider_id: &str,
    provider: &ModelProviderInfo,
) -> ontocode_protocol::error::Result<SharedAuthProvider> {
    resolve_provider_auth_with_manager_for_request(
        auth_manager,
        auth,
        provider_id,
        /*auth_profile_id*/ None,
        provider,
    )
    .await
}

pub(crate) async fn resolve_provider_auth_with_manager_for_request(
    auth_manager: Option<&AuthManager>,
    auth: Option<&CodexAuth>,
    provider_id: &str,
    auth_profile_id: Option<&str>,
    provider: &ModelProviderInfo,
) -> ontocode_protocol::error::Result<SharedAuthProvider> {
    if let Some(auth) = bearer_auth_for_provider(provider)? {
        return Ok(Arc::new(auth));
    }

    if let Some(auth_manager) = auth_manager
        && let Some(credential) = auth_manager
            .provider_oauth_credential_for_request_auth(provider_id, auth_profile_id)
            .await?
    {
        return Ok(Arc::new(
            BearerAuthProvider::from_provider_oauth_credential(&credential),
        ));
    }

    if let Some(auth) = auth {
        return Ok(auth_provider_from_auth(auth));
    }

    Ok(unauthenticated_auth_provider())
}

fn bearer_auth_for_provider(
    provider: &ModelProviderInfo,
) -> ontocode_protocol::error::Result<Option<BearerAuthProvider>> {
    if let Some(api_key) = provider.env_key.as_ref().and_then(|env_key| {
        std::env::var(env_key)
            .ok()
            .filter(|api_key| !api_key.trim().is_empty())
    }) {
        return Ok(Some(BearerAuthProvider::new(api_key)));
    }

    if let Some(token) = provider.experimental_bearer_token.clone() {
        return Ok(Some(BearerAuthProvider::new(token)));
    }

    Ok(None)
}

/// Builds request-header auth for a first-party Codex auth snapshot.
pub fn auth_provider_from_auth(auth: &CodexAuth) -> SharedAuthProvider {
    match auth {
        CodexAuth::AgentIdentity(auth) => {
            Arc::new(AgentIdentityAuthProvider { auth: auth.clone() })
        }
        CodexAuth::ApiKey(_) | CodexAuth::Chatgpt(_) | CodexAuth::ChatgptAuthTokens(_) => {
            Arc::new(BearerAuthProvider {
                token: auth.get_token().ok(),
                account_id: auth.get_account_id(),
                is_fedramp_account: auth.is_fedramp_account(),
            })
        }
    }
}

#[cfg(test)]
mod tests {

    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use http::HeaderMap;
    use http::header;
    use ontocode_login::AuthCredentialsStoreMode;
    use ontocode_login::AuthManager;
    use ontocode_login::CodexAuth;
    use ontocode_login::auth::upsert_provider_oauth_credential;
    use ontocode_model_provider_info::ModelProviderInfo;
    use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
    use ontocode_provider_auth::ProviderOAuthCredential;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    use ontocode_model_provider_info::WireApi;
    use ontocode_model_provider_info::create_oss_provider_with_base_url;

    use super::*;

    fn temp_dir_path(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "ontocode-model-provider-{}-{}-{}",
            label,
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ))
    }

    #[test]
    fn unauthenticated_auth_provider_adds_no_headers() {
        let provider =
            create_oss_provider_with_base_url("http://localhost:11434/v1", WireApi::Responses);
        let auth = resolve_provider_auth(/*auth*/ None, &provider).expect("auth should resolve");

        assert!(auth.to_auth_headers().is_empty());
    }

    #[tokio::test]
    async fn request_auth_uses_selected_provider_oauth_credential() {
        let codex_home = temp_dir_path("request-auth-exact");
        std::fs::create_dir_all(&codex_home).expect("temp dir should be created");
        let credential_one = ProviderOAuthCredential::new(
            "gemini",
            ProviderCredentialSourceKind::ExternalImport,
            "gemini:workspace-1",
            "gemini-access-token-1",
        );
        let credential_two = ProviderOAuthCredential::new(
            "gemini",
            ProviderCredentialSourceKind::ExternalImport,
            "gemini:workspace-2",
            "gemini-access-token-2",
        );

        upsert_provider_oauth_credential(
            codex_home.as_path(),
            AuthCredentialsStoreMode::File,
            credential_one,
        )
        .expect("first credential should persist");
        upsert_provider_oauth_credential(
            codex_home.as_path(),
            AuthCredentialsStoreMode::File,
            credential_two,
        )
        .expect("second credential should persist");

        let auth_manager = AuthManager::from_auth_for_testing_with_home(
            CodexAuth::from_api_key("openai-api-key"),
            codex_home.clone(),
        );
        let provider = ModelProviderInfo::create_gemini_provider();
        let auth = resolve_provider_auth_with_manager_for_request(
            Some(&auth_manager),
            /*auth*/ None,
            "gemini",
            Some("gemini:workspace-2"),
            &provider,
        )
        .await
        .expect("request auth should resolve");

        let mut headers = HeaderMap::new();
        auth.add_auth_headers(&mut headers);

        assert_eq!(
            headers
                .get(header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok()),
            Some("Bearer gemini-access-token-2")
        );
    }

    #[tokio::test]
    async fn request_auth_with_profile_id_still_prefers_provider_bearer_auth() {
        let provider = ModelProviderInfo {
            name: "Anthropic".to_string(),
            base_url: Some("https://api.anthropic.com/v1".to_string()),
            experimental_bearer_token: Some("provider-bearer-token".to_string()),
            requires_openai_auth: false,
            ..Default::default()
        };
        let codex_home = temp_dir_path("request-auth-bearer");
        std::fs::create_dir_all(&codex_home).expect("temp dir should be created");
        let auth_manager = AuthManager::from_auth_for_testing_with_home(
            CodexAuth::from_api_key("openai-api-key"),
            codex_home,
        );
        let auth = resolve_provider_auth_with_manager_for_request(
            Some(&auth_manager),
            Some(&CodexAuth::from_api_key("openai-api-key")),
            "anthropic",
            Some("anthropic:workspace-1"),
            &provider,
        )
        .await
        .expect("provider bearer auth should resolve");

        let mut headers = HeaderMap::new();
        auth.add_auth_headers(&mut headers);

        assert_eq!(
            headers
                .get(header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok()),
            Some("Bearer provider-bearer-token")
        );
    }
}
