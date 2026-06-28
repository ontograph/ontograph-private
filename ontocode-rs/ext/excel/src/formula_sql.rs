use std::collections::BTreeSet;

use crate::formula_ast::FormulaAstBinaryOperator;
use crate::formula_ast::FormulaAstNode;
use crate::formula_ast::FormulaAstParseState;
use crate::formula_ast::FormulaAstUnaryOperator;
use crate::preview::bounded_text;
use crate::tool::DefinedNameSummary;
use crate::tool::FormulaSqlPreviewState;
use crate::tool::FormulaSqlPreviewSummary;
use crate::tool::FormulaSqlReferenceKind;
use crate::tool::FormulaSqlReferenceSummary;
use crate::tool::SheetFormulaSummary;

const MAX_SQL_EXPRESSION_CHARS: usize = 512;
const MAX_SQL_REFERENCES: usize = 16;
const MAX_BLOCKERS: usize = 8;

#[derive(Clone, Copy)]
enum AggregateFunction {
    Sum,
    Count,
    Average,
    Max,
    Min,
}

pub(crate) fn plan_formula_sql_preview(
    sheet_name: &str,
    formula: &SheetFormulaSummary,
    defined_names: &[DefinedNameSummary],
    has_external_links: bool,
) -> FormulaSqlPreviewSummary {
    let mut blockers = BTreeSet::new();
    if has_external_links {
        blockers.insert("workbook_has_external_links".to_string());
    }

    let mut references = Vec::new();
    let formula_row = parse_a1_reference(formula.reference.as_str()).map(|(_, row)| row);
    let sql_expression = match formula.parse.state {
        FormulaAstParseState::Missing => {
            blockers.insert("formula_text_missing".to_string());
            None
        }
        FormulaAstParseState::Malformed => {
            blockers.insert("formula_parse_malformed".to_string());
            None
        }
        FormulaAstParseState::Parsed | FormulaAstParseState::Unsupported => {
            formula.parse.root.as_ref().and_then(|root| {
                emit_sql_expression(
                    root,
                    sheet_name,
                    formula.reference.as_str(),
                    formula_row,
                    defined_names,
                    &mut references,
                    &mut blockers,
                )
            })
        }
    };

    blockers.extend(formula.parse.unsupported_reasons.iter().cloned());

    let blocker_reasons = blockers.into_iter().take(MAX_BLOCKERS).collect::<Vec<_>>();
    FormulaSqlPreviewSummary {
        state: if blocker_reasons.is_empty() && sql_expression.is_some() {
            FormulaSqlPreviewState::ReviewOnly
        } else {
            FormulaSqlPreviewState::Blocked
        },
        sql_expression: if blocker_reasons.is_empty() {
            sql_expression.map(|value| bounded_text(&value, MAX_SQL_EXPRESSION_CHARS))
        } else {
            None
        },
        references,
        blocker_reasons,
        cached_value_present: formula.cached_value.is_some(),
    }
}

