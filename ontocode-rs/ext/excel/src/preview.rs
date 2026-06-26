use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use quick_xml::Reader;
use quick_xml::escape::resolve_predefined_entity;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use zip::ZipArchive;

use crate::backend::ExcelInspectionError;
use crate::backend::inspect_workbook_with_display_path;
use crate::tool::CellContentMode;
use crate::tool::ReadSheetPreviewResult;
use crate::tool::SheetDataValidationSummary;
use crate::tool::SheetDimension;
use crate::tool::SheetPreview;
use crate::tool::SheetPreviewCell;
use crate::tool::SheetPreviewRow;
use crate::tool::SheetSelector;
use crate::tool::WorkbookFormat;

const DEFAULT_PREVIEW_ROWS: usize = 20;
const MAX_PREVIEW_ROWS: usize = 50;
const MAX_PREVIEW_COLUMNS: usize = 32;
const MAX_PREVIEW_CELL_TEXT_CHARS: usize = 256;
const MAX_PREVIEW_SHARED_STRINGS: usize = 4096;
const MAX_PREVIEW_SHARED_STRING_TEXT_CHARS: usize = 1024;
const MAX_WORKSHEET_XML_BYTES: usize = 2 * 1024 * 1024;
const MAX_SHARED_STRINGS_XML_BYTES: usize = 2 * 1024 * 1024;
const MAX_DATA_VALIDATION_SUMMARIES: usize = 64;
const MAX_DATA_VALIDATION_RANGE_SAMPLE: usize = 16;
const MAX_DATA_VALIDATION_FORMULA_CHARS: usize = 256;
const MAX_DATA_VALIDATION_RESOLVED_VALUES: usize = 128;
const MAX_DATA_VALIDATION_TOTAL_RESOLVED_VALUES: usize = 256;

pub(crate) fn read_sheet_preview_with_display_path(
    path: &Path,
    display_path: &Path,
    sheet: &SheetSelector,
    max_rows: Option<usize>,
    cell_content: CellContentMode,
) -> Result<ReadSheetPreviewResult, ExcelInspectionError> {
    let workbook = inspect_workbook_with_display_path(path, display_path)?;
    if !matches!(workbook.format, WorkbookFormat::Xlsx | WorkbookFormat::Xlsm) {
        return Err(ExcelInspectionError::Message(
            "excel.read_sheet_preview supports only .xlsx and .xlsm in this stage".to_string(),
        ));
    }

    let selected_sheet = select_sheet(&workbook.sheets, sheet)?;
    let sheet_name = selected_sheet.name.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.read_sheet_preview could not resolve a sheet name for the selected sheet"
                .to_string(),
        )
    })?;
    let sheet_part_path = selected_sheet.part_path.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.read_sheet_preview could not resolve a worksheet part path for the selected sheet"
                .to_string(),
        )
    })?;

    let capped_rows = max_rows
        .unwrap_or(DEFAULT_PREVIEW_ROWS)
        .min(MAX_PREVIEW_ROWS);

    let file = File::open(path).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to open workbook {}: {err}", path.display()))
    })?;
    let mut archive = ZipArchive::new(file).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to read workbook archive {}: {err}",
            path.display()
        ))
    })?;

    let shared_strings = read_shared_strings(&mut archive)?;
    let worksheet_xml = read_xml_entry(&mut archive, &sheet_part_path, MAX_WORKSHEET_XML_BYTES)?;
    let dimension = parse_sheet_dimension(&worksheet_xml)?;
    let (rows, truncated) =
        parse_sheet_preview(&worksheet_xml, &shared_strings, capped_rows, cell_content)?;
    let cell_values = parse_sheet_cell_values(&worksheet_xml, &shared_strings)?;
    let (data_validations, validation_warnings) =
        parse_data_validations(&worksheet_xml, &cell_values, &sheet_name)?;

    let mut warnings = Vec::new();
    if max_rows.unwrap_or(DEFAULT_PREVIEW_ROWS) > MAX_PREVIEW_ROWS {
        warnings.push(format!(
            "max_rows capped to {MAX_PREVIEW_ROWS} for excel.read_sheet_preview"
        ));
    }
    if truncated {
        warnings.push(format!(
            "sheet preview truncated to {capped_rows} rows and {MAX_PREVIEW_COLUMNS} columns"
        ));
    }
    warnings.extend(validation_warnings);

    Ok(ReadSheetPreviewResult {
        path: display_path.display().to_string(),
        sheet: SheetPreview {
            name: sheet_name,
            sheet_id: selected_sheet.sheet_id,
            part_path: sheet_part_path,
        },
        dimension,
        max_rows_applied: capped_rows,
        cell_content,
        rows,
        data_validations,
        truncated,
        warnings,
    })
}

