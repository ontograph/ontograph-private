use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use ontocode_extension_api::FunctionCallError;
use ontocode_extension_api::JsonToolOutput;
use ontocode_extension_api::ToolCall;
use ontocode_extension_api::ToolExecutor;
use ontocode_extension_api::ToolName;
use ontocode_extension_api::ToolOutput;
use ontocode_extension_api::ToolSpec;
use ontocode_extension_api::parse_tool_input_schema;
use quick_xml::Reader;
use quick_xml::events::Event;
use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_value;
use zip::ZipArchive;

use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;

pub(crate) const EXTRACT_POWERQUERY_QUERIES_TOOL_NAME: &str = "extract_powerquery_queries";

const EXTRACT_POWERQUERY_QUERIES_DESCRIPTION: &str = "Read Power Query M definitions from a workbook and return bounded read-only query text, metadata, and lexical lint findings.";
const MAX_CUSTOM_XML_ENTRY_BYTES: usize = 8 * 1024 * 1024;
const MAX_CUSTOM_XML_SCAN_ENTRIES: usize = 64;
const MAX_CUSTOM_XML_SCAN_BYTES: usize = 8 * 1024 * 1024;
const MAX_QUERY_COUNT: usize = 16;
const MAX_QUERY_SOURCE_CHARS: usize = 4096;
const MAX_PART_NAME_CHARS: usize = 256;
const MAX_CONNECTION_TEXT_CHARS: usize = 512;
const MAX_CONNECTION_COUNT: usize = 64;
const MAX_CONNECTION_IDS_PER_QUERY: usize = 8;
const MAX_WORKSHEET_LOAD_TARGETS_PER_QUERY: usize = 8;
const MAX_DATA_MODEL_LOAD_TARGETS_PER_QUERY: usize = 8;
const MAX_DATA_MODEL_PIVOT_CONSUMERS_PER_QUERY: usize = 16;
const MAX_LEXICAL_SOURCE_EXCERPT_CHARS: usize = 160;
const MAX_LINT_FINDINGS_PER_QUERY: usize = 8;
const MAX_WARNINGS: usize = 16;

