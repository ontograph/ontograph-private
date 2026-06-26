use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use quick_xml::Reader;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use zip::ZipArchive;

use crate::tool::InspectWorkbookResult;
use crate::tool::MarkerSummary;
use crate::tool::SheetKind;
use crate::tool::SheetSummary;
use crate::tool::SheetVisibility;
use crate::tool::WorkbookFormat;
use crate::tool::WorkbookMarkers;

const MAX_PACKAGE_PART_SAMPLE: usize = 40;
const MAX_MARKER_PART_SAMPLE: usize = 16;
const MAX_SHEET_SUMMARY_COUNT: usize = 64;
const MAX_PACKAGE_PART_COUNT: usize = 4096;
const MAX_XML_ENTRY_BYTES: usize = 1024 * 1024;
const MAX_XML_SCAN_ENTRIES: usize = 128;
const MAX_XML_SCAN_BYTES: usize = 8 * 1024 * 1024;
const MAX_RESULT_PATH_CHARS: usize = 512;
const MAX_PART_PATH_CHARS: usize = 512;
const MAX_SHEET_TEXT_CHARS: usize = 256;

#[derive(Debug, Error)]
pub enum ExcelInspectionError {
    #[error("{0}")]
    Message(String),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
struct RawSheetSummary {
    name: Option<String>,
    sheet_id: Option<u32>,
    relationship_id: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Relationship {
    target: String,
    relationship_type: String,
}

#[derive(Debug)]
struct XmlReadBudget {
    remaining_entries: usize,
    remaining_bytes: usize,
}

impl XmlReadBudget {
    fn new() -> Self {
        Self {
            remaining_entries: MAX_XML_SCAN_ENTRIES,
            remaining_bytes: MAX_XML_SCAN_BYTES,
        }
    }

