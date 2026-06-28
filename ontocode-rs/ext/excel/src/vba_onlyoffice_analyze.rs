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

pub(crate) const ANALYZE_VBA_ONLYOFFICE_MIGRATION_TOOL_NAME: &str =
    "analyze_vba_onlyoffice_migration";

const ANALYZE_VBA_ONLYOFFICE_MIGRATION_DESCRIPTION: &str =
    "Analyze pasted VBA source for a bounded ONLYOFFICE spreadsheet macro migration preview.";
const MAX_SOURCE_CHARS: usize = 32_768;
const MAX_PROCEDURE_COUNT: usize = 16;
const MAX_OPERATION_COUNT: usize = 32;
const MAX_WARNINGS: usize = 16;

static PROCEDURE_START_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^\s*(?:Public\s+|Private\s+|Friend\s+|Static\s+)?Sub\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:\([^)]*\))?\s*$")
        .unwrap_or_else(|err| panic!("procedure regex should compile: {err}"))
});
static FUNCTION_START_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^\s*(?:Public\s+|Private\s+|Friend\s+|Static\s+)?Function\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:\([^)]*\))?\s*(?:As\s+[A-Za-z_][A-Za-z0-9_]*)?\s*$")
        .unwrap_or_else(|err| panic!("function regex should compile: {err}"))
});
static END_SUB_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^\s*End\s+Sub\s*$")
        .unwrap_or_else(|err| panic!("end sub regex should compile: {err}"))
});
static END_FUNCTION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^\s*End\s+Function\s*$")
        .unwrap_or_else(|err| panic!("end function regex should compile: {err}"))
});
static RGB_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^RGB\s*\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)$")
        .unwrap_or_else(|err| panic!("rgb regex should compile: {err}"))
});
static UNSUPPORTED_CALLS: [(&str, &str, &str); 12] = [
    (
        "(?i)\\bCreateObject\\s*\\(",
        "CreateObject",
        "COM automation is not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bGetObject\\s*\\(",
        "GetObject",
        "late-bound COM access is not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bCallByName\\s*\\(",
        "CallByName",
        "dynamic invocation is not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bShell\\s*\\(",
        "Shell",
        "shell execution is not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bOpen\\b.*\\bFor\\b",
        "FileOpen",
        "file I/O is not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bKill\\b",
        "Kill",
        "file deletion is not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bFileCopy\\b",
        "FileCopy",
        "file copy operations are not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bMkDir\\b",
        "MkDir",
        "directory creation is not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bOn\\s+Error\\b",
        "OnError",
        "error handling is not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bWith\\b",
        "WithBlock",
        "With blocks are not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bSelect\\s+Case\\b",
        "SelectCase",
        "branching blocks are not supported by the first ONLYOFFICE analyzer slice.",
    ),
    (
        "(?i)\\bEnviron\\s*\\(",
        "EnvironmentRead",
        "environment reads are not supported by the first ONLYOFFICE analyzer slice.",
    ),
];
static UNSUPPORTED_MEMBER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(.*?)(?:\.)(Merge|UnMerge|Sort|AutoFilter|AddComment|AddHyperlink|Comments\.Add|Hyperlinks\.Add|PasteSpecial|ClearContents|ClearFormats|Protect|Unprotect)\s*(?:(?:\(.*\))|(?:.+))?\s*$")
        .unwrap_or_else(|err| panic!("unsupported member regex should compile: {err}"))
});

