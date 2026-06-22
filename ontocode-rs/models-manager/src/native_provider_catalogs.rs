use ontocode_protocol::config_types::ReasoningSummary;
use ontocode_protocol::openai_models::ConfigShellToolType;
use ontocode_protocol::openai_models::InputModality;
use ontocode_protocol::openai_models::ModelInfo;
use ontocode_protocol::openai_models::ModelVisibility;
use ontocode_protocol::openai_models::ModelsResponse;
use ontocode_protocol::openai_models::TruncationPolicyConfig;
use ontocode_protocol::openai_models::WebSearchToolType;

use crate::model_info::BASE_INSTRUCTIONS;

const ANTHROPIC_CONTEXT_WINDOW: i64 = 200_000;
const GEMINI_CONTEXT_WINDOW: i64 = 1_048_576;
const COPILOT_CONTEXT_WINDOW: i64 = 128_000;

pub fn anthropic_models_response() -> ModelsResponse {
    ModelsResponse {
        models: vec![native_model(
            "claude-sonnet-4-5",
            "Claude Sonnet 4.5",
            "Anthropic Claude model through the native Messages API.",
            ANTHROPIC_CONTEXT_WINDOW,
            /*priority*/ 0,
        )],
    }
}

pub fn gemini_models_response() -> ModelsResponse {
    ModelsResponse {
        models: vec![native_model(
            "gemini-2.5-pro",
            "Gemini 2.5 Pro",
            "Google Gemini model through the native GenerateContent API.",
            GEMINI_CONTEXT_WINDOW,
            /*priority*/ 0,
        )],
    }
}

pub fn github_copilot_models_response() -> ModelsResponse {
    ModelsResponse {
        models: vec![native_model(
            "gpt-4o-copilot",
            "GPT-4o Copilot",
            "GitHub Copilot model through the native chat completions API.",
            COPILOT_CONTEXT_WINDOW,
            /*priority*/ 0,
        )],
    }
}

fn native_model(
    slug: &str,
    display_name: &str,
    description: &str,
    context_window: i64,
    priority: i32,
) -> ModelInfo {
    ModelInfo {
        slug: slug.to_string(),
        display_name: display_name.to_string(),
        description: Some(description.to_string()),
        default_reasoning_level: None,
        supported_reasoning_levels: Vec::new(),
        shell_type: ConfigShellToolType::ShellCommand,
        visibility: ModelVisibility::List,
        supported_in_api: true,
        priority,
        additional_speed_tiers: Vec::new(),
        service_tiers: Vec::new(),
        default_service_tier: None,
        availability_nux: None,
        upgrade: None,
        base_instructions: BASE_INSTRUCTIONS.to_string(),
        model_messages: None,
        supports_reasoning_summaries: false,
        default_reasoning_summary: ReasoningSummary::Auto,
        support_verbosity: false,
        default_verbosity: None,
        apply_patch_tool_type: None,
        web_search_tool_type: WebSearchToolType::Text,
        truncation_policy: TruncationPolicyConfig::bytes(/*limit*/ 10_000),
        supports_parallel_tool_calls: false,
        supports_image_detail_original: false,
        context_window: Some(context_window),
        max_context_window: Some(context_window),
        auto_compact_token_limit: None,
        effective_context_window_percent: 95,
        experimental_supported_tools: Vec::new(),
        input_modalities: vec![InputModality::Text],
        used_fallback_model_metadata: false,
        supports_search_tool: false,
        auto_review_model_override: None,
        tool_mode: None,
        multi_agent_version: None,
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn native_catalogs_expose_expected_model_ids() {
        assert_eq!(
            model_slugs(anthropic_models_response()),
            vec!["claude-sonnet-4-5"]
        );
        assert_eq!(
            model_slugs(gemini_models_response()),
            vec!["gemini-2.5-pro"]
        );
        assert_eq!(
            model_slugs(github_copilot_models_response()),
            vec!["gpt-4o-copilot"]
        );
    }

    #[test]
    fn native_catalogs_are_conservative_about_capabilities() {
        let catalogs = [
            anthropic_models_response(),
            gemini_models_response(),
            github_copilot_models_response(),
        ];

        for catalog in catalogs {
            for model in catalog.models {
                assert_eq!(model.default_reasoning_level, None);
                assert_eq!(model.supported_reasoning_levels, Vec::new());
                assert_eq!(model.additional_speed_tiers, Vec::<String>::new());
                assert_eq!(model.service_tiers, Vec::new());
                assert_eq!(model.default_service_tier, None);
                assert_eq!(model.apply_patch_tool_type, None);
                assert_eq!(model.experimental_supported_tools, Vec::<String>::new());
                assert_eq!(model.input_modalities, vec![InputModality::Text]);
                assert!(!model.supports_parallel_tool_calls);
                assert!(!model.supports_image_detail_original);
                assert!(!model.supports_reasoning_summaries);
                assert!(!model.supports_search_tool);
                assert!(!model.support_verbosity);
                assert!(model.supported_in_api);
            }
        }
    }

    fn model_slugs(catalog: ModelsResponse) -> Vec<String> {
        catalog.models.into_iter().map(|model| model.slug).collect()
    }
}
