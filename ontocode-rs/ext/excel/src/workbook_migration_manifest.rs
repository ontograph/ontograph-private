use std::collections::BTreeSet;
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

use crate::backend::ExcelInspectionError;
use crate::backend::inspect_workbook_with_display_path;
use crate::formula_sql_readiness::FormulaSqlBlockedReasonCount;
use crate::formula_sql_readiness::FormulaSqlReadinessCounts;
use crate::formula_sql_readiness::FormulaSqlReadinessFamily;
use crate::formula_sql_readiness::inspect_formula_sql_readiness_from_workbook;
use crate::pivot_report_metadata::InspectPivotReportMetadataResult;
use crate::pivot_report_metadata::inspect_pivot_report_metadata_from_workbook;
use crate::powerquery_extract::extract_powerquery_queries_from_workbook;
use crate::slider_query::scan_sheet_formulas_dependency_with_display_path;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::InspectWorkbookResult;
use crate::tool::SheetKind;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;
use crate::tool::workbook_path_from_model_arg;
use crate::vba_extract::extract_vba_modules_from_workbook;

pub(crate) const GENERATE_WORKBOOK_MIGRATION_MANIFEST_TOOL_NAME: &str =
    "generate_workbook_migration_manifest";

const GENERATE_WORKBOOK_MIGRATION_MANIFEST_DESCRIPTION: &str = "Generate a deterministic offline workbook migration manifest by composing existing Excel inspection, formula, Power Query, pivot, and VBA evidence.";
const DEFAULT_MAX_FORMULAS_PER_SHEET: usize = 128;
const MAX_FORMULAS_PER_SHEET: usize = 512;
const MAX_QUERY_NAMES: usize = 16;
const MAX_PIVOT_TABLE_NAMES: usize = 16;
const MAX_VBA_MODULE_NAMES: usize = 16;

