use crate::tools::handlers::DynamicToolHandler;
use crate::tools::planning::CoreToolPlanContext;
use crate::tools::planning::PlannedTools;

pub(crate) fn add_dynamic_tools(
    context: &CoreToolPlanContext<'_>,
    planned_tools: &mut PlannedTools,
) {
    for tool in context.dynamic_tools {
        let Some(handler) = DynamicToolHandler::new(tool) else {
            tracing::error!(
                "Failed to convert dynamic tool {:?} to OpenAI tool",
                tool.name
            );
            continue;
        };

        planned_tools.add(handler);
    }
}
