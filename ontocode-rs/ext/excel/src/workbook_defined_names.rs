use std::fs::File;
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
use zip::ZipArchive;

use crate::backend::ExcelInspectionError;
use crate::backend::inspect_workbook_with_display_path;
use crate::formula_inspect::MAX_DEFINED_NAME_CHARS;
use crate::formula_inspect::parse_workbook_context;
use crate::preview::read_xml_entry;
use crate::tool::DefinedNameSummary;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::WorkbookFormat;
use crate::vba_extract::parse_tool_args;
use crate::vba_extract::resolve_workbook_path_from_model_arg;

pub(crate) const INSPECT_WORKBOOK_DEFINED_NAMES_TOOL_NAME: &str = "inspect_workbook_defined_names";

const INSPECT_WORKBOOK_DEFINED_NAMES_DESCRIPTION: &str = "Inspect bounded offline workbook defined-name metadata, including workbook-level defined-name counts, summaries, and explicit truncation warnings.";
const MAX_WORKBOOK_XML_BYTES: usize = 2 * 1024 * 1024;
const MAX_WARNINGS: usize = 32;

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectWorkbookDefinedNamesTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectWorkbookDefinedNamesArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectWorkbookDefinedNamesResult {
    pub mode: String,
    pub path: String,
    pub defined_name_count: usize,
    pub defined_names: Vec<DefinedNameSummary>,
    pub defined_names_sample: Vec<String>,
    pub inventory_truncated: bool,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectWorkbookDefinedNamesTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_DEFINED_NAMES_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(InspectWorkbookDefinedNamesArgs))
                .unwrap_or_else(|err| {
                    panic!("inspect_workbook_defined_names args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(InspectWorkbookDefinedNamesResult))
                .unwrap_or_else(|err| {
                    panic!("inspect_workbook_defined_names result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_WORKBOOK_DEFINED_NAMES_TOOL_NAME.to_string(),
                    description: INSPECT_WORKBOOK_DEFINED_NAMES_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_workbook_defined_names args schema should parse: {err}")
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
        let args = parse_tool_args::<InspectWorkbookDefinedNamesArgs>(
            &call,
            "excel.inspect_workbook_defined_names",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_workbook_defined_names workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = resolve_workbook_path_from_model_arg(
            "excel.inspect_workbook_defined_names",
            &args.path,
            &cwd,
        )?;
        let result = inspect_workbook_defined_names_from_workbook(
            &workbook_path,
            Path::new(args.path.trim()),
        )
        .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize workbook defined names: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectWorkbookDefinedNamesTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn inspect_workbook_defined_names_from_workbook(
    path: &Path,
    display_path: &Path,
) -> Result<InspectWorkbookDefinedNamesResult, ExcelInspectionError> {
    let workbook = inspect_workbook_with_display_path(path, display_path)?;
    match workbook.format {
        WorkbookFormat::Xlsx | WorkbookFormat::Xlsm => {
            inspect_openxml_workbook_defined_names(path, display_path)
        }
        WorkbookFormat::Xlsb => Err(ExcelInspectionError::Message(
            "excel.inspect_workbook_defined_names supports only .xlsx and .xlsm in this stage; .xlsb defined-name inventory remains unsupported"
                .to_string(),
        )),
        WorkbookFormat::Unknown => Err(ExcelInspectionError::Message(
            "excel.inspect_workbook_defined_names supports only .xlsx and .xlsm in this stage"
                .to_string(),
        )),
    }
}

fn inspect_openxml_workbook_defined_names(
    path: &Path,
    display_path: &Path,
) -> Result<InspectWorkbookDefinedNamesResult, ExcelInspectionError> {
    let file = File::open(path).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to open workbook {}: {err}", path.display()))
    })?;
    let mut archive = ZipArchive::new(file).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to read workbook archive {}: {err}",
            path.display()
        ))
    })?;
    let workbook_xml = read_xml_entry(&mut archive, "xl/workbook.xml", MAX_WORKBOOK_XML_BYTES)?;
    let context = parse_workbook_context(&workbook_xml)?;
    let mut warnings = Vec::new();
    push_defined_name_warnings(
        context.defined_name_count,
        context.defined_names.len(),
        context.truncated_defined_name_targets,
        context.unresolved_sheet_scope_count,
        &mut warnings,
    );
    let inventory_truncated = context.defined_name_count > context.defined_names.len();
    Ok(InspectWorkbookDefinedNamesResult {
        mode: "read_only_inspection".to_string(),
        path: display_path.display().to_string(),
        defined_name_count: context.defined_name_count,
        defined_names: context.defined_names,
        defined_names_sample: context.defined_names_sample,
        inventory_truncated,
        warnings,
    })
}

fn push_defined_name_warnings(
    defined_name_count: usize,
    reported_defined_name_count: usize,
    truncated_defined_name_targets: usize,
    unresolved_sheet_scope_count: usize,
    warnings: &mut Vec<String>,
) {
    if defined_name_count > reported_defined_name_count {
        push_warning(
            warnings,
            format!(
                "defined name inventory truncated to {reported_defined_name_count} of {defined_name_count} names"
            ),
        );
    }
    if truncated_defined_name_targets > 0 {
        push_warning(
            warnings,
            format!(
                "{truncated_defined_name_targets} defined name targets truncated to {MAX_DEFINED_NAME_CHARS} characters"
            ),
        );
    }
    if unresolved_sheet_scope_count > 0 {
        push_warning(
            warnings,
            format!(
                "{unresolved_sheet_scope_count} defined names kept unresolved localSheetId values without matching sheet names"
            ),
        );
    }
}

fn push_warning(warnings: &mut Vec<String>, warning: String) {
    if warnings.len() < MAX_WARNINGS && !warnings.iter().any(|item| item == &warning) {
        warnings.push(warning);
    }
}