fn parse_sheet_dimension(
    worksheet_xml: &str,
) -> Result<Option<SheetDimension>, ExcelInspectionError> {
    let mut reader = Reader::from_str(worksheet_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event) | Event::Empty(event))
                if event.name().as_ref() == b"dimension" =>
            {
                return Ok(attr_value(&event, b"ref")?
                    .filter(|reference| !reference.is_empty())
                    .map(|reference| SheetDimension {
                        reference: bounded_text(&reference, 64),
                    }));
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse worksheet dimension: {err}"
                )));
            }
        }
        buf.clear();
    }

    Ok(None)
}

pub(crate) fn select_sheet<'a>(
    sheets: &'a [crate::tool::SheetSummary],
    selector: &SheetSelector,
) -> Result<&'a crate::tool::SheetSummary, ExcelInspectionError> {
    match selector {
        SheetSelector::Name { name } => sheets
            .iter()
            .find(|sheet| sheet.name.as_deref() == Some(name.as_str()))
            .ok_or_else(|| {
                ExcelInspectionError::Message(format!(
                    "excel.read_sheet_preview could not find sheet named {name}"
                ))
            }),
        SheetSelector::Index { index } => sheets.get(*index).ok_or_else(|| {
            ExcelInspectionError::Message(format!(
                "excel.read_sheet_preview could not find sheet index {index}"
            ))
        }),
    }
}

pub(crate) fn read_shared_strings(
    archive: &mut ZipArchive<File>,
) -> Result<Vec<String>, ExcelInspectionError> {
    let Ok(xml) = read_xml_entry(
        archive,
        "xl/sharedStrings.xml",
        MAX_SHARED_STRINGS_XML_BYTES,
    ) else {
        return Ok(Vec::new());
    };

    parse_shared_strings(&xml)
}

pub(crate) fn read_xml_entry(
    archive: &mut ZipArchive<File>,
    name: &str,
    max_bytes: usize,
) -> Result<String, ExcelInspectionError> {
    let mut entry = archive.by_name(name).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to read workbook entry {name}: {err}"))
    })?;
    let entry_size = usize::try_from(entry.size()).map_err(|_| {
        ExcelInspectionError::Message(format!("workbook entry {name} is too large to inspect"))
    })?;
    if entry_size > max_bytes {
        return Err(ExcelInspectionError::Message(format!(
            "workbook entry {name} exceeds {max_bytes} bytes"
        )));
    }

    let mut contents = String::new();
    entry.read_to_string(&mut contents).map_err(|err| {
        ExcelInspectionError::Message(format!("failed to read workbook entry {name}: {err}"))
    })?;
    Ok(contents)
}

fn parse_shared_strings(xml: &str) -> Result<Vec<String>, ExcelInspectionError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut strings = Vec::new();
    let mut in_si = false;
    let mut capture_text = false;
    let mut current = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => match event.name().as_ref() {
                b"si" => {
                    in_si = true;
                    current.clear();
                }
                b"t" if in_si => {
                    capture_text = true;
                }
                _ => {}
            },
            Ok(Event::End(event)) => match event.name().as_ref() {
                b"t" => capture_text = false,
                b"si" => {
                    if strings.len() < MAX_PREVIEW_SHARED_STRINGS {
                        strings.push(bounded_text(
                            current.as_str(),
                            MAX_PREVIEW_SHARED_STRING_TEXT_CHARS,
                        ));
                    }
                    current.clear();
                    in_si = false;
                }
                _ => {}
            },
            Ok(Event::Text(text)) if capture_text && in_si => {
                current.push_str(
                    text.decode()
                        .map_err(|err| {
                            ExcelInspectionError::Message(format!(
                                "failed to decode shared string text: {err}"
                            ))
                        })?
                        .as_ref(),
                );
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse shared strings: {err}"
                )));
            }
        }
        buf.clear();
    }

    Ok(strings)
}

