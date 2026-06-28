use pretty_assertions::assert_eq;

use crate::formula_ast::FormulaAstBinaryOperator;
use crate::formula_ast::FormulaAstNode;
use crate::formula_ast::FormulaAstParseState;
use crate::formula_ast::FormulaAstSummary;
use crate::formula_sql::plan_formula_sql_preview;
use crate::tool::DefinedNameSummary;
use crate::tool::FormulaSqlPreviewState;
use crate::tool::FormulaSqlReferenceKind;
use crate::tool::FormulaSqlReferenceSummary;
use crate::tool::SheetFormulaSummary;

fn formula_summary(
    reference: &str,
    formula: &str,
    cached_value: Option<&str>,
    parse: FormulaAstSummary,
) -> SheetFormulaSummary {
    SheetFormulaSummary {
        reference: reference.to_string(),
        formula: formula.to_string(),
        cached_value: cached_value.map(ToOwned::to_owned),
        parse,
        sql_preview: Default::default(),
        warnings: Vec::new(),
        formula_type: None,
        shared_index: None,
        shared_range: None,
        style_index: None,
        number_format_id: None,
        number_format_code: None,
    }
}

#[test]
fn formula_sql_preview_supports_same_row_scalar_arithmetic() {
    let formula = formula_summary(
        "D2",
        "A2+B2*2",
        Some("19"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::BinaryOperation {
                operator: FormulaAstBinaryOperator::Add,
                left: Box::new(FormulaAstNode::CellReference {
                    reference: "A2".to_string(),
                    sheet_name: None,
                }),
                right: Box::new(FormulaAstNode::BinaryOperation {
                    operator: FormulaAstBinaryOperator::Multiply,
                    left: Box::new(FormulaAstNode::CellReference {
                        reference: "B2".to_string(),
                        sheet_name: None,
                    }),
                    right: Box::new(FormulaAstNode::NumberLiteral {
                        value: "2".to_string(),
                    }),
                }),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &[], false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::ReviewOnly,
            sql_expression: Some("([col_a] + ([col_b] * 2))".to_string()),
            references: vec![
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Cell,
                    reference: "A2".to_string(),
                    sheet_name: None,
                    same_row: Some(true),
                    sql_identifier: Some("col_a".to_string()),
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Cell,
                    reference: "B2".to_string(),
                    sheet_name: None,
                    same_row: Some(true),
                    sql_identifier: Some("col_b".to_string()),
                },
            ],
            blocker_reasons: Vec::new(),
            cached_value_present: true,
        }
    );
}

#[test]
fn formula_sql_preview_blocks_cross_row_references() {
    let formula = formula_summary(
        "D2",
        "A1+1",
        Some("8"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::BinaryOperation {
                operator: FormulaAstBinaryOperator::Add,
                left: Box::new(FormulaAstNode::CellReference {
                    reference: "A1".to_string(),
                    sheet_name: None,
                }),
                right: Box::new(FormulaAstNode::NumberLiteral {
                    value: "1".to_string(),
                }),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &[], false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::Blocked,
            sql_expression: None,
            references: vec![FormulaSqlReferenceSummary {
                kind: FormulaSqlReferenceKind::Cell,
                reference: "A1".to_string(),
                sheet_name: None,
                same_row: Some(false),
                sql_identifier: Some("col_a".to_string()),
            }],
            blocker_reasons: vec!["cross_row_reference_not_supported_in_scalar_phase".to_string()],
            cached_value_present: true,
        }
    );
}

#[test]
fn formula_sql_preview_blocks_function_calls_and_ranges() {
    let formula = formula_summary(
        "D2",
        "SUM(A2:B2)",
        Some("7"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "SUM".to_string(),
                args: vec![FormulaAstNode::RangeReference {
                    start_reference: "A2".to_string(),
                    end_reference: "B2".to_string(),
                    sheet_name: None,
                }],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &[], false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::Blocked,
            sql_expression: None,
            references: vec![FormulaSqlReferenceSummary {
                kind: FormulaSqlReferenceKind::Range,
                reference: "A2:B2".to_string(),
                sheet_name: None,
                same_row: None,
                sql_identifier: None,
            }],
            blocker_reasons: vec![
                "function_call_not_supported_in_scalar_phase".to_string(),
                "range_reference_not_supported_in_scalar_phase".to_string(),
            ],
            cached_value_present: true,
        }
    );
}

#[test]
fn formula_sql_preview_blocks_workbooks_with_external_links() {
    let formula = formula_summary(
        "D2",
        "A2+1",
        None,
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::BinaryOperation {
                operator: FormulaAstBinaryOperator::Add,
                left: Box::new(FormulaAstNode::CellReference {
                    reference: "A2".to_string(),
                    sheet_name: None,
                }),
                right: Box::new(FormulaAstNode::NumberLiteral {
                    value: "1".to_string(),
                }),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &[], true),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::Blocked,
            sql_expression: None,
            references: vec![FormulaSqlReferenceSummary {
                kind: FormulaSqlReferenceKind::Cell,
                reference: "A2".to_string(),
                sheet_name: None,
                same_row: Some(true),
                sql_identifier: Some("col_a".to_string()),
            }],
            blocker_reasons: vec!["workbook_has_external_links".to_string()],
            cached_value_present: false,
        }
    );
}

