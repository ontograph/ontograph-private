use std::collections::BTreeMap;
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

const EXTRACT_POWERQUERY_QUERIES_DESCRIPTION: &str = "Read Power Query M definitions from a workbook and return bounded read-only query text and metadata.";
const MAX_CUSTOM_XML_ENTRY_BYTES: usize = 8 * 1024 * 1024;
const MAX_CUSTOM_XML_SCAN_ENTRIES: usize = 64;
const MAX_CUSTOM_XML_SCAN_BYTES: usize = 8 * 1024 * 1024;
const MAX_QUERY_COUNT: usize = 16;
const MAX_QUERY_SOURCE_CHARS: usize = 4096;
const MAX_PART_NAME_CHARS: usize = 256;
const MAX_CONNECTION_TEXT_CHARS: usize = 512;
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
pub(crate) struct ExtractedPowerQueryQuery {
    pub name: String,
    pub source_part: String,
    pub source: String,
    pub source_truncated: bool,
    pub connection_name: Option<String>,
    pub location: Option<String>,
    pub command_preview: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct ExtractPowerQueryQueriesResult {
    pub mode: String,
    pub path: String,
    pub has_power_query: bool,
    pub query_count: usize,
    pub queries: Vec<ExtractedPowerQueryQuery>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct PowerQueryConnection {
    connection_name: Option<String>,
    location: Option<String>,
    command_preview: Option<String>,
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
    let (queries, saw_power_query_marker) =
        read_data_mashup_queries(&mut archive, &connections, &mut warnings);
    let has_power_query = saw_power_query_marker || !queries.is_empty();

    if !has_power_query && warnings.is_empty() {
        push_warning(
            &mut warnings,
            "no Power Query DataMashup payload was found in the workbook".to_string(),
        );
    }

    ExtractPowerQueryQueriesResult {
        mode: "read_only_extraction".to_string(),
        path: display_path.display().to_string(),
        has_power_query,
        query_count: queries.len(),
        queries,
        warnings,
    }
}

fn empty_result(path: &Path, warnings: Vec<String>) -> ExtractPowerQueryQueriesResult {
    ExtractPowerQueryQueriesResult {
        mode: "read_only_extraction".to_string(),
        path: path.display().to_string(),
        has_power_query: false,
        query_count: 0,
        queries: Vec::new(),
        warnings,
    }
}

fn read_connections(
    archive: &mut ZipArchive<File>,
    warnings: &mut Vec<String>,
) -> BTreeMap<String, PowerQueryConnection> {
    let Some(xml) = read_utf8_entry(archive, "xl/connections.xml", warnings) else {
        return BTreeMap::new();
    };
    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut current = PowerQueryConnection::default();
    let mut current_key = None::<String>;
    let mut mapping = BTreeMap::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) if event.name().as_ref() == b"connection" => {
                current = PowerQueryConnection {
                    connection_name: read_attr(&reader, &event, b"name"),
                    ..PowerQueryConnection::default()
                };
                current_key = None;
            }
            Ok(Event::Empty(event)) if event.name().as_ref() == b"dbPr" => {
                let connection = read_attr(&reader, &event, b"connection");
                let command = read_attr(&reader, &event, b"command");
                let location = connection
                    .as_deref()
                    .and_then(extract_connection_location)
                    .map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
                if let Some(location_name) = location.clone() {
                    current.location = Some(location_name.clone());
                    current.command_preview =
                        command.map(|value| bounded_text(&value, MAX_CONNECTION_TEXT_CHARS).0);
                    current_key = Some(location_name);
                }
            }
            Ok(Event::End(event)) if event.name().as_ref() == b"connection" => {
                if let Some(key) = current_key.take() {
                    mapping.insert(key, current.clone());
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

    mapping
}

fn read_data_mashup_queries(
    archive: &mut ZipArchive<File>,
    connections: &BTreeMap<String, PowerQueryConnection>,
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

        for query in read_queries_from_embedded_archive(&mut embedded, connections, warnings) {
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
    connections: &BTreeMap<String, PowerQueryConnection>,
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
            warnings,
        ));
    }

    queries
}

fn split_shared_queries(
    entry_name: &str,
    source: &str,
    connections: &BTreeMap<String, PowerQueryConnection>,
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
        let (bounded_source, source_truncated) = bounded_text(source, MAX_QUERY_SOURCE_CHARS);
        return vec![build_query_result(
            "Section1".to_string(),
            entry_name,
            bounded_source,
            source_truncated,
            connections.get("Section1"),
        )];
    }

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
        let (bounded_source, source_truncated) = bounded_text(query_source, MAX_QUERY_SOURCE_CHARS);
        queries.push(build_query_result(
            name.clone(),
            entry_name,
            bounded_source,
            source_truncated,
            connections.get(name),
        ));
    }

    queries
}

fn build_query_result(
    name: String,
    entry_name: &str,
    source: String,
    source_truncated: bool,
    connection: Option<&PowerQueryConnection>,
) -> ExtractedPowerQueryQuery {
    ExtractedPowerQueryQuery {
        name,
        source_part: bounded_text(entry_name, MAX_PART_NAME_CHARS).0,
        source,
        source_truncated,
        connection_name: connection.and_then(|value| value.connection_name.clone()),
        location: connection.and_then(|value| value.location.clone()),
        command_preview: connection.and_then(|value| value.command_preview.clone()),
    }
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
