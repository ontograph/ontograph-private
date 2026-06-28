use pretty_assertions::assert_eq;

use crate::formula_ast::FormulaAstBinaryOperator;
use crate::formula_ast::FormulaAstNode;
use crate::formula_ast::FormulaAstParseState;
use crate::formula_ast::FormulaAstSummary;
use crate::formula_ast::FormulaAstUnaryOperator;
use crate::formula_ast::parse_formula_ast;

#[test]
fn parse_formula_ast_respects_operator_precedence() {
    assert_eq!(
        parse_formula_ast("1+2*3", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::BinaryOperation {
                operator: FormulaAstBinaryOperator::Add,
                left: Box::new(FormulaAstNode::NumberLiteral {
                    value: "1".to_string(),
                }),
                right: Box::new(FormulaAstNode::BinaryOperation {
                    operator: FormulaAstBinaryOperator::Multiply,
                    left: Box::new(FormulaAstNode::NumberLiteral {
                        value: "2".to_string(),
                    }),
                    right: Box::new(FormulaAstNode::NumberLiteral {
                        value: "3".to_string(),
                    }),
                }),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_puts_exponent_above_unary_minus() {
    assert_eq!(
        parse_formula_ast("-1^2", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::UnaryOperation {
                operator: FormulaAstUnaryOperator::Minus,
                operand: Box::new(FormulaAstNode::BinaryOperation {
                    operator: FormulaAstBinaryOperator::Power,
                    left: Box::new(FormulaAstNode::NumberLiteral {
                        value: "1".to_string(),
                    }),
                    right: Box::new(FormulaAstNode::NumberLiteral {
                        value: "2".to_string(),
                    }),
                }),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_parses_core_literals() {
    assert_eq!(
        parse_formula_ast(".5", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::NumberLiteral {
                value: ".5".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
    assert_eq!(
        parse_formula_ast("1.", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::NumberLiteral {
                value: "1.".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
    assert_eq!(
        parse_formula_ast("\"a\"\"b\"", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::StringLiteral {
                value: "a\"b".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
    assert_eq!(
        parse_formula_ast("TRUE", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::BooleanLiteral { value: true }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
    assert_eq!(
        parse_formula_ast("#DIV/0!", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::ErrorLiteral {
                value: "#DIV/0!".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_keeps_concatenation_above_comparison() {
    assert_eq!(
        parse_formula_ast("A1&\"x\"=B1", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::BinaryOperation {
                operator: FormulaAstBinaryOperator::Equal,
                left: Box::new(FormulaAstNode::BinaryOperation {
                    operator: FormulaAstBinaryOperator::Concatenate,
                    left: Box::new(FormulaAstNode::CellReference {
                        reference: "A1".to_string(),
                        sheet_name: None,
                    }),
                    right: Box::new(FormulaAstNode::StringLiteral {
                        value: "x".to_string(),
                    }),
                }),
                right: Box::new(FormulaAstNode::CellReference {
                    reference: "B1".to_string(),
                    sheet_name: None,
                }),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_parses_percent_after_parenthesized_expression() {
    assert_eq!(
        parse_formula_ast("(1+2)%", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::Percent {
                operand: Box::new(FormulaAstNode::BinaryOperation {
                    operator: FormulaAstBinaryOperator::Add,
                    left: Box::new(FormulaAstNode::NumberLiteral {
                        value: "1".to_string(),
                    }),
                    right: Box::new(FormulaAstNode::NumberLiteral {
                        value: "2".to_string(),
                    }),
                }),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_binds_percent_tighter_than_power() {
    assert_eq!(
        parse_formula_ast("2^3%", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::BinaryOperation {
                operator: FormulaAstBinaryOperator::Power,
                left: Box::new(FormulaAstNode::NumberLiteral {
                    value: "2".to_string(),
                }),
                right: Box::new(FormulaAstNode::Percent {
                    operand: Box::new(FormulaAstNode::NumberLiteral {
                        value: "3".to_string(),
                    }),
                }),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_parses_function_calls_with_blank_arguments() {
    assert_eq!(
        parse_formula_ast("IF(,TRUE,FALSE)", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "IF".to_string(),
                args: vec![
                    FormulaAstNode::BlankArgument,
                    FormulaAstNode::BooleanLiteral { value: true },
                    FormulaAstNode::BooleanLiteral { value: false },
                ],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_parses_sheet_qualified_cells() {
    assert_eq!(
        parse_formula_ast("'Q1 Data'!$B$2", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::CellReference {
                reference: "$B$2".to_string(),
                sheet_name: Some("Q1 Data".to_string()),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_parses_sheet_qualified_ranges() {
    assert_eq!(
        parse_formula_ast("SUM('Q1 Data'!$A$1:$A$3)", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::FunctionCall {
                name: "SUM".to_string(),
                args: vec![FormulaAstNode::RangeReference {
                    start_reference: "$A$1".to_string(),
                    end_reference: "$A$3".to_string(),
                    sheet_name: Some("Q1 Data".to_string()),
                }],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_marks_array_constants_unsupported_in_phase_1a() {
    assert_eq!(
        parse_formula_ast("{1,2}", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Unsupported,
            root: Some(FormulaAstNode::Unsupported {
                reason: "array_constant".to_string(),
                text: "{1,2}".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: vec!["array_constant".to_string()],
            truncated: false,
        }
    );
    assert_eq!(
        parse_formula_ast("SUM({1,2,3})", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Unsupported,
            root: Some(FormulaAstNode::FunctionCall {
                name: "SUM".to_string(),
                args: vec![FormulaAstNode::Unsupported {
                    reason: "array_constant".to_string(),
                    text: "{1,2,3}".to_string(),
                }],
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: vec!["array_constant".to_string()],
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_parses_defined_name_references_in_phase_1b() {
    assert_eq!(
        parse_formula_ast("Threshold", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::DefinedNameReference {
                name: "Threshold".to_string(),
                sheet_name: None,
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
    assert_eq!(
        parse_formula_ast("'Summary'!Threshold", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Parsed,
            root: Some(FormulaAstNode::DefinedNameReference {
                name: "Threshold".to_string(),
                sheet_name: Some("Summary".to_string()),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_keeps_structured_references_explicitly_unsupported() {
    assert_eq!(
        parse_formula_ast("Table1[Amount]", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Unsupported,
            root: Some(FormulaAstNode::Unsupported {
                reason: "structured_reference".to_string(),
                text: "Table1[Amount]".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: vec!["structured_reference".to_string()],
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_marks_dynamic_array_functions_unsupported() {
    assert_eq!(
        parse_formula_ast("FILTER(A1:A3,A1:A3>0)", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Unsupported,
            root: Some(FormulaAstNode::Unsupported {
                reason: "dynamic_array_or_spill_marker".to_string(),
                text: "FILTER".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: vec!["dynamic_array_or_spill_marker".to_string()],
            truncated: false,
        }
    );
    assert_eq!(
        parse_formula_ast("_xlfn.FILTER(A1:A3,A1:A3>0)", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Unsupported,
            root: Some(FormulaAstNode::Unsupported {
                reason: "dynamic_array_or_spill_marker".to_string(),
                text: "_xlfn.FILTER".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: vec!["dynamic_array_or_spill_marker".to_string()],
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_marks_dynamic_arrays_unsupported() {
    assert_eq!(
        parse_formula_ast("FILTER(A1:A3,A1:A3>0)#", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Unsupported,
            root: Some(FormulaAstNode::Unsupported {
                reason: "dynamic_array_or_spill_marker".to_string(),
                text: "#".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: vec!["dynamic_array_or_spill_marker".to_string()],
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_marks_external_workbook_refs_unsupported() {
    assert_eq!(
        parse_formula_ast("[Book2.xlsx]Sheet1!A1", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Unsupported,
            root: Some(FormulaAstNode::Unsupported {
                reason: "external_workbook_reference".to_string(),
                text: "[Book2.xlsx]Sheet1!A1".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: vec!["external_workbook_reference".to_string()],
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_marks_r1c1_references_unsupported() {
    assert_eq!(
        parse_formula_ast("R1C1", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Unsupported,
            root: Some(FormulaAstNode::Unsupported {
                reason: "r1c1_reference".to_string(),
                text: "R1C1".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: vec!["r1c1_reference".to_string()],
            truncated: false,
        }
    );
    assert_eq!(
        parse_formula_ast("R[1]C[2]", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Unsupported,
            root: Some(FormulaAstNode::Unsupported {
                reason: "r1c1_reference".to_string(),
                text: "R[1]C[2]".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: vec!["r1c1_reference".to_string()],
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_does_not_treat_backslash_fragments_as_defined_names() {
    assert_eq!(
        parse_formula_ast("\\BadRef", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Unsupported,
            root: Some(FormulaAstNode::Unsupported {
                reason: "unclassified_fragment".to_string(),
                text: "\\BadRef".to_string(),
            }),
            diagnostics: Vec::new(),
            unsupported_reasons: vec!["unclassified_fragment".to_string()],
            truncated: false,
        }
    );
}

#[test]
fn parse_formula_ast_reports_malformed_formulas() {
    assert_eq!(
        parse_formula_ast("SUM(", false),
        FormulaAstSummary {
            state: FormulaAstParseState::Malformed,
            root: None,
            diagnostics: vec!["unexpected token end of formula".to_string()],
            unsupported_reasons: Vec::new(),
            truncated: false,
        }
    );
}
