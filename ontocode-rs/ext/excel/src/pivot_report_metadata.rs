use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
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
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::workbook_path_from_model_arg;

pub(crate) const INSPECT_PIVOT_REPORT_METADATA_TOOL_NAME: &str = "inspect_pivot_report_metadata";

const INSPECT_PIVOT_REPORT_METADATA_DESCRIPTION: &str = "Inspect bounded offline PivotTable and pivot cache metadata from a .xlsx or .xlsm workbook package.";
const MODE: &str = "openxml_package";
const MAX_XML_ENTRY_BYTES: usize = 1024 * 1024;
const MAX_PIVOT_TABLES: usize = 64;
const MAX_PIVOT_CACHES: usize = 64;
const MAX_FIELDS_PER_ROLE_SAMPLE: usize = 32;
const MAX_CACHE_FIELDS_SAMPLE: usize = 64;
const MAX_STORED_MDX_PREVIEW_CHARS: usize = 4096;
const MAX_WARNINGS: usize = 32;

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectPivotReportMetadataTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectPivotReportMetadataArgs {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectPivotReportMetadataResult {
    pub mode: String,
    pub path: String,
    pub pivot_table_count: usize,
    pub pivot_cache_count: usize,
    pub pivot_tables: Vec<PivotTableReportSummary>,
    pub pivot_caches: Vec<PivotCacheReportSummary>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct PivotTableReportSummary {
    pub name: Option<String>,
    pub worksheet_name: Option<String>,
    pub part_path: Option<String>,
    pub range_ref: Option<String>,
    pub cache_id: Option<String>,
    pub source_type: Option<String>,
    pub source_name: Option<String>,
    pub source_range: Option<String>,
    pub connection_id: Option<String>,
    pub connection_name: Option<String>,
    pub connection_type: Option<String>,
    pub olap: bool,
    pub data_model: bool,
    pub stored_mdx_preview: Option<String>,
    pub stored_mdx_truncated: bool,
    pub row_fields_sample: Vec<String>,
    pub column_fields_sample: Vec<String>,
    pub data_fields_sample: Vec<String>,
    pub page_fields_sample: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct PivotCacheReportSummary {
    pub cache_id: Option<String>,
    pub source_type: Option<String>,
    pub source_name: Option<String>,
    pub source_range: Option<String>,
    pub connection_id: Option<String>,
    pub connection_name: Option<String>,
    pub connection_type: Option<String>,
    pub olap: bool,
    pub data_model: bool,
    pub cache_fields_sample: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct WorkbookSheetLocation {
    name: String,
    part_path: String,
}

#[derive(Debug, Clone, Default)]
struct ConnectionInfo {
    name: Option<String>,
    connection_type: Option<String>,
    command: Option<String>,
    olap: bool,
    data_model: bool,
}

#[derive(Debug, Clone)]
struct CacheInfo {
    public: PivotCacheReportSummary,
    field_lookup: Vec<String>,
    stored_mdx_preview: Option<String>,
    stored_mdx_truncated: bool,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectPivotReportMetadataTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_PIVOT_REPORT_METADATA_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(InspectPivotReportMetadataArgs))
                .unwrap_or_else(|err| {
                    panic!("inspect_pivot_report_metadata args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(InspectPivotReportMetadataResult))
                .unwrap_or_else(|err| {
                    panic!("inspect_pivot_report_metadata result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_PIVOT_REPORT_METADATA_TOOL_NAME.to_string(),
                    description: INSPECT_PIVOT_REPORT_METADATA_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_pivot_report_metadata args schema should parse: {err}")
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
        let args = parse_tool_args::<InspectPivotReportMetadataArgs>(
            &call,
            "excel.inspect_pivot_report_metadata",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_pivot_report_metadata workspace context is unavailable for this turn"
                    .to_string(),
            )
        })?;
        let workbook_path = workbook_path_from_model_arg(&args.path, &cwd)?;
        let result = inspect_pivot_report_metadata_from_workbook(
            &workbook_path,
            Path::new(args.path.trim()),
        )
        .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize pivot report metadata: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectPivotReportMetadataTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn inspect_pivot_report_metadata_from_workbook(
    path: &Path,
    display_path: &Path,
) -> Result<InspectPivotReportMetadataResult, ExcelInspectionError> {
    let file = File::open(path).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to open workbook {}: {err}", path.display()))
    })?;
    let mut archive = ZipArchive::new(file).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to read workbook archive {}: {err}",
            path.display()
        ))
    })?;

    let workbook_xml = read_optional_entry(&mut archive, "xl/workbook.xml")?;
    let workbook_rels = read_optional_entry(&mut archive, "xl/_rels/workbook.xml.rels")?;
    let mut warnings = Vec::new();

    let (Some(workbook_xml), Some(workbook_rels)) = (workbook_xml, workbook_rels) else {
        push_warning(
            &mut warnings,
            "pivot report metadata currently supports OpenXML .xlsx/.xlsm workbook XML only",
        );
        return Ok(InspectPivotReportMetadataResult {
            mode: MODE.to_string(),
            path: display_path.display().to_string(),
            pivot_table_count: 0,
            pivot_cache_count: 0,
            pivot_tables: Vec::new(),
            pivot_caches: Vec::new(),
            warnings,
        });
    };

    let workbook_relationships = parse_relationship_targets(&workbook_rels)?;
    let sheets = parse_workbook_sheets(&workbook_xml, &workbook_relationships)?;
    let workbook_pivot_caches = parse_workbook_pivot_caches(&workbook_xml)?;
    let connections =
        if let Some(connections_xml) = read_optional_entry(&mut archive, "xl/connections.xml")? {
            parse_connections(&connections_xml)?
        } else {
            BTreeMap::new()
        };

    let mut caches = BTreeMap::new();
    for (cache_id, rel_id) in workbook_pivot_caches {
        let Some(definition_target) = workbook_relationships.get(&rel_id) else {
            push_warning(
                &mut warnings,
                format!("pivot cache {cache_id} is missing workbook relationship {rel_id}"),
            );
            continue;
        };
        let definition_path = normalize_part_path("xl", definition_target);
        let Some(definition_xml) = read_optional_entry(&mut archive, &definition_path)? else {
            push_warning(
                &mut warnings,
                format!("pivot cache {cache_id} definition {definition_path} is missing"),
            );
            continue;
        };
        let cache_info = parse_cache_definition(&definition_xml, &cache_id, &connections)?;
        caches.insert(cache_id, cache_info);
    }

    let mut pivot_tables = Vec::new();
    let mut total_pivot_tables = 0usize;
    for sheet in &sheets {
        let rel_path = relationships_path(&sheet.part_path);
        let Some(sheet_rels_xml) = read_optional_entry(&mut archive, &rel_path)? else {
            continue;
        };
        let base_dir = directory_of(&sheet.part_path);
        for target in parse_pivot_table_targets(&sheet_rels_xml)? {
            total_pivot_tables += 1;
            if pivot_tables.len() >= MAX_PIVOT_TABLES {
                continue;
            }
            let part_path = normalize_part_path(&base_dir, &target);
            let Some(pivot_table_xml) = read_optional_entry(&mut archive, &part_path)? else {
                push_warning(
                    &mut warnings,
                    format!(
                        "pivot table relationship from {} points to missing part {}",
                        sheet.part_path, part_path
                    ),
                );
                continue;
            };
            let summary = parse_pivot_table(&pivot_table_xml, &sheet.name, &part_path, &caches)
                .unwrap_or_else(|err| PivotTableReportSummary {
                    name: None,
                    worksheet_name: Some(sheet.name.clone()),
                    part_path: Some(part_path.clone()),
                    range_ref: None,
                    cache_id: None,
                    source_type: None,
                    source_name: None,
                    source_range: None,
                    connection_id: None,
                    connection_name: None,
                    connection_type: None,
                    olap: false,
                    data_model: false,
                    stored_mdx_preview: None,
                    stored_mdx_truncated: false,
                    row_fields_sample: Vec::new(),
                    column_fields_sample: Vec::new(),
                    data_fields_sample: Vec::new(),
                    page_fields_sample: Vec::new(),
                    warnings: vec![err.to_string()],
                });
            pivot_tables.push(summary);
        }
    }
    if total_pivot_tables > MAX_PIVOT_TABLES {
        push_warning(
            &mut warnings,
            format!(
                "pivot table output truncated to {MAX_PIVOT_TABLES} of {total_pivot_tables} entries"
            ),
        );
    }

    let total_pivot_caches = caches.len();
    let mut pivot_caches = caches
        .into_values()
        .map(|cache| cache.public)
        .collect::<Vec<_>>();
    if pivot_caches.len() > MAX_PIVOT_CACHES {
        pivot_caches.truncate(MAX_PIVOT_CACHES);
        push_warning(
            &mut warnings,
            format!(
                "pivot cache output truncated to {MAX_PIVOT_CACHES} of {total_pivot_caches} entries"
            ),
        );
    }

    Ok(InspectPivotReportMetadataResult {
        mode: MODE.to_string(),
        path: display_path.display().to_string(),
        pivot_table_count: total_pivot_tables,
        pivot_cache_count: total_pivot_caches,
        pivot_tables,
        pivot_caches,
        warnings,
    })
}

fn read_optional_entry(
    archive: &mut ZipArchive<File>,
    entry_name: &str,
) -> Result<Option<String>, ExcelInspectionError> {
    let mut entry = match archive.by_name(entry_name) {
        Ok(entry) => entry,
        Err(zip::result::ZipError::FileNotFound) => return Ok(None),
        Err(err) => {
            return Err(ExcelInspectionError::Message(format!(
                "failed to read workbook entry {entry_name}: {err}"
            )));
        }
    };
    let entry_size = usize::try_from(entry.size()).map_err(|_| {
        ExcelInspectionError::Message(format!(
            "workbook entry {entry_name} is too large to inspect"
        ))
    })?;
    if entry_size > MAX_XML_ENTRY_BYTES {
        return Err(ExcelInspectionError::Message(format!(
            "workbook entry {entry_name} exceeds {MAX_XML_ENTRY_BYTES} bytes"
        )));
    }
    let mut contents = String::new();
    entry.read_to_string(&mut contents).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to decode workbook entry {entry_name}: {err}"
        ))
    })?;
    Ok(Some(contents))
}

