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

pub(crate) const TRANSLATE_VBA_TO_M_PREVIEW_TOOL_NAME: &str = "translate_vba_to_m_preview";

const TRANSLATE_VBA_TO_M_PREVIEW_DESCRIPTION: &str =
    "Translate pasted VBA source into a bounded heuristic Power Query M preview.";
const MAX_SOURCE_CHARS: usize = 32_768;
const MAX_OUTPUT_CHARS: usize = 8_192;
const MAX_QUERY_COUNT: usize = 8;
const MAX_QUERY_CODE_CHARS: usize = 4_096;
const MAX_WARNINGS: usize = 16;
const UNSUPPORTED_PATTERN_WARNINGS: [(&str, &str); 6] = [
    (
        "CreateObject(",
        "CreateObject automation is not translated in this preview.",
    ),
    (
        "QueryDef",
        "DAO QueryDef flows are not translated in this preview.",
    ),
    (
        ".Parameters",
        "Parameterized ADO command bindings are not translated in this preview.",
    ),
    (
        "OpenRecordset",
        "DAO OpenRecordset flows are not translated in this preview.",
    ),
    (
        "Execute(",
        "SQL execution side effects are not translated in this preview.",
    ),
    (
        "CopyFromRecordset",
        "Worksheet CopyFromRecordset flows are not translated in this preview.",
    ),
];

static PROCEDURE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^\s*(?:Public\s+|Private\s+)?(?:Sub|Function)\s+([A-Za-z_][A-Za-z0-9_]*)")
        .unwrap_or_else(|err| panic!("procedure regex should compile: {err}"))
});
static STRING_ASSIGNMENT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(?:(?:[A-Za-z_][A-Za-z0-9_]*)\s*&\s*)?"((?:[^"]|"")*)""#)
        .unwrap_or_else(|err| panic!("string assignment regex should compile: {err}"))
});
static CONNECTION_OPEN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\.Open\s+"((?:[^"]|"")*)""#)
        .unwrap_or_else(|err| panic!("connection regex should compile: {err}"))
});
static RECORDSET_OPEN_VARIABLE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\.Open\s+([A-Za-z_][A-Za-z0-9_]*)\s*,"#)
        .unwrap_or_else(|err| panic!("recordset variable regex should compile: {err}"))
});
static RECORDSET_OPEN_INLINE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\.Open\s+"((?:[^"]|"")*)""#)
        .unwrap_or_else(|err| panic!("recordset inline regex should compile: {err}"))
});
static COMMAND_TEXT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\.CommandText\s*=\s*(?:"((?:[^"]|"")*)"|([A-Za-z_][A-Za-z0-9_]*))"#)
        .unwrap_or_else(|err| panic!("command text regex should compile: {err}"))
});
static SQL_LOOKUP_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(select|with|insert|update|delete)\b")
        .unwrap_or_else(|err| panic!("sql lookup regex should compile: {err}"))
});
static SELECT_COLUMNS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?is)select\s+(.*?)\s+from\s")
        .unwrap_or_else(|err| panic!("select columns regex should compile: {err}"))
});
static ALIAS_SUFFIX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\s+as\s+.+$").unwrap_or_else(|err| panic!("alias regex should compile: {err}"))
});
static WHITESPACE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\s+").unwrap_or_else(|err| panic!("whitespace regex should compile: {err}"))
});
static SANITIZE_NAME_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[^A-Za-z0-9_]")
        .unwrap_or_else(|err| panic!("sanitize regex should compile: {err}"))
});

