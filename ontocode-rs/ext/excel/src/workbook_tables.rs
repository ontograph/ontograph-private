use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
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
use quick_xml::Reader;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_value;
use zip::ZipArchive;

use crate::backend::ExcelInspectionError;
use crate::backend::inspect_workbook_with_display_path;
use crate::preview::bounded_text;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::SheetKind;
use crate::tool::SheetSummary;
use crate::vba_extract::parse_tool_args;
use crate::vba_extract::resolve_workbook_path_from_model_arg;

pub(crate) const INSPECT_WORKBOOK_TABLES_TOOL_NAME: &str = "inspect_workbook_tables";

const INSPECT_WORKBOOK_TABLES_DESCRIPTION: &str = "Inspect bounded offline workbook table metadata, including table counts, ranges, and column-name samples.";
const MAX_TABLE_XML_ENTRY_BYTES: usize = 256 * 1024;
const MAX_TABLE_XML_SCAN_ENTRIES: usize = 64;
const MAX_TABLE_COUNT: usize = 64;
const MAX_TABLE_NAME_CHARS: usize = 128;
const MAX_PART_PATH_CHARS: usize = 256;
const MAX_RANGE_REFERENCE_CHARS: usize = 64;
const MAX_COLUMN_NAME_CHARS: usize = 128;
const MAX_COLUMN_NAME_SAMPLE: usize = 8;
const MAX_WARNINGS: usize = 32;

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectWorkbookTablesTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectWorkbookTablesArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct WorkbookTableSummary {
    pub name: String,
    pub alt_name: Option<String>,
    pub sheet_name: String,
    pub part_path: String,
    pub range_reference: String,
    pub has_header_row: Option<bool>,
    pub has_totals_row: Option<bool>,
    pub column_names_sample: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectWorkbookTablesResult {
    pub mode: String,
    pub path: String,
    pub table_count: usize,
    pub tables: Vec<WorkbookTableSummary>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedWorkbookTable {
    pub(crate) name: String,
    pub(crate) alt_name: Option<String>,
    pub(crate) sheet_name: String,
    pub(crate) worksheet_index: usize,
    pub(crate) worksheet_part_path: String,
    pub(crate) part_path: String,
    pub(crate) relationship_part_path: String,
    pub(crate) relationship_id: String,
    pub(crate) range_reference: String,
    pub(crate) start_reference: String,
    pub(crate) end_reference: String,
    pub(crate) has_header_row: Option<bool>,
    pub(crate) has_totals_row: Option<bool>,
    pub(crate) column_names_sample: Vec<String>,
}

pub(crate) struct WorkbookTableParseResult {
    pub(crate) tables: Vec<ParsedWorkbookTable>,
    pub(crate) had_unresolved_table_metadata: bool,
    pub(crate) warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct ParsedTableXml {
    name: String,
    alt_name: Option<String>,
    range_reference: String,
    has_header_row: Option<bool>,
    has_totals_row: Option<bool>,
    column_names_sample: Vec<String>,
    column_names_truncated: bool,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectWorkbookTablesTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_TABLES_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema = serde_json::to_value(schemars::schema_for!(InspectWorkbookTablesArgs))
            .unwrap_or_else(|err| {
                panic!("inspect_workbook_tables args schema should serialize: {err}")
            });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(InspectWorkbookTablesResult))
                .unwrap_or_else(|err| {
                    panic!("inspect_workbook_tables result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_WORKBOOK_TABLES_TOOL_NAME.to_string(),
                    description: INSPECT_WORKBOOK_TABLES_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_workbook_tables args schema should parse: {err}")
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
        let args =
            parse_tool_args::<InspectWorkbookTablesArgs>(&call, "excel.inspect_workbook_tables")?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_workbook_tables workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = resolve_workbook_path_from_model_arg(
            "excel.inspect_workbook_tables",
            &args.path,
            &cwd,
        )?;
        let result =
            inspect_workbook_tables_from_workbook(&workbook_path, Path::new(args.path.trim()));
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize workbook table metadata: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectWorkbookTablesTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

impl From<&ParsedWorkbookTable> for WorkbookTableSummary {
    fn from(value: &ParsedWorkbookTable) -> Self {
        Self {
            name: value.name.clone(),
            alt_name: value.alt_name.clone(),
            sheet_name: value.sheet_name.clone(),
            part_path: value.part_path.clone(),
            range_reference: value.range_reference.clone(),
            has_header_row: value.has_header_row,
            has_totals_row: value.has_totals_row,
            column_names_sample: value.column_names_sample.clone(),
        }
    }
}

pub(crate) fn inspect_workbook_tables_from_workbook(
    path: &Path,
    display_path: &Path,
) -> InspectWorkbookTablesResult {
    let workbook = match inspect_workbook_with_display_path(path, display_path) {
        Ok(workbook) => workbook,
        Err(err) => {
            return InspectWorkbookTablesResult {
                mode: "read_only_inspection".to_string(),
                path: display_path.display().to_string(),
                table_count: 0,
                tables: Vec::new(),
                warnings: vec![err.to_string()],
            };
        }
    };
    let worksheet_summaries = workbook
        .sheets
        .iter()
        .enumerate()
        .filter(|(_, summary)| summary.kind == SheetKind::Worksheet)
        .collect::<Vec<_>>();

    let parse = match parse_workbook_tables(path, &worksheet_summaries) {
        Ok(parse) => parse,
        Err(err) => {
            return InspectWorkbookTablesResult {
                mode: "read_only_inspection".to_string(),
                path: display_path.display().to_string(),
                table_count: 0,
                tables: Vec::new(),
                warnings: vec![err.to_string()],
            };
        }
    };

    let mut warnings = workbook.warnings.clone();
    for warning in parse.warnings {
        push_warning(&mut warnings, warning);
    }
    if workbook.markers.has_tables && parse.tables.is_empty() {
        push_warning(
            &mut warnings,
            "workbook markers report tables but no bounded table metadata could be resolved"
                .to_string(),
        );
    }
    if parse.had_unresolved_table_metadata {
        push_warning(
            &mut warnings,
            "workbook contains unresolved table metadata; only tables with proven worksheet ownership and range metadata are reported".to_string(),
        );
    }

    InspectWorkbookTablesResult {
        mode: "read_only_inspection".to_string(),
        path: display_path.display().to_string(),
        table_count: parse.tables.len(),
        tables: parse
            .tables
            .iter()
            .map(WorkbookTableSummary::from)
            .collect(),
        warnings,
    }
}

pub(crate) fn parse_workbook_tables(
    workbook_path: &Path,
    worksheet_summaries: &[(usize, &SheetSummary)],
) -> Result<WorkbookTableParseResult, ExcelInspectionError> {
    let file = File::open(workbook_path).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to open workbook {} for table metadata parsing: {err}",
            workbook_path.display()
        ))
    })?;
    let mut archive = ZipArchive::new(file).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to read workbook archive {} for table metadata parsing: {err}",
            workbook_path.display()
        ))
    })?;
    let mut tables = Vec::new();
    let mut warnings = Vec::new();
    let mut had_unresolved_table_metadata = false;
    let mut remaining_entries = MAX_TABLE_XML_SCAN_ENTRIES;
    let mut table_count_truncated = false;

    for (worksheet_index, summary) in worksheet_summaries {
        let Some(worksheet_part_path) = summary.part_path.as_deref() else {
            continue;
        };
        let relationship_part_path = worksheet_relationship_part_path(worksheet_part_path);
        let Some(relationship_xml) = read_table_entry(
            &mut archive,
            &relationship_part_path,
            &mut remaining_entries,
        )?
        else {
            continue;
        };
        let table_relationships = parse_table_relationships(&relationship_xml)?;
        for (relationship_id, target) in table_relationships {
            if tables.len() >= MAX_TABLE_COUNT {
                table_count_truncated = true;
                break;
            }
            let table_part_path = resolve_relationship_target(worksheet_part_path, &target);
            let Some(table_xml) =
                read_table_entry(&mut archive, &table_part_path, &mut remaining_entries)?
            else {
                had_unresolved_table_metadata = true;
                continue;
            };
            let Some(table_info) = parse_table_xml(&table_xml)? else {
                had_unresolved_table_metadata = true;
                continue;
            };
            let range_reference = table_info.range_reference.clone();
            let Some((start_reference, end_reference)) =
                split_table_range_reference(&range_reference)
            else {
                had_unresolved_table_metadata = true;
                continue;
            };
            let start_reference = start_reference.to_string();
            let end_reference = end_reference.to_string();
            let Some(sheet_name) = summary.name.clone() else {
                had_unresolved_table_metadata = true;
                continue;
            };
            if table_info.column_names_truncated {
                push_warning(
                    &mut warnings,
                    format!(
                        "column-name sample for table `{sheet_name}!{}` truncated to {MAX_COLUMN_NAME_SAMPLE} entries",
                        table_info.name
                    ),
                );
            }
            tables.push(ParsedWorkbookTable {
                name: table_info.name,
                alt_name: table_info.alt_name,
                sheet_name,
                worksheet_index: *worksheet_index,
                worksheet_part_path: worksheet_part_path.to_string(),
                part_path: table_part_path,
                relationship_part_path: relationship_part_path.clone(),
                relationship_id,
                range_reference,
                start_reference,
                end_reference,
                has_header_row: table_info.has_header_row,
                has_totals_row: table_info.has_totals_row,
                column_names_sample: table_info.column_names_sample,
            });
        }
        if table_count_truncated {
            break;
        }
    }

    if table_count_truncated {
        push_warning(
            &mut warnings,
            format!("workbook table inventory truncated to {MAX_TABLE_COUNT} entries"),
        );
    }

    Ok(WorkbookTableParseResult {
        tables,
        had_unresolved_table_metadata,
        warnings,
    })
}

