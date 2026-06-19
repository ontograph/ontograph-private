use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use ontocode_protocol::credential_routing::ProviderCredentialRoutingSummary;

mod oauth_credential;
mod redacted;
mod route;

pub use oauth_credential::ProviderOAuthCredential;
pub use oauth_credential::ProviderOAuthCredentialRecord;
pub use oauth_credential::ProviderOAuthCredentialSourceKindRecord;
pub use redacted::ProviderAuthRedactedError;
pub use route::ProviderAuthKind;
pub use route::ProviderRoute;
pub use route::derive_provider_route;
pub use route::provider_auth_kind_for_provider_info;

/// Shared source contract for imported third-party OAuth credentials.
///
/// Implementations own their storage or in-memory state and expose the current
/// credential material plus redacted routing and refresh projections.
pub trait ProviderOAuthCredentialSource: Send + Sync {
    fn current_credential(&self) -> ProviderOAuthCredential;

    fn current_routing_summary(&self) -> ProviderCredentialRoutingSummary {
        self.current_credential().to_routing_summary()
    }

    fn current_refresh_descriptor(&self) -> ProviderCredentialRefreshDescriptor {
        self.current_credential().to_refresh_descriptor()
    }
}

/// Simple in-memory provider OAuth source for tests and lightweight callers.
#[derive(Clone, Debug)]
pub struct StaticProviderOAuthCredentialSource {
    credential: ProviderOAuthCredential,
}

impl StaticProviderOAuthCredentialSource {
    pub fn new(credential: ProviderOAuthCredential) -> Self {
        Self { credential }
    }
}