#[derive(Clone, Default)]
pub(crate) struct ExcelExtractPowerQueryQueriesTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ExtractPowerQueryQueriesArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PowerQueryLexicalReferenceKind {
    QueryName,
    WorkbookName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PowerQueryLexicalEvidenceKind {
    SharedQueryIdentifier,
    ExcelCurrentWorkbookName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct PowerQueryLexicalReference {
    pub kind: PowerQueryLexicalReferenceKind,
    pub target_name: String,
    pub evidence_kind: PowerQueryLexicalEvidenceKind,
    pub source_line: usize,
    pub source_excerpt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PowerQueryLintCode {
    EmptyQuerySource,
    EmptyQueryBody,
    MissingSharedQueryDefinition,
    MissingInClauseForLetExpression,
    MissingLetClauseForInExpression,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct PowerQueryLintFinding {
    pub code: PowerQueryLintCode,
    pub message: String,
    pub source_line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ExtractedPowerQueryQuery {
    pub name: String,
    pub source_part: String,
    pub source: String,
    pub source_truncated: bool,
    pub lint_findings: Vec<PowerQueryLintFinding>,
    pub lexical_references: Vec<PowerQueryLexicalReference>,
    pub connection_name: Option<String>,
    pub location: Option<String>,
    pub command_preview: Option<String>,
    pub command_type: Option<String>,
    pub workbook_connection_ids: Vec<String>,
    pub load_target_hint: PowerQueryLoadTargetHint,
    pub worksheet_load_targets: Vec<PowerQueryWorksheetLoadTarget>,
    pub data_model_load_targets: Vec<PowerQueryDataModelLoadTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub(crate) enum PowerQueryLoadTargetHint {
    #[default]
    Unknown,
    WorkbookConnection,
    DataModel,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct PowerQueryWorkbookConnectionSummary {
    pub id: Option<String>,
    pub name: Option<String>,
    pub connection_type: Option<String>,
    pub location: Option<String>,
    pub command_preview: Option<String>,
    pub command_type: Option<String>,
    pub query_name_hint: Option<String>,
    pub load_target_hint: PowerQueryLoadTargetHint,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct PowerQueryWorksheetLoadTarget {
    pub external_data_name: Option<String>,
    pub table_name: Option<String>,
    pub sheet_name: Option<String>,
    pub range_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct PowerQueryDataModelLoadTarget {
    pub model_table_name: Option<String>,
    pub model_table_id: Option<String>,
    pub source_connection_name: Option<String>,
    pub source_workbook_connection_ids: Vec<String>,
    pub model_connection_ids: Vec<String>,
    pub pivot_consumers: Vec<PowerQueryDataModelPivotConsumer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct PowerQueryDataModelPivotConsumer {
    pub pivot_table_name: Option<String>,
    pub pivot_table_part: Option<String>,
    pub pivot_cache_id: Option<String>,
    pub source_connection_id: Option<String>,
    pub location_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ExtractPowerQueryQueriesResult {
    pub mode: String,
    pub path: String,
    pub has_power_query: bool,
    pub query_count: usize,
    pub lint_finding_count: usize,
    pub queries: Vec<ExtractedPowerQueryQuery>,
    pub connections: Vec<PowerQueryWorkbookConnectionSummary>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct WorkbookConnectionInventory {
    pub connection_count: usize,
    pub connections: Vec<PowerQueryWorkbookConnectionSummary>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct PowerQueryConnection {
    connection_id: Option<String>,
    connection_name: Option<String>,
    connection_type: Option<String>,
    location: Option<String>,
    command_preview: Option<String>,
    command_type: Option<String>,
    query_name_hint: Option<String>,
    load_target_hint: PowerQueryLoadTargetHint,
}

#[derive(Debug, Clone, Default)]
struct QueryConnectionMetadata {
    connection_name: Option<String>,
    location: Option<String>,
    command_preview: Option<String>,
    command_type: Option<String>,
    workbook_connection_ids: Vec<String>,
    load_target_hint: PowerQueryLoadTargetHint,
    worksheet_load_targets: Vec<PowerQueryWorksheetLoadTarget>,
    data_model_load_targets: Vec<PowerQueryDataModelLoadTarget>,
}

#[derive(Debug, Clone)]
struct QueryTableLink {
    part_name: String,
    connection_id: Option<String>,
    external_data_name: Option<String>,
}

#[derive(Debug, Clone)]
struct WorksheetLoadTargetCandidate {
    connection_id: Option<String>,
    external_data_name: Option<String>,
    table_name: Option<String>,
    sheet_name: Option<String>,
    range_ref: Option<String>,
}

#[derive(Debug, Clone)]
struct DataModelLoadTargetCandidate {
    query_name: String,
    model_table_name: Option<String>,
    model_table_id: Option<String>,
    source_connection_name: Option<String>,
}

#[derive(Debug, Clone)]
struct PivotCacheConnection {
    cache_id: Option<String>,
    source_connection_id: Option<String>,
}

#[derive(Debug, Clone)]
struct PivotTableConsumerCandidate {
    pivot_table_name: Option<String>,
    pivot_table_part: Option<String>,
    pivot_cache_id: Option<String>,
    source_connection_id: Option<String>,
    location_ref: Option<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelExtractPowerQueryQueriesTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, EXTRACT_POWERQUERY_QUERIES_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(ExtractPowerQueryQueriesArgs))
                .unwrap_or_else(|err| {
                    panic!("extract_powerquery_queries args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(ExtractPowerQueryQueriesResult))
                .unwrap_or_else(|err| {
                    panic!("extract_powerquery_queries result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: EXTRACT_POWERQUERY_QUERIES_TOOL_NAME.to_string(),
                    description: EXTRACT_POWERQUERY_QUERIES_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("extract_powerquery_queries args schema should parse: {err}")
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
        let args = parse_tool_args::<ExtractPowerQueryQueriesArgs>(
            &call,
            "excel.extract_powerquery_queries",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.extract_powerquery_queries workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = resolve_workbook_path_from_model_arg(&args.path, &cwd)?;
        let result =
            extract_powerquery_queries_from_workbook(&workbook_path, Path::new(args.path.trim()));
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize Power Query extraction result: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelExtractPowerQueryQueriesTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn extract_powerquery_queries_from_workbook(
    path: &Path,
    display_path: &Path,
) -> ExtractPowerQueryQueriesResult {
    let mut warnings = Vec::new();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            push_warning(
                &mut warnings,
                format!("failed to open workbook {}: {err}", display_path.display()),
            );
            return empty_result(display_path, warnings);
        }
    };
    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(err) => {
            push_warning(
                &mut warnings,
                format!(
                    "failed to read workbook archive {}: {err}",
                    display_path.display()
                ),
            );
            return empty_result(display_path, warnings);
        }
    };

    let connections = read_connections(&mut archive, &mut warnings);
    let worksheet_load_targets = read_worksheet_load_targets(&mut archive, &mut warnings);
    let data_model_load_targets = read_data_model_load_targets(&mut archive, &mut warnings);
    let data_model_pivot_consumers =
        read_data_model_pivot_consumers(&mut archive, &connections, &mut warnings);
    let (queries, saw_power_query_marker) = read_data_mashup_queries(
        &mut archive,
        &connections,
        &worksheet_load_targets,
        &data_model_load_targets,
        &data_model_pivot_consumers,
        &mut warnings,
    );
    let has_power_query = saw_power_query_marker || !queries.is_empty();

    if !has_power_query && warnings.is_empty() {
        push_warning(
            &mut warnings,
            "no Power Query DataMashup payload was found in the workbook".to_string(),
        );
    }

    let lint_finding_count = queries.iter().map(|query| query.lint_findings.len()).sum();

    ExtractPowerQueryQueriesResult {
        mode: "read_only_extraction".to_string(),
        path: display_path.display().to_string(),
        has_power_query,
        query_count: queries.len(),
        lint_finding_count,
        queries,
        connections: build_connection_summaries(&connections, &mut warnings),
        warnings,
    }
}

pub(crate) fn inspect_workbook_connection_inventory_from_workbook(
    path: &Path,
    display_path: &Path,
) -> WorkbookConnectionInventory {
    let mut warnings = Vec::new();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            push_warning(
                &mut warnings,
                format!("failed to open workbook {}: {err}", display_path.display()),
            );
            return WorkbookConnectionInventory {
                connection_count: 0,
                connections: Vec::new(),
                warnings,
            };
        }
    };
    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(err) => {
            push_warning(
                &mut warnings,
                format!(
                    "failed to read workbook archive {}: {err}",
                    display_path.display()
                ),
            );
            return WorkbookConnectionInventory {
                connection_count: 0,
                connections: Vec::new(),
                warnings,
            };
        }
    };

    let connections = read_connections(&mut archive, &mut warnings);
    let summaries = build_connection_summaries(&connections, &mut warnings);
    WorkbookConnectionInventory {
        connection_count: summaries.len(),
        connections: summaries,
        warnings,
    }
}

fn empty_result(path: &Path, warnings: Vec<String>) -> ExtractPowerQueryQueriesResult {
    ExtractPowerQueryQueriesResult {
        mode: "read_only_extraction".to_string(),
        path: path.display().to_string(),
        has_power_query: false,
        query_count: 0,
        lint_finding_count: 0,
        queries: Vec::new(),
        connections: Vec::new(),
        warnings,
    }
}

fn read_connections(
    archive: &mut ZipArchive<File>,
    warnings: &mut Vec<String>,
) -> Vec<PowerQueryConnection> {
    let Some(xml) = read_utf8_entry(archive, "xl/connections.xml", warnings) else {
        return Vec::new();
    };
    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut current = PowerQueryConnection::default();
    let mut in_connection = false;
    let mut connections = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) if event.name().as_ref() == b"connection" => {
                current = PowerQueryConnection {
                    connection_id: read_attr(&reader, &event, b"id")
                        .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0),
                    connection_name: read_attr(&reader, &event, b"name")
                        .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0),
                    connection_type: read_attr(&reader, &event, b"type")
                        .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0),
                    ..PowerQueryConnection::default()
                };
                current.query_name_hint = current
                    .connection_name
                    .as_deref()
                    .and_then(extract_query_name_hint);
                in_connection = true;
            }
            Ok(Event::Empty(event)) if event.name().as_ref() == b"connection" => {
                current = PowerQueryConnection {
                    connection_id: read_attr(&reader, &event, b"id")
                        .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0),
                    connection_name: read_attr(&reader, &event, b"name")
                        .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0),
                    connection_type: read_attr(&reader, &event, b"type")
                        .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0),
                    ..PowerQueryConnection::default()
                };
                current.query_name_hint = current
                    .connection_name
                    .as_deref()
                    .and_then(extract_query_name_hint);
                finalize_connection(&mut current);
                if has_connection_evidence(&current) {
                    if connections.len() >= MAX_CONNECTION_COUNT {
                        push_warning(
                            warnings,
                            format!(
                                "workbook connection inventory truncated to {MAX_CONNECTION_COUNT} entries"
                            ),
                        );
                        break;
                    }
                    connections.push(current.clone());
                }
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if in_connection && tag_matches(event.name().as_ref(), b"dbPr") =>
            {
                let connection = read_attr(&reader, &event, b"connection");
                let command = read_attr(&reader, &event, b"command").map(|value| {
                    let (bounded_command, truncated) =
                        bounded_text(&value, MAX_CONNECTION_TEXT_CHARS);
                    if truncated {
                        let connection_label = current
                            .connection_name
                            .as_deref()
                            .or(current.connection_id.as_deref())
                            .unwrap_or("unnamed");
                        push_warning(
                            warnings,
                            format!(
                                "command text for workbook connection `{connection_label}` truncated to {MAX_CONNECTION_TEXT_CHARS} characters"
                            ),
                        );
                    }
                    bounded_command
                });
                let command_type = read_attr(&reader, &event, b"commandType")
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
                let location = connection
                    .as_deref()
                    .and_then(extract_connection_location)
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
                if let Some(location_name) = location {
                    current.location = Some(location_name);
                }
                if let Some(command_preview) = command {
                    current.command_preview = Some(command_preview);
                }
                if let Some(command_type) = command_type {
                    current.command_type = Some(command_type);
                }
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if in_connection && tag_matches(event.name().as_ref(), b"connection") =>
            {
                if read_attr(&reader, &event, b"model").as_deref() == Some("1") {
                    current.load_target_hint = PowerQueryLoadTargetHint::DataModel;
                }
            }
            Ok(Event::End(event)) if event.name().as_ref() == b"connection" => {
                if in_connection {
                    finalize_connection(&mut current);
                    if has_connection_evidence(&current) {
                        if connections.len() >= MAX_CONNECTION_COUNT {
                            push_warning(
                                warnings,
                                format!(
                                    "workbook connection inventory truncated to {MAX_CONNECTION_COUNT} entries"
                                ),
                            );
                            break;
                        }
                        connections.push(current.clone());
                    }
                    in_connection = false;
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to parse workbook connections.xml: {err}"),
                );
                break;
            }
        }
        buf.clear();
    }

    connections
}

fn read_data_mashup_queries(
    archive: &mut ZipArchive<File>,
    connections: &[PowerQueryConnection],
    worksheet_load_targets: &[WorksheetLoadTargetCandidate],
    data_model_load_targets: &[DataModelLoadTargetCandidate],
    data_model_pivot_consumers: &[PivotTableConsumerCandidate],
    warnings: &mut Vec<String>,
) -> (Vec<ExtractedPowerQueryQuery>, bool) {
    let mut queries = Vec::new();
    let mut saw_power_query_marker = false;
    let mut scanned_entries = 0usize;
    let mut scanned_bytes = 0usize;

    for index in 0..archive.len() {
        let (entry_name, entry_size) = match archive.by_index(index) {
            Ok(entry) => {
                let entry_size =
                    usize::try_from(entry.size()).unwrap_or(MAX_CUSTOM_XML_ENTRY_BYTES + 1);
                (entry.name().replace('\\', "/"), entry_size)
            }
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to enumerate workbook entry {index}: {err}"),
                );
                continue;
            }
        };
        if !entry_name.starts_with("customXml/item") || !entry_name.ends_with(".xml") {
            continue;
        }

        scanned_entries = scanned_entries.saturating_add(1);
        if scanned_entries > MAX_CUSTOM_XML_SCAN_ENTRIES {
            push_warning(
                warnings,
                format!(
                    "custom XML scan exceeded {MAX_CUSTOM_XML_SCAN_ENTRIES} entries while searching for Power Query payloads"
                ),
            );
            break;
        }
        scanned_bytes = scanned_bytes.saturating_add(entry_size.min(MAX_CUSTOM_XML_ENTRY_BYTES));
        if scanned_bytes > MAX_CUSTOM_XML_SCAN_BYTES {
            push_warning(
                warnings,
                format!(
                    "custom XML scan exceeded {MAX_CUSTOM_XML_SCAN_BYTES} bytes while searching for Power Query payloads"
                ),
            );
            break;
        }

        let Some(xml) = read_text_entry_with_fallback(archive, &entry_name, warnings) else {
            continue;
        };
        let Some(payload) = extract_data_mashup_payload(&xml) else {
            continue;
        };
        saw_power_query_marker = true;
        let mashup_bytes = match BASE64_STANDARD.decode(payload.trim()) {
            Ok(bytes) => bytes,
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to decode DataMashup payload in {entry_name}: {err}"),
                );
                continue;
            }
        };
        let Some((start, end)) = find_embedded_zip_bounds(&mashup_bytes) else {
            push_warning(
                warnings,
                format!(
                    "DataMashup payload in {entry_name} does not contain a readable embedded zip package"
                ),
            );
            continue;
        };
        let cursor = Cursor::new(&mashup_bytes[start..end]);
        let mut embedded = match ZipArchive::new(cursor) {
            Ok(archive) => archive,
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to read embedded DataMashup package from {entry_name}: {err}"),
                );
                continue;
            }
        };

        for query in read_queries_from_embedded_archive(
            &mut embedded,
            connections,
            worksheet_load_targets,
            data_model_load_targets,
            data_model_pivot_consumers,
            warnings,
        ) {
            if queries.len() >= MAX_QUERY_COUNT {
                push_warning(
                    warnings,
                    format!("Power Query list truncated to {MAX_QUERY_COUNT} queries"),
                );
                return (queries, saw_power_query_marker);
            }
            queries.push(query);
        }
    }

    (queries, saw_power_query_marker)
}

fn read_queries_from_embedded_archive(
    archive: &mut ZipArchive<Cursor<&[u8]>>,
    connections: &[PowerQueryConnection],
    worksheet_load_targets: &[WorksheetLoadTargetCandidate],
    data_model_load_targets: &[DataModelLoadTargetCandidate],
    data_model_pivot_consumers: &[PivotTableConsumerCandidate],
    warnings: &mut Vec<String>,
) -> Vec<ExtractedPowerQueryQuery> {
    let mut queries = Vec::new();

    for index in 0..archive.len() {
        let mut entry = match archive.by_index(index) {
            Ok(entry) => entry,
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to enumerate DataMashup entry {index}: {err}"),
                );
                continue;
            }
        };
        let entry_name = entry.name().replace('\\', "/");
        if !entry_name.ends_with(".m") {
            continue;
        }
        let entry_size = match usize::try_from(entry.size()) {
            Ok(size) => size,
            Err(_) => {
                push_warning(
                    warnings,
                    format!("embedded DataMashup entry {entry_name} is too large to inspect"),
                );
                continue;
            }
        };
        if entry_size > MAX_CUSTOM_XML_ENTRY_BYTES {
            push_warning(
                warnings,
                format!(
                    "embedded DataMashup entry {entry_name} exceeds {MAX_CUSTOM_XML_ENTRY_BYTES} bytes"
                ),
            );
            continue;
        }
        let mut source = String::new();
        if let Err(err) = entry.read_to_string(&mut source) {
            push_warning(
                warnings,
                format!("failed to read embedded DataMashup entry {entry_name}: {err}"),
            );
            continue;
        }
        queries.extend(split_shared_queries(
            &entry_name,
            &source,
            connections,
            worksheet_load_targets,
            data_model_load_targets,
            data_model_pivot_consumers,
            warnings,
        ));
    }

    queries
}

