use std::collections::HashSet;

use crate::tools::handlers::extension_tools::ExtensionToolAdapter;
use crate::tools::planning::CoreToolPlanContext;
use crate::tools::planning::PlannedTools;
use crate::tools::planning::hosted::standalone_image_generation_model_visible;
use crate::tools::registry::ToolExposure;
use crate::tools::spec_plan::search_tool_enabled;
use ontocode_code_mode::PUBLIC_TOOL_NAME;
use ontocode_code_mode::WAIT_TOOL_NAME;
use ontocode_protocol::openai_models::ToolMode;
use ontocode_tools::TOOL_SEARCH_TOOL_NAME;
use ontocode_tools::ToolCall as ExtensionToolCall;
use ontocode_tools::ToolExecutor;
use ontocode_tools::ToolName;
use tracing::warn;

pub(crate) fn add_extension_tools(
    context: &CoreToolPlanContext<'_>,
    planned_tools: &mut PlannedTools,
) {
    append_extension_tool_executors(
        context.turn_context,
        context.extension_tool_executors,
        planned_tools,
    );
}

fn append_extension_tool_executors(
    turn_context: &crate::session::turn_context::TurnContext,
    executors: &[std::sync::Arc<dyn ToolExecutor<ExtensionToolCall>>],
    planned_tools: &mut PlannedTools,
) {
    if executors.is_empty() {
        return;
    }

    let mut reserved_tool_names = planned_tools
        .runtimes()
        .iter()
        .map(|executor| executor.tool_name())
        .collect::<HashSet<_>>();
    if matches!(
        turn_context.tool_mode,
        ToolMode::CodeMode | ToolMode::CodeModeOnly
    ) {
        reserved_tool_names.insert(ToolName::plain(PUBLIC_TOOL_NAME));
        reserved_tool_names.insert(ToolName::plain(WAIT_TOOL_NAME));
    }
    if search_tool_enabled(turn_context)
        && turn_context.provider.capabilities().namespace_tools
        && planned_tools
            .runtimes()
            .iter()
            .any(|executor| executor.exposure() == ToolExposure::Deferred)
    {
        reserved_tool_names.insert(ToolName::plain(TOOL_SEARCH_TOOL_NAME));
    }

    for executor in executors.iter().cloned() {
        let tool_name = executor.tool_name();
        if tool_name == ToolName::namespaced("image_gen", "imagegen")
            && !standalone_image_generation_model_visible(turn_context)
        {
            continue;
        }
        if !reserved_tool_names.insert(tool_name.clone()) {
            warn!("Skipping extension tool `{tool_name}`: tool already registered");
            continue;
        }
        planned_tools.add(ExtensionToolAdapter::new(executor));
    }
}
