use std::sync::Arc;

use crate::session::turn_context::TurnContext;
use crate::tools::hosted_spec::WebSearchToolOptions;
use crate::tools::hosted_spec::create_image_generation_tool;
use crate::tools::hosted_spec::create_web_search_tool;
use crate::tools::planning::CoreToolPlanContext;
use crate::tools::planning::PlannedTools;
use ontocode_features::Feature;
use ontocode_login::AuthManager;
use ontocode_protocol::openai_models::InputModality;
use ontocode_tools::ToolCall as ExtensionToolCall;
use ontocode_tools::ToolExecutor;
use ontocode_tools::ToolName;
use ontocode_tools::ToolSpec;

pub(crate) fn add_hosted_tools(
    context: &CoreToolPlanContext<'_>,
    planned_tools: &mut PlannedTools,
) {
    for spec in hosted_model_tool_specs(context) {
        planned_tools.add_hosted_spec(spec);
    }
}

fn hosted_model_tool_specs(context: &CoreToolPlanContext<'_>) -> Vec<ToolSpec> {
    let turn_context = context.turn_context;
    let mut specs = Vec::new();
    let provider_capabilities = turn_context.provider.capabilities();
    let web_search_mode = (!standalone_web_run_available(context.extension_tool_executors)
        && provider_capabilities.web_search)
        .then_some(turn_context.config.web_search_mode.value());
    let web_search_config = if provider_capabilities.web_search {
        turn_context.config.web_search_config.as_ref()
    } else {
        None
    };
    if let Some(web_search_tool) = create_web_search_tool(WebSearchToolOptions {
        web_search_mode,
        web_search_config,
        web_search_tool_type: turn_context.model_info.web_search_tool_type,
    }) {
        specs.push(web_search_tool);
    }
    if image_generation_tool_enabled(turn_context)
        && !standalone_image_generation_available(turn_context, context.extension_tool_executors)
    {
        specs.push(create_image_generation_tool("png"));
    }
    specs
}

fn namespace_tools_enabled(turn_context: &TurnContext) -> bool {
    turn_context.provider.capabilities().namespace_tools
}

fn image_generation_tool_enabled(turn_context: &TurnContext) -> bool {
    image_generation_runtime_enabled(turn_context)
        && turn_context
            .features
            .get()
            .enabled(Feature::ImageGeneration)
}

fn image_generation_runtime_enabled(turn_context: &TurnContext) -> bool {
    turn_context
        .auth_manager
        .as_deref()
        .is_some_and(AuthManager::current_auth_uses_codex_backend)
        && turn_context.provider.capabilities().image_generation
        && turn_context
            .model_info
            .input_modalities
            .contains(&InputModality::Image)
}

pub(crate) fn standalone_image_generation_model_visible(turn_context: &TurnContext) -> bool {
    image_generation_runtime_enabled(turn_context)
        && turn_context.features.get().enabled(Feature::ImageGenExt)
        && namespace_tools_enabled(turn_context)
}

fn standalone_image_generation_available(
    turn_context: &TurnContext,
    extension_tools: &[Arc<dyn ToolExecutor<ExtensionToolCall>>],
) -> bool {
    standalone_image_generation_model_visible(turn_context)
        && extension_tools
            .iter()
            .any(|executor| executor.tool_name() == ToolName::namespaced("image_gen", "imagegen"))
}

fn standalone_web_run_available(
    extension_tools: &[Arc<dyn ToolExecutor<ExtensionToolCall>>],
) -> bool {
    let web_run = ToolName::namespaced("web", "run");
    extension_tools
        .iter()
        .any(|executor| executor.tool_name() == web_run)
}