fn read_table_entry(
    archive: &mut ZipArchive<File>,
    name: &str,
    remaining_entries: &mut usize,
) -> Result<Option<String>, ExcelInspectionError> {
    let Ok(mut entry) = archive.by_name(name) else {
        return Ok(None);
    };
    if *remaining_entries == 0 {
        return Err(ExcelInspectionError::Message(
            "workbook table parsing exceeded XML entry budget".to_string(),
        ));
    }
    *remaining_entries -= 1;
    let entry_size = usize::try_from(entry.size()).map_err(|_| {
        ExcelInspectionError::Message(format!("workbook table entry {name} is too large"))
    })?;
    if entry_size > MAX_TABLE_XML_ENTRY_BYTES {
        return Err(ExcelInspectionError::Message(format!(
            "workbook table entry {name} exceeds {MAX_TABLE_XML_ENTRY_BYTES} bytes"
        )));
    }
    let mut bytes = Vec::with_capacity(entry_size.saturating_add(1));
    entry.read_to_end(&mut bytes).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to read workbook table entry {name}: {err}"))
    })?;
    Ok(Some(decode_table_xml_text_bytes(&bytes)?))
}

fn parse_table_relationships(
    relationship_xml: &str,
) -> Result<BTreeMap<String, String>, ExcelInspectionError> {
    let mut reader = Reader::from_str(relationship_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut relationships = BTreeMap::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"Relationship" =>
            {
                let relationship_type = table_attr_value(&event, b"Type")?.unwrap_or_default();
                if !relationship_type.ends_with("/table") {
                    buf.clear();
                    continue;
                }
                let id = table_attr_value(&event, b"Id")?.unwrap_or_default();
                let target = table_attr_value(&event, b"Target")?.unwrap_or_default();
                if !id.is_empty() && !target.is_empty() {
                    relationships.insert(id, target);
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse worksheet table relationships: {err}"
                )));
            }
        }
        buf.clear();
    }
    Ok(relationships)
}