fn parse_sheet_preview(
    worksheet_xml: &str,
    shared_strings: &[String],
    max_rows: usize,
    cell_content: CellContentMode,
) -> Result<(Vec<SheetPreviewRow>, bool), ExcelInspectionError> {
    let mut reader = Reader::from_str(worksheet_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut rows = Vec::new();
    let mut truncated = false;

    let mut current_row_index = None;
    let mut current_cells = Vec::new();
    let mut current_cell = None;
    let mut current_formula = String::new();
    let mut current_value = String::new();
    let mut current_inline = String::new();
    let mut capture_formula = false;
    let mut capture_value = false;
    let mut capture_inline = false;
    let mut in_sheet_data = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => match event.name().as_ref() {
                b"sheetData" => in_sheet_data = true,
                b"row" if in_sheet_data => {
                    if rows.len() >= max_rows {
                        truncated = true;
                    } else {
                        current_row_index = Some(row_index(&event)?);
                        current_cells.clear();
                    }
                }
                b"c" if in_sheet_data && rows.len() < max_rows => {
                    if current_cells.len() >= MAX_PREVIEW_COLUMNS {
                        current_cell = None;
                        truncated = true;
                    } else {
                        current_formula.clear();
                        current_value.clear();
                        current_inline.clear();
                        current_cell = Some(CellAccumulator::from_event(&event)?);
                    }
                }
                b"f" if current_cell.is_some()
                    && matches!(cell_content, CellContentMode::ValuesAndFormulas) =>
                {
                    capture_formula = true;
                }
                b"v" if current_cell.is_some() => {
                    capture_value = true;
                }
                b"is" if current_cell.is_some() => {
                    current_inline.clear();
                }
                b"t" if current_cell.is_some() && (current_inline.is_empty() || capture_inline) => {
                    capture_inline = true;
                }
                _ => {}
            },
            Ok(Event::End(event)) => match event.name().as_ref() {
                b"sheetData" => in_sheet_data = false,
                b"row" if current_row_index.is_some() && rows.len() < max_rows => {
                    rows.push(SheetPreviewRow {
                        row_index: current_row_index.take().unwrap_or_default(),
                        cells: std::mem::take(&mut current_cells),
                    });
                }
                b"c" if rows.len() < max_rows => {
                    if let Some(cell) = current_cell.take() {
                        current_cells.push(cell.finish(
                            shared_strings,
                            current_value.as_str(),
                            current_inline.as_str(),
                            current_formula.as_str(),
                            cell_content,
                        ));
                    }
                    current_formula.clear();
                    current_value.clear();
                    current_inline.clear();
                    capture_formula = false;
                    capture_value = false;
                    capture_inline = false;
                }
                b"f" => capture_formula = false,
                b"v" => capture_value = false,
                b"t" => capture_inline = false,
                _ => {}
            },
            Ok(Event::Text(text)) => {
                let decoded = text.decode().map_err(|err| {
                    ExcelInspectionError::Message(format!(
                        "failed to decode worksheet preview text: {err}"
                    ))
                })?;
                append_captured_text(
                    decoded.as_ref(),
                    TextCapture {
                        capture_formula,
                        capture_value,
                        capture_inline,
                    },
                    CapturedTextTargets {
                        formula: &mut current_formula,
                        value: &mut current_value,
                        inline: &mut current_inline,
                    },
                );
            }
            Ok(Event::GeneralRef(reference)) => {
                let decoded = if let Some(value) = reference.resolve_char_ref().map_err(|err| {
                    ExcelInspectionError::Message(format!(
                        "failed to decode worksheet preview entity reference: {err}"
                    ))
                })? {
                    value.to_string()
                } else {
                    let entity = reference.decode().map_err(|err| {
                        ExcelInspectionError::Message(format!(
                            "failed to decode worksheet preview entity reference: {err}"
                        ))
                    })?;
                    resolve_predefined_entity(entity.as_ref())
                        .map(str::to_string)
                        .unwrap_or_else(|| format!("&{entity};"))
                };
                append_captured_text(
                    decoded.as_str(),
                    TextCapture {
                        capture_formula,
                        capture_value,
                        capture_inline,
                    },
                    CapturedTextTargets {
                        formula: &mut current_formula,
                        value: &mut current_value,
                        inline: &mut current_inline,
                    },
                );
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse worksheet preview: {err}"
                )));
            }
        }
        buf.clear();
    }

    Ok((rows, truncated))
}

