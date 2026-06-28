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
use serde::de::DeserializeOwned;
use serde_json::to_value;

use crate::backend::ExcelInspectionError;
use crate::backend::inspect_workbook_with_display_path;
use crate::formula_ast::FormulaAstNode;
use crate::formula_ast::parse_formula_ast;
use crate::formula_inspect::inspect_sheet_formulas_with_display_path;
use crate::powerquery_extract::ExtractedPowerQueryQuery;
use crate::powerquery_extract::PowerQueryLexicalReferenceKind;
use crate::powerquery_extract::extract_powerquery_queries_from_workbook;
use crate::tool::DefinedNameSummary;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::SheetKind;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;
use crate::workbook_tables::ParsedWorkbookTable;
use crate::workbook_tables::parse_workbook_tables;

pub(crate) const INSPECT_WORKBOOK_GRAPH_TOOL_NAME: &str = "inspect_workbook_graph";

const INSPECT_WORKBOOK_GRAPH_DESCRIPTION: &str = "Inspect a bounded partial workbook graph preview: workbook structure, selected-sheet formula membership, and AST-backed formula dependency edges where proven.";
#[derive(Clone, Default)]
pub(crate) struct ExcelInspectWorkbookGraphTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectWorkbookGraphArgs {
    pub path: String,
    pub sheet: SheetSelector,
    #[serde(default)]
    pub max_formulas: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WorkbookGraphMode {
    PackageStructurePlusPerSheetFormulaMembership,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WorkbookGraphFormulaInventoryScope {
    SelectedSheetOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WorkbookGraphNodeKind {
    Workbook,
    Worksheet,
    CellFormula,
    PowerQueryQuery,
    DefinedName,
    DefinedNameFormulaText,
    Table,
    ReferencedCell,
    ReferencedRange,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WorkbookGraphEdgeKind {
    WorkbookContainsWorksheet,
    WorksheetContainsFormula,
    PowerQueryReferencesQuery,
    PowerQueryReferencesDefinedName,
    PowerQueryReferencesTable,
    FormulaReferencesDefinedName,
    FormulaReferencesTable,
    FormulaReferencesWorksheet,
    FormulaReferencesCell,
    FormulaReferencesRange,
    DefinedNameTargetsCell,
    DefinedNameTargetsRange,
    DefinedNameTargetsFormulaText,
    TableHasRange,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WorkbookGraphEvidenceKind {
    WorkbookSheetEntry,
    WorkbookRelationship,
    WorksheetFormulaCell,
    PowerQueryLexicalQueryReference,
    PowerQueryLexicalWorkbookNameReference,
    FormulaAstDefinedNameReference,
    FormulaAstStructuredReference,
    FormulaAstWorksheetReference,
    FormulaAstCellReference,
    FormulaAstRangeReference,
    WorkbookDefinedNameTarget,
    WorksheetTableRelationship,
    TableXmlRange,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct WorkbookGraphNode {
    pub id: String,
    pub kind: WorkbookGraphNodeKind,
    pub label: String,
    pub sheet_name: Option<String>,
    pub part_path: Option<String>,
    pub cell_reference: Option<String>,
    pub formula: Option<String>,
    pub cached_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct WorkbookGraphEvidence {
    pub kind: WorkbookGraphEvidenceKind,
    pub part_path: String,
    pub cell_reference: Option<String>,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct WorkbookGraphEdge {
    pub kind: WorkbookGraphEdgeKind,
    pub from: String,
    pub to: String,
    pub evidence: Vec<WorkbookGraphEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectWorkbookGraphResult {
    pub path: String,
    pub mode: WorkbookGraphMode,
    pub is_partial: bool,
    pub formula_inventory_scope: WorkbookGraphFormulaInventoryScope,
    pub formula_inventory_sheet: SheetPreview,
    pub max_formulas_applied: usize,
    pub formula_inventory_truncated: bool,
    pub nodes: Vec<WorkbookGraphNode>,
    pub edges: Vec<WorkbookGraphEdge>,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectWorkbookGraphTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_GRAPH_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema = serde_json::to_value(schemars::schema_for!(InspectWorkbookGraphArgs))
            .unwrap_or_else(|err| {
                panic!("inspect_workbook_graph args schema should serialize: {err}")
            });
        let output_schema = serde_json::to_value(schemars::schema_for!(InspectWorkbookGraphResult))
            .unwrap_or_else(|err| {
                panic!("inspect_workbook_graph result schema should serialize: {err}")
            });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_WORKBOOK_GRAPH_TOOL_NAME.to_string(),
                    description: INSPECT_WORKBOOK_GRAPH_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_workbook_graph args schema should parse: {err}")
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
        let args = parse_tool_args::<InspectWorkbookGraphArgs>(&call)?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_workbook_graph workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let path = workbook_path_from_graph_arg(&args.path, &cwd)?;
        let result = inspect_workbook_graph_with_display_path(
            &path,
            Path::new(args.path.trim()),
            &args.sheet,
            args.max_formulas,
        )
        .map_err(FunctionCallError::from)?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize workbook graph preview: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectWorkbookGraphTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn inspect_workbook_graph_with_display_path(
    workbook_path: &Path,
    workbook_display_path: &Path,
    sheet: &SheetSelector,
    max_formulas: Option<usize>,
) -> Result<InspectWorkbookGraphResult, ExcelInspectionError> {
    let workbook = inspect_workbook_with_display_path(workbook_path, workbook_display_path)?;
    let formulas = inspect_sheet_formulas_with_display_path(
        workbook_path,
        workbook_display_path,
        sheet,
        max_formulas,
    )?;

    let worksheet_summaries = workbook
        .sheets
        .iter()
        .enumerate()
        .filter(|(_, summary)| summary.kind == SheetKind::Worksheet)
        .collect::<Vec<_>>();
    let table_parse = parse_workbook_tables(workbook_path, &worksheet_summaries)?;
    let power_query_extract = workbook
        .markers
        .has_power_query
        .then(|| extract_powerquery_queries_from_workbook(workbook_path, workbook_display_path));

    let selected_worksheet_index = worksheet_summaries
        .iter()
        .position(|(_, summary)| {
            summary.part_path.as_deref() == Some(formulas.sheet.part_path.as_str())
        })
        .ok_or_else(|| {
            ExcelInspectionError::Message(
                "excel.inspect_workbook_graph could not match the selected worksheet back to workbook structure"
                    .to_string(),
            )
        })?;
    let selected_worksheet_node_id = worksheet_node_id(selected_worksheet_index);

    let mut nodes = vec![WorkbookGraphNode {
        id: "workbook".to_string(),
        kind: WorkbookGraphNodeKind::Workbook,
        label: workbook_display_path.display().to_string(),
        sheet_name: None,
        part_path: None,
        cell_reference: None,
        formula: None,
        cached_value: None,
    }];
    let mut node_ids = nodes
        .iter()
        .map(|node| node.id.clone())
        .collect::<BTreeSet<_>>();
    let mut edges = Vec::new();
    let mut edge_keys = BTreeSet::new();
    let mut warning_flags = GraphWarningFlags::default();
    let mut graph = GraphBuildState {
        nodes: &mut nodes,
        node_ids: &mut node_ids,
        edges: &mut edges,
        edge_keys: &mut edge_keys,
    };

    for (worksheet_index, (_, summary)) in worksheet_summaries.iter().enumerate() {
        let node_id = worksheet_node_id(worksheet_index);
        graph.nodes.push(WorkbookGraphNode {
            id: node_id.clone(),
            kind: WorkbookGraphNodeKind::Worksheet,
            label: worksheet_label(summary, worksheet_index),
            sheet_name: summary.name.clone(),
            part_path: summary.part_path.clone(),
            cell_reference: None,
            formula: None,
            cached_value: None,
        });
        graph.node_ids.insert(node_id.clone());

        let mut evidence = vec![WorkbookGraphEvidence {
            kind: WorkbookGraphEvidenceKind::WorkbookSheetEntry,
            part_path: "xl/workbook.xml".to_string(),
            cell_reference: None,
            detail: format!(
                "sheet entry name={} sheet_id={} relationship_id={}",
                summary.name.as_deref().unwrap_or("<unnamed>"),
                summary
                    .sheet_id
                    .map_or_else(|| "<missing>".to_string(), |value| value.to_string()),
                summary.relationship_id.as_deref().unwrap_or("<missing>"),
            ),
        }];
        if let Some(part_path) = &summary.part_path {
            evidence.push(WorkbookGraphEvidence {
                kind: WorkbookGraphEvidenceKind::WorkbookRelationship,
                part_path: "xl/_rels/workbook.xml.rels".to_string(),
                cell_reference: None,
                detail: format!("worksheet part target {part_path}"),
            });
        }
        push_edge(
            &mut graph,
            WorkbookGraphEdgeKind::WorkbookContainsWorksheet,
            "workbook".to_string(),
            node_id,
            evidence,
        );
    }

    for table in &table_parse.tables {
        let table_node_id = table_node_id(table);
        if graph.node_ids.insert(table_node_id.clone()) {
            graph.nodes.push(WorkbookGraphNode {
                id: table_node_id.clone(),
                kind: WorkbookGraphNodeKind::Table,
                label: format!("{}!{}", table.sheet_name, table.name),
                sheet_name: Some(table.sheet_name.clone()),
                part_path: Some(table.part_path.clone()),
                cell_reference: Some(table.range_reference.clone()),
                formula: None,
                cached_value: None,
            });
        }
        let range_node_id = range_reference_node_id(
            table.worksheet_index,
            &table.start_reference,
            &table.end_reference,
        );
        if graph.node_ids.insert(range_node_id.clone()) {
            graph.nodes.push(WorkbookGraphNode {
                id: range_node_id.clone(),
                kind: WorkbookGraphNodeKind::ReferencedRange,
                label: format!("{}!{}", table.sheet_name, table.range_reference),
                sheet_name: Some(table.sheet_name.clone()),
                part_path: Some(table.worksheet_part_path.clone()),
                cell_reference: Some(table.range_reference.clone()),
                formula: None,
                cached_value: None,
            });
        }
        push_edge(
            &mut graph,
            WorkbookGraphEdgeKind::TableHasRange,
            table_node_id,
            range_node_id,
            vec![
                WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::WorksheetTableRelationship,
                    part_path: table.relationship_part_path.clone(),
                    cell_reference: None,
                    detail: format!(
                        "worksheet {} table relationship {} -> {}",
                        table.sheet_name, table.relationship_id, table.part_path
                    ),
                },
                WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::TableXmlRange,
                    part_path: table.part_path.clone(),
                    cell_reference: None,
                    detail: format!("table {} range {}", table.name, table.range_reference),
                },
            ],
        );
    }

    let dependency_context = DependencyResolutionContext {
        selected_worksheet_index,
        worksheet_summaries: &worksheet_summaries,
        defined_names: &formulas.defined_names,
        selected_sheet_name: &formulas.sheet.name,
        tables: &table_parse.tables,
    };

    for formula in &formulas.formulas {
        let formula_node_id = formula_node_id(selected_worksheet_index, &formula.reference);
        graph.nodes.push(WorkbookGraphNode {
            id: formula_node_id.clone(),
            kind: WorkbookGraphNodeKind::CellFormula,
            label: format!("{}!{}", formulas.sheet.name, formula.reference),
            sheet_name: Some(formulas.sheet.name.clone()),
            part_path: Some(formulas.sheet.part_path.clone()),
            cell_reference: Some(formula.reference.clone()),
            formula: Some(formula.formula.clone()),
            cached_value: formula.cached_value.clone(),
        });
        graph.node_ids.insert(formula_node_id.clone());
        push_edge(
            &mut graph,
            WorkbookGraphEdgeKind::WorksheetContainsFormula,
            selected_worksheet_node_id.clone(),
            formula_node_id.clone(),
            vec![WorkbookGraphEvidence {
                kind: WorkbookGraphEvidenceKind::WorksheetFormulaCell,
                part_path: formulas.sheet.part_path.clone(),
                cell_reference: Some(formula.reference.clone()),
                detail: "selected-sheet formula inventory entry".to_string(),
            }],
        );
        emit_formula_dependency_edges(
            formula,
            &formula_node_id,
            &dependency_context,
            &mut graph,
            &mut warning_flags,
        );
    }

    if let Some(power_query_extract) = power_query_extract.as_ref() {
        emit_power_query_lineage_edges(
            &power_query_extract.queries,
            &formulas.defined_names,
            &table_parse.tables,
            &worksheet_summaries,
            &mut graph,
            &mut warning_flags,
        );
    }

    if workbook.markers.has_tables && table_parse.tables.is_empty() {
        warning_flags.unresolved_table_metadata = true;
    }
    if table_parse.had_unresolved_table_metadata {
        warning_flags.unresolved_table_metadata = true;
    }

    let power_query_warnings = power_query_extract
        .as_ref()
        .map(|result| result.warnings.as_slice())
        .unwrap_or_default();
    let warnings = build_graph_warnings(
        &workbook.warnings,
        power_query_warnings,
        &formulas,
        warning_flags,
    );

    Ok(InspectWorkbookGraphResult {
        path: workbook_display_path.display().to_string(),
        mode: WorkbookGraphMode::PackageStructurePlusPerSheetFormulaMembership,
        is_partial: true,
        formula_inventory_scope: WorkbookGraphFormulaInventoryScope::SelectedSheetOnly,
        formula_inventory_sheet: formulas.sheet,
        max_formulas_applied: formulas.max_formulas_applied,
        formula_inventory_truncated: formulas.truncated,
        nodes,
        edges,
        warnings,
    })
}

fn build_graph_warnings(
    workbook_warnings: &[String],
    power_query_warnings: &[String],
    formulas: &crate::tool::InspectSheetFormulasResult,
    warning_flags: GraphWarningFlags,
) -> Vec<String> {
    let mut warnings = BTreeSet::new();
    warnings.extend(workbook_warnings.iter().cloned());
    warnings.extend(power_query_warnings.iter().cloned());
    warnings.extend(formulas.warnings.iter().cloned());
    warnings.insert(
        "workbook graph is partial: it emits workbook structure, selected-sheet formula membership, and AST-backed dependency edges only where proven"
            .to_string(),
    );
    if formulas.has_external_links {
        warnings.insert(
            "workbook has external links; phase 1 graph omits external-link lineage".to_string(),
        );
    }
    if formulas
        .formulas
        .iter()
        .any(|formula| !formula.parse.unsupported_reasons.is_empty())
    {
        warnings.insert(
            "selected sheet contains unsupported formulas; graph omits dependency edges for unsupported constructs"
                .to_string(),
        );
    }
    if formulas.formulas.iter().any(|formula| {
        matches!(
            formula.parse.state,
            crate::formula_ast::FormulaAstParseState::Missing
        )
    }) {
        warnings.insert(
            "selected sheet contains formulas without parsed ASTs; graph omits dependency edges for missing formulas"
                .to_string(),
        );
    }
    if formulas
        .formulas
        .iter()
        .any(|formula| !formula.parse.diagnostics.is_empty())
    {
        warnings.insert(
            "selected sheet contains malformed formulas; graph omits dependency edges for malformed formulas"
                .to_string(),
        );
    }
    if formulas.formulas.is_empty() {
        warnings.insert(
            "selected sheet emitted no formulas; phase 1 graph contains structure only".to_string(),
        );
    }
    if warning_flags.unresolved_defined_name_reference {
        warnings.insert(
            "selected sheet contains unresolved defined-name references; graph omits dependency edges for unresolved names"
                .to_string(),
        );
    }
    if warning_flags.ambiguous_defined_name_target {
        warnings.insert(
            "workbook contains defined names with unqualified workbook-scope cell or range targets; graph keeps those targets as opaque formula text"
                .to_string(),
        );
    }
    if warning_flags.unresolved_table_reference {
        warnings.insert(
            "selected sheet contains unresolved structured table references; graph omits dependency edges for unresolved table targets"
                .to_string(),
        );
    }
    if warning_flags.unresolved_table_metadata {
        warnings.insert(
            "workbook contains unresolved table metadata; graph omits table edges unless worksheet ownership and table ranges are proven"
                .to_string(),
        );
    }
    if warning_flags.unresolved_power_query_query_reference {
        warnings.insert(
            "workbook contains unresolved Power Query shared-query references; graph omits lineage edges for unresolved query targets"
                .to_string(),
        );
    }
    if warning_flags.unresolved_power_query_workbook_name_reference {
        warnings.insert(
            "workbook contains unresolved Power Query Excel.CurrentWorkbook name references; graph omits lineage edges for unresolved workbook-name targets"
                .to_string(),
        );
    }
    warnings.into_iter().collect()
}

fn worksheet_label(summary: &crate::tool::SheetSummary, worksheet_index: usize) -> String {
    summary
        .name
        .clone()
        .unwrap_or_else(|| format!("worksheet-{}", worksheet_index + 1))
}

fn worksheet_node_id(worksheet_index: usize) -> String {
    format!("worksheet:{worksheet_index}")
}

fn formula_node_id(worksheet_index: usize, reference: &str) -> String {
    format!("formula:{worksheet_index}:{reference}")
}

#[derive(Clone)]
enum FormulaReference {
    Cell {
        reference: String,
        sheet_name: Option<String>,
    },
    DefinedName {
        name: String,
        sheet_name: Option<String>,
    },
    Range {
        start_reference: String,
        end_reference: String,
        sheet_name: Option<String>,
    },
    StructuredReference {
        text: String,
    },
}

#[derive(Default)]
struct GraphWarningFlags {
    unresolved_defined_name_reference: bool,
    ambiguous_defined_name_target: bool,
    unresolved_table_reference: bool,
    unresolved_table_metadata: bool,
    unresolved_power_query_query_reference: bool,
    unresolved_power_query_workbook_name_reference: bool,
}

struct GraphBuildState<'a> {
    nodes: &'a mut Vec<WorkbookGraphNode>,
    node_ids: &'a mut BTreeSet<String>,
    edges: &'a mut Vec<WorkbookGraphEdge>,
    edge_keys: &'a mut BTreeSet<(String, String, String)>,
}

struct DependencyResolutionContext<'a> {
    selected_worksheet_index: usize,
    worksheet_summaries: &'a [(usize, &'a crate::tool::SheetSummary)],
    defined_names: &'a [DefinedNameSummary],
    selected_sheet_name: &'a str,
    tables: &'a [ParsedWorkbookTable],
}

enum PowerQueryWorkbookNameTarget<'a> {
    DefinedName(&'a DefinedNameSummary),
    Table(&'a ParsedWorkbookTable),
}

fn emit_formula_dependency_edges(
    formula: &crate::tool::SheetFormulaSummary,
    formula_node_id: &str,
    context: &DependencyResolutionContext<'_>,
    graph: &mut GraphBuildState<'_>,
    warning_flags: &mut GraphWarningFlags,
) {
    let Some(root) = formula.parse.root.as_ref() else {
        return;
    };
    let mut references = Vec::new();
    collect_formula_references(root, &mut references);
    let Some((_, selected_summary)) = context
        .worksheet_summaries
        .get(context.selected_worksheet_index)
    else {
        return;
    };
    let Some(selected_part_path) = selected_summary.part_path.clone() else {
        return;
    };

    for reference in references {
        match reference {
            FormulaReference::Cell {
                reference,
                sheet_name,
            } => {
                let (worksheet_index, sheet_name, part_path) =
                    if let Some(sheet_name) = sheet_name.as_deref() {
                        let Some((worksheet_index, sheet_name, part_path)) =
                            resolve_reference_sheet(context.worksheet_summaries, sheet_name)
                        else {
                            continue;
                        };
                        push_edge(
                            graph,
                            WorkbookGraphEdgeKind::FormulaReferencesWorksheet,
                            formula_node_id.to_string(),
                            worksheet_node_id(worksheet_index),
                            vec![WorkbookGraphEvidence {
                                kind: WorkbookGraphEvidenceKind::FormulaAstWorksheetReference,
                                part_path: part_path.clone(),
                                cell_reference: Some(formula.reference.clone()),
                                detail: format!("ast reference resolved to worksheet {sheet_name}"),
                            }],
                        );
                        (worksheet_index, sheet_name, part_path)
                    } else {
                        (
                            context.selected_worksheet_index,
                            context.selected_sheet_name.to_string(),
                            selected_part_path.clone(),
                        )
                    };
                let node_id = cell_reference_node_id(worksheet_index, &reference);
                if graph.node_ids.insert(node_id.clone()) {
                    graph.nodes.push(WorkbookGraphNode {
                        id: node_id.clone(),
                        kind: WorkbookGraphNodeKind::ReferencedCell,
                        label: format!("{sheet_name}!{reference}"),
                        sheet_name: Some(sheet_name.clone()),
                        part_path: Some(part_path.clone()),
                        cell_reference: Some(reference.clone()),
                        formula: None,
                        cached_value: None,
                    });
                }
                push_edge(
                    graph,
                    WorkbookGraphEdgeKind::FormulaReferencesCell,
                    formula_node_id.to_string(),
                    node_id,
                    vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::FormulaAstCellReference,
                        part_path,
                        cell_reference: Some(formula.reference.clone()),
                        detail: format!("ast cell reference {sheet_name}!{reference}"),
                    }],
                );
            }
            FormulaReference::Range {
                start_reference,
                end_reference,
                sheet_name,
            } => {
                let range_reference = format!("{start_reference}:{end_reference}");
                let (worksheet_index, sheet_name, part_path) =
                    if let Some(sheet_name) = sheet_name.as_deref() {
                        let Some((worksheet_index, sheet_name, part_path)) =
                            resolve_reference_sheet(context.worksheet_summaries, sheet_name)
                        else {
                            continue;
                        };
                        push_edge(
                            graph,
                            WorkbookGraphEdgeKind::FormulaReferencesWorksheet,
                            formula_node_id.to_string(),
                            worksheet_node_id(worksheet_index),
                            vec![WorkbookGraphEvidence {
                                kind: WorkbookGraphEvidenceKind::FormulaAstWorksheetReference,
                                part_path: part_path.clone(),
                                cell_reference: Some(formula.reference.clone()),
                                detail: format!("ast reference resolved to worksheet {sheet_name}"),
                            }],
                        );
                        (worksheet_index, sheet_name, part_path)
                    } else {
                        (
                            context.selected_worksheet_index,
                            context.selected_sheet_name.to_string(),
                            selected_part_path.clone(),
                        )
                    };
                let node_id =
                    range_reference_node_id(worksheet_index, &start_reference, &end_reference);
                if graph.node_ids.insert(node_id.clone()) {
                    graph.nodes.push(WorkbookGraphNode {
                        id: node_id.clone(),
                        kind: WorkbookGraphNodeKind::ReferencedRange,
                        label: format!("{sheet_name}!{range_reference}"),
                        sheet_name: Some(sheet_name.clone()),
                        part_path: Some(part_path.clone()),
                        cell_reference: Some(range_reference.clone()),
                        formula: None,
                        cached_value: None,
                    });
                }
                push_edge(
                    graph,
                    WorkbookGraphEdgeKind::FormulaReferencesRange,
                    formula_node_id.to_string(),
                    node_id,
                    vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::FormulaAstRangeReference,
                        part_path,
                        cell_reference: Some(formula.reference.clone()),
                        detail: format!("ast range reference {sheet_name}!{range_reference}"),
                    }],
                );
            }
            FormulaReference::DefinedName { name, sheet_name } => {
                let Some(defined_name) = resolve_defined_name_reference(
                    context.defined_names,
                    context.selected_sheet_name,
                    &name,
                    sheet_name.as_deref(),
                ) else {
                    warning_flags.unresolved_defined_name_reference = true;
                    continue;
                };
                let node_id = defined_name_node_id(defined_name);
                if graph.node_ids.insert(node_id.clone()) {
                    graph.nodes.push(WorkbookGraphNode {
                        id: node_id.clone(),
                        kind: WorkbookGraphNodeKind::DefinedName,
                        label: defined_name_label(defined_name),
                        sheet_name: defined_name.sheet_scope.clone(),
                        part_path: Some("xl/workbook.xml".to_string()),
                        cell_reference: None,
                        formula: None,
                        cached_value: None,
                    });
                }
                push_edge(
                    graph,
                    WorkbookGraphEdgeKind::FormulaReferencesDefinedName,
                    formula_node_id.to_string(),
                    node_id.clone(),
                    vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::FormulaAstDefinedNameReference,
                        part_path: selected_part_path.clone(),
                        cell_reference: Some(formula.reference.clone()),
                        detail: format!(
                            "ast defined-name reference {}",
                            defined_name_label(defined_name)
                        ),
                    }],
                );
                emit_defined_name_target_edges(
                    defined_name,
                    &node_id,
                    context.worksheet_summaries,
                    graph,
                    warning_flags,
                );
            }
            FormulaReference::StructuredReference { text } => {
                let Some(table_name) = parse_structured_reference_table_name(&text) else {
                    warning_flags.unresolved_table_reference = true;
                    continue;
                };
                let Some(table) = resolve_table_reference(context.tables, table_name) else {
                    warning_flags.unresolved_table_reference = true;
                    continue;
                };
                let table_node_id = table_node_id(table);
                push_edge(
                    graph,
                    WorkbookGraphEdgeKind::FormulaReferencesTable,
                    formula_node_id.to_string(),
                    table_node_id,
                    vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::FormulaAstStructuredReference,
                        part_path: selected_part_path.clone(),
                        cell_reference: Some(formula.reference.clone()),
                        detail: format!(
                            "structured reference {} resolved to table {}!{}",
                            text, table.sheet_name, table.name
                        ),
                    }],
                );
            }
        }
    }
}