fn parse_workbook_sheets(
    workbook_xml: &str,
    workbook_relationships: &BTreeMap<String, String>,
) -> Result<Vec<WorkbookSheetLocation>, ExcelInspectionError> {
    let mut reader = Reader::from_str(workbook_xml);
    reader.config_mut().trim_text(true);
    let mut sheets = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"sheet") =>
            {
                let Some(name) = read_attr(&reader, &event, b"name") else {
                    continue;
                };
                let Some(rel_id) = read_attr(&reader, &event, b"r:id")
                    .or_else(|| read_attr_local(&reader, &event, b"id"))
                else {
                    continue;
                };
                let Some(target) = workbook_relationships.get(&rel_id) else {
                    continue;
                };
                sheets.push(WorkbookSheetLocation {
                    name,
                    part_path: normalize_part_path("xl", target),
                });
            }
            Ok(Event::Eof) => break,
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse workbook sheet metadata: {err}"
                )));
            }
            _ => {}
        }
    }
    Ok(sheets)
}

fn parse_workbook_pivot_caches(
    workbook_xml: &str,
) -> Result<Vec<(String, String)>, ExcelInspectionError> {
    let mut reader = Reader::from_str(workbook_xml);
    reader.config_mut().trim_text(true);
    let mut pivot_caches = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"pivotCache") =>
            {
                let Some(cache_id) = read_attr(&reader, &event, b"cacheId") else {
                    continue;
                };
                let Some(rel_id) = read_attr(&reader, &event, b"r:id")
                    .or_else(|| read_attr_local(&reader, &event, b"id"))
                else {
                    continue;
                };
                pivot_caches.push((cache_id, rel_id));
            }
            Ok(Event::Eof) => break,
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse workbook pivot caches: {err}"
                )));
            }
            _ => {}
        }
    }
    Ok(pivot_caches)
}

