use pretty_assertions::assert_eq;

use crate::schedule::ProviderCredentialCandidate;
use crate::schedule::ProviderCredentialCandidateStatus;
use crate::schedule::ProviderCredentialScheduleDecision;
use crate::schedule::ProviderCredentialScheduleRequest;
use crate::schedule::ProviderCredentialScheduler;
use crate::schedule::ProviderCredentialSchedulingPolicy;
use ontocode_protocol::credential_routing::ProviderCredentialAuthKind;
use ontocode_protocol::credential_routing::ProviderCredentialRoutingSummary;
use ontocode_protocol::credential_routing::ProviderCredentialSourceKind;
use ontocode_provider_auth::ProviderCredentialRefreshDiagnostics;
use ontocode_provider_auth::ProviderCredentialRefreshState;

fn candidate(
    credential_key: &str,
    provider_name: &str,
    account_group: Option<&str>,
    priority: u16,
    state: ProviderCredentialRefreshState,
) -> ProviderCredentialCandidate {
    ProviderCredentialCandidate {
        credential_key: credential_key.to_string(),
        routing: ProviderCredentialRoutingSummary {
            provider_name: provider_name.to_string(),
            source_kind: ProviderCredentialSourceKind::FirstPartyLogin,
            auth_kind: ProviderCredentialAuthKind::OAuthBearer,
            has_account_id: true,
            has_endpoint: false,
            has_client_id: false,
            scope_count: 0,
            expires_at: None,
            provenance: Some("test".to_string()),
        },
        account_group: account_group.map(str::to_string),
        priority,
        diagnostics: ProviderCredentialRefreshDiagnostics {
            state,
            consecutive_failures: 0,
            next_retry_at: None,
            last_failure_kind: None,
            last_error_detail: None,
        },
    }
}

