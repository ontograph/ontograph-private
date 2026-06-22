use pretty_assertions::assert_eq;

use super::ProviderRouteDecision;
use super::ProviderRouteDiagnosticSnapshot;
use super::ProviderRouteRequest;
use super::ProviderRouteResolutionError;
use super::ProviderRouteRule;
use super::RouteBlockedReason;
use super::RouteDiagnosticStatus;
use super::RouteFallbackPolicy;
use super::RouteMatchKind;
use super::resolve_provider_route;
use super::route_diagnostic_snapshot;

#[test]
fn alias_rule_resolves_requested_model_to_target_model() {
    let decision = resolve_provider_route(
        ProviderRouteRequest {
            requested_model: "claude-code",
            provider_hint: None,
        },
        &[
            ProviderRouteRule::alias("claude", "claude-code", "claude-3-7-sonnet")
                .with_account_group("team-a")
                .with_fallback_policy(RouteFallbackPolicy::SameProviderOnly),
        ],
    )
    .expect("route should resolve")
    .expect("route should match");

    assert_eq!(
        decision,
        ProviderRouteDecision {
            requested_model: "claude-code".to_string(),
            resolved_model: "claude-3-7-sonnet".to_string(),
            provider_name: "claude".to_string(),
            account_group: Some("team-a".to_string()),
            fallback_policy: RouteFallbackPolicy::SameProviderOnly,
            match_kind: RouteMatchKind::Alias,
        }
    );
}

#[test]
fn prefix_rule_strips_prefix_and_routes_to_provider() {
    let decision = resolve_provider_route(
        ProviderRouteRequest {
            requested_model: "gemini/gemini-2.5-pro",
            provider_hint: None,
        },
        &[ProviderRouteRule::prefix("gemini", "gemini/")],
    )
    .expect("route should resolve")
    .expect("route should match");

    assert_eq!(
        decision,
        ProviderRouteDecision {
            requested_model: "gemini/gemini-2.5-pro".to_string(),
            resolved_model: "gemini-2.5-pro".to_string(),
            provider_name: "gemini".to_string(),
            account_group: None,
            fallback_policy: RouteFallbackPolicy::FailClosed,
            match_kind: RouteMatchKind::Prefix,
        }
    );
}

#[test]
fn longest_prefix_wins_when_multiple_prefix_rules_match() {
    let decision = resolve_provider_route(
        ProviderRouteRequest {
            requested_model: "claude/pro/sonnet",
            provider_hint: None,
        },
        &[
            ProviderRouteRule::prefix("claude", "claude/"),
            ProviderRouteRule::prefix("claude-pro", "claude/pro/"),
        ],
    )
    .expect("route should resolve")
    .expect("route should match");

    assert_eq!(decision.provider_name, "claude-pro");
    assert_eq!(decision.resolved_model, "sonnet");
}

#[test]
fn ambiguous_alias_fails_closed() {
    let err = resolve_provider_route(
        ProviderRouteRequest {
            requested_model: "assistant-default",
            provider_hint: None,
        },
        &[
            ProviderRouteRule::alias("claude", "assistant-default", "claude-3-7-sonnet"),
            ProviderRouteRule::alias("gemini", "assistant-default", "gemini-2.5-pro"),
        ],
    )
    .expect_err("route should fail");

    assert_eq!(
        err,
        ProviderRouteResolutionError::AmbiguousAlias {
            requested_model: "assistant-default".to_string(),
            providers: vec!["claude".to_string(), "gemini".to_string()],
        }
    );
}

#[test]
fn empty_prefix_body_fails_closed() {
    let err = resolve_provider_route(
        ProviderRouteRequest {
            requested_model: "claude/",
            provider_hint: None,
        },
        &[ProviderRouteRule::prefix("claude", "claude/")],
    )
    .expect_err("route should fail");

    assert_eq!(
        err,
        ProviderRouteResolutionError::EmptyPrefixBody {
            requested_model: "claude/".to_string(),
            prefix: "claude/".to_string(),
        }
    );
}

#[test]
fn provider_hint_mismatch_rejects_alias_match() {
    let err = resolve_provider_route(
        ProviderRouteRequest {
            requested_model: "claude-code",
            provider_hint: Some("gemini"),
        },
        &[ProviderRouteRule::alias(
            "claude",
            "claude-code",
            "claude-3-7-sonnet",
        )],
    )
    .expect_err("route should fail");

    assert_eq!(
        err,
        ProviderRouteResolutionError::ProviderHintMismatch {
            requested_model: "claude-code".to_string(),
            provider_hint: "gemini".to_string(),
            matched_provider: "claude".to_string(),
        }
    );
}

#[test]
fn antigravity_provider_hint_mismatch_keeps_route_blocked() {
    let err = resolve_provider_route(
        ProviderRouteRequest {
            requested_model: "claude-code",
            provider_hint: Some("antigravity"),
        },
        &[ProviderRouteRule::alias(
            "claude",
            "claude-code",
            "claude-3-7-sonnet",
        )],
    )
    .expect_err("route should fail");

    assert_eq!(
        err,
        ProviderRouteResolutionError::ProviderHintMismatch {
            requested_model: "claude-code".to_string(),
            provider_hint: "antigravity".to_string(),
            matched_provider: "claude".to_string(),
        }
    );
}

#[test]
fn unmatched_route_returns_none() {
    let decision = resolve_provider_route(
        ProviderRouteRequest {
            requested_model: "gpt-5.5",
            provider_hint: None,
        },
        &[ProviderRouteRule::prefix("claude", "claude/")],
    )
    .expect("route should not fail");

    assert_eq!(decision, None);
}

#[test]
fn diagnostic_snapshot_reports_matched_alias_route() {
    let snapshot = route_diagnostic_snapshot(
        ProviderRouteRequest {
            requested_model: "claude-code",
            provider_hint: Some("claude"),
        },
        &[ProviderRouteRule::alias(
            "claude",
            "claude-code",
            "claude-3-7-sonnet",
        )],
    );

    assert_eq!(
        snapshot,
        ProviderRouteDiagnosticSnapshot {
            requested_model: "claude-code".to_string(),
            provider_hint: Some("claude".to_string()),
            status: RouteDiagnosticStatus::Matched,
            resolved_model: Some("claude-3-7-sonnet".to_string()),
            resolved_provider: Some("claude".to_string()),
            account_group: None,
            fallback_policy: Some(RouteFallbackPolicy::FailClosed),
            match_kind: Some(RouteMatchKind::Alias),
            blocked_reason: None,
        }
    );
}

#[test]
fn diagnostic_snapshot_reports_blocked_reason_for_ambiguous_alias() {
    let snapshot = route_diagnostic_snapshot(
        ProviderRouteRequest {
            requested_model: "assistant-default",
            provider_hint: None,
        },
        &[
            ProviderRouteRule::alias("claude", "assistant-default", "claude-3-7-sonnet"),
            ProviderRouteRule::alias("gemini", "assistant-default", "gemini-2.5-pro"),
        ],
    );

    assert_eq!(snapshot.status, RouteDiagnosticStatus::Blocked);
    assert_eq!(
        snapshot.blocked_reason,
        Some(RouteBlockedReason::AmbiguousAlias)
    );
    assert_eq!(snapshot.resolved_model, None);
    assert_eq!(snapshot.resolved_provider, None);
}