fn parse_relationship_targets(
    relationships_xml: &str,
) -> Result<BTreeMap<String, String>, ExcelInspectionError> {
    let mut reader = Reader::from_str(relationships_xml);
    reader.config_mut().trim_text(true);
    let mut relationships = BTreeMap::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"Relationship") =>
            {
                let Some(id) = read_attr(&reader, &event, b"Id") else {
                    continue;
                };
                let Some(target) = read_attr(&reader, &event, b"Target") else {
                    continue;
                };
                relationships.insert(id, target);
            }
            Ok(Event::Eof) => break,
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse package relationships: {err}"
                )));
            }
            _ => {}
        }
    }
    Ok(relationships)
}

fn parse_pivot_table_targets(relationships_xml: &str) -> Result<Vec<String>, ExcelInspectionError> {
    let mut reader = Reader::from_str(relationships_xml);
    reader.config_mut().trim_text(true);
    let mut targets = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"Relationship") =>
            {
                let Some(relationship_type) = read_attr(&reader, &event, b"Type") else {
                    continue;
                };
                if !relationship_type.ends_with("/pivotTable") {
                    continue;
                }
                if let Some(target) = read_attr(&reader, &event, b"Target") {
                    targets.push(target);
                }
            }
            Ok(Event::Eof) => break,
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse worksheet relationships: {err}"
                )));
            }
            _ => {}
        }
    }
    Ok(targets)
}

