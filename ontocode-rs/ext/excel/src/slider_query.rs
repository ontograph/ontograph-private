use std::collections::BTreeMap;
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
use crate::formula_ast::FormulaAstBinaryOperator;
use crate::formula_ast::FormulaAstNode;
use crate::formula_ast::FormulaAstParseState;
use crate::formula_ast::FormulaAstUnaryOperator;
use crate::formula_inspect::inspect_sheet_formulas_with_display_path;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::SheetFormulaSummary;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;
use crate::tool::WorkbookFormat;

pub(crate) const SCAN_SHEET_FORMULAS_DEPENDENCY_TOOL_NAME: &str = "scan_sheet_formulas_dependency";
pub(crate) const GENERATE_SLIDER_QUERY_PACKAGE_TOOL_NAME: &str = "generate_slider_query_package";

const SCAN_SHEET_FORMULAS_DEPENDENCY_DESCRIPTION: &str =
    "Resolve bounded worksheet formula dependencies into a local DAG preview and cycle report.";
const GENERATE_SLIDER_QUERY_PACKAGE_DESCRIPTION: &str =
    "Generate a bounded SliderQuery package from one worksheet's clean formula dependency chain.";

const DEFAULT_MAX_FORMULAS: usize = 128;
const MAX_FORMULAS: usize = 512;