fn emit_power_query_lineage_edges(
    queries: &[ExtractedPowerQueryQuery],
    defined_names: &[DefinedNameSummary],
    tables: &[ParsedWorkbookTable],
    worksheet_summaries: &[(usize, &crate::tool::SheetSummary)],
    graph: &mut GraphBuildState<'_>,
    warning_flags: &mut GraphWarningFlags,
) {
    for query in queries {
        let node_id = power_query_query_node_id(&query.name);
        if graph.node_ids.insert(node_id.clone()) {
            graph.nodes.push(WorkbookGraphNode {
                id: node_id,
                kind: WorkbookGraphNodeKind::PowerQueryQuery,
                label: query.name.clone(),
                sheet_name: None,
                part_path: Some(query.source_part.clone()),
                cell_reference: None,
                formula: Some(query.source.clone()),
                cached_value: None,
            });
        }
    }

    for query in queries {
        let from_id = power_query_query_node_id(&query.name);
        for reference in &query.lexical_references {
            match reference.kind {
                PowerQueryLexicalReferenceKind::QueryName => {
                    let Some(target_query) =
                        resolve_power_query_reference(queries, &reference.target_name)
                    else {
                        warning_flags.unresolved_power_query_query_reference = true;
                        continue;
                    };
                    push_edge(
                        graph,
                        WorkbookGraphEdgeKind::PowerQueryReferencesQuery,
                        from_id.clone(),
                        power_query_query_node_id(&target_query.name),
                        vec![WorkbookGraphEvidence {
                            kind: WorkbookGraphEvidenceKind::PowerQueryLexicalQueryReference,
                            part_path: query.source_part.clone(),
                            cell_reference: None,
                            detail: format!(
                                "Power Query {} lexically references query {} on line {}",
                                query.name, target_query.name, reference.source_line
                            ),
                        }],
                    );
                }
                PowerQueryLexicalReferenceKind::WorkbookName => {
                    let Some(target) = resolve_power_query_workbook_name_target(
                        defined_names,
                        tables,
                        &reference.target_name,
                    ) else {
                        warning_flags.unresolved_power_query_workbook_name_reference = true;
                        continue;
                    };
                    match target {
                        PowerQueryWorkbookNameTarget::DefinedName(defined_name) => {
                            let node_id = ensure_defined_name_node(graph, defined_name);
                            push_edge(
                                graph,
                                WorkbookGraphEdgeKind::PowerQueryReferencesDefinedName,
                                from_id.clone(),
                                node_id.clone(),
                                vec![WorkbookGraphEvidence {
                                    kind: WorkbookGraphEvidenceKind::PowerQueryLexicalWorkbookNameReference,
                                    part_path: query.source_part.clone(),
                                    cell_reference: None,
                                    detail: format!(
                                        "Power Query {} resolved Excel.CurrentWorkbook name {} to defined name {} on line {}",
                                        query.name,
                                        reference.target_name,
                                        defined_name_label(defined_name),
                                        reference.source_line
                                    ),
                                }],
                            );
                            emit_defined_name_target_edges(
                                defined_name,
                                &node_id,
                                worksheet_summaries,
                                graph,
                                warning_flags,
                            );
                        }
                        PowerQueryWorkbookNameTarget::Table(table) => {
                            push_edge(
                                graph,
                                WorkbookGraphEdgeKind::PowerQueryReferencesTable,
                                from_id.clone(),
                                table_node_id(table),
                                vec![WorkbookGraphEvidence {
                                    kind: WorkbookGraphEvidenceKind::PowerQueryLexicalWorkbookNameReference,
                                    part_path: query.source_part.clone(),
                                    cell_reference: None,
                                    detail: format!(
                                        "Power Query {} resolved Excel.CurrentWorkbook name {} to table {}!{} on line {}",
                                        query.name,
                                        reference.target_name,
                                        table.sheet_name,
                                        table.name,
                                        reference.source_line
                                    ),
                                }],
                            );
                        }
                    }
                }
            }
        }
    }
}