fn parse_connections(
    connections_xml: &str,
) -> Result<BTreeMap<String, ConnectionInfo>, ExcelInspectionError> {
    let mut reader = Reader::from_str(connections_xml);
    reader.config_mut().trim_text(true);
    let mut connections = BTreeMap::new();
    let mut current_id: Option<String> = None;
    let mut current = ConnectionInfo::default();
    let mut in_connection = false;
    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) if tag_matches(event.name().as_ref(), b"connection") => {
                in_connection = true;
                current_id = read_attr(&reader, &event, b"id");
                current = ConnectionInfo {
                    name: read_attr(&reader, &event, b"name"),
                    connection_type: read_attr(&reader, &event, b"type"),
                    command: None,
                    olap: false,
                    data_model: false,
                };
            }
            Ok(Event::Empty(event))
                if in_connection && tag_matches(event.name().as_ref(), b"dbPr") =>
            {
                current.command = read_attr(&reader, &event, b"command");
                current.data_model |= read_attr(&reader, &event, b"connection")
                    .as_deref()
                    .is_some_and(|connection| connection.contains("Data Model"));
            }
            Ok(Event::Empty(event))
                if in_connection && tag_matches(event.name().as_ref(), b"olapPr") =>
            {
                current.olap = true;
            }
            Ok(Event::Empty(event))
                if in_connection
                    && tag_matches(event.name().as_ref(), b"connection")
                    && read_attr(&reader, &event, b"model").as_deref() == Some("1") =>
            {
                current.data_model = true;
            }
            Ok(Event::End(event)) if tag_matches(event.name().as_ref(), b"connection") => {
                in_connection = false;
                if let Some(id) = current_id.take() {
                    let mut normalized = current.clone();
                    normalized.connection_type = Some(normalize_connection_type(
                        normalized.connection_type.as_deref(),
                        normalized.olap,
                        normalized.command.as_deref(),
                    ));
                    normalized.data_model |= normalized
                        .name
                        .as_deref()
                        .is_some_and(|name| name == "ThisWorkbookDataModel");
                    connections.insert(id, normalized);
                }
            }
            Ok(Event::Eof) => break,
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse workbook connections: {err}"
                )));
            }
            _ => {}
        }
    }
    Ok(connections)
}