#[derive(Clone, Default)]
pub(crate) struct ExcelScanSheetFormulasDependencyTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Clone, Default)]
pub(crate) struct ExcelGenerateSliderQueryPackageTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScanSheetFormulasDependencyArgs {
    pub path: String,
    pub sheet: SheetSelector,
    #[serde(default)]
    pub max_formulas: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct GenerateSliderQueryPackageArgs {
    pub path: String,
    pub sheet: SheetSelector,
    pub output_package_path: String,
    #[serde(default)]
    pub max_formulas: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FormulaDependencySummary {
    pub cell: String,
    pub formula: String,
    pub dependencies: Vec<String>,
    pub has_cycle: bool,
    pub is_supported: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsupported_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScanSheetFormulasDependencyResult {
    pub path: String,
    pub sheet: SheetPreview,
    pub max_formulas_applied: usize,
    pub nodes: Vec<FormulaDependencySummary>,
    pub cycles_detected: Vec<Vec<String>>,
    pub truncated: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum SliderQueryGeneratedQueryType {
    PreparedColumns,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SliderQueryGeneratedQuerySummary {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: SliderQueryGeneratedQueryType,
    pub sql_path: String,
    pub variable_path: String,
    pub blocked_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SliderQueryBlockedFormulaSummary {
    pub cell: String,
    pub formula: String,
    pub reason: String,
    pub blocked_by: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SliderQueryPackageManifest {
    pub package_name: String,
    pub generated_queries: Vec<SliderQueryGeneratedQuerySummary>,
    pub blocked_formulas: Vec<SliderQueryBlockedFormulaSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenerateSliderQueryPackageResult {
    pub path: String,
    pub sheet: SheetPreview,
    pub package_path: String,
    pub manifest_path: String,
    pub manifest: SliderQueryPackageManifest,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct DependencyAnalysis {
    summary: SheetFormulaSummary,
    dependencies: Vec<String>,
    formula_dependencies: Vec<String>,
    is_supported: bool,
    unsupported_reason: Option<String>,
    has_cycle: bool,
    blocked_by: Vec<String>,
    package_reason: Option<String>,
    expression: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct SliderQueryVariableFile {
    sheet_name: String,
    source_table: String,
    source_columns: Vec<String>,
    prepared_columns: Vec<SliderQueryVariableColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct SliderQueryVariableColumn {
    cell: String,
    sql_alias: String,
    expression: String,
    dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
struct DependencyPlan {
    sheet_slug: String,
    nodes: Vec<DependencyAnalysis>,
    prepared_columns: Vec<SliderQueryVariableColumn>,
    cycles_detected: Vec<Vec<String>>,
    truncated: bool,
    warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelScanSheetFormulasDependencyTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, SCAN_SHEET_FORMULAS_DEPENDENCY_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(ScanSheetFormulasDependencyArgs))
                .unwrap_or_else(|err| {
                    panic!("scan_sheet_formulas_dependency args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(ScanSheetFormulasDependencyResult))
                .unwrap_or_else(|err| {
                    panic!("scan_sheet_formulas_dependency result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: SCAN_SHEET_FORMULAS_DEPENDENCY_TOOL_NAME.to_string(),
                    description: SCAN_SHEET_FORMULAS_DEPENDENCY_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("scan_sheet_formulas_dependency args schema should parse: {err}")
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
        let args = parse_tool_args::<ScanSheetFormulasDependencyArgs>(
            &call,
            "excel.scan_sheet_formulas_dependency",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.scan_sheet_formulas_dependency workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let result = scan_sheet_formulas_dependency_with_display_path(
            &path,
            Path::new(args.path.trim()),
            &args.sheet,
            args.max_formulas,
        )
        .map_err(FunctionCallError::from)?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize worksheet dependency scan: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelGenerateSliderQueryPackageTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, GENERATE_SLIDER_QUERY_PACKAGE_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(GenerateSliderQueryPackageArgs))
                .unwrap_or_else(|err| {
                    panic!("generate_slider_query_package args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(GenerateSliderQueryPackageResult))
                .unwrap_or_else(|err| {
                    panic!("generate_slider_query_package result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: GENERATE_SLIDER_QUERY_PACKAGE_TOOL_NAME.to_string(),
                    description: GENERATE_SLIDER_QUERY_PACKAGE_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("generate_slider_query_package args schema should parse: {err}")
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
        let args = parse_tool_args::<GenerateSliderQueryPackageArgs>(
            &call,
            "excel.generate_slider_query_package",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.generate_slider_query_package workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let output_package_path =
            package_output_path_from_model_arg(&args.output_package_path, &cwd)?;
        let result = generate_slider_query_package_with_display_path(
            &path,
            Path::new(args.path.trim()),
            &args.sheet,
            &output_package_path,
            Path::new(args.output_package_path.trim()),
            args.max_formulas,
        )
        .map_err(FunctionCallError::from)?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize slider query package generation: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelScanSheetFormulasDependencyTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

impl ExcelGenerateSliderQueryPackageTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn scan_sheet_formulas_dependency_with_display_path(
    path: &Path,
    display_path: &Path,
    sheet: &SheetSelector,
    max_formulas: Option<usize>,
) -> Result<ScanSheetFormulasDependencyResult, ExcelInspectionError> {
    let workbook = inspect_workbook_with_display_path(path, display_path)?;
    if !matches!(workbook.format, WorkbookFormat::Xlsx | WorkbookFormat::Xlsm) {
        return Err(ExcelInspectionError::Message(
            "excel.scan_sheet_formulas_dependency supports only .xlsx and .xlsm in this stage"
                .to_string(),
        ));
    }

    let selected_sheet = crate::preview::select_sheet(&workbook.sheets, sheet)?;
    let sheet_name = selected_sheet.name.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.scan_sheet_formulas_dependency could not resolve a sheet name for the selected sheet"
                .to_string(),
        )
    })?;
    let sheet_part_path = selected_sheet.part_path.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.scan_sheet_formulas_dependency could not resolve a worksheet part path for the selected sheet"
                .to_string(),
        )
    })?;
    let max_formulas_applied = max_formulas
        .unwrap_or(DEFAULT_MAX_FORMULAS)
        .min(MAX_FORMULAS);

    let dependency_plan =
        build_dependency_plan(path, display_path, &sheet_name, sheet, max_formulas_applied)?;

    Ok(ScanSheetFormulasDependencyResult {
        path: display_path.display().to_string(),
        sheet: SheetPreview {
            name: sheet_name,
            sheet_id: selected_sheet.sheet_id,
            part_path: sheet_part_path,
        },
        max_formulas_applied,
        nodes: dependency_plan
            .nodes
            .into_iter()
            .map(|node| FormulaDependencySummary {
                cell: node.summary.reference,
                formula: node.summary.formula,
                dependencies: node.dependencies,
                has_cycle: node.has_cycle,
                is_supported: node.is_supported,
                unsupported_reason: node.unsupported_reason,
            })
            .collect(),
        cycles_detected: dependency_plan.cycles_detected,
        truncated: dependency_plan.truncated,
        warnings: dependency_plan.warnings,
    })
}

pub(crate) fn generate_slider_query_package_with_display_path(
    path: &Path,
    display_path: &Path,
    sheet: &SheetSelector,
    output_package_path: &Path,
    output_package_display_path: &Path,
    max_formulas: Option<usize>,
) -> Result<GenerateSliderQueryPackageResult, ExcelInspectionError> {
    let workbook = inspect_workbook_with_display_path(path, display_path)?;
    if !matches!(workbook.format, WorkbookFormat::Xlsx | WorkbookFormat::Xlsm) {
        return Err(ExcelInspectionError::Message(
            "excel.generate_slider_query_package supports only .xlsx and .xlsm in this stage"
                .to_string(),
        ));
    }

    let selected_sheet = crate::preview::select_sheet(&workbook.sheets, sheet)?;
    let sheet_name = selected_sheet.name.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.generate_slider_query_package could not resolve a sheet name for the selected sheet"
                .to_string(),
        )
    })?;
    let sheet_part_path = selected_sheet.part_path.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.generate_slider_query_package could not resolve a worksheet part path for the selected sheet"
                .to_string(),
        )
    })?;
    let max_formulas_applied = max_formulas
        .unwrap_or(DEFAULT_MAX_FORMULAS)
        .min(MAX_FORMULAS);

    let dependency_plan =
        build_dependency_plan(path, display_path, &sheet_name, sheet, max_formulas_applied)?;

    let manifest = build_slider_query_package(
        &sheet_name,
        &sheet_part_path,
        &dependency_plan,
        output_package_path,
        output_package_display_path,
    )?;

    Ok(GenerateSliderQueryPackageResult {
        path: display_path.display().to_string(),
        sheet: SheetPreview {
            name: sheet_name,
            sheet_id: selected_sheet.sheet_id,
            part_path: sheet_part_path,
        },
        package_path: output_package_display_path.display().to_string(),
        manifest_path: output_package_display_path
            .join("manifest.json")
            .display()
            .to_string(),
        manifest,
        warnings: dependency_plan.warnings,
    })
}

fn build_dependency_plan(
    workbook_path: &Path,
    display_path: &Path,
    sheet_name: &str,
    sheet: &SheetSelector,
    max_formulas_applied: usize,
) -> Result<DependencyPlan, ExcelInspectionError> {
    let result = inspect_sheet_formulas_with_display_path(
        workbook_path,
        display_path,
        sheet,
        Some(max_formulas_applied),
    )?;
    let formula_refs = result
        .formulas
        .iter()
        .map(|formula| normalize_a1_reference(formula.reference.as_str()))
        .collect::<BTreeSet<_>>();
    let scan_index = result
        .formulas
        .iter()
        .enumerate()
        .map(|(index, formula)| (normalize_a1_reference(formula.reference.as_str()), index))
        .collect::<BTreeMap<_, _>>();

    let mut nodes = result
        .formulas
        .into_iter()
        .map(|formula| analyze_formula(formula, sheet_name, &formula_refs))
        .collect::<Vec<_>>();

    let cycles_detected = detect_cycles(&nodes, &scan_index);
    let cycle_members = cycles_detected
        .iter()
        .flat_map(|cycle| cycle.iter().cloned())
        .collect::<BTreeSet<_>>();
    for node in &mut nodes {
        if cycle_members.contains(&node.summary.reference) {
            node.has_cycle = true;
        }
    }

    let mut blocked = BTreeMap::new();
    for node in &nodes {
        if let Some(reason) = node.package_reason.clone() {
            blocked.insert(node.summary.reference.clone(), reason);
        }
    }
    for cycle in &cycles_detected {
        for cell in cycle {
            blocked
                .entry(cell.clone())
                .or_insert_with(|| "circular_dependency".to_string());
        }
    }
    let mut blocked_by = BTreeMap::<String, Vec<String>>::new();
    let mut changed = true;
    while changed {
        changed = false;
        for node in &nodes {
            if blocked.contains_key(&node.summary.reference) {
                continue;
            }
            if let Some(blocked_dependency) = node
                .formula_dependencies
                .iter()
                .find(|dependency| blocked.contains_key(*dependency))
            {
                blocked.insert(
                    node.summary.reference.clone(),
                    "blocked_dependency".to_string(),
                );
                blocked_by
                    .entry(node.summary.reference.clone())
                    .or_default()
                    .push(blocked_dependency.clone());
                changed = true;
            }
        }
    }

    let blocked_cells = blocked.keys().cloned().collect::<BTreeSet<_>>();
    let topo_order = topological_order(&nodes, &blocked_cells, &scan_index);
    let mut expression_map = BTreeMap::<String, String>::new();
    let mut prepared_columns = Vec::new();
    for reference in topo_order {
        let Some(node_index) = nodes
            .iter()
            .position(|node| normalize_a1_reference(node.summary.reference.as_str()) == reference)
        else {
            continue;
        };
        let summary = nodes[node_index].summary.clone();
        let expression = emit_slider_query_expression(
            &summary,
            sheet_name,
            &expression_map,
            &nodes,
            &blocked_cells,
        )?;
        let sql_alias = cell_to_sql_alias(summary.reference.as_str());
        expression_map.insert(
            normalize_a1_reference(summary.reference.as_str()),
            expression.clone(),
        );
        let node = &mut nodes[node_index];
        node.expression = Some(expression.clone());
        prepared_columns.push(SliderQueryVariableColumn {
            cell: summary.reference,
            sql_alias,
            expression,
            dependencies: node.dependencies.clone(),
        });
    }

    for node in &mut nodes {
        if let Some(reason) = blocked.get(&node.summary.reference) {
            node.package_reason = Some(reason.clone());
            node.blocked_by = blocked_by
                .get(&node.summary.reference)
                .cloned()
                .unwrap_or_default();
        }
    }

    Ok(DependencyPlan {
        sheet_slug: sheet_slug(sheet_name),
        nodes,
        prepared_columns,
        cycles_detected,
        truncated: result.truncated,
        warnings: result.warnings,
    })
}

fn analyze_formula(
    formula: SheetFormulaSummary,
    current_sheet_name: &str,
    formula_refs: &BTreeSet<String>,
) -> DependencyAnalysis {
    let mut dependencies = Vec::new();
    let mut formula_dependencies = Vec::new();
    let mut blockers = BTreeSet::new();
    let mut package_reason = None;

    match formula.parse.state {
        FormulaAstParseState::Missing => {
            package_reason = Some("formula_text_missing".to_string());
            blockers.insert("formula_text_missing".to_string());
        }
        FormulaAstParseState::Malformed => {
            package_reason = Some("formula_parse_malformed".to_string());
            blockers.insert("formula_parse_malformed".to_string());
        }
        FormulaAstParseState::Unsupported => {
            if let Some(reason) = formula.parse.unsupported_reasons.first() {
                let mapped = map_unsupported_reason(reason, &formula.formula);
                package_reason = Some(mapped.clone());
                blockers.insert(mapped);
                if should_use_lexical_fallback(reason) {
                    dependencies.extend(scan_lexical_dependencies(
                        &formula.formula,
                        current_sheet_name,
                    ));
                }
            }
        }
        FormulaAstParseState::Parsed => {
            if let Some(root) = formula.parse.root.as_ref() {
                collect_ast_dependencies(
                    root,
                    current_sheet_name,
                    formula_refs,
                    &mut dependencies,
                    &mut formula_dependencies,
                    &mut blockers,
                );
                if package_reason.is_none() {
                    package_reason = evaluate_slider_query_support(
                        root,
                        current_sheet_name,
                        formula.reference.as_str(),
                    );
                }
            }
        }
    }

    if formula.parse.state == FormulaAstParseState::Parsed {
        dependencies.extend(formula_dependencies.iter().cloned());
    }
    dependencies = dedup_preserve_order(dependencies);
    formula_dependencies = formula_dependencies
        .into_iter()
        .map(|reference| normalize_a1_reference(reference.as_str()))
        .filter(|reference| formula_refs.contains(reference))
        .collect::<Vec<_>>();
    formula_dependencies = dedup_preserve_order(formula_dependencies);

    let unsupported_reason = if formula.parse.state == FormulaAstParseState::Parsed {
        None
    } else if let Some(reason) = formula.parse.unsupported_reasons.first() {
        Some(map_unsupported_reason(reason, &formula.formula))
    } else {
        package_reason.clone()
    };

    let package_reason = package_reason.or_else(|| {
        if formula.parse.state == FormulaAstParseState::Parsed {
            None
        } else {
            Some("formula_parse_malformed".to_string())
        }
    });

    DependencyAnalysis {
        summary: formula.clone(),
        dependencies,
        formula_dependencies,
        is_supported: formula.parse.state == FormulaAstParseState::Parsed,
        unsupported_reason,
        has_cycle: false,
        blocked_by: Vec::new(),
        package_reason,
        expression: None,
    }
}

fn evaluate_slider_query_support(
    node: &FormulaAstNode,
    current_sheet_name: &str,
    formula_reference: &str,
) -> Option<String> {
    match node {
        FormulaAstNode::NumberLiteral { .. }
        | FormulaAstNode::StringLiteral { .. }
        | FormulaAstNode::BooleanLiteral { .. }
        | FormulaAstNode::BlankArgument => None,
        FormulaAstNode::ErrorLiteral { .. } => {
            Some("error_literal_not_supported_in_phase_2".to_string())
        }
        FormulaAstNode::UnaryOperation { operand, .. } | FormulaAstNode::Percent { operand } => {
            evaluate_slider_query_support(operand, current_sheet_name, formula_reference)
        }
        FormulaAstNode::BinaryOperation { left, right, .. } => {
            evaluate_slider_query_support(left, current_sheet_name, formula_reference).or_else(
                || evaluate_slider_query_support(right, current_sheet_name, formula_reference),
            )
        }
        FormulaAstNode::CellReference {
            reference,
            sheet_name,
        } => {
            if let Some(sheet_name) = sheet_name
                && !sheet_name.eq_ignore_ascii_case(current_sheet_name)
            {
                return Some("cross_sheet_reference_not_supported_in_phase_2".to_string());
            }
            let Some((_, row)) = parse_a1_reference(reference.as_str()) else {
                return Some("cell_reference_not_a1".to_string());
            };
            let Some((_, formula_row)) = parse_a1_reference(formula_reference) else {
                return Some("cell_reference_not_a1".to_string());
            };
            if row != formula_row {
                return Some("cross_row_reference_not_supported_in_phase_2".to_string());
            }
            None
        }
        FormulaAstNode::FunctionCall { .. } => {
            Some("function_call_not_supported_in_phase_2".to_string())
        }
        FormulaAstNode::RangeReference { .. } => {
            Some("range_reference_not_supported_in_phase_2".to_string())
        }
        FormulaAstNode::DefinedNameReference { .. } => {
            Some("defined_name_reference_not_supported_in_phase_2".to_string())
        }
        FormulaAstNode::Unsupported { reason, text } => Some(map_unsupported_reason(reason, text)),
    }
}

fn collect_ast_dependencies(
    node: &FormulaAstNode,
    current_sheet_name: &str,
    formula_refs: &BTreeSet<String>,
    dependencies: &mut Vec<String>,
    formula_dependencies: &mut Vec<String>,
    blockers: &mut BTreeSet<String>,
) {
    match node {
        FormulaAstNode::UnaryOperation { operand, .. } | FormulaAstNode::Percent { operand } => {
            collect_ast_dependencies(
                operand,
                current_sheet_name,
                formula_refs,
                dependencies,
                formula_dependencies,
                blockers,
            );
        }
        FormulaAstNode::BinaryOperation { left, right, .. } => {
            collect_ast_dependencies(
                left,
                current_sheet_name,
                formula_refs,
                dependencies,
                formula_dependencies,
                blockers,
            );
            collect_ast_dependencies(
                right,
                current_sheet_name,
                formula_refs,
                dependencies,
                formula_dependencies,
                blockers,
            );
        }
        FormulaAstNode::FunctionCall { args, .. } => {
            for arg in args {
                collect_ast_dependencies(
                    arg,
                    current_sheet_name,
                    formula_refs,
                    dependencies,
                    formula_dependencies,
                    blockers,
                );
            }
        }
        FormulaAstNode::CellReference {
            reference,
            sheet_name,
        } => {
            if let Some(sheet_name) = sheet_name
                && !sheet_name.eq_ignore_ascii_case(current_sheet_name)
            {
                blockers.insert("cross_sheet_reference_not_supported_in_phase_2".to_string());
                return;
            }
            let normalized = normalize_a1_reference(reference);
            dependencies.push(normalized.clone());
            if formula_refs.contains(&normalized) {
                formula_dependencies.push(normalized);
            }
        }
        FormulaAstNode::RangeReference {
            start_reference,
            end_reference,
            sheet_name,
        } => {
            if let Some(sheet_name) = sheet_name
                && !sheet_name.eq_ignore_ascii_case(current_sheet_name)
            {
                blockers.insert("cross_sheet_reference_not_supported_in_phase_2".to_string());
                return;
            }
            let normalized_start = normalize_a1_reference(start_reference);
            let normalized_end = normalize_a1_reference(end_reference);
            dependencies.push(normalized_start.clone());
            dependencies.push(normalized_end.clone());
            if formula_refs.contains(&normalized_start) {
                formula_dependencies.push(normalized_start);
            }
            if formula_refs.contains(&normalized_end) {
                formula_dependencies.push(normalized_end);
            }
        }
        FormulaAstNode::DefinedNameReference { .. } => {
            blockers.insert("defined_name_reference_not_supported_in_phase_2".to_string());
        }
        FormulaAstNode::Unsupported { reason, text } => {
            blockers.insert(map_unsupported_reason(reason, text));
        }
        FormulaAstNode::NumberLiteral { .. }
        | FormulaAstNode::StringLiteral { .. }
        | FormulaAstNode::BooleanLiteral { .. }
        | FormulaAstNode::BlankArgument
        | FormulaAstNode::ErrorLiteral { .. } => {}
    }
}

fn detect_cycles(
    nodes: &[DependencyAnalysis],
    scan_index: &BTreeMap<String, usize>,
) -> Vec<Vec<String>> {
    let mut adjacency = BTreeMap::<String, Vec<String>>::new();
    for node in nodes {
        adjacency.insert(
            node.summary.reference.clone(),
            node.formula_dependencies.clone(),
        );
    }
    let mut visited = BTreeSet::<String>::new();
    let mut stack = Vec::<String>::new();
    let mut active = BTreeSet::<String>::new();
    let mut cycles = Vec::<Vec<String>>::new();

    fn dfs(
        reference: &str,
        adjacency: &BTreeMap<String, Vec<String>>,
        visited: &mut BTreeSet<String>,
        stack: &mut Vec<String>,
        active: &mut BTreeSet<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        if !visited.insert(reference.to_string()) {
            return;
        }
        stack.push(reference.to_string());
        active.insert(reference.to_string());
        if let Some(children) = adjacency.get(reference) {
            for child in children {
                if !visited.contains(child) {
                    dfs(child, adjacency, visited, stack, active, cycles);
                } else if active.contains(child)
                    && let Some(start) = stack.iter().position(|value| value == child)
                {
                    cycles.push(stack[start..].to_vec());
                }
            }
        }
        stack.pop();
        active.remove(reference);
    }

    let mut roots = adjacency.keys().cloned().collect::<Vec<_>>();
    roots.sort_by_key(|reference| scan_index.get(reference).copied().unwrap_or(usize::MAX));
    for root in roots {
        if !visited.contains(&root) {
            dfs(
                &root,
                &adjacency,
                &mut visited,
                &mut stack,
                &mut active,
                &mut cycles,
            );
        }
    }
    normalize_cycles(cycles)
}

fn normalize_cycles(cycles: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut seen = BTreeSet::<String>::new();
    let mut normalized = Vec::<Vec<String>>::new();
    for mut cycle in cycles {
        cycle = dedup_preserve_order(cycle);
        if cycle.is_empty() {
            continue;
        }
        let key = cycle.join("|");
        if seen.insert(key) {
            normalized.push(cycle);
        }
    }
    normalized
}

fn topological_order(
    nodes: &[DependencyAnalysis],
    blocked_cells: &BTreeSet<String>,
    scan_index: &BTreeMap<String, usize>,
) -> Vec<String> {
    let mut indegree = BTreeMap::<String, usize>::new();
    let mut adjacency = BTreeMap::<String, Vec<String>>::new();
    for node in nodes {
        if blocked_cells.contains(&node.summary.reference) {
            continue;
        }
        indegree.entry(node.summary.reference.clone()).or_insert(0);
        for dependency in &node.formula_dependencies {
            if blocked_cells.contains(dependency) {
                continue;
            }
            adjacency
                .entry(dependency.clone())
                .or_default()
                .push(node.summary.reference.clone());
            *indegree.entry(node.summary.reference.clone()).or_insert(0) += 1;
        }
    }
    let mut ready = indegree
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(reference, _)| reference.clone())
        .collect::<Vec<_>>();
    ready.sort_by_key(|reference| scan_index.get(reference).copied().unwrap_or(usize::MAX));
    let mut order = Vec::<String>::new();
    let mut ready = ready.into_iter().collect::<BTreeSet<_>>();

    while let Some(reference) = ready
        .iter()
        .min_by_key(|candidate| scan_index.get(*candidate).copied().unwrap_or(usize::MAX))
        .cloned()
    {
        ready.remove(&reference);
        order.push(reference.clone());
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
    order
}

fn emit_slider_query_expression(
    formula: &SheetFormulaSummary,
    current_sheet_name: &str,
    computed_expressions: &BTreeMap<String, String>,
    nodes: &[DependencyAnalysis],
    blocked_cells: &BTreeSet<String>,
) -> Result<String, ExcelInspectionError> {
    let Some(root) = formula.parse.root.as_ref() else {
        return Err(ExcelInspectionError::Message(
            "formula text missing from SliderQuery phase-2 planner".to_string(),
        ));
    };
    let formula_row = parse_a1_reference(formula.reference.as_str()).map(|(_, row)| row);
    emit_slider_expression_node(
        root,
        current_sheet_name,
        formula.reference.as_str(),
        formula_row,
        computed_expressions,
        nodes,
        blocked_cells,
    )
    .ok_or_else(|| {
        ExcelInspectionError::Message(format!(
            "excel.generate_slider_query_package could not emit SQL for {}",
            formula.reference
        ))
    })
}

fn emit_slider_expression_node(
    node: &FormulaAstNode,
    current_sheet_name: &str,
    formula_reference: &str,
    formula_row: Option<u32>,
    computed_expressions: &BTreeMap<String, String>,
    nodes: &[DependencyAnalysis],
    blocked_cells: &BTreeSet<String>,
) -> Option<String> {
    match node {
        FormulaAstNode::NumberLiteral { value } => Some(value.clone()),
        FormulaAstNode::StringLiteral { value } => Some(format!("'{}'", value.replace('\'', "''"))),
        FormulaAstNode::BooleanLiteral { value } => {
            Some(if *value { "TRUE" } else { "FALSE" }.to_string())
        }
        FormulaAstNode::BlankArgument => Some("NULL".to_string()),
        FormulaAstNode::ErrorLiteral { .. } => None,
        FormulaAstNode::UnaryOperation { operator, operand } => emit_slider_expression_node(
            operand,
            current_sheet_name,
            formula_reference,
            formula_row,
            computed_expressions,
            nodes,
            blocked_cells,
        )
        .map(|operand| match operator {
            FormulaAstUnaryOperator::Plus => format!("(+{operand})"),
            FormulaAstUnaryOperator::Minus => format!("(-{operand})"),
        }),
        FormulaAstNode::BinaryOperation {
            operator,
            left,
            right,
        } => {
            let left = emit_slider_expression_node(
                left,
                current_sheet_name,
                formula_reference,
                formula_row,
                computed_expressions,
                nodes,
                blocked_cells,
            )?;
            let right = emit_slider_expression_node(
                right,
                current_sheet_name,
                formula_reference,
                formula_row,
                computed_expressions,
                nodes,
                blocked_cells,
            )?;
            Some(match operator {
                FormulaAstBinaryOperator::Add => format!("({left} + {right})"),
                FormulaAstBinaryOperator::Subtract => format!("({left} - {right})"),
                FormulaAstBinaryOperator::Multiply => format!("({left} * {right})"),
                FormulaAstBinaryOperator::Divide => format!("({left} / {right})"),
                FormulaAstBinaryOperator::Power => format!("POWER({left}, {right})"),
                FormulaAstBinaryOperator::Concatenate => format!("CONCAT({left}, {right})"),
                FormulaAstBinaryOperator::Equal => format!("({left} = {right})"),
                FormulaAstBinaryOperator::NotEqual => format!("({left} <> {right})"),
                FormulaAstBinaryOperator::LessThan => format!("({left} < {right})"),
                FormulaAstBinaryOperator::LessThanOrEqual => format!("({left} <= {right})"),
                FormulaAstBinaryOperator::GreaterThan => format!("({left} > {right})"),
                FormulaAstBinaryOperator::GreaterThanOrEqual => format!("({left} >= {right})"),
            })
        }
        FormulaAstNode::Percent { operand } => emit_slider_expression_node(
            operand,
            current_sheet_name,
            formula_reference,
            formula_row,
            computed_expressions,
            nodes,
            blocked_cells,
        )
        .map(|operand| format!("({operand} / 100.0)")),
        FormulaAstNode::FunctionCall { .. }
        | FormulaAstNode::RangeReference { .. }
        | FormulaAstNode::DefinedNameReference { .. }
        | FormulaAstNode::Unsupported { .. } => None,
        FormulaAstNode::CellReference {
            reference,
            sheet_name,
        } => {
            if let Some(sheet_name) = sheet_name
                && !sheet_name.eq_ignore_ascii_case(current_sheet_name)
            {
                return None;
            }
            let (_column, row) = parse_a1_reference(reference.as_str())?;
            if let Some(formula_row) = formula_row
                && row != formula_row
            {
                return None;
            }
            let normalized = normalize_a1_reference(reference);
            if blocked_cells.contains(&normalized) {
                return None;
            }
            if let Some(expression) = computed_expressions.get(&normalized) {
                return Some(expression.clone());
            }
            let is_formula_cell = nodes
                .iter()
                .any(|node| normalize_a1_reference(node.summary.reference.as_str()) == normalized);
            if is_formula_cell {
                return None;
            }
            Some(cell_to_sql_alias(reference.as_str()))
        }
    }
}

fn build_slider_query_package(
    sheet_name: &str,
    sheet_part_path: &str,
    plan: &DependencyPlan,
    output_package_path: &Path,
    output_package_display_path: &Path,
) -> Result<SliderQueryPackageManifest, ExcelInspectionError> {
    std::fs::create_dir_all(output_package_path).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to create slider query package directory {}: {err}",
            output_package_path.display()
        ))
    })?;
    let queries_dir = output_package_path.join("queries");
    let variables_dir = output_package_path.join("variables");
    std::fs::create_dir_all(&queries_dir).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to create slider query package queries directory {}: {err}",
            queries_dir.display()
        ))
    })?;
    std::fs::create_dir_all(&variables_dir).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to create slider query package variables directory {}: {err}",
            variables_dir.display()
        ))
    })?;

    let package_name = format!("{}_slider_package", plan.sheet_slug);
    let mut generated_queries = Vec::new();
    let blocked_formulas = plan
        .nodes
        .iter()
        .filter_map(|node| {
            node.package_reason
                .as_ref()
                .map(|reason| SliderQueryBlockedFormulaSummary {
                    cell: node.summary.reference.clone(),
                    formula: node.summary.formula.clone(),
                    reason: reason.clone(),
                    blocked_by: node.blocked_by.clone(),
                    dependencies: node.dependencies.clone(),
                })
        })
        .collect::<Vec<_>>();

    if !plan.prepared_columns.is_empty() {
        let source_columns = collect_source_columns(&plan.prepared_columns);
        let query_name = format!("{}_prepared", plan.sheet_slug);
        let sql_path = queries_dir.join(format!("{query_name}.sql"));
        let blocked_path = queries_dir.join(format!("{}_blocked.json", plan.sheet_slug));
        let variable_path = variables_dir.join(format!("{query_name}.json"));

        let sql = render_prepared_sql(
            sheet_name,
            &plan.sheet_slug,
            &source_columns,
            &plan.prepared_columns,
        );
        std::fs::write(&sql_path, sql.as_bytes()).map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to write slider query SQL {}: {err}",
                sql_path.display()
            ))
        })?;

        let variable_file = SliderQueryVariableFile {
            sheet_name: sheet_name.to_string(),
            source_table: format!("raw.\"{}\"", plan.sheet_slug),
            source_columns,
            prepared_columns: plan.prepared_columns.clone(),
        };
        std::fs::write(
            &variable_path,
            serde_json::to_string_pretty(&variable_file)
                .map_err(|err| {
                    ExcelInspectionError::Message(format!(
                        "failed to serialize slider query variable file: {err}"
                    ))
                })?
                .as_bytes(),
        )
        .map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to write slider query variable file {}: {err}",
                variable_path.display()
            ))
        })?;

        let blocked_json = serde_json::to_string_pretty(&blocked_formulas).map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to serialize slider query blocked formulas: {err}"
            ))
        })?;
        std::fs::write(&blocked_path, blocked_json.as_bytes()).map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to write slider query blocked formulas {}: {err}",
                blocked_path.display()
            ))
        })?;

        generated_queries.push(SliderQueryGeneratedQuerySummary {
            name: query_name,
            r#type: SliderQueryGeneratedQueryType::PreparedColumns,
            sql_path: relative_path(output_package_path, &sql_path),
            variable_path: relative_path(output_package_path, &variable_path),
            blocked_path: relative_path(output_package_path, &blocked_path),
        });
    } else {
        let blocked_path = queries_dir.join(format!("{}_blocked.json", plan.sheet_slug));
        let blocked_json = serde_json::to_string_pretty(&blocked_formulas).map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to serialize slider query blocked formulas: {err}"
            ))
        })?;
        std::fs::write(&blocked_path, blocked_json.as_bytes()).map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to write slider query blocked formulas {}: {err}",
                blocked_path.display()
            ))
        })?;
    }

    let manifest = SliderQueryPackageManifest {
        package_name,
        generated_queries,
        blocked_formulas,
    };
    let manifest_path = output_package_path.join("manifest.json");
    std::fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest)
            .map_err(|err| {
                ExcelInspectionError::Message(format!(
                    "failed to serialize slider query manifest: {err}"
                ))
            })?
            .as_bytes(),
    )
    .map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to write slider query manifest {}: {err}",
            manifest_path.display()
        ))
    })?;

    let _ = output_package_display_path;
    let _ = sheet_part_path;
    Ok(manifest)
}

