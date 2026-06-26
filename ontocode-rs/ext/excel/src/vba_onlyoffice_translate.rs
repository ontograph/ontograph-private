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

use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::vba_onlyoffice_analyze::AnalyzeVbaOnlyofficeMigrationResult;
use crate::vba_onlyoffice_analyze::AnalyzeVbaOperationSummary;
use crate::vba_onlyoffice_analyze::AnalyzeVbaProcedureSummary;
use crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration;

pub(crate) const TRANSLATE_VBA_TO_ONLYOFFICE_JS_PREVIEW_TOOL_NAME: &str =
    "translate_vba_to_onlyoffice_js_preview";

const TRANSLATE_VBA_TO_ONLYOFFICE_JS_PREVIEW_DESCRIPTION: &str =
    "Translate analyzer-approved VBA into a bounded ONLYOFFICE JavaScript macro preview.";
const MAX_MACRO_VALUE_CHARS: usize = 8_192;

#[derive(Clone, Default)]
pub(crate) struct ExcelTranslateVbaToOnlyofficeJsPreviewTool {
    _thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct TranslateVbaToOnlyofficeJsPreviewArgs {
    pub source_text: String,
    #[serde(default)]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct TranslateVbaToOnlyofficeJsPreviewResult {
    pub macro_value: String,
    pub function_body: String,
    pub procedure_summaries: Vec<AnalyzeVbaProcedureSummary>,
    pub unsupported_operations: Vec<AnalyzeVbaOperationSummary>,
    pub redactions: Vec<String>,
    pub warnings: Vec<String>,
    pub success: bool,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelTranslateVbaToOnlyofficeJsPreviewTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(
            EXCEL_NAMESPACE,
            TRANSLATE_VBA_TO_ONLYOFFICE_JS_PREVIEW_TOOL_NAME,
        )
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(TranslateVbaToOnlyofficeJsPreviewArgs))
                .unwrap_or_else(|err| {
                    panic!(
                        "translate_vba_to_onlyoffice_js_preview args schema should serialize: {err}"
                    )
                });
        let output_schema = serde_json::to_value(schemars::schema_for!(
            TranslateVbaToOnlyofficeJsPreviewResult
        ))
        .unwrap_or_else(|err| {
            panic!("translate_vba_to_onlyoffice_js_preview result schema should serialize: {err}")
        });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: TRANSLATE_VBA_TO_ONLYOFFICE_JS_PREVIEW_TOOL_NAME.to_string(),
                    description: TRANSLATE_VBA_TO_ONLYOFFICE_JS_PREVIEW_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!(
                            "translate_vba_to_onlyoffice_js_preview args schema should parse: {err}"
                        )
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
        let args = parse_tool_args::<TranslateVbaToOnlyofficeJsPreviewArgs>(
            &call,
            "excel.translate_vba_to_onlyoffice_js_preview",
        )?;
        let result =
            translate_vba_to_onlyoffice_js_preview(&args.source_text, args.source_name.as_deref());
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize ONLYOFFICE macro preview: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelTranslateVbaToOnlyofficeJsPreviewTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self {
            _thread_state: thread_state,
        }
    }
}

pub(crate) fn translate_vba_to_onlyoffice_js_preview(
    source_text: &str,
    source_name: Option<&str>,
) -> TranslateVbaToOnlyofficeJsPreviewResult {
    let analysis = analyze_vba_onlyoffice_migration(source_text, source_name);
    translate_analyzed_vba_to_onlyoffice_js_preview(analysis)
}

pub(crate) fn translate_analyzed_vba_to_onlyoffice_js_preview(
    analysis: AnalyzeVbaOnlyofficeMigrationResult,
) -> TranslateVbaToOnlyofficeJsPreviewResult {
    let redactions = collect_redactions(&analysis);
    let mut warnings = analysis.warnings.clone();
    if !analysis.success
        || analysis.requires_manual_rewrite
        || !analysis.unsupported_operations.is_empty()
        || !warnings.is_empty()
        || !redactions.is_empty()
    {
        if warnings.is_empty() {
            warnings.push(
                "ONLYOFFICE macro preview was not emitted because analyzer output is not fully safe."
                    .to_string(),
            );
        }
        return non_emitting_result(analysis, redactions, warnings);
    }

    let mut body_lines = vec![
        "    let worksheet = Api.GetActiveSheet();".to_string(),
        "    let workbook = Api.GetActiveWorkbook();".to_string(),
    ];
    for operation in &analysis.supported_operations {
        let Some(line) = emit_operation(operation) else {
            warnings.push(format!(
                "ONLYOFFICE macro preview was not emitted because operation {} has no approved mapping.",
                operation.operation
            ));
            return non_emitting_result(analysis, redactions, warnings);
        };
        body_lines.push(format!("    {line}"));
    }

    let function_body = body_lines.join("\n");
    let macro_value = format!("(function()\n{{\n{function_body}\n}})();");
    if macro_value.len() > MAX_MACRO_VALUE_CHARS {
        warnings.push(format!(
            "ONLYOFFICE macro preview exceeded {MAX_MACRO_VALUE_CHARS} characters."
        ));
        return non_emitting_result(analysis, redactions, warnings);
    }

    TranslateVbaToOnlyofficeJsPreviewResult {
        macro_value,
        function_body,
        procedure_summaries: analysis.procedures,
        unsupported_operations: analysis.unsupported_operations,
        redactions,
        warnings,
        success: true,
    }
}