fn collect_formula_references(node: &FormulaAstNode, references: &mut Vec<FormulaReference>) {
    match node {
        FormulaAstNode::UnaryOperation { operand, .. } | FormulaAstNode::Percent { operand } => {
            collect_formula_references(operand, references)
        }
        FormulaAstNode::BinaryOperation { left, right, .. } => {
            collect_formula_references(left, references);
            collect_formula_references(right, references);
        }
        FormulaAstNode::FunctionCall { args, .. } => {
            for arg in args {
                collect_formula_references(arg, references);
            }
        }
        FormulaAstNode::CellReference {
            reference,
            sheet_name,
        } => references.push(FormulaReference::Cell {
            reference: reference.clone(),
            sheet_name: sheet_name.clone(),
        }),
        FormulaAstNode::DefinedNameReference { name, sheet_name } => {
            references.push(FormulaReference::DefinedName {
                name: name.clone(),
                sheet_name: sheet_name.clone(),
            })
        }
        FormulaAstNode::RangeReference {
            start_reference,
            end_reference,
            sheet_name,
        } => references.push(FormulaReference::Range {
            start_reference: start_reference.clone(),
            end_reference: end_reference.clone(),
            sheet_name: sheet_name.clone(),
        }),
        FormulaAstNode::Unsupported { reason, text } if reason == "structured_reference" => {
            references.push(FormulaReference::StructuredReference { text: text.clone() });
        }
        FormulaAstNode::NumberLiteral { .. }
        | FormulaAstNode::StringLiteral { .. }
        | FormulaAstNode::BooleanLiteral { .. }
        | FormulaAstNode::BlankArgument
        | FormulaAstNode::ErrorLiteral { .. }
        | FormulaAstNode::Unsupported { .. } => {}
    }
}

