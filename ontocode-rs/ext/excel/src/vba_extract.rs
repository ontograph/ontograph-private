use std::fs::File;
use std::io::Read;
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
use ovba::ModuleType;
use ovba::Project;
use ovba::open_project;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_value;
use zip::ZipArchive;

use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;

pub(crate) const EXTRACT_VBA_MODULES_TOOL_NAME: &str = "extract_vba_modules";

const EXTRACT_VBA_MODULES_DESCRIPTION: &str =
    "Read VBA modules from a workbook and return bounded read-only module text and metadata.";
const MAX_VBA_PROJECT_BIN_BYTES: usize = 16 * 1024 * 1024;
const MAX_EXTRACTED_MODULES: usize = 16;
const MAX_MODULE_SOURCE_CHARS: usize = 4096;
const MAX_MODULE_DOC_CHARS: usize = 512;
const MAX_WARNINGS: usize = 16;

#[derive(Clone, Default)]
pub(crate) struct ExcelExtractVbaModulesTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ExtractVbaModulesArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum VbaModuleKind {
    Procedural,
    DocClsDesigner,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ExtractedVbaModule {
    pub name: String,
    pub stream_name: String,
    pub module_kind: VbaModuleKind,
    pub text_offset: usize,
    pub read_only: bool,
    pub private: bool,
    pub doc_string: String,
    pub source: String,
    pub source_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ExtractVbaModulesResult {
    pub mode: String,
    pub path: String,
    pub has_vba_project: bool,
    pub code_page: Option<u16>,
    pub module_count: usize,
    pub modules: Vec<ExtractedVbaModule>,
    pub warnings: Vec<String>,
}

pub(crate) struct ParsedVbaProjectResult {
    pub has_vba_project: bool,
    pub code_page: Option<u16>,
    pub project: Option<Project>,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelExtractVbaModulesTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, EXTRACT_VBA_MODULES_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema = serde_json::to_value(schemars::schema_for!(ExtractVbaModulesArgs))
            .unwrap_or_else(|err| {
                panic!("extract_vba_modules args schema should serialize: {err}")
            });
        let output_schema = serde_json::to_value(schemars::schema_for!(ExtractVbaModulesResult))
            .unwrap_or_else(|err| {
                panic!("extract_vba_modules result schema should serialize: {err}")
            });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: EXTRACT_VBA_MODULES_TOOL_NAME.to_string(),
                    description: EXTRACT_VBA_MODULES_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("extract_vba_modules args schema should parse: {err}")
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
        let args = parse_tool_args::<ExtractVbaModulesArgs>(&call, "excel.extract_vba_modules")?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.extract_vba_modules workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path =
            resolve_workbook_path_from_model_arg("excel.extract_vba_modules", &args.path, &cwd)?;
        let result = extract_vba_modules_from_workbook(&workbook_path, Path::new(args.path.trim()));
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize VBA extraction result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelExtractVbaModulesTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn extract_vba_modules_from_workbook(
    path: &Path,
    display_path: &Path,
) -> ExtractVbaModulesResult {
    let mut modules = Vec::new();
    let ParsedVbaProjectResult {
        has_vba_project,
        code_page,
        project,
        mut warnings,
    } = parse_vba_project_from_workbook(path, display_path);
    let Some(project) = project else {
        return ExtractVbaModulesResult {
            mode: "read_only_extraction".to_string(),
            path: display_path.display().to_string(),
            has_vba_project,
            code_page,
            module_count: 0,
            modules,
            warnings,
        };
    };

    let module_total = project.modules.len();
    if module_total > MAX_EXTRACTED_MODULES {
        warnings.push(format!(
            "VBA module list truncated to {MAX_EXTRACTED_MODULES} of {module_total} entries"
        ));
    }

    for module in project.modules.iter().take(MAX_EXTRACTED_MODULES) {
        let source_result = project.module_source(&module.name);
        let source = match source_result {
            Ok(source) => source,
            Err(err) => {
                warnings.push(format!("failed to read VBA module {}: {err}", module.name));
                continue;
            }
        };
        let (source, source_truncated) = bounded_text(&source, MAX_MODULE_SOURCE_CHARS);
        let (doc_string, doc_truncated) = bounded_text(&module.doc_string, MAX_MODULE_DOC_CHARS);
        if source_truncated {
            warnings.push(format!(
                "VBA module {} source truncated to {MAX_MODULE_SOURCE_CHARS} characters",
                module.name
            ));
        }
        if doc_truncated {
            warnings.push(format!(
                "VBA module {} doc_string truncated to {MAX_MODULE_DOC_CHARS} characters",
                module.name
            ));
        }
        modules.push(ExtractedVbaModule {
            name: module.name.clone(),
            stream_name: module.stream_name.clone(),
            module_kind: module_kind(&module.module_type),
            text_offset: module.text_offset,
            read_only: module.read_only,
            private: module.private,
            doc_string,
            source,
            source_truncated,
        });
    }

    if !has_vba_project {
        warnings.push("no VBA project found in workbook".to_string());
    }

    warnings.truncate(MAX_WARNINGS);

    ExtractVbaModulesResult {
        mode: "read_only_extraction".to_string(),
        path: display_path.display().to_string(),
        has_vba_project,
        code_page,
        module_count: modules.len(),
        modules,
        warnings,
    }
}

pub(crate) fn parse_vba_project_from_workbook(
    path: &Path,
    display_path: &Path,
) -> ParsedVbaProjectResult {
    let mut warnings = Vec::new();
    let mut has_vba_project = false;
    let mut code_page = None;

    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            warnings.push(format!(
                "failed to open workbook {}: {err}",
                display_path.display()
            ));
            return ParsedVbaProjectResult {
                has_vba_project,
                code_page,
                project: None,
                warnings,
            };
        }
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(err) => {
            warnings.push(format!(
                "failed to read workbook archive {}: {err}",
                display_path.display()
            ));
            return ParsedVbaProjectResult {
                has_vba_project,
                code_page,
                project: None,
                warnings,
            };
        }
    };

    let Some(vba_project_bin) = read_vba_project_bin(&mut archive, display_path, &mut warnings)
    else {
        warnings.truncate(MAX_WARNINGS);
        return ParsedVbaProjectResult {
            has_vba_project,
            code_page,
            project: None,
            warnings,
        };
    };

    has_vba_project = true;
    if vba_project_bin.len() > MAX_VBA_PROJECT_BIN_BYTES {
        warnings.push(format!(
            "vbaProject.bin exceeds {MAX_VBA_PROJECT_BIN_BYTES} bytes and was not parsed"
        ));
        warnings.truncate(MAX_WARNINGS);
        return ParsedVbaProjectResult {
            has_vba_project,
            code_page,
            project: None,
            warnings,
        };
    }

    let project = match open_project(vba_project_bin) {
        Ok(project) => project,
        Err(err) => {
            warnings.push(format!("failed to parse vbaProject.bin: {err}"));
            warnings.truncate(MAX_WARNINGS);
            return ParsedVbaProjectResult {
                has_vba_project,
                code_page,
                project: None,
                warnings,
            };
        }
    };

    code_page = Some(project.information.code_page);
    warnings.truncate(MAX_WARNINGS);
    ParsedVbaProjectResult {
        has_vba_project,
        code_page,
        project: Some(project),
        warnings,
    }
}

pub(crate) fn parse_tool_args<T: serde::de::DeserializeOwned>(
    call: &ToolCall,
    tool_name: &str,
) -> Result<T, FunctionCallError> {
    let arguments = call.function_arguments()?;
    serde_json::from_str(arguments).map_err(|err| {
        FunctionCallError::RespondToModel(format!("invalid {tool_name} arguments: {err}"))
    })
}

fn read_vba_project_bin(
    archive: &mut ZipArchive<File>,
    path: &Path,
    warnings: &mut Vec<String>,
) -> Option<Vec<u8>> {
    for entry_name in ["xl/vbaProject.bin", "xl/macros/vbaProject.bin"] {
        if let Ok(mut entry) = archive.by_name(entry_name) {
            let entry_size = match usize::try_from(entry.size()) {
                Ok(size) => size,
                Err(_) => {
                    warnings.push(format!(
                        "workbook entry {entry_name} is too large to inspect"
                    ));
                    return None;
                }
            };
            if entry_size > MAX_VBA_PROJECT_BIN_BYTES {
                warnings.push(format!(
                    "workbook entry {entry_name} exceeds {MAX_VBA_PROJECT_BIN_BYTES} bytes"
                ));
                return None;
            }
            let mut bytes = Vec::with_capacity(entry_size);
            if let Err(err) = entry.read_to_end(&mut bytes) {
                warnings.push(format!(
                    "failed to read workbook entry {entry_name} in {}: {err}",
                    path.display()
                ));
                return None;
            }
            return Some(bytes);
        }
    }

    warnings.push("no VBA project binary was found in the workbook".to_string());
    None
}

pub(crate) fn resolve_workbook_path_from_model_arg(
    tool_name: &str,
    path: &str,
    cwd: &Path,
) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(format!(
            "{tool_name} path must not be empty"
        )));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(format!(
            "{tool_name} path must be a local workbook path"
        )));
    }

    let path = Path::new(trimmed);
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(FunctionCallError::RespondToModel(format!(
            "{tool_name} path must be relative and stay within the current working directory"
        )));
    }

    let Some(extension) = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
    else {
        return Err(FunctionCallError::RespondToModel(format!(
            "{tool_name} path must end in .xlsx, .xlsm, or .xlsb"
        )));
    };
    if !matches!(extension.as_str(), "xlsx" | "xlsm" | "xlsb") {
        return Err(FunctionCallError::RespondToModel(format!(
            "{tool_name} path must end in .xlsx, .xlsm, or .xlsb"
        )));
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
            return Err(FunctionCallError::RespondToModel(format!(
                "{tool_name} path must not traverse symlinks"
            )));
        }
    }

    Ok(resolved_path)
}

fn module_kind(module_type: &ModuleType) -> VbaModuleKind {
    match module_type {
        ModuleType::Procedural => VbaModuleKind::Procedural,
        ModuleType::DocClsDesigner => VbaModuleKind::DocClsDesigner,
    }
}

fn bounded_text(value: &str, max_chars: usize) -> (String, bool) {
    let mut truncated = false;
    let mut text = String::new();
    for (count, ch) in value.chars().enumerate() {
        if count >= max_chars {
            truncated = true;
            break;
        }
        text.push(ch);
    }
    (text, truncated)
}