    fn consume_entry(
        &mut self,
        name: &str,
        entry_size: u64,
    ) -> Result<usize, ExcelInspectionError> {
        if self.remaining_entries == 0 {
            return Err(ExcelInspectionError::Message(format!(
                "workbook XML scan exceeded {MAX_XML_SCAN_ENTRIES} entries"
            )));
        }
        let entry_size = usize::try_from(entry_size).map_err(|_| {
            ExcelInspectionError::Message(format!("workbook entry {name} is too large to inspect"))
        })?;
        if entry_size > MAX_XML_ENTRY_BYTES {
            return Err(ExcelInspectionError::Message(format!(
                "workbook entry {name} exceeds {MAX_XML_ENTRY_BYTES} bytes"
            )));
        }
        if entry_size > self.remaining_bytes {
            return Err(ExcelInspectionError::Message(format!(
                "workbook XML scan exceeds {MAX_XML_SCAN_BYTES} total bytes"
            )));
        }
        self.remaining_entries -= 1;
        self.remaining_bytes -= entry_size;
        Ok(entry_size)
    }
}

#[cfg(test)]
pub(crate) fn inspect_workbook(path: &Path) -> Result<InspectWorkbookResult, ExcelInspectionError> {
    inspect_workbook_with_display_path(path, path)
}

pub(crate) fn inspect_workbook_with_display_path(
    path: &Path,
    display_path: &Path,
) -> Result<InspectWorkbookResult, ExcelInspectionError> {
    let file = File::open(path).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to open workbook {}: {err}", path.display()))
    })?;
    let mut archive = ZipArchive::new(file).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to read workbook archive {}: {err}",
            path.display()
        ))
    })?;

    if archive.len() > MAX_PACKAGE_PART_COUNT {
        return Err(ExcelInspectionError::Message(format!(
            "workbook package has {} entries; maximum supported is {MAX_PACKAGE_PART_COUNT}",
            archive.len()
        )));
    }

    let mut part_names = Vec::new();
    for index in 0..archive.len() {
        let entry = archive.by_index(index).map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to read workbook entry {index} in {}: {err}",
                path.display()
            ))
        })?;
        part_names.push(normalize_part_name(entry.name()));
    }

    let mut xml_read_budget = XmlReadBudget::new();
    let content_types =
        read_optional_entry(&mut archive, "[Content_Types].xml", &mut xml_read_budget)?;
    let workbook_xml = read_optional_entry(&mut archive, "xl/workbook.xml", &mut xml_read_budget)?;
    let workbook_rels = read_optional_entry(
        &mut archive,
        "xl/_rels/workbook.xml.rels",
        &mut xml_read_budget,
    )?;

    let vba_parts = matching_parts(&part_names, |name| name == "xl/vbaProject.bin");
    let connection_parts = matching_parts(&part_names, |name| {
        name == "xl/connections.xml" || name == "xl/connections.bin"
    });
    let custom_xml_parts = matching_parts(&part_names, |name| name.starts_with("customXml/"));
    let external_link_parts =
        matching_parts(&part_names, |name| name.starts_with("xl/externalLinks/"));
    let table_parts = matching_parts(&part_names, |name| name.starts_with("xl/tables/"));
    let comment_parts = matching_parts(&part_names, |name| {
        name.starts_with("xl/comments") || name.starts_with("xl/threadedComments/")
    });
    let drawing_parts = matching_parts(&part_names, |name| name.starts_with("xl/drawings/"));
    let embedded_object_parts = matching_parts(&part_names, |name| {
        name.starts_with("xl/embeddings/") || name.contains("oleObject")
    });
    let chart_parts = matching_parts(&part_names, |name| {
        name.starts_with("xl/charts/") || name.starts_with("xl/chartsheets/")
    });
    let pivot_parts = matching_parts(&part_names, |name| {
        name.starts_with("xl/pivotTables/") || name.starts_with("xl/pivotCache/")
    });
    let xlsb_parts = matching_parts(&part_names, |name| {
        name == "xl/workbook.bin" || name.ends_with("/workbook.bin")
    });
    let formula_parts = formula_parts(&mut archive, &part_names, &mut xml_read_budget)?;
    let power_query_parts = power_query_parts(&mut archive, &part_names, &mut xml_read_budget)?;

    let markers = WorkbookMarkers {
        has_vba_project: !vba_parts.is_empty(),
        has_macro_enabled_package: content_types
            .as_deref()
            .is_some_and(|value| value.contains("macroEnabled")),
        has_power_query: !power_query_parts.is_empty(),
        has_connections: !connection_parts.is_empty(),
        has_custom_xml: !custom_xml_parts.is_empty(),
        has_external_links: !external_link_parts.is_empty(),
        has_tables: !table_parts.is_empty(),
        has_comments: !comment_parts.is_empty(),
        has_drawings: !drawing_parts.is_empty(),
        has_embedded_objects: !embedded_object_parts.is_empty(),
        has_charts: !chart_parts.is_empty(),
        has_pivot_tables: !pivot_parts.is_empty(),
        has_formulas: !formula_parts.is_empty(),
        has_xlsb_package: !xlsb_parts.is_empty(),
    };

    let marker_summaries = marker_summaries([
        ("vba_project", vba_parts),
        ("power_query", power_query_parts),
        ("connections", connection_parts),
        ("custom_xml", custom_xml_parts),
        ("external_links", external_link_parts),
        ("tables", table_parts),
        ("comments", comment_parts),
        ("drawings", drawing_parts),
        ("embedded_objects", embedded_object_parts),
        ("charts", chart_parts),
        ("pivot_tables", pivot_parts),
        ("formulas", formula_parts),
        ("xlsb_package", xlsb_parts),
    ]);

    let format = workbook_format(
        content_types.as_deref(),
        markers.has_xlsb_package,
        markers.has_macro_enabled_package,
        workbook_xml.is_some(),
    );

    let mut warnings = Vec::new();
    if part_names.len() > MAX_PACKAGE_PART_SAMPLE {
        warnings.push(format!(
            "package part list truncated to {} of {} entries",
            MAX_PACKAGE_PART_SAMPLE,
            part_names.len()
        ));
    }

    let package_parts_sample = part_names
        .iter()
        .take(MAX_PACKAGE_PART_SAMPLE)
        .map(|name| bounded_text(name, MAX_PART_PATH_CHARS))
        .collect::<Vec<_>>();

    let mut sheets = match format {
        WorkbookFormat::Xlsx | WorkbookFormat::Xlsm => {
            let workbook_xml = workbook_xml.ok_or_else(|| {
                ExcelInspectionError::Message(format!(
                    "workbook.xml is missing from {}",
                    path.display()
                ))
            })?;
            parse_openxml_workbook(&workbook_xml, workbook_rels.as_deref())?
        }
        WorkbookFormat::Xlsb => {
            warnings.push("xlsb workbook names are not decoded in this stage; only package parts are reported".to_string());
            part_names
                .iter()
                .filter(|name| name.starts_with("xl/worksheets/") && name.ends_with(".bin"))
                .take(MAX_SHEET_SUMMARY_COUNT)
                .map(|name| SheetSummary {
                    name: None,
                    sheet_id: None,
                    relationship_id: None,
                    part_path: Some(bounded_text(name, MAX_PART_PATH_CHARS)),
                    visibility: SheetVisibility::Unknown,
                    kind: SheetKind::Worksheet,
                })
                .collect()
        }
        WorkbookFormat::Unknown => {
            warnings.push("workbook package format could not be classified".to_string());
            Vec::new()
        }
    };

    if let WorkbookFormat::Xlsx | WorkbookFormat::Xlsm = format
        && sheets.is_empty()
    {
        warnings.push("no workbook sheets were decoded from workbook.xml".to_string());
    }
    if sheets.len() > MAX_SHEET_SUMMARY_COUNT {
        warnings.push(format!(
            "sheet summary truncated to {MAX_SHEET_SUMMARY_COUNT} entries"
        ));
        sheets.truncate(MAX_SHEET_SUMMARY_COUNT);
    }

    Ok(InspectWorkbookResult {
        path: bounded_text(&display_path.display().to_string(), MAX_RESULT_PATH_CHARS),
        format,
        package_part_count: part_names.len(),
        package_parts_sample,
        sheets,
        markers,
        marker_summaries,
        warnings,
    })
}