fn parse_sheet_cell_values(
    worksheet_xml: &str,
    shared_strings: &[String],
) -> Result<HashMap<String, String>, ExcelInspectionError> {
    let mut reader = Reader::from_str(worksheet_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut values = HashMap::new();

    let mut current_cell = None;
    let mut current_value = String::new();
    let mut current_inline = String::new();
    let mut ignored_formula = String::new();
    let mut capture_value = false;
    let mut capture_inline = false;
    let mut in_sheet_data = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => match event.name().as_ref() {
                b"sheetData" => in_sheet_data = true,
                b"c" if in_sheet_data => {
                    current_value.clear();
                    current_inline.clear();
                    current_cell = Some(CellAccumulator::from_event(&event)?);
                }
                b"v" if current_cell.is_some() => capture_value = true,
                b"is" if current_cell.is_some() => current_inline.clear(),
                b"t" if current_cell.is_some() && (current_inline.is_empty() || capture_inline) => {
                    capture_inline = true;
                }
                _ => {}
            },
            Ok(Event::End(event)) => match event.name().as_ref() {
                b"sheetData" => in_sheet_data = false,
                b"c" => {
                    if let Some(cell) = current_cell.take()
                        && let Some(value) = resolve_cell_value(
                            cell.cell_type.as_deref(),
                            current_value.as_str(),
                            current_inline.as_str(),
                            shared_strings,
                        )
                    {
                        values.insert(cell.reference, value);
                    }
                    current_value.clear();
                    current_inline.clear();
                    capture_value = false;
                    capture_inline = false;
                }
                b"v" => capture_value = false,
                b"t" => capture_inline = false,
                _ => {}
            },
            Ok(Event::Text(text)) => {
                let decoded = text.decode().map_err(|err| {
                    ExcelInspectionError::Message(format!(
                        "failed to decode worksheet preview text: {err}"
                    ))
                })?;
                append_captured_text(
                    decoded.as_ref(),
                    TextCapture {
                        capture_formula: false,
                        capture_value,
                        capture_inline,
                    },
                    CapturedTextTargets {
                        formula: &mut ignored_formula,
                        value: &mut current_value,
                        inline: &mut current_inline,
                    },
                );
            }
            Ok(Event::GeneralRef(reference)) => {
                let decoded = decode_general_ref(&reference)?;
                append_captured_text(
                    decoded.as_str(),
                    TextCapture {
                        capture_formula: false,
                        capture_value,
                        capture_inline,
                    },
                    CapturedTextTargets {
                        formula: &mut ignored_formula,
                        value: &mut current_value,
                        inline: &mut current_inline,
                    },
                );
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse worksheet values: {err}"
                )));
            }
        }
        buf.clear();
    }

    Ok(values)
}