#[derive(Clone, Default)]
pub(crate) struct ExcelGenerateWorkbookMigrationManifestTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct GenerateWorkbookMigrationManifestArgs {
    pub path: String,
    pub output_bundle_path: String,
    #[serde(default)]
    pub max_formulas_per_sheet: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenerateWorkbookMigrationManifestResult {
    pub path: String,
    pub bundle_path: String,
    pub manifest_path: String,
    pub manifest: WorkbookMigrationManifest,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkbookMigrationManifest {
    pub bundle_name: String,
    pub workbook: InspectWorkbookResult,
    pub formula_sheets: Vec<WorkbookMigrationFormulaSheetSummary>,
    pub formula_sql_lineage: Vec<WorkbookMigrationFormulaSqlLineageEntry>,
    pub power_query: WorkbookMigrationPowerQuerySummary,
    pub pivot: WorkbookMigrationPivotSummary,
    pub vba: WorkbookMigrationVbaSummary,
    pub unsupported_sections: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkbookMigrationFormulaSheetSummary {
    pub sheet: SheetPreview,
    pub formula_count: Option<usize>,
    pub readiness_counts: Option<FormulaSqlReadinessCounts>,
    pub blocked_reason_counts: Vec<FormulaSqlBlockedReasonCount>,
    pub dependency_node_count: Option<usize>,
    pub dependency_cycle_count: Option<usize>,
    pub dependency_unsupported_formula_count: Option<usize>,
    pub readiness_truncated: Option<bool>,
    pub dependency_truncated: Option<bool>,
    pub readiness_unavailable_reason: Option<String>,
    pub dependency_unavailable_reason: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WorkbookMigrationFormulaSqlLineageState {
    Ready,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkbookMigrationFormulaSqlLineageEntry {
    pub source_id: String,
    pub sheet: String,
    pub reference: String,
    pub family: FormulaSqlReadinessFamily,
    pub readiness_state: WorkbookMigrationFormulaSqlLineageState,
    pub sql_expression: Option<String>,
    pub blocker_reasons: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkbookMigrationPowerQuerySummary {
    pub has_power_query: bool,
    pub query_count: usize,
    pub lint_finding_count: usize,
    pub connection_count: usize,
    pub query_names: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkbookMigrationPivotSummary {
    pub pivot_table_count: usize,
    pub pivot_cache_count: usize,
    pub pivot_table_names: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkbookMigrationVbaSummary {
    pub has_vba_project: bool,
    pub module_count: usize,
    pub module_names: Vec<String>,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelGenerateWorkbookMigrationManifestTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(
            EXCEL_NAMESPACE,
            GENERATE_WORKBOOK_MIGRATION_MANIFEST_TOOL_NAME,
        )
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(GenerateWorkbookMigrationManifestArgs))
                .unwrap_or_else(|err| {
                    panic!(
                        "generate_workbook_migration_manifest args schema should serialize: {err}"
                    )
                });
        let output_schema = serde_json::to_value(schemars::schema_for!(
            GenerateWorkbookMigrationManifestResult
        ))
        .unwrap_or_else(|err| {
            panic!("generate_workbook_migration_manifest result schema should serialize: {err}")
        });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: GENERATE_WORKBOOK_MIGRATION_MANIFEST_TOOL_NAME.to_string(),
                    description: GENERATE_WORKBOOK_MIGRATION_MANIFEST_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!(
                            "generate_workbook_migration_manifest args schema should parse: {err}"
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
        let args = parse_tool_args::<GenerateWorkbookMigrationManifestArgs>(
            &call,
            "excel.generate_workbook_migration_manifest",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.generate_workbook_migration_manifest workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let output_bundle_path = output_bundle_path_from_model_arg(&args.output_bundle_path, &cwd)?;
        let result = generate_workbook_migration_manifest_with_display_path(
            &workbook_path,
            Path::new(args.path.trim()),
            &output_bundle_path,
            Path::new(args.output_bundle_path.trim()),
            args.max_formulas_per_sheet,
        )
        .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize workbook migration manifest result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelGenerateWorkbookMigrationManifestTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn generate_workbook_migration_manifest_with_display_path(
    path: &Path,
    display_path: &Path,
    output_bundle_path: &Path,
    output_bundle_display_path: &Path,
    max_formulas_per_sheet: Option<usize>,
) -> Result<GenerateWorkbookMigrationManifestResult, ExcelInspectionError> {
    let max_formulas_per_sheet = normalize_max_formulas(max_formulas_per_sheet)?;
    let workbook = inspect_workbook_with_display_path(path, display_path)?;
    let mut warnings = workbook.warnings.clone();
    let mut unsupported_sections = BTreeSet::from([
        "full_workbook_translation_or_apply_not_supported".to_string(),
        "offline_read_only_manifest_no_live_excel_or_write_surfaces".to_string(),
    ]);

    let (formula_sheets, formula_sql_lineage) = build_formula_sheet_outputs(
        path,
        display_path,
        &workbook,
        max_formulas_per_sheet,
        &mut warnings,
        &mut unsupported_sections,
    );
    let power_query =
        build_power_query_summary(path, display_path, &mut warnings, &mut unsupported_sections);
    let pivot = build_pivot_summary(path, display_path, &mut warnings, &mut unsupported_sections);
    let vba = build_vba_summary(path, display_path, &mut warnings, &mut unsupported_sections);

    let manifest = WorkbookMigrationManifest {
        bundle_name: slugify(
            display_path
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("workbook"),
        ),
        workbook,
        formula_sheets,
        formula_sql_lineage,
        power_query,
        pivot,
        vba,
        unsupported_sections: unsupported_sections.into_iter().collect(),
        warnings: warnings.clone(),
    };

    std::fs::create_dir_all(output_bundle_path).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to create workbook migration manifest directory {}: {err}",
            output_bundle_path.display()
        ))
    })?;
    let manifest_path = output_bundle_path.join("manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to serialize workbook migration manifest: {err}"
        ))
    })?;
    std::fs::write(&manifest_path, manifest_json).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to write workbook migration manifest {}: {err}",
            manifest_path.display()
        ))
    })?;

    Ok(GenerateWorkbookMigrationManifestResult {
        path: display_path.display().to_string(),
        bundle_path: output_bundle_display_path.display().to_string(),
        manifest_path: output_bundle_display_path
            .join("manifest.json")
            .display()
            .to_string(),
        manifest,
        warnings,
    })
}

