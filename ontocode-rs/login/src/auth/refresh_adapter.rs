use std::sync::Arc;

use ontocode_protocol::credential_routing::ProviderCredentialAuthKind;
use ontocode_protocol::credential_routing::ProviderCredentialRoutingSummary;
use ontocode_protocol::credential_routing::ProviderCredentialRoutingView;
use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
use ontocode_provider_auth::ProviderCredentialRefreshAdapter;
use ontocode_provider_auth::ProviderCredentialRefreshDescriptor;
use ontocode_provider_auth::ProviderCredentialRefreshDescriptorFuture;
use ontocode_provider_auth::ProviderCredentialRefreshFailureKind;
use ontocode_provider_auth::ProviderCredentialRefreshFuture;
use ontocode_provider_auth::ProviderCredentialRefreshOutcome;
use ontocode_provider_auth::ProviderCredentialRefreshState;

use super::AuthManager;
use super::CodexAuth;
use super::RefreshTokenError;
use crate::auth::RefreshTokenFailedReason;

#[derive(Clone)]
pub struct AuthManagerRefreshAdapter {
    manager: Arc<AuthManager>,
}

impl AuthManagerRefreshAdapter {
    pub fn new(manager: Arc<AuthManager>) -> Self {
        Self { manager }
    }

    fn descriptor_for_auth(
        manager: &AuthManager,
        auth: CodexAuth,
    ) -> ProviderCredentialRefreshDescriptor {
        let state = match auth.clone() {
            CodexAuth::ApiKey(_) | CodexAuth::AgentIdentity(_) => {
                ProviderCredentialRefreshState::NonRefreshable
            }
            CodexAuth::Chatgpt(_) | CodexAuth::ChatgptAuthTokens(_) => {
                if manager.refresh_failure_for_auth(&auth).is_some() {
                    ProviderCredentialRefreshState::RefreshFailed
                } else if AuthManager::should_refresh_proactively(&auth) {
                    ProviderCredentialRefreshState::RefreshEligible
                } else {
                    ProviderCredentialRefreshState::RefreshHealthy
                }
            }
        };

        ProviderCredentialRefreshDescriptor {
            credential_key: auth_credential_id(&auth).to_string(),
            routing: routing_summary_for_auth(&auth),
            state,
            expires_at: None,
        }
    }
}

impl ProviderCredentialRefreshAdapter for AuthManagerRefreshAdapter {
    fn current_refresh_descriptor(&self) -> ProviderCredentialRefreshDescriptorFuture<'_> {
        let manager = Arc::clone(&self.manager);
        Box::pin(async move {
            manager
                .auth_cached()
                .map(|auth| Self::descriptor_for_auth(&manager, auth))
        })
    }

    fn refresh_if_eligible(&self) -> ProviderCredentialRefreshFuture<'_> {
        let manager = Arc::clone(&self.manager);
        Box::pin(async move {
            let Some(descriptor) = AuthManagerRefreshAdapter::new(Arc::clone(&manager))
                .current_refresh_descriptor()
                .await
            else {
                return ProviderCredentialRefreshOutcome::Skipped(
                    ProviderCredentialRefreshState::RefreshableButUnavailable,
                );
            };

            if !descriptor.state.is_eligible() {
                return ProviderCredentialRefreshOutcome::Skipped(descriptor.state);
            }

            match manager.refresh_token().await {
                Ok(()) => ProviderCredentialRefreshOutcome::Completed,
                Err(error) => ProviderCredentialRefreshOutcome::Failed {
                    kind: classify_refresh_error(&error),
                    detail: error.to_string(),
                },
            }
        })
    }
}

fn routing_summary_for_auth(auth: &CodexAuth) -> ProviderCredentialRoutingSummary {
    let mut view = ProviderCredentialRoutingView::new(
        "openai",
        ProviderCredentialSourceKind::FirstPartyLogin,
        ProviderCredentialAuthKind::OAuthBearer,
        auth_credential_id(auth),
    );
    view.account_id = auth.get_account_id();
    view.provenance = Some("auth_manager".to_string());

    if matches!(auth, CodexAuth::ApiKey(_) | CodexAuth::AgentIdentity(_)) {
        view.auth_kind = ProviderCredentialAuthKind::AccessToken;
    }

    view.to_summary()
}

fn auth_credential_id(auth: &CodexAuth) -> &'static str {
    match auth {
        CodexAuth::ApiKey(_) => "openai:api_key",
        CodexAuth::Chatgpt(_) => "openai:chatgpt",
        CodexAuth::ChatgptAuthTokens(_) => "openai:external_tokens",
        CodexAuth::AgentIdentity(_) => "openai:agent_identity",
    }
}

fn classify_refresh_error(error: &RefreshTokenError) -> ProviderCredentialRefreshFailureKind {
    match error {
        RefreshTokenError::Permanent(error) => match error.reason {
            RefreshTokenFailedReason::Expired => ProviderCredentialRefreshFailureKind::Expired,
            RefreshTokenFailedReason::Exhausted => ProviderCredentialRefreshFailureKind::Exhausted,
            RefreshTokenFailedReason::Revoked => ProviderCredentialRefreshFailureKind::Revoked,
            RefreshTokenFailedReason::Other => ProviderCredentialRefreshFailureKind::Unknown,
        },
        RefreshTokenError::Transient(error) => {
            if error.kind() == std::io::ErrorKind::TimedOut {
                ProviderCredentialRefreshFailureKind::Timeout
            } else {
                ProviderCredentialRefreshFailureKind::Transient
            }
        }
    }
}