fn render_prepared_sql(
    sheet_name: &str,
    sheet_slug: &str,
    source_columns: &[String],
    prepared_columns: &[SliderQueryVariableColumn],
) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "-- Source: {sheet_name}, cells {}",
        prepared_columns
            .iter()
            .map(|column| column.cell.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    ));
    if !source_columns.is_empty() {
        lines.push(format!("-- Inputs: {}", source_columns.join(", ")));
    }
    lines.push("-- Confidence: high".to_string());
    lines.push(String::new());
    lines.push("WITH base_source AS (".to_string());
    lines.push("    SELECT *".to_string());
    lines.push(format!("    FROM raw.\"{sheet_slug}\""));
    lines.push(")".to_string());
    lines.push("SELECT".to_string());
    lines.push("    *,".to_string());
    for (index, column) in prepared_columns.iter().enumerate() {
        let suffix = if index + 1 == prepared_columns.len() {
            String::new()
        } else {
            ",".to_string()
        };
        lines.push(format!(
            "    {} AS {}{}",
            column.expression, column.sql_alias, suffix
        ));
    }
    lines.push("FROM base_source;".to_string());
    lines.join("\n")
}

fn collect_source_columns(prepared_columns: &[SliderQueryVariableColumn]) -> Vec<String> {
    let mut source_columns = BTreeSet::<String>::new();
    let formula_refs = prepared_columns
        .iter()
        .map(|node| node.cell.clone())
        .collect::<BTreeSet<_>>();
    for node in prepared_columns {
        for dependency in &node.dependencies {
            if formula_refs.contains(dependency) {
                continue;
            }
            if let Some((column, _)) = parse_a1_reference(dependency) {
                source_columns.insert(column);
            }
        }
    }
    source_columns
        .into_iter()
        .map(|column| column.to_ascii_uppercase())
        .collect()
}

