use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::fs::read_to_string;
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
use quick_xml::Reader;
use quick_xml::events::Event;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_value;
use zip::ZipArchive;

use crate::backend::ExcelInspectionError;
use crate::formula_inspect::inspect_sheet_formulas_with_display_path;
use crate::preview::attr_value;
use crate::preview::read_xml_entry;
use crate::tool::DefinedNameSummary;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;

pub(crate) const NAMED_RANGE_REWRITE_DRY_RUN_TOOL_NAME: &str = "named_range_rewrite_dry_run";

const NAMED_RANGE_REWRITE_DRY_RUN_DESCRIPTION: &str = "Run a bounded dry-run that proposes exact-text replacements from direct worksheet references to existing defined names in one worksheet formula set.";
const MAX_WORKBOOK_XML_BYTES: usize = 2 * 1024 * 1024;

#[derive(Clone, Default)]
pub(crate) struct ExcelNamedRangeRewriteDryRunTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct NamedRangeRewriteDryRunArgs {
    pub path: String,
    pub sheet: SheetSelector,
    pub mapping_path: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RewriteScopeExpectation {
    Workbook,
    Sheet,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct NamedRangeRewriteMappingEntry {
    pub workbook_path: String,
    pub sheet_name: String,
    pub formula_targets: Vec<String>,
    pub from_ref: String,
    pub to_name: String,
    pub scope_expectation: RewriteScopeExpectation,
    pub sheet_name_for_scope: Option<String>,
    pub max_replacements_per_formula: usize,
    pub reference_mode: String,
    pub all_or_nothing: bool,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct NamedRangeRewriteMatch {
    pub from_ref: String,
    pub to_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct NamedRangeRewriteDryRunFormulaResult {
    pub formula_reference: String,
    pub original_formula: String,
    pub proposed_rewritten_formula: Option<String>,
    pub matched_mapping_entries: Vec<NamedRangeRewriteMatch>,
    pub blocker_reasons: Vec<String>,
    pub confidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct NamedRangeRewriteDryRunResult {
    pub path: String,
    pub mapping_path: String,
    pub sheet: SheetPreview,
    pub results: Vec<NamedRangeRewriteDryRunFormulaResult>,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelNamedRangeRewriteDryRunTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, NAMED_RANGE_REWRITE_DRY_RUN_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema = serde_json::to_value(schemars::schema_for!(NamedRangeRewriteDryRunArgs))
            .unwrap_or_else(|err| {
                panic!("named_range_rewrite_dry_run args schema should serialize: {err}")
            });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(NamedRangeRewriteDryRunResult))
                .unwrap_or_else(|err| {
                    panic!("named_range_rewrite_dry_run result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: NAMED_RANGE_REWRITE_DRY_RUN_TOOL_NAME.to_string(),
                    description: NAMED_RANGE_REWRITE_DRY_RUN_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("named_range_rewrite_dry_run args schema should parse: {err}")
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
        let args = parse_tool_args(&call)?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.named_range_rewrite_dry_run workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = workbook_path_from_named_range_arg(&args.path, &cwd)?;
        let mapping_path = mapping_path_from_model_arg(&args.mapping_path, &cwd)?;
        let result = named_range_rewrite_dry_run_with_display_path(
            &workbook_path,
            Path::new(args.path.trim()),
            &args.sheet,
            &mapping_path,
            Path::new(args.mapping_path.trim()),
        )
        .map_err(FunctionCallError::from)?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize named-range rewrite dry-run: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelNamedRangeRewriteDryRunTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn named_range_rewrite_dry_run_with_display_path(
    workbook_path: &Path,
    workbook_display_path: &Path,
    sheet: &SheetSelector,
    mapping_path: &Path,
    mapping_display_path: &Path,
) -> Result<NamedRangeRewriteDryRunResult, ExcelInspectionError> {
    let inspected = inspect_sheet_formulas_with_display_path(
        workbook_path,
        workbook_display_path,
        sheet,
        None,
    )?;
    let workbook_reference_mode = workbook_reference_mode(workbook_path)?;
    let mappings = load_mappings(
        mapping_path,
        workbook_display_path,
        inspected.sheet.name.as_str(),
    )?;
    let formulas_by_ref = inspected
        .formulas
        .iter()
        .map(|formula| (formula.reference.clone(), formula.formula.clone()))
        .collect::<BTreeMap<_, _>>();
    let name_catalog = build_name_catalog(&inspected.defined_names);

    let mut results = Vec::new();
    for (formula_reference, entries) in mappings {
        let original_formula = formulas_by_ref
            .get(formula_reference.as_str())
            .cloned()
            .unwrap_or_default();
        let mut proposed_formula = Some(original_formula.clone());
        let mut matched_mapping_entries = Vec::new();
        let mut blocker_reasons = BTreeSet::new();

        if original_formula.is_empty() {
            blocker_reasons.insert("formula-target-not-found".to_string());
        }
        if inspected.has_external_links {
            blocker_reasons.insert("external-link-blocked".to_string());
        }
        if workbook_reference_mode.as_deref() == Some("R1C1") {
            blocker_reasons.insert("unsupported-reference-mode".to_string());
        }

        for entry in &entries {
            blocker_reasons.extend(validate_mapping_entry(
                entry,
                inspected.sheet.name.as_str(),
                &name_catalog,
            ));
            if !blocker_reasons.is_empty() {
                continue;
            }

            let formula = proposed_formula.clone().unwrap_or_default();
            let match_count = formula.match_indices(entry.from_ref.as_str()).count();
            if match_count == 0 || match_count > entry.max_replacements_per_formula {
                blocker_reasons.insert("replacement-count-mismatch".to_string());
                continue;
            }

            proposed_formula =
                Some(formula.replace(entry.from_ref.as_str(), entry.to_name.as_str()));
            matched_mapping_entries.push(NamedRangeRewriteMatch {
                from_ref: entry.from_ref.clone(),
                to_name: entry.to_name.clone(),
            });
        }

        if !blocker_reasons.is_empty() {
            proposed_formula = None;
        }

        let blocker_reasons = blocker_reasons.into_iter().collect::<Vec<_>>();
        results.push(NamedRangeRewriteDryRunFormulaResult {
            formula_reference,
            original_formula,
            proposed_rewritten_formula: proposed_formula,
            matched_mapping_entries,
            confidence: if blocker_reasons.is_empty() {
                "high".to_string()
            } else {
                "blocked".to_string()
            },
            blocker_reasons,
        });
    }

    Ok(NamedRangeRewriteDryRunResult {
        path: workbook_display_path.display().to_string(),
        mapping_path: mapping_display_path.display().to_string(),
        sheet: inspected.sheet,
        results,
        warnings: Vec::new(),
    })
}

fn parse_tool_args(call: &ToolCall) -> Result<NamedRangeRewriteDryRunArgs, FunctionCallError> {
    let arguments = call.function_arguments()?;
    serde_json::from_str(arguments).map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "invalid excel.named_range_rewrite_dry_run arguments: {err}"
        ))
    })
}

fn workbook_path_from_named_range_arg(
    path: &str,
    cwd: &Path,
) -> Result<PathBuf, FunctionCallError> {
    local_relative_path_from_model_arg(
        path,
        cwd,
        "excel.named_range_rewrite_dry_run path",
        &["xlsx", "xlsm", "xlsb"],
    )
}

fn mapping_path_from_model_arg(path: &str, cwd: &Path) -> Result<PathBuf, FunctionCallError> {
    local_relative_path_from_model_arg(
        path,
        cwd,
        "excel.named_range_rewrite_dry_run mapping_path",
        &["json"],
    )
}

fn local_relative_path_from_model_arg(
    path: &str,
    cwd: &Path,
    label: &str,
    extensions: &[&str],
) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(format!(
            "{label} must not be empty"
        )));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(format!(
            "{label} must be a local path"
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
            "{label} must be relative and stay within the current working directory"
        )));
    }