fn workbook_format(
    content_types: Option<&str>,
    has_xlsb_package: bool,
    has_macro_enabled_package: bool,
    has_workbook_xml: bool,
) -> WorkbookFormat {
    if has_xlsb_package
        || content_types.is_some_and(|value| value.contains("sheet.binary.macroEnabled"))
    {
        return WorkbookFormat::Xlsb;
    }
    if has_macro_enabled_package
        || content_types.is_some_and(|value| value.contains("sheet.macroEnabled"))
    {
        return WorkbookFormat::Xlsm;
    }
    if has_workbook_xml {
        return WorkbookFormat::Xlsx;
    }
    WorkbookFormat::Unknown
}

fn parse_openxml_workbook(
    workbook_xml: &str,
    workbook_rels: Option<&str>,
) -> Result<Vec<SheetSummary>, ExcelInspectionError> {
    let raw_sheets = parse_sheet_nodes(workbook_xml)?;
    let relationships = workbook_rels
        .map(parse_relationships)
        .transpose()?
        .unwrap_or_default();
    let sheets = raw_sheets
        .into_iter()
        .map(|sheet| {
            let relationship = sheet
                .relationship_id
                .as_deref()
                .and_then(|id| relationships.get(id));
            let part_path = relationship.map(|rel| {
                bounded_text(
                    &normalize_part_path("xl/workbook.xml", &rel.target),
                    MAX_PART_PATH_CHARS,
                )
            });
            let kind = relationship
                .map(|rel| sheet_kind_from_relationship_type(&rel.relationship_type))
                .or_else(|| part_path.as_deref().map(sheet_kind_from_part_path))
                .unwrap_or(SheetKind::Unknown);
            SheetSummary {
                name: sheet
                    .name
                    .as_deref()
                    .map(|name| bounded_text(name, MAX_SHEET_TEXT_CHARS)),
                sheet_id: sheet.sheet_id,
                relationship_id: sheet
                    .relationship_id
                    .as_deref()
                    .map(|id| bounded_text(id, MAX_SHEET_TEXT_CHARS)),
                part_path,
                visibility: sheet_visibility(sheet.state.as_deref()),
                kind,
            }
        })
        .collect::<Vec<_>>();
    Ok(sheets)
}