fn parse_data_validations(
    worksheet_xml: &str,
    cell_values: &HashMap<String, String>,
    sheet_name: &str,
) -> Result<(Vec<SheetDataValidationSummary>, Vec<String>), ExcelInspectionError> {
    let mut reader = Reader::from_str(worksheet_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut summaries = Vec::new();
    let mut warnings = Vec::new();
    let mut remaining_resolved_values = MAX_DATA_VALIDATION_TOTAL_RESOLVED_VALUES;

    let mut current = None;
    let mut capture_formula1 = false;
    let mut capture_formula2 = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => match event.name().as_ref() {
                b"dataValidation" => {
                    if summaries.len() >= MAX_DATA_VALIDATION_SUMMARIES {
                        current = None;
                        warnings.push(format!(
                            "data validation summaries truncated to {MAX_DATA_VALIDATION_SUMMARIES}"
                        ));
                    } else {
                        current = Some(DataValidationAccumulator::from_event(&event)?);
                    }
                }
                b"formula1" if current.is_some() => capture_formula1 = true,
                b"formula2" if current.is_some() => capture_formula2 = true,
                _ => {}
            },
            Ok(Event::Empty(event)) if event.name().as_ref() == b"dataValidation" => {
                if summaries.len() >= MAX_DATA_VALIDATION_SUMMARIES {
                    warnings.push(format!(
                        "data validation summaries truncated to {MAX_DATA_VALIDATION_SUMMARIES}"
                    ));
                } else {
                    let accumulator = DataValidationAccumulator::from_event(&event)?;
                    summaries.push(accumulator.finish(
                        cell_values,
                        sheet_name,
                        &mut remaining_resolved_values,
                        &mut warnings,
                    ));
                }
            }
            Ok(Event::End(event)) => match event.name().as_ref() {
                b"dataValidation" => {
                    if let Some(accumulator) = current.take() {
                        summaries.push(accumulator.finish(
                            cell_values,
                            sheet_name,
                            &mut remaining_resolved_values,
                            &mut warnings,
                        ));
                    }
                    capture_formula1 = false;
                    capture_formula2 = false;
                }
                b"formula1" => capture_formula1 = false,
                b"formula2" => capture_formula2 = false,
                _ => {}
            },
            Ok(Event::Text(text)) => {
                if let Some(accumulator) = current.as_mut() {
                    let decoded = text.decode().map_err(|err| {
                        ExcelInspectionError::Message(format!(
                            "failed to decode data validation text: {err}"
                        ))
                    })?;
                    if capture_formula1 {
                        accumulator.formula1.push_str(decoded.as_ref());
                    } else if capture_formula2 {
                        accumulator.formula2.push_str(decoded.as_ref());
                    }
                }
            }
            Ok(Event::GeneralRef(reference)) => {
                if let Some(accumulator) = current.as_mut() {
                    let decoded = decode_general_ref(&reference)?;
                    if capture_formula1 {
                        accumulator.formula1.push_str(decoded.as_str());
                    } else if capture_formula2 {
                        accumulator.formula2.push_str(decoded.as_str());
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse data validations: {err}"
                )));
            }
        }
        buf.clear();
    }

    Ok((summaries, warnings))
}

struct TextCapture {
    capture_formula: bool,
    capture_value: bool,
    capture_inline: bool,
}

struct CapturedTextTargets<'a> {
    formula: &'a mut String,
    value: &'a mut String,
    inline: &'a mut String,
}

fn append_captured_text(text: &str, capture: TextCapture, targets: CapturedTextTargets<'_>) {
    if capture.capture_formula {
        targets.formula.push_str(text);
    } else if capture.capture_value {
        targets.value.push_str(text);
    } else if capture.capture_inline {
        targets.inline.push_str(text);
    }
}

pub(crate) fn decode_general_ref(
    reference: &quick_xml::events::BytesRef<'_>,
) -> Result<String, ExcelInspectionError> {
    if let Some(value) = reference.resolve_char_ref().map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to decode worksheet preview entity reference: {err}"
        ))
    })? {
        return Ok(value.to_string());
    }

    let entity = reference.decode().map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to decode worksheet preview entity reference: {err}"
        ))
    })?;
    Ok(resolve_predefined_entity(entity.as_ref())
        .map(str::to_string)
        .unwrap_or_else(|| format!("&{entity};")))
}

fn row_index(event: &BytesStart<'_>) -> Result<u32, ExcelInspectionError> {
    attr_value(event, b"r")?
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(|| {
            ExcelInspectionError::Message(
                "failed to parse worksheet row index for excel.read_sheet_preview".to_string(),
            )
        })
}

pub(crate) fn attr_value(
    event: &BytesStart<'_>,
    key: &[u8],
) -> Result<Option<String>, ExcelInspectionError> {
    for attr in event.attributes().with_checks(false) {
        let attr = attr.map_err(|err| {
            ExcelInspectionError::Message(format!("failed to parse worksheet attribute: {err}"))
        })?;
        if attr.key.as_ref() == key {
            return Ok(Some(
                String::from_utf8_lossy(attr.value.as_ref()).into_owned(),
            ));
        }
    }
    Ok(None)
}

