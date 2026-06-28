use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

use ontocode_extension_api::FunctionCallError;
use ontocode_extension_api::JsonToolOutput;
use ontocode_extension_api::ToolCall;
use ontocode_extension_api::ToolExecutor;
use ontocode_extension_api::ToolName;
use ontocode_extension_api::ToolOutput;
use ontocode_extension_api::ToolSpec;
use ontocode_extension_api::parse_tool_input_schema;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_value;

use crate::powerquery_extract::PowerQueryLoadTargetHint;
use crate::powerquery_extract::PowerQueryWorkbookConnectionSummary;
use crate::powerquery_extract::inspect_workbook_connection_inventory_from_workbook;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::vba_extract::parse_tool_args;
use crate::vba_extract::resolve_workbook_path_from_model_arg;

pub(crate) const INSPECT_WORKBOOK_CONNECTIONS_TOOL_NAME: &str = "inspect_workbook_connections";

const INSPECT_WORKBOOK_CONNECTIONS_DESCRIPTION: &str = "Inspect bounded offline workbook connection metadata, including connection counts, connection summaries, and provable warning hints.";
const MAX_WARNINGS: usize = 32;

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectWorkbookConnectionsTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectWorkbookConnectionsArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectWorkbookConnectionsResult {
    pub mode: String,
    pub path: String,
    pub connection_count: usize,
    pub connections: Vec<PowerQueryWorkbookConnectionSummary>,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectWorkbookConnectionsTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_CONNECTIONS_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(InspectWorkbookConnectionsArgs))
                .unwrap_or_else(|err| {
                    panic!("inspect_workbook_connections args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(InspectWorkbookConnectionsResult))
                .unwrap_or_else(|err| {
                    panic!("inspect_workbook_connections result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_WORKBOOK_CONNECTIONS_TOOL_NAME.to_string(),
                    description: INSPECT_WORKBOOK_CONNECTIONS_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_workbook_connections args schema should parse: {err}")
                    }),
                    output_schema: Some(output_schema),
                },
            )],
        })
    }

    fn exposure(&self) -> ontocode_tools::ToolExposure {
        ontocode_tools::ToolExposure::DirectModelOnly
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    async fn handle(&self, call: ToolCall) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args = parse_tool_args::<InspectWorkbookConnectionsArgs>(
            &call,
            "excel.inspect_workbook_connections",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_workbook_connections workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = resolve_workbook_path_from_model_arg(
            "excel.inspect_workbook_connections",
            &args.path,
            &cwd,
        )?;
        let result =
            inspect_workbook_connections_from_workbook(&workbook_path, Path::new(args.path.trim()));
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize workbook connection metadata: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectWorkbookConnectionsTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn inspect_workbook_connections_from_workbook(
    path: &Path,
    display_path: &Path,
) -> InspectWorkbookConnectionsResult {
    let mut inventory = inspect_workbook_connection_inventory_from_workbook(path, display_path);
    push_ambiguous_load_target_warnings(&inventory.connections, &mut inventory.warnings);
    push_unsupported_connection_kind_warnings(&inventory.connections, &mut inventory.warnings);
    InspectWorkbookConnectionsResult {
        mode: "read_only_inspection".to_string(),
        path: display_path.display().to_string(),
        connection_count: inventory.connection_count,
        connections: inventory.connections,
        warnings: inventory.warnings,
    }
}

fn push_ambiguous_load_target_warnings(
    connections: &[PowerQueryWorkbookConnectionSummary],
    warnings: &mut Vec<String>,
) {
    let mut target_hints_by_query = BTreeMap::<&str, BTreeSet<&'static str>>::new();
    for connection in connections {
        let Some(query_name_hint) = connection.query_name_hint.as_deref() else {
            continue;
        };
        target_hints_by_query
            .entry(query_name_hint)
            .or_default()
            .insert(match connection.load_target_hint {
                PowerQueryLoadTargetHint::WorkbookConnection => "workbook_connection",
                PowerQueryLoadTargetHint::DataModel => "data_model",
                PowerQueryLoadTargetHint::Unknown => "unknown",
            });
    }

    for (query_name_hint, target_hints) in target_hints_by_query {
        if target_hints.contains("workbook_connection") && target_hints.contains("data_model") {
            push_warning(
                warnings,
                format!(
                    "query-name hint `{query_name_hint}` maps to both workbook_connection and data_model load targets"
                ),
            );
        }
    }
}

fn push_unsupported_connection_kind_warnings(
    connections: &[PowerQueryWorkbookConnectionSummary],
    warnings: &mut Vec<String>,
) {
    for (index, connection) in connections.iter().enumerate() {
        if connection.location.is_some() || connection.query_name_hint.is_some() {
            continue;
        }
        if connection.load_target_hint == PowerQueryLoadTargetHint::DataModel
            || connection.connection_type.as_deref() == Some("100")
        {
            continue;
        }
        let label = connection
            .name
            .as_deref()
            .or(connection.id.as_deref())
            .map(str::to_string)
            .unwrap_or_else(|| format!("entry {}", index + 1));
        let connection_type = connection.connection_type.as_deref().unwrap_or("unknown");
        push_warning(
            warnings,
            format!(
                "workbook connection `{label}` uses unsupported connection kind `{connection_type}`; only bounded offline metadata is reported"
            ),
        );
    }
}

fn push_warning(warnings: &mut Vec<String>, warning: String) {
    if warnings.len() < MAX_WARNINGS && !warnings.iter().any(|item| item == &warning) {
        warnings.push(warning);
    }
}
