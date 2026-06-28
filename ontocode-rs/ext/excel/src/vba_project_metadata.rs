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
use ovba::Reference;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_value;

use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::vba_extract::ParsedVbaProjectResult;
use crate::vba_extract::parse_tool_args;
use crate::vba_extract::parse_vba_project_from_workbook;
use crate::vba_extract::resolve_workbook_path_from_model_arg;

pub(crate) const INSPECT_VBA_PROJECT_METADATA_TOOL_NAME: &str = "inspect_vba_project_metadata";

const INSPECT_VBA_PROJECT_METADATA_DESCRIPTION: &str = "Inspect bounded offline VBA project metadata including module inventory, module-type counts, and provable reference-kind counts.";
const MAX_MODULE_NAMES: usize = 32;
const MAX_DOC_CLS_DESIGNER_MODULE_NAMES: usize = 16;

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectVbaProjectMetadataTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectVbaProjectMetadataArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct VbaProjectReferenceCounts {
    pub control: usize,
    pub original: usize,
    pub registered: usize,
    pub project: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InspectVbaProjectMetadataResult {
    pub mode: String,
    pub path: String,
    pub has_vba_project: bool,
    pub code_page: Option<u16>,
    pub module_count: usize,
    pub procedural_module_count: usize,
    pub doc_cls_designer_module_count: usize,
    pub module_names: Vec<String>,
    pub module_names_truncated: bool,
    pub doc_cls_designer_module_names: Vec<String>,
    pub doc_cls_designer_module_names_truncated: bool,
    pub reference_counts: VbaProjectReferenceCounts,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectVbaProjectMetadataTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_VBA_PROJECT_METADATA_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(InspectVbaProjectMetadataArgs))
                .unwrap_or_else(|err| {
                    panic!("inspect_vba_project_metadata args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(InspectVbaProjectMetadataResult))
                .unwrap_or_else(|err| {
                    panic!("inspect_vba_project_metadata result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_VBA_PROJECT_METADATA_TOOL_NAME.to_string(),
                    description: INSPECT_VBA_PROJECT_METADATA_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_vba_project_metadata args schema should parse: {err}")
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
        let args = parse_tool_args::<InspectVbaProjectMetadataArgs>(
            &call,
            "excel.inspect_vba_project_metadata",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_vba_project_metadata workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = resolve_workbook_path_from_model_arg(
            "excel.inspect_vba_project_metadata",
            &args.path,
            &cwd,
        )?;
        let result =
            inspect_vba_project_metadata_from_workbook(&workbook_path, Path::new(args.path.trim()));
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize VBA project metadata result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectVbaProjectMetadataTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn inspect_vba_project_metadata_from_workbook(
    path: &Path,
    display_path: &Path,
) -> InspectVbaProjectMetadataResult {
    let ParsedVbaProjectResult {
        has_vba_project,
        code_page,
        project,
        mut warnings,
    } = parse_vba_project_from_workbook(path, display_path);
    let Some(project) = project else {
        return InspectVbaProjectMetadataResult {
            mode: "read_only_inspection".to_string(),
            path: display_path.display().to_string(),
            has_vba_project,
            code_page,
            module_count: 0,
            procedural_module_count: 0,
            doc_cls_designer_module_count: 0,
            module_names: Vec::new(),
            module_names_truncated: false,
            doc_cls_designer_module_names: Vec::new(),
            doc_cls_designer_module_names_truncated: false,
            reference_counts: VbaProjectReferenceCounts {
                control: 0,
                original: 0,
                registered: 0,
                project: 0,
            },
            warnings,
        };
    };

    let mut procedural_module_count = 0usize;
    let mut doc_cls_designer_module_count = 0usize;
    let mut module_names = Vec::new();
    let mut module_names_truncated = false;
    let mut doc_cls_designer_module_names = Vec::new();
    let mut doc_cls_designer_module_names_truncated = false;
    for module in &project.modules {
        if module_names.len() < MAX_MODULE_NAMES {
            module_names.push(module.name.clone());
        } else {
            module_names_truncated = true;
        }
        match module.module_type {
            ovba::ModuleType::Procedural => procedural_module_count += 1,
            ovba::ModuleType::DocClsDesigner => {
                doc_cls_designer_module_count += 1;
                if doc_cls_designer_module_names.len() < MAX_DOC_CLS_DESIGNER_MODULE_NAMES {
                    doc_cls_designer_module_names.push(module.name.clone());
                } else {
                    doc_cls_designer_module_names_truncated = true;
                }
            }
        }
    }

    let mut reference_counts = VbaProjectReferenceCounts {
        control: 0,
        original: 0,
        registered: 0,
        project: 0,
    };
    for reference in &project.references {
        match reference {
            Reference::Control(_) => reference_counts.control += 1,
            Reference::Original(_) => reference_counts.original += 1,
            Reference::Registered(_) => reference_counts.registered += 1,
            Reference::Project(_) => reference_counts.project += 1,
        }
    }

    if !project.references.is_empty() {
        warnings.push(
            "VBA reference detail strings are not exposed by the current parser; only reference-kind counts are reported"
                .to_string(),
        );
    }
    if doc_cls_designer_module_count > 0 {
        warnings.push(
            "doc_cls_designer modules may represent document, class, or designer/forms metadata; current parser does not distinguish them further"
                .to_string(),
        );
    }
    if module_names_truncated {
        warnings.push(format!(
            "module_names is a bounded sample capped at {MAX_MODULE_NAMES} entries"
        ));
    }
    if doc_cls_designer_module_names_truncated {
        warnings.push(format!(
            "doc_cls_designer_module_names is a bounded sample capped at {MAX_DOC_CLS_DESIGNER_MODULE_NAMES} entries"
        ));
    }

    InspectVbaProjectMetadataResult {
        mode: "read_only_inspection".to_string(),
        path: display_path.display().to_string(),
        has_vba_project,
        code_page,
        module_count: project.modules.len(),
        procedural_module_count,
        doc_cls_designer_module_count,
        module_names,
        module_names_truncated,
        doc_cls_designer_module_names,
        doc_cls_designer_module_names_truncated,
        reference_counts,
        warnings,
    }
}