fn split_shared_queries(
    entry_name: &str,
    source: &str,
    connections: &[PowerQueryConnection],
    worksheet_load_targets: &[WorksheetLoadTargetCandidate],
    data_model_load_targets: &[DataModelLoadTargetCandidate],
    data_model_pivot_consumers: &[PivotTableConsumerCandidate],
    warnings: &mut Vec<String>,
) -> Vec<ExtractedPowerQueryQuery> {
    let query_re =
        Regex::new(r#"(?im)^\s*shared\s+(?:#\"([^\"]+)\"|([A-Za-z_][A-Za-z0-9_]*))\s*="#)
            .unwrap_or_else(|err| panic!("Power Query matcher should compile: {err}"));
    let matches = query_re
        .captures_iter(source)
        .filter_map(|caps| {
            caps.get(0).map(|whole| {
                (
                    whole.start(),
                    caps.get(1)
                        .or_else(|| caps.get(2))
                        .map(|value| value.as_str().to_string())
                        .unwrap_or_else(|| "Query".to_string()),
                )
            })
        })
        .collect::<Vec<_>>();

    if matches.is_empty() {
        let lint_findings = lint_query_source(source, true);
        let (bounded_source, source_truncated) = bounded_text(source, MAX_QUERY_SOURCE_CHARS);
        let matching_connections = matching_connections_for_query("Section1", connections);
        let connection_metadata = build_query_connection_metadata(
            "Section1",
            &matching_connections,
            connections,
            worksheet_load_targets,
            data_model_load_targets,
            data_model_pivot_consumers,
            warnings,
        );
        return vec![build_query_result(
            "Section1".to_string(),
            entry_name,
            bounded_source,
            source_truncated,
            lint_findings,
            Vec::new(),
            connection_metadata,
        )];
    }

    let known_query_names = matches
        .iter()
        .map(|(_, name)| name.clone())
        .collect::<Vec<_>>();
    let mut queries = Vec::new();
    for (index, (start, name)) in matches.iter().enumerate() {
        if queries.len() >= MAX_QUERY_COUNT {
            push_warning(
                warnings,
                format!("Power Query list truncated to {MAX_QUERY_COUNT} queries"),
            );
            break;
        }
        let end = matches
            .get(index + 1)
            .map(|(next_start, _)| *next_start)
            .unwrap_or(source.len());
        let query_source = source[*start..end].trim();
        let lint_findings = lint_query_source(query_source, false);
        let lexical_references = collect_lexical_references(name, query_source, &known_query_names);
        let (bounded_source, source_truncated) = bounded_text(query_source, MAX_QUERY_SOURCE_CHARS);
        let matching_connections = matching_connections_for_query(name, connections);
        let connection_metadata = build_query_connection_metadata(
            name,
            &matching_connections,
            connections,
            worksheet_load_targets,
            data_model_load_targets,
            data_model_pivot_consumers,
            warnings,
        );
        queries.push(build_query_result(
            name.clone(),
            entry_name,
            bounded_source,
            source_truncated,
            lint_findings,
            lexical_references,
            connection_metadata,
        ));
    }

    queries
}

fn build_query_result(
    name: String,
    entry_name: &str,
    source: String,
    source_truncated: bool,
    lint_findings: Vec<PowerQueryLintFinding>,
    lexical_references: Vec<PowerQueryLexicalReference>,
    connection_metadata: QueryConnectionMetadata,
) -> ExtractedPowerQueryQuery {
    ExtractedPowerQueryQuery {
        name,
        source_part: bounded_text(entry_name, MAX_PART_NAME_CHARS).0,
        source,
        source_truncated,
        lint_findings,
        lexical_references,
        connection_name: connection_metadata.connection_name,
        location: connection_metadata.location,
        command_preview: connection_metadata.command_preview,
        command_type: connection_metadata.command_type,
        workbook_connection_ids: connection_metadata.workbook_connection_ids,
        load_target_hint: connection_metadata.load_target_hint,
        worksheet_load_targets: connection_metadata.worksheet_load_targets,
        data_model_load_targets: connection_metadata.data_model_load_targets,
    }
}

fn build_connection_summaries(
    connections: &[PowerQueryConnection],
    warnings: &mut Vec<String>,
) -> Vec<PowerQueryWorkbookConnectionSummary> {
    if connections.len() > MAX_CONNECTION_COUNT {
        push_warning(
            warnings,
            format!("workbook connection inventory truncated to {MAX_CONNECTION_COUNT} entries"),
        );
    }
    connections
        .iter()
        .take(MAX_CONNECTION_COUNT)
        .map(|connection| PowerQueryWorkbookConnectionSummary {
            id: connection.connection_id.clone(),
            name: connection.connection_name.clone(),
            connection_type: connection.connection_type.clone(),
            location: connection.location.clone(),
            command_preview: connection.command_preview.clone(),
            command_type: connection.command_type.clone(),
            query_name_hint: connection.query_name_hint.clone(),
            load_target_hint: connection.load_target_hint.clone(),
        })
        .collect()
}

fn matching_connections_for_query<'a>(
    query_name: &str,
    connections: &'a [PowerQueryConnection],
) -> Vec<&'a PowerQueryConnection> {
    let mut matches = connections
        .iter()
        .enumerate()
        .filter_map(|(index, connection)| {
            connection_match_priority(connection, query_name)
                .map(|priority| (priority, index, connection))
        })
        .collect::<Vec<_>>();
    matches.sort_by_key(|(priority, index, _)| (*priority, *index));
    matches
        .into_iter()
        .map(|(_, _, connection)| connection)
        .collect()
}