fn resolve_defined_name_reference<'a>(
    defined_names: &'a [DefinedNameSummary],
    selected_sheet_name: &str,
    name: &str,
    explicit_sheet_name: Option<&str>,
) -> Option<&'a DefinedNameSummary> {
    let matching_name = defined_names
        .iter()
        .filter(|defined_name| defined_name.name.eq_ignore_ascii_case(name))
        .collect::<Vec<_>>();
    if matching_name.is_empty() {
        return None;
    }
    if let Some(sheet_name) = explicit_sheet_name {
        let scoped = matching_name
            .into_iter()
            .filter(|defined_name| {
                defined_name
                    .sheet_scope
                    .as_deref()
                    .is_some_and(|scope| scope.eq_ignore_ascii_case(sheet_name))
            })
            .collect::<Vec<_>>();
        return if scoped.len() == 1 {
            Some(scoped[0])
        } else {
            None
        };
    }
    let sheet_scoped = matching_name
        .iter()
        .copied()
        .filter(|defined_name| {
            defined_name
                .sheet_scope
                .as_deref()
                .is_some_and(|scope| scope.eq_ignore_ascii_case(selected_sheet_name))
        })
        .collect::<Vec<_>>();
    if sheet_scoped.len() == 1 {
        return Some(sheet_scoped[0]);
    }
    let workbook_scoped = matching_name
        .into_iter()
        .filter(|defined_name| defined_name.sheet_scope.is_none())
        .collect::<Vec<_>>();
    if workbook_scoped.len() == 1 {
        Some(workbook_scoped[0])
    } else {
        None
    }
}