fn parse_table_xml(table_xml: &str) -> Result<Option<ParsedTableXml>, ExcelInspectionError> {
    let mut reader = Reader::from_str(table_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut name = None::<String>;
    let mut alt_name = None::<String>;
    let mut range_reference = None::<String>;
    let mut has_header_row = None::<bool>;
    let mut has_totals_row = None::<bool>;
    let mut column_names_sample = Vec::new();
    let mut column_names_truncated = false;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"table" =>
            {
                let display_name = table_attr_value(&event, b"displayName")?
                    .or_else(|| table_attr_value(&event, b"name").ok().flatten());
                name = display_name.map(|value| bounded_text(&value, MAX_TABLE_NAME_CHARS));
                alt_name = table_attr_value(&event, b"name")?
                    .map(|value| bounded_text(&value, MAX_TABLE_NAME_CHARS));
                range_reference = table_attr_value(&event, b"ref")?
                    .map(|value| bounded_text(&value, MAX_RANGE_REFERENCE_CHARS));
                has_header_row = table_attr_value(&event, b"headerRowCount")?
                    .and_then(|value| value.parse::<u32>().ok().map(|count| count > 0));
                has_totals_row = table_attr_value(&event, b"totalsRowShown")?
                    .and_then(|value| parse_excel_bool(&value))
                    .or_else(|| {
                        table_attr_value(&event, b"totalsRowCount")
                            .ok()
                            .flatten()
                            .and_then(|value| value.parse::<u32>().ok().map(|count| count > 0))
                    });
            }
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"tableColumn" =>
            {
                if let Some(column_name) = table_attr_value(&event, b"name")? {
                    if column_names_sample.len() < MAX_COLUMN_NAME_SAMPLE {
                        column_names_sample.push(bounded_text(&column_name, MAX_COLUMN_NAME_CHARS));
                    } else {
                        column_names_truncated = true;
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse table XML: {err}"
                )));
            }
        }
        buf.clear();
    }

    let Some(name) = name else {
        return Ok(None);
    };
    let Some(range_reference) = range_reference else {
        return Ok(None);
    };

    Ok(Some(ParsedTableXml {
        name,
        alt_name,
        range_reference,
        has_header_row,
        has_totals_row,
        column_names_sample,
        column_names_truncated,
    }))
}

