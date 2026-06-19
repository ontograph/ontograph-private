use std::env;

use ontocode_login::AuthManager;
use ontocode_model_provider_info::GEMINI_CLI_PROVIDER_ID;
use ontocode_model_provider_info::ModelProviderInfo;
use ontocode_provider_auth::ProviderAuthRedactedError;

const API_KEY_ACTIVE_LABEL: &str = "API key";
const GEMINI_CLI_DISABLED_REASON: &str = "Gemini CLI runtime is not available yet.";
const NOT_CONFIGURED_REASON: &str = "Not configured";

/// Bounded provider-auth readiness row for status views.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderAuthStatusRow {
    pub provider_id: String,
    pub display_label: String,
    pub state: ProviderAuthStatusState,
    pub active_source_label: Option<String>,
    pub redacted_account_label: Option<String>,
    pub disabled_reason: Option<String>,
}

/// Compact provider-auth readiness states rendered by the status contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProviderAuthStatusState {
    Ready,
    NotConfigured,
    ApiKeyActive,
    Blocked,
    ErrorRedacted,
}

impl ProviderAuthStatusRow {
    pub fn builder(
        provider_id: impl Into<String>,
        display_label: impl Into<String>,
    ) -> ProviderAuthStatusRowBuilder {
        ProviderAuthStatusRowBuilder::new(provider_id, display_label)
    }

    pub fn from_provider(
        provider_id: impl Into<String>,
        provider: &ModelProviderInfo,
        auth_manager: Option<&AuthManager>,
    ) -> Self {
        ProviderAuthStatusRowBuilder::from_provider(provider_id, provider, auth_manager).build()
    }
}

/// Builder for a bounded provider-auth readiness row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderAuthStatusRowBuilder {
    row: ProviderAuthStatusRow,
}

impl ProviderAuthStatusRowBuilder {
    pub fn new(provider_id: impl Into<String>, display_label: impl Into<String>) -> Self {
        Self {
            row: ProviderAuthStatusRow {
                provider_id: provider_id.into(),
                display_label: display_label.into(),
                state: ProviderAuthStatusState::NotConfigured,
                active_source_label: None,
                redacted_account_label: None,
                disabled_reason: Some(NOT_CONFIGURED_REASON.to_string()),
            },
        }
    }

    pub fn ready(mut self) -> Self {
        self.row.state = ProviderAuthStatusState::Ready;
        self.row.active_source_label = None;
        self.row.redacted_account_label = None;
        self.row.disabled_reason = None;
        self
    }

    pub fn not_configured(mut self, reason: impl Into<String>) -> Self {
        self.row.state = ProviderAuthStatusState::NotConfigured;
        self.row.active_source_label = None;
        self.row.redacted_account_label = None;
        self.row.disabled_reason = Some(reason.into());
        self
    }

    pub fn api_key_active(mut self) -> Self {
        self.row.state = ProviderAuthStatusState::ApiKeyActive;
        self.row.active_source_label = Some(API_KEY_ACTIVE_LABEL.to_string());
        self.row.redacted_account_label = None;
        self.row.disabled_reason = None;
        self
    }

    pub fn redacted_account_label(mut self, label: impl Into<String>) -> Self {
        self.row.redacted_account_label = Some(label.into());
        self
    }

    pub fn blocked(mut self, reason: impl Into<String>) -> Self {
        self.row.state = ProviderAuthStatusState::Blocked;
        self.row.active_source_label = None;
        self.row.redacted_account_label = None;
        self.row.disabled_reason = Some(reason.into());
        self
    }

    pub fn error_redacted(mut self, reason: ProviderAuthRedactedError) -> Self {
        self.row.state = ProviderAuthStatusState::ErrorRedacted;
        self.row.active_source_label = None;
        self.row.redacted_account_label = None;
        self.row.disabled_reason = Some(reason.to_string());
        self
    }

    pub fn build(self) -> ProviderAuthStatusRow {
        self.row
    }

    pub fn from_provider(
        provider_id: impl Into<String>,
        provider: &ModelProviderInfo,
        auth_manager: Option<&AuthManager>,
    ) -> Self {
        let provider_id = provider_id.into();
        let builder = Self::new(provider_id.clone(), provider.name.clone());

        if provider_id == GEMINI_CLI_PROVIDER_ID {
            return builder.blocked(GEMINI_CLI_DISABLED_REASON);
        }

        if let Some(env_key) = provider.env_key.as_deref() {
            match env::var(env_key) {
                Ok(api_key) if !api_key.trim().is_empty() => {
                    return builder.api_key_active();
                }
                Ok(_) | Err(env::VarError::NotPresent) => {}
                Err(err) => {
                    return builder.error_redacted(ProviderAuthRedactedError::new(format!(
                        "{} API key lookup failed: {err}",
                        provider.name
                    )));
                }
            }
        }

        let _ = auth_manager;

        if provider.has_command_auth() {
            return builder.ready();
        }

        if provider.requires_openai_auth || provider.env_key.is_some() {
            return builder.not_configured(format!("{} auth is not configured", provider.name));
        }

        builder.ready()
    }
}

#[cfg(test)]
#[path = "auth_status_tests.rs"]
mod tests;