fn parse_sheet_nodes(workbook_xml: &str) -> Result<Vec<RawSheetSummary>, ExcelInspectionError> {
    let mut reader = Reader::from_str(workbook_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut sheets = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"sheet" =>
            {
                if sheets.len() < MAX_SHEET_SUMMARY_COUNT {
                    sheets.push(RawSheetSummary {
                        name: attr_value(&event, b"name")?,
                        sheet_id: attr_value(&event, b"sheetId")?
                            .and_then(|value| value.parse().ok()),
                        relationship_id: match attr_value(&event, b"r:id")? {
                            Some(value) => Some(value),
                            None => attr_value(&event, b"id")?,
                        },
                        state: attr_value(&event, b"state")?,
                    });
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse workbook sheet list: {err}"
                )));
            }
        }
        buf.clear();
    }
    Ok(sheets)
}

fn parse_relationships(
    workbook_rels: &str,
) -> Result<std::collections::BTreeMap<String, Relationship>, ExcelInspectionError> {
    let mut reader = Reader::from_str(workbook_rels);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut relationships = std::collections::BTreeMap::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"Relationship" =>
            {
                let id = attr_value(&event, b"Id")?.unwrap_or_default();
                if id.is_empty() {
                    buf.clear();
                    continue;
                }
                relationships.insert(
                    id,
                    Relationship {
                        target: attr_value(&event, b"Target")?.unwrap_or_default(),
                        relationship_type: attr_value(&event, b"Type")?.unwrap_or_default(),
                    },
                );
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse workbook relationships: {err}"
                )));
            }
        }
        buf.clear();
    }
    Ok(relationships)
}

fn read_optional_entry(
    archive: &mut ZipArchive<File>,
    name: &str,
    budget: &mut XmlReadBudget,
) -> Result<Option<String>, ExcelInspectionError> {
    read_optional_named_entry(archive, name, budget)
}

fn read_optional_named_entry(
    archive: &mut ZipArchive<File>,
    name: &str,
    budget: &mut XmlReadBudget,
) -> Result<Option<String>, ExcelInspectionError> {
    let Ok(mut entry) = archive.by_name(name) else {
        return Ok(None);
    };
    let limit = budget.consume_entry(name, entry.size())?;
    let mut limited = entry.by_ref().take((limit + 1) as u64);
    let mut bytes = Vec::with_capacity(limit.saturating_add(1));
    limited.read_to_end(&mut bytes).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to read workbook entry {name}: {err}"))
    })?;
    if bytes.len() > limit {
        return Err(ExcelInspectionError::Message(format!(
            "workbook entry {name} exceeds its declared inspection budget"
        )));
    }
    let contents = decode_xml_text_bytes(&bytes)?;
    Ok(Some(contents))
}

fn matching_parts(part_names: &[String], matches: impl Fn(&str) -> bool) -> Vec<String> {
    part_names
        .iter()
        .filter(|name| matches(name))
        .cloned()
        .collect()
}

fn formula_parts(
    archive: &mut ZipArchive<File>,
    part_names: &[String],
    budget: &mut XmlReadBudget,
) -> Result<Vec<String>, ExcelInspectionError> {
    let mut parts = Vec::new();
    for name in part_names
        .iter()
        .filter(|name| name.starts_with("xl/worksheets/") && name.ends_with(".xml"))
    {
        if let Some(text) = read_optional_named_entry(archive, name, budget)?
            && text.contains("<f")
        {
            parts.push(name.clone());
        }
    }
    Ok(parts)
}