#[derive(Clone, Default)]
pub(crate) struct ExcelTranslateVbaToMPreviewTool {
    _thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct TranslateVbaToMPreviewArgs {
    pub source_text: String,
    #[serde(default)]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct MPreviewQuery {
    pub name: String,
    pub source_procedure: String,
    pub native_sql: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct TranslateVbaToMPreviewResult {
    pub mode: String,
    pub source_name: String,
    pub source_line_count: usize,
    pub source_truncated: bool,
    pub m_queries: Vec<MPreviewQuery>,
    pub modified_vba: String,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelTranslateVbaToMPreviewTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, TRANSLATE_VBA_TO_M_PREVIEW_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema = serde_json::to_value(schemars::schema_for!(TranslateVbaToMPreviewArgs))
            .unwrap_or_else(|err| {
                panic!("translate_vba_to_m_preview args schema should serialize: {err}")
            });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(TranslateVbaToMPreviewResult))
                .unwrap_or_else(|err| {
                    panic!("translate_vba_to_m_preview result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: TRANSLATE_VBA_TO_M_PREVIEW_TOOL_NAME.to_string(),
                    description: TRANSLATE_VBA_TO_M_PREVIEW_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("translate_vba_to_m_preview args schema should parse: {err}")
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
        let args = parse_tool_args::<TranslateVbaToMPreviewArgs>(
            &call,
            "excel.translate_vba_to_m_preview",
        )?;
        let result = translate_vba_to_m_preview(&args.source_text, args.source_name.as_deref());
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize VBA translation preview: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelTranslateVbaToMPreviewTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self {
            _thread_state: thread_state,
        }
    }
}

pub(crate) fn translate_vba_to_m_preview(
    source_text: &str,
    source_name: Option<&str>,
) -> TranslateVbaToMPreviewResult {
    let source_name = source_name.unwrap_or("pasted VBA").to_string();
    let (source_text, source_truncated) = bounded_text(source_text, MAX_SOURCE_CHARS);
    let lines = split_lines(&source_text);
    let mut warnings = Vec::new();

    if source_truncated {
        warnings.push(format!(
            "source text truncated to {MAX_SOURCE_CHARS} characters for preview"
        ));
    }
    collect_unsupported_pattern_warnings(&lines, &mut warnings);

    let mut sql_vars = HashMap::<String, String>::new();
    let mut data_lines = BTreeSet::<usize>::new();
    let mut connection = String::new();
    let mut procedure = "Workbook".to_string();
    let mut query_index = 1usize;
    let mut m_queries = Vec::new();
    let mut query_limit_reached = false;

    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1;
        if let Some(caps) = PROCEDURE_RE.captures(line) {
            procedure = caps[1].to_string();
        }

        if let Some(caps) = CONNECTION_OPEN_RE.captures(line)
            && looks_like_connection_string(&caps[1])
        {
            connection = unescape_vba_string(&caps[1]);
            data_lines.insert(line_number);
            continue;
        }

        if let Some(caps) = COMMAND_TEXT_RE.captures(line) {
            let inline_sql = caps.get(1).map(|value| unescape_vba_string(value.as_str()));
            let variable_name = caps.get(2).map(|value| value.as_str());
            let sql = inline_sql
                .as_deref()
                .filter(|value| !value.is_empty())
                .map_or_else(
                    || read_sql_variable(&sql_vars, variable_name),
                    ToOwned::to_owned,
                );
            if looks_like_sql(&sql) {
                add_query(
                    &mut m_queries,
                    &procedure,
                    &sql,
                    &connection,
                    query_index,
                    &mut query_limit_reached,
                );
                query_index = query_index.saturating_add(1);
                data_lines.insert(line_number);
            }
            continue;
        }

        if let Some(caps) = STRING_ASSIGNMENT_RE.captures(line) {
            let variable = normalize_identifier(&caps[1]);
            let text = unescape_vba_string(&caps[2]);
            if is_sql_variable(&variable)
                || looks_like_sql(&text)
                || sql_vars.contains_key(&variable)
            {
                let value = sql_vars.entry(variable).or_default();
                if !value.is_empty() {
                    value.push(' ');
                }
                value.push_str(text.trim());
                data_lines.insert(line_number);
            }
        }

        if let Some(caps) = RECORDSET_OPEN_INLINE_RE.captures(line) {
            let sql = unescape_vba_string(&caps[1]);
            if looks_like_sql(&sql) {
                add_query(
                    &mut m_queries,
                    &procedure,
                    &sql,
                    &connection,
                    query_index,
                    &mut query_limit_reached,
                );
                query_index = query_index.saturating_add(1);
                data_lines.insert(line_number);
                continue;
            }
        }

        if let Some(caps) = RECORDSET_OPEN_VARIABLE_RE.captures(line) {
            let variable_name = caps.get(1).map(|value| value.as_str());
            let sql = read_sql_variable(&sql_vars, variable_name);
            if looks_like_sql(&sql) {
                add_query(
                    &mut m_queries,
                    &procedure,
                    &sql,
                    &connection,
                    query_index,
                    &mut query_limit_reached,
                );
                query_index = query_index.saturating_add(1);
                data_lines.insert(line_number);
            }
        }
    }

    if m_queries.is_empty() {
        for sql in sql_vars.values() {
            if looks_like_sql(sql) {
                add_query(
                    &mut m_queries,
                    &procedure,
                    sql,
                    &connection,
                    query_index,
                    &mut query_limit_reached,
                );
                break;
            }
        }
    }

    if connection.is_empty() {
        warnings.push(
            "No ADODB connection string was found. Generated M uses a placeholder data source."
                .to_string(),
        );
    }
    if m_queries.is_empty() {
        warnings.push(
            "No SQL-like VBA data-access blocks were detected in the source preview.".to_string(),
        );
    }
    if query_limit_reached {
        warnings.push(format!(
            "M query preview truncated to {MAX_QUERY_COUNT} candidates"
        ));
    }
    warnings.push(
        "Preview is heuristic and source-first; review the generated M before reuse.".to_string(),
    );

    let modified_vba = emit_modified_vba(&lines, &data_lines);
    let (modified_vba, modified_truncated) = bounded_text(&modified_vba, MAX_OUTPUT_CHARS);
    if modified_truncated {
        warnings.push(format!(
            "modified VBA preview truncated to {MAX_OUTPUT_CHARS} characters"
        ));
    }

    warnings.truncate(MAX_WARNINGS);

    TranslateVbaToMPreviewResult {
        mode: "heuristic_preview".to_string(),
        source_name,
        source_line_count: lines.len(),
        source_truncated,
        m_queries,
        modified_vba,
        warnings,
    }
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

fn add_query(
    queries: &mut Vec<MPreviewQuery>,
    procedure: &str,
    sql: &str,
    connection: &str,
    index: usize,
    query_limit_reached: &mut bool,
) {
    if queries.len() >= MAX_QUERY_COUNT {
        *query_limit_reached = true;
        return;
    }
    let normalized_sql = normalize_sql(sql);
    if queries
        .iter()
        .any(|query| query.native_sql.eq_ignore_ascii_case(&normalized_sql))
    {
        return;
    }

    let query_name = format!(
        "{}{}",
        sanitize_name(procedure),
        if index == 1 {
            "_Query".to_string()
        } else {
            format!("_Query{index}")
        }
    );
    let (code, _) = bounded_text(
        &generate_m(&query_name, &normalized_sql, connection),
        MAX_QUERY_CODE_CHARS,
    );
    queries.push(MPreviewQuery {
        name: query_name,
        source_procedure: procedure.to_string(),
        native_sql: normalized_sql,
        code,
    });
}

fn generate_m(query_name: &str, sql: &str, connection_string: &str) -> String {
    let _ = query_name;
    let source = build_source_expression(connection_string);
    let columns = extract_select_columns(sql);
    let mut builder = String::new();
    builder.push_str("let\n");
    builder.push_str("    Source = ");
    builder.push_str(&source);
    builder.push_str(",\n");
    builder.push_str("    QueryResult = Value.NativeQuery(Source, \"");
    builder.push_str(&escape_m_string(sql));
    builder.push_str("\")");

    if !columns.is_empty() {
        builder.push_str(",\n");
        builder.push_str("    SelectColumns = Table.SelectColumns(QueryResult, {");
        for (index, column) in columns.iter().enumerate() {
            if index > 0 {
                builder.push_str(", ");
            }
            builder.push('"');
            builder.push_str(&escape_m_string(column));
            builder.push('"');
        }
        builder.push_str("})\n");
        builder.push_str("in\n");
        builder.push_str("    SelectColumns");
    } else {
        builder.push('\n');
        builder.push_str("in\n");
        builder.push_str("    QueryResult");
    }

    builder
}

fn build_source_expression(connection_string: &str) -> String {
    if connection_string.trim().is_empty() {
        return "Sql.Database(\"SERVER\", \"DATABASE\")".to_string();
    }

    let provider = get_connection_value(connection_string, "Provider");
    let data_source = get_connection_value(connection_string, "Data Source");
    let server = first_non_empty(
        get_connection_value(connection_string, "Server"),
        &data_source,
    );
    let database = first_non_empty(
        get_connection_value(connection_string, "Initial Catalog"),
        &get_connection_value(connection_string, "Database"),
    );
    let dsn = get_connection_value(connection_string, "DSN");
    let dbq = get_connection_value(connection_string, "DBQ");

    if contains_ci(&provider, "Microsoft.ACE")
        || contains_ci(&provider, "Microsoft.Jet")
        || !dbq.trim().is_empty()
    {
        let path = first_non_empty(dbq, &data_source);
        return format!(
            "Access.Database(File.Contents(\"{}\"), [CreateNavigationProperties=true])",
            escape_m_string(&path)
        );
    }

    if !dsn.trim().is_empty() {
        return format!("Odbc.DataSource(\"dsn={}\")", escape_m_string(&dsn));
    }

    if contains_ci(&provider, "MSDASQL") {
        return format!(
            "Odbc.DataSource(\"{}\")",
            escape_m_string(connection_string)
        );
    }

    let server = if server.trim().is_empty() {
        "SERVER".to_string()
    } else {
        server
    };
    let database = if database.trim().is_empty() {
        "DATABASE".to_string()
    } else {
        database
    };
    format!(
        "Sql.Database(\"{}\", \"{}\")",
        escape_m_string(&server),
        escape_m_string(&database)
    )
}

fn extract_select_columns(sql: &str) -> Vec<String> {
    let mut columns = Vec::new();
    let Some(caps) = SELECT_COLUMNS_RE.captures(sql) else {
        return columns;
    };

    let select_list = caps[1].trim();
    if select_list == "*" || select_list.to_ascii_lowercase().starts_with("top ") {
        return columns;
    }

    for raw in split_top_level(select_list, ',') {
        let mut column = raw.trim().to_string();
        column = ALIAS_SUFFIX_RE.replace(&column, "").to_string();
        if let Some(dot) = column.rfind('.') {
            column = column[dot + 1..].to_string();
        }
        column = column
            .trim_matches(['[', ']', '"', '`', ' '].as_ref())
            .to_string();
        if !column.is_empty() && !column.contains('(') {
            columns.push(column);
        }
    }

    columns
}

fn emit_modified_vba(lines: &[&str], data_lines: &BTreeSet<usize>) -> String {
    let mut builder = String::new();
    let mut inserted_header = false;

    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1;
        if data_lines.contains(&line_number) {
            if !inserted_header {
                builder.push_str(
                    "    ' VBA to M preview: data-access block was moved to generated Power Query M.\n",
                );
                inserted_header = true;
            }
            builder.push_str("    ' VBA to M preview replaced: ");
            builder.push_str(line.trim());
            builder.push('\n');
        } else {
            builder.push_str(line);
            builder.push('\n');
        }
    }

    builder
}

fn read_sql_variable(sql_vars: &HashMap<String, String>, variable_name: Option<&str>) -> String {
    variable_name
        .map(normalize_identifier)
        .and_then(|name| sql_vars.get(&name).cloned())
        .unwrap_or_default()
}

fn is_sql_variable(variable: &str) -> bool {
    contains_ci(variable, "sql")
        || contains_ci(variable, "query")
        || contains_ci(variable, "command")
}

fn looks_like_connection_string(value: &str) -> bool {
    contains_ci(value, "Provider=")
        || contains_ci(value, "Data Source=")
        || contains_ci(value, "Server=")
        || contains_ci(value, "DSN=")
}

fn looks_like_sql(value: &str) -> bool {
    !value.trim().is_empty() && SQL_LOOKUP_RE.is_match(value)
}

fn normalize_sql(sql: &str) -> String {
    WHITESPACE_RE.replace_all(sql, " ").trim().to_string()
}

fn get_connection_value(connection_string: &str, key: &str) -> String {
    if connection_string.trim().is_empty() {
        return String::new();
    }
    let pattern = format!(r"(?:^|;)\s*{}\s*=\s*([^;]*)", regex::escape(key));
    let Ok(regex) = Regex::new(&pattern) else {
        return String::new();
    };
    regex
        .captures(connection_string)
        .and_then(|caps| caps.get(1).map(|value| value.as_str().trim().to_string()))
        .unwrap_or_default()
}

fn split_lines(value: &str) -> Vec<&str> {
    value.lines().collect()
}

fn split_top_level(value: &str, separator: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0usize;
    let mut depth = 0i32;
    let mut in_string = false;
    let chars = value.char_indices().collect::<Vec<_>>();
    let mut index = 0usize;

    while index < chars.len() {
        let (offset, ch) = chars[index];
        if ch == '\'' && !in_string {
            index += 1;
            continue;
        }
        if ch == '"' {
            if in_string && index + 1 < chars.len() && chars[index + 1].1 == '"' {
                index += 2;
                continue;
            }
            in_string = !in_string;
            index += 1;
            continue;
        }
        if in_string {
            index += 1;
            continue;
        }
        if ch == '(' {
            depth += 1;
        } else if ch == ')' {
            depth -= 1;
        } else if ch == separator && depth == 0 {
            result.push(value[start..offset].to_string());
            start = offset + ch.len_utf8();
        }
        index += 1;
    }

    result.push(value[start..].to_string());
    result
}

fn sanitize_name(value: &str) -> String {
    let cleaned = SANITIZE_NAME_RE.replace_all(value, "_").to_string();
    if cleaned.is_empty() {
        "Query".to_string()
    } else {
        cleaned
    }
}

fn unescape_vba_string(value: &str) -> String {
    value.replace("\"\"", "\"")
}

fn escape_m_string(value: &str) -> String {
    value.replace('"', "\"\"")
}

fn first_non_empty(first: String, second: &str) -> String {
    if !first.trim().is_empty() {
        first
    } else {
        second.to_string()
    }
}

fn contains_ci(value: &str, fragment: &str) -> bool {
    !value.is_empty()
        && value
            .to_ascii_lowercase()
            .contains(&fragment.to_ascii_lowercase())
}

fn normalize_identifier(value: &str) -> String {
    value.to_ascii_lowercase()
}

fn collect_unsupported_pattern_warnings(lines: &[&str], warnings: &mut Vec<String>) {
    for (pattern, warning) in UNSUPPORTED_PATTERN_WARNINGS {
        if lines.iter().any(|line| contains_ci(line, pattern)) {
            warnings.push(warning.to_string());
        }
    }
    if lines.iter().any(|line| line.trim_end().ends_with(" _")) {
        warnings.push(
            "VBA line continuations may hide SQL assembly that this preview does not reconstruct."
                .to_string(),
        );
    }
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