pub(crate) fn bounded_text(value: &str, max_chars: usize) -> String {
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

struct DataValidationAccumulator {
    ranges: Vec<String>,
    validation_type: String,
    operator: Option<String>,
    allow_blank: Option<bool>,
    dropdown_visible: Option<bool>,
    error_style: Option<String>,
    show_error_message: Option<bool>,
    formula1: String,
    formula2: String,
}

impl DataValidationAccumulator {
    fn from_event(event: &BytesStart<'_>) -> Result<Self, ExcelInspectionError> {
        let ranges = attr_value(event, b"sqref")?
            .unwrap_or_default()
            .split_whitespace()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        Ok(Self {
            ranges,
            validation_type: attr_value(event, b"type")?.unwrap_or_else(|| "any".to_string()),
            operator: attr_value(event, b"operator")?,
            allow_blank: attr_value(event, b"allowBlank")?.and_then(|value| parse_bool(&value)),
            dropdown_visible: attr_value(event, b"showDropDown")?
                .and_then(|value| parse_bool(&value))
                .map(|hide_dropdown| !hide_dropdown),
            error_style: attr_value(event, b"errorStyle")?,
            show_error_message: attr_value(event, b"showErrorMessage")?
                .and_then(|value| parse_bool(&value)),
            formula1: String::new(),
            formula2: String::new(),
        })
    }

    fn finish(
        self,
        cell_values: &HashMap<String, String>,
        sheet_name: &str,
        remaining_resolved_values: &mut usize,
        warnings: &mut Vec<String>,
    ) -> SheetDataValidationSummary {
        let formula1 = (!self.formula1.is_empty())
            .then(|| bounded_text(self.formula1.as_str(), MAX_DATA_VALIDATION_FORMULA_CHARS));
        let formula2 = (!self.formula2.is_empty())
            .then(|| bounded_text(self.formula2.as_str(), MAX_DATA_VALIDATION_FORMULA_CHARS));
        let (resolved_values_source, resolved_values_sample, resolved_values_truncated) =
            resolve_validation_values(
                formula1.as_deref(),
                cell_values,
                sheet_name,
                remaining_resolved_values,
            );
        if resolved_values_source == "unresolved" {
            warnings.push(format!(
                "data validation formula could not be resolved: {}",
                formula1.as_deref().unwrap_or_default()
            ));
        }
        if resolved_values_truncated {
            warnings.push(format!(
                "data validation resolved values truncated for formula: {}",
                formula1.as_deref().unwrap_or_default()
            ));
        }

        SheetDataValidationSummary {
            ranges_sample: self
                .ranges
                .iter()
                .take(MAX_DATA_VALIDATION_RANGE_SAMPLE)
                .cloned()
                .collect(),
            range_count: self.ranges.len(),
            validation_type: self.validation_type,
            operator: self.operator,
            allow_blank: self.allow_blank,
            dropdown_visible: self.dropdown_visible,
            error_style: self.error_style,
            show_error_message: self.show_error_message,
            formula1,
            formula2,
            resolved_values_source,
            resolved_values_sample,
            resolved_values_truncated,
        }
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "1" | "true" | "TRUE" => Some(true),
        "0" | "false" | "FALSE" => Some(false),
        _ => None,
    }
}

fn resolve_validation_values(
    formula: Option<&str>,
    cell_values: &HashMap<String, String>,
    sheet_name: &str,
    remaining_resolved_values: &mut usize,
) -> (String, Vec<String>, bool) {
    let Some(formula) = formula else {
        return ("none".to_string(), Vec::new(), false);
    };
    if formula.starts_with('"') && formula.ends_with('"') && formula.len() >= 2 {
        let values = formula[1..formula.len() - 1]
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        return capped_values("inline_list", values, remaining_resolved_values);
    }

    if let Some(cells) = simple_same_sheet_range_cells(formula, sheet_name) {
        let values = cells
            .into_iter()
            .filter_map(|cell| cell_values.get(&cell).cloned())
            .collect::<Vec<_>>();
        return capped_values("same_sheet_range", values, remaining_resolved_values);
    }

    ("unresolved".to_string(), Vec::new(), false)
}

fn capped_values(
    source: &str,
    values: Vec<String>,
    remaining_resolved_values: &mut usize,
) -> (String, Vec<String>, bool) {
    let limit = MAX_DATA_VALIDATION_RESOLVED_VALUES.min(*remaining_resolved_values);
    let truncated = values.len() > limit;
    let sample = values.into_iter().take(limit).collect::<Vec<_>>();
    *remaining_resolved_values = remaining_resolved_values.saturating_sub(sample.len());
    (source.to_string(), sample, truncated)
}

fn simple_same_sheet_range_cells(formula: &str, sheet_name: &str) -> Option<Vec<String>> {
    let formula = formula.trim_start_matches('=');
    let range = if let Some((sheet, range)) = formula.rsplit_once('!') {
        let sheet = sheet.trim_matches('\'');
        if sheet != sheet_name {
            return None;
        }
        range
    } else {
        formula
    };
    let (start, end) = range.split_once(':').unwrap_or((range, range));
    let (start_col, start_row) = parse_cell_ref(start)?;
    let (end_col, end_row) = parse_cell_ref(end)?;
    let min_col = start_col.min(end_col);
    let max_col = start_col.max(end_col);
    let min_row = start_row.min(end_row);
    let max_row = start_row.max(end_row);
    let mut cells = Vec::new();
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            cells.push(format!("{}{}", column_name(col), row));
        }
    }
    Some(cells)
}