fn parse_cache_definition(
    cache_definition_xml: &str,
    cache_id: &str,
    connections: &BTreeMap<String, ConnectionInfo>,
) -> Result<CacheInfo, ExcelInspectionError> {
    let mut reader = Reader::from_str(cache_definition_xml);
    reader.config_mut().trim_text(true);
    let mut source_type = None;
    let mut source_name = None;
    let mut source_range = None;
    let mut connection_id = None;
    let mut field_lookup = Vec::new();
    let mut hierarchy_fallback = Vec::new();
    let mut in_cache_fields = false;
    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) if tag_matches(event.name().as_ref(), b"cacheFields") => {
                in_cache_fields = true;
            }
            Ok(Event::End(event)) if tag_matches(event.name().as_ref(), b"cacheFields") => {
                in_cache_fields = false;
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"cacheSource") =>
            {
                source_type = read_attr(&reader, &event, b"type");
                connection_id = read_attr(&reader, &event, b"connectionId");
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"worksheetSource") =>
            {
                let sheet = read_attr(&reader, &event, b"sheet");
                let range_ref = read_attr(&reader, &event, b"ref");
                let defined_name = read_attr(&reader, &event, b"name");
                source_type = Some("worksheet".to_string());
                source_name = defined_name.or_else(|| match (sheet, range_ref.as_deref()) {
                    (Some(sheet), Some(range_ref)) => Some(format!("{sheet}!{range_ref}")),
                    (Some(sheet), None) => Some(sheet),
                    (None, Some(range_ref)) => Some(range_ref.to_string()),
                    (None, None) => None,
                });
                source_range = range_ref;
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if in_cache_fields && tag_matches(event.name().as_ref(), b"cacheField") =>
            {
                if let Some(field_name) = read_attr(&reader, &event, b"caption")
                    .or_else(|| read_attr(&reader, &event, b"name"))
                {
                    field_lookup.push(field_name);
                }
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"cacheHierarchy") =>
            {
                if let Some(field_name) = read_attr(&reader, &event, b"caption")
                    .or_else(|| read_attr(&reader, &event, b"name"))
                {
                    hierarchy_fallback.push(field_name);
                }
            }
            Ok(Event::Eof) => break,
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse pivot cache {cache_id}: {err}"
                )));
            }
            _ => {}
        }
    }

    if field_lookup.is_empty() {
        field_lookup = hierarchy_fallback;
    }

    let connection = connection_id
        .as_deref()
        .and_then(|id| connections.get(id))
        .cloned();
    let (stored_mdx_preview, stored_mdx_truncated) = connection
        .as_ref()
        .and_then(|connection| connection.command.as_deref())
        .filter(|command| looks_like_mdx(command))
        .map(preview_text)
        .unwrap_or((None, false));

    let mut warnings = Vec::new();
    if let Some(id) = connection_id.as_deref()
        && connection.is_none()
    {
        push_warning(
            &mut warnings,
            format!("cache {cache_id} references missing connection id {id}"),
        );
    }

    let mut cache_fields_sample = sample_strings(&field_lookup, MAX_CACHE_FIELDS_SAMPLE);
    if field_lookup.len() > MAX_CACHE_FIELDS_SAMPLE {
        push_warning(
            &mut warnings,
            format!(
                "cache field sample truncated to {MAX_CACHE_FIELDS_SAMPLE} of {} entries",
                field_lookup.len()
            ),
        );
    }

    let mut public = PivotCacheReportSummary {
        cache_id: Some(cache_id.to_string()),
        source_type,
        source_name,
        source_range,
        connection_id,
        connection_name: connection.as_ref().and_then(|item| item.name.clone()),
        connection_type: connection
            .as_ref()
            .and_then(|item| item.connection_type.clone()),
        olap: connection.as_ref().is_some_and(|item| item.olap),
        data_model: connection.as_ref().is_some_and(|item| item.data_model),
        cache_fields_sample: Vec::new(),
        warnings,
    };
    if public.source_type.as_deref() == Some("external") && public.olap {
        public.source_type = Some("olap".to_string());
    }
    if public.source_name.is_none() && public.connection_name.is_some() {
        public.source_name = public.connection_name.clone();
    }
    public.cache_fields_sample.append(&mut cache_fields_sample);

    Ok(CacheInfo {
        public,
        field_lookup,
        stored_mdx_preview,
        stored_mdx_truncated,
    })
}