fn table_attr_value(
    event: &BytesStart<'_>,
    key: &[u8],
) -> Result<Option<String>, ExcelInspectionError> {
    for attr in event.attributes().with_checks(false) {
        let attr = attr.map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to parse workbook table attribute: {err}"
            ))
        })?;
        if attr.key.as_ref() == key {
            return Ok(Some(
                String::from_utf8_lossy(attr.value.as_ref()).into_owned(),
            ));
        }
    }
    Ok(None)
}

fn decode_table_xml_text_bytes(bytes: &[u8]) -> Result<String, ExcelInspectionError> {
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return String::from_utf16(
            &bytes[2..]
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .map_err(|err| {
            ExcelInspectionError::Message(format!("failed to decode UTF-16LE workbook XML: {err}"))
        });
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return String::from_utf16(
            &bytes[2..]
                .chunks_exact(2)
                .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .map_err(|err| {
            ExcelInspectionError::Message(format!("failed to decode UTF-16BE workbook XML: {err}"))
        });
    }
    if bytes.len() > 3 && bytes[1] == 0x00 && bytes[3] == 0x00 {
        return String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .map_err(|err| {
            ExcelInspectionError::Message(format!("failed to decode UTF-16LE workbook XML: {err}"))
        });
    }
    String::from_utf8(bytes.to_vec()).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to decode workbook XML: {err}"))
    })
}

fn worksheet_relationship_part_path(worksheet_part_path: &str) -> String {
    let path = Path::new(worksheet_part_path);
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    normalize_part_path(&format!("{}/_rels/{}.rels", parent.display(), file_name))
}

fn resolve_relationship_target(source_part_path: &str, target: &str) -> String {
    let parent = Path::new(source_part_path)
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let joined = if target.starts_with('/') {
        PathBuf::from(target.trim_start_matches('/'))
    } else {
        parent.join(target)
    };
    normalize_part_path(&joined.display().to_string())
}

fn normalize_part_path(path: &str) -> String {
    let mut parts = Vec::new();
    for component in Path::new(path).components() {
        match component {
            Component::Normal(segment) => parts.push(segment.to_string_lossy().into_owned()),
            Component::ParentDir => {
                let _ = parts.pop();
            }
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {}
        }
    }
    bounded_text(&parts.join("/"), MAX_PART_PATH_CHARS)
}

fn split_table_range_reference(text: &str) -> Option<(&str, &str)> {
    text.split_once(':')
}

fn parse_excel_bool(value: &str) -> Option<bool> {
    match value.trim() {
        "1" | "true" | "TRUE" => Some(true),
        "0" | "false" | "FALSE" => Some(false),
        _ => None,
    }
}

fn push_warning(warnings: &mut Vec<String>, warning: String) {
    if warnings.len() < MAX_WARNINGS && !warnings.iter().any(|existing| existing == &warning) {
        warnings.push(warning);
    }
}