fn dedup_preserve_order(values: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::<String>::new();
    values
        .into_iter()
        .filter(|value| seen.insert(value.clone()))
        .collect()
}

fn should_use_lexical_fallback(reason: &str) -> bool {
    matches!(
        reason,
        "volatile_function" | "dynamic_array_or_spill_marker"
    )
}

fn scan_lexical_dependencies(formula: &str, _current_sheet_name: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    let mut chars = formula.chars().collect::<Vec<_>>();
    strip_quoted_strings(&mut chars);
    let text = chars.into_iter().collect::<String>();
    for candidate in text.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '$' || ch == '!'))
    {
        if candidate.is_empty() {
            continue;
        }
        if let Some((_, reference)) = candidate.rsplit_once('!') {
            if let Some(reference) = normalize_reference_candidate(reference) {
                dependencies.push(reference);
            }
            continue;
        }
        if let Some(reference) = normalize_reference_candidate(candidate) {
            dependencies.push(reference);
        }
    }
    dependencies
        .into_iter()
        .filter(|dependency| !dependency.is_empty())
        .collect::<Vec<_>>()
}

fn strip_quoted_strings(chars: &mut [char]) {
    let mut index = 0usize;
    let len = chars.len();
    while index < len {
        if chars[index] == '"' {
            index += 1;
            while index < len {
                if chars[index] == '"' {
                    if chars.get(index + 1) == Some(&'"') {
                        index += 2;
                        continue;
                    }
                    index += 1;
                    break;
                }
                chars[index] = ' ';
                index += 1;
            }
            continue;
        }
        index += 1;
    }
}

