use std::collections::BTreeSet;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;

use ontocode_extension_api::FunctionCallError;
use ontocode_extension_api::JsonToolOutput;
use ontocode_extension_api::ToolCall;
use ontocode_extension_api::ToolExecutor;
use ontocode_extension_api::ToolName;
use ontocode_extension_api::ToolOutput;
use ontocode_extension_api::ToolSpec;
use ontocode_extension_api::parse_tool_input_schema;
use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_value;

use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;

pub(crate) const TRANSLATE_POWERQUERY_TO_SQL_PREVIEW_TOOL_NAME: &str =
    "translate_powerquery_to_sql_preview";

const TRANSLATE_POWERQUERY_TO_SQL_PREVIEW_DESCRIPTION: &str =
    "Translate pasted Power Query M source into a bounded heuristic SQL preview.";
const MAX_SOURCE_CHARS: usize = 32_768;
const MAX_SQL_CHARS: usize = 8_192;
const MAX_WARNINGS: usize = 16;
const IGNORED_FUNCTIONS: [&str; 5] = [
    "Table.TransformColumnTypes",
    "Table.PromoteHeaders",
    "Table.RenameColumns",
    "Table.ReplaceValue",
    "Table.RemoveRowsWithErrors",
];

static VALUE_NATIVE_QUERY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?is)Value\.NativeQuery\([^,]+,\s*"((?:[^"]|"")*)""#)
        .unwrap_or_else(|err| panic!("Value.NativeQuery regex should compile: {err}"))
});
static SHARED_QUERY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?im)^\s*shared\s+(?:#\"([^\"]+)\"|([A-Za-z_][A-Za-z0-9_]*))\s*="#)
        .unwrap_or_else(|err| panic!("shared query regex should compile: {err}"))
});
static ITEM_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"Item\s*=\s*"([^"]+)""#)
        .unwrap_or_else(|err| panic!("item regex should compile: {err}"))
});
static SCHEMA_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"Schema\s*=\s*"([^"]+)""#)
        .unwrap_or_else(|err| panic!("schema regex should compile: {err}"))
});
static SIMPLE_PREDICATE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*\[([^\]]+)\]\s*(=|<>|>=|<=|>|<)\s*(.+?)\s*$"#)
        .unwrap_or_else(|err| panic!("predicate regex should compile: {err}"))
});
static CONNECTOR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\s+(and|or)\s+")
        .unwrap_or_else(|err| panic!("connector regex should compile: {err}"))
});