fn emit_sql_expression(
    node: &FormulaAstNode,
    current_sheet_name: &str,
    formula_reference: &str,
    formula_row: Option<u32>,
    defined_names: &[DefinedNameSummary],
    references: &mut Vec<FormulaSqlReferenceSummary>,
    blockers: &mut BTreeSet<String>,
) -> Option<String> {
    match node {
        FormulaAstNode::NumberLiteral { value } => Some(value.clone()),
        FormulaAstNode::StringLiteral { value } => Some(format!("'{}'", value.replace('\'', "''"))),
        FormulaAstNode::BooleanLiteral { value } => {
            Some(if *value { "TRUE" } else { "FALSE" }.to_string())
        }
        FormulaAstNode::BlankArgument => {
            blockers.insert("blank_argument_not_supported_in_scalar_phase".to_string());
            None
        }
        FormulaAstNode::ErrorLiteral { .. } => {
            blockers.insert("error_literal_not_supported_in_scalar_phase".to_string());
            None
        }
        FormulaAstNode::UnaryOperation { operator, operand } => emit_sql_expression(
            operand,
            current_sheet_name,
            formula_reference,
            formula_row,
            defined_names,
            references,
            blockers,
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
            let left = emit_sql_expression(
                left,
                current_sheet_name,
                formula_reference,
                formula_row,
                defined_names,
                references,
                blockers,
            );
            let right = emit_sql_expression(
                right,
                current_sheet_name,
                formula_reference,
                formula_row,
                defined_names,
                references,
                blockers,
            );
            match (left, right) {
                (Some(left), Some(right)) => Some(match operator {
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
                    FormulaAstBinaryOperator::GreaterThanOrEqual => {
                        format!("({left} >= {right})")
                    }
                }),
                _ => None,
            }
        }
        FormulaAstNode::Percent { operand } => emit_sql_expression(
            operand,
            current_sheet_name,
            formula_reference,
            formula_row,
            defined_names,
            references,
            blockers,
        )
        .map(|operand| format!("({operand} / 100.0)")),
        FormulaAstNode::FunctionCall { name, args } => {
            if name.eq_ignore_ascii_case("VLOOKUP") {
                return emit_vlookup_expression(
                    args,
                    current_sheet_name,
                    formula_reference,
                    formula_row,
                    defined_names,
                    references,
                    blockers,
                );
            }
            if name.eq_ignore_ascii_case("XLOOKUP") {
                return emit_xlookup_expression(
                    args,
                    current_sheet_name,
                    formula_reference,
                    formula_row,
                    defined_names,
                    references,
                    blockers,
                );
            }
            if name.eq_ignore_ascii_case("INDEX") {
                return emit_index_match_expression(
                    args,
                    current_sheet_name,
                    formula_reference,
                    formula_row,
                    defined_names,
                    references,
                    blockers,
                );
            }
            if let Some(aggregate_function) = aggregate_function(name.as_str()) {
                return emit_aggregate_expression(
                    aggregate_function,
                    args,
                    current_sheet_name,
                    formula_reference,
                    defined_names,
                    references,
                    blockers,
                );
            }
            if name.eq_ignore_ascii_case("MATCH") {
                blockers.insert("match_not_supported_outside_index".to_string());
            } else {
                blockers.insert("function_call_not_supported_in_scalar_phase".to_string());
            }
            for argument in args {
                let _ = emit_sql_expression(
                    argument,
                    current_sheet_name,
                    formula_reference,
                    formula_row,
                    defined_names,
                    references,
                    blockers,
                );
            }
            None
        }
        FormulaAstNode::CellReference {
            reference,
            sheet_name,
        } => emit_cell_reference(
            reference,
            sheet_name.as_deref(),
            current_sheet_name,
            formula_reference,
            formula_row,
            references,
            blockers,
        ),
        FormulaAstNode::RangeReference {
            start_reference,
            end_reference,
            sheet_name,
        } => {
            push_reference(
                references,
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: format!("{start_reference}:{end_reference}"),
                    sheet_name: sheet_name.clone(),
                    same_row: None,
                    sql_identifier: None,
                },
            );
            blockers.insert("range_reference_not_supported_in_scalar_phase".to_string());
            None
        }
        FormulaAstNode::DefinedNameReference { name, sheet_name } => {
            push_reference(
                references,
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::DefinedName,
                    reference: name.clone(),
                    sheet_name: sheet_name.clone(),
                    same_row: None,
                    sql_identifier: None,
                },
            );
            blockers.insert("defined_name_reference_not_supported_in_scalar_phase".to_string());
            None
        }
        FormulaAstNode::Unsupported { reason, .. } => {
            blockers.insert(reason.clone());
            None
        }
    }
}

