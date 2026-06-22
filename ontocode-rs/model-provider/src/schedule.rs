use std::collections::HashMap;

use ontocode_protocol::credential_routing::ProviderCredentialRoutingSummary;
use ontocode_provider_auth::ProviderCredentialRefreshDiagnostics;
use ontocode_provider_auth::ProviderCredentialRefreshState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProviderCredentialSchedulingPolicy {
    RoundRobin,
    Priority,
    Failover,
    StickySession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderCredentialScheduleRequest<'a> {
    pub(crate) provider_name: &'a str,
    pub(crate) account_group: Option<&'a str>,
    pub(crate) sticky_session_key: Option<&'a str>,
    pub(crate) sticky_session_reset: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderCredentialCandidate {
    pub(crate) credential_key: String,
    pub(crate) routing: ProviderCredentialRoutingSummary,
    pub(crate) account_group: Option<String>,
    pub(crate) priority: u16,
    pub(crate) diagnostics: ProviderCredentialRefreshDiagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProviderCredentialCandidateStatus {
    Eligible,
    BlockedByProvider,
    BlockedByAccountGroup,
    BlockedByRefreshState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderCredentialCandidateTrace {
    pub(crate) credential_key: String,
    pub(crate) status: ProviderCredentialCandidateStatus,
    pub(crate) priority: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderCredentialScheduleDecision {
    pub(crate) selected_credential_key: Option<String>,
    pub(crate) selected_provider: Option<String>,
    pub(crate) selected_account_group: Option<String>,
    pub(crate) policy: ProviderCredentialSchedulingPolicy,
    pub(crate) trace: Vec<ProviderCredentialCandidateTrace>,
}

#[derive(Default)]
pub(crate) struct ProviderCredentialScheduler {
    round_robin_cursor_by_scope: HashMap<String, usize>,
    sticky_selection_by_scope: HashMap<String, String>,
}

impl ProviderCredentialScheduler {
    pub(crate) fn select(
        &mut self,
        request: ProviderCredentialScheduleRequest<'_>,
        policy: ProviderCredentialSchedulingPolicy,
        candidates: &[ProviderCredentialCandidate],
    ) -> ProviderCredentialScheduleDecision {
        let trace = candidates
            .iter()
            .map(|candidate| ProviderCredentialCandidateTrace {
                credential_key: candidate.credential_key.clone(),
                status: candidate_status(&request, candidate),
                priority: candidate.priority,
            })
            .collect::<Vec<_>>();

        let eligible = candidates
            .iter()
            .filter(|candidate| {
                candidate_status(&request, candidate) == ProviderCredentialCandidateStatus::Eligible
            })
            .collect::<Vec<_>>();

        let selected = match policy {
            ProviderCredentialSchedulingPolicy::RoundRobin => {
                select_round_robin(&mut self.round_robin_cursor_by_scope, &request, &eligible)
            }
            ProviderCredentialSchedulingPolicy::Priority => select_by_priority(&eligible),
            ProviderCredentialSchedulingPolicy::Failover => select_failover(&eligible),
            ProviderCredentialSchedulingPolicy::StickySession => {
                select_sticky_session(&mut self.sticky_selection_by_scope, &request, &eligible)
            }
        };

        ProviderCredentialScheduleDecision {
            selected_credential_key: selected.map(|candidate| candidate.credential_key.clone()),
            selected_provider: selected.map(|candidate| candidate.routing.provider_name.clone()),
            selected_account_group: selected.and_then(|candidate| candidate.account_group.clone()),
            policy,
            trace,
        }
    }
}

fn candidate_status(
    request: &ProviderCredentialScheduleRequest<'_>,
    candidate: &ProviderCredentialCandidate,
) -> ProviderCredentialCandidateStatus {
    if !candidate
        .routing
        .provider_name
        .eq_ignore_ascii_case(request.provider_name)
    {
        return ProviderCredentialCandidateStatus::BlockedByProvider;
    }

    if request.account_group != candidate.account_group.as_deref() {
        return ProviderCredentialCandidateStatus::BlockedByAccountGroup;
    }

    if !is_schedulable_state(candidate.diagnostics.state) {
        return ProviderCredentialCandidateStatus::BlockedByRefreshState;
    }

    ProviderCredentialCandidateStatus::Eligible
}

fn is_schedulable_state(state: ProviderCredentialRefreshState) -> bool {
    matches!(
        state,
        ProviderCredentialRefreshState::NonRefreshable
            | ProviderCredentialRefreshState::RefreshEligible
            | ProviderCredentialRefreshState::RefreshHealthy
    )
}

fn select_round_robin<'a>(
    cursor_by_scope: &mut HashMap<String, usize>,
    request: &ProviderCredentialScheduleRequest<'_>,
    eligible: &[&'a ProviderCredentialCandidate],
) -> Option<&'a ProviderCredentialCandidate> {
    let mut ordered = eligible.to_vec();
    ordered.sort_by(|left, right| left.credential_key.cmp(&right.credential_key));
    if ordered.is_empty() {
        return None;
    }

    let scope = schedule_scope_key(request);
    let cursor = cursor_by_scope.entry(scope).or_insert(0);
    let selected = ordered[*cursor % ordered.len()];
    *cursor = cursor.saturating_add(1);
    Some(selected)
}

fn select_by_priority<'a>(
    eligible: &[&'a ProviderCredentialCandidate],
) -> Option<&'a ProviderCredentialCandidate> {
    eligible.iter().copied().max_by(|left, right| {
        left.priority
            .cmp(&right.priority)
            .then_with(|| right.credential_key.cmp(&left.credential_key))
    })
}

fn select_failover<'a>(
    eligible: &[&'a ProviderCredentialCandidate],
) -> Option<&'a ProviderCredentialCandidate> {
    let healthy = eligible
        .iter()
        .copied()
        .filter(|candidate| {
            candidate.diagnostics.state == ProviderCredentialRefreshState::RefreshHealthy
        })
        .collect::<Vec<_>>();
    if !healthy.is_empty() {
        return select_by_priority(&healthy);
    }
    select_by_priority(eligible)
}

fn select_sticky_session<'a>(
    sticky_selection_by_scope: &mut HashMap<String, String>,
    request: &ProviderCredentialScheduleRequest<'_>,
    eligible: &[&'a ProviderCredentialCandidate],
) -> Option<&'a ProviderCredentialCandidate> {
    let scope = sticky_scope_key(request);
    if request.sticky_session_reset {
        sticky_selection_by_scope.remove(scope.as_str());
    }

    if let Some(existing) = sticky_selection_by_scope.get(scope.as_str())
        && let Some(candidate) = eligible
            .iter()
            .copied()
            .find(|candidate| candidate.credential_key == *existing)
    {
        return Some(candidate);
    }

    let selected = select_by_priority(eligible)?;
    sticky_selection_by_scope.insert(scope, selected.credential_key.clone());
    Some(selected)
}

fn schedule_scope_key(request: &ProviderCredentialScheduleRequest<'_>) -> String {
    let account_group = request.account_group.unwrap_or("-");
    format!(
        "{}::{account_group}",
        request.provider_name.to_ascii_lowercase()
    )
}

fn sticky_scope_key(request: &ProviderCredentialScheduleRequest<'_>) -> String {
    let sticky_session_key = request.sticky_session_key.unwrap_or("-");
    format!("{}::{sticky_session_key}", schedule_scope_key(request))
}

#[cfg(test)]
#[path = "schedule_tests.rs"]
mod tests;