fn normalize_reference_candidate(candidate: &str) -> Option<String> {
    let trimmed = candidate.trim_matches('$');
    if parse_a1_reference(trimmed).is_some() {
        Some(normalize_a1_reference(trimmed))
    } else {
        None
    }
}

fn map_unsupported_reason(reason: &str, text: &str) -> String {
    match reason {
        "volatile_function" => volatile_function_reason(text),
        other => other.to_string(),
    }
}

fn volatile_function_reason(text: &str) -> String {
    let function_name = text
        .split_once('(')
        .map(|(name, _)| name)
        .unwrap_or(text)
        .trim();
    if function_name.is_empty() {
        return "volatile_function".to_string();
    }
    format!(
        "volatile_function_{}",
        function_name
            .to_ascii_uppercase()
            .trim_start_matches("_XLFN.")
            .trim_start_matches("_XLWS.")
            .to_ascii_lowercase()
    )
}

fn parse_a1_reference(reference: &str) -> Option<(String, u32)> {
    let trimmed = reference.trim();
    let mut column = String::new();
    let mut row = String::new();
    for ch in trimmed.chars() {
        if ch == '$' {
            continue;
        }
        if ch.is_ascii_alphabetic() && row.is_empty() {
            column.push(ch.to_ascii_uppercase());
            continue;
        }
        if ch.is_ascii_digit() && !column.is_empty() {
            row.push(ch);
            continue;
        }
        return None;
    }
    if column.is_empty() || row.is_empty() {
        return None;
    }
    Some((column, row.parse().ok()?))
}