#[derive(Clone, Default)]
pub(crate) struct ExcelAnalyzeVbaOnlyofficeMigrationTool {
    _thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct AnalyzeVbaOnlyofficeMigrationArgs {
    pub source_text: String,
    #[serde(default)]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct AnalyzeVbaProcedureSummary {
    pub name: String,
    pub kind: String,
    pub start_line: usize,
    pub end_line: usize,
    pub supported_operation_count: usize,
    pub unsupported_operation_count: usize,
    pub requires_manual_rewrite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct AnalyzeVbaOperationSummary {
    pub procedure: String,
    pub line: usize,
    pub operation: String,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct AnalyzeVbaOnlyofficeMigrationResult {
    pub procedures: Vec<AnalyzeVbaProcedureSummary>,
    pub supported_operations: Vec<AnalyzeVbaOperationSummary>,
    pub unsupported_operations: Vec<AnalyzeVbaOperationSummary>,
    pub requires_manual_rewrite: bool,
    pub warnings: Vec<String>,
    pub success: bool,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelAnalyzeVbaOnlyofficeMigrationTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, ANALYZE_VBA_ONLYOFFICE_MIGRATION_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(AnalyzeVbaOnlyofficeMigrationArgs))
                .unwrap_or_else(|err| {
                    panic!("analyze_vba_onlyoffice_migration args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(AnalyzeVbaOnlyofficeMigrationResult))
                .unwrap_or_else(|err| {
                    panic!("analyze_vba_onlyoffice_migration result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: ANALYZE_VBA_ONLYOFFICE_MIGRATION_TOOL_NAME.to_string(),
                    description: ANALYZE_VBA_ONLYOFFICE_MIGRATION_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("analyze_vba_onlyoffice_migration args schema should parse: {err}")
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
        let args = parse_tool_args::<AnalyzeVbaOnlyofficeMigrationArgs>(
            &call,
            "excel.analyze_vba_onlyoffice_migration",
        )?;
        let result =
            analyze_vba_onlyoffice_migration(&args.source_text, args.source_name.as_deref());
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize VBA migration analysis: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelAnalyzeVbaOnlyofficeMigrationTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self {
            _thread_state: thread_state,
        }
    }
}

pub(crate) fn analyze_vba_onlyoffice_migration(
    source_text: &str,
    source_name: Option<&str>,
) -> AnalyzeVbaOnlyofficeMigrationResult {
    let _ = source_name;
    let (source_text, source_truncated) = bounded_text(source_text, MAX_SOURCE_CHARS);
    let lines = split_lines(&source_text);
    let mut warnings = Vec::new();
    let mut procedures = Vec::new();
    let mut supported_operations = Vec::new();
    let mut unsupported_operations = Vec::new();
    let mut saw_supported_operation = false;
    let mut redacted_value_seen = false;
    let mut procedure_truncated = false;

    if source_truncated {
        warnings.push(format!(
            "source text truncated to {MAX_SOURCE_CHARS} characters for analysis"
        ));
    }

    let mut index = 0usize;
    while index < lines.len() {
        let raw_line = &lines[index];
        let line_number = index + 1;
        let line_buf = strip_comment(raw_line);
        let line = line_buf.trim();
        if line.is_empty() {
            index += 1;
            continue;
        }

        if let Some(caps) = PROCEDURE_START_RE.captures(line) {
            let name = caps[1].to_string();
            let start_line = line_number;
            let (end_index, event_like) = scan_procedure_end(&lines, index + 1, false);
            let mut summary = AnalyzeVbaProcedureSummary {
                name: name.clone(),
                kind: if event_like {
                    "event_sub".to_string()
                } else {
                    "sub".to_string()
                },
                start_line,
                end_line: end_index + 1,
                supported_operation_count: 0,
                unsupported_operation_count: 0,
                requires_manual_rewrite: event_like,
            };

            if event_like {
                push_bounded(
                    &mut unsupported_operations,
                    MAX_OPERATION_COUNT,
                    AnalyzeVbaOperationSummary {
                        procedure: name.clone(),
                        line: line_number,
                        operation: "EventProcedure".to_string(),
                        target: canonicalize_expression(line),
                        value: None,
                        reason: Some(
                            "event procedures are not supported by the first ONLYOFFICE analyzer slice."
                                .to_string(),
                        ),
                    },
                    &mut warnings,
                    "unsupported operations",
                );
            }

            let mut body_index = index + 1;
            while body_index < end_index {
                let body_line_number = body_index + 1;
                let body_raw = &lines[body_index];
                let body_buf = strip_comment(body_raw);
                let body_line = body_buf.trim();
                if body_line.is_empty() {
                    body_index += 1;
                    continue;
                }

                if let Some((operation, target, value, supported, reason, redacted)) =
                    classify_statement(body_line)
                {
                    let op = AnalyzeVbaOperationSummary {
                        procedure: name.clone(),
                        line: body_line_number,
                        operation,
                        target,
                        value,
                        reason,
                    };
                    if redacted {
                        redacted_value_seen = true;
                    }
                    if supported {
                        saw_supported_operation = true;
                        summary.supported_operation_count += 1;
                        push_bounded(
                            &mut supported_operations,
                            MAX_OPERATION_COUNT,
                            op,
                            &mut warnings,
                            "supported operations",
                        );
                    } else {
                        summary.unsupported_operation_count += 1;
                        summary.requires_manual_rewrite = true;
                        push_bounded(
                            &mut unsupported_operations,
                            MAX_OPERATION_COUNT,
                            op,
                            &mut warnings,
                            "unsupported operations",
                        );
                    }
                } else if !is_harmless_procedure_line(body_line) {
                    summary.unsupported_operation_count += 1;
                    summary.requires_manual_rewrite = true;
                    push_bounded(
                        &mut unsupported_operations,
                        MAX_OPERATION_COUNT,
                        AnalyzeVbaOperationSummary {
                            procedure: name.clone(),
                            line: body_line_number,
                            operation: "UnsupportedStatement".to_string(),
                            target: canonicalize_expression(body_line),
                            value: None,
                            reason: Some(
                                "unrecognized executable statements are not supported by the first ONLYOFFICE analyzer slice."
                                    .to_string(),
                            ),
                        },
                        &mut warnings,
                        "unsupported operations",
                    );
                }

                body_index += 1;
            }

            if summary.supported_operation_count == 0 && !summary.requires_manual_rewrite {
                summary.requires_manual_rewrite = true;
            }
            procedures.push(summary);
            if procedures.len() >= MAX_PROCEDURE_COUNT {
                procedure_truncated = end_index + 1 < lines.len();
                if procedure_truncated {
                    warnings.push(format!(
                        "procedure analysis truncated to {MAX_PROCEDURE_COUNT} procedures"
                    ));
                }
                break;
            }
            index = end_index + 1;
            continue;
        }

        if FUNCTION_START_RE.is_match(line) {
            let name = FUNCTION_START_RE
                .captures(line)
                .map(|caps| caps[1].to_string())
                .unwrap_or_else(|| "Function".to_string());
            let (end_index, _) = scan_procedure_end(&lines, index + 1, true);
            procedures.push(AnalyzeVbaProcedureSummary {
                name: name.clone(),
                kind: "function".to_string(),
                start_line: line_number,
                end_line: end_index + 1,
                supported_operation_count: 0,
                unsupported_operation_count: 1,
                requires_manual_rewrite: true,
            });
            push_bounded(
                &mut unsupported_operations,
                MAX_OPERATION_COUNT,
                AnalyzeVbaOperationSummary {
                    procedure: name,
                    line: line_number,
                    operation: "FunctionProcedure".to_string(),
                    target: canonicalize_expression(line),
                    value: None,
                    reason: Some(
                        "Function procedures are not supported by the first ONLYOFFICE analyzer slice."
                            .to_string(),
                    ),
                },
                &mut warnings,
                "unsupported operations",
            );
            index = end_index + 1;
            continue;
        }

        index += 1;
    }

    if procedures.is_empty() {
        warnings
            .push("No supported Sub procedures were detected in the source preview.".to_string());
    }
    if !saw_supported_operation {
        warnings.push(
            "No supported spreadsheet operations were detected in the source preview.".to_string(),
        );
    }
    if redacted_value_seen {
        warnings.push(
            "Sensitive-looking literals were redacted from the analysis preview.".to_string(),
        );
    }

    let requires_manual_rewrite = source_truncated
        || procedure_truncated
        || procedures
            .iter()
            .any(|procedure| procedure.requires_manual_rewrite)
        || !unsupported_operations.is_empty()
        || !saw_supported_operation;

    let success = !requires_manual_rewrite
        && !source_truncated
        && !procedure_truncated
        && !warnings.iter().any(|warning| warning.contains("truncated"));

    warnings.truncate(MAX_WARNINGS);

    AnalyzeVbaOnlyofficeMigrationResult {
        procedures,
        supported_operations,
        unsupported_operations,
        requires_manual_rewrite,
        warnings,
        success,
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

type ClassifiedStatement = (String, String, Option<String>, bool, Option<String>, bool);

fn classify_statement(statement: &str) -> Option<ClassifiedStatement> {
    if let Some((operation, reason)) = classify_unsupported_keyword(statement) {
        return Some((
            operation.to_string(),
            canonicalize_expression(statement),
            None,
            false,
            Some(reason.to_string()),
            false,
        ));
    }

    if let Some(caps) = UNSUPPORTED_MEMBER_RE.captures(statement) {
        let target = canonicalize_expression(statement);
        return Some((
            caps[2].to_string(),
            target,
            None,
            false,
            Some(
                "this spreadsheet operation is deferred from the first ONLYOFFICE analyzer slice."
                    .to_string(),
            ),
            false,
        ));
    }

    let (lhs, rhs) = split_assignment(statement)?;
    if !looks_like_supported_target(&lhs) {
        return None;
    }

    let target = canonicalize_expression(&lhs);
    let rhs = rhs.trim();
    let operation = classify_supported_operation(&lhs);
    let Some(operation) = operation else {
        return Some((
            "UnsupportedAssignment".to_string(),
            target,
            None,
            false,
            Some("assignment target is outside the supported recorder subset.".to_string()),
            false,
        ));
    };

    match operation.as_str() {
        "SetCellValue" => {
            if let Some(value) = render_rhs_literal(rhs) {
                let redacted = value == "<redacted>";
                return Some((operation, target, Some(value), true, None, redacted));
            }
            Some((
                operation,
                target,
                None,
                false,
                Some("cell value assignments must use a literal or redacted literal.".to_string()),
                false,
            ))
        }
        "SetCellFormula" => {
            let Some(value) = render_rhs_string(rhs) else {
                return Some((
                    operation,
                    target,
                    None,
                    false,
                    Some("cell formulas must use a string literal that starts with =.".to_string()),
                    false,
                ));
            };
            if !value.trim_start_matches('"').starts_with('=') {
                return Some((
                    operation,
                    target,
                    None,
                    false,
                    Some("cell formulas must start with =.".to_string()),
                    false,
                ));
            }
            let redacted = value == "<redacted>";
            Some((operation, target, Some(value), true, None, redacted))
        }
        "SetFontBold" | "SetFontItalic" | "SetWrap" => {
            if is_boolean_literal(rhs) {
                Some((
                    operation,
                    target,
                    Some(rhs.trim().to_ascii_lowercase()),
                    true,
                    None,
                    false,
                ))
            } else {
                Some((
                    operation,
                    target,
                    None,
                    false,
                    Some("the property expects a boolean literal.".to_string()),
                    false,
                ))
            }
        }
        "SetFontSize" => {
            if is_numeric_literal(rhs) {
                Some((
                    operation,
                    target,
                    Some(rhs.trim().to_string()),
                    true,
                    None,
                    false,
                ))
            } else {
                Some((
                    operation,
                    target,
                    None,
                    false,
                    Some("font size expects a numeric literal.".to_string()),
                    false,
                ))
            }
        }
        "SetFontName" | "SetNumberFormat" => {
            let Some(value) = render_rhs_string(rhs) else {
                return Some((
                    operation,
                    target,
                    None,
                    false,
                    Some("the property expects a string literal.".to_string()),
                    false,
                ));
            };
            let redacted = value == "<redacted>";
            Some((operation, target, Some(value), true, None, redacted))
        }
        "SetTextColor" | "SetFillColor" => {
            let Some(value) = render_rgb(rhs) else {
                return Some((
                    operation,
                    target,
                    None,
                    false,
                    Some("the color property expects RGB(r, g, b).".to_string()),
                    false,
                ));
            };
            Some((operation, target, Some(value), true, None, false))
        }
        "SetAlignHorizontal" | "SetAlignVertical" => {
            if is_numeric_literal(rhs) {
                Some((
                    operation,
                    target,
                    Some(rhs.trim().to_string()),
                    true,
                    None,
                    false,
                ))
            } else if let Some(value) = render_rhs_string(rhs) {
                let redacted = value == "<redacted>";
                Some((operation, target, Some(value), true, None, redacted))
            } else {
                Some((
                    operation,
                    target,
                    None,
                    false,
                    Some("alignment expects a numeric literal or quoted string.".to_string()),
                    false,
                ))
            }
        }
        _ => Some((
            operation,
            target,
            None,
            false,
            Some(
                "assignment target is not supported by the first ONLYOFFICE analyzer slice."
                    .to_string(),
            ),
            false,
        )),
    }
}

fn classify_supported_operation(lhs: &str) -> Option<String> {
    let normalized = compact_whitespace(lhs);
    if !is_supported_root(&normalized) {
        return None;
    }
    let operation = if normalized.ends_with(".Value") || normalized.ends_with(".Text") {
        "SetCellValue"
    } else if normalized.ends_with(".Formula") || normalized.ends_with(".FormulaLocal") {
        "SetCellFormula"
    } else if normalized.ends_with(".Font.Bold") {
        "SetFontBold"
    } else if normalized.ends_with(".Font.Italic") {
        "SetFontItalic"
    } else if normalized.ends_with(".Font.Name") {
        "SetFontName"
    } else if normalized.ends_with(".Font.Size") {
        "SetFontSize"
    } else if normalized.ends_with(".Font.Color") {
        "SetTextColor"
    } else if normalized.ends_with(".Interior.Color") {
        "SetFillColor"
    } else if normalized.ends_with(".NumberFormat") {
        "SetNumberFormat"
    } else if normalized.ends_with(".WrapText") {
        "SetWrap"
    } else if normalized.ends_with(".HorizontalAlignment") {
        "SetAlignHorizontal"
    } else if normalized.ends_with(".VerticalAlignment") {
        "SetAlignVertical"
    } else {
        return None;
    };
    Some(operation.to_string())
}

fn is_supported_root(lhs: &str) -> bool {
    let normalized = lhs.trim_start();
    normalized.starts_with("Selection")
        || normalized.starts_with("ActiveCell")
        || normalized.starts_with("Range(")
        || normalized.starts_with("Cells(")
}

fn looks_like_supported_target(lhs: &str) -> bool {
    let normalized = compact_whitespace(lhs);
    is_supported_root(&normalized)
        && normalized.contains('.')
        && !normalized.starts_with("Dim ")
        && !normalized.starts_with("Set ")
}

fn classify_unsupported_keyword(statement: &str) -> Option<(&'static str, &'static str)> {
    for (pattern, operation, reason) in UNSUPPORTED_CALLS {
        if Regex::new(pattern)
            .ok()
            .is_some_and(|regex| regex.is_match(statement))
        {
            return Some((operation, reason));
        }
    }
    None
}

fn render_rhs_literal(rhs: &str) -> Option<String> {
    if let Some(value) = render_rhs_string(rhs) {
        return Some(value);
    }
    if is_boolean_literal(rhs) {
        return Some(rhs.trim().to_ascii_lowercase());
    }
    if is_numeric_literal(rhs) {
        return Some(rhs.trim().to_string());
    }
    if let Some(value) = render_rgb(rhs) {
        return Some(value);
    }
    None
}

fn render_rhs_string(rhs: &str) -> Option<String> {
    let trimmed = rhs.trim();
    if !trimmed.starts_with('"') || !trimmed.ends_with('"') || trimmed.len() < 2 {
        return None;
    }
    let inner = trimmed[1..trimmed.len() - 1].replace("\"\"", "\"");
    if is_secret_like_literal(&inner) {
        Some("<redacted>".to_string())
    } else {
        Some(format!("\"{inner}\""))
    }
}

fn render_rgb(rhs: &str) -> Option<String> {
    let trimmed = rhs.trim();
    let caps = RGB_RE.captures(trimmed)?;
    let red: u8 = caps[1].parse().ok()?;
    let green: u8 = caps[2].parse().ok()?;
    let blue: u8 = caps[3].parse().ok()?;
    Some(format!("RGB({red}, {green}, {blue})"))
}

fn is_secret_like_literal(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    let secret_markers = [
        "password",
        "passwd",
        "pwd",
        "token",
        "secret",
        "bearer",
        "authorization",
        "credential",
        "keychain",
        "client_secret",
        "api_key",
        "private_key",
        "provider=",
        "data source=",
        "server=",
        "dsn=",
        "uid=",
        "user id=",
    ];
    if secret_markers.iter().any(|marker| lower.contains(marker)) {
        return true;
    }
    if trimmed.contains("://") && trimmed.contains('@') {
        return true;
    }
    if trimmed.starts_with("C:\\")
        || trimmed.starts_with("\\\\")
        || trimmed.starts_with('/')
        || trimmed.contains('\\')
    {
        return true;
    }
    trimmed.len() > 96
}

fn canonicalize_expression(value: &str) -> String {
    let compact = compact_whitespace(value);
    let mut output = String::new();
    let mut in_string = false;
    let mut chars = compact.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '"' {
            if in_string && matches!(chars.peek(), Some('"')) {
                output.push('"');
                chars.next();
                continue;
            }
            in_string = !in_string;
            output.push(ch);
            continue;
        }
        if in_string {
            output.push(ch);
            continue;
        }
        output.push(ch);
    }
    redact_literals_in_expression(&output)
}

fn redact_literals_in_expression(value: &str) -> String {
    let mut output = String::new();
    let mut in_string = false;
    let mut current = String::new();
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '"' {
            if in_string && matches!(chars.peek(), Some('"')) {
                current.push('"');
                chars.next();
                continue;
            }
            if in_string {
                let literal = std::mem::take(&mut current);
                if is_secret_like_literal(&literal) {
                    output.push_str("<redacted>");
                } else {
                    output.push('"');
                    output.push_str(&literal);
                    output.push('"');
                }
            }
            in_string = !in_string;
            continue;
        }
        if in_string {
            current.push(ch);
        } else {
            output.push(ch);
        }
    }
    if in_string {
        output.push_str("<redacted>");
    }
    output
}

fn compact_whitespace(value: &str) -> String {
    let mut output = String::new();
    let mut in_string = false;
    let mut previous_was_space = false;
    let chars = value.chars().peekable();
    for ch in chars {
        if ch == '"' {
            output.push(ch);
            in_string = !in_string;
            previous_was_space = false;
            continue;
        }
        if in_string {
            output.push(ch);
            continue;
        }
        if ch.is_whitespace() {
            if !previous_was_space {
                output.push(' ');
                previous_was_space = true;
            }
        } else {
            output.push(ch);
            previous_was_space = false;
        }
    }
    output.trim().to_string()
}

fn split_assignment(statement: &str) -> Option<(String, String)> {
    let mut in_string = false;
    let chars = statement.char_indices().collect::<Vec<_>>();
    let mut index = 0usize;
    while index < chars.len() {
        let (offset, ch) = chars[index];
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
        if ch == '=' {
            let lhs = statement[..offset].trim();
            let rhs = statement[offset + ch.len_utf8()..].trim();
            if lhs.is_empty() || rhs.is_empty() {
                return None;
            }
            if lhs.ends_with('<') || lhs.ends_with('>') || lhs.ends_with(':') {
                return None;
            }
            return Some((lhs.to_string(), rhs.to_string()));
        }
        index += 1;
    }
    None
}

fn is_boolean_literal(value: &str) -> bool {
    matches!(value.trim().to_ascii_lowercase().as_str(), "true" | "false")
}

fn is_numeric_literal(value: &str) -> bool {
    value.trim().parse::<f64>().is_ok()
}

fn strip_comment(line: &str) -> String {
    let mut output = String::new();
    let mut in_string = false;
    let chars = line.chars().peekable();
    for ch in chars {
        if ch == '"' {
            in_string = !in_string;
            output.push(ch);
            continue;
        }
        if ch == '\'' && !in_string {
            break;
        }
        output.push(ch);
    }
    output
}

fn split_lines(value: &str) -> Vec<String> {
    let mut lines = value.lines().map(ToString::to_string).collect::<Vec<_>>();
    let mut anchor = None;

    for index in 0..lines.len() {
        let target = anchor.unwrap_or(index);
        if index != target {
            let continuation = lines[index].trim_start().to_string();
            let base = trim_vba_line_continuation(&lines[target]);
            lines[target] = if continuation.is_empty() {
                base
            } else {
                format!("{base} {continuation}")
            };
            lines[index].clear();
        }
        if line_has_vba_continuation(&lines[target]) {
            anchor = Some(target);
        } else {
            anchor = None;
        }
    }

    lines
}

fn line_has_vba_continuation(line: &str) -> bool {
    strip_comment(line).trim_end().ends_with('_')
}

fn trim_vba_line_continuation(line: &str) -> String {
    let mut trimmed = line.trim_end().to_string();
    if trimmed.ends_with('_') {
        trimmed.pop();
    }
    trimmed.trim_end().to_string()
}

fn is_harmless_procedure_line(statement: &str) -> bool {
    let normalized = compact_whitespace(statement).to_ascii_uppercase();
    matches!(
        normalized.as_str(),
        "END IF"
            | "END SELECT"
            | "ELSE"
            | "ELSEIF"
            | "CASE"
            | "CASE ELSE"
            | "NEXT"
            | "LOOP"
            | "WEND"
    ) || normalized.starts_with("DIM ")
        || normalized.starts_with("CONST ")
        || normalized.starts_with("STATIC ")
        || normalized.starts_with("ATTRIBUTE ")
        || normalized.starts_with("OPTION ")
        || normalized.starts_with("REM ")
        || normalized.starts_with("DECLARE ")
        || normalized.starts_with("EXIT ")
}

fn scan_procedure_end(lines: &[String], start_index: usize, function: bool) -> (usize, bool) {
    let mut index = start_index;
    let mut event_like = false;
    if start_index > 0 {
        let start_line = strip_comment(&lines[start_index - 1]).trim().to_string();
        if let Some(caps) = PROCEDURE_START_RE.captures(&start_line) {
            event_like = is_event_procedure_name(&caps[1]);
        }
    }
    while index < lines.len() {
        let trimmed = strip_comment(&lines[index]).trim().to_string();
        if function {
            if END_FUNCTION_RE.is_match(&trimmed) {
                return (index, event_like);
            }
        } else if END_SUB_RE.is_match(&trimmed) {
            return (index, event_like);
        }
        index += 1;
    }
    (lines.len().saturating_sub(1), event_like)
}

fn is_event_procedure_name(name: &str) -> bool {
    matches!(
        name.split('_').next(),
        Some("Workbook" | "Worksheet" | "Application" | "Chart" | "Document")
    )
}

fn push_bounded<T>(
    values: &mut Vec<T>,
    max_items: usize,
    value: T,
    warnings: &mut Vec<String>,
    label: &str,
) {
    if values.len() < max_items {
        values.push(value);
    } else if !warnings
        .iter()
        .any(|warning| warning.contains(label) && warning.contains("truncated"))
    {
        warnings.push(format!("{label} truncated to {max_items} entries"));
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