impl ProviderOAuthCredentialSource for StaticProviderOAuthCredentialSource {
    fn current_credential(&self) -> ProviderOAuthCredential {
        self.credential.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderCredentialRefreshState {
    NonRefreshable,
    RefreshableButUnavailable,
    RefreshSuppressed,
    RefreshEligible,
    RefreshHealthy,
    RefreshFailed,
}

impl ProviderCredentialRefreshState {
    pub fn is_eligible(self) -> bool {
        self == Self::RefreshEligible
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderCredentialRefreshFailureKind {
    Expired,
    Exhausted,
    Revoked,
    Timeout,
    Transient,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCredentialRefreshDescriptor {
    pub credential_key: String,
    pub routing: ProviderCredentialRoutingSummary,
    pub state: ProviderCredentialRefreshState,
    pub expires_at: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderCredentialRefreshOutcome {
    Completed,
    Skipped(ProviderCredentialRefreshState),
    Failed {
        kind: ProviderCredentialRefreshFailureKind,
        detail: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCredentialRefreshDiagnostics {
    pub state: ProviderCredentialRefreshState,
    pub consecutive_failures: u32,
    pub next_retry_at: Option<u64>,
    pub last_failure_kind: Option<ProviderCredentialRefreshFailureKind>,
    pub last_error_detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCredentialRefreshRecord {
    pub credential_key: String,
    pub routing: ProviderCredentialRoutingSummary,
    pub outcome: ProviderCredentialRefreshOutcome,
    pub diagnostics: ProviderCredentialRefreshDiagnostics,
}

pub type ProviderCredentialRefreshFuture<'a> =
    Pin<Box<dyn Future<Output = ProviderCredentialRefreshOutcome> + Send + 'a>>;
pub type ProviderCredentialRefreshDescriptorFuture<'a> =
    Pin<Box<dyn Future<Output = Option<ProviderCredentialRefreshDescriptor>> + Send + 'a>>;

/// Shared contract for refreshable provider-credential owners.
///
/// Implementations keep ownership of their persistence and provider-specific
/// refresh logic. The orchestrator only consumes redacted descriptors and
/// triggers refresh work through this contract.
pub trait ProviderCredentialRefreshAdapter: Send + Sync {
    fn current_refresh_descriptor(&self) -> ProviderCredentialRefreshDescriptorFuture<'_>;

    fn refresh_if_eligible(&self) -> ProviderCredentialRefreshFuture<'_>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProviderCredentialRefreshMemory {
    consecutive_failures: u32,
    next_retry_at: Option<u64>,
    last_failure_kind: Option<ProviderCredentialRefreshFailureKind>,
    last_error_detail: Option<String>,
}

pub struct ProviderCredentialRefreshOrchestrator {
    diagnostics_by_key: std::collections::HashMap<String, ProviderCredentialRefreshMemory>,
}

impl Default for ProviderCredentialRefreshOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderCredentialRefreshOrchestrator {
    pub fn new() -> Self {
        Self {
            diagnostics_by_key: std::collections::HashMap::new(),
        }
    }

    pub async fn collect_descriptors<'a>(
        adapters: impl IntoIterator<Item = &'a dyn ProviderCredentialRefreshAdapter>,
    ) -> Vec<ProviderCredentialRefreshDescriptor> {
        let mut descriptors = Vec::new();

        for adapter in adapters {
            if let Some(descriptor) = adapter.current_refresh_descriptor().await {
                descriptors.push(descriptor);
            }
        }

        descriptors
    }

    pub async fn refresh_eligible<'a>(
        &mut self,
        adapters: impl IntoIterator<Item = &'a dyn ProviderCredentialRefreshAdapter>,
    ) -> Vec<ProviderCredentialRefreshRecord> {
        let mut records = Vec::new();

        for adapter in adapters {
            let Some(descriptor) = adapter.current_refresh_descriptor().await else {
                continue;
            };

            if self.is_backoff_active(&descriptor.credential_key) {
                let diagnostics = self.diagnostics_for(
                    &descriptor.credential_key,
                    ProviderCredentialRefreshState::RefreshSuppressed,
                );
                records.push(ProviderCredentialRefreshRecord {
                    credential_key: descriptor.credential_key,
                    routing: descriptor.routing,
                    outcome: ProviderCredentialRefreshOutcome::Skipped(
                        ProviderCredentialRefreshState::RefreshSuppressed,
                    ),
                    diagnostics,
                });
                continue;
            }

            let outcome = adapter.refresh_if_eligible().await;
            let diagnostics = self.update_diagnostics(&descriptor, &outcome);
            records.push(ProviderCredentialRefreshRecord {
                credential_key: descriptor.credential_key,
                routing: descriptor.routing,
                outcome,
                diagnostics,
            });
        }

        records
    }

    fn is_backoff_active(&self, credential_key: &str) -> bool {
        self.diagnostics_by_key
            .get(credential_key)
            .and_then(|memory| memory.next_retry_at)
            .is_some_and(|next_retry_at| next_retry_at > now_millis())
    }

    fn diagnostics_for(
        &self,
        credential_key: &str,
        state: ProviderCredentialRefreshState,
    ) -> ProviderCredentialRefreshDiagnostics {
        let memory = self.diagnostics_by_key.get(credential_key);
        ProviderCredentialRefreshDiagnostics {
            state,
            consecutive_failures: memory.map_or(0, |entry| entry.consecutive_failures),
            next_retry_at: memory.and_then(|entry| entry.next_retry_at),
            last_failure_kind: memory.and_then(|entry| entry.last_failure_kind),
            last_error_detail: memory.and_then(|entry| entry.last_error_detail.clone()),
        }
    }

    fn update_diagnostics(
        &mut self,
        descriptor: &ProviderCredentialRefreshDescriptor,
        outcome: &ProviderCredentialRefreshOutcome,
    ) -> ProviderCredentialRefreshDiagnostics {
        match outcome {
            ProviderCredentialRefreshOutcome::Completed => {
                self.diagnostics_by_key
                    .remove(descriptor.credential_key.as_str());
                ProviderCredentialRefreshDiagnostics {
                    state: ProviderCredentialRefreshState::RefreshHealthy,
                    consecutive_failures: 0,
                    next_retry_at: None,
                    last_failure_kind: None,
                    last_error_detail: None,
                }
            }
            ProviderCredentialRefreshOutcome::Skipped(state) => {
                self.diagnostics_for(descriptor.credential_key.as_str(), *state)
            }
            ProviderCredentialRefreshOutcome::Failed { kind, detail } => {
                let next_retry_at =
                    backoff_retry_at(*kind, self.next_failure_count(&descriptor.credential_key));
                let consecutive_failures = self.next_failure_count(&descriptor.credential_key);
                let detail = bounded_detail(detail);
                self.diagnostics_by_key.insert(
                    descriptor.credential_key.clone(),
                    ProviderCredentialRefreshMemory {
                        consecutive_failures,
                        next_retry_at,
                        last_failure_kind: Some(*kind),
                        last_error_detail: Some(detail.clone()),
                    },
                );
                ProviderCredentialRefreshDiagnostics {
                    state: ProviderCredentialRefreshState::RefreshFailed,
                    consecutive_failures,
                    next_retry_at,
                    last_failure_kind: Some(*kind),
                    last_error_detail: Some(detail),
                }
            }
        }
    }

    fn next_failure_count(&self, credential_key: &str) -> u32 {
        self.diagnostics_by_key
            .get(credential_key)
            .map_or(1, |memory| memory.consecutive_failures.saturating_add(1))
    }
}

fn bounded_detail(detail: &str) -> String {
    const MAX_CHARS: usize = 160;
    if detail.chars().count() <= MAX_CHARS {
        return detail.to_string();
    }
    let mut truncated = detail.chars().take(MAX_CHARS).collect::<String>();
    truncated.push('…');
    truncated
}

fn backoff_retry_at(
    kind: ProviderCredentialRefreshFailureKind,
    consecutive_failures: u32,
) -> Option<u64> {
    let backoff = match kind {
        ProviderCredentialRefreshFailureKind::Expired
        | ProviderCredentialRefreshFailureKind::Exhausted
        | ProviderCredentialRefreshFailureKind::Revoked => None,
        ProviderCredentialRefreshFailureKind::Timeout
        | ProviderCredentialRefreshFailureKind::Transient
        | ProviderCredentialRefreshFailureKind::Unknown => {
            let seconds = consecutive_failures.min(4) as u64 * 30;
            Some(Duration::from_secs(seconds.max(30)))
        }
    }?;
    Some(now_millis().saturating_add(backoff.as_millis() as u64))
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_millis() as u64
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