fn emit_cell_reference(
    reference: &str,
    sheet_name: Option<&str>,
    current_sheet_name: &str,
    formula_reference: &str,
    formula_row: Option<u32>,
    references: &mut Vec<FormulaSqlReferenceSummary>,
    blockers: &mut BTreeSet<String>,
) -> Option<String> {
    let parsed_reference = parse_a1_reference(reference);
    let same_row = match (formula_row, parsed_reference.as_ref()) {
        (Some(formula_row), Some((_, reference_row))) => Some(*reference_row == formula_row),
        _ => None,
    };
    let sql_identifier = parsed_reference
        .as_ref()
        .map(|(column_letters, _)| format!("col_{}", column_letters.to_ascii_lowercase()));

    push_reference(
        references,
        FormulaSqlReferenceSummary {
            kind: FormulaSqlReferenceKind::Cell,
            reference: reference.to_string(),
            sheet_name: sheet_name.map(ToOwned::to_owned),
            same_row,
            sql_identifier: sql_identifier.clone(),
        },
    );

    if parsed_reference.is_none() {
        blockers.insert("cell_reference_not_a1".to_string());
        return None;
    }
    if same_row != Some(true) {
        blockers.insert("cross_row_reference_not_supported_in_scalar_phase".to_string());
    }
    if let Some(sheet_name) = sheet_name
        && !sheet_name.eq_ignore_ascii_case(current_sheet_name)
    {
        blockers.insert("cross_sheet_reference_not_supported_in_scalar_phase".to_string());
    }
    if same_reference(reference, formula_reference) {
        blockers.insert("self_reference_not_supported_in_scalar_phase".to_string());
    }

    sql_identifier.map(|identifier| format!("[{identifier}]"))
}

fn aggregate_function(name: &str) -> Option<AggregateFunction> {
    if name.eq_ignore_ascii_case("SUMIFS") {
        Some(AggregateFunction::Sum)
    } else if name.eq_ignore_ascii_case("COUNTIFS") {
        Some(AggregateFunction::Count)
    } else if name.eq_ignore_ascii_case("AVERAGEIFS") {
        Some(AggregateFunction::Average)
    } else if name.eq_ignore_ascii_case("MAXIFS") {
        Some(AggregateFunction::Max)
    } else if name.eq_ignore_ascii_case("MINIFS") {
        Some(AggregateFunction::Min)
    } else {
        None
    }
}

fn emit_aggregate_expression(
    aggregate_function: AggregateFunction,
    args: &[FormulaAstNode],
    current_sheet_name: &str,
    formula_reference: &str,
    defined_names: &[DefinedNameSummary],
    references: &mut Vec<FormulaSqlReferenceSummary>,
    blockers: &mut BTreeSet<String>,
) -> Option<String> {
    if !aggregate_function.supports_arity(args.len()) {
        blockers.insert(aggregate_function.arity_blocker().to_string());
        return None;
    }

    let mut criteria = Vec::new();
    let base_vector = if aggregate_function.requires_value_range() {
        let value_vector = resolve_lookup_vector(
            &args[0],
            current_sheet_name,
            defined_names,
            references,
            blockers,
            "aggregate_value_range_not_proven_vector",
        )?;
        let mut index = 1;
        let mut criteria_index = 1usize;
        while index < args.len() {
            let criteria_vector = resolve_lookup_vector(
                &args[index],
                current_sheet_name,
                defined_names,
                references,
                blockers,
                "aggregate_criteria_range_not_proven_vector",
            )?;
            let criteria_sql = emit_aggregate_criteria_sql(
                &args[index + 1],
                current_sheet_name,
                formula_reference,
                defined_names,
                references,
                blockers,
            )?;
            criteria.push((criteria_index, criteria_vector, criteria_sql));
            criteria_index += 1;
            index += 2;
        }
        validate_aligned_aggregate_vectors(Some(&value_vector), &criteria, blockers)?;
        value_vector
    } else {
        let base_vector = resolve_lookup_vector(
            &args[0],
            current_sheet_name,
            defined_names,
            references,
            blockers,
            "aggregate_criteria_range_not_proven_vector",
        )?;
        let first_criteria_sql = emit_aggregate_criteria_sql(
            &args[1],
            current_sheet_name,
            formula_reference,
            defined_names,
            references,
            blockers,
        )?;
        criteria.push((1, base_vector.clone(), first_criteria_sql));

        let mut index = 2;
        let mut criteria_index = 2usize;
        while index < args.len() {
            let criteria_vector = resolve_lookup_vector(
                &args[index],
                current_sheet_name,
                defined_names,
                references,
                blockers,
                "aggregate_criteria_range_not_proven_vector",
            )?;
            let criteria_sql = emit_aggregate_criteria_sql(
                &args[index + 1],
                current_sheet_name,
                formula_reference,
                defined_names,
                references,
                blockers,
            )?;
            criteria.push((criteria_index, criteria_vector, criteria_sql));
            criteria_index += 1;
            index += 2;
        }
        validate_aligned_aggregate_vectors(None, &criteria, blockers)?;
        base_vector
    };

    let where_clause = criteria
        .iter()
        .map(|(index, _, criteria_sql)| format!("[criteria_col_{index}] = {criteria_sql}"))
        .collect::<Vec<_>>()
        .join(" AND ");
    let aggregate_expression = aggregate_function.sql_aggregate();
    let source_id = build_aggregate_source_id(&base_vector, &criteria);
    Some(format!(
        "(SELECT {aggregate_expression} FROM [{source_id}] WHERE {where_clause})"
    ))
}

