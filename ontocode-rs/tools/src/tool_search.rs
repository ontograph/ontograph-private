use crate::JsonSchema;
use crate::LoadableToolSpec;
use crate::ResponsesApiNamespaceTool;
use crate::ResponsesApiTool;
use crate::ToolName;
use crate::ToolSearchSourceInfo;
use crate::ToolSpec;
use crate::default_namespace_description;

#[derive(Clone)]
pub struct ToolSearchEntry {
    pub search_text: String,
    pub output: LoadableToolSpec,
    pub disabled_reason: Option<String>,
    pub source: Option<String>,
}

#[derive(Clone)]
pub struct ToolSearchInfo {
    pub entry: ToolSearchEntry,
    pub source_info: Option<ToolSearchSourceInfo>,
}

impl ToolSearchInfo {
    pub fn from_tool_spec(
        tool_name: &ToolName,
        spec: ToolSpec,
        source_info: Option<ToolSearchSourceInfo>,
    ) -> Option<Self> {
        let search_text = default_tool_search_text(tool_name, &spec, None, None);
        Self::from_spec(search_text, spec, source_info, None, None)
    }

    pub fn from_spec(
        search_text: String,
        spec: ToolSpec,
        source_info: Option<ToolSearchSourceInfo>,
        disabled_reason: Option<String>,
        source: Option<String>,
    ) -> Option<Self> {
        let output = match spec {
            ToolSpec::Function(mut tool) => {
                tool.defer_loading = Some(true);
                tool.output_schema = None;
                LoadableToolSpec::Function(tool)
            }
            ToolSpec::Namespace(mut namespace) => {
                if namespace.description.trim().is_empty() {
                    namespace.description = default_namespace_description(&namespace.name);
                }
                for tool in &mut namespace.tools {
                    let ResponsesApiNamespaceTool::Function(tool) = tool;
                    tool.defer_loading = Some(true);
                    tool.output_schema = None;
                }
                LoadableToolSpec::Namespace(namespace)
            }
            ToolSpec::ToolSearch { .. }
            | ToolSpec::ImageGeneration { .. }
            | ToolSpec::WebSearch { .. }
            | ToolSpec::Freeform(_) => return None,
        };

        Some(Self {
            entry: ToolSearchEntry {
                search_text,
                output,
                disabled_reason,
                source,
            },
            source_info,
        })
    }
}

pub fn default_tool_search_text(
    tool_name: &ToolName,
    spec: &ToolSpec,
    disabled_reason: Option<&str>,
    source: Option<&str>,
) -> String {
    let mut parts = Vec::new();
    push_search_part(&mut parts, tool_name.to_string());
    push_search_part(&mut parts, tool_name.name.replace('_', " "));
    if let Some(namespace) = &tool_name.namespace {
        push_search_part(&mut parts, namespace.clone());
    }

    match spec {
        ToolSpec::Function(tool) => append_function_search_text(tool, &mut parts),
        ToolSpec::Namespace(namespace) => {
            push_search_part(&mut parts, namespace.name.clone());
            push_search_part(&mut parts, namespace.description.clone());
            for tool in &namespace.tools {
                let ResponsesApiNamespaceTool::Function(tool) = tool;
                append_function_search_text(tool, &mut parts);
            }
        }
        ToolSpec::ToolSearch { description, .. } => {
            push_search_part(&mut parts, description.clone());
        }
        ToolSpec::ImageGeneration { .. } => {
            push_search_part(&mut parts, "image generation".to_string());
        }
        ToolSpec::WebSearch { .. } => {
            push_search_part(&mut parts, "web search".to_string());
        }
        ToolSpec::Freeform(tool) => {
            push_search_part(&mut parts, tool.name.clone());
            push_search_part(&mut parts, tool.description.clone());
            push_search_part(&mut parts, tool.format.syntax.clone());
        }
    }

    if let Some(reason) = disabled_reason {
        push_search_part(&mut parts, reason.to_string());
    }
    if let Some(src) = source {
        push_search_part(&mut parts, src.to_string());
    }

    parts.join(" ")
}

fn append_function_search_text(tool: &ResponsesApiTool, parts: &mut Vec<String>) {
    push_search_part(parts, tool.name.clone());
    push_search_part(parts, tool.name.replace('_', " "));
    push_search_part(parts, tool.description.clone());
    append_schema_search_text(&tool.parameters, parts);
}

fn append_schema_search_text(schema: &JsonSchema, parts: &mut Vec<String>) {
    if let Some(description) = &schema.description {
        push_search_part(parts, description.clone());
    }
    if let Some(properties) = &schema.properties {
        for (name, schema) in properties {
            push_search_part(parts, name.clone());
            append_schema_search_text(schema, parts);
        }
    }
    if let Some(items) = &schema.items {
        append_schema_search_text(items, parts);
    }
    if let Some(variants) = &schema.any_of {
        for variant in variants {
            append_schema_search_text(variant, parts);
        }
    }
}

fn push_search_part(parts: &mut Vec<String>, part: String) {
    let part = part.trim();
    if !part.is_empty() {
        parts.push(part.to_string());
    }
}