fn parse_cell_ref(reference: &str) -> Option<(u32, u32)> {
    let reference = reference.replace('$', "");
    let split = reference
        .char_indices()
        .find_map(|(index, ch)| ch.is_ascii_digit().then_some(index))?;
    let (column, row) = reference.split_at(split);
    if column.is_empty() || row.is_empty() {
        return None;
    }
    let mut column_index = 0u32;
    for ch in column.chars() {
        if !ch.is_ascii_alphabetic() {
            return None;
        }
        column_index = column_index * 26 + u32::from(ch.to_ascii_uppercase() as u8 - b'A' + 1);
    }
    Some((column_index, row.parse().ok()?))
}

fn column_name(mut index: u32) -> String {
    let mut name = String::new();
    while index > 0 {
        index -= 1;
        name.insert(0, char::from(b'A' + (index % 26) as u8));
        index /= 26;
    }
    name
}

struct CellAccumulator {
    reference: String,
    cell_type: Option<String>,
}

impl CellAccumulator {
    fn from_event(event: &BytesStart<'_>) -> Result<Self, ExcelInspectionError> {
        Ok(Self {
            reference: attr_value(event, b"r")?.unwrap_or_default(),
            cell_type: attr_value(event, b"t")?,
        })
    }

    fn finish(
        self,
        shared_strings: &[String],
        raw_value: &str,
        inline_value: &str,
        formula: &str,
        cell_content: CellContentMode,
    ) -> SheetPreviewCell {
        let value = resolve_cell_value(
            self.cell_type.as_deref(),
            raw_value,
            inline_value,
            shared_strings,
        );
        let formula = matches!(cell_content, CellContentMode::ValuesAndFormulas)
            .then(|| bounded_text(formula, MAX_PREVIEW_CELL_TEXT_CHARS))
            .filter(|formula| !formula.is_empty());

        SheetPreviewCell {
            reference: bounded_text(self.reference.as_str(), 32),
            value,
            formula,
        }
    }
}

pub(crate) fn resolve_cell_value(
    cell_type: Option<&str>,
    raw_value: &str,
    inline_value: &str,
    shared_strings: &[String],
) -> Option<String> {
    let value = match cell_type {
        Some("s") => raw_value
            .parse::<usize>()
            .ok()
            .and_then(|index| shared_strings.get(index).cloned())
            .unwrap_or_else(|| raw_value.to_string()),
        Some("inlineStr") => inline_value.to_string(),
        _ => raw_value.to_string(),
    };
    (!value.is_empty()).then(|| bounded_text(value.as_str(), MAX_PREVIEW_CELL_TEXT_CHARS))
}