fn resolve_power_query_reference<'a>(
    queries: &'a [ExtractedPowerQueryQuery],
    name: &str,
) -> Option<&'a ExtractedPowerQueryQuery> {
    let matching = queries
        .iter()
        .filter(|query| query.name.eq_ignore_ascii_case(name))
        .collect::<Vec<_>>();
    if matching.len() == 1 {
        Some(matching[0])
    } else {
        None
    }
}

fn resolve_power_query_workbook_name_target<'a>(
    defined_names: &'a [DefinedNameSummary],
    tables: &'a [ParsedWorkbookTable],
    name: &str,
) -> Option<PowerQueryWorkbookNameTarget<'a>> {
    let matching_defined_names = defined_names
        .iter()
        .filter(|defined_name| {
            defined_name.sheet_scope.is_none() && defined_name.name.eq_ignore_ascii_case(name)
        })
        .collect::<Vec<_>>();
    let matching_tables = tables
        .iter()
        .filter(|table| {
            table.name.eq_ignore_ascii_case(name)
                || table
                    .alt_name
                    .as_deref()
                    .is_some_and(|table_name| table_name.eq_ignore_ascii_case(name))
        })
        .collect::<Vec<_>>();
    match (matching_defined_names.len(), matching_tables.len()) {
        (1, 0) => Some(PowerQueryWorkbookNameTarget::DefinedName(
            matching_defined_names[0],
        )),
        (0, 1) => Some(PowerQueryWorkbookNameTarget::Table(matching_tables[0])),
        _ => None,
    }
}

