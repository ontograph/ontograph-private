use oauth2::TokenResponse;
use ontocode_provider_auth::ProviderCredentialRefreshAdapter;
use ontocode_provider_auth::ProviderCredentialRefreshDescriptor;
use ontocode_provider_auth::ProviderCredentialRefreshDescriptorFuture;
use ontocode_provider_auth::ProviderCredentialRefreshFailureKind;
use ontocode_provider_auth::ProviderCredentialRefreshFuture;
use ontocode_provider_auth::ProviderCredentialRefreshOutcome;
use ontocode_provider_auth::ProviderCredentialRefreshState;

use crate::oauth::OAuthPersistor;
use crate::oauth::token_needs_refresh;

#[derive(Clone)]
pub(crate) struct OAuthPersistorRefreshAdapter {
    persistor: OAuthPersistor,
}

impl OAuthPersistorRefreshAdapter {
    pub(crate) fn new(persistor: OAuthPersistor) -> Self {
        Self { persistor }
    }
}

impl ProviderCredentialRefreshAdapter for OAuthPersistorRefreshAdapter {
    fn current_refresh_descriptor(&self) -> ProviderCredentialRefreshDescriptorFuture<'_> {
        let persistor = self.persistor.clone();
        Box::pin(async move {
            let tokens = persistor.current_tokens().await?;
            let state = if tokens.token_response.0.refresh_token().is_none() {
                ProviderCredentialRefreshState::RefreshableButUnavailable
            } else if token_needs_refresh(tokens.expires_at) {
                ProviderCredentialRefreshState::RefreshEligible
            } else {
                ProviderCredentialRefreshState::RefreshHealthy
            };

            Some(ProviderCredentialRefreshDescriptor {
                credential_key: tokens.server_name.clone(),
                routing: tokens.to_provider_credential_routing_view().to_summary(),
                state,
                expires_at: tokens.expires_at,
            })
        })
    }

    fn refresh_if_eligible(&self) -> ProviderCredentialRefreshFuture<'_> {
        let persistor = self.persistor.clone();
        Box::pin(async move {
            let Some(descriptor) = OAuthPersistorRefreshAdapter::new(persistor.clone())
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

            match persistor.refresh_if_needed().await {
                Ok(()) => ProviderCredentialRefreshOutcome::Completed,
                Err(error) => ProviderCredentialRefreshOutcome::Failed {
                    kind: classify_refresh_error(error.as_ref()),
                    detail: error.to_string(),
                },
            }
        })
    }
}

fn classify_refresh_error(
    error: &(dyn std::error::Error + 'static),
) -> ProviderCredentialRefreshFailureKind {
    if error.to_string().contains("timed out") {
        ProviderCredentialRefreshFailureKind::Timeout
    } else {
        ProviderCredentialRefreshFailureKind::Transient
    }
}