#[derive(Clone, Default)]
pub(crate) struct ExcelTranslatePowerQueryToSqlPreviewTool {
    _thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct TranslatePowerQueryToSqlPreviewArgs {
    pub source_text: String,
    #[serde(default)]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct TranslatePowerQueryToSqlPreviewResult {
    pub mode: String,
    pub source_name: String,
    pub source_line_count: usize,
    pub source_truncated: bool,
    pub success: bool,
    pub sql: String,
    pub unsupported_functions: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct MProgram {
    steps: HashMap<String, String>,
    final_expression: String,
}

#[derive(Debug, Default, Clone)]
struct SqlState {
    table_name: String,
    select_columns: Vec<String>,
    where_clause: String,
    order_by_clause: String,
    top_n: Option<usize>,
    distinct: bool,
}

#[derive(Debug, Clone)]
struct FunctionCallParts {
    name: String,
    arguments: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelTranslatePowerQueryToSqlPreviewTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(
            EXCEL_NAMESPACE,
            TRANSLATE_POWERQUERY_TO_SQL_PREVIEW_TOOL_NAME,
        )
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(TranslatePowerQueryToSqlPreviewArgs))
                .unwrap_or_else(|err| {
                    panic!(
                        "translate_powerquery_to_sql_preview args schema should serialize: {err}"
                    )
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(TranslatePowerQueryToSqlPreviewResult))
                .unwrap_or_else(|err| {
                    panic!(
                        "translate_powerquery_to_sql_preview result schema should serialize: {err}"
                    )
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: TRANSLATE_POWERQUERY_TO_SQL_PREVIEW_TOOL_NAME.to_string(),
                    description: TRANSLATE_POWERQUERY_TO_SQL_PREVIEW_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!(
                            "translate_powerquery_to_sql_preview args schema should parse: {err}"
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
        let args = parse_tool_args::<TranslatePowerQueryToSqlPreviewArgs>(
            &call,
            "excel.translate_powerquery_to_sql_preview",
        )?;
        let result =
            translate_powerquery_to_sql_preview(&args.source_text, args.source_name.as_deref());
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize Power Query translation preview: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelTranslatePowerQueryToSqlPreviewTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self {
            _thread_state: thread_state,
        }
    }
}

pub(crate) fn translate_powerquery_to_sql_preview(
    source_text: &str,
    source_name: Option<&str>,
) -> TranslatePowerQueryToSqlPreviewResult {
    let source_name = source_name.unwrap_or("pasted Power Query").to_string();
    let (source_text, source_truncated) = bounded_text(source_text, MAX_SOURCE_CHARS);
    let mut warnings = Vec::new();
    if source_truncated {
        warnings.push(format!(
            "source text truncated to {MAX_SOURCE_CHARS} characters for preview"
        ));
    }

    if let Some(captures) = VALUE_NATIVE_QUERY_RE.captures(&source_text) {
        let sql = normalize_sql(&unescape_m_string(&captures[1]));
        let (sql, sql_truncated) = bounded_text(&sql, MAX_SQL_CHARS);
        if sql_truncated {
            warnings.push(format!(
                "SQL preview truncated to {MAX_SQL_CHARS} characters"
            ));
        }
        warnings.push("SQL preview was derived directly from Value.NativeQuery().".to_string());
        warnings.push(
            "Preview is heuristic and source-first; review the generated SQL before execution."
                .to_string(),
        );
        warnings.truncate(MAX_WARNINGS);
        return TranslatePowerQueryToSqlPreviewResult {
            mode: "heuristic_preview".to_string(),
            source_name,
            source_line_count: source_text.lines().count(),
            source_truncated,
            success: true,
            sql,
            unsupported_functions: Vec::new(),
            warnings,
        };
    }

    let mut unsupported_functions = BTreeSet::new();
    let program = parse_program(&source_text);
    let mut state = SqlState::default();
    analyze_expression(
        &program.final_expression,
        &program.steps,
        &mut BTreeSet::new(),
        &mut state,
        &mut warnings,
        &mut unsupported_functions,
    );

    let mut success = !state.table_name.is_empty();
    let mut sql = if success {
        render_sql(&state)
    } else {
        warnings.push(
            "No supported Power Query table pipeline was detected in the source preview."
                .to_string(),
        );
        String::new()
    };

    if sql.is_empty() && success {
        sql = "SELECT * FROM [UnknownTable]".to_string();
        success = false;
        warnings.push("SQL preview could not determine a stable table target.".to_string());
    }

    let (sql, sql_truncated) = bounded_text(&sql, MAX_SQL_CHARS);
    if sql_truncated {
        warnings.push(format!(
            "SQL preview truncated to {MAX_SQL_CHARS} characters"
        ));
    }
    warnings.push(
        "Preview is heuristic and source-first; review the generated SQL before execution."
            .to_string(),
    );
    warnings.truncate(MAX_WARNINGS);

    TranslatePowerQueryToSqlPreviewResult {
        mode: "heuristic_preview".to_string(),
        source_name,
        source_line_count: source_text.lines().count(),
        source_truncated,
        success,
        sql,
        unsupported_functions: unsupported_functions.into_iter().collect(),
        warnings,
    }
}

fn parse_program(source: &str) -> MProgram {
    if let Some(shared_start) = SHARED_QUERY_RE.find(source).map(|match_| match_.start()) {
        return parse_single_query(&source[shared_start..]);
    }
    parse_single_query(source)
}

fn parse_single_query(source: &str) -> MProgram {
    let query_body = if let Some(start) = source.find('=') {
        source[start + 1..].trim().trim_end_matches(';').trim()
    } else {
        source.trim().trim_end_matches(';').trim()
    };
    let Some(let_start) = query_body.find("let") else {
        return MProgram {
            steps: HashMap::new(),
            final_expression: query_body.to_string(),
        };
    };
    let Some(in_index) = find_top_level_keyword(query_body, "in", let_start + 3) else {
        return MProgram {
            steps: HashMap::new(),
            final_expression: query_body.to_string(),
        };
    };
    let body = query_body[let_start + 3..in_index].trim();
    let final_expression = query_body[in_index + 2..]
        .trim()
        .trim_end_matches(';')
        .trim()
        .to_string();

    let mut steps = HashMap::new();
    for part in split_top_level(body, ',') {
        let Some(eq_index) = find_top_level_char(&part, '=') else {
            continue;
        };
        let name = clean_identifier(part[..eq_index].trim());
        let expression = part[eq_index + 1..].trim();
        if !name.is_empty() && !expression.is_empty() {
            steps.insert(name, expression.to_string());
        }
    }

    MProgram {
        steps,
        final_expression,
    }
}

fn analyze_expression(
    expression: &str,
    steps: &HashMap<String, String>,
    visited: &mut BTreeSet<String>,
    state: &mut SqlState,
    warnings: &mut Vec<String>,
    unsupported_functions: &mut BTreeSet<String>,
) {
    let expr = expression.trim().trim_end_matches(';').trim();
    let identifier = clean_identifier(expr);
    if let Some(step_expression) = steps.get(&identifier) {
        if !visited.insert(identifier.clone()) {
            return;
        }
        analyze_expression(
            step_expression,
            steps,
            visited,
            state,
            warnings,
            unsupported_functions,
        );
        return;
    }

    if let Some(table_name) = extract_table_name(expr) {
        state.table_name = table_name;
        return;
    }

    let Some(call) = parse_function_call(expr) else {
        return;
    };

    if let Some(first_argument) = call.arguments.first() {
        analyze_expression(
            first_argument,
            steps,
            visited,
            state,
            warnings,
            unsupported_functions,
        );
    }

    match call.name.as_str() {
        "Sql.Database" => {
            if state.table_name.is_empty() && call.arguments.len() >= 2 {
                state.table_name = escape_ident(&unquote(&call.arguments[1]));
            }
        }
        "Table.SelectColumns" => {
            if call.arguments.len() >= 2 {
                state.select_columns = parse_string_list(&call.arguments[1]);
            }
        }
        "Table.SelectRows" => {
            if call.arguments.len() >= 2 {
                if let Some(where_clause) = parse_where_clause(&call.arguments[1]) {
                    state.where_clause = where_clause;
                } else {
                    warnings.push(
                        "Table.SelectRows predicate was not translated in SQL preview.".to_string(),
                    );
                }
            }
        }
        "Table.FirstN" => {
            if call.arguments.len() >= 2 {
                state.top_n = parse_integer(&call.arguments[1]);
            }
        }
        "Table.Distinct" => {
            state.distinct = true;
        }
        "Table.Sort" => {
            if call.arguments.len() >= 2
                && let Some(order_by) = parse_order_by(&call.arguments[1])
            {
                state.order_by_clause = order_by;
            }
        }
        "Table.TransformColumnTypes"
        | "Table.PromoteHeaders"
        | "Table.RenameColumns"
        | "Table.ReplaceValue"
        | "Table.RemoveRowsWithErrors" => {
            warnings.push(format!(
                "Function '{}' was ignored in SQL preview.",
                call.name
            ));
        }
        "Excel.Workbook" | "Csv.Document" | "Access.Database" | "Odbc.DataSource"
        | "OleDb.DataSource" => {
            if state.table_name.is_empty() {
                state.table_name = "[ExternalSource]".to_string();
            }
        }
        _ => {
            if !IGNORED_FUNCTIONS.iter().any(|value| *value == call.name) {
                unsupported_functions.insert(call.name.clone());
                warnings.push(format!(
                    "Function '{}' is not fully supported and was ignored in SQL preview.",
                    call.name
                ));
            }
        }
    }
}

fn render_sql(state: &SqlState) -> String {
    let selected_columns = if state.select_columns.is_empty() {
        "*".to_string()
    } else {
        state.select_columns.join(", ")
    };
    let distinct = if state.distinct { "DISTINCT " } else { "" };
    let top = state
        .top_n
        .map(|value| format!("TOP {value} "))
        .unwrap_or_default();
    let mut sql = format!(
        "SELECT {distinct}{top}{selected_columns} FROM {}",
        state.table_name
    );
    if !state.where_clause.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&state.where_clause);
    }
    if !state.order_by_clause.is_empty() {
        sql.push_str(" ORDER BY ");
        sql.push_str(&state.order_by_clause);
    }
    normalize_sql(&sql)
}

fn extract_table_name(expression: &str) -> Option<String> {
    let item = ITEM_RE
        .captures(expression)
        .and_then(|captures| captures.get(1).map(|value| value.as_str().to_string()))?;
    let schema = SCHEMA_RE
        .captures(expression)
        .and_then(|captures| captures.get(1).map(|value| value.as_str().to_string()));
    let mut name = String::new();
    if let Some(schema_name) = schema
        && !schema_name.is_empty()
    {
        name.push_str(&escape_ident(&schema_name));
        name.push('.');
    }
    name.push_str(&escape_ident(&item));
    Some(name)
}

fn parse_function_call(expression: &str) -> Option<FunctionCallParts> {
    let open_index = find_top_level_char(expression, '(')?;
    let close_index = find_matching_paren(expression, open_index)?;
    let name = expression[..open_index].trim();
    if name.is_empty() {
        return None;
    }
    let args = split_top_level(&expression[open_index + 1..close_index], ',');
    Some(FunctionCallParts {
        name: clean_identifier(name),
        arguments: args
            .into_iter()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect(),
    })
}

fn parse_where_clause(argument: &str) -> Option<String> {
    let each_index = argument.find("each")?;
    let predicate = argument[each_index + 4..].trim();
    if predicate.contains('(') || predicate.contains(')') {
        return None;
    }

    let mut fragments = Vec::new();
    let mut last = 0usize;
    let mut connectors = Vec::new();
    for captures in CONNECTOR_RE.captures_iter(predicate) {
        let whole = captures.get(0)?;
        fragments.push(predicate[last..whole.start()].trim());
        connectors.push(captures.get(1)?.as_str().to_ascii_uppercase());
        last = whole.end();
    }
    fragments.push(predicate[last..].trim());

    let normalized_parts = fragments
        .into_iter()
        .map(normalize_predicate_fragment)
        .collect::<Option<Vec<_>>>()?;

    let mut clause = String::new();
    for (index, part) in normalized_parts.iter().enumerate() {
        if index > 0 {
            clause.push(' ');
            clause.push_str(&connectors[index - 1]);
            clause.push(' ');
        }
        clause.push_str(part);
    }
    Some(normalize_sql(&clause))
}

fn normalize_predicate_fragment(fragment: &str) -> Option<String> {
    let captures = SIMPLE_PREDICATE_RE.captures(fragment)?;
    let left = format!("[{}]", captures.get(1)?.as_str());
    let op = captures.get(2)?.as_str();
    let right = normalize_predicate_literal(captures.get(3)?.as_str().trim())?;
    Some(format!("{left} {op} {right}"))
}

fn normalize_predicate_literal(literal: &str) -> Option<String> {
    if literal.eq_ignore_ascii_case("true")
        || literal.eq_ignore_ascii_case("false")
        || literal.eq_ignore_ascii_case("null")
        || literal.parse::<i64>().is_ok()
        || literal.parse::<f64>().is_ok()
    {
        return Some(literal.to_string());
    }
    if let Some(value) = literal
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
    {
        return Some(format!("'{}'", value.replace('\'', "''")));
    }
    if let Some(value) = literal
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''))
    {
        return Some(format!("'{}'", value.replace('\'', "''")));
    }
    None
}