fn normalize_a1_reference(reference: &str) -> String {
    reference.trim().trim_start_matches('$').replace('$', "")
}

fn cell_to_sql_alias(reference: &str) -> String {
    let (column, _) = parse_a1_reference(reference).unwrap_or_else(|| ("A".to_string(), 1));
    format!("col_{}", column.to_ascii_lowercase())
}

fn sheet_slug(sheet_name: &str) -> String {
    let slug = sheet_name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string();
    if slug.is_empty() {
        "sheet".to_string()
    } else {
        slug
    }
}

fn relative_path(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf())
        .display()
        .to_string()
}

fn workbook_path_from_model_arg(path: &str, cwd: &Path) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "excel.scan_sheet_formulas_dependency path must not be empty".to_string(),
        ));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(
            "excel.scan_sheet_formulas_dependency path must be a local workbook path".to_string(),
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
            "excel.scan_sheet_formulas_dependency path must be relative and stay within the current working directory"
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
                "excel.scan_sheet_formulas_dependency path must not traverse symlinks".to_string(),
            ));
        }
    }

    Ok(resolved_path)
}

fn package_output_path_from_model_arg(
    path: &str,
    cwd: &Path,
) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_slider_query_package output_package_path must not be empty".to_string(),
        ));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(
            "excel.generate_slider_query_package output_package_path must be a local path"
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
            "excel.generate_slider_query_package output_package_path must be relative and stay within the current working directory"
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
                "excel.generate_slider_query_package output_package_path must not traverse symlinks"
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