fn emit_defined_name_target_edges(
    defined_name: &DefinedNameSummary,
    defined_name_node_id: &str,
    worksheet_summaries: &[(usize, &crate::tool::SheetSummary)],
    graph: &mut GraphBuildState<'_>,
    warning_flags: &mut GraphWarningFlags,
) {
    let target_parse = parse_formula_ast(&defined_name.target, defined_name.truncated);
    let Some(root) = target_parse.root.as_ref() else {
        emit_defined_name_formula_text_edge(defined_name, defined_name_node_id, graph);
        return;
    };

    match root {
        FormulaAstNode::CellReference {
            reference,
            sheet_name,
        } => {
            let Some((worksheet_index, resolved_sheet_name, part_path)) =
                resolve_defined_name_target_sheet(defined_name, worksheet_summaries, sheet_name)
            else {
                warning_flags.ambiguous_defined_name_target = true;
                emit_defined_name_formula_text_edge(defined_name, defined_name_node_id, graph);
                return;
            };
            let node_id = cell_reference_node_id(worksheet_index, reference);
            if graph.node_ids.insert(node_id.clone()) {
                graph.nodes.push(WorkbookGraphNode {
                    id: node_id.clone(),
                    kind: WorkbookGraphNodeKind::ReferencedCell,
                    label: format!("{resolved_sheet_name}!{reference}"),
                    sheet_name: Some(resolved_sheet_name.clone()),
                    part_path: Some(part_path),
                    cell_reference: Some(reference.clone()),
                    formula: None,
                    cached_value: None,
                });
            }
            push_edge(
                graph,
                WorkbookGraphEdgeKind::DefinedNameTargetsCell,
                defined_name_node_id.to_string(),
                node_id,
                vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::WorkbookDefinedNameTarget,
                    part_path: "xl/workbook.xml".to_string(),
                    cell_reference: None,
                    detail: format!(
                        "defined name {} targets cell {resolved_sheet_name}!{reference}",
                        defined_name_label(defined_name)
                    ),
                }],
            );
        }
        FormulaAstNode::RangeReference {
            start_reference,
            end_reference,
            sheet_name,
        } => {
            let Some((worksheet_index, resolved_sheet_name, part_path)) =
                resolve_defined_name_target_sheet(defined_name, worksheet_summaries, sheet_name)
            else {
                warning_flags.ambiguous_defined_name_target = true;
                emit_defined_name_formula_text_edge(defined_name, defined_name_node_id, graph);
                return;
            };
            let node_id = range_reference_node_id(worksheet_index, start_reference, end_reference);
            let range_reference = format!("{start_reference}:{end_reference}");
            if graph.node_ids.insert(node_id.clone()) {
                graph.nodes.push(WorkbookGraphNode {
                    id: node_id.clone(),
                    kind: WorkbookGraphNodeKind::ReferencedRange,
                    label: format!("{resolved_sheet_name}!{range_reference}"),
                    sheet_name: Some(resolved_sheet_name.clone()),
                    part_path: Some(part_path),
                    cell_reference: Some(range_reference.clone()),
                    formula: None,
                    cached_value: None,
                });
            }
            push_edge(
                graph,
                WorkbookGraphEdgeKind::DefinedNameTargetsRange,
                defined_name_node_id.to_string(),
                node_id,
                vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::WorkbookDefinedNameTarget,
                    part_path: "xl/workbook.xml".to_string(),
                    cell_reference: None,
                    detail: format!(
                        "defined name {} targets range {resolved_sheet_name}!{range_reference}",
                        defined_name_label(defined_name)
                    ),
                }],
            );
        }
        _ => emit_defined_name_formula_text_edge(defined_name, defined_name_node_id, graph),
    }
}

