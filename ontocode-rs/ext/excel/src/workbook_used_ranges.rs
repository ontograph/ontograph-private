use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use calamine::Reader as CalamineReader;
use calamine::open_workbook_auto;
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
use crate::preview::parse_sheet_dimension;
use crate::preview::read_xml_entry;
use crate::preview::xlsb_range_dimension;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::SheetDimension;
use crate::tool::SheetKind;
use crate::tool::WorkbookFormat;
use crate::vba_extract::parse_tool_args;
use crate::vba_extract::resolve_workbook_path_from_model_arg;

pub(crate) const INSPECT_WORKBOOK_USED_RANGES_TOOL_NAME: &str = "inspect_workbook_used_ranges";

const INSPECT_WORKBOOK_USED_RANGES_DESCRIPTION: &str = "Inspect bounded offline workbook used-range metadata, including per-sheet range references and explicit warnings for unsupported or unreadable sheets.";
const MAX_WORKSHEETS: usize = 128;
const MAX_WORKSHEET_XML_BYTES: usize = 2 * 1024 * 1024;
const MAX_WARNINGS: usize = 32;

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectWorkbookUsedRangesTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectWorkbookUsedRangesArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct WorkbookUsedRangeSummary {
    pub sheet_name: String,
    pub sheet_id: Option<u32>,
    pub part_path: Option<String>,
    pub used_range: Option<SheetDimension>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectWorkbookUsedRangesResult {
    pub mode: String,
    pub path: String,
    pub sheet_count: usize,
    pub sheet_used_ranges: Vec<WorkbookUsedRangeSummary>,
    pub inventory_truncated: bool,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectWorkbookUsedRangesTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_USED_RANGES_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(InspectWorkbookUsedRangesArgs))
                .unwrap_or_else(|err| {
                    panic!("inspect_workbook_used_ranges args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(InspectWorkbookUsedRangesResult))
                .unwrap_or_else(|err| {
                    panic!("inspect_workbook_used_ranges result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_WORKBOOK_USED_RANGES_TOOL_NAME.to_string(),
                    description: INSPECT_WORKBOOK_USED_RANGES_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_workbook_used_ranges args schema should parse: {err}")
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
        let args = parse_tool_args::<InspectWorkbookUsedRangesArgs>(
            &call,
            "excel.inspect_workbook_used_ranges",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_workbook_used_ranges workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = resolve_workbook_path_from_model_arg(
            "excel.inspect_workbook_used_ranges",
            &args.path,
            &cwd,
        )?;
        let result =
            inspect_workbook_used_ranges_from_workbook(&workbook_path, Path::new(args.path.trim()))
                .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize workbook used ranges: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectWorkbookUsedRangesTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn inspect_workbook_used_ranges_from_workbook(
    path: &Path,
    display_path: &Path,
) -> Result<InspectWorkbookUsedRangesResult, ExcelInspectionError> {
    let workbook = inspect_workbook_with_display_path(path, display_path)?;
    match workbook.format {
        WorkbookFormat::Xlsx | WorkbookFormat::Xlsm => {
            inspect_openxml_workbook_used_ranges(path, display_path, &workbook)
        }
        WorkbookFormat::Xlsb => inspect_xlsb_workbook_used_ranges(path, display_path, &workbook),
        WorkbookFormat::Unknown => Err(ExcelInspectionError::Message(
            "excel.inspect_workbook_used_ranges supports only .xlsx, .xlsm, or .xlsb in this stage"
                .to_string(),
        )),
    }
}

fn inspect_openxml_workbook_used_ranges(
    path: &Path,
    display_path: &Path,
    workbook: &crate::tool::InspectWorkbookResult,
) -> Result<InspectWorkbookUsedRangesResult, ExcelInspectionError> {
    let file = File::open(path).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to open workbook {}: {err}", path.display()))
    })?;
    let mut archive = ZipArchive::new(file).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to read workbook archive {}: {err}",
            path.display()
        ))
    })?;
    let sheet_count = workbook.sheets.len();
    let inventory_truncated = sheet_count > MAX_WORKSHEETS;
    let mut warnings = Vec::new();
    if inventory_truncated {
        push_warning(
            &mut warnings,
            format!(
                "workbook used-range inventory truncated to {MAX_WORKSHEETS} of {sheet_count} sheets"
            ),
        );
    }
    let sheet_used_ranges = workbook
        .sheets
        .iter()
        .take(MAX_WORKSHEETS)
        .map(|sheet| {
            let used_range = if sheet.kind != SheetKind::Worksheet {
                push_warning(
                    &mut warnings,
                    format!(
                        "sheet `{}` uses unsupported kind `{:?}`; no used range reported",
                        sheet.name.clone().unwrap_or_default(),
                        sheet.kind
                    ),
                );
                None
            } else if let Some(part_path) = sheet.part_path.as_deref() {
                match read_xml_entry(&mut archive, part_path, MAX_WORKSHEET_XML_BYTES)
                    .and_then(|xml| parse_sheet_dimension(&xml))
                {
                    Ok(dimension) => dimension,
                    Err(err) => {
                        push_warning(
                            &mut warnings,
                            format!(
                                "sheet `{}` used range could not be read: {err}",
                                sheet.name.clone().unwrap_or_default()
                            ),
                        );
                        None
                    }
                }
            } else {
                push_warning(
                    &mut warnings,
                    format!(
                        "sheet `{}` has no worksheet part path; no used range reported",
                        sheet.name.clone().unwrap_or_default()
                    ),
                );
                None
            };
            WorkbookUsedRangeSummary {
                sheet_name: sheet.name.clone().unwrap_or_default(),
                sheet_id: sheet.sheet_id,
                part_path: sheet.part_path.clone(),
                used_range,
            }
        })
        .collect();

    Ok(InspectWorkbookUsedRangesResult {
        mode: "read_only_inspection".to_string(),
        path: display_path.display().to_string(),
        sheet_count,
        sheet_used_ranges,
        inventory_truncated,
        warnings,
    })
}

fn inspect_xlsb_workbook_used_ranges(
    path: &Path,
    display_path: &Path,
    workbook: &crate::tool::InspectWorkbookResult,
) -> Result<InspectWorkbookUsedRangesResult, ExcelInspectionError> {
    let mut xlsb = open_workbook_auto(path).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to open .xlsb workbook {} for excel.inspect_workbook_used_ranges: {err}",
            path.display()
        ))
    })?;
    let sheet_names = xlsb.sheet_names().to_vec();
    let sheet_count = sheet_names.len();
    let inventory_truncated = sheet_count > MAX_WORKSHEETS;
    let mut warnings = Vec::new();
    if inventory_truncated {
        push_warning(
            &mut warnings,
            format!(
                "workbook used-range inventory truncated to {MAX_WORKSHEETS} of {sheet_count} sheets"
            ),
        );
    }
    let mut sheet_used_ranges = Vec::new();
    for (index, sheet_name) in sheet_names.iter().take(MAX_WORKSHEETS).enumerate() {
        let used_range = match xlsb.worksheet_range(sheet_name) {
            Ok(worksheet) => {
                let formulas = xlsb.worksheet_formula(sheet_name).ok();
                xlsb_range_dimension(&worksheet, formulas.as_ref())
            }
            Err(err) => {
                push_warning(
                    &mut warnings,
                    format!("sheet `{sheet_name}` used range could not be read: {err}"),
                );
                None
            }
        };
        sheet_used_ranges.push(WorkbookUsedRangeSummary {
            sheet_name: sheet_name.clone(),
            sheet_id: workbook.sheets.get(index).and_then(|sheet| sheet.sheet_id),
            part_path: workbook
                .sheets
                .get(index)
                .and_then(|sheet| sheet.part_path.clone()),
            used_range,
        });
    }
    if workbook
        .sheets
        .iter()
        .any(|sheet| sheet.part_path.is_none())
    {
        push_warning(
            &mut warnings,
            "excel.inspect_workbook_used_ranges could not resolve all .xlsb worksheet part paths in this stage"
                .to_string(),
        );
    }

    Ok(InspectWorkbookUsedRangesResult {
        mode: "read_only_inspection".to_string(),
        path: display_path.display().to_string(),
        sheet_count,
        sheet_used_ranges,
        inventory_truncated,
        warnings,
    })
}

fn push_warning(warnings: &mut Vec<String>, warning: String) {
    if warnings.len() < MAX_WARNINGS && !warnings.iter().any(|item| item == &warning) {
        warnings.push(warning);
    }
}
