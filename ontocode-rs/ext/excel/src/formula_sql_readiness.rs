use std::collections::BTreeMap;
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
use serde::de::DeserializeOwned;
use serde_json::to_value;

use crate::backend::ExcelInspectionError;
use crate::formula_ast::FormulaAstNode;
use crate::formula_ast::FormulaAstParseState;
use crate::formula_inspect::inspect_sheet_formulas_with_display_path;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::FormulaSqlPreviewState;
use crate::tool::SheetFormulaSummary;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;
use crate::tool::workbook_path_from_model_arg;

pub(crate) const INSPECT_FORMULA_SQL_READINESS_TOOL_NAME: &str = "inspect_formula_sql_readiness";

const INSPECT_FORMULA_SQL_READINESS_DESCRIPTION: &str = "Inspect bounded worksheet formula readiness for the currently supported review-only SQL preview families.";
const MAX_READY_FORMULAS: usize = 64;
const MAX_BLOCKED_FORMULAS: usize = 64;
const MAX_BLOCKER_REASON_COUNTS: usize = 32;

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectFormulaSqlReadinessTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectFormulaSqlReadinessArgs {
    pub path: String,
    pub sheet: SheetSelector,
    #[serde(default)]
    pub max_formulas: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectFormulaSqlReadinessResult {
    pub path: String,
    pub sheet: SheetPreview,
    pub max_formulas_applied: usize,
    pub formula_count: usize,
    pub readiness_counts: FormulaSqlReadinessCounts,
    pub blocked_reason_counts: Vec<FormulaSqlBlockedReasonCount>,
    pub ready_formulas: Vec<FormulaSqlReadyFormula>,
    pub blocked_formulas: Vec<FormulaSqlBlockedFormula>,
    pub truncated: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct FormulaSqlReadinessCounts {
    pub scalar_row_local: usize,
    pub exact_lookup: usize,
    pub aligned_aggregate: usize,
    pub blocked: usize,
    pub malformed: usize,
    pub unsupported: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct FormulaSqlBlockedReasonCount {
    pub reason: String,
    pub count: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FormulaSqlReadinessFamily {
    ScalarRowLocal,
    ExactLookup,
    AlignedAggregate,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct FormulaSqlReadyFormula {
    pub reference: String,
    pub formula: String,
    pub family: FormulaSqlReadinessFamily,
    pub sql_expression: String,
    pub parse_state: FormulaAstParseState,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct FormulaSqlBlockedFormula {
    pub reference: String,
    pub formula: String,
    pub family_hint: FormulaSqlReadinessFamily,
    pub parse_state: FormulaAstParseState,
    pub blocker_reasons: Vec<String>,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectFormulaSqlReadinessTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_FORMULA_SQL_READINESS_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(InspectFormulaSqlReadinessArgs))
                .unwrap_or_else(|err| {
                    panic!("inspect_formula_sql_readiness args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(InspectFormulaSqlReadinessResult))
                .unwrap_or_else(|err| {
                    panic!("inspect_formula_sql_readiness result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_FORMULA_SQL_READINESS_TOOL_NAME.to_string(),
                    description: INSPECT_FORMULA_SQL_READINESS_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_formula_sql_readiness args schema should parse: {err}")
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
        let args = parse_tool_args::<InspectFormulaSqlReadinessArgs>(
            &call,
            "excel.inspect_formula_sql_readiness",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_formula_sql_readiness workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let result = inspect_formula_sql_readiness_from_workbook(
            &path,
            Path::new(args.path.trim()),
            &args.sheet,
            args.max_formulas,
        )
        .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize formula SQL readiness: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectFormulaSqlReadinessTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn inspect_formula_sql_readiness_from_workbook(
    path: &Path,
    display_path: &Path,
    sheet: &SheetSelector,
    max_formulas: Option<usize>,
) -> Result<InspectFormulaSqlReadinessResult, ExcelInspectionError> {
    let formulas =
        inspect_sheet_formulas_with_display_path(path, display_path, sheet, max_formulas)?;
    let mut readiness_counts = FormulaSqlReadinessCounts::default();
    let mut reason_counts = BTreeMap::<String, usize>::new();
    let mut ready_formulas = Vec::new();
    let mut blocked_formulas = Vec::new();
    let mut warnings = formulas.warnings.clone();

    for formula in &formulas.formulas {
        let family = classify_formula_family(formula);
        if formula.sql_preview.state == FormulaSqlPreviewState::ReviewOnly {
            increment_ready_count(&mut readiness_counts, family);
            if ready_formulas.len() < MAX_READY_FORMULAS
                && let Some(sql_expression) = formula.sql_preview.sql_expression.clone()
            {
                ready_formulas.push(FormulaSqlReadyFormula {
                    reference: formula.reference.clone(),
                    formula: formula.formula.clone(),
                    family,
                    sql_expression,
                    parse_state: formula.parse.state.clone(),
                    warnings: formula.warnings.clone(),
                });
            }
        } else {
            readiness_counts.blocked += 1;
            if formula.parse.state == FormulaAstParseState::Malformed {
                readiness_counts.malformed += 1;
            }
            if formula.parse.state == FormulaAstParseState::Unsupported {
                readiness_counts.unsupported += 1;
            }
            for reason in &formula.sql_preview.blocker_reasons {
                *reason_counts.entry(reason.clone()).or_default() += 1;
            }
            if blocked_formulas.len() < MAX_BLOCKED_FORMULAS {
                blocked_formulas.push(FormulaSqlBlockedFormula {
                    reference: formula.reference.clone(),
                    formula: formula.formula.clone(),
                    family_hint: family,
                    parse_state: formula.parse.state.clone(),
                    blocker_reasons: formula.sql_preview.blocker_reasons.clone(),
                    warnings: formula.warnings.clone(),
                });
            }
        }
    }

    let ready_sample_truncated = ready_formulas.len() < ready_count(&readiness_counts);
    if ready_sample_truncated {
        warnings.push(format!(
            "ready formula output truncated to {MAX_READY_FORMULAS} entries"
        ));
    }
    let blocked_sample_truncated = blocked_formulas.len() < readiness_counts.blocked;
    if blocked_sample_truncated {
        warnings.push(format!(
            "blocked formula output truncated to {MAX_BLOCKED_FORMULAS} entries"
        ));
    }
    let total_blocked_reason_kinds = reason_counts.len();
    let blocked_reason_counts = summarize_blocked_reason_counts(reason_counts);
    let blocker_reason_counts_truncated = total_blocked_reason_kinds > blocked_reason_counts.len();
    if blocker_reason_counts_truncated {
        warnings.push(format!(
            "blocked reason counts truncated to {MAX_BLOCKER_REASON_COUNTS} entries"
        ));
    }

    Ok(InspectFormulaSqlReadinessResult {
        path: formulas.path,
        sheet: formulas.sheet,
        max_formulas_applied: formulas.max_formulas_applied,
        formula_count: formulas.formulas.len(),
        readiness_counts,
        blocked_reason_counts,
        ready_formulas,
        blocked_formulas,
        truncated: formulas.truncated
            || ready_sample_truncated
            || blocked_sample_truncated
            || blocker_reason_counts_truncated,
        warnings,
    })
}

fn classify_formula_family(formula: &SheetFormulaSummary) -> FormulaSqlReadinessFamily {
    let Some(root) = formula.parse.root.as_ref() else {
        return FormulaSqlReadinessFamily::Unknown;
    };
    classify_ast_root(root)
}

fn classify_ast_root(root: &FormulaAstNode) -> FormulaSqlReadinessFamily {
    match root {
        FormulaAstNode::FunctionCall { name, .. } if is_exact_lookup_function(name) => {
            FormulaSqlReadinessFamily::ExactLookup
        }
        FormulaAstNode::FunctionCall { name, .. } if is_aligned_aggregate_function(name) => {
            FormulaSqlReadinessFamily::AlignedAggregate
        }
        FormulaAstNode::Unsupported { .. } => FormulaSqlReadinessFamily::Unknown,
        _ => FormulaSqlReadinessFamily::ScalarRowLocal,
    }
}

fn is_exact_lookup_function(name: &str) -> bool {
    name.eq_ignore_ascii_case("VLOOKUP")
        || name.eq_ignore_ascii_case("XLOOKUP")
        || name.eq_ignore_ascii_case("INDEX")
}

fn is_aligned_aggregate_function(name: &str) -> bool {
    name.eq_ignore_ascii_case("SUMIFS")
        || name.eq_ignore_ascii_case("COUNTIFS")
        || name.eq_ignore_ascii_case("AVERAGEIFS")
        || name.eq_ignore_ascii_case("MAXIFS")
        || name.eq_ignore_ascii_case("MINIFS")
}

fn increment_ready_count(
    counts: &mut FormulaSqlReadinessCounts,
    family: FormulaSqlReadinessFamily,
) {
    match family {
        FormulaSqlReadinessFamily::ScalarRowLocal => counts.scalar_row_local += 1,
        FormulaSqlReadinessFamily::ExactLookup => counts.exact_lookup += 1,
        FormulaSqlReadinessFamily::AlignedAggregate => counts.aligned_aggregate += 1,
        FormulaSqlReadinessFamily::Unknown => counts.blocked += 1,
    }
}

fn ready_count(counts: &FormulaSqlReadinessCounts) -> usize {
    counts.scalar_row_local + counts.exact_lookup + counts.aligned_aggregate
}

fn summarize_blocked_reason_counts(
    reason_counts: BTreeMap<String, usize>,
) -> Vec<FormulaSqlBlockedReasonCount> {
    let mut entries = reason_counts
        .into_iter()
        .map(|(reason, count)| FormulaSqlBlockedReasonCount { reason, count })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| left.reason.cmp(&right.reason))
    });
    entries.truncate(MAX_BLOCKER_REASON_COUNTS);
    entries
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