fn emit_defined_name_formula_text_edge(
    defined_name: &DefinedNameSummary,
    defined_name_node_id: &str,
    graph: &mut GraphBuildState<'_>,
) {
    let node_id = defined_name_formula_text_node_id(defined_name);
    if graph.node_ids.insert(node_id.clone()) {
        graph.nodes.push(WorkbookGraphNode {
            id: node_id.clone(),
            kind: WorkbookGraphNodeKind::DefinedNameFormulaText,
            label: format!(
                "{}={}",
                defined_name_label(defined_name),
                defined_name.target
            ),
            sheet_name: defined_name.sheet_scope.clone(),
            part_path: Some("xl/workbook.xml".to_string()),
            cell_reference: None,
            formula: Some(defined_name.target.clone()),
            cached_value: None,
        });
    }
    push_edge(
        graph,
        WorkbookGraphEdgeKind::DefinedNameTargetsFormulaText,
        defined_name_node_id.to_string(),
        node_id,
        vec![WorkbookGraphEvidence {
            kind: WorkbookGraphEvidenceKind::WorkbookDefinedNameTarget,
            part_path: "xl/workbook.xml".to_string(),
            cell_reference: None,
            detail: format!(
                "defined name {} keeps opaque formula text {}",
                defined_name_label(defined_name),
                defined_name.target
            ),
        }],
    );
}

