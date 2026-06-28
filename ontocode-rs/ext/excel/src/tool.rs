use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;

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
use serde::de::DeserializeOwned;
use serde_json::to_value;

use crate::backend::ExcelInspectionError;
use crate::backend::inspect_workbook_with_display_path;
use crate::export::export_sheet_to_csv_with_display_path;
use crate::formula_ast::FormulaAstSummary;
use crate::formula_inspect::inspect_sheet_formulas_with_display_path;
use crate::preview::read_sheet_preview_with_display_path;

pub(crate) const EXCEL_NAMESPACE: &str = "excel";
pub(crate) const INSPECT_WORKBOOK_TOOL_NAME: &str = "inspect_workbook";
pub(crate) const READ_SHEET_PREVIEW_TOOL_NAME: &str = "read_sheet_preview";
pub(crate) const INSPECT_SHEET_FORMULAS_TOOL_NAME: &str = "inspect_sheet_formulas";
pub(crate) const EXPORT_SHEET_TO_CSV_TOOL_NAME: &str = "export_sheet_to_csv";

const INSPECT_WORKBOOK_DESCRIPTION: &str =
    "Inspect a workbook package and return bounded metadata for .xlsx, .xlsm, or .xlsb files.";
const READ_SHEET_PREVIEW_DESCRIPTION: &str =
    "Read a bounded preview from one worksheet in a .xlsx or .xlsm workbook.";
const INSPECT_SHEET_FORMULAS_DESCRIPTION: &str =
    "Inspect bounded formula metadata from one worksheet in a .xlsx or .xlsm workbook.";
