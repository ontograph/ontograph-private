use crate::tools::handlers::ListMcpResourceTemplatesHandler;
use crate::tools::handlers::ListMcpResourcesHandler;
use crate::tools::handlers::McpHandler;
use crate::tools::handlers::ReadMcpResourceHandler;
use crate::tools::planning::CoreToolPlanContext;
use crate::tools::planning::PlannedTools;
use crate::tools::registry::ToolExposure;
use tracing::warn;

pub(crate) fn add_mcp_tools(context: &CoreToolPlanContext<'_>, planned_tools: &mut PlannedTools) {
    add_mcp_resource_tools(context, planned_tools);
    add_mcp_runtime_tools(context, planned_tools);
}

fn add_mcp_resource_tools(context: &CoreToolPlanContext<'_>, planned_tools: &mut PlannedTools) {
    if context.mcp_tools.is_some() {
        planned_tools.add(ListMcpResourcesHandler);
        planned_tools.add(ListMcpResourceTemplatesHandler);
        planned_tools.add(ReadMcpResourceHandler);
    }
}

fn add_mcp_runtime_tools(context: &CoreToolPlanContext<'_>, planned_tools: &mut PlannedTools) {
    if let Some(mcp_tools) = context.mcp_tools {
        for tool in mcp_tools {
            match McpHandler::new(tool.clone()) {
                Ok(handler) => planned_tools.add(handler),
                Err(err) => warn!(
                    "Skipping MCP tool `{}`: failed to build tool spec: {err}",
                    tool.canonical_tool_name()
                ),
            }
        }
    }

    if let Some(deferred_mcp_tools) = context.deferred_mcp_tools {
        for tool in deferred_mcp_tools {
            match McpHandler::new(tool.clone()) {
                Ok(handler) => planned_tools.add_with_exposure(handler, ToolExposure::Deferred),
                Err(err) => warn!(
                    "Skipping deferred MCP tool `{}`: failed to build tool spec: {err}",
                    tool.canonical_tool_name()
                ),
            }
        }
    }
}