fn build_formula_sheet_outputs(
    path: &Path,
    display_path: &Path,
    workbook: &InspectWorkbookResult,
    max_formulas_per_sheet: usize,
    warnings: &mut Vec<String>,
    unsupported_sections: &mut BTreeSet<String>,
) -> (
    Vec<WorkbookMigrationFormulaSheetSummary>,
    Vec<WorkbookMigrationFormulaSqlLineageEntry>,
) {
    let mut sheets = Vec::new();
    let mut lineage = Vec::new();
    for (index, sheet) in workbook.sheets.iter().enumerate() {
        if sheet.kind != SheetKind::Worksheet {
            continue;
        }
        let Some(sheet_preview) = sheet_preview_from_summary(sheet) else {
            warnings.push(format!(
                "sheet {} is missing name or part path and was skipped in formula manifest generation",
                index + 1
            ));
            unsupported_sections.insert("formula_sheet_without_name_or_part_path".to_string());
            continue;
        };
        let selector = SheetSelector::Name {
            name: sheet_preview.name.clone(),
        };
        let readiness = inspect_formula_sql_readiness_from_workbook(
            path,
            display_path,
            &selector,
            Some(max_formulas_per_sheet),
        );
        let dependency = scan_sheet_formulas_dependency_with_display_path(
            path,
            display_path,
            &selector,
            Some(max_formulas_per_sheet),
        );

        let mut sheet_warnings = Vec::new();
        let (
            formula_count,
            readiness_counts,
            blocked_reason_counts,
            readiness_truncated,
            readiness_unavailable_reason,
        ) = match readiness {
            Ok(result) => {
                lineage.extend(build_formula_sql_lineage_entries(
                    &sheet_preview.name,
                    &result,
                ));
                sheet_warnings.extend(result.warnings);
                (
                    Some(result.formula_count),
                    Some(result.readiness_counts),
                    result.blocked_reason_counts,
                    Some(result.truncated),
                    None,
                )
            }
            Err(err) => {
                let reason = err.to_string();
                sheet_warnings.push(reason.clone());
                unsupported_sections.insert("formula_readiness_unavailable".to_string());
                (None, None, Vec::new(), None, Some(reason))
            }
        };

        let (
            dependency_node_count,
            dependency_cycle_count,
            dependency_unsupported_formula_count,
            dependency_truncated,
            dependency_unavailable_reason,
        ) = match dependency {
            Ok(result) => {
                let unsupported_count = result
                    .nodes
                    .iter()
                    .filter(|node| !node.is_supported)
                    .count();
                sheet_warnings.extend(result.warnings);
                (
                    Some(result.nodes.len()),
                    Some(result.cycles_detected.len()),
                    Some(unsupported_count),
                    Some(result.truncated),
                    None,
                )
            }
            Err(err) => {
                let reason = err.to_string();
                sheet_warnings.push(reason.clone());
                unsupported_sections.insert("formula_dependency_scan_unavailable".to_string());
                (None, None, None, None, Some(reason))
            }
        };

        sheets.push(WorkbookMigrationFormulaSheetSummary {
            sheet: sheet_preview,
            formula_count,
            readiness_counts,
            blocked_reason_counts,
            dependency_node_count,
            dependency_cycle_count,
            dependency_unsupported_formula_count,
            readiness_truncated,
            dependency_truncated,
            readiness_unavailable_reason,
            dependency_unavailable_reason,
            warnings: sheet_warnings,
        });
    }
    (sheets, lineage)
}

fn build_formula_sql_lineage_entries(
    sheet_name: &str,
    result: &crate::formula_sql_readiness::InspectFormulaSqlReadinessResult,
) -> Vec<WorkbookMigrationFormulaSqlLineageEntry> {
    let mut lineage = Vec::new();
    for formula in &result.ready_formulas {
        lineage.push(WorkbookMigrationFormulaSqlLineageEntry {
            source_id: formula_source_id(sheet_name, &formula.reference),
            sheet: sheet_name.to_string(),
            reference: formula.reference.clone(),
            family: formula.family,
            readiness_state: WorkbookMigrationFormulaSqlLineageState::Ready,
            sql_expression: Some(formula.sql_expression.clone()),
            blocker_reasons: Vec::new(),
            warnings: formula.warnings.clone(),
        });
    }
    for formula in &result.blocked_formulas {
        lineage.push(WorkbookMigrationFormulaSqlLineageEntry {
            source_id: formula_source_id(sheet_name, &formula.reference),
            sheet: sheet_name.to_string(),
            reference: formula.reference.clone(),
            family: formula.family_hint,
            readiness_state: WorkbookMigrationFormulaSqlLineageState::Blocked,
            sql_expression: None,
            blocker_reasons: formula.blocker_reasons.clone(),
            warnings: formula.warnings.clone(),
        });
    }
    lineage
}

fn formula_source_id(sheet_name: &str, reference: &str) -> String {
    format!("{sheet_name}!{reference}")
}