fn connection_match_priority(connection: &PowerQueryConnection, query_name: &str) -> Option<u8> {
    if connection.location.as_deref() == Some(query_name) {
        return Some(0);
    }
    if connection.query_name_hint.as_deref() == Some(query_name) {
        return Some(match connection.load_target_hint {
            PowerQueryLoadTargetHint::DataModel => 2,
            PowerQueryLoadTargetHint::WorkbookConnection => 1,
            PowerQueryLoadTargetHint::Unknown => 3,
        });
    }
    None
}

fn combined_load_target_hint(connections: &[&PowerQueryConnection]) -> PowerQueryLoadTargetHint {
    if connections
        .iter()
        .any(|connection| connection.load_target_hint == PowerQueryLoadTargetHint::DataModel)
    {
        PowerQueryLoadTargetHint::DataModel
    } else if connections.is_empty() {
        PowerQueryLoadTargetHint::Unknown
    } else {
        PowerQueryLoadTargetHint::WorkbookConnection
    }
}

fn build_query_connection_metadata(
    query_name: &str,
    connections: &[&PowerQueryConnection],
    all_connections: &[PowerQueryConnection],
    worksheet_load_targets: &[WorksheetLoadTargetCandidate],
    data_model_load_targets: &[DataModelLoadTargetCandidate],
    data_model_pivot_consumers: &[PivotTableConsumerCandidate],
    warnings: &mut Vec<String>,
) -> QueryConnectionMetadata {
    let primary_connection = connections.first().copied();
    QueryConnectionMetadata {
        connection_name: primary_connection.and_then(|value| value.connection_name.clone()),
        location: primary_connection.and_then(|value| value.location.clone()),
        command_preview: primary_connection.and_then(|value| value.command_preview.clone()),
        command_type: primary_connection.and_then(|value| value.command_type.clone()),
        workbook_connection_ids: bounded_connection_ids(query_name, connections, warnings),
        load_target_hint: combined_load_target_hint(connections),
        worksheet_load_targets: bounded_worksheet_load_targets(
            query_name,
            connections,
            worksheet_load_targets,
            warnings,
        ),
        data_model_load_targets: bounded_data_model_load_targets(
            query_name,
            data_model_load_targets,
            all_connections,
            data_model_pivot_consumers,
            warnings,
        ),
    }
}

fn bounded_data_model_load_targets(
    query_name: &str,
    data_model_load_targets: &[DataModelLoadTargetCandidate],
    connections: &[PowerQueryConnection],
    data_model_pivot_consumers: &[PivotTableConsumerCandidate],
    warnings: &mut Vec<String>,
) -> Vec<PowerQueryDataModelLoadTarget> {
    let mut output = Vec::new();
    for target in data_model_load_targets
        .iter()
        .filter(|target| target.query_name == query_name)
    {
        if output.len() >= MAX_DATA_MODEL_LOAD_TARGETS_PER_QUERY {
            push_warning(
                warnings,
                format!(
                    "Data Model load targets for Power Query `{query_name}` truncated to {MAX_DATA_MODEL_LOAD_TARGETS_PER_QUERY} entries"
                ),
            );
            break;
        }
        output.push(PowerQueryDataModelLoadTarget {
            model_table_name: target.model_table_name.clone(),
            model_table_id: target.model_table_id.clone(),
            source_connection_name: target.source_connection_name.clone(),
            source_workbook_connection_ids: data_model_source_connection_ids(target, connections),
            model_connection_ids: data_model_connection_ids(connections),
            pivot_consumers: bounded_data_model_pivot_consumers(
                query_name,
                data_model_pivot_consumers,
                warnings,
            ),
        });
    }
    output
}

fn data_model_source_connection_ids(
    target: &DataModelLoadTargetCandidate,
    connections: &[PowerQueryConnection],
) -> Vec<String> {
    let mut ids = BTreeSet::new();
    for connection in connections {
        let Some(id) = connection.connection_id.as_ref() else {
            continue;
        };
        let name_matches = target
            .source_connection_name
            .as_deref()
            .is_some_and(|name| connection.connection_name.as_deref() == Some(name));
        if name_matches || connection.query_name_hint.as_deref() == Some(target.query_name.as_str())
        {
            ids.insert(id.clone());
        }
    }
    ids.into_iter().take(MAX_CONNECTION_IDS_PER_QUERY).collect()
}

fn data_model_connection_ids(connections: &[PowerQueryConnection]) -> Vec<String> {
    connections
        .iter()
        .filter(|connection| connection.load_target_hint == PowerQueryLoadTargetHint::DataModel)
        .filter_map(|connection| connection.connection_id.clone())
        .take(MAX_CONNECTION_IDS_PER_QUERY)
        .collect()
}

fn bounded_data_model_pivot_consumers(
    query_name: &str,
    data_model_pivot_consumers: &[PivotTableConsumerCandidate],
    warnings: &mut Vec<String>,
) -> Vec<PowerQueryDataModelPivotConsumer> {
    let mut output = Vec::new();
    let mut dedupe = BTreeSet::new();
    for consumer in data_model_pivot_consumers {
        let key = format!(
            "{}|{}|{}|{}|{}",
            consumer.pivot_table_name.as_deref().unwrap_or_default(),
            consumer.pivot_table_part.as_deref().unwrap_or_default(),
            consumer.pivot_cache_id.as_deref().unwrap_or_default(),
            consumer.source_connection_id.as_deref().unwrap_or_default(),
            consumer.location_ref.as_deref().unwrap_or_default()
        );
        if !dedupe.insert(key) {
            continue;
        }
        if output.len() >= MAX_DATA_MODEL_PIVOT_CONSUMERS_PER_QUERY {
            push_warning(
                warnings,
                format!(
                    "Data Model pivot consumers for Power Query `{query_name}` truncated to {MAX_DATA_MODEL_PIVOT_CONSUMERS_PER_QUERY} entries"
                ),
            );
            break;
        }
        output.push(PowerQueryDataModelPivotConsumer {
            pivot_table_name: consumer.pivot_table_name.clone(),
            pivot_table_part: consumer.pivot_table_part.clone(),
            pivot_cache_id: consumer.pivot_cache_id.clone(),
            source_connection_id: consumer.source_connection_id.clone(),
            location_ref: consumer.location_ref.clone(),
        });
    }
    output
}

fn bounded_worksheet_load_targets(
    query_name: &str,
    connections: &[&PowerQueryConnection],
    worksheet_load_targets: &[WorksheetLoadTargetCandidate],
    warnings: &mut Vec<String>,
) -> Vec<PowerQueryWorksheetLoadTarget> {
    let connection_ids = connections
        .iter()
        .filter_map(|connection| connection.connection_id.as_deref())
        .collect::<BTreeSet<_>>();
    if connection_ids.is_empty() {
        return Vec::new();
    }

    let mut dedupe = BTreeSet::new();
    let mut output = Vec::new();
    for target in worksheet_load_targets {
        let Some(connection_id) = target.connection_id.as_deref() else {
            continue;
        };
        if !connection_ids.contains(connection_id) {
            continue;
        }
        let key = format!(
            "{}|{}|{}|{}",
            target.external_data_name.as_deref().unwrap_or_default(),
            target.table_name.as_deref().unwrap_or_default(),
            target.sheet_name.as_deref().unwrap_or_default(),
            target.range_ref.as_deref().unwrap_or_default()
        );
        if !dedupe.insert(key) {
            continue;
        }
        if output.len() >= MAX_WORKSHEET_LOAD_TARGETS_PER_QUERY {
            push_warning(
                warnings,
                format!(
                    "worksheet load targets for Power Query `{query_name}` truncated to {MAX_WORKSHEET_LOAD_TARGETS_PER_QUERY} entries"
                ),
            );
            break;
        }
        output.push(PowerQueryWorksheetLoadTarget {
            external_data_name: target.external_data_name.clone(),
            table_name: target.table_name.clone(),
            sheet_name: target.sheet_name.clone(),
            range_ref: target.range_ref.clone(),
        });
    }
    output
}

