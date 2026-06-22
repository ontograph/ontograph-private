use ontocode_model_provider_info::ModelProviderInfo;
use ontocode_provider_auth::ProviderAuthKind;
use ontocode_provider_auth::ProviderRoute;

use crate::config::Config;

/// Derives the effective route for a fully resolved config.
///
/// The route key stays pinned to the stable configured provider id while the
/// model and auth class come from the resolved runtime config.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn resolved_provider_route(config: &Config, model: Option<&str>) -> ProviderRoute {
    provider_route(
        config.model_provider_id.as_str(),
        model
            .or(config.model.as_deref())
            .unwrap_or(config.model_provider_id.as_str()),
        &config.model_provider,
    )
}

/// Derives the child route after role and runtime overrides have been applied.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn child_provider_route(config: &Config) -> ProviderRoute {
    resolved_provider_route(config, config.model.as_deref())
}

#[cfg_attr(not(test), allow(dead_code))]
fn provider_route(provider_id: &str, model: &str, provider: &ModelProviderInfo) -> ProviderRoute {
    ProviderRoute::new(
        provider_id,
        model,
        provider_auth_kind_for_provider_info(provider),
    )
}

#[cfg_attr(not(test), allow(dead_code))]
fn provider_auth_kind_for_provider_info(provider: &ModelProviderInfo) -> ProviderAuthKind {
    ontocode_provider_auth::provider_auth_kind_for_provider_info(provider)
}

#[cfg(test)]
#[path = "provider_route_tests.rs"]
mod tests;
