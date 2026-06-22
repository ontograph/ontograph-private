use std::fmt;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use ontocode_protocol::credential_routing::ProviderCredentialAuthKind;
use ontocode_protocol::credential_routing::ProviderCredentialRoutingSummary;
use ontocode_protocol::credential_routing::ProviderCredentialRoutingView;
use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
use serde::Deserialize;
use serde::Serialize;

use crate::ProviderCredentialRefreshDescriptor;
use crate::ProviderCredentialRefreshState;

/// Canonical internal OAuth credential record shared across provider auth owners.
///
/// This type carries secret-bearing OAuth material for internal projection between
/// existing auth owners, refresh orchestration, and runtime auth builders. It is
/// intentionally not a persistence authority and must not be exposed directly in
/// user-visible diagnostics.
#[derive(Clone, PartialEq, Eq)]
pub struct ProviderOAuthCredential {
    pub provider_name: String,
    pub source_kind: ProviderCredentialSourceKind,
    pub credential_id: String,
    pub account_id: Option<String>,
    pub endpoint: Option<String>,
    pub client_id: Option<String>,
    pub token_endpoint: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<u64>,
    pub provenance: Option<String>,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

/// Serializable on-disk source kind for a provider OAuth credential record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderOAuthCredentialSourceKindRecord {
    FirstPartyLogin,
    McpOAuth,
    ExternalImport,
}

impl From<ProviderCredentialSourceKind> for ProviderOAuthCredentialSourceKindRecord {
    fn from(value: ProviderCredentialSourceKind) -> Self {
        match value {
            ProviderCredentialSourceKind::FirstPartyLogin => Self::FirstPartyLogin,
            ProviderCredentialSourceKind::McpOAuth => Self::McpOAuth,
            ProviderCredentialSourceKind::ExternalImport => Self::ExternalImport,
        }
    }
}

impl From<ProviderOAuthCredentialSourceKindRecord> for ProviderCredentialSourceKind {
    fn from(value: ProviderOAuthCredentialSourceKindRecord) -> Self {
        match value {
            ProviderOAuthCredentialSourceKindRecord::FirstPartyLogin => Self::FirstPartyLogin,
            ProviderOAuthCredentialSourceKindRecord::McpOAuth => Self::McpOAuth,
            ProviderOAuthCredentialSourceKindRecord::ExternalImport => Self::ExternalImport,
        }
    }
}

/// Serializable provider OAuth credential record stored in `auth.json`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderOAuthCredentialRecord {
    pub provider_name: String,
    pub source_kind: ProviderOAuthCredentialSourceKindRecord,
    pub credential_id: String,
    pub account_id: Option<String>,
    pub endpoint: Option<String>,
    pub client_id: Option<String>,
    pub token_endpoint: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<u64>,
    pub provenance: Option<String>,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

impl fmt::Debug for ProviderOAuthCredentialRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProviderOAuthCredentialRecord")
            .field("provider_name", &self.provider_name)
            .field("source_kind", &self.source_kind)
            .field("credential_id", &self.credential_id)
            .field("account_id", &self.account_id)
            .field("endpoint", &self.endpoint)
            .field("client_id", &self.client_id)
            .field("token_endpoint", &self.token_endpoint)
            .field("scopes", &self.scopes)
            .field("expires_at", &self.expires_at)
            .field("provenance", &self.provenance)
            .field("access_token", &"<redacted>")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

impl ProviderOAuthCredentialRecord {
    pub fn provider_id(&self) -> &str {
        &self.provider_name
    }

    pub fn matches_provider_id(&self, provider_id: &str, credential_id: &str) -> bool {
        self.provider_id() == provider_id && self.credential_id == credential_id
    }

    pub fn matches_provider_credential(&self, provider_id: &str, credential_id: &str) -> bool {
        self.matches_provider_id(provider_id, credential_id)
    }
}

