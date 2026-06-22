use std::sync::Arc;

use pretty_assertions::assert_eq;

use super::ProviderCredentialRefreshAdapter;
use super::ProviderCredentialRefreshDescriptor;
use super::ProviderCredentialRefreshDescriptorFuture;
use super::ProviderCredentialRefreshDiagnostics;
use super::ProviderCredentialRefreshFailureKind;
use super::ProviderCredentialRefreshFuture;
use super::ProviderCredentialRefreshOrchestrator;
use super::ProviderCredentialRefreshOutcome;
use super::ProviderCredentialRefreshState;
use super::ProviderOAuthCredentialSource;
use super::StaticProviderOAuthCredentialSource;
use crate::ProviderOAuthCredential;
use ontocode_protocol::credential_routing::ProviderCredentialAuthKind;
use ontocode_protocol::credential_routing::ProviderCredentialRoutingSummary;
use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;

#[derive(Clone)]
struct FakeRefreshAdapter {
    descriptor: Option<ProviderCredentialRefreshDescriptor>,
    outcome: ProviderCredentialRefreshOutcome,
}

impl ProviderCredentialRefreshAdapter for FakeRefreshAdapter {
    fn current_refresh_descriptor(&self) -> ProviderCredentialRefreshDescriptorFuture<'_> {
        let descriptor = self.descriptor.clone();
        Box::pin(async move { descriptor })
    }

    fn refresh_if_eligible(&self) -> ProviderCredentialRefreshFuture<'_> {
        let outcome = self.outcome.clone();
        Box::pin(async move { outcome })
    }
}

fn sample_descriptor(state: ProviderCredentialRefreshState) -> ProviderCredentialRefreshDescriptor {
    ProviderCredentialRefreshDescriptor {
        credential_key: "claude:acct_123".to_string(),
        routing: ProviderCredentialRoutingSummary {
            provider_name: "claude".to_string(),
            source_kind: ProviderCredentialSourceKind::ExternalImport,
            auth_kind: ProviderCredentialAuthKind::OAuthBearer,
            has_account_id: true,
            has_endpoint: true,
            has_client_id: true,
            scope_count: 2,
            expires_at: Some(1234),
            provenance: Some("claude-code".to_string()),
        },
        state,
        expires_at: Some(1234),
    }
}

fn sample_oauth_credential(
    expires_at: Option<u64>,
    refresh_token: Option<&str>,
) -> ProviderOAuthCredential {
    let mut credential = ProviderOAuthCredential::new(
        "claude",
        ProviderCredentialSourceKind::ExternalImport,
        "claude:acct_123",
        "secret-access-token",
    );
    credential.account_id = Some("acct_123".to_string());
    credential.endpoint = Some("https://example.test/token".to_string());
    credential.client_id = Some("client-id".to_string());
    credential.scopes = vec!["scope-a".to_string(), "scope-b".to_string()];
    credential.expires_at = expires_at;
    credential.provenance = Some("claude-code".to_string());
    credential.refresh_token = refresh_token.map(str::to_string);
    credential
}

#[test]
fn static_provider_oauth_credential_source_returns_cloned_current_credential() {
    let credential = sample_oauth_credential(Some(1234), Some("refresh-token"));
    let source = StaticProviderOAuthCredentialSource::new(credential.clone());

    let mut current = source.current_credential();
    current.access_token = "changed-access-token".to_string();

    assert_eq!(source.current_credential(), credential);
}

#[test]
fn static_provider_oauth_credential_source_exposes_redacted_routing_summary_and_descriptor() {
    let source = StaticProviderOAuthCredentialSource::new(sample_oauth_credential(
        Some(1234),
        Some("refresh-token"),
    ));

    let summary = source.current_routing_summary();
    let descriptor = source.current_refresh_descriptor();
    let summary_debug = format!("{summary:?}");
    let descriptor_debug = format!("{descriptor:?}");

    assert_eq!(
        summary,
        ProviderCredentialRoutingSummary {
            provider_name: "claude".to_string(),
            source_kind: ProviderCredentialSourceKind::ExternalImport,
            auth_kind: ProviderCredentialAuthKind::OAuthBearer,
            has_account_id: true,
            has_endpoint: true,
            has_client_id: true,
            scope_count: 2,
            expires_at: Some(1234),
            provenance: Some("claude-code".to_string()),
        }
    );
    assert_eq!(descriptor.credential_key, "claude:acct_123".to_string());
    assert_eq!(descriptor.routing, summary);
    assert_eq!(
        descriptor.state,
        ProviderCredentialRefreshState::RefreshEligible
    );
    assert_eq!(descriptor.expires_at, Some(1234));
    assert!(!summary_debug.contains("secret-access-token"));
    assert!(!summary_debug.contains("refresh-token"));
    assert!(!descriptor_debug.contains("secret-access-token"));
    assert!(!descriptor_debug.contains("refresh-token"));
}

#[tokio::test]
async fn collect_descriptors_skips_absent_adapters() {
    let eligible = FakeRefreshAdapter {
        descriptor: Some(sample_descriptor(
            ProviderCredentialRefreshState::RefreshEligible,
        )),
        outcome: ProviderCredentialRefreshOutcome::Completed,
    };
    let absent = FakeRefreshAdapter {
        descriptor: None,
        outcome: ProviderCredentialRefreshOutcome::Skipped(
            ProviderCredentialRefreshState::RefreshableButUnavailable,
        ),
    };

    assert_eq!(
        ProviderCredentialRefreshOrchestrator::collect_descriptors([
            &eligible as &dyn ProviderCredentialRefreshAdapter,
            &absent as &dyn ProviderCredentialRefreshAdapter,
        ])
        .await,
        vec![sample_descriptor(
            ProviderCredentialRefreshState::RefreshEligible
        )]
    );
}

