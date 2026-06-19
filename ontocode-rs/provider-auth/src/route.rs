use ontocode_model_provider_info::ModelProviderInfo;

/// Internal provider-auth routing class derived from already resolved provider state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderAuthKind {
    OAuth,
    ApiKey,
    ExternalBearer,
    Unauthenticated,
    ProviderSpecific,
}

/// Internal provider execution route shared by provider/auth and model-provider owners.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderRoute {
    pub provider_id: String,
    pub model: String,
    pub auth_kind: ProviderAuthKind,
    pub auth_profile_id: Option<String>,
    pub runtime_id: Option<String>,
}

impl ProviderRoute {
    pub fn new(
        provider_id: impl Into<String>,
        model: impl Into<String>,
        auth_kind: ProviderAuthKind,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            model: model.into(),
            auth_kind,
            auth_profile_id: None,
            runtime_id: None,
        }
    }

    pub fn with_auth_profile_id(mut self, auth_profile_id: Option<String>) -> Self {
        self.auth_profile_id = auth_profile_id;
        self
    }

    pub fn with_runtime_id(mut self, runtime_id: Option<String>) -> Self {
        self.runtime_id = runtime_id;
        self
    }
}

pub fn provider_auth_kind_for_provider_info(provider: &ModelProviderInfo) -> ProviderAuthKind {
    if provider.aws.is_some() {
        return ProviderAuthKind::ProviderSpecific;
    }

    if provider.auth.is_some() {
        return ProviderAuthKind::ExternalBearer;
    }

    if provider.env_key.is_some() || provider.experimental_bearer_token.is_some() {
        return ProviderAuthKind::ApiKey;
    }

    if provider.requires_openai_auth {
        return ProviderAuthKind::OAuth;
    }

    ProviderAuthKind::Unauthenticated
}

pub fn derive_provider_route(
    provider_id: impl Into<String>,
    model: impl Into<String>,
    provider: &ModelProviderInfo,
) -> ProviderRoute {
    ProviderRoute::new(
        provider_id,
        model,
        provider_auth_kind_for_provider_info(provider),
    )
}

#[cfg(test)]
#[path = "route_tests.rs"]
mod tests;