fn bounded_connection_ids(
    query_name: &str,
    connections: &[&PowerQueryConnection],
    warnings: &mut Vec<String>,
) -> Vec<String> {
    let mut ids = Vec::new();
    for connection in connections {
        let Some(id) = connection.connection_id.as_ref() else {
            continue;
        };
        if ids.len() >= MAX_CONNECTION_IDS_PER_QUERY {
            push_warning(
                warnings,
                format!(
                    "workbook connection ids for Power Query `{query_name}` truncated to {MAX_CONNECTION_IDS_PER_QUERY} entries"
                ),
            );
            break;
        }
        ids.push(id.clone());
    }
    ids
}

fn finalize_connection(connection: &mut PowerQueryConnection) {
    if connection.load_target_hint != PowerQueryLoadTargetHint::DataModel
        && has_connection_evidence(connection)
    {
        connection.load_target_hint = PowerQueryLoadTargetHint::WorkbookConnection;
    }
    if connection.query_name_hint.is_none() {
        connection.query_name_hint = connection.location.clone();
    }
    if connection.query_name_hint.is_none()
        && connection.load_target_hint == PowerQueryLoadTargetHint::DataModel
    {
        connection.query_name_hint = connection
            .command_preview
            .as_deref()
            .and_then(extract_data_model_query_name_hint);
    }
}

fn has_connection_evidence(connection: &PowerQueryConnection) -> bool {
    connection.connection_id.is_some()
        || connection.connection_name.is_some()
        || connection.connection_type.is_some()
        || connection.location.is_some()
        || connection.command_preview.is_some()
        || connection.command_type.is_some()
}

fn read_worksheet_load_targets(
    archive: &mut ZipArchive<File>,
    warnings: &mut Vec<String>,
) -> Vec<WorksheetLoadTargetCandidate> {
    let query_tables = read_query_tables(archive, warnings);
    if query_tables.is_empty() {
        return Vec::new();
    }

    let workbook_ranges = read_external_data_ranges(archive, warnings);
    let table_links = read_table_query_table_links(archive, warnings);
    let table_link_by_query_table = table_links
        .into_iter()
        .map(|link| (link.query_table_part.clone(), link))
        .collect::<std::collections::BTreeMap<_, _>>();

    query_tables
        .into_iter()
        .map(|query_table| {
            let table_link = table_link_by_query_table.get(&query_table.part_name);
            let external_data_name = query_table.external_data_name;
            let workbook_range = external_data_name
                .as_deref()
                .and_then(|name| workbook_ranges.get(name));

            WorksheetLoadTargetCandidate {
                connection_id: query_table.connection_id,
                external_data_name,
                table_name: table_link.and_then(|link| link.table_name.clone()),
                sheet_name: workbook_range.map(|(sheet_name, _)| sheet_name.clone()),
                range_ref: workbook_range
                    .map(|(_, range_ref)| range_ref.clone())
                    .or_else(|| table_link.and_then(|link| link.range_ref.clone())),
            }
        })
        .filter(|target| {
            target.connection_id.is_some()
                || target.external_data_name.is_some()
                || target.table_name.is_some()
                || target.sheet_name.is_some()
                || target.range_ref.is_some()
        })
        .collect()
}

fn read_data_model_load_targets(
    archive: &mut ZipArchive<File>,
    warnings: &mut Vec<String>,
) -> Vec<DataModelLoadTargetCandidate> {
    let Some(xml) = read_utf8_entry(archive, "xl/workbook.xml", warnings) else {
        return Vec::new();
    };
    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut targets = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"modelTable") =>
            {
                let source_connection_name = read_attr(&reader, &event, b"connection")
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
                let query_name = source_connection_name
                    .as_deref()
                    .and_then(extract_query_name_hint)
                    .or_else(|| {
                        read_attr(&reader, &event, b"name")
                            .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0)
                    });
                let Some(query_name) = query_name else {
                    continue;
                };
                targets.push(DataModelLoadTargetCandidate {
                    query_name,
                    model_table_name: read_attr(&reader, &event, b"name")
                        .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0),
                    model_table_id: read_attr(&reader, &event, b"id")
                        .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0),
                    source_connection_name,
                });
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to parse workbook Data Model table routing: {err}"),
                );
                break;
            }
        }
        buf.clear();
    }

    targets
}

fn read_data_model_pivot_consumers(
    archive: &mut ZipArchive<File>,
    connections: &[PowerQueryConnection],
    warnings: &mut Vec<String>,
) -> Vec<PivotTableConsumerCandidate> {
    let data_model_connection_ids = connections
        .iter()
        .filter(|connection| connection.load_target_hint == PowerQueryLoadTargetHint::DataModel)
        .filter_map(|connection| connection.connection_id.as_deref())
        .collect::<BTreeSet<_>>();
    if data_model_connection_ids.is_empty() {
        return Vec::new();
    }

    let cache_connections = read_pivot_cache_connections(archive, warnings)
        .into_iter()
        .filter(|cache| {
            cache
                .source_connection_id
                .as_deref()
                .is_some_and(|id| data_model_connection_ids.contains(id))
        })
        .filter_map(|cache| Some((cache.cache_id?, cache.source_connection_id?)))
        .collect::<BTreeMap<_, _>>();
    if cache_connections.is_empty() {
        return Vec::new();
    }

    read_pivot_tables(archive, warnings)
        .into_iter()
        .filter_map(|mut pivot_table| {
            let cache_id = pivot_table.pivot_cache_id.as_ref()?;
            pivot_table.source_connection_id = cache_connections.get(cache_id).cloned();
            pivot_table.source_connection_id.as_ref()?;
            Some(pivot_table)
        })
        .collect()
}

fn read_pivot_cache_connections(
    archive: &mut ZipArchive<File>,
    warnings: &mut Vec<String>,
) -> Vec<PivotCacheConnection> {
    let pivot_cache_parts = read_workbook_pivot_cache_parts(archive, warnings);
    let pivot_cache_part_by_cache_id = pivot_cache_parts.into_iter().collect::<BTreeMap<_, _>>();

    pivot_cache_part_by_cache_id
        .into_iter()
        .filter_map(|(cache_id, part_name)| {
            let xml = read_utf8_entry(archive, &part_name, warnings)?;
            let source_connection_id =
                parse_pivot_cache_source_connection_id(&part_name, &xml, warnings);
            Some(PivotCacheConnection {
                cache_id: Some(cache_id),
                source_connection_id,
            })
        })
        .collect()
}

fn read_workbook_pivot_cache_parts(
    archive: &mut ZipArchive<File>,
    warnings: &mut Vec<String>,
) -> Vec<(String, String)> {
    let Some(workbook_xml) = read_utf8_entry(archive, "xl/workbook.xml", warnings) else {
        return Vec::new();
    };
    let Some(rels_xml) = read_utf8_entry(archive, "xl/_rels/workbook.xml.rels", warnings) else {
        return Vec::new();
    };
    let rel_targets = parse_relationship_targets("xl/workbook.xml", &rels_xml, warnings);
    let mut reader = Reader::from_str(&workbook_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut parts = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"pivotCache") =>
            {
                let Some(cache_id) = read_attr(&reader, &event, b"cacheId")
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0)
                else {
                    continue;
                };
                let Some(rel_id) = read_attr(&reader, &event, b"r:id")
                    .or_else(|| read_attr_local(&reader, &event, b"id"))
                else {
                    continue;
                };
                if let Some(part_name) = rel_targets.get(&rel_id) {
                    parts.push((cache_id, part_name.clone()));
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to parse workbook pivot cache routing: {err}"),
                );
                break;
            }
        }
        buf.clear();
    }

    parts
}

