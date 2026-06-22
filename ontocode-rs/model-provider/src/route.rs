#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RouteFallbackPolicy {
    FailClosed,
    SameProviderOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RouteMatchKind {
    Alias,
    Prefix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProviderRouteRequest<'a> {
    pub(crate) requested_model: &'a str,
    pub(crate) provider_hint: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RouteDiagnosticStatus {
    Matched,
    NoMatch,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RouteBlockedReason {
    AmbiguousAlias,
    AmbiguousPrefix,
    EmptyPrefixBody,
    ProviderHintMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderRouteDecision {
    pub(crate) requested_model: String,
    pub(crate) resolved_model: String,
    pub(crate) provider_name: String,
    pub(crate) account_group: Option<String>,
    pub(crate) fallback_policy: RouteFallbackPolicy,
    pub(crate) match_kind: RouteMatchKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderRouteDiagnosticSnapshot {
    pub(crate) requested_model: String,
    pub(crate) provider_hint: Option<String>,
    pub(crate) status: RouteDiagnosticStatus,
    pub(crate) resolved_model: Option<String>,
    pub(crate) resolved_provider: Option<String>,
    pub(crate) account_group: Option<String>,
    pub(crate) fallback_policy: Option<RouteFallbackPolicy>,
    pub(crate) match_kind: Option<RouteMatchKind>,
    pub(crate) blocked_reason: Option<RouteBlockedReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ProviderRouteResolutionError {
    AmbiguousAlias {
        requested_model: String,
        providers: Vec<String>,
    },
    AmbiguousPrefix {
        requested_model: String,
        prefix: String,
        providers: Vec<String>,
    },
    EmptyPrefixBody {
        requested_model: String,
        prefix: String,
    },
    ProviderHintMismatch {
        requested_model: String,
        provider_hint: String,
        matched_provider: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProviderRouteRuleKind {
    Alias { alias: String, target_model: String },
    Prefix { prefix: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderRouteRule {
    provider_name: String,
    account_group: Option<String>,
    fallback_policy: RouteFallbackPolicy,
    kind: ProviderRouteRuleKind,
}

impl ProviderRouteRule {
    pub(crate) fn alias(
        provider_name: impl Into<String>,
        alias: impl Into<String>,
        target_model: impl Into<String>,
    ) -> Self {
        Self {
            provider_name: provider_name.into(),
            account_group: None,
            fallback_policy: RouteFallbackPolicy::FailClosed,
            kind: ProviderRouteRuleKind::Alias {
                alias: alias.into(),
                target_model: target_model.into(),
            },
        }
    }

    pub(crate) fn prefix(provider_name: impl Into<String>, prefix: impl Into<String>) -> Self {
        Self {
            provider_name: provider_name.into(),
            account_group: None,
            fallback_policy: RouteFallbackPolicy::FailClosed,
            kind: ProviderRouteRuleKind::Prefix {
                prefix: prefix.into(),
            },
        }
    }

    pub(crate) fn with_account_group(mut self, account_group: impl Into<String>) -> Self {
        self.account_group = Some(account_group.into());
        self
    }

    pub(crate) fn with_fallback_policy(mut self, fallback_policy: RouteFallbackPolicy) -> Self {
        self.fallback_policy = fallback_policy;
        self
    }
}

pub(crate) fn resolve_provider_route(
    request: ProviderRouteRequest<'_>,
    rules: &[ProviderRouteRule],
) -> Result<Option<ProviderRouteDecision>, ProviderRouteResolutionError> {
    if let Some(alias_decision) = resolve_alias_route(&request, rules)? {
        return Ok(Some(alias_decision));
    }

    resolve_prefix_route(&request, rules)
}

pub(crate) fn route_diagnostic_snapshot(
    request: ProviderRouteRequest<'_>,
    rules: &[ProviderRouteRule],
) -> ProviderRouteDiagnosticSnapshot {
    match resolve_provider_route(request, rules) {
        Ok(Some(decision)) => ProviderRouteDiagnosticSnapshot {
            requested_model: decision.requested_model.clone(),
            provider_hint: request.provider_hint.map(str::to_string),
            status: RouteDiagnosticStatus::Matched,
            resolved_model: Some(decision.resolved_model),
            resolved_provider: Some(decision.provider_name),
            account_group: decision.account_group,
            fallback_policy: Some(decision.fallback_policy),
            match_kind: Some(decision.match_kind),
            blocked_reason: None,
        },
        Ok(None) => ProviderRouteDiagnosticSnapshot {
            requested_model: request.requested_model.to_string(),
            provider_hint: request.provider_hint.map(str::to_string),
            status: RouteDiagnosticStatus::NoMatch,
            resolved_model: None,
            resolved_provider: None,
            account_group: None,
            fallback_policy: None,
            match_kind: None,
            blocked_reason: None,
        },
        Err(err) => ProviderRouteDiagnosticSnapshot {
            requested_model: request.requested_model.to_string(),
            provider_hint: request.provider_hint.map(str::to_string),
            status: RouteDiagnosticStatus::Blocked,
            resolved_model: None,
            resolved_provider: None,
            account_group: None,
            fallback_policy: None,
            match_kind: None,
            blocked_reason: Some(RouteBlockedReason::from(&err)),
        },
    }
}

fn resolve_alias_route(
    request: &ProviderRouteRequest<'_>,
    rules: &[ProviderRouteRule],
) -> Result<Option<ProviderRouteDecision>, ProviderRouteResolutionError> {
    let alias_matches = rules
        .iter()
        .filter_map(|rule| match &rule.kind {
            ProviderRouteRuleKind::Alias {
                alias,
                target_model,
            } if alias.eq_ignore_ascii_case(request.requested_model) => Some((rule, target_model)),
            ProviderRouteRuleKind::Alias { .. } | ProviderRouteRuleKind::Prefix { .. } => None,
        })
        .collect::<Vec<_>>();

    if alias_matches.is_empty() {
        return Ok(None);
    }

    if alias_matches.len() > 1 {
        return Err(ProviderRouteResolutionError::AmbiguousAlias {
            requested_model: request.requested_model.to_string(),
            providers: alias_matches
                .into_iter()
                .map(|(rule, _)| rule.provider_name.clone())
                .collect(),
        });
    }

    let (rule, target_model) = &alias_matches[0];
    validate_provider_hint(request, &rule.provider_name)?;
    Ok(Some(ProviderRouteDecision {
        requested_model: request.requested_model.to_string(),
        resolved_model: target_model.to_string(),
        provider_name: rule.provider_name.clone(),
        account_group: rule.account_group.clone(),
        fallback_policy: rule.fallback_policy,
        match_kind: RouteMatchKind::Alias,
    }))
}

fn resolve_prefix_route(
    request: &ProviderRouteRequest<'_>,
    rules: &[ProviderRouteRule],
) -> Result<Option<ProviderRouteDecision>, ProviderRouteResolutionError> {
    let prefix_matches = rules
        .iter()
        .filter_map(|rule| match &rule.kind {
            ProviderRouteRuleKind::Prefix { prefix }
                if request.requested_model.starts_with(prefix) =>
            {
                Some((rule, prefix))
            }
            ProviderRouteRuleKind::Alias { .. } | ProviderRouteRuleKind::Prefix { .. } => None,
        })
        .collect::<Vec<_>>();

    if prefix_matches.is_empty() {
        return Ok(None);
    }

    let Some(longest_prefix_len) = prefix_matches.iter().map(|(_, prefix)| prefix.len()).max()
    else {
        return Ok(None);
    };
    let strongest_matches = prefix_matches
        .into_iter()
        .filter(|(_, prefix)| prefix.len() == longest_prefix_len)
        .collect::<Vec<_>>();

    if strongest_matches.len() > 1 {
        let (_, prefix) = strongest_matches[0];
        return Err(ProviderRouteResolutionError::AmbiguousPrefix {
            requested_model: request.requested_model.to_string(),
            prefix: prefix.clone(),
            providers: strongest_matches
                .into_iter()
                .map(|(rule, _)| rule.provider_name.clone())
                .collect(),
        });
    }

    let (rule, prefix) = &strongest_matches[0];
    validate_provider_hint(request, &rule.provider_name)?;
    let Some(resolved_model) = request.requested_model.strip_prefix(prefix.as_str()) else {
        return Ok(None);
    };
    if resolved_model.is_empty() {
        return Err(ProviderRouteResolutionError::EmptyPrefixBody {
            requested_model: request.requested_model.to_string(),
            prefix: prefix.to_string(),
        });
    }

    Ok(Some(ProviderRouteDecision {
        requested_model: request.requested_model.to_string(),
        resolved_model: resolved_model.to_string(),
        provider_name: rule.provider_name.clone(),
        account_group: rule.account_group.clone(),
        fallback_policy: rule.fallback_policy,
        match_kind: RouteMatchKind::Prefix,
    }))
}

fn validate_provider_hint(
    request: &ProviderRouteRequest<'_>,
    matched_provider: &str,
) -> Result<(), ProviderRouteResolutionError> {
    if let Some(provider_hint) = request.provider_hint
        && !provider_hint.eq_ignore_ascii_case(matched_provider)
    {
        return Err(ProviderRouteResolutionError::ProviderHintMismatch {
            requested_model: request.requested_model.to_string(),
            provider_hint: provider_hint.to_string(),
            matched_provider: matched_provider.to_string(),
        });
    }

    Ok(())
}

impl From<&ProviderRouteResolutionError> for RouteBlockedReason {
    fn from(value: &ProviderRouteResolutionError) -> Self {
        match value {
            ProviderRouteResolutionError::AmbiguousAlias { .. } => Self::AmbiguousAlias,
            ProviderRouteResolutionError::AmbiguousPrefix { .. } => Self::AmbiguousPrefix,
            ProviderRouteResolutionError::EmptyPrefixBody { .. } => Self::EmptyPrefixBody,
            ProviderRouteResolutionError::ProviderHintMismatch { .. } => Self::ProviderHintMismatch,
        }
    }
}

#[cfg(test)]
#[path = "route_tests.rs"]
mod tests;