fn emit_aggregate_criteria_sql(
    node: &FormulaAstNode,
    current_sheet_name: &str,
    formula_reference: &str,
    defined_names: &[DefinedNameSummary],
    references: &mut Vec<FormulaSqlReferenceSummary>,
    blockers: &mut BTreeSet<String>,
) -> Option<String> {
    if let FormulaAstNode::StringLiteral { value } = node {
        let trimmed = value.trim_start();
        if trimmed.starts_with(">=")
            || trimmed.starts_with("<=")
            || trimmed.starts_with("<>")
            || trimmed.starts_with('>')
            || trimmed.starts_with('<')
            || trimmed.starts_with('=')
        {
            blockers.insert("aggregate_criteria_operator_string_not_supported".to_string());
            return None;
        }
        if value.contains('*') || value.contains('?') {
            blockers.insert("aggregate_criteria_wildcards_not_supported".to_string());
            return None;
        }
    }

    let formula_row = parse_a1_reference(formula_reference).map(|(_, row)| row);
    emit_sql_expression(
        node,
        current_sheet_name,
        formula_reference,
        formula_row,
        defined_names,
        references,
        blockers,
    )
}

fn emit_vlookup_expression(
    args: &[FormulaAstNode],
    current_sheet_name: &str,
    formula_reference: &str,
    formula_row: Option<u32>,
    defined_names: &[DefinedNameSummary],
    references: &mut Vec<FormulaSqlReferenceSummary>,
    blockers: &mut BTreeSet<String>,
) -> Option<String> {
    if args.len() != 4 {
        blockers.insert("vlookup_arity_not_supported".to_string());
        return None;
    }

    let lookup_value = emit_sql_expression(
        &args[0],
        current_sheet_name,
        formula_reference,
        formula_row,
        defined_names,
        references,
        blockers,
    );
    let (defined_name, defined_name_range) = resolve_defined_name_range(
        &args[1],
        current_sheet_name,
        defined_names,
        references,
        blockers,
    )?;
    let column_index = parse_vlookup_column_index(&args[2], blockers)?;
    if !is_exact_vlookup_match(&args[3]) {
        blockers.insert("approximate_lookup_not_supported".to_string());
        return None;
    }
    let lookup_value = lookup_value?;
    let width = defined_name_range
        .end_column
        .saturating_sub(defined_name_range.start_column)
        + 1;
    if column_index == 0 || column_index > width {
        blockers.insert("lookup_column_index_out_of_bounds".to_string());
        return None;
    }

    Some(format!(
        "(SELECT [lookup_col_{column_index}] FROM [lookup_{}] WHERE [lookup_col_1] = {lookup_value})",
        normalize_lookup_identifier(defined_name.name.as_str())
    ))
}