#[tokio::test]
async fn refresh_eligible_preserves_redacted_routing_records() {
    let mut orchestrator = ProviderCredentialRefreshOrchestrator::new();
    let eligible = Arc::new(FakeRefreshAdapter {
        descriptor: Some(sample_descriptor(
            ProviderCredentialRefreshState::RefreshEligible,
        )),
        outcome: ProviderCredentialRefreshOutcome::Completed,
    });
    let healthy = Arc::new(FakeRefreshAdapter {
        descriptor: Some(sample_descriptor(
            ProviderCredentialRefreshState::RefreshHealthy,
        )),
        outcome: ProviderCredentialRefreshOutcome::Skipped(
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
    });

    assert_eq!(
        orchestrator
            .refresh_eligible([
                eligible.as_ref() as &dyn ProviderCredentialRefreshAdapter,
                healthy.as_ref() as &dyn ProviderCredentialRefreshAdapter,
            ])
            .await,
        vec![
            super::ProviderCredentialRefreshRecord {
                credential_key: "claude:acct_123".to_string(),
                routing: sample_descriptor(ProviderCredentialRefreshState::RefreshEligible).routing,
                outcome: ProviderCredentialRefreshOutcome::Completed,
                diagnostics: ProviderCredentialRefreshDiagnostics {
                    state: ProviderCredentialRefreshState::RefreshHealthy,
                    consecutive_failures: 0,
                    next_retry_at: None,
                    last_failure_kind: None,
                    last_error_detail: None,
                },
            },
            super::ProviderCredentialRefreshRecord {
                credential_key: "claude:acct_123".to_string(),
                routing: sample_descriptor(ProviderCredentialRefreshState::RefreshHealthy).routing,
                outcome: ProviderCredentialRefreshOutcome::Skipped(
                    ProviderCredentialRefreshState::RefreshHealthy,
                ),
                diagnostics: ProviderCredentialRefreshDiagnostics {
                    state: ProviderCredentialRefreshState::RefreshHealthy,
                    consecutive_failures: 0,
                    next_retry_at: None,
                    last_failure_kind: None,
                    last_error_detail: None,
                },
            }
        ]
    );
}

#[tokio::test]
async fn refresh_failures_record_bounded_detail_and_backoff() {
    let mut orchestrator = ProviderCredentialRefreshOrchestrator::new();
    let failing = FakeRefreshAdapter {
        descriptor: Some(sample_descriptor(
            ProviderCredentialRefreshState::RefreshEligible,
        )),
        outcome: ProviderCredentialRefreshOutcome::Failed {
            kind: ProviderCredentialRefreshFailureKind::Transient,
            detail: "x".repeat(200),
        },
    };

    let records = orchestrator
        .refresh_eligible([&failing as &dyn ProviderCredentialRefreshAdapter])
        .await;
    let record = records.first().expect("refresh record should exist");

    assert_eq!(
        record.diagnostics.state,
        ProviderCredentialRefreshState::RefreshFailed
    );
    assert_eq!(record.diagnostics.consecutive_failures, 1);
    assert_eq!(
        record.diagnostics.last_failure_kind,
        Some(ProviderCredentialRefreshFailureKind::Transient)
    );
    assert!(record.diagnostics.next_retry_at.is_some());
    assert_eq!(
        record
            .diagnostics
            .last_error_detail
            .as_ref()
            .expect("detail should be present")
            .chars()
            .count(),
        161
    );
}

#[tokio::test]
async fn refresh_backoff_suppresses_immediate_retry() {
    let mut orchestrator = ProviderCredentialRefreshOrchestrator::new();
    let failing = FakeRefreshAdapter {
        descriptor: Some(sample_descriptor(
            ProviderCredentialRefreshState::RefreshEligible,
        )),
        outcome: ProviderCredentialRefreshOutcome::Failed {
            kind: ProviderCredentialRefreshFailureKind::Timeout,
            detail: "temporary timeout".to_string(),
        },
    };

    let suppressed = FakeRefreshAdapter {
        descriptor: Some(sample_descriptor(
            ProviderCredentialRefreshState::RefreshEligible,
        )),
        outcome: ProviderCredentialRefreshOutcome::Completed,
    };

    orchestrator
        .refresh_eligible([&failing as &dyn ProviderCredentialRefreshAdapter])
        .await;
    let records = orchestrator
        .refresh_eligible([&suppressed as &dyn ProviderCredentialRefreshAdapter])
        .await;

    assert_eq!(
        records,
        vec![super::ProviderCredentialRefreshRecord {
            credential_key: "claude:acct_123".to_string(),
            routing: sample_descriptor(ProviderCredentialRefreshState::RefreshEligible).routing,
            outcome: ProviderCredentialRefreshOutcome::Skipped(
                ProviderCredentialRefreshState::RefreshSuppressed,
            ),
            diagnostics: ProviderCredentialRefreshDiagnostics {
                state: ProviderCredentialRefreshState::RefreshSuppressed,
                consecutive_failures: 1,
                next_retry_at: records[0].diagnostics.next_retry_at,
                last_failure_kind: Some(ProviderCredentialRefreshFailureKind::Timeout),
                last_error_detail: Some("temporary timeout".to_string()),
            },
        }]
    );
}
