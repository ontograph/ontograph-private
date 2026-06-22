//! Redacted provider-credential routing metadata shared across auth owners.
//!
//! These types intentionally exclude raw secret material. They let existing
//! auth owners expose a normalized credential view for routing, diagnostics,
//! and future scheduler work without introducing a second persistence layer.

/// Redacted source kind for a provider credential view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderCredentialSourceKind {
    FirstPartyLogin,
    McpOAuth,
    ExternalImport,
}

/// Redacted auth kind for a provider credential view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderCredentialAuthKind {
    AccessToken,
    OAuthBearer,
}

/// Provider-neutral, redacted credential metadata used for routing decisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCredentialRoutingView {
    pub provider_name: String,
    pub source_kind: ProviderCredentialSourceKind,
    pub auth_kind: ProviderCredentialAuthKind,
    pub credential_id: String,
    pub account_id: Option<String>,
    pub endpoint: Option<String>,
    pub client_id: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<u64>,
    pub provenance: Option<String>,
}

/// Bounded redacted summary derived from a provider credential routing view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCredentialRoutingSummary {
    pub provider_name: String,
    pub source_kind: ProviderCredentialSourceKind,
    pub auth_kind: ProviderCredentialAuthKind,
    pub has_account_id: bool,
    pub has_endpoint: bool,
    pub has_client_id: bool,
    pub scope_count: usize,
    pub expires_at: Option<u64>,
    pub provenance: Option<String>,
}

impl ProviderCredentialRoutingView {
    pub fn new(
        provider_name: impl Into<String>,
        source_kind: ProviderCredentialSourceKind,
        auth_kind: ProviderCredentialAuthKind,
        credential_id: impl Into<String>,
    ) -> Self {
        Self {
            provider_name: provider_name.into(),
            source_kind,
            auth_kind,
            credential_id: credential_id.into(),
            account_id: None,
            endpoint: None,
            client_id: None,
            scopes: Vec::new(),
            expires_at: None,
            provenance: None,
        }
    }

    pub fn to_summary(&self) -> ProviderCredentialRoutingSummary {
        ProviderCredentialRoutingSummary {
            provider_name: self.provider_name.clone(),
            source_kind: self.source_kind,
            auth_kind: self.auth_kind,
            has_account_id: self.account_id.is_some(),
            has_endpoint: self.endpoint.is_some(),
            has_client_id: self.client_id.is_some(),
            scope_count: self.scopes.len(),
            expires_at: self.expires_at,
            provenance: self.provenance.clone(),
        }
    }
}

#[cfg(test)]
#[path = "credential_routing_tests.rs"]
mod tests;