fn parse_relationship_targets(
    base_part_name: &str,
    xml: &str,
    warnings: &mut Vec<String>,
) -> BTreeMap<String, String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut rel_targets = BTreeMap::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"Relationship") =>
            {
                let Some(id) = read_attr(&reader, &event, b"Id") else {
                    continue;
                };
                let Some(target) = read_attr(&reader, &event, b"Target") else {
                    continue;
                };
                if let Some(target) = normalize_relationship_target(base_part_name, &target) {
                    rel_targets.insert(id, bounded_text(&target, MAX_PART_NAME_CHARS).0);
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to parse workbook relationship targets: {err}"),
                );
                break;
            }
        }
        buf.clear();
    }

    rel_targets
}

fn parse_pivot_cache_source_connection_id(
    part_name: &str,
    xml: &str,
    warnings: &mut Vec<String>,
) -> Option<String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"cacheSource") =>
            {
                return read_attr(&reader, &event, b"connectionId")
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to parse pivot cache source {part_name}: {err}"),
                );
                break;
            }
        }
        buf.clear();
    }

    None
}

fn read_pivot_tables(
    archive: &mut ZipArchive<File>,
    warnings: &mut Vec<String>,
) -> Vec<PivotTableConsumerCandidate> {
    archive_entry_names(archive)
        .into_iter()
        .filter(|name| name.starts_with("xl/pivotTables/pivotTable") && name.ends_with(".xml"))
        .filter_map(|entry_name| {
            let xml = read_utf8_entry(archive, &entry_name, warnings)?;
            parse_pivot_table(&entry_name, &xml, warnings)
        })
        .collect()
}

fn parse_pivot_table(
    entry_name: &str,
    xml: &str,
    warnings: &mut Vec<String>,
) -> Option<PivotTableConsumerCandidate> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut pivot_table_name = None;
    let mut pivot_cache_id = None;
    let mut location_ref = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"pivotTableDefinition") =>
            {
                pivot_table_name = read_attr(&reader, &event, b"name")
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
                pivot_cache_id = read_attr(&reader, &event, b"cacheId")
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"location") =>
            {
                location_ref = read_attr(&reader, &event, b"ref")
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to parse pivot table {entry_name}: {err}"),
                );
                break;
            }
        }
        buf.clear();
    }

    if pivot_table_name.is_none() && pivot_cache_id.is_none() && location_ref.is_none() {
        return None;
    }

    Some(PivotTableConsumerCandidate {
        pivot_table_name,
        pivot_table_part: Some(bounded_text(entry_name, MAX_PART_NAME_CHARS).0),
        pivot_cache_id,
        source_connection_id: None,
        location_ref,
    })
}

#[derive(Debug, Clone)]
struct TableQueryTableLink {
    query_table_part: String,
    table_name: Option<String>,
    range_ref: Option<String>,
}

fn read_query_tables(
    archive: &mut ZipArchive<File>,
    warnings: &mut Vec<String>,
) -> Vec<QueryTableLink> {
    archive_entry_names(archive)
        .into_iter()
        .filter(|name| name.starts_with("xl/queryTables/queryTable") && name.ends_with(".xml"))
        .filter_map(|entry_name| {
            let xml = read_utf8_entry(archive, &entry_name, warnings)?;
            parse_query_table(&entry_name, &xml, warnings)
        })
        .collect()
}

fn parse_query_table(
    entry_name: &str,
    xml: &str,
    warnings: &mut Vec<String>,
) -> Option<QueryTableLink> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"queryTable") =>
            {
                return Some(QueryTableLink {
                    part_name: entry_name.to_string(),
                    connection_id: read_attr(&reader, &event, b"connectionId")
                        .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0),
                    external_data_name: read_attr(&reader, &event, b"name")
                        .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0),
                });
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to parse worksheet query table {entry_name}: {err}"),
                );
                break;
            }
        }
        buf.clear();
    }

    None
}

fn read_table_query_table_links(
    archive: &mut ZipArchive<File>,
    warnings: &mut Vec<String>,
) -> Vec<TableQueryTableLink> {
    archive_entry_names(archive)
        .into_iter()
        .filter(|name| name.starts_with("xl/tables/table") && name.ends_with(".xml"))
        .filter_map(|entry_name| {
            let xml = read_utf8_entry(archive, &entry_name, warnings)?;
            parse_table_query_table_link(archive, &entry_name, &xml, warnings)
        })
        .collect()
}

fn parse_table_query_table_link(
    archive: &mut ZipArchive<File>,
    entry_name: &str,
    xml: &str,
    warnings: &mut Vec<String>,
) -> Option<TableQueryTableLink> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut table_name = None;
    let mut range_ref = None;
    let mut is_query_table = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"table") =>
            {
                table_name = read_attr(&reader, &event, b"displayName")
                    .or_else(|| read_attr(&reader, &event, b"name"))
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
                range_ref = read_attr(&reader, &event, b"ref")
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
                is_query_table =
                    read_attr(&reader, &event, b"tableType").as_deref() == Some("queryTable");
                break;
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to parse worksheet table {entry_name}: {err}"),
                );
                return None;
            }
        }
        buf.clear();
    }

    if !is_query_table {
        return None;
    }

    let rels_entry_name = table_relationship_part_name(entry_name)?;
    let rels_xml = read_utf8_entry(archive, &rels_entry_name, warnings)?;
    let query_table_part =
        parse_table_relationship_target(entry_name, &rels_entry_name, &rels_xml, warnings)?;
    Some(TableQueryTableLink {
        query_table_part,
        table_name,
        range_ref,
    })
}

fn table_relationship_part_name(table_part_name: &str) -> Option<String> {
    let (parent, file_name) = table_part_name.rsplit_once('/')?;
    Some(format!("{parent}/_rels/{file_name}.rels"))
}

fn parse_table_relationship_target(
    table_part_name: &str,
    rels_entry_name: &str,
    xml: &str,
    warnings: &mut Vec<String>,
) -> Option<String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"Relationship") =>
            {
                let target = read_attr(&reader, &event, b"Target")?;
                return normalize_relationship_target(table_part_name, &target)
                    .map(|value| bounded_text(&value, MAX_PART_NAME_CHARS).0);
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to parse table relationship part {rels_entry_name}: {err}"),
                );
                break;
            }
        }
        buf.clear();
    }

    None
}

fn normalize_relationship_target(base_part_name: &str, target: &str) -> Option<String> {
    if let Some(stripped) = target.strip_prefix('/') {
        let normalized = stripped.replace('\\', "/");
        return if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        };
    }

    let mut parts = base_part_name
        .replace('\\', "/")
        .split('/')
        .map(str::to_string)
        .collect::<Vec<_>>();
    parts.pop();

    for segment in target.replace('\\', "/").split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            value => parts.push(value.to_string()),
        }
    }

    Some(parts.join("/"))
}

fn read_external_data_ranges(
    archive: &mut ZipArchive<File>,
    warnings: &mut Vec<String>,
) -> std::collections::BTreeMap<String, (String, String)> {
    let Some(xml) = read_utf8_entry(archive, "xl/workbook.xml", warnings) else {
        return std::collections::BTreeMap::new();
    };
    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut current_defined_name = None;
    let mut current_defined_name_text = String::new();
    let mut ranges = std::collections::BTreeMap::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) if tag_matches(event.name().as_ref(), b"definedName") => {
                current_defined_name = read_attr(&reader, &event, b"name")
                    .filter(|value| value.starts_with("ExternalData_"));
                current_defined_name_text.clear();
            }
            Ok(Event::Text(text)) if current_defined_name.is_some() => {
                if let Ok(decoded) = text.decode() {
                    current_defined_name_text.push_str(decoded.as_ref());
                }
            }
            Ok(Event::GeneralRef(reference)) if current_defined_name.is_some() => {
                if let Ok(Some(value)) = reference.resolve_char_ref() {
                    current_defined_name_text.push(value);
                } else if let Ok(decoded) = reference.decode() {
                    current_defined_name_text.push_str(decoded.as_ref());
                }
            }
            Ok(Event::End(event)) if tag_matches(event.name().as_ref(), b"definedName") => {
                if let Some(name) = current_defined_name.take()
                    && let Some((sheet_name, range_ref)) =
                        parse_defined_name_sheet_target(&current_defined_name_text)
                {
                    ranges.insert(name, (sheet_name, range_ref));
                }
                current_defined_name_text.clear();
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                push_warning(
                    warnings,
                    format!("failed to parse workbook defined names: {err}"),
                );
                break;
            }
        }
        buf.clear();
    }

    ranges
}