    let Some(extension) = path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
    else {
        return Err(FunctionCallError::RespondToModel(format!(
            "{label} must end in .{}",
            extensions.join(", .")
        )));
    };
    if !extensions.contains(&extension.as_str()) {
        return Err(FunctionCallError::RespondToModel(format!(
            "{label} must end in .{}",
            extensions.join(", .")
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
                "{label} must not traverse symlinks"
            )));
        }
    }

    Ok(resolved_path)
}

fn load_mappings(
    mapping_path: &Path,
    workbook_display_path: &Path,
    selected_sheet_name: &str,
) -> Result<BTreeMap<String, Vec<NamedRangeRewriteMappingEntry>>, ExcelInspectionError> {
    let payload = read_to_string(mapping_path).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to read named-range rewrite mapping {}: {err}",
            mapping_path.display()
        ))
    })?;
    let entries: Vec<NamedRangeRewriteMappingEntry> = serde_json::from_str(payload.as_str())
        .map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to parse named-range rewrite mapping {}: {err}",
                mapping_path.display()
            ))
        })?;
    let workbook_display = workbook_display_path.display().to_string();
    let mut grouped = BTreeMap::<String, Vec<NamedRangeRewriteMappingEntry>>::new();
    for entry in entries {
        if entry.workbook_path.trim() != workbook_display
            || entry.sheet_name.trim() != selected_sheet_name
        {
            continue;
        }
        for formula_target in &entry.formula_targets {
            grouped
                .entry(formula_target.trim().to_string())
                .or_default()
                .push(entry.clone());
        }
    }
    if grouped.is_empty() {
        return Err(ExcelInspectionError::Message(format!(
            "named-range rewrite mapping {} has no entries for workbook {} and sheet {}",
            mapping_path.display(),
            workbook_display,
            selected_sheet_name
        )));
    }
    Ok(grouped)
}

