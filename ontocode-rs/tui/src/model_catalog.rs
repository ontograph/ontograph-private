use ontocode_model_provider_info::GEMINI_CLI_PROVIDER_ID;
use ontocode_model_provider_info::GEMINI_PROVIDER_ID;
use ontocode_models_manager::native_provider_catalogs;
use ontocode_protocol::openai_models::ModelPreset;
use std::convert::Infallible;

pub(crate) const GEMINI_CLI_DISABLED_REASON: &str = "Gemini CLI runtime is not available yet.";

#[derive(Debug, Clone)]
pub(crate) struct ModelCatalog {
    models: Vec<ModelPreset>,
    provider_groups: Vec<ProviderModelGroup>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ProviderModelGroup {
    pub(crate) provider_id: String,
    pub(crate) display_name: String,
    pub(crate) models: Vec<ModelPreset>,
    pub(crate) disabled_reason: Option<String>,
}

pub(crate) fn gemini_cli_disabled_provider_group() -> ProviderModelGroup {
    ProviderModelGroup {
        provider_id: GEMINI_CLI_PROVIDER_ID.to_string(),
        display_name: "Gemini CLI".to_string(),
        models: native_provider_catalogs::gemini_models_response()
            .models
            .into_iter()
            .map(Into::into)
            .collect(),
        disabled_reason: Some(GEMINI_CLI_DISABLED_REASON.to_string()),
    }
}

pub(crate) fn gemini_provider_group() -> ProviderModelGroup {
    ProviderModelGroup {
        provider_id: GEMINI_PROVIDER_ID.to_string(),
        display_name: "Gemini".to_string(),
        models: native_provider_catalogs::gemini_models_response()
            .models
            .into_iter()
            .map(Into::into)
            .collect(),
        disabled_reason: None,
    }
}

impl ModelCatalog {
    #[cfg(test)]
    pub(crate) fn new(models: Vec<ModelPreset>) -> Self {
        Self {
            models,
            provider_groups: Vec::new(),
        }
    }

    pub(crate) fn with_provider_groups(
        models: Vec<ModelPreset>,
        provider_groups: Vec<ProviderModelGroup>,
    ) -> Self {
        Self {
            models,
            provider_groups,
        }
    }

    pub(crate) fn try_list_models(&self) -> Result<Vec<ModelPreset>, Infallible> {
        Ok(self.models.clone())
    }

    pub(crate) fn try_list_provider_groups(&self) -> Result<Vec<ProviderModelGroup>, Infallible> {
        Ok(self.provider_groups.clone())
    }
}