fn parse_defined_name_sheet_target(target: &str) -> Option<(String, String)> {
    let (sheet_name, range_ref) = target.trim().split_once('!')?;
    let sheet_name = sheet_name.trim().trim_matches('\'').replace("''", "'");
    let range_ref = range_ref.trim();
    if sheet_name.is_empty() || range_ref.is_empty() {
        return None;
    }
    Some((
        bounded_text(&sheet_name, MAX_CONNECTION_TEXT_CHARS).0,
        bounded_text(range_ref, MAX_CONNECTION_TEXT_CHARS).0,
    ))
}

fn archive_entry_names(archive: &mut ZipArchive<File>) -> Vec<String> {
    let mut names = Vec::new();
    for index in 0..archive.len() {
        if let Ok(entry) = archive.by_index(index) {
            names.push(entry.name().replace('\\', "/"));
        }
    }
    names
}

fn extract_query_name_hint(connection_name: &str) -> Option<String> {
    for prefix in [
        "Query - ",
        "Query – ",
        "Query — ",
        "Запрос - ",
        "Запрос – ",
        "Запрос — ",
    ] {
        let Some(rest) = connection_name.trim().strip_prefix(prefix) else {
            continue;
        };
        let trimmed = rest.trim();
        if !trimmed.is_empty() {
            return Some(bounded_text(trimmed, MAX_CONNECTION_TEXT_CHARS).0);
        }
    }
    None
}

fn extract_data_model_query_name_hint(command: &str) -> Option<String> {
    let trimmed = command.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("Model") {
        return None;
    }
    Some(bounded_text(trimmed, MAX_CONNECTION_TEXT_CHARS).0)
}

fn lint_query_source(
    query_source: &str,
    missing_shared_query_definition: bool,
) -> Vec<PowerQueryLintFinding> {
    let mut findings = Vec::new();
    let trimmed_source = query_source.trim();
    if trimmed_source.is_empty() {
        findings.push(PowerQueryLintFinding {
            code: PowerQueryLintCode::EmptyQuerySource,
            message: "query source is empty after offline extraction".to_string(),
            source_line: None,
        });
        return findings;
    }

    if missing_shared_query_definition {
        findings.push(PowerQueryLintFinding {
            code: PowerQueryLintCode::MissingSharedQueryDefinition,
            message: "DataMashup source did not contain a parseable shared query definition"
                .to_string(),
            source_line: None,
        });
    }

    if let Some((_, body)) = trimmed_source.split_once('=')
        && body.trim().trim_end_matches(';').trim().is_empty()
    {
        findings.push(PowerQueryLintFinding {
            code: PowerQueryLintCode::EmptyQueryBody,
            message: "shared query definition has no expression after `=`".to_string(),
            source_line: Some(1),
        });
    }

    let masked_source = mask_string_literals(trimmed_source);
    let first_let = find_m_keyword_offset(&masked_source, "let");
    let first_in = find_m_keyword_offset(&masked_source, "in");
    if first_let.is_some() && first_in.is_none() {
        findings.push(PowerQueryLintFinding {
            code: PowerQueryLintCode::MissingInClauseForLetExpression,
            message: "let expression is missing a matching `in` clause".to_string(),
            source_line: first_let.map(|offset| source_line_number(trimmed_source, offset)),
        });
    } else if first_in.is_some() && first_let.is_none() {
        findings.push(PowerQueryLintFinding {
            code: PowerQueryLintCode::MissingLetClauseForInExpression,
            message: "`in` clause appears without a preceding `let` expression".to_string(),
            source_line: first_in.map(|offset| source_line_number(trimmed_source, offset)),
        });
    }

    if findings.len() > MAX_LINT_FINDINGS_PER_QUERY {
        findings.truncate(MAX_LINT_FINDINGS_PER_QUERY);
    }

    findings
}

fn collect_lexical_references(
    query_name: &str,
    query_source: &str,
    known_query_names: &[String],
) -> Vec<PowerQueryLexicalReference> {
    let workbook_name_re = Regex::new(
        r##"Excel\.CurrentWorkbook\s*\(\s*\)\s*\{\s*\[\s*Name\s*=\s*"((?:[^"]|"")+)"\s*\]"##,
    )
    .unwrap_or_else(|err| panic!("Power Query workbook-name matcher should compile: {err}"));
    let local_binding_re = Regex::new(r##"(?im)^\s*(?:#"([^"]+)"|([A-Za-z_][A-Za-z0-9_]*))\s*="##)
        .unwrap_or_else(|err| panic!("Power Query local-binding matcher should compile: {err}"));
    let mut references = Vec::new();
    let mut seen = BTreeSet::new();

    for captures in workbook_name_re.captures_iter(query_source) {
        let Some(whole) = captures.get(0) else {
            continue;
        };
        let Some(name_match) = captures.get(1) else {
            continue;
        };
        let target_name = name_match.as_str().replace("\"\"", "\"");
        if seen.insert(format!("workbook:{target_name}")) {
            references.push(PowerQueryLexicalReference {
                kind: PowerQueryLexicalReferenceKind::WorkbookName,
                target_name,
                evidence_kind: PowerQueryLexicalEvidenceKind::ExcelCurrentWorkbookName,
                source_line: source_line_number(query_source, whole.start()),
                source_excerpt: bounded_text(whole.as_str(), MAX_LEXICAL_SOURCE_EXCERPT_CHARS).0,
            });
        }
    }

    let local_bindings = local_binding_re
        .captures_iter(query_source)
        .filter_map(|captures| {
            captures
                .get(1)
                .or_else(|| captures.get(2))
                .map(|value| value.as_str().to_string())
        })
        .collect::<BTreeSet<_>>();
    let masked_source = mask_string_literals(query_source);

    for candidate in known_query_names {
        if candidate == query_name || local_bindings.contains(candidate) {
            continue;
        }
        if !is_bare_m_identifier(candidate) {
            let escaped_candidate = candidate.replace('"', "\"\"");
            let pattern = format!("#\"{escaped_candidate}\"");
            if let Some(offset) = masked_source.find(&pattern)
                && seen.insert(format!("query:{candidate}"))
            {
                references.push(PowerQueryLexicalReference {
                    kind: PowerQueryLexicalReferenceKind::QueryName,
                    target_name: candidate.clone(),
                    evidence_kind: PowerQueryLexicalEvidenceKind::SharedQueryIdentifier,
                    source_line: source_line_number(query_source, offset),
                    source_excerpt: bounded_text(&pattern, MAX_LEXICAL_SOURCE_EXCERPT_CHARS).0,
                });
            }
            continue;
        }

        let candidate_re = Regex::new(&format!(
            r"(^|[^A-Za-z0-9_])({})([^A-Za-z0-9_]|$)",
            regex::escape(candidate)
        ))
        .unwrap_or_else(|err| panic!("Power Query candidate matcher should compile: {err}"));
        if let Some(captures) = candidate_re.captures(&masked_source)
            && let Some(identifier) = captures.get(2)
            && seen.insert(format!("query:{candidate}"))
        {
            references.push(PowerQueryLexicalReference {
                kind: PowerQueryLexicalReferenceKind::QueryName,
                target_name: candidate.clone(),
                evidence_kind: PowerQueryLexicalEvidenceKind::SharedQueryIdentifier,
                source_line: source_line_number(query_source, identifier.start()),
                source_excerpt: bounded_text(identifier.as_str(), MAX_LEXICAL_SOURCE_EXCERPT_CHARS)
                    .0,
            });
        }
    }

    references
}

fn source_line_number(source: &str, offset: usize) -> usize {
    source[..offset.min(source.len())]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1
}

fn find_m_keyword_offset(source: &str, keyword: &str) -> Option<usize> {
    Regex::new(&format!(
        r"(?i)(^|[^A-Za-z0-9_])({})([^A-Za-z0-9_]|$)",
        regex::escape(keyword)
    ))
    .unwrap_or_else(|err| panic!("Power Query keyword matcher should compile: {err}"))
    .captures(source)
    .and_then(|captures| captures.get(2))
    .map(|keyword_match| keyword_match.start())
}

fn is_bare_m_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if first != '_' && !first.is_ascii_alphabetic() {
        return false;
    }
    chars.all(|value| value == '_' || value.is_ascii_alphanumeric())
}