#[derive(Default)]
struct NameCatalog {
    workbook_names: BTreeMap<String, DefinedNameSummary>,
    names_with_sheet_scope: BTreeSet<String>,
}

fn build_name_catalog(defined_names: &[DefinedNameSummary]) -> NameCatalog {
    let mut catalog = NameCatalog::default();
    for defined_name in defined_names {
        if defined_name.sheet_scope.is_some() || defined_name.local_sheet_id.is_some() {
            catalog
                .names_with_sheet_scope
                .insert(defined_name.name.clone());
            continue;
        }
        catalog
            .workbook_names
            .entry(defined_name.name.clone())
            .or_insert_with(|| defined_name.clone());
    }
    catalog
}

fn validate_mapping_entry(
    entry: &NamedRangeRewriteMappingEntry,
    selected_sheet_name: &str,
    name_catalog: &NameCatalog,
) -> BTreeSet<String> {
    let mut blockers = BTreeSet::new();
    if entry.sheet_name.trim() != selected_sheet_name {
        blockers.insert("sheet-name-mismatch".to_string());
    }
    if !matches!(entry.scope_expectation, RewriteScopeExpectation::Workbook) {
        blockers.insert("scope-mismatch".to_string());
    }
    if entry.reference_mode != "exact_textual_match_only" {
        blockers.insert("unsupported-reference-mode".to_string());
    }
    let Some(name) = name_catalog.workbook_names.get(entry.to_name.as_str()) else {
        blockers.insert("name-not-found".to_string());
        return blockers;
    };
    if name_catalog
        .names_with_sheet_scope
        .contains(entry.to_name.as_str())
    {
        blockers.insert("ambiguous-sheet-scope".to_string());
    }
    if name.hidden.unwrap_or(false) || entry.to_name.starts_with("_xlnm.") {
        blockers.insert("hidden-or-internal-name".to_string());
    }
    if name.target.contains('[') {
        blockers.insert("external-link-blocked".to_string());
    }
    if name.target.contains("#REF!") || name.target.contains('(') || name.target.contains(')') {
        blockers.insert("unsupported-name-target".to_string());
    }
    if name.target != entry.from_ref {
        blockers.insert("target-mismatch".to_string());
    }
    blockers
}

fn workbook_reference_mode(path: &Path) -> Result<Option<String>, ExcelInspectionError> {
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
    let mut reader = Reader::from_str(workbook_xml.as_str());
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if event.name().as_ref() == b"calcPr" =>
            {
                return attr_value(&event, b"refMode");
            }
            Ok(Event::Eof) => return Ok(None),
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse workbook reference mode: {err}"
                )));
            }
        }
        buf.clear();
    }
}