fn resolve_defined_name_target_sheet(
    defined_name: &DefinedNameSummary,
    worksheet_summaries: &[(usize, &crate::tool::SheetSummary)],
    parsed_sheet_name: &Option<String>,
) -> Option<(usize, String, String)> {
    if let Some(sheet_name) = parsed_sheet_name.as_deref() {
        return resolve_reference_sheet(worksheet_summaries, sheet_name);
    }
    let sheet_scope = defined_name.sheet_scope.as_deref()?;
    resolve_reference_sheet(worksheet_summaries, sheet_scope)
}

fn resolve_reference_sheet(
    worksheet_summaries: &[(usize, &crate::tool::SheetSummary)],
    sheet_name: &str,
) -> Option<(usize, String, String)> {
    let resolved = worksheet_summaries
        .iter()
        .enumerate()
        .find(|(_, (_, summary))| {
            summary
                .name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(sheet_name))
        });
    resolved.and_then(|(worksheet_index, (_, summary))| {
        Some((
            worksheet_index,
            summary.name.clone()?,
            summary.part_path.clone()?,
        ))
    })
}

fn parse_structured_reference_table_name(text: &str) -> Option<&str> {
    let (table_name, _) = text.split_once('[')?;
    let trimmed = table_name.trim();
    (!trimmed.is_empty() && !trimmed.starts_with('[')).then_some(trimmed)
}

fn resolve_table_reference<'a>(
    tables: &'a [ParsedWorkbookTable],
    table_name: &str,
) -> Option<&'a ParsedWorkbookTable> {
    let matching = tables
        .iter()
        .filter(|table| {
            table.name.eq_ignore_ascii_case(table_name)
                || table
                    .alt_name
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case(table_name))
        })
        .collect::<Vec<_>>();
    if matching.len() == 1 {
        Some(matching[0])
    } else {
        None
    }
}

fn ensure_defined_name_node(
    graph: &mut GraphBuildState<'_>,
    defined_name: &DefinedNameSummary,
) -> String {
    let node_id = defined_name_node_id(defined_name);
    if graph.node_ids.insert(node_id.clone()) {
        graph.nodes.push(WorkbookGraphNode {
            id: node_id.clone(),
            kind: WorkbookGraphNodeKind::DefinedName,
            label: defined_name_label(defined_name),
            sheet_name: defined_name.sheet_scope.clone(),
            part_path: Some("xl/workbook.xml".to_string()),
            cell_reference: None,
            formula: None,
            cached_value: None,
        });
    }
    node_id
}

fn power_query_query_node_id(name: &str) -> String {
    format!("power-query:{}", name.to_ascii_lowercase())
}

fn table_node_id(table: &ParsedWorkbookTable) -> String {
    format!(
        "table:{}:{}",
        table.worksheet_index,
        table.name.to_ascii_lowercase()
    )
}

fn push_edge(
    graph: &mut GraphBuildState<'_>,
    kind: WorkbookGraphEdgeKind,
    from: String,
    to: String,
    evidence: Vec<WorkbookGraphEvidence>,
) {
    let key = (format!("{kind:?}"), from.clone(), to.clone());
    if graph.edge_keys.insert(key) {
        graph.edges.push(WorkbookGraphEdge {
            kind,
            from,
            to,
            evidence,
        });
    }
}

fn cell_reference_node_id(worksheet_index: usize, reference: &str) -> String {
    format!("ref-cell:{worksheet_index}:{reference}")
}

fn defined_name_label(defined_name: &DefinedNameSummary) -> String {
    let scope_prefix = defined_name
        .sheet_scope
        .as_deref()
        .map(|sheet_name| format!("{sheet_name}!"))
        .unwrap_or_default();
    let hidden_suffix = if defined_name.hidden.is_some_and(|hidden| hidden) {
        " [hidden]"
    } else {
        Default::default()
    };
    format!("{scope_prefix}{}{}", defined_name.name, hidden_suffix)
}

fn defined_name_node_id(defined_name: &DefinedNameSummary) -> String {
    let scope_key = defined_name
        .sheet_scope
        .as_deref()
        .unwrap_or("workbook")
        .to_ascii_lowercase();
    format!(
        "defined-name:{scope_key}:{}",
        defined_name.name.to_ascii_lowercase()
    )
}

fn defined_name_formula_text_node_id(defined_name: &DefinedNameSummary) -> String {
    format!(
        "defined-name-formula:{}",
        defined_name_node_id(defined_name)
    )
}

fn range_reference_node_id(
    worksheet_index: usize,
    start_reference: &str,
    end_reference: &str,
) -> String {
    format!("ref-range:{worksheet_index}:{start_reference}:{end_reference}")
}

fn parse_tool_args<T: DeserializeOwned>(call: &ToolCall) -> Result<T, FunctionCallError> {
    let arguments = call.function_arguments()?;
    serde_json::from_str(arguments).map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "invalid excel.inspect_workbook_graph arguments: {err}"
        ))
    })
}

fn workbook_path_from_graph_arg(path: &str, cwd: &Path) -> Result<PathBuf, FunctionCallError> {
    local_relative_path_from_model_arg(
        path,
        cwd,
        "excel.inspect_workbook_graph path",
        &["xlsx", "xlsm", "xlsb"],
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
            "{label} must be a local file path"
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
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
    else {
        return Err(FunctionCallError::RespondToModel(format!(
            "{label} must end in .{}",
            extensions.join(", .")
        )));
    };
    if !extensions.iter().any(|allowed| extension == *allowed) {
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