fn build_power_query_summary(
    path: &Path,
    display_path: &Path,
    warnings: &mut Vec<String>,
    unsupported_sections: &mut BTreeSet<String>,
) -> WorkbookMigrationPowerQuerySummary {
    let extraction = extract_powerquery_queries_from_workbook(path, display_path);
    warnings.extend(extraction.warnings.iter().cloned());
    if !extraction.has_power_query {
        unsupported_sections.insert("power_query_not_present".to_string());
    }
    WorkbookMigrationPowerQuerySummary {
        has_power_query: extraction.has_power_query,
        query_count: extraction.query_count,
        lint_finding_count: extraction.lint_finding_count,
        connection_count: extraction.connections.len(),
        query_names: extraction
            .queries
            .iter()
            .take(MAX_QUERY_NAMES)
            .map(|query| query.name.clone())
            .collect(),
        warnings: extraction.warnings,
    }
}

fn build_pivot_summary(
    path: &Path,
    display_path: &Path,
    warnings: &mut Vec<String>,
    unsupported_sections: &mut BTreeSet<String>,
) -> WorkbookMigrationPivotSummary {
    match inspect_pivot_report_metadata_from_workbook(path, display_path) {
        Ok(pivot) => {
            warnings.extend(pivot.warnings.iter().cloned());
            if pivot.pivot_table_count == 0 {
                unsupported_sections.insert("pivot_reports_not_present".to_string());
            }
            WorkbookMigrationPivotSummary {
                pivot_table_count: pivot.pivot_table_count,
                pivot_cache_count: pivot.pivot_cache_count,
                pivot_table_names: pivot_table_names(&pivot),
                warnings: pivot.warnings,
            }
        }
        Err(err) => {
            let warning = err.to_string();
            warnings.push(warning.clone());
            unsupported_sections.insert("pivot_metadata_unavailable".to_string());
            WorkbookMigrationPivotSummary {
                pivot_table_count: 0,
                pivot_cache_count: 0,
                pivot_table_names: Vec::new(),
                warnings: vec![warning],
            }
        }
    }
}

fn build_vba_summary(
    path: &Path,
    display_path: &Path,
    warnings: &mut Vec<String>,
    unsupported_sections: &mut BTreeSet<String>,
) -> WorkbookMigrationVbaSummary {
    let extraction = extract_vba_modules_from_workbook(path, display_path);
    warnings.extend(extraction.warnings.iter().cloned());
    if !extraction.has_vba_project {
        unsupported_sections.insert("vba_project_not_present".to_string());
    }
    WorkbookMigrationVbaSummary {
        has_vba_project: extraction.has_vba_project,
        module_count: extraction.module_count,
        module_names: extraction
            .modules
            .iter()
            .take(MAX_VBA_MODULE_NAMES)
            .map(|module| module.name.clone())
            .collect(),
        warnings: extraction.warnings,
    }
}

fn pivot_table_names(result: &InspectPivotReportMetadataResult) -> Vec<String> {
    result
        .pivot_tables
        .iter()
        .take(MAX_PIVOT_TABLE_NAMES)
        .filter_map(|table| table.name.clone())
        .collect()
}

fn sheet_preview_from_summary(sheet: &crate::tool::SheetSummary) -> Option<SheetPreview> {
    Some(SheetPreview {
        name: sheet.name.clone()?,
        sheet_id: sheet.sheet_id,
        part_path: sheet.part_path.clone()?,
    })
}

fn normalize_max_formulas(value: Option<usize>) -> Result<usize, ExcelInspectionError> {
    let value = value.unwrap_or(DEFAULT_MAX_FORMULAS_PER_SHEET);
    if !(1..=MAX_FORMULAS_PER_SHEET).contains(&value) {
        return Err(ExcelInspectionError::Message(format!(
            "excel.generate_workbook_migration_manifest max_formulas_per_sheet must be between 1 and {MAX_FORMULAS_PER_SHEET}"
        )));
    }
    Ok(value)
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_separator = false;
    for ch in value.chars() {
        let mapped = if ch.is_ascii_alphanumeric() {
            last_was_separator = false;
            Some(ch.to_ascii_lowercase())
        } else if last_was_separator {
            None
        } else {
            last_was_separator = true;
            Some('_')
        };
        if let Some(ch) = mapped {
            slug.push(ch);
        }
    }
    let slug = slug.trim_matches('_');
    if slug.is_empty() {
        "workbook".to_string()
    } else {
        slug.to_string()
    }
}

fn output_bundle_path_from_model_arg(path: &str, cwd: &Path) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_workbook_migration_manifest output_bundle_path must not be empty"
                .to_string(),
        ));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_workbook_migration_manifest output_bundle_path must be a local path"
                .to_string(),
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
            "excel.generate_workbook_migration_manifest output_bundle_path must be relative and stay within the current working directory"
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
                "excel.generate_workbook_migration_manifest output_bundle_path must not traverse symlinks"
                    .to_string(),
            ));
        }
    }

    Ok(resolved_path)
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