impl From<&ProviderOAuthCredential> for ProviderOAuthCredentialRecord {
    fn from(credential: &ProviderOAuthCredential) -> Self {
        Self {
            provider_name: credential.provider_name.clone(),
            source_kind: credential.source_kind.into(),
            credential_id: credential.credential_id.clone(),
            account_id: credential.account_id.clone(),
            endpoint: credential.endpoint.clone(),
            client_id: credential.client_id.clone(),
            token_endpoint: credential.token_endpoint.clone(),
            scopes: credential.scopes.clone(),
            expires_at: credential.expires_at,
            provenance: credential.provenance.clone(),
            access_token: credential.access_token.clone(),
            refresh_token: credential.refresh_token.clone(),
        }
    }
}

impl From<ProviderOAuthCredentialRecord> for ProviderOAuthCredential {
    fn from(record: ProviderOAuthCredentialRecord) -> Self {
        Self {
            provider_name: record.provider_name,
            source_kind: record.source_kind.into(),
            credential_id: record.credential_id,
            account_id: record.account_id,
            endpoint: record.endpoint,
            client_id: record.client_id,
            token_endpoint: record.token_endpoint,
            scopes: record.scopes,
            expires_at: record.expires_at,
            provenance: record.provenance,
            access_token: record.access_token,
            refresh_token: record.refresh_token,
        }
    }
}

impl fmt::Debug for ProviderOAuthCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProviderOAuthCredential")
            .field("provider_name", &self.provider_name)
            .field("source_kind", &self.source_kind)
            .field("credential_id", &self.credential_id)
            .field("account_id", &self.account_id)
            .field("endpoint", &self.endpoint)
            .field("client_id", &self.client_id)
            .field("token_endpoint", &self.token_endpoint)
            .field("scopes", &self.scopes)
            .field("expires_at", &self.expires_at)
            .field("provenance", &self.provenance)
            .field("access_token", &"<redacted>")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

impl ProviderOAuthCredential {
    pub fn provider_id(&self) -> &str {
        &self.provider_name
    }

    pub fn new(
        provider_name: impl Into<String>,
        source_kind: ProviderCredentialSourceKind,
        credential_id: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        Self {
            provider_name: provider_name.into(),
            source_kind,
            credential_id: credential_id.into(),
            account_id: None,
            endpoint: None,
            client_id: None,
            token_endpoint: None,
            scopes: Vec::new(),
            expires_at: None,
            provenance: None,
            access_token: access_token.into(),
            refresh_token: None,
        }
    }

    pub fn is_refreshable(&self) -> bool {
        self.refresh_token.is_some()
    }

    pub fn refresh_state(&self) -> ProviderCredentialRefreshState {
        if !self.is_refreshable() {
            return ProviderCredentialRefreshState::NonRefreshable;
        }

        if token_needs_refresh(self.expires_at) {
            ProviderCredentialRefreshState::RefreshEligible
        } else {
            ProviderCredentialRefreshState::RefreshHealthy
        }
    }

    pub fn to_routing_view(&self) -> ProviderCredentialRoutingView {
        let mut view = ProviderCredentialRoutingView::new(
            self.provider_name.clone(),
            self.source_kind,
            ProviderCredentialAuthKind::OAuthBearer,
            self.credential_id.clone(),
        );
        view.account_id = self.account_id.clone();
        view.endpoint = self.endpoint.clone();
        view.client_id = self.client_id.clone();
        view.scopes = self.scopes.clone();
        view.expires_at = self.expires_at;
        view.provenance = self.provenance.clone();
        view
    }

    pub fn to_routing_summary(&self) -> ProviderCredentialRoutingSummary {
        self.to_routing_view().to_summary()
    }

    pub fn to_refresh_descriptor(&self) -> ProviderCredentialRefreshDescriptor {
        ProviderCredentialRefreshDescriptor {
            credential_key: self.credential_id.clone(),
            routing: self.to_routing_summary(),
            state: self.refresh_state(),
            expires_at: self.expires_at,
        }
    }
}

const REFRESH_SKEW_MILLIS: u64 = 30_000;

fn token_needs_refresh(expires_at: Option<u64>) -> bool {
    let Some(expires_at) = expires_at else {
        return false;
    };

    now_millis().saturating_add(REFRESH_SKEW_MILLIS) >= expires_at
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_millis() as u64
}

#[cfg(test)]
#[path = "oauth_credential_tests.rs"]
mod tests;