fn emit_xlookup_expression(
    args: &[FormulaAstNode],
    current_sheet_name: &str,
    formula_reference: &str,
    formula_row: Option<u32>,
    defined_names: &[DefinedNameSummary],
    references: &mut Vec<FormulaSqlReferenceSummary>,
    blockers: &mut BTreeSet<String>,
) -> Option<String> {
    if !(3..=6).contains(&args.len()) {
        blockers.insert("xlookup_arity_not_supported".to_string());
        return None;
    }

    let lookup_value = emit_sql_expression(
        &args[0],
        current_sheet_name,
        formula_reference,
        formula_row,
        defined_names,
        references,
        blockers,
    )?;
    let lookup_vector = resolve_lookup_vector(
        &args[1],
        current_sheet_name,
        defined_names,
        references,
        blockers,
        "xlookup_lookup_array_not_proven_vector",
    )?;
    let return_vector = resolve_lookup_vector(
        &args[2],
        current_sheet_name,
        defined_names,
        references,
        blockers,
        "xlookup_return_array_not_proven_vector",
    )?;
    if let Some(if_not_found) = args.get(3)
        && !matches!(if_not_found, FormulaAstNode::BlankArgument)
    {
        blockers.insert("xlookup_if_not_found_not_supported".to_string());
        return None;
    }
    if let Some(match_mode) = args.get(4)
        && !matches!(match_mode, FormulaAstNode::NumberLiteral { value } if value == "0")
    {
        blockers.insert("approximate_lookup_not_supported".to_string());
        return None;
    }
    if let Some(search_mode) = args.get(5)
        && !matches!(search_mode, FormulaAstNode::NumberLiteral { value } if value == "1")
    {
        blockers.insert("xlookup_search_mode_not_supported".to_string());
        return None;
    }

    let source_id = validate_aligned_lookup_vectors(
        &lookup_vector,
        &return_vector,
        blockers,
        "xlookup_lookup_array_must_be_single_column",
        "xlookup_return_array_must_be_single_column",
    )?;

    Some(format!(
        "(SELECT [lookup_return_1] FROM [{source_id}] WHERE [lookup_key_1] = {lookup_value})"
    ))
}

fn emit_index_match_expression(
    args: &[FormulaAstNode],
    current_sheet_name: &str,
    formula_reference: &str,
    formula_row: Option<u32>,
    defined_names: &[DefinedNameSummary],
    references: &mut Vec<FormulaSqlReferenceSummary>,
    blockers: &mut BTreeSet<String>,
) -> Option<String> {
    if args.len() != 2 {
        blockers.insert("index_match_arity_not_supported".to_string());
        return None;
    }

    let return_vector = resolve_lookup_vector(
        &args[0],
        current_sheet_name,
        defined_names,
        references,
        blockers,
        "index_return_array_not_proven_vector",
    )?;
    let FormulaAstNode::FunctionCall {
        name,
        args: match_args,
    } = &args[1]
    else {
        blockers.insert("index_second_arg_must_be_exact_match".to_string());
        return None;
    };
    if !name.eq_ignore_ascii_case("MATCH") || match_args.len() != 3 {
        blockers.insert("index_second_arg_must_be_exact_match".to_string());
        return None;
    }

    let lookup_value = emit_sql_expression(
        &match_args[0],
        current_sheet_name,
        formula_reference,
        formula_row,
        defined_names,
        references,
        blockers,
    )?;
    let lookup_vector = resolve_lookup_vector(
        &match_args[1],
        current_sheet_name,
        defined_names,
        references,
        blockers,
        "match_lookup_array_not_proven_vector",
    )?;
    if !matches!(&match_args[2], FormulaAstNode::NumberLiteral { value } if value == "0") {
        blockers.insert("approximate_lookup_not_supported".to_string());
        return None;
    }

    let source_id = validate_aligned_lookup_vectors(
        &lookup_vector,
        &return_vector,
        blockers,
        "match_lookup_array_must_be_single_column",
        "index_return_array_must_be_single_column",
    )?;

    Some(format!(
        "(SELECT [lookup_return_1] FROM [{source_id}] WHERE [lookup_key_1] = {lookup_value})"
    ))
}