#[test]
fn formula_sql_preview_supports_exact_vlookup_over_defined_name_range() {
    let formula = formula_summary(
        "D2",
        "VLOOKUP(C2,KPI_Name,2,FALSE)",
        Some("North"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "VLOOKUP".to_string(),
                args: vec![
                    FormulaAstNode::CellReference {
                        reference: "C2".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::DefinedNameReference {
                        name: "KPI_Name".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::NumberLiteral {
                        value: "2".to_string(),
                    },
                    FormulaAstNode::BooleanLiteral { value: false },
                ],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );
    let defined_names = vec![DefinedNameSummary {
        name: "KPI_Name".to_string(),
        sheet_scope: None,
        local_sheet_id: None,
        hidden: None,
        target: "Lookup!$A$2:$B$4".to_string(),
        truncated: false,
    }];

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &defined_names, false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::ReviewOnly,
            sql_expression: Some(
                "(SELECT [lookup_col_2] FROM [lookup_kpi_name] WHERE [lookup_col_1] = [col_c])"
                    .to_string()
            ),
            references: vec![
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Cell,
                    reference: "C2".to_string(),
                    sheet_name: None,
                    same_row: Some(true),
                    sql_identifier: Some("col_c".to_string()),
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::DefinedName,
                    reference: "KPI_Name".to_string(),
                    sheet_name: None,
                    same_row: None,
                    sql_identifier: Some("lookup_kpi_name".to_string()),
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "$A$2:$B$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
            ],
            blocker_reasons: Vec::new(),
            cached_value_present: true,
        }
    );
}

#[test]
fn formula_sql_preview_blocks_approximate_vlookup() {
    let formula = formula_summary(
        "D2",
        "VLOOKUP(C2,KPI_Name,2,TRUE)",
        Some("North"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "VLOOKUP".to_string(),
                args: vec![
                    FormulaAstNode::CellReference {
                        reference: "C2".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::DefinedNameReference {
                        name: "KPI_Name".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::NumberLiteral {
                        value: "2".to_string(),
                    },
                    FormulaAstNode::BooleanLiteral { value: true },
                ],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );
    let defined_names = vec![DefinedNameSummary {
        name: "KPI_Name".to_string(),
        sheet_scope: None,
        local_sheet_id: None,
        hidden: None,
        target: "Lookup!$A$2:$B$4".to_string(),
        truncated: false,
    }];

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &defined_names, false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::Blocked,
            sql_expression: None,
            references: vec![
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Cell,
                    reference: "C2".to_string(),
                    sheet_name: None,
                    same_row: Some(true),
                    sql_identifier: Some("col_c".to_string()),
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::DefinedName,
                    reference: "KPI_Name".to_string(),
                    sheet_name: None,
                    same_row: None,
                    sql_identifier: Some("lookup_kpi_name".to_string()),
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "$A$2:$B$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
            ],
            blocker_reasons: vec!["approximate_lookup_not_supported".to_string()],
            cached_value_present: true,
        }
    );
}

#[test]
fn formula_sql_preview_supports_exact_xlookup_over_aligned_ranges() {
    let formula = formula_summary(
        "D2",
        "XLOOKUP(C2,A2:A4,B2:B4)",
        Some("North"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "XLOOKUP".to_string(),
                args: vec![
                    FormulaAstNode::CellReference {
                        reference: "C2".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::RangeReference {
                        start_reference: "A2".to_string(),
                        end_reference: "A4".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::RangeReference {
                        start_reference: "B2".to_string(),
                        end_reference: "B4".to_string(),
                        sheet_name: None,
                    },
                ],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &[], false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::ReviewOnly,
            sql_expression: Some(
                "(SELECT [lookup_return_1] FROM [lookup_pair_summary_a2_a4_b2_b4] WHERE [lookup_key_1] = [col_c])"
                    .to_string()
            ),
            references: vec![
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Cell,
                    reference: "C2".to_string(),
                    sheet_name: None,
                    same_row: Some(true),
                    sql_identifier: Some("col_c".to_string()),
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "A2:A4".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "B2:B4".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
            ],
            blocker_reasons: Vec::new(),
            cached_value_present: true,
        }
    );
}

#[test]
fn formula_sql_preview_blocks_reverse_xlookup_modes() {
    let formula = formula_summary(
        "D2",
        "XLOOKUP(C2,A2:A4,B2:B4,,0,-1)",
        Some("North"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "XLOOKUP".to_string(),
                args: vec![
                    FormulaAstNode::CellReference {
                        reference: "C2".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::RangeReference {
                        start_reference: "A2".to_string(),
                        end_reference: "A4".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::RangeReference {
                        start_reference: "B2".to_string(),
                        end_reference: "B4".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::BlankArgument,
                    FormulaAstNode::NumberLiteral {
                        value: "0".to_string(),
                    },
                    FormulaAstNode::UnaryOperation {
                        operator: crate::formula_ast::FormulaAstUnaryOperator::Minus,
                        operand: Box::new(FormulaAstNode::NumberLiteral {
                            value: "1".to_string(),
                        }),
                    },
                ],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &[], false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::Blocked,
            sql_expression: None,
            references: vec![
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Cell,
                    reference: "C2".to_string(),
                    sheet_name: None,
                    same_row: Some(true),
                    sql_identifier: Some("col_c".to_string()),
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "A2:A4".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "B2:B4".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
            ],
            blocker_reasons: vec!["xlookup_search_mode_not_supported".to_string()],
            cached_value_present: true,
        }
    );
}

#[test]
fn formula_sql_preview_supports_exact_index_match_over_aligned_ranges() {
    let formula = formula_summary(
        "D2",
        "INDEX(B2:B4,MATCH(C2,A2:A4,0))",
        Some("North"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "INDEX".to_string(),
                args: vec![
                    FormulaAstNode::RangeReference {
                        start_reference: "B2".to_string(),
                        end_reference: "B4".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::FunctionCall {
                        name: "MATCH".to_string(),
                        args: vec![
                            FormulaAstNode::CellReference {
                                reference: "C2".to_string(),
                                sheet_name: None,
                            },
                            FormulaAstNode::RangeReference {
                                start_reference: "A2".to_string(),
                                end_reference: "A4".to_string(),
                                sheet_name: None,
                            },
                            FormulaAstNode::NumberLiteral {
                                value: "0".to_string(),
                            },
                        ],
                    },
                ],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &[], false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::ReviewOnly,
            sql_expression: Some(
                "(SELECT [lookup_return_1] FROM [lookup_pair_summary_a2_a4_b2_b4] WHERE [lookup_key_1] = [col_c])"
                    .to_string()
            ),
            references: vec![
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "B2:B4".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Cell,
                    reference: "C2".to_string(),
                    sheet_name: None,
                    same_row: Some(true),
                    sql_identifier: Some("col_c".to_string()),
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "A2:A4".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
            ],
            blocker_reasons: Vec::new(),
            cached_value_present: true,
        }
    );
}

#[test]
fn formula_sql_preview_supports_aligned_sumifs_with_multiple_criteria() {
    let formula = formula_summary(
        "E2",
        "SUMIFS(Lookup!$B$2:$B$4,Lookup!$A$2:$A$4,C2,Lookup!$C$2:$C$4,D2)",
        Some("10"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "SUMIFS".to_string(),
                args: vec![
                    FormulaAstNode::RangeReference {
                        start_reference: "$B$2".to_string(),
                        end_reference: "$B$4".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                    },
                    FormulaAstNode::RangeReference {
                        start_reference: "$A$2".to_string(),
                        end_reference: "$A$4".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                    },
                    FormulaAstNode::CellReference {
                        reference: "C2".to_string(),
                        sheet_name: None,
                    },
                    FormulaAstNode::RangeReference {
                        start_reference: "$C$2".to_string(),
                        end_reference: "$C$4".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                    },
                    FormulaAstNode::CellReference {
                        reference: "D2".to_string(),
                        sheet_name: None,
                    },
                ],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &[], false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::ReviewOnly,
            sql_expression: Some(
                "(SELECT SUM([aggregate_value_1]) FROM [aggregate_source_lookup_b_2_b_4_a_2_a_4_c_2_c_4] WHERE [criteria_col_1] = [col_c] AND [criteria_col_2] = [col_d])"
                    .to_string()
            ),
            references: vec![
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "$B$2:$B$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "$A$2:$A$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Cell,
                    reference: "C2".to_string(),
                    sheet_name: None,
                    same_row: Some(true),
                    sql_identifier: Some("col_c".to_string()),
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "$C$2:$C$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Cell,
                    reference: "D2".to_string(),
                    sheet_name: None,
                    same_row: Some(true),
                    sql_identifier: Some("col_d".to_string()),
                },
            ],
            blocker_reasons: Vec::new(),
            cached_value_present: true,
        }
    );
}

#[test]
fn formula_sql_preview_supports_other_aligned_aggregate_functions() {
    let cases = vec![
        (
            "COUNTIFS".to_string(),
            "COUNTIFS(Lookup!$A$2:$A$4,C2)".to_string(),
            "F2".to_string(),
            Some("2"),
            vec![
                FormulaAstNode::RangeReference {
                    start_reference: "$A$2".to_string(),
                    end_reference: "$A$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                },
                FormulaAstNode::CellReference {
                    reference: "C2".to_string(),
                    sheet_name: None,
                },
            ],
            "(SELECT COUNT(*) FROM [aggregate_source_lookup_a_2_a_4_a_2_a_4] WHERE [criteria_col_1] = [col_c])"
                .to_string(),
        ),
        (
            "AVERAGEIFS".to_string(),
            "AVERAGEIFS(Lookup!$B$2:$B$4,Lookup!$A$2:$A$4,C2)".to_string(),
            "G2".to_string(),
            Some("5"),
            vec![
                FormulaAstNode::RangeReference {
                    start_reference: "$B$2".to_string(),
                    end_reference: "$B$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                },
                FormulaAstNode::RangeReference {
                    start_reference: "$A$2".to_string(),
                    end_reference: "$A$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                },
                FormulaAstNode::CellReference {
                    reference: "C2".to_string(),
                    sheet_name: None,
                },
            ],
            "(SELECT AVG([aggregate_value_1]) FROM [aggregate_source_lookup_b_2_b_4_a_2_a_4] WHERE [criteria_col_1] = [col_c])"
                .to_string(),
        ),
        (
            "MAXIFS".to_string(),
            "MAXIFS(Lookup!$B$2:$B$4,Lookup!$A$2:$A$4,C2)".to_string(),
            "H2".to_string(),
            Some("7"),
            vec![
                FormulaAstNode::RangeReference {
                    start_reference: "$B$2".to_string(),
                    end_reference: "$B$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                },
                FormulaAstNode::RangeReference {
                    start_reference: "$A$2".to_string(),
                    end_reference: "$A$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                },
                FormulaAstNode::CellReference {
                    reference: "C2".to_string(),
                    sheet_name: None,
                },
            ],
            "(SELECT MAX([aggregate_value_1]) FROM [aggregate_source_lookup_b_2_b_4_a_2_a_4] WHERE [criteria_col_1] = [col_c])"
                .to_string(),
        ),
        (
            "MINIFS".to_string(),
            "MINIFS(Lookup!$B$2:$B$4,Lookup!$A$2:$A$4,C2)".to_string(),
            "I2".to_string(),
            Some("3"),
            vec![
                FormulaAstNode::RangeReference {
                    start_reference: "$B$2".to_string(),
                    end_reference: "$B$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                },
                FormulaAstNode::RangeReference {
                    start_reference: "$A$2".to_string(),
                    end_reference: "$A$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                },
                FormulaAstNode::CellReference {
                    reference: "C2".to_string(),
                    sheet_name: None,
                },
            ],
            "(SELECT MIN([aggregate_value_1]) FROM [aggregate_source_lookup_b_2_b_4_a_2_a_4] WHERE [criteria_col_1] = [col_c])"
                .to_string(),
        ),
    ];

    for (name, formula_text, reference, cached_value, args, expected_sql) in cases {
        let formula = formula_summary(
            reference.as_str(),
            formula_text.as_str(),
            cached_value,
            FormulaAstSummary {
                state: FormulaAstParseState::Parsed,
                root: Some(FormulaAstNode::FunctionCall { name, args }),
                diagnostics: Vec::new(),
                unsupported_reasons: Vec::new(),
                truncated: false,
            },
        );

        let preview = plan_formula_sql_preview("Summary", &formula, &[], false);
        assert_eq!(preview.state, FormulaSqlPreviewState::ReviewOnly);
        assert_eq!(preview.sql_expression, Some(expected_sql));
        assert_eq!(preview.blocker_reasons, Vec::<String>::new());
    }
}

#[test]
fn formula_sql_preview_blocks_aggregate_ranges_without_same_grain() {
    let formula = formula_summary(
        "E2",
        "SUMIFS(Lookup!$B$2:$B$4,Lookup!$A$2:$A$3,C2)",
        Some("10"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "SUMIFS".to_string(),
                args: vec![
                    FormulaAstNode::RangeReference {
                        start_reference: "$B$2".to_string(),
                        end_reference: "$B$4".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                    },
                    FormulaAstNode::RangeReference {
                        start_reference: "$A$2".to_string(),
                        end_reference: "$A$3".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                    },
                    FormulaAstNode::CellReference {
                        reference: "C2".to_string(),
                        sheet_name: None,
                    },
                ],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &[], false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::Blocked,
            sql_expression: None,
            references: vec![
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "$B$2:$B$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "$A$2:$A$3".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Cell,
                    reference: "C2".to_string(),
                    sheet_name: None,
                    same_row: Some(true),
                    sql_identifier: Some("col_c".to_string()),
                },
            ],
            blocker_reasons: vec!["aggregate_ranges_not_same_grain".to_string()],
            cached_value_present: true,
        }
    );
}

#[test]
fn formula_sql_preview_blocks_operator_string_criteria() {
    let formula = formula_summary(
        "E2",
        r#"SUMIFS(Lookup!$B$2:$B$4,Lookup!$A$2:$A$4,">=n")"#,
        Some("10"),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "SUMIFS".to_string(),
                args: vec![
                    FormulaAstNode::RangeReference {
                        start_reference: "$B$2".to_string(),
                        end_reference: "$B$4".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                    },
                    FormulaAstNode::RangeReference {
                        start_reference: "$A$2".to_string(),
                        end_reference: "$A$4".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                    },
                    FormulaAstNode::StringLiteral {
                        value: ">=n".to_string(),
                    },
                ],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        },
    );

    assert_eq!(
        plan_formula_sql_preview("Summary", &formula, &[], false),
        crate::tool::FormulaSqlPreviewSummary {
            state: FormulaSqlPreviewState::Blocked,
            sql_expression: None,
            references: vec![
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "$B$2:$B$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
                FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::Range,
                    reference: "$A$2:$A$4".to_string(),
                    sheet_name: Some("Lookup".to_string()),
                    same_row: None,
                    sql_identifier: None,
                },
            ],
            blocker_reasons: vec!["aggregate_criteria_operator_string_not_supported".to_string()],
            cached_value_present: true,
        }
    );
}