const EXPORT_SHEET_TO_CSV_DESCRIPTION: &str =
    "Export one worksheet from a .xlsx or .xlsm workbook to a local CSV file.";

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectionTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Clone, Default)]
pub(crate) struct ExcelReadSheetPreviewTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectSheetFormulasTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Clone, Default)]
pub(crate) struct ExcelExportSheetToCsvTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Default)]
pub(crate) struct ExcelThreadState {
    current_cwd: Mutex<Option<PathBuf>>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectWorkbookArgs {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReadSheetPreviewArgs {
    pub path: String,
    pub sheet: SheetSelector,
    #[serde(default)]
    pub max_rows: Option<usize>,
    #[serde(default)]
    pub cell_content: CellContentMode,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectSheetFormulasArgs {
    pub path: String,
    pub sheet: SheetSelector,
    #[serde(default)]
    pub max_formulas: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ExportSheetToCsvArgs {
    pub path: String,
    pub sheet: SheetSelector,
    #[serde(default)]
    pub output_csv_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WorkbookFormat {
    Xlsx,
    Xlsm,
    Xlsb,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SheetVisibility {
    Visible,
    Hidden,
    VeryHidden,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SheetKind {
    Worksheet,
    Chartsheet,
    DialogSheet,
    MacroSheet,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum SheetSelector {
    Name { name: String },
    Index { index: usize },
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CellContentMode {
    #[default]
    Values,
    ValuesAndFormulas,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SheetSummary {
    pub name: Option<String>,
    pub sheet_id: Option<u32>,
    pub relationship_id: Option<String>,
    pub part_path: Option<String>,
    pub visibility: SheetVisibility,
    pub kind: SheetKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct MarkerSummary {
    pub category: String,
    pub count: usize,
    pub part_paths_sample: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Default)]
pub(crate) struct WorkbookMarkers {
    pub has_vba_project: bool,
    pub has_macro_enabled_package: bool,
    pub has_power_query: bool,
    pub has_connections: bool,
    pub has_custom_xml: bool,
    pub has_external_links: bool,
    pub has_tables: bool,
    pub has_comments: bool,
    pub has_drawings: bool,
    pub has_embedded_objects: bool,
    pub has_charts: bool,
    pub has_pivot_tables: bool,
    pub has_formulas: bool,
    pub has_xlsb_package: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectWorkbookResult {
    pub path: String,
    pub format: WorkbookFormat,
    pub package_part_count: usize,
    pub package_parts_sample: Vec<String>,
    pub sheets: Vec<SheetSummary>,
    pub markers: WorkbookMarkers,
    pub marker_summaries: Vec<MarkerSummary>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SheetPreview {
    pub name: String,
    pub sheet_id: Option<u32>,
    pub part_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SheetPreviewCell {
    pub reference: String,
    pub value: Option<String>,
    pub formula: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SheetPreviewRow {
    pub row_index: u32,
    pub cells: Vec<SheetPreviewCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SheetDimension {
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SheetDataValidationSummary {
    pub ranges_sample: Vec<String>,
    pub range_count: usize,
    pub validation_type: String,
    pub operator: Option<String>,
    pub allow_blank: Option<bool>,
    pub dropdown_visible: Option<bool>,
    pub error_style: Option<String>,
    pub show_error_message: Option<bool>,
    pub formula1: Option<String>,
    pub formula2: Option<String>,
    pub resolved_values_source: String,
    pub resolved_values_sample: Vec<String>,
    pub resolved_values_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ReadSheetPreviewResult {
    pub path: String,
    pub sheet: SheetPreview,
    pub dimension: Option<SheetDimension>,
    pub max_rows_applied: usize,
    pub cell_content: CellContentMode,
    pub rows: Vec<SheetPreviewRow>,
    pub data_validations: Vec<SheetDataValidationSummary>,
    pub truncated: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FormulaSqlPreviewState {
    ReviewOnly,
    #[default]
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FormulaSqlReferenceKind {
    Cell,
    Range,
    DefinedName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct FormulaSqlReferenceSummary {
    pub kind: FormulaSqlReferenceKind,
    pub reference: String,
    pub sheet_name: Option<String>,
    pub same_row: Option<bool>,
    pub sql_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct FormulaSqlPreviewSummary {
    pub state: FormulaSqlPreviewState,
    pub sql_expression: Option<String>,
    pub references: Vec<FormulaSqlReferenceSummary>,
    pub blocker_reasons: Vec<String>,
    pub cached_value_present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SheetFormulaSummary {
    pub reference: String,
    pub formula: String,
    pub cached_value: Option<String>,
    pub parse: FormulaAstSummary,
    #[serde(default)]
    pub sql_preview: FormulaSqlPreviewSummary,
    pub warnings: Vec<String>,
    pub formula_type: Option<String>,
    pub shared_index: Option<u32>,
    pub shared_range: Option<String>,
    pub style_index: Option<u32>,
    pub number_format_id: Option<u32>,
    pub number_format_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectSheetFormulasResult {
    pub path: String,
    pub sheet: SheetPreview,
    pub max_formulas_applied: usize,
    pub formulas: Vec<SheetFormulaSummary>,
    pub calculation_mode: Option<String>,
    pub full_calc_on_load: Option<bool>,
    pub force_full_calc: Option<bool>,
    pub defined_names: Vec<DefinedNameSummary>,
    pub defined_names_sample: Vec<String>,
    pub has_external_links: bool,
    pub truncated: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct DefinedNameSummary {
    pub name: String,
    pub sheet_scope: Option<String>,
    pub local_sheet_id: Option<u32>,
    pub hidden: Option<bool>,
    pub target: String,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ExportSheetToCsvResult {
    pub path: String,
    pub sheet: SheetPreview,
    pub output_csv_path: String,
    pub row_count: usize,
    pub column_count: usize,
    pub truncated: bool,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectionTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema = serde_json::to_value(schemars::schema_for!(InspectWorkbookArgs))
            .unwrap_or_else(|err| panic!("inspect_workbook args schema should serialize: {err}"));
        let output_schema = serde_json::to_value(schemars::schema_for!(InspectWorkbookResult))
            .unwrap_or_else(|err| panic!("inspect_workbook result schema should serialize: {err}"));
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_WORKBOOK_TOOL_NAME.to_string(),
                    description: INSPECT_WORKBOOK_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_workbook args schema should parse: {err}")
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
        let args = parse_tool_args::<InspectWorkbookArgs>(&call, "excel.inspect_workbook")?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_workbook workspace context is unavailable for this turn".to_string(),
            )
        })?;
        let path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let result = inspect_workbook_with_display_path(&path, Path::new(args.path.trim()))
            .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize workbook inventory: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectionTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelReadSheetPreviewTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, READ_SHEET_PREVIEW_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema = serde_json::to_value(schemars::schema_for!(ReadSheetPreviewArgs))
            .unwrap_or_else(|err| panic!("read_sheet_preview args schema should serialize: {err}"));
        let output_schema = serde_json::to_value(schemars::schema_for!(ReadSheetPreviewResult))
            .unwrap_or_else(|err| {
                panic!("read_sheet_preview result schema should serialize: {err}")
            });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: READ_SHEET_PREVIEW_TOOL_NAME.to_string(),
                    description: READ_SHEET_PREVIEW_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("read_sheet_preview args schema should parse: {err}")
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
        let args = parse_tool_args::<ReadSheetPreviewArgs>(&call, "excel.read_sheet_preview")?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.read_sheet_preview workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let result = read_sheet_preview_with_display_path(
            &path,
            Path::new(args.path.trim()),
            &args.sheet,
            args.max_rows,
            args.cell_content,
        )
        .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!("failed to serialize sheet preview: {err}"))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelReadSheetPreviewTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectSheetFormulasTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_SHEET_FORMULAS_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema = serde_json::to_value(schemars::schema_for!(InspectSheetFormulasArgs))
            .unwrap_or_else(|err| {
                panic!("inspect_sheet_formulas args schema should serialize: {err}")
            });
        let output_schema = serde_json::to_value(schemars::schema_for!(InspectSheetFormulasResult))
            .unwrap_or_else(|err| {
                panic!("inspect_sheet_formulas result schema should serialize: {err}")
            });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_SHEET_FORMULAS_TOOL_NAME.to_string(),
                    description: INSPECT_SHEET_FORMULAS_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_sheet_formulas args schema should parse: {err}")
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
        let args =
            parse_tool_args::<InspectSheetFormulasArgs>(&call, "excel.inspect_sheet_formulas")?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_sheet_formulas workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let result = inspect_sheet_formulas_with_display_path(
            &path,
            Path::new(args.path.trim()),
            &args.sheet,
            args.max_formulas,
        )
        .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize sheet formula inventory: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectSheetFormulasTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelExportSheetToCsvTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, EXPORT_SHEET_TO_CSV_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema = serde_json::to_value(schemars::schema_for!(ExportSheetToCsvArgs))
            .unwrap_or_else(|err| {
                panic!("export_sheet_to_csv args schema should serialize: {err}")
            });
        let output_schema = serde_json::to_value(schemars::schema_for!(ExportSheetToCsvResult))
            .unwrap_or_else(|err| {
                panic!("export_sheet_to_csv result schema should serialize: {err}")
            });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: EXPORT_SHEET_TO_CSV_TOOL_NAME.to_string(),
                    description: EXPORT_SHEET_TO_CSV_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("export_sheet_to_csv args schema should parse: {err}")
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
        let args = parse_tool_args::<ExportSheetToCsvArgs>(&call, "excel.export_sheet_to_csv")?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.export_sheet_to_csv workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let workbook_display_path = Path::new(args.path.trim());
        let output_display_path = args.output_csv_path.as_deref().map_or_else(
            || default_output_csv_display_path(workbook_display_path, &args.sheet),
            PathBuf::from,
        );
        let output_path = if let Some(output_csv_path) = args.output_csv_path.as_deref() {
            csv_output_path_from_model_arg(output_csv_path, &cwd)?
        } else {
            cwd.join(&output_display_path)
        };
        let result = export_sheet_to_csv_with_display_path(
            &workbook_path,
            workbook_display_path,
            &args.sheet,
            &output_path,
            &output_display_path,
        )
        .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize sheet csv export: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelExportSheetToCsvTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

fn parse_tool_args<T: DeserializeOwned>(
    call: &ToolCall,
    tool_name: &str,
) -> Result<T, FunctionCallError> {
    let arguments = call.function_arguments()?;
    serde_json::from_str(arguments).map_err(|err| {
        FunctionCallError::RespondToModel(format!("invalid {tool_name} arguments: {err}"))
    })
}

pub(crate) fn workbook_path_from_model_arg(
    path: &str,
    cwd: &Path,
) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "excel.inspect_workbook path must not be empty".to_string(),
        ));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(
            "excel.inspect_workbook path must be a local workbook path".to_string(),
        ));
    }

    let path = Path::new(trimmed);
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(FunctionCallError::RespondToModel(
            "excel.inspect_workbook path must be relative and stay within the current working directory"
                .to_string(),
        ));
    }

    let Some(extension) = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
    else {
        return Err(FunctionCallError::RespondToModel(
            "excel.inspect_workbook path must end in .xlsx, .xlsm, or .xlsb".to_string(),
        ));
    };
    if !matches!(extension.as_str(), "xlsx" | "xlsm" | "xlsb") {
        return Err(FunctionCallError::RespondToModel(
            "excel.inspect_workbook path must end in .xlsx, .xlsm, or .xlsb".to_string(),
        ));
    }

    let resolved_path = cwd.join(path);
    let mut scoped_path = cwd.to_path_buf();
    for component in path.components() {
        let Component::Normal(segment) = component else {
            continue;
        };
        scoped_path.push(segment);
        let Ok(metadata) = std::fs::symlink_metadata(&scoped_path) else {
            break;
        };
        if metadata.file_type().is_symlink() {
            return Err(FunctionCallError::RespondToModel(
                "excel.inspect_workbook path must not traverse symlinks".to_string(),
            ));
        }
    }

    Ok(resolved_path)
}

fn csv_output_path_from_model_arg(path: &str, cwd: &Path) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "excel.export_sheet_to_csv output_csv_path must not be empty".to_string(),
        ));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(
            "excel.export_sheet_to_csv output_csv_path must be a local csv path".to_string(),
        ));
    }

    let path = Path::new(trimmed);
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(FunctionCallError::RespondToModel(
            "excel.export_sheet_to_csv output_csv_path must be relative and stay within the current working directory"
                .to_string(),
        ));
    }
    if path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
        != Some("csv")
    {
        return Err(FunctionCallError::RespondToModel(
            "excel.export_sheet_to_csv output_csv_path must end in .csv".to_string(),
        ));
    }

    let resolved_path = cwd.join(path);
    let mut scoped_path = cwd.to_path_buf();
    for component in path.components() {
        let Component::Normal(segment) = component else {
            continue;
        };
        scoped_path.push(segment);
        let Ok(metadata) = std::fs::symlink_metadata(&scoped_path) else {
            break;
        };
        if metadata.file_type().is_symlink() {
            return Err(FunctionCallError::RespondToModel(
                "excel.export_sheet_to_csv output_csv_path must not traverse symlinks".to_string(),
            ));
        }
    }
    Ok(resolved_path)
}

fn default_output_csv_display_path(workbook_path: &Path, sheet: &SheetSelector) -> PathBuf {
    let mut output = workbook_path.to_path_buf();
    let stem = workbook_path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("sheet");
    let sheet_suffix = match sheet {
        SheetSelector::Name { name } => sanitize_sheet_name(name),
        SheetSelector::Index { index } => format!("sheet-{}", index + 1),
    };
    output.set_file_name(format!("{stem}-{sheet_suffix}.csv"));
    output
}

fn sanitize_sheet_name(name: &str) -> String {
    let sanitized = name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    if sanitized.is_empty() {
        "sheet".to_string()
    } else {
        sanitized
    }
}

impl From<ExcelInspectionError> for FunctionCallError {
    fn from(value: ExcelInspectionError) -> Self {
        FunctionCallError::RespondToModel(value.to_string())
    }
}

impl ExcelThreadState {
    pub(crate) fn set_current_cwd(&self, cwd: PathBuf) {
        *self
            .current_cwd
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = Some(cwd);
    }

    pub(crate) fn current_cwd(&self) -> Option<PathBuf> {
        self.current_cwd
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}