fn resolve_defined_name_range<'a>(
    node: &FormulaAstNode,
    current_sheet_name: &str,
    defined_names: &'a [DefinedNameSummary],
    references: &mut Vec<FormulaSqlReferenceSummary>,
    blockers: &mut BTreeSet<String>,
) -> Option<(&'a DefinedNameSummary, ResolvedDefinedNameRange)> {
    let FormulaAstNode::DefinedNameReference { name, sheet_name } = node else {
        blockers.insert("lookup_table_must_be_defined_name_range".to_string());
        return None;
    };
    push_reference(
        references,
        FormulaSqlReferenceSummary {
            kind: FormulaSqlReferenceKind::DefinedName,
            reference: name.clone(),
            sheet_name: sheet_name.clone(),
            same_row: None,
            sql_identifier: Some(format!(
                "lookup_{}",
                normalize_lookup_identifier(name.as_str())
            )),
        },
    );
    let Some(defined_name) = resolve_defined_name_reference(
        defined_names,
        current_sheet_name,
        name.as_str(),
        sheet_name.as_deref(),
    ) else {
        blockers.insert("lookup_defined_name_unresolved".to_string());
        return None;
    };
    let Some(defined_name_range) = parse_defined_name_range_target(defined_name) else {
        blockers.insert("lookup_defined_name_target_not_proven_range".to_string());
        return None;
    };
    push_reference(
        references,
        FormulaSqlReferenceSummary {
            kind: FormulaSqlReferenceKind::Range,
            reference: format!(
                "{}:{}",
                defined_name_range.start_reference, defined_name_range.end_reference
            ),
            sheet_name: Some(defined_name_range.sheet_name.clone()),
            same_row: None,
            sql_identifier: None,
        },
    );
    Some((defined_name, defined_name_range))
}

fn resolve_lookup_vector(
    node: &FormulaAstNode,
    current_sheet_name: &str,
    defined_names: &[DefinedNameSummary],
    references: &mut Vec<FormulaSqlReferenceSummary>,
    blockers: &mut BTreeSet<String>,
    blocker_reason: &str,
) -> Option<ResolvedLookupVector> {
    match node {
        FormulaAstNode::RangeReference {
            start_reference,
            end_reference,
            sheet_name,
        } => {
            let (start_column_letters, start_row) = parse_a1_reference(start_reference.as_str())?;
            let (end_column_letters, end_row) = parse_a1_reference(end_reference.as_str())?;
            let resolved_sheet_name = sheet_name
                .clone()
                .unwrap_or_else(|| current_sheet_name.to_string());
            push_reference(
                references,
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: format!("{start_reference}:{end_reference}"),
                    sheet_name: Some(resolved_sheet_name.clone()),
                    same_row: None,
                    sql_identifier: None,
                },
            );
            Some(ResolvedLookupVector {
                sheet_name: resolved_sheet_name,
                start_reference: start_reference.clone(),
                end_reference: end_reference.clone(),
                start_column: column_letters_to_index(start_column_letters.as_str())?,
                end_column: column_letters_to_index(end_column_letters.as_str())?,
                start_row,
                end_row,
            })
        }
        FormulaAstNode::DefinedNameReference { name, sheet_name } => {
            push_reference(
                references,
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::DefinedName,
                    reference: name.clone(),
                    sheet_name: sheet_name.clone(),
                    same_row: None,
                    sql_identifier: Some(format!(
                        "lookup_{}",
                        normalize_lookup_identifier(name.as_str())
                    )),
                },
            );
            let Some(defined_name) = resolve_defined_name_reference(
                defined_names,
                current_sheet_name,
                name.as_str(),
                sheet_name.as_deref(),
            ) else {
                blockers.insert("lookup_defined_name_unresolved".to_string());
                return None;
            };
            let Some(parsed_range) = parse_defined_name_range_target(defined_name) else {
                blockers.insert("lookup_defined_name_target_not_proven_range".to_string());
                return None;
            };
            push_reference(
                references,
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: format!(
                        "{}:{}",
                        parsed_range.start_reference, parsed_range.end_reference
                    ),
                    sheet_name: Some(parsed_range.sheet_name.clone()),
                    same_row: None,
                    sql_identifier: None,
                },
            );
            Some(ResolvedLookupVector {
                sheet_name: parsed_range.sheet_name,
                start_reference: parsed_range.start_reference,
                end_reference: parsed_range.end_reference,
                start_column: parsed_range.start_column,
                end_column: parsed_range.end_column,
                start_row: parsed_range.start_row,
                end_row: parsed_range.end_row,
            })
        }
        _ => {
            blockers.insert(blocker_reason.to_string());
            None
        }
    }
}

