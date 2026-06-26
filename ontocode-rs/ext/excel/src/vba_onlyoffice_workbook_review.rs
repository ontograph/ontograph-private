use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
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
use crate::vba_extract::ExtractVbaModulesResult;
use crate::vba_extract::ExtractedVbaModule;
use crate::vba_extract::VbaModuleKind;
use crate::vba_extract::extract_vba_modules_from_workbook;
use crate::vba_onlyoffice_analyze::AnalyzeVbaOnlyofficeMigrationResult;
use crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration;
use crate::vba_onlyoffice_translate::translate_analyzed_vba_to_onlyoffice_js_preview;

pub(crate) const REVIEW_VBA_ONLYOFFICE_WORKBOOK_TOOL_NAME: &str = "review_vba_onlyoffice_workbook";

const REVIEW_VBA_ONLYOFFICE_WORKBOOK_DESCRIPTION: &str = "Review VBA modules from a workbook for ONLYOFFICE migration and emit previews only for analyzer-approved modules.";
const MAX_REQUESTED_MODULES: usize = 16;
const MAX_WARNINGS: usize = 32;

#[derive(Clone, Default)]
pub(crate) struct ExcelReviewVbaOnlyofficeWorkbookTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReviewVbaOnlyofficeWorkbookArgs {
    pub path: String,
    #[serde(default)]
    pub module_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ReviewedVbaOnlyofficeWorkbookModule {
    pub name: String,
    pub stream_name: String,
    pub module_kind: VbaModuleKind,
    pub source_truncated: bool,
    pub analysis: AnalyzeVbaOnlyofficeMigrationResult,
    pub warnings: Vec<String>,
    pub macro_value: String,
    pub function_body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ReviewVbaOnlyofficeWorkbookResult {
    pub mode: String,
    pub path: String,
    pub has_vba_project: bool,
    pub code_page: Option<u16>,
    pub extracted_module_count: usize,
    pub reviewed_module_count: usize,
    pub requested_module_names: Vec<String>,
    pub modules: Vec<ReviewedVbaOnlyofficeWorkbookModule>,
    pub warnings: Vec<String>,
    pub success: bool,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelReviewVbaOnlyofficeWorkbookTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, REVIEW_VBA_ONLYOFFICE_WORKBOOK_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(ReviewVbaOnlyofficeWorkbookArgs))
                .unwrap_or_else(|err| {
                    panic!("review_vba_onlyoffice_workbook args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(ReviewVbaOnlyofficeWorkbookResult))
                .unwrap_or_else(|err| {
                    panic!("review_vba_onlyoffice_workbook result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: REVIEW_VBA_ONLYOFFICE_WORKBOOK_TOOL_NAME.to_string(),
                    description: REVIEW_VBA_ONLYOFFICE_WORKBOOK_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("review_vba_onlyoffice_workbook args schema should parse: {err}")
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
        let args = parse_tool_args::<ReviewVbaOnlyofficeWorkbookArgs>(
            &call,
            "excel.review_vba_onlyoffice_workbook",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.review_vba_onlyoffice_workbook workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = resolve_workbook_path_from_model_arg(&args.path, &cwd)?;
        let result = review_vba_onlyoffice_workbook(
            &workbook_path,
            Path::new(args.path.trim()),
            args.module_names.as_deref(),
        );
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize ONLYOFFICE workbook review: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelReviewVbaOnlyofficeWorkbookTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn review_vba_onlyoffice_workbook(
    path: &Path,
    display_path: &Path,
    requested_module_names: Option<&[String]>,
) -> ReviewVbaOnlyofficeWorkbookResult {
    let extraction = extract_vba_modules_from_workbook(path, display_path);
    build_review_result(extraction, requested_module_names)
}

fn build_review_result(
    extraction: ExtractVbaModulesResult,
    requested_module_names: Option<&[String]>,
) -> ReviewVbaOnlyofficeWorkbookResult {
    let requested_module_names = sanitize_requested_module_names(requested_module_names);
    let requested_module_refs = requested_module_names
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let mut warnings = extraction.warnings;
    let mut modules = Vec::new();

    for module in extraction.modules {
        if !should_review_module(&module, &requested_module_refs) {
            continue;
        }

        let analysis = analyze_vba_onlyoffice_migration(&module.source, Some(&module.name));
        let preview = if analysis.success {
            Some(translate_analyzed_vba_to_onlyoffice_js_preview(
                analysis.clone(),
            ))
        } else {
            None
        };
        let (macro_value, function_body, preview_warnings) = preview
            .map(|preview| (preview.macro_value, preview.function_body, preview.warnings))
            .unwrap_or_default();
        let mut module_warnings = analysis.warnings.clone();
        module_warnings.extend(preview_warnings);
        modules.push(ReviewedVbaOnlyofficeWorkbookModule {
            name: module.name,
            stream_name: module.stream_name,
            module_kind: module.module_kind,
            source_truncated: module.source_truncated,
            analysis,
            warnings: module_warnings,
            macro_value,
            function_body,
        });
    }

    for requested_name in &requested_module_names {
        if !modules.iter().any(|module| {
            module.name.eq_ignore_ascii_case(requested_name)
                || module.stream_name.eq_ignore_ascii_case(requested_name)
        }) {
            warnings.push(format!(
                "requested module {requested_name} was not found in the workbook review set"
            ));
        }
    }

    if extraction.has_vba_project && modules.is_empty() && requested_module_names.is_empty() {
        warnings.push("no VBA modules were available for ONLYOFFICE review".to_string());
    }

    let reviewed_module_count = modules.len();
    warnings.truncate(MAX_WARNINGS);
    ReviewVbaOnlyofficeWorkbookResult {
        mode: "read_only_workbook_review".to_string(),
        path: extraction.path,
        has_vba_project: extraction.has_vba_project,
        code_page: extraction.code_page,
        extracted_module_count: extraction.module_count,
        reviewed_module_count,
        requested_module_names,
        modules,
        warnings,
        success: extraction.has_vba_project && reviewed_module_count > 0,
    }
}

fn sanitize_requested_module_names(requested_module_names: Option<&[String]>) -> Vec<String> {
    requested_module_names
        .map(|names| {
            names
                .iter()
                .map(|name| name.trim())
                .filter(|name| !name.is_empty())
                .take(MAX_REQUESTED_MODULES)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn should_review_module(module: &ExtractedVbaModule, requested_module_names: &[&str]) -> bool {
    if requested_module_names.is_empty() {
        return true;
    }
    requested_module_names.iter().any(|requested_name| {
        module.name.eq_ignore_ascii_case(requested_name)
            || module.stream_name.eq_ignore_ascii_case(requested_name)
    })
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

fn resolve_workbook_path_from_model_arg(
    path: &str,
    cwd: &Path,
) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "excel.review_vba_onlyoffice_workbook path must not be empty".to_string(),
        ));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(
            "excel.review_vba_onlyoffice_workbook path must be a local workbook path".to_string(),
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
            "excel.review_vba_onlyoffice_workbook path must be relative and stay within the current working directory"
                .to_string(),
        ));
    }

    let Some(extension) = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
    else {
        return Err(FunctionCallError::RespondToModel(
            "excel.review_vba_onlyoffice_workbook path must end in .xlsx, .xlsm, or .xlsb"
                .to_string(),
        ));
    };
    if !matches!(extension.as_str(), "xlsx" | "xlsm" | "xlsb") {
        return Err(FunctionCallError::RespondToModel(
            "excel.review_vba_onlyoffice_workbook path must end in .xlsx, .xlsm, or .xlsb"
                .to_string(),
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
                "excel.review_vba_onlyoffice_workbook path must not traverse symlinks".to_string(),
            ));
        }
    }

    Ok(resolved_path)
}