fn parse_order_by(argument: &str) -> Option<String> {
    let mut orderings = Vec::new();
    let order_item_re = Regex::new(r#"\{\s*"([^"]+)"\s*,\s*Order\.(Ascending|Descending)\s*\}"#)
        .unwrap_or_else(|err| panic!("order item regex should compile: {err}"));
    for captures in order_item_re.captures_iter(argument) {
        let column = escape_ident(&captures[1]);
        let direction = if captures[2].eq_ignore_ascii_case("descending") {
            "DESC"
        } else {
            "ASC"
        };
        orderings.push(format!("{column} {direction}"));
    }
    if orderings.is_empty() {
        None
    } else {
        Some(orderings.join(", "))
    }
}

fn parse_string_list(argument: &str) -> Vec<String> {
    let trimmed = argument.trim().trim_matches(['{', '}']);
    split_top_level(trimmed, ',')
        .into_iter()
        .map(|value| escape_ident(&unquote(&value)))
        .filter(|value| !value.is_empty())
        .collect()
}

fn parse_integer(value: &str) -> Option<usize> {
    value.trim().parse().ok()
}

fn split_top_level(value: &str, separator: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0usize;
    let mut round_depth = 0i32;
    let mut brace_depth = 0i32;
    let mut bracket_depth = 0i32;
    let mut in_string = false;
    let chars = value.char_indices().collect::<Vec<_>>();
    let mut index = 0usize;

    while index < chars.len() {
        let (offset, ch) = chars[index];
        match ch {
            '"' => {
                if in_string && chars.get(index + 1).is_some_and(|(_, next)| *next == '"') {
                    index += 1;
                } else {
                    in_string = !in_string;
                }
            }
            '(' if !in_string => round_depth += 1,
            ')' if !in_string => round_depth -= 1,
            '{' if !in_string => brace_depth += 1,
            '}' if !in_string => brace_depth -= 1,
            '[' if !in_string => bracket_depth += 1,
            ']' if !in_string => bracket_depth -= 1,
            _ => {}
        }

        if ch == separator
            && !in_string
            && round_depth == 0
            && brace_depth == 0
            && bracket_depth == 0
        {
            result.push(value[start..offset].trim().to_string());
            start = offset + ch.len_utf8();
        }

        index += 1;
    }

    if start < value.len() {
        result.push(value[start..].trim().to_string());
    }

    result
}

fn find_top_level_char(value: &str, target: char) -> Option<usize> {
    let mut round_depth = 0i32;
    let mut brace_depth = 0i32;
    let mut bracket_depth = 0i32;
    let mut in_string = false;
    let chars = value.char_indices().collect::<Vec<_>>();
    let mut index = 0usize;

    while index < chars.len() {
        let (offset, ch) = chars[index];
        if ch == target && !in_string && round_depth == 0 && brace_depth == 0 && bracket_depth == 0
        {
            return Some(offset);
        }
        match ch {
            '"' => {
                if in_string && chars.get(index + 1).is_some_and(|(_, next)| *next == '"') {
                    index += 1;
                } else {
                    in_string = !in_string;
                }
            }
            '(' if !in_string => round_depth += 1,
            ')' if !in_string => round_depth -= 1,
            '{' if !in_string => brace_depth += 1,
            '}' if !in_string => brace_depth -= 1,
            '[' if !in_string => bracket_depth += 1,
            ']' if !in_string => bracket_depth -= 1,
            _ => {}
        }
        index += 1;
    }
    None
}

fn find_matching_paren(value: &str, open_index: usize) -> Option<usize> {
    let mut depth = 0i32;
    let chars = value.char_indices().collect::<Vec<_>>();
    let mut in_string = false;
    let mut index = 0usize;

    while index < chars.len() {
        let (offset, ch) = chars[index];
        if offset < open_index {
            index += 1;
            continue;
        }
        match ch {
            '"' => {
                if in_string && chars.get(index + 1).is_some_and(|(_, next)| *next == '"') {
                    index += 1;
                } else {
                    in_string = !in_string;
                }
            }
            '(' if !in_string => depth += 1,
            ')' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return Some(offset);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn find_top_level_keyword(value: &str, keyword: &str, start_offset: usize) -> Option<usize> {
    let mut round_depth = 0i32;
    let mut brace_depth = 0i32;
    let mut bracket_depth = 0i32;
    let mut in_string = false;
    let chars = value.char_indices().collect::<Vec<_>>();
    let mut index = 0usize;

    while index < chars.len() {
        let (offset, ch) = chars[index];
        if offset < start_offset {
            index += 1;
            continue;
        }
        match ch {
            '"' => {
                if in_string && chars.get(index + 1).is_some_and(|(_, next)| *next == '"') {
                    index += 1;
                } else {
                    in_string = !in_string;
                }
            }
            '(' if !in_string => round_depth += 1,
            ')' if !in_string => round_depth -= 1,
            '{' if !in_string => brace_depth += 1,
            '}' if !in_string => brace_depth -= 1,
            '[' if !in_string => bracket_depth += 1,
            ']' if !in_string => bracket_depth -= 1,
            _ => {}
        }

        if !in_string
            && round_depth == 0
            && brace_depth == 0
            && bracket_depth == 0
            && value[offset..].starts_with(keyword)
        {
            let before = value[..offset].chars().next_back();
            let after = value[offset + keyword.len()..].chars().next();
            let boundary_before = before.is_none_or(char::is_whitespace);
            let boundary_after = after.is_none_or(char::is_whitespace);
            if boundary_before && boundary_after {
                return Some(offset);
            }
        }

        index += 1;
    }

    None
}

fn clean_identifier(value: &str) -> String {
    let trimmed = value.trim().trim_matches(',');
    if let Some(stripped) = trimmed
        .strip_prefix("#\"")
        .and_then(|value| value.strip_suffix('"'))
    {
        return stripped.to_string();
    }
    trimmed.trim_matches('"').to_string()
}

fn unquote(value: &str) -> String {
    clean_identifier(value).replace("\"\"", "\"")
}

fn escape_ident(value: &str) -> String {
    format!("[{}]", value.replace(']', "]]"))
}

fn normalize_sql(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn unescape_m_string(value: &str) -> String {
    value.replace("\"\"", "\"")
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

fn bounded_text(value: &str, max_chars: usize) -> (String, bool) {
    let mut truncated = false;
    let mut text = String::new();
    for (count, ch) in value.chars().enumerate() {
        if count >= max_chars {
            truncated = true;
            break;
        }
        text.push(ch);
    }
    (text, truncated)
}