fn validate_aligned_lookup_vectors(
    lookup_vector: &ResolvedLookupVector,
    return_vector: &ResolvedLookupVector,
    blockers: &mut BTreeSet<String>,
    lookup_width_reason: &str,
    return_width_reason: &str,
) -> Option<String> {
    if lookup_vector.width() != 1 {
        blockers.insert(lookup_width_reason.to_string());
        return None;
    }
    if return_vector.width() != 1 {
        blockers.insert(return_width_reason.to_string());
        return None;
    }
    if lookup_vector.sheet_name != return_vector.sheet_name {
        blockers.insert("lookup_vectors_not_same_sheet".to_string());
        return None;
    }
    if lookup_vector.start_row != return_vector.start_row
        || lookup_vector.end_row != return_vector.end_row
    {
        blockers.insert("lookup_vectors_not_aligned".to_string());
        return None;
    }
    Some(format!(
        "lookup_pair_{}_{}_{}_{}_{}",
        normalize_lookup_identifier(lookup_vector.sheet_name.as_str()),
        normalize_lookup_identifier(lookup_vector.start_reference.as_str()),
        normalize_lookup_identifier(lookup_vector.end_reference.as_str()),
        normalize_lookup_identifier(return_vector.start_reference.as_str()),
        normalize_lookup_identifier(return_vector.end_reference.as_str()),
    ))
}

fn parse_vlookup_column_index(
    node: &FormulaAstNode,
    blockers: &mut BTreeSet<String>,
) -> Option<u32> {
    let FormulaAstNode::NumberLiteral { value } = node else {
        blockers.insert("lookup_column_index_not_literal".to_string());
        return None;
    };
    let Ok(parsed) = value.parse::<u32>() else {
        blockers.insert("lookup_column_index_not_literal".to_string());
        return None;
    };
    Some(parsed)
}

fn is_exact_vlookup_match(node: &FormulaAstNode) -> bool {
    match node {
        FormulaAstNode::BooleanLiteral { value } => !value,
        FormulaAstNode::NumberLiteral { value } => value == "0",
        _ => false,
    }
}

fn push_reference(
    references: &mut Vec<FormulaSqlReferenceSummary>,
    reference: FormulaSqlReferenceSummary,
) {
    if references.len() < MAX_SQL_REFERENCES {
        references.push(reference);
    }
}

fn same_reference(left: &str, right: &str) -> bool {
    parse_a1_reference(left) == parse_a1_reference(right)
}

#[derive(Clone)]
struct ResolvedDefinedNameRange {
    sheet_name: String,
    start_reference: String,
    end_reference: String,
    start_column: u32,
    end_column: u32,
    start_row: u32,
    end_row: u32,
}

#[derive(Clone)]
struct ResolvedLookupVector {
    sheet_name: String,
    start_reference: String,
    end_reference: String,
    start_column: u32,
    end_column: u32,
    start_row: u32,
    end_row: u32,
}

impl ResolvedLookupVector {
    fn width(&self) -> u32 {
        self.end_column.saturating_sub(self.start_column) + 1
    }
}

impl AggregateFunction {
    fn supports_arity(self, arg_len: usize) -> bool {
        match self {
            Self::Count => arg_len >= 2 && arg_len.is_multiple_of(2),
            Self::Sum | Self::Average | Self::Max | Self::Min => arg_len >= 3 && arg_len % 2 == 1,
        }
    }

    fn requires_value_range(self) -> bool {
        !matches!(self, Self::Count)
    }