fn parse_pivot_table(
    pivot_table_xml: &str,
    worksheet_name: &str,
    part_path: &str,
    caches: &BTreeMap<String, CacheInfo>,
) -> Result<PivotTableReportSummary, ExcelInspectionError> {
    let mut reader = Reader::from_str(pivot_table_xml);
    reader.config_mut().trim_text(true);
    let mut name = None;
    let mut cache_id = None;
    let mut range_ref = None;
    let mut row_field_indices = Vec::new();
    let mut column_field_indices = Vec::new();
    let mut data_field_indices = Vec::new();
    let mut page_field_indices = Vec::new();
    let mut in_row_fields = false;
    let mut in_col_fields = false;
    let mut in_data_fields = false;
    let mut in_page_fields = false;
    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) if tag_matches(event.name().as_ref(), b"rowFields") => {
                in_row_fields = true;
            }
            Ok(Event::End(event)) if tag_matches(event.name().as_ref(), b"rowFields") => {
                in_row_fields = false;
            }
            Ok(Event::Start(event)) if tag_matches(event.name().as_ref(), b"colFields") => {
                in_col_fields = true;
            }
            Ok(Event::End(event)) if tag_matches(event.name().as_ref(), b"colFields") => {
                in_col_fields = false;
            }
            Ok(Event::Start(event)) if tag_matches(event.name().as_ref(), b"dataFields") => {
                in_data_fields = true;
            }
            Ok(Event::End(event)) if tag_matches(event.name().as_ref(), b"dataFields") => {
                in_data_fields = false;
            }
            Ok(Event::Start(event)) if tag_matches(event.name().as_ref(), b"pageFields") => {
                in_page_fields = true;
            }
            Ok(Event::End(event)) if tag_matches(event.name().as_ref(), b"pageFields") => {
                in_page_fields = false;
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"pivotTableDefinition") =>
            {
                name = read_attr(&reader, &event, b"name");
                cache_id = read_attr(&reader, &event, b"cacheId");
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if tag_matches(event.name().as_ref(), b"location") =>
            {
                range_ref = read_attr(&reader, &event, b"ref");
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if in_row_fields && tag_matches(event.name().as_ref(), b"field") =>
            {
                if let Some(index) = read_index_attr(&reader, &event, b"x") {
                    row_field_indices.push(index);
                }
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if in_col_fields && tag_matches(event.name().as_ref(), b"field") =>
            {
                if let Some(index) = read_index_attr(&reader, &event, b"x") {
                    column_field_indices.push(index);
                }
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if in_data_fields && tag_matches(event.name().as_ref(), b"dataField") =>
            {
                if let Some(index) = read_index_attr(&reader, &event, b"fld") {
                    data_field_indices.push(index);
                }
            }
            Ok(Event::Start(event)) | Ok(Event::Empty(event))
                if in_page_fields && tag_matches(event.name().as_ref(), b"pageField") =>
            {
                if let Some(index) = read_index_attr(&reader, &event, b"fld") {
                    page_field_indices.push(index);
                }
            }
            Ok(Event::Eof) => break,
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse pivot table {part_path}: {err}"
                )));
            }
            _ => {}
        }
    }

    let mut warnings = Vec::new();
    let cache = cache_id.as_deref().and_then(|id| caches.get(id));
    if let Some(id) = cache_id.as_deref()
        && cache.is_none()
    {
        push_warning(
            &mut warnings,
            format!("pivot table references missing cache id {id}"),
        );
    }
    let resolve_fields = |indices: &[usize], warnings: &mut Vec<String>| {
        let lookup = cache.map_or(&[][..], |cache| cache.field_lookup.as_slice());
        sample_fields(indices, lookup, warnings)
    };

    let cache_public = cache.map(|cache| &cache.public);
    Ok(PivotTableReportSummary {
        name,
        worksheet_name: Some(worksheet_name.to_string()),
        part_path: Some(part_path.to_string()),
        range_ref,
        cache_id,
        source_type: cache_public.and_then(|cache| cache.source_type.clone()),
        source_name: cache_public.and_then(|cache| cache.source_name.clone()),
        source_range: cache_public.and_then(|cache| cache.source_range.clone()),
        connection_id: cache_public.and_then(|cache| cache.connection_id.clone()),
        connection_name: cache_public.and_then(|cache| cache.connection_name.clone()),
        connection_type: cache_public.and_then(|cache| cache.connection_type.clone()),
        olap: cache_public.is_some_and(|cache| cache.olap),
        data_model: cache_public.is_some_and(|cache| cache.data_model),
        stored_mdx_preview: cache.and_then(|cache| cache.stored_mdx_preview.clone()),
        stored_mdx_truncated: cache.is_some_and(|cache| cache.stored_mdx_truncated),
        row_fields_sample: resolve_fields(&row_field_indices, &mut warnings),
        column_fields_sample: resolve_fields(&column_field_indices, &mut warnings),
        data_fields_sample: resolve_fields(&data_field_indices, &mut warnings),
        page_fields_sample: resolve_fields(&page_field_indices, &mut warnings),
        warnings,
    })
}

