use crate::tools::flat_tool_name;
use ontocode_mcp::ToolInfo;
use ontocode_tools::ResponsesApiNamespace;
use ontocode_tools::ResponsesApiNamespaceTool;
use ontocode_tools::ToolName;
use ontocode_tools::ToolSearchInfo;
use ontocode_tools::ToolSearchSourceInfo;
use ontocode_tools::ToolSpec;
use ontocode_tools::mcp_tool_to_responses_api_tool;
use serde_json::Map;
use serde_json::Value;

const LEGACY_MCP_TOOL_NAME_PREFIX: &str = "mcp__";
const MCP_TOOL_NAME_DELIMITER: &str = "__";

pub(super) fn build_search_info(tool_info: &ToolInfo, spec: ToolSpec) -> Option<ToolSearchInfo> {
    let source_name = tool_info
        .connector_name
        .as_deref()
        .map(str::trim)
        .filter(|connector_name| !connector_name.is_empty())
        .unwrap_or_else(|| tool_info.server_name.trim());
    let source_info = (!source_name.is_empty()).then(|| ToolSearchSourceInfo {
        name: source_name.to_string(),
        description: tool_info
            .namespace_description
            .as_deref()
            .map(str::trim)
            .filter(|description| !description.is_empty())
            .map(str::to_string),
    });

    ToolSearchInfo::from_spec(
        build_mcp_search_text(tool_info),
        spec,
        source_info,
        None,
        None,
    )
}

pub(super) fn join_tool_name(tool_name: &ToolName) -> String {
    match tool_name.namespace.as_deref() {
        Some(namespace) => {
            let namespace = namespace.trim_end_matches('_');
            let name = tool_name.name.trim_start_matches('_');
            format!("{namespace}{MCP_TOOL_NAME_DELIMITER}{name}")
        }
        None => tool_name.name.clone(),
    }
}

pub(super) fn ensure_mcp_prefix(name: &str) -> String {
    if name.starts_with(LEGACY_MCP_TOOL_NAME_PREFIX) {
        name.to_string()
    } else {
        format!("{LEGACY_MCP_TOOL_NAME_PREFIX}{name}")
    }
}

pub(super) fn create_tool_spec(tool_info: &ToolInfo) -> Result<ToolSpec, serde_json::Error> {
    let tool_name = tool_info.canonical_tool_name();
    let tool = mcp_tool_to_responses_api_tool(&tool_name, &tool_info.tool)?;
    let description = tool_info
        .namespace_description
        .as_deref()
        .map(str::trim)
        .filter(|description| !description.is_empty())
        .map(str::to_string)
        .or_else(|| {
            tool_info
                .connector_name
                .as_deref()
                .map(str::trim)
                .filter(|connector_name| !connector_name.is_empty())
                .map(|connector_name| format!("Tools for working with {connector_name}."))
        })
        .unwrap_or_default();

    Ok(ToolSpec::Namespace(ResponsesApiNamespace {
        name: tool_info.callable_namespace.clone(),
        description,
        tools: vec![ResponsesApiNamespaceTool::Function(tool)],
    }))
}

pub(super) fn mcp_hook_tool_input(raw_arguments: &str) -> Value {
    if raw_arguments.trim().is_empty() {
        return Value::Object(Map::new());
    }

    serde_json::from_str(raw_arguments).unwrap_or_else(|_| Value::String(raw_arguments.to_string()))
}

fn build_mcp_search_text(info: &ToolInfo) -> String {
    let tool_name = info.canonical_tool_name();
    let mut schema_properties = info
        .tool
        .input_schema
        .get("properties")
        .and_then(serde_json::Value::as_object)
        .map(|map| map.keys().cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    schema_properties.sort();
    let mut parts = vec![
        flat_tool_name(&tool_name).into_owned(),
        info.callable_name.clone(),
        info.tool.name.to_string(),
        info.server_name.clone(),
    ];
    if let Some(title) = info.tool.title.as_deref().map(str::trim)
        && !title.is_empty()
    {
        parts.push(title.to_string());
    }
    if let Some(description) = info.tool.description.as_deref().map(str::trim)
        && !description.is_empty()
    {
        parts.push(description.to_string());
    }
    if let Some(connector_name) = info.connector_name.as_deref().map(str::trim)
        && !connector_name.is_empty()
    {
        parts.push(connector_name.to_string());
    }
    if let Some(namespace_description) = info.namespace_description.as_deref().map(str::trim)
        && !namespace_description.is_empty()
    {
        parts.push(namespace_description.to_string());
    }
    parts.extend(
        info.plugin_display_names
            .iter()
            .map(String::as_str)
            .map(str::trim)
            .filter(|display_name| !display_name.is_empty())
            .map(str::to_string),
    );
    parts.extend(schema_properties);
    parts.join(" ")
}
