use std::collections::BTreeMap;
use std::collections::BTreeSet;
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

use crate::backend::ExcelInspectionError;
use crate::formula_sql_readiness::FormulaSqlBlockedFormula;
use crate::formula_sql_readiness::FormulaSqlReadinessFamily;
use crate::formula_sql_readiness::FormulaSqlReadyFormula;
use crate::formula_sql_readiness::inspect_formula_sql_readiness_from_workbook;
use crate::slider_query::FormulaDependencySummary;
use crate::slider_query::ScanSheetFormulasDependencyResult;
use crate::slider_query::scan_sheet_formulas_dependency_with_display_path;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;
use crate::tool::workbook_path_from_model_arg;

pub(crate) const INSPECT_FORMULA_CTE_PIPELINE_TOOL_NAME: &str = "inspect_formula_cte_pipeline";

const INSPECT_FORMULA_CTE_PIPELINE_DESCRIPTION: &str = "Inspect review-only staged CTE candidates by composing worksheet formula dependency and SQL-readiness evidence.";

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectFormulaCtePipelineTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectFormulaCtePipelineArgs {
    pub path: String,
    pub sheet: SheetSelector,
    #[serde(default)]
    pub max_formulas: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InspectFormulaCtePipelineResult {
    pub path: String,
    pub sheet: SheetPreview,
    pub max_formulas_applied: usize,
    pub formula_count: usize,
    pub ready_formula_count: usize,
    pub blocked_formula_count: usize,
    pub stage_count: usize,
    pub stages: Vec<FormulaCteStage>,
    pub blocked_formulas: Vec<FormulaCteBlockedFormula>,
    pub cycles_detected: Vec<Vec<String>>,
    pub truncated: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FormulaCteStage {
    pub stage_index: usize,
    pub depends_on_stages: Vec<usize>,
    pub candidate_groups: Vec<FormulaCteCandidateGroup>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FormulaCteCandidateGroup {
    pub group_name: String,
    pub family: FormulaSqlReadinessFamily,
    pub formula_references: Vec<String>,
    pub formulas: Vec<FormulaCteCandidateFormula>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FormulaCteCandidateFormula {
    pub reference: String,
    pub formula: String,
    pub sql_expression: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FormulaCteBlockedFormula {
    pub reference: String,
    pub formula: String,
    pub blocker_reasons: Vec<String>,
    pub blocked_by: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
struct BlockedFormulaAccumulator {
    reference: String,
    formula: String,
    dependencies: Vec<String>,
    blocked_by: Vec<String>,
    blocker_reasons: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct ReadyFormulaPlan {
    reference: String,
    formula: String,
    family: FormulaSqlReadinessFamily,
    sql_expression: String,
    dependencies: Vec<String>,
    formula_dependencies: Vec<String>,
    stage_index: Option<usize>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectFormulaCtePipelineTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_FORMULA_CTE_PIPELINE_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(InspectFormulaCtePipelineArgs))
                .unwrap_or_else(|err| {
                    panic!("inspect_formula_cte_pipeline args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(InspectFormulaCtePipelineResult))
                .unwrap_or_else(|err| {
                    panic!("inspect_formula_cte_pipeline result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_FORMULA_CTE_PIPELINE_TOOL_NAME.to_string(),
                    description: INSPECT_FORMULA_CTE_PIPELINE_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_formula_cte_pipeline args schema should parse: {err}")
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
        let args = crate::vba_extract::parse_tool_args::<InspectFormulaCtePipelineArgs>(
            &call,
            "excel.inspect_formula_cte_pipeline",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_formula_cte_pipeline workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let result = inspect_formula_cte_pipeline_from_workbook(
            &path,
            Path::new(args.path.trim()),
            &args.sheet,
            args.max_formulas,
        )
        .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize formula CTE pipeline result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectFormulaCtePipelineTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn inspect_formula_cte_pipeline_from_workbook(
    path: &Path,
    display_path: &Path,
    sheet: &SheetSelector,
    max_formulas: Option<usize>,
) -> Result<InspectFormulaCtePipelineResult, ExcelInspectionError> {
    let readiness =
        inspect_formula_sql_readiness_from_workbook(path, display_path, sheet, max_formulas)?;
    let dependency =
        scan_sheet_formulas_dependency_with_display_path(path, display_path, sheet, max_formulas)?;

    let mut warnings = readiness.warnings.clone();
    warnings.extend(dependency.warnings.iter().cloned());
    if readiness.formula_count != dependency.nodes.len() {
        warnings.push(format!(
            "formula inventory mismatch between readiness ({}) and dependency scan ({})",
            readiness.formula_count,
            dependency.nodes.len()
        ));
    }

    let dependency_index = dependency
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.cell.clone(), index))
        .collect::<BTreeMap<_, _>>();
    let formula_cells = dependency_index.keys().cloned().collect::<BTreeSet<_>>();
    let mut blocked = collect_initial_blocked_formulas(&dependency, &readiness.blocked_formulas);
    propagate_blocked_dependencies(&dependency.nodes, &formula_cells, &mut blocked);

    let mut ready_plans = readiness
        .ready_formulas
        .iter()
        .filter(|formula| !blocked.contains_key(&formula.reference))
        .map(|formula| build_ready_formula_plan(formula, &dependency.nodes, &formula_cells))
        .collect::<Vec<_>>();

    let stage_warnings = assign_stage_indexes(&mut ready_plans, &dependency_index);
    warnings.extend(stage_warnings);
    let stages = build_cte_stages(&ready_plans, &dependency_index, &mut warnings);
    let blocked_formulas = collect_blocked_formulas(&dependency.nodes, blocked);

    Ok(InspectFormulaCtePipelineResult {
        path: readiness.path,
        sheet: readiness.sheet,
        max_formulas_applied: readiness
            .max_formulas_applied
            .max(dependency.max_formulas_applied),
        formula_count: dependency.nodes.len(),
        ready_formula_count: ready_plans.len(),
        blocked_formula_count: blocked_formulas.len(),
        stage_count: stages.len(),
        stages,
        blocked_formulas,
        cycles_detected: dependency.cycles_detected,
        truncated: readiness.truncated || dependency.truncated,
        warnings,
    })
}

fn collect_initial_blocked_formulas(
    dependency: &ScanSheetFormulasDependencyResult,
    readiness_blocked: &[FormulaSqlBlockedFormula],
) -> BTreeMap<String, BlockedFormulaAccumulator> {
    let mut blocked = BTreeMap::<String, BlockedFormulaAccumulator>::new();
    let dependency_by_reference = dependency
        .nodes
        .iter()
        .map(|node| (node.cell.clone(), node))
        .collect::<BTreeMap<_, _>>();

    for node in &dependency.nodes {
        if node.has_cycle {
            blocked
                .entry(node.cell.clone())
                .or_insert_with(|| BlockedFormulaAccumulator {
                    reference: node.cell.clone(),
                    formula: node.formula.clone(),
                    dependencies: node.dependencies.clone(),
                    blocked_by: Vec::new(),
                    blocker_reasons: BTreeSet::new(),
                })
                .blocker_reasons
                .insert("circular_dependency".to_string());
        }
        if !node.is_supported {
            blocked
                .entry(node.cell.clone())
                .or_insert_with(|| BlockedFormulaAccumulator {
                    reference: node.cell.clone(),
                    formula: node.formula.clone(),
                    dependencies: node.dependencies.clone(),
                    blocked_by: Vec::new(),
                    blocker_reasons: BTreeSet::new(),
                })
                .blocker_reasons
                .insert(
                    node.unsupported_reason
                        .clone()
                        .unwrap_or_else(|| "unsupported_formula".to_string()),
                );
        }
    }

    for formula in readiness_blocked {
        let entry =
            blocked
                .entry(formula.reference.clone())
                .or_insert_with(|| BlockedFormulaAccumulator {
                    reference: formula.reference.clone(),
                    formula: formula.formula.clone(),
                    dependencies: dependency_by_reference
                        .get(&formula.reference)
                        .map(|node| node.dependencies.clone())
                        .unwrap_or_default(),
                    blocked_by: Vec::new(),
                    blocker_reasons: BTreeSet::new(),
                });
        for reason in &formula.blocker_reasons {
            entry.blocker_reasons.insert(reason.clone());
        }
    }

    blocked
}

fn propagate_blocked_dependencies(
    nodes: &[FormulaDependencySummary],
    formula_cells: &BTreeSet<String>,
    blocked: &mut BTreeMap<String, BlockedFormulaAccumulator>,
) {
    let node_by_reference = nodes
        .iter()
        .map(|node| (node.cell.clone(), node))
        .collect::<BTreeMap<_, _>>();
    let mut changed = true;
    while changed {
        changed = false;
        for node in nodes {
            if blocked.contains_key(&node.cell) {
                continue;
            }
            let blocked_dependencies = node
                .dependencies
                .iter()
                .filter(|dependency| {
                    formula_cells.contains(*dependency) && blocked.contains_key(*dependency)
                })
                .cloned()
                .collect::<Vec<_>>();
            if blocked_dependencies.is_empty() {
                continue;
            }
            let Some(node_summary) = node_by_reference.get(&node.cell) else {
                continue;
            };
            blocked.insert(
                node.cell.clone(),
                BlockedFormulaAccumulator {
                    reference: node.cell.clone(),
                    formula: node.formula.clone(),
                    dependencies: node_summary.dependencies.clone(),
                    blocked_by: blocked_dependencies,
                    blocker_reasons: BTreeSet::from(["blocked_dependency".to_string()]),
                },
            );
            changed = true;
        }
    }
}

fn build_ready_formula_plan(
    formula: &FormulaSqlReadyFormula,
    dependency_nodes: &[FormulaDependencySummary],
    formula_cells: &BTreeSet<String>,
) -> ReadyFormulaPlan {
    let dependency_node = dependency_nodes
        .iter()
        .find(|node| node.cell == formula.reference);
    let dependencies = dependency_node
        .map(|node| node.dependencies.clone())
        .unwrap_or_default();
    let formula_dependencies = dependencies
        .iter()
        .filter(|dependency| formula_cells.contains(*dependency))
        .cloned()
        .collect::<Vec<_>>();

    ReadyFormulaPlan {
        reference: formula.reference.clone(),
        formula: formula.formula.clone(),
        family: formula.family,
        sql_expression: formula.sql_expression.clone(),
        dependencies,
        formula_dependencies,
        stage_index: None,
    }
}

fn assign_stage_indexes(
    ready_plans: &mut [ReadyFormulaPlan],
    dependency_index: &BTreeMap<String, usize>,
) -> Vec<String> {
    let plan_index = ready_plans
        .iter()
        .enumerate()
        .map(|(index, plan)| (plan.reference.clone(), index))
        .collect::<BTreeMap<_, _>>();
    let mut indegree = BTreeMap::<String, usize>::new();
    let mut adjacency = BTreeMap::<String, Vec<String>>::new();
    for plan in ready_plans.iter() {
        indegree.entry(plan.reference.clone()).or_insert(0);
        for dependency in &plan.formula_dependencies {
            if !plan_index.contains_key(dependency) {
                continue;
            }
            adjacency
                .entry(dependency.clone())
                .or_default()
                .push(plan.reference.clone());
            *indegree.entry(plan.reference.clone()).or_insert(0) += 1;
        }
    }

    let mut ready = indegree
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(reference, _)| reference.clone())
        .collect::<Vec<_>>();
    ready.sort_by_key(|reference| {
        dependency_index
            .get(reference)
            .copied()
            .unwrap_or(usize::MAX)
    });
    let mut ready = ready.into_iter().collect::<BTreeSet<_>>();
    let mut warnings = Vec::new();
    let mut assigned_count = 0usize;

    while let Some(reference) = ready
        .iter()
        .min_by_key(|candidate| {
            dependency_index
                .get(*candidate)
                .copied()
                .unwrap_or(usize::MAX)
        })
        .cloned()
    {
        ready.remove(&reference);
        let Some(plan_position) = plan_index.get(&reference).copied() else {
            continue;
        };
        let stage_index = ready_plans[plan_position]
            .formula_dependencies
            .iter()
            .filter_map(|dependency| {
                plan_index
                    .get(dependency)
                    .and_then(|dependency_index| ready_plans[*dependency_index].stage_index)
            })
            .max()
            .map_or(0, |stage| stage + 1);
        ready_plans[plan_position].stage_index = Some(stage_index);
        assigned_count += 1;

        if let Some(children) = adjacency.get(&reference) {
            for child in children {
                if let Some(degree) = indegree.get_mut(child) {
                    *degree = degree.saturating_sub(1);
                    if *degree == 0 {
                        ready.insert(child.clone());
                    }
                }
            }
        }
    }

    if assigned_count != ready_plans.len() {
        warnings.push(
            "some ready formulas could not be assigned to a deterministic CTE stage".to_string(),
        );
    }
    warnings
}

fn build_cte_stages(
    ready_plans: &[ReadyFormulaPlan],
    dependency_index: &BTreeMap<String, usize>,
    warnings: &mut Vec<String>,
) -> Vec<FormulaCteStage> {
    let stage_map = ready_plans
        .iter()
        .enumerate()
        .filter_map(|(index, plan)| {
            plan.stage_index
                .map(|stage_index| (stage_index, (index, plan.clone())))
        })
        .fold(
            BTreeMap::<usize, Vec<(usize, ReadyFormulaPlan)>>::new(),
            |mut acc, (stage_index, value)| {
                acc.entry(stage_index).or_default().push(value);
                acc
            },
        );

    let plan_stage_by_reference = ready_plans
        .iter()
        .filter_map(|plan| {
            plan.stage_index
                .map(|stage_index| (plan.reference.clone(), stage_index))
        })
        .collect::<BTreeMap<_, _>>();

    stage_map
        .into_iter()
        .map(|(stage_index, mut entries)| {
            entries.sort_by_key(|(_, plan)| {
                dependency_index
                    .get(&plan.reference)
                    .copied()
                    .unwrap_or(usize::MAX)
            });

            let depends_on_stages = entries
                .iter()
                .flat_map(|(_, plan)| plan.formula_dependencies.iter())
                .filter_map(|dependency| plan_stage_by_reference.get(dependency).copied())
                .filter(|dependency_stage| *dependency_stage < stage_index)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            let family_count = entries
                .iter()
                .map(|(_, plan)| family_slug(plan.family).to_string())
                .collect::<BTreeSet<_>>()
                .len();
            let mut stage_warnings = Vec::new();
            if family_count > 1 {
                let warning = format!(
                    "stage_{stage_index} mixes multiple readiness families; candidate CTE grouping is heuristic"
                );
                warnings.push(warning.clone());
                stage_warnings.push(warning);
            }

            let mut grouped =
                BTreeMap::<String, (FormulaSqlReadinessFamily, Vec<ReadyFormulaPlan>)>::new();
            for (_, plan) in entries {
                grouped
                    .entry(family_slug(plan.family).to_string())
                    .or_insert_with(|| (plan.family, Vec::new()))
                    .1
                    .push(plan);
            }
            let candidate_groups = grouped
                .into_iter()
                .map(|(family_key, (family, formulas))| FormulaCteCandidateGroup {
                    group_name: format!("stage_{stage_index}_{family_key}"),
                    family,
                    formula_references: formulas
                        .iter()
                        .map(|formula| formula.reference.clone())
                        .collect(),
                    formulas: formulas
                        .into_iter()
                        .map(|formula| FormulaCteCandidateFormula {
                            reference: formula.reference,
                            formula: formula.formula,
                            sql_expression: formula.sql_expression,
                            dependencies: formula.dependencies,
                        })
                        .collect(),
                })
                .collect::<Vec<_>>();

            FormulaCteStage {
                stage_index,
                depends_on_stages,
                candidate_groups,
                warnings: stage_warnings,
            }
        })
        .collect()
}

fn collect_blocked_formulas(
    dependency_nodes: &[FormulaDependencySummary],
    blocked: BTreeMap<String, BlockedFormulaAccumulator>,
) -> Vec<FormulaCteBlockedFormula> {
    let order = dependency_nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.cell.clone(), index))
        .collect::<BTreeMap<_, _>>();
    let mut blocked = blocked.into_values().collect::<Vec<_>>();
    blocked.sort_by_key(|formula| order.get(&formula.reference).copied().unwrap_or(usize::MAX));
    blocked
        .into_iter()
        .map(|formula| FormulaCteBlockedFormula {
            reference: formula.reference,
            formula: formula.formula,
            blocker_reasons: formula.blocker_reasons.into_iter().collect(),
            blocked_by: formula.blocked_by,
            dependencies: formula.dependencies,
        })
        .collect()
}

fn family_slug(family: FormulaSqlReadinessFamily) -> &'static str {
    match family {
        FormulaSqlReadinessFamily::ScalarRowLocal => "scalar_row_local",
        FormulaSqlReadinessFamily::ExactLookup => "exact_lookup",
        FormulaSqlReadinessFamily::AlignedAggregate => "aligned_aggregate",
        FormulaSqlReadinessFamily::Unknown => "unknown",
    }
}