fn sample_fields(indices: &[usize], lookup: &[String], warnings: &mut Vec<String>) -> Vec<String> {
    let mut resolved = Vec::new();
    for index in indices.iter().copied().take(MAX_FIELDS_PER_ROLE_SAMPLE) {
        if let Some(name) = lookup.get(index) {
            resolved.push(name.clone());
        } else {
            push_warning(
                warnings,
                format!("pivot field index {index} is outside the cache field sample"),
            );
        }
    }
    if indices.len() > MAX_FIELDS_PER_ROLE_SAMPLE {
        push_warning(
            warnings,
            format!(
                "field role sample truncated to {MAX_FIELDS_PER_ROLE_SAMPLE} of {} entries",
                indices.len()
            ),
        );
    }
    resolved
}

fn sample_strings(values: &[String], limit: usize) -> Vec<String> {
    values.iter().take(limit).cloned().collect()
}

fn preview_text(text: &str) -> (Option<String>, bool) {
    let truncated = text.chars().count() > MAX_STORED_MDX_PREVIEW_CHARS;
    let preview = text
        .chars()
        .take(MAX_STORED_MDX_PREVIEW_CHARS)
        .collect::<String>();
    (Some(preview), truncated)
}

fn looks_like_mdx(command: &str) -> bool {
    let trimmed = command.trim_start();
    let lowercase = trimmed.to_ascii_lowercase();
    lowercase.starts_with("select ")
        || lowercase.starts_with("with ")
        || lowercase.starts_with("drillthrough ")
}

fn normalize_connection_type(raw_type: Option<&str>, olap: bool, command: Option<&str>) -> String {
    if olap {
        return "olap".to_string();
    }
    if raw_type == Some("100") {
        return "query".to_string();
    }
    if command.is_some() {
        return "database".to_string();
    }
    raw_type.unwrap_or("unknown").to_string()
}

fn relationships_path(part_path: &str) -> String {
    let base_dir = directory_of(part_path);
    let file_name = part_path.rsplit('/').next().unwrap_or(part_path);
    format!("{base_dir}/_rels/{file_name}.rels")
}

fn directory_of(part_path: &str) -> String {
    part_path
        .rsplit_once('/')
        .map(|(directory, _)| directory.to_string())
        .unwrap_or_default()
}

fn normalize_part_path(base_dir: &str, target: &str) -> String {
    let mut parts = base_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    for segment in target.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            other => parts.push(other.to_string()),
        }
    }
    parts.join("/")
}

fn read_index_attr(reader: &Reader<&[u8]>, event: &BytesStart<'_>, key: &[u8]) -> Option<usize> {
    read_attr(reader, event, key)?.parse().ok()
}

fn tag_matches(name: &[u8], local_name: &[u8]) -> bool {
    name == local_name || name.rsplit(|byte| *byte == b':').next() == Some(local_name)
}

fn read_attr(reader: &Reader<&[u8]>, event: &BytesStart<'_>, key: &[u8]) -> Option<String> {
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
    event: &BytesStart<'_>,
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

fn push_warning(warnings: &mut Vec<String>, warning: impl Into<String>) {
    if warnings.len() < MAX_WARNINGS {
        warnings.push(warning.into());
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