fn power_query_parts(
    archive: &mut ZipArchive<File>,
    part_names: &[String],
    budget: &mut XmlReadBudget,
) -> Result<Vec<String>, ExcelInspectionError> {
    let mut parts = Vec::new();
    if let Some(connections) = read_optional_named_entry(archive, "xl/connections.xml", budget)?
        && (connections.contains("Microsoft.Mashup") || connections.contains("DataMashup"))
    {
        parts.push("xl/connections.xml".to_string());
    }
    for custom_xml in part_names
        .iter()
        .filter(|name| name.starts_with("customXml/item") && name.ends_with(".xml"))
    {
        if let Some(text) = read_optional_named_entry(archive, custom_xml, budget)?
            && text.contains("DataMashup")
        {
            parts.push(custom_xml.clone());
        }
    }
    Ok(parts)
}

fn marker_summaries<const N: usize>(
    parts_by_category: [(&str, Vec<String>); N],
) -> Vec<MarkerSummary> {
    parts_by_category
        .into_iter()
        .filter(|(_, parts)| !parts.is_empty())
        .map(|(category, parts)| MarkerSummary {
            category: category.to_string(),
            count: parts.len(),
            part_paths_sample: parts
                .iter()
                .take(MAX_MARKER_PART_SAMPLE)
                .map(|part| bounded_text(part, MAX_PART_PATH_CHARS))
                .collect(),
        })
        .collect()
}

fn bounded_text(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let mut truncated = value
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    truncated.push_str("...");
    truncated
}

fn attr_value(event: &BytesStart<'_>, key: &[u8]) -> Result<Option<String>, ExcelInspectionError> {
    for attr in event.attributes().with_checks(false) {
        let attr = attr.map_err(|err| {
            ExcelInspectionError::Message(format!("failed to parse workbook attribute: {err}"))
        })?;
        if attr.key.as_ref() == key {
            return Ok(Some(
                String::from_utf8_lossy(attr.value.as_ref()).into_owned(),
            ));
        }
    }
    Ok(None)
}

fn normalize_part_name(name: &str) -> String {
    name.trim_start_matches("./").replace('\\', "/")
}

fn decode_xml_text_bytes(bytes: &[u8]) -> Result<String, ExcelInspectionError> {
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

fn normalize_part_path(base_part: &str, target: &str) -> String {
    let mut base = PathBuf::from(base_part);
    let _ = base.pop();
    let mut path = if target.starts_with('/') {
        PathBuf::new()
    } else {
        base
    };
    path.push(target);
    normalize_path(path)
}

fn normalize_path(path: PathBuf) -> String {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            std::path::Component::CurDir => {}
            std::path::Component::Normal(segment) => normalized.push(segment),
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {
                normalized.push(component.as_os_str())
            }
        }
    }
    normalized.to_string_lossy().replace('\\', "/")
}

fn sheet_visibility(state: Option<&str>) -> SheetVisibility {
    match state {
        Some("hidden") => SheetVisibility::Hidden,
        Some("veryHidden") => SheetVisibility::VeryHidden,
        Some("visible") | None => SheetVisibility::Visible,
        Some(_) => SheetVisibility::Unknown,
    }
}

fn sheet_kind_from_relationship_type(relationship_type: &str) -> SheetKind {
    if relationship_type.contains("/chartsheet") {
        SheetKind::Chartsheet
    } else if relationship_type.contains("/dialogsheet") {
        SheetKind::DialogSheet
    } else if relationship_type.contains("/macrosheet") {
        SheetKind::MacroSheet
    } else if relationship_type.contains("/worksheet") {
        SheetKind::Worksheet
    } else {
        SheetKind::Unknown
    }
}

fn sheet_kind_from_part_path(part_path: &str) -> SheetKind {
    if part_path.contains("/chartsheets/") {
        SheetKind::Chartsheet
    } else if part_path.contains("/dialogsheets/") {
        SheetKind::DialogSheet
    } else if part_path.contains("/macrosheets/") {
        SheetKind::MacroSheet
    } else if part_path.contains("/worksheets/") {
        SheetKind::Worksheet
    } else {
        SheetKind::Unknown
    }
}
