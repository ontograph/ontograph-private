use std::sync::LazyLock;

use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::preview::bounded_text;

const MAX_AST_NODES: usize = 128;
const MAX_DIAGNOSTICS: usize = 8;
const MAX_REASON_TEXT_CHARS: usize = 96;
const MAX_NODE_TEXT_CHARS: usize = 128;

static CELL_REF_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\$?[A-Za-z]{1,3}\$?[1-9][0-9]{0,6}$")
        .unwrap_or_else(|err| panic!("valid cell ref regex: {err}"))
});
static NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[Ee][+-]?[0-9]+)?$")
        .unwrap_or_else(|err| panic!("valid number regex: {err}"))
});
static NAME_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Za-z_][A-Za-z0-9_.]*$").unwrap_or_else(|err| panic!("valid name regex: {err}"))
});
static R1C1_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^R(?:\[-?[0-9]+\]|[0-9]+)?C(?:\[-?[0-9]+\]|[0-9]+)?$")
        .unwrap_or_else(|err| panic!("valid r1c1 regex: {err}"))
});

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FormulaAstParseState {
    Missing,
    Parsed,
    Unsupported,
    Malformed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct FormulaAstSummary {
    pub state: FormulaAstParseState,
    pub root: Option<FormulaAstNode>,
    pub diagnostics: Vec<String>,
    pub unsupported_reasons: Vec<String>,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FormulaAstUnaryOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FormulaAstBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Concatenate,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum FormulaAstNode {
    NumberLiteral {
        value: String,
    },
    StringLiteral {
        value: String,
    },
    BooleanLiteral {
        value: bool,
    },
    BlankArgument,
    ErrorLiteral {
        value: String,
    },
    UnaryOperation {
        operator: FormulaAstUnaryOperator,
        operand: Box<FormulaAstNode>,
    },
    BinaryOperation {
        operator: FormulaAstBinaryOperator,
        left: Box<FormulaAstNode>,
        right: Box<FormulaAstNode>,
    },
    Percent {
        operand: Box<FormulaAstNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<FormulaAstNode>,
    },
    CellReference {
        reference: String,
        sheet_name: Option<String>,
    },
    RangeReference {
        start_reference: String,
        end_reference: String,
        sheet_name: Option<String>,
    },
    DefinedNameReference {
        name: String,
        sheet_name: Option<String>,
    },
    Unsupported {
        reason: String,
        text: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Fragment(String),
    StringLiteral(String),
    LParen,
    RParen,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Amp,
    Percent,
    SpillMarker,
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    Eof,
}

pub(crate) fn parse_formula_ast(formula: &str, was_truncated: bool) -> FormulaAstSummary {
    if formula.is_empty() {
        return FormulaAstSummary {
            state: FormulaAstParseState::Missing,
            root: None,
            diagnostics: Vec::new(),
            unsupported_reasons: Vec::new(),
            truncated: was_truncated,
        };
    }

    let tokens = match tokenize(formula) {
        Ok(tokens) => tokens,
        Err(err) => {
            return FormulaAstSummary {
                state: FormulaAstParseState::Malformed,
                root: None,
                diagnostics: vec![bounded_text(&err, MAX_REASON_TEXT_CHARS)],
                unsupported_reasons: Vec::new(),
                truncated: was_truncated,
            };
        }
    };
    let mut parser = Parser::new(tokens);
    let root = match parser.parse_expression(0) {
        Ok(root) => root,
        Err(err) => {
            return FormulaAstSummary {
                state: FormulaAstParseState::Malformed,
                root: None,
                diagnostics: vec![bounded_text(&err, MAX_REASON_TEXT_CHARS)],
                unsupported_reasons: Vec::new(),
                truncated: was_truncated,
            };
        }
    };
    if parser.peek() != &Token::Eof {
        parser.push_diagnostic(format!(
            "unexpected trailing token {}",
            parser.describe_token(parser.peek())
        ));
        return FormulaAstSummary {
            state: FormulaAstParseState::Malformed,
            root: None,
            diagnostics: parser.diagnostics,
            unsupported_reasons: Vec::new(),
            truncated: was_truncated,
        };
    }

    let mut unsupported_reasons = Vec::new();
    collect_unsupported_reasons(&root, &mut unsupported_reasons);
    unsupported_reasons.dedup();
    FormulaAstSummary {
        state: if unsupported_reasons.is_empty() {
            FormulaAstParseState::Parsed
        } else {
            FormulaAstParseState::Unsupported
        },
        root: Some(root),
        diagnostics: parser.diagnostics,
        unsupported_reasons,
        truncated: was_truncated,
    }
}

fn tokenize(formula: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars = formula.chars().collect::<Vec<_>>();
    let mut index = 0;
    while index < chars.len() {
        match chars[index] {
            ' ' | '\t' | '\r' | '\n' => index += 1,
            '(' => {
                tokens.push(Token::LParen);
                index += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                index += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                index += 1;
            }
            '+' => {
                tokens.push(Token::Plus);
                index += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                index += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                index += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                index += 1;
            }
            '^' => {
                tokens.push(Token::Caret);
                index += 1;
            }
            '&' => {
                tokens.push(Token::Amp);
                index += 1;
            }
            '%' => {
                tokens.push(Token::Percent);
                index += 1;
            }
            '#' => {
                let (fragment, next_index) = parse_hash_fragment(&chars, index);
                if fragment.len() == 1 {
                    tokens.push(Token::SpillMarker);
                } else {
                    tokens.push(Token::Fragment(fragment));
                }
                index = next_index;
            }
            '{' => {
                let (fragment, next_index) = parse_array_constant_fragment(&chars, index);
                tokens.push(Token::Fragment(fragment));
                index = next_index;
            }
            '=' => {
                tokens.push(Token::Eq);
                index += 1;
            }
            '<' => {
                if chars.get(index + 1) == Some(&'=') {
                    tokens.push(Token::Lte);
                    index += 2;
                } else if chars.get(index + 1) == Some(&'>') {
                    tokens.push(Token::Ne);
                    index += 2;
                } else {
                    tokens.push(Token::Lt);
                    index += 1;
                }
            }
            '>' => {
                if chars.get(index + 1) == Some(&'=') {
                    tokens.push(Token::Gte);
                    index += 2;
                } else {
                    tokens.push(Token::Gt);
                    index += 1;
                }
            }
            '"' => {
                let (value, next_index) = parse_string_literal(&chars, index)?;
                tokens.push(Token::StringLiteral(value));
                index = next_index;
            }
            _ => {
                let (fragment, next_index) = parse_fragment(&chars, index);
                tokens.push(Token::Fragment(fragment));
                index = next_index;
            }
        }
    }
    tokens.push(Token::Eof);
    Ok(tokens)
}

fn parse_string_literal(chars: &[char], mut index: usize) -> Result<(String, usize), String> {
    index += 1;
    let mut value = String::new();
    while index < chars.len() {
        match chars[index] {
            '"' if chars.get(index + 1) == Some(&'"') => {
                value.push('"');
                index += 2;
            }
            '"' => return Ok((bounded_text(&value, MAX_NODE_TEXT_CHARS), index + 1)),
            ch => {
                value.push(ch);
                index += 1;
            }
        }
    }
    Err("unterminated string literal".to_string())
}

fn parse_hash_fragment(chars: &[char], mut index: usize) -> (String, usize) {
    let mut fragment = String::new();
    while index < chars.len() {
        let ch = chars[index];
        if matches!(
            ch,
            ' ' | '\t'
                | '\r'
                | '\n'
                | '('
                | ')'
                | ','
                | '+'
                | '-'
                | '*'
                | '^'
                | '&'
                | '%'
                | '='
                | '<'
                | '>'
        ) {
            break;
        }
        fragment.push(ch);
        index += 1;
    }
    (bounded_text(&fragment, MAX_NODE_TEXT_CHARS), index)
}

fn parse_array_constant_fragment(chars: &[char], mut index: usize) -> (String, usize) {
    let mut fragment = String::new();
    while index < chars.len() {
        let ch = chars[index];
        fragment.push(ch);
        index += 1;
        if ch == '}' {
            break;
        }
    }
    (bounded_text(&fragment, MAX_NODE_TEXT_CHARS), index)
}

fn parse_fragment(chars: &[char], mut index: usize) -> (String, usize) {
    let mut fragment = String::new();
    while index < chars.len() {
        let ch = chars[index];
        if is_delimiter(ch) {
            break;
        }
        if ch == '\'' {
            fragment.push(ch);
            index += 1;
            while index < chars.len() {
                let quoted = chars[index];
                fragment.push(quoted);
                index += 1;
                if quoted == '\'' {
                    if chars.get(index) == Some(&'\'') {
                        fragment.push('\'');
                        index += 1;
                        continue;
                    }
                    break;
                }
            }
            continue;
        }
        fragment.push(ch);
        index += 1;
    }
    (bounded_text(&fragment, MAX_NODE_TEXT_CHARS), index)
}

fn is_delimiter(ch: char) -> bool {
    matches!(
        ch,
        ' ' | '\t'
            | '\r'
            | '\n'
            | '('
            | ')'
            | ','
            | '+'
            | '-'
            | '*'
            | '/'
            | '^'
            | '&'
            | '%'
            | '='
            | '<'
            | '>'
    )
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
    diagnostics: Vec<String>,
    node_budget: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            diagnostics: Vec::new(),
            node_budget: MAX_AST_NODES,
        }
    }

    fn parse_expression(&mut self, min_precedence: u8) -> Result<FormulaAstNode, String> {
        let mut lhs = self.parse_prefix()?;
        loop {
            if matches!(self.peek(), Token::Percent) {
                if postfix_precedence() < min_precedence {
                    break;
                }
                self.next();
                lhs = self.alloc(FormulaAstNode::Percent {
                    operand: Box::new(lhs),
                })?;
                continue;
            }
            if matches!(self.peek(), Token::SpillMarker) {
                self.next();
                lhs = self.alloc(FormulaAstNode::Unsupported {
                    reason: "dynamic_array_or_spill_marker".to_string(),
                    text: "#".to_string(),
                })?;
                continue;
            }
            let Some((operator, precedence, right_associative)) = binary_operator(self.peek())
            else {
                break;
            };
            if precedence < min_precedence {
                break;
            }
            self.next();
            let rhs = self.parse_expression(if right_associative {
                precedence
            } else {
                precedence + 1
            })?;
            lhs = self.alloc(FormulaAstNode::BinaryOperation {
                operator,
                left: Box::new(lhs),
                right: Box::new(rhs),
            })?;
        }
        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<FormulaAstNode, String> {
        match self.peek() {
            Token::Plus => {
                self.next();
                let operand = self.parse_expression(unary_precedence())?;
                self.alloc(FormulaAstNode::UnaryOperation {
                    operator: FormulaAstUnaryOperator::Plus,
                    operand: Box::new(operand),
                })
            }
            Token::Minus => {
                self.next();
                let operand = self.parse_expression(unary_precedence())?;
                self.alloc(FormulaAstNode::UnaryOperation {
                    operator: FormulaAstUnaryOperator::Minus,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<FormulaAstNode, String> {
        match self.next() {
            Token::StringLiteral(value) => self.alloc(FormulaAstNode::StringLiteral { value }),
            Token::Fragment(text) => {
                if matches!(self.peek(), Token::LParen) {
                    self.parse_function_call(text)
                } else {
                    self.alloc(classify_fragment(&text))
                }
            }
            Token::LParen => {
                let expr = self.parse_expression(0)?;
                if !matches!(self.next(), Token::RParen) {
                    return Err("expected closing ')'".to_string());
                }
                Ok(expr)
            }
            token => Err(format!("unexpected token {}", self.describe_token(&token))),
        }
    }

    fn parse_function_call(&mut self, name: String) -> Result<FormulaAstNode, String> {
        if !matches!(self.next(), Token::LParen) {
            return Err("expected '(' after function name".to_string());
        }

        let mut args = Vec::new();
        if !matches!(self.peek(), Token::RParen) {
            loop {
                if matches!(self.peek(), Token::Comma | Token::RParen) {
                    args.push(self.alloc(FormulaAstNode::BlankArgument)?);
                } else {
                    args.push(self.parse_expression(0)?);
                }
                if matches!(self.peek(), Token::Comma) {
                    self.next();
                    continue;
                }
                break;
            }
        }
        if !matches!(self.next(), Token::RParen) {
            return Err(format!("expected ')' after function {name}"));
        }

        if is_dynamic_array_function(&name) {
            return self.alloc(FormulaAstNode::Unsupported {
                reason: "dynamic_array_or_spill_marker".to_string(),
                text: bounded_text(&name, MAX_NODE_TEXT_CHARS),
            });
        }

        if is_volatile_function(&name) {
            return self.alloc(FormulaAstNode::Unsupported {
                reason: "volatile_function".to_string(),
                text: bounded_text(&name, MAX_NODE_TEXT_CHARS),
            });
        }

        self.alloc(FormulaAstNode::FunctionCall {
            name: bounded_text(&name, MAX_NODE_TEXT_CHARS),
            args,
        })
    }

    fn alloc(&mut self, node: FormulaAstNode) -> Result<FormulaAstNode, String> {
        if self.node_budget == 0 {
            return Err("formula AST node budget exceeded".to_string());
        }
        self.node_budget -= 1;
        Ok(node)
    }

    fn push_diagnostic(&mut self, message: String) {
        if self.diagnostics.len() < MAX_DIAGNOSTICS {
            self.diagnostics
                .push(bounded_text(&message, MAX_REASON_TEXT_CHARS));
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.index).unwrap_or(&Token::Eof)
    }

    fn next(&mut self) -> Token {
        let token = self.tokens.get(self.index).cloned().unwrap_or(Token::Eof);
        self.index = self.index.saturating_add(1);
        token
    }

    fn describe_token(&self, token: &Token) -> String {
        match token {
            Token::Fragment(text) => format!("fragment '{text}'"),
            Token::StringLiteral(_) => "string literal".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::Comma => ",".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Star => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Caret => "^".to_string(),
            Token::Amp => "&".to_string(),
            Token::Percent => "%".to_string(),
            Token::SpillMarker => "#".to_string(),
            Token::Eq => "=".to_string(),
            Token::Ne => "<>".to_string(),
            Token::Lt => "<".to_string(),
            Token::Lte => "<=".to_string(),
            Token::Gt => ">".to_string(),
            Token::Gte => ">=".to_string(),
            Token::Eof => "end of formula".to_string(),
        }
    }
}

fn postfix_precedence() -> u8 {
    8
}

fn unary_precedence() -> u8 {
    5
}

fn binary_operator(token: &Token) -> Option<(FormulaAstBinaryOperator, u8, bool)> {
    match token {
        Token::Eq => Some((FormulaAstBinaryOperator::Equal, 1, false)),
        Token::Ne => Some((FormulaAstBinaryOperator::NotEqual, 1, false)),
        Token::Lt => Some((FormulaAstBinaryOperator::LessThan, 1, false)),
        Token::Lte => Some((FormulaAstBinaryOperator::LessThanOrEqual, 1, false)),
        Token::Gt => Some((FormulaAstBinaryOperator::GreaterThan, 1, false)),
        Token::Gte => Some((FormulaAstBinaryOperator::GreaterThanOrEqual, 1, false)),
        Token::Amp => Some((FormulaAstBinaryOperator::Concatenate, 2, false)),
        Token::Plus => Some((FormulaAstBinaryOperator::Add, 3, false)),
        Token::Minus => Some((FormulaAstBinaryOperator::Subtract, 3, false)),
        Token::Star => Some((FormulaAstBinaryOperator::Multiply, 4, false)),
        Token::Slash => Some((FormulaAstBinaryOperator::Divide, 4, false)),
        Token::Caret => Some((FormulaAstBinaryOperator::Power, 7, true)),
        _ => None,
    }
}

fn classify_fragment(text: &str) -> FormulaAstNode {
    if is_error_literal(text) {
        return FormulaAstNode::ErrorLiteral {
            value: bounded_text(text, MAX_NODE_TEXT_CHARS),
        };
    }
    if text == "#" || text.ends_with('#') {
        return unsupported_node("dynamic_array_or_spill_marker", text);
    }
    if text.contains('{') || text.contains('}') {
        return unsupported_node("array_constant", text);
    }
    if is_r1c1_reference(text) {
        return unsupported_node("r1c1_reference", text);
    }
    if text.contains('[') || text.contains(']') {
        return if text.contains('!') {
            unsupported_node("external_workbook_reference", text)
        } else {
            unsupported_node("structured_reference", text)
        };
    }
    if let Some((sheet_name, reference)) = split_sheet_reference(text) {
        return classify_reference(reference, Some(sheet_name));
    }
    if is_boolean(text) {
        return FormulaAstNode::BooleanLiteral {
            value: text.eq_ignore_ascii_case("TRUE"),
        };
    }
    if NUMBER_RE.is_match(text) {
        return FormulaAstNode::NumberLiteral {
            value: bounded_text(text, MAX_NODE_TEXT_CHARS),
        };
    }
    classify_reference(text, None)
}

fn classify_reference(reference: &str, sheet_name: Option<String>) -> FormulaAstNode {
    if let Some((start, end)) = split_range(reference)
        && CELL_REF_RE.is_match(start)
        && CELL_REF_RE.is_match(end)
    {
        return FormulaAstNode::RangeReference {
            start_reference: bounded_text(start, MAX_NODE_TEXT_CHARS),
            end_reference: bounded_text(end, MAX_NODE_TEXT_CHARS),
            sheet_name,
        };
    }
    if CELL_REF_RE.is_match(reference) {
        return FormulaAstNode::CellReference {
            reference: bounded_text(reference, MAX_NODE_TEXT_CHARS),
            sheet_name,
        };
    }
    if NAME_RE.is_match(reference) {
        return FormulaAstNode::DefinedNameReference {
            name: bounded_text(reference, MAX_NODE_TEXT_CHARS),
            sheet_name,
        };
    }
    unsupported_node("unclassified_fragment", reference)
}

fn unsupported_node(reason: &str, text: &str) -> FormulaAstNode {
    FormulaAstNode::Unsupported {
        reason: bounded_text(reason, MAX_REASON_TEXT_CHARS),
        text: bounded_text(text, MAX_NODE_TEXT_CHARS),
    }
}

fn split_range(text: &str) -> Option<(&str, &str)> {
    let (start, end) = text.split_once(':')?;
    (!start.is_empty() && !end.is_empty()).then_some((start, end))
}

fn split_sheet_reference(text: &str) -> Option<(String, &str)> {
    let bang_index = text.rfind('!')?;
    let (sheet_text, reference) = text.split_at(bang_index);
    let reference = reference.strip_prefix('!')?;
    if sheet_text.contains('[') || sheet_text.contains(']') {
        return Some((String::new(), text));
    }
    Some((normalize_sheet_name(sheet_text), reference))
}

fn normalize_sheet_name(sheet_text: &str) -> String {
    let unquoted = if sheet_text.starts_with('\'') && sheet_text.ends_with('\'') {
        &sheet_text[1..sheet_text.len().saturating_sub(1)]
    } else {
        sheet_text
    };
    bounded_text(&unquoted.replace("''", "'"), MAX_NODE_TEXT_CHARS)
}

fn is_error_literal(text: &str) -> bool {
    matches!(
        text.to_ascii_uppercase().as_str(),
        "#N/A"
            | "#VALUE!"
            | "#REF!"
            | "#DIV/0!"
            | "#NUM!"
            | "#NAME?"
            | "#NULL!"
            | "#SPILL!"
            | "#CALC!"
            | "#FIELD!"
            | "#GETTING_DATA"
    )
}

fn is_boolean(text: &str) -> bool {
    text.eq_ignore_ascii_case("TRUE") || text.eq_ignore_ascii_case("FALSE")
}

fn is_r1c1_reference(text: &str) -> bool {
    R1C1_RE.is_match(text)
}

fn is_volatile_function(name: &str) -> bool {
    matches!(
        normalized_function_name(name).as_str(),
        "NOW" | "TODAY" | "RAND" | "RANDBETWEEN" | "OFFSET" | "INDIRECT"
    )
}

fn is_dynamic_array_function(name: &str) -> bool {
    matches!(
        normalized_function_name(name).as_str(),
        "FILTER" | "UNIQUE" | "SORT" | "SORTBY" | "SEQUENCE" | "RANDARRAY"
    )
}

fn normalized_function_name(name: &str) -> String {
    name.to_ascii_uppercase()
        .trim_start_matches("_XLFN.")
        .trim_start_matches("_XLWS.")
        .to_string()
}

fn collect_unsupported_reasons(node: &FormulaAstNode, reasons: &mut Vec<String>) {
    match node {
        FormulaAstNode::UnaryOperation { operand, .. } | FormulaAstNode::Percent { operand } => {
            collect_unsupported_reasons(operand, reasons)
        }
        FormulaAstNode::BinaryOperation { left, right, .. } => {
            collect_unsupported_reasons(left, reasons);
            collect_unsupported_reasons(right, reasons);
        }
        FormulaAstNode::FunctionCall { args, .. } => {
            for arg in args {
                collect_unsupported_reasons(arg, reasons);
            }
        }
        FormulaAstNode::Unsupported { reason, .. } => reasons.push(reason.clone()),
        FormulaAstNode::NumberLiteral { .. }
        | FormulaAstNode::StringLiteral { .. }
        | FormulaAstNode::BooleanLiteral { .. }
        | FormulaAstNode::BlankArgument
        | FormulaAstNode::ErrorLiteral { .. }
        | FormulaAstNode::CellReference { .. }
        | FormulaAstNode::RangeReference { .. }
        | FormulaAstNode::DefinedNameReference { .. } => {}
    }
}