    fn arity_blocker(self) -> &'static str {
        match self {
            Self::Sum => "sumifs_arity_not_supported",
            Self::Count => "countifs_arity_not_supported",
            Self::Average => "averageifs_arity_not_supported",
            Self::Max => "maxifs_arity_not_supported",
            Self::Min => "minifs_arity_not_supported",
        }
    }

    fn sql_aggregate(self) -> &'static str {
        match self {
            Self::Sum => "SUM([aggregate_value_1])",
            Self::Count => "COUNT(*)",
            Self::Average => "AVG([aggregate_value_1])",
            Self::Max => "MAX([aggregate_value_1])",
            Self::Min => "MIN([aggregate_value_1])",
        }
    }
}

fn validate_aligned_aggregate_vectors(
    value_vector: Option<&ResolvedLookupVector>,
    criteria: &[(usize, ResolvedLookupVector, String)],
    blockers: &mut BTreeSet<String>,
) -> Option<()> {
    let base_vector = if let Some(value_vector) = value_vector {
        if value_vector.width() != 1 {
            blockers.insert("aggregate_value_range_must_be_single_column".to_string());
            return None;
        }
        value_vector
    } else {
        let (_, base_vector, _) = criteria.first()?;
        if base_vector.width() != 1 {
            blockers.insert("aggregate_criteria_range_must_be_single_column".to_string());
            return None;
        }
        base_vector
    };

    for (_, criteria_vector, _) in criteria {
        if criteria_vector.width() != 1 {
            blockers.insert("aggregate_criteria_range_must_be_single_column".to_string());
            return None;
        }
        if criteria_vector.sheet_name != base_vector.sheet_name {
            blockers.insert("aggregate_ranges_not_same_sheet".to_string());
            return None;
        }
        if criteria_vector.start_row != base_vector.start_row
            || criteria_vector.end_row != base_vector.end_row
        {
            blockers.insert("aggregate_ranges_not_same_grain".to_string());
            return None;
        }
    }

    Some(())
}

fn build_aggregate_source_id(
    base_vector: &ResolvedLookupVector,
    criteria: &[(usize, ResolvedLookupVector, String)],
) -> String {
    let mut parts = vec![
        normalize_lookup_identifier(base_vector.sheet_name.as_str()),
        normalize_lookup_identifier(base_vector.start_reference.as_str()),
        normalize_lookup_identifier(base_vector.end_reference.as_str()),
    ];
    for (_, criteria_vector, _) in criteria {
        parts.push(normalize_lookup_identifier(
            criteria_vector.start_reference.as_str(),
        ));
        parts.push(normalize_lookup_identifier(
            criteria_vector.end_reference.as_str(),
        ));
    }
    format!("aggregate_source_{}", parts.join("_"))
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

fn parse_defined_name_range_target(
    defined_name: &DefinedNameSummary,
) -> Option<ResolvedDefinedNameRange> {
    let target_parse =
        crate::formula_ast::parse_formula_ast(defined_name.target.as_str(), defined_name.truncated);
    let FormulaAstNode::RangeReference {
        start_reference,
        end_reference,
        sheet_name,
    } = target_parse.root.as_ref()?
    else {
        return None;
    };
    let sheet_name = sheet_name
        .clone()
        .or_else(|| defined_name.sheet_scope.clone())?;
    let (start_column, start_row) = parse_a1_reference(start_reference.as_str())?;
    let (end_column, end_row) = parse_a1_reference(end_reference.as_str())?;
    Some(ResolvedDefinedNameRange {
        sheet_name,
        start_reference: start_reference.clone(),
        end_reference: end_reference.clone(),
        start_column: column_letters_to_index(start_column.as_str())?,
        end_column: column_letters_to_index(end_column.as_str())?,
        start_row,
        end_row,
    })
}

fn column_letters_to_index(column: &str) -> Option<u32> {
    let mut value = 0u32;
    for ch in column.chars() {
        if !ch.is_ascii_alphabetic() {
            return None;
        }
        value = value
            .checked_mul(26)?
            .checked_add(u32::from(ch.to_ascii_uppercase() as u8 - b'A' + 1))?;
    }
    Some(value)
}

fn normalize_lookup_identifier(value: &str) -> String {
    let mut normalized = String::new();
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
        } else if !normalized.ends_with('_') {
            normalized.push('_');
        }
    }
    normalized.trim_matches('_').to_string()
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