fn mask_string_literals(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut masked = String::with_capacity(source.len());
    let mut index = 0usize;

    while index < bytes.len() {
        if bytes[index] == b'#' && index + 1 < bytes.len() && bytes[index + 1] == b'"' {
            masked.push('#');
            masked.push('"');
            index += 2;
            while index < bytes.len() {
                let byte = bytes[index];
                masked.push(byte as char);
                index += 1;
                if byte == b'"' {
                    if index < bytes.len() && bytes[index] == b'"' {
                        masked.push('"');
                        index += 1;
                        continue;
                    }
                    break;
                }
            }
            continue;
        }

        if bytes[index] == b'"' && (index == 0 || bytes[index - 1] != b'#') {
            masked.push(' ');
            index += 1;
            while index < bytes.len() {
                if bytes[index] == b'"' {
                    if index + 1 < bytes.len() && bytes[index + 1] == b'"' {
                        masked.push_str("  ");
                        index += 2;
                        continue;
                    }
                    masked.push(' ');
                    index += 1;
                    break;
                }
                masked.push(' ');
                index += 1;
            }
            continue;
        }

        masked.push(bytes[index] as char);
        index += 1;
    }

    masked
}

fn read_utf8_entry(
    archive: &mut ZipArchive<File>,
    entry_name: &str,
    warnings: &mut Vec<String>,
) -> Option<String> {
    let mut entry = archive.by_name(entry_name).ok()?;
    let entry_size = match usize::try_from(entry.size()) {
        Ok(size) => size,
        Err(_) => {
            push_warning(
                warnings,
                format!("workbook entry {entry_name} is too large to inspect"),
            );
            return None;
        }
    };
    if entry_size > MAX_CUSTOM_XML_ENTRY_BYTES {
        push_warning(
            warnings,
            format!("workbook entry {entry_name} exceeds {MAX_CUSTOM_XML_ENTRY_BYTES} bytes"),
        );
        return None;
    }
    let mut text = String::new();
    if let Err(err) = entry.read_to_string(&mut text) {
        push_warning(
            warnings,
            format!("failed to read workbook entry {entry_name}: {err}"),
        );
        return None;
    }
    Some(text)
}

fn read_text_entry_with_fallback(
    archive: &mut ZipArchive<File>,
    entry_name: &str,
    warnings: &mut Vec<String>,
) -> Option<String> {
    let mut entry = match archive.by_name(entry_name) {
        Ok(entry) => entry,
        Err(err) => {
            push_warning(
                warnings,
                format!("failed to open workbook entry {entry_name}: {err}"),
            );
            return None;
        }
    };
    let entry_size = match usize::try_from(entry.size()) {
        Ok(size) => size,
        Err(_) => {
            push_warning(
                warnings,
                format!("workbook entry {entry_name} is too large to inspect"),
            );
            return None;
        }
    };
    if entry_size > MAX_CUSTOM_XML_ENTRY_BYTES {
        push_warning(
            warnings,
            format!("workbook entry {entry_name} exceeds {MAX_CUSTOM_XML_ENTRY_BYTES} bytes"),
        );
        return None;
    }
    let mut bytes = Vec::with_capacity(entry_size);
    if let Err(err) = entry.read_to_end(&mut bytes) {
        push_warning(
            warnings,
            format!("failed to read workbook entry {entry_name}: {err}"),
        );
        return None;
    }
    decode_text_bytes(&bytes)
        .map_err(|err| push_warning(warnings, err))
        .ok()
}

fn decode_text_bytes(bytes: &[u8]) -> Result<String, String> {
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return String::from_utf16(
            &bytes[2..]
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .map_err(|err| format!("failed to decode UTF-16LE workbook XML: {err}"));
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return String::from_utf16(
            &bytes[2..]
                .chunks_exact(2)
                .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .map_err(|err| format!("failed to decode UTF-16BE workbook XML: {err}"));
    }
    if bytes.len() > 3 && bytes[1] == 0x00 && bytes[3] == 0x00 {
        return String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .map_err(|err| format!("failed to decode UTF-16LE workbook XML: {err}"));
    }
    String::from_utf8(bytes.to_vec()).map_err(|err| format!("failed to decode workbook XML: {err}"))
}

fn extract_data_mashup_payload(xml: &str) -> Option<&str> {
    let lower = xml.to_ascii_lowercase();
    let start_tag = lower.find("<datamashup")?;
    let content_start = xml[start_tag..]
        .find('>')
        .map(|index| start_tag + index + 1)?;
    let end_tag = lower.rfind("</datamashup>")?;
    Some(xml[content_start..end_tag].trim())
}

fn find_embedded_zip_bounds(bytes: &[u8]) -> Option<(usize, usize)> {
    let start = bytes
        .windows(4)
        .position(|window| window == b"PK\x03\x04")?;
    let mut end = None;

    for index in (start..bytes.len().saturating_sub(21)).rev() {
        if bytes[index..].starts_with(b"PK\x05\x06") {
            let comment_length = u16::from_le_bytes([bytes[index + 20], bytes[index + 21]]);
            let candidate_end = index + 22 + usize::from(comment_length);
            if candidate_end <= bytes.len() {
                end = Some(candidate_end);
                break;
            }
        }
    }

    end.map(|value| (start, value))
}

fn extract_connection_location(connection: &str) -> Option<String> {
    for segment in connection.split(';') {
        let (key, value) = segment.split_once('=')?;
        if key.trim().eq_ignore_ascii_case("Location") {
            return Some(value.trim().trim_matches('"').to_string());
        }
    }
    None
}

fn tag_matches(name: &[u8], local_name: &[u8]) -> bool {
    name == local_name || name.rsplit(|byte| *byte == b':').next() == Some(local_name)
}

fn read_attr(
    reader: &Reader<&[u8]>,
    event: &quick_xml::events::BytesStart<'_>,
    key: &[u8],
) -> Option<String> {
    event
        .attributes()
        .with_checks(false)
        .flatten()
        .find(|attr| attr.key.as_ref() == key)
        .and_then(|attr| attr.decode_and_unescape_value(reader.decoder()).ok())
        .map(std::borrow::Cow::into_owned)
}

fn read_attr_local(
    reader: &Reader<&[u8]>,
    event: &quick_xml::events::BytesStart<'_>,
    local_key: &[u8],
) -> Option<String> {
    event
        .attributes()
        .with_checks(false)
        .flatten()
        .find(|attr| tag_matches(attr.key.as_ref(), local_key))
        .and_then(|attr| attr.decode_and_unescape_value(reader.decoder()).ok())
        .map(std::borrow::Cow::into_owned)
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

fn resolve_workbook_path_from_model_arg(
    path: &str,
    cwd: &Path,
) -> Result<PathBuf, FunctionCallError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "excel.extract_powerquery_queries path must not be empty".to_string(),
        ));
    }
    if trimmed.contains('\0') || trimmed.contains("://") {
        return Err(FunctionCallError::RespondToModel(
            "excel.extract_powerquery_queries path must be a local workbook path".to_string(),
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
            "excel.extract_powerquery_queries path must be relative and stay within the current working directory"
                .to_string(),
        ));
    }

    let Some(extension) = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
    else {
        return Err(FunctionCallError::RespondToModel(
            "excel.extract_powerquery_queries path must end in .xlsx, .xlsm, or .xlsb".to_string(),
        ));
    };
    if !matches!(extension.as_str(), "xlsx" | "xlsm" | "xlsb") {
        return Err(FunctionCallError::RespondToModel(
            "excel.extract_powerquery_queries path must end in .xlsx, .xlsm, or .xlsb".to_string(),
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
                "excel.extract_powerquery_queries path must not traverse symlinks".to_string(),
            ));
        }
    }

    Ok(resolved_path)
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

fn push_warning(warnings: &mut Vec<String>, warning: String) {
    if warnings.len() < MAX_WARNINGS {
        warnings.push(warning);
    }
}