fn non_emitting_result(
    analysis: AnalyzeVbaOnlyofficeMigrationResult,
    redactions: Vec<String>,
    warnings: Vec<String>,
) -> TranslateVbaToOnlyofficeJsPreviewResult {
    TranslateVbaToOnlyofficeJsPreviewResult {
        macro_value: String::new(),
        function_body: String::new(),
        procedure_summaries: analysis.procedures,
        unsupported_operations: analysis.unsupported_operations,
        redactions,
        warnings,
        success: false,
    }
}

fn collect_redactions(analysis: &AnalyzeVbaOnlyofficeMigrationResult) -> Vec<String> {
    analysis
        .supported_operations
        .iter()
        .filter(|operation| operation.value.as_deref() == Some("<redacted>"))
        .map(|operation| format!("{}:{}", operation.procedure, operation.line))
        .collect()
}

fn emit_operation(operation: &AnalyzeVbaOperationSummary) -> Option<String> {
    let value = operation.value.as_deref()?;
    match operation.operation.as_str() {
        "SetCellValue" => Some(format!(
            "worksheet.GetActiveCell().SetValue({});",
            js_value(value)?
        )),
        "SetCellFormula" => Some(format!(
            "worksheet.GetActiveCell().SetFormulaArray({});",
            js_value(value)?
        )),
        "SetFontBold" => Some(format!("Api.GetSelection().SetBold({});", js_value(value)?)),
        "SetFontItalic" => Some(format!(
            "Api.GetSelection().SetItalic({});",
            js_value(value)?
        )),
        "SetFontName" => Some(format!(
            "Api.GetSelection().SetFontName({});",
            js_value(value)?
        )),
        "SetFontSize" => Some(format!(
            "Api.GetSelection().SetFontSize({});",
            js_string(value)
        )),
        "SetTextColor" => Some(format!(
            "Api.GetSelection().SetFontColor({});",
            js_color(value)?
        )),
        "SetFillColor" => Some(format!(
            "Api.GetSelection().SetBackgroundColor({});",
            js_color(value)?
        )),
        "SetNumberFormat" => Some(format!(
            "Api.GetSelection().SetNumberFormat({});",
            js_value(value)?
        )),
        "SetWrap" => Some(format!("Api.GetSelection().SetWrap({});", js_value(value)?)),
        "SetAlignHorizontal" => Some(format!(
            "Api.GetSelection().SetAlignHorizontal({});",
            js_number_or_string(value)?
        )),
        "SetAlignVertical" => Some(format!(
            "Api.GetSelection().SetAlignVertical({});",
            js_number_or_string(value)?
        )),
        _ => None,
    }
}

fn js_value(value: &str) -> Option<String> {
    if value == "<redacted>" {
        return None;
    }
    if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        return Some(js_string(&value[1..value.len() - 1]));
    }
    if value.parse::<f64>().is_ok() || matches!(value, "true" | "false") {
        return Some(value.to_string());
    }
    None
}

fn js_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|err| panic!("JS string should serialize: {err}"))
}

fn js_number_or_string(value: &str) -> Option<String> {
    if value.parse::<f64>().is_ok() {
        return Some(value.to_string());
    }
    if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        return Some(js_string(&value[1..value.len() - 1]));
    }
    None
}

fn js_color(value: &str) -> Option<String> {
    let inner = value
        .strip_prefix("RGB(")
        .and_then(|value| value.strip_suffix(')'))?;
    Some(format!("Api.CreateColorFromRGB({inner})"))
}

fn parse_tool_args<T: serde::de::DeserializeOwned>(
    call: &ToolCall,
    tool_name: &str,
) -> Result<T, FunctionCallError> {
    let arguments = call.function_arguments()?;
    serde_json::from_str(arguments).map_err(|err| {
        FunctionCallError::RespondToModel(format!("invalid {tool_name} arguments: {err}"))
    })
}