#[test]
fn round_robin_rotates_eligible_candidates_deterministically() {
    let mut scheduler = ProviderCredentialScheduler::default();
    let request = ProviderCredentialScheduleRequest {
        provider_name: "openai",
        account_group: Some("primary"),
        sticky_session_key: None,
        sticky_session_reset: false,
    };
    let candidates = vec![
        candidate(
            "acct-b",
            "openai",
            Some("primary"),
            1,
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
        candidate(
            "acct-a",
            "openai",
            Some("primary"),
            5,
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
    ];

    assert_eq!(
        scheduler.select(
            request.clone(),
            ProviderCredentialSchedulingPolicy::RoundRobin,
            &candidates
        ),
        ProviderCredentialScheduleDecision {
            selected_credential_key: Some("acct-a".to_string()),
            selected_provider: Some("openai".to_string()),
            selected_account_group: Some("primary".to_string()),
            policy: ProviderCredentialSchedulingPolicy::RoundRobin,
            trace: vec![
                crate::schedule::ProviderCredentialCandidateTrace {
                    credential_key: "acct-b".to_string(),
                    status: ProviderCredentialCandidateStatus::Eligible,
                    priority: 1,
                },
                crate::schedule::ProviderCredentialCandidateTrace {
                    credential_key: "acct-a".to_string(),
                    status: ProviderCredentialCandidateStatus::Eligible,
                    priority: 5,
                },
            ],
        }
    );
    assert_eq!(
        scheduler
            .select(
                request,
                ProviderCredentialSchedulingPolicy::RoundRobin,
                &candidates
            )
            .selected_credential_key,
        Some("acct-b".to_string())
    );
}

#[test]
fn priority_policy_prefers_highest_priority_with_stable_tiebreak() {
    let mut scheduler = ProviderCredentialScheduler::default();
    let candidates = vec![
        candidate(
            "acct-b",
            "claude",
            Some("team"),
            10,
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
        candidate(
            "acct-a",
            "claude",
            Some("team"),
            10,
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
        candidate(
            "acct-c",
            "claude",
            Some("team"),
            2,
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
    ];

    assert_eq!(
        scheduler
            .select(
                ProviderCredentialScheduleRequest {
                    provider_name: "claude",
                    account_group: Some("team"),
                    sticky_session_key: None,
                    sticky_session_reset: false,
                },
                ProviderCredentialSchedulingPolicy::Priority,
                &candidates,
            )
            .selected_credential_key,
        Some("acct-a".to_string())
    );
}

#[test]
fn failover_policy_prefers_healthy_candidate_over_refresh_eligible_backup() {
    let mut scheduler = ProviderCredentialScheduler::default();
    let candidates = vec![
        candidate(
            "acct-backup",
            "gemini",
            Some("workspace"),
            20,
            ProviderCredentialRefreshState::RefreshEligible,
        ),
        candidate(
            "acct-primary",
            "gemini",
            Some("workspace"),
            5,
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
    ];

    assert_eq!(
        scheduler
            .select(
                ProviderCredentialScheduleRequest {
                    provider_name: "gemini",
                    account_group: Some("workspace"),
                    sticky_session_key: None,
                    sticky_session_reset: false,
                },
                ProviderCredentialSchedulingPolicy::Failover,
                &candidates,
            )
            .selected_credential_key,
        Some("acct-primary".to_string())
    );
}

#[test]
fn scheduler_blocks_candidates_by_provider_group_and_refresh_state() {
    let mut scheduler = ProviderCredentialScheduler::default();
    let candidates = vec![
        candidate(
            "wrong-provider",
            "claude",
            Some("primary"),
            10,
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
        candidate(
            "wrong-group",
            "openai",
            Some("secondary"),
            10,
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
        candidate(
            "suppressed",
            "openai",
            Some("primary"),
            10,
            ProviderCredentialRefreshState::RefreshSuppressed,
        ),
    ];

    assert_eq!(
        scheduler.select(
            ProviderCredentialScheduleRequest {
                provider_name: "openai",
                account_group: Some("primary"),
                sticky_session_key: None,
                sticky_session_reset: false,
            },
            ProviderCredentialSchedulingPolicy::Priority,
            &candidates,
        ),
        ProviderCredentialScheduleDecision {
            selected_credential_key: None,
            selected_provider: None,
            selected_account_group: None,
            policy: ProviderCredentialSchedulingPolicy::Priority,
            trace: vec![
                crate::schedule::ProviderCredentialCandidateTrace {
                    credential_key: "wrong-provider".to_string(),
                    status: ProviderCredentialCandidateStatus::BlockedByProvider,
                    priority: 10,
                },
                crate::schedule::ProviderCredentialCandidateTrace {
                    credential_key: "wrong-group".to_string(),
                    status: ProviderCredentialCandidateStatus::BlockedByAccountGroup,
                    priority: 10,
                },
                crate::schedule::ProviderCredentialCandidateTrace {
                    credential_key: "suppressed".to_string(),
                    status: ProviderCredentialCandidateStatus::BlockedByRefreshState,
                    priority: 10,
                },
            ],
        }
    );
}

#[test]
fn sticky_session_reuses_previous_selection_until_reset() {
    let mut scheduler = ProviderCredentialScheduler::default();
    let candidates = vec![
        candidate(
            "acct-primary",
            "openai",
            Some("primary"),
            10,
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
        candidate(
            "acct-backup",
            "openai",
            Some("primary"),
            20,
            ProviderCredentialRefreshState::RefreshHealthy,
        ),
    ];

    let first = scheduler.select(
        ProviderCredentialScheduleRequest {
            provider_name: "openai",
            account_group: Some("primary"),
            sticky_session_key: Some("thread-1"),
            sticky_session_reset: false,
        },
        ProviderCredentialSchedulingPolicy::StickySession,
        &candidates,
    );
    let second = scheduler.select(
        ProviderCredentialScheduleRequest {
            provider_name: "openai",
            account_group: Some("primary"),
            sticky_session_key: Some("thread-1"),
            sticky_session_reset: false,
        },
        ProviderCredentialSchedulingPolicy::StickySession,
        &candidates,
    );
    let reset = scheduler.select(
        ProviderCredentialScheduleRequest {
            provider_name: "openai",
            account_group: Some("primary"),
            sticky_session_key: Some("thread-1"),
            sticky_session_reset: true,
        },
        ProviderCredentialSchedulingPolicy::StickySession,
        &[
            candidate(
                "acct-primary",
                "openai",
                Some("primary"),
                30,
                ProviderCredentialRefreshState::RefreshHealthy,
            ),
            candidate(
                "acct-backup",
                "openai",
                Some("primary"),
                20,
                ProviderCredentialRefreshState::RefreshHealthy,
            ),
        ],
    );

    assert_eq!(
        first.selected_credential_key,
        Some("acct-backup".to_string())
    );
    assert_eq!(
        second.selected_credential_key,
        Some("acct-backup".to_string())
    );
    assert_eq!(
        reset.selected_credential_key,
        Some("acct-primary".to_string())
    );
}

#[test]
fn sticky_session_fails_over_when_previous_selection_is_no_longer_eligible() {
    let mut scheduler = ProviderCredentialScheduler::default();
    scheduler.select(
        ProviderCredentialScheduleRequest {
            provider_name: "claude",
            account_group: Some("team"),
            sticky_session_key: Some("thread-2"),
            sticky_session_reset: false,
        },
        ProviderCredentialSchedulingPolicy::StickySession,
        &[
            candidate(
                "acct-primary",
                "claude",
                Some("team"),
                10,
                ProviderCredentialRefreshState::RefreshHealthy,
            ),
            candidate(
                "acct-backup",
                "claude",
                Some("team"),
                5,
                ProviderCredentialRefreshState::RefreshHealthy,
            ),
        ],
    );

    let decision = scheduler.select(
        ProviderCredentialScheduleRequest {
            provider_name: "claude",
            account_group: Some("team"),
            sticky_session_key: Some("thread-2"),
            sticky_session_reset: false,
        },
        ProviderCredentialSchedulingPolicy::StickySession,
        &[
            candidate(
                "acct-primary",
                "claude",
                Some("team"),
                10,
                ProviderCredentialRefreshState::RefreshSuppressed,
            ),
            candidate(
                "acct-backup",
                "claude",
                Some("team"),
                5,
                ProviderCredentialRefreshState::RefreshHealthy,
            ),
        ],
    );

    assert_eq!(
        decision.selected_credential_key,
        Some("acct-backup".to_string())
    );
}
