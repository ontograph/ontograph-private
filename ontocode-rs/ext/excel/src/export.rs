use std::fs::File;
use std::path::Path;

use quick_xml::Reader;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use zip::ZipArchive;

use crate::backend::ExcelInspectionError;
use crate::backend::inspect_workbook_with_display_path;
use crate::preview::attr_value;
use crate::preview::bounded_text;
use crate::preview::read_shared_strings;
use crate::preview::read_xml_entry;
use crate::preview::resolve_cell_value;
use crate::preview::select_sheet;
use crate::tool::ExportSheetToCsvResult;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;
use crate::tool::WorkbookFormat;

const MAX_WORKSHEET_XML_BYTES: usize = 8 * 1024 * 1024;
const MAX_EXPORT_ROWS: usize = 10_000;
const MAX_EXPORT_COLUMNS: usize = 256;
const MAX_EXPORT_CELL_TEXT_CHARS: usize = 4096;
const MAX_EXPORT_CSV_BYTES: usize = 8 * 1024 * 1024;

pub(crate) fn export_sheet_to_csv_with_display_path(
    path: &Path,
    display_path: &Path,
    sheet: &SheetSelector,
    output_path: &Path,
    output_display_path: &Path,
) -> Result<ExportSheetToCsvResult, ExcelInspectionError> {
    let workbook = inspect_workbook_with_display_path(path, display_path)?;
    if !matches!(workbook.format, WorkbookFormat::Xlsx | WorkbookFormat::Xlsm) {
        return Err(ExcelInspectionError::Message(
            "excel.export_sheet_to_csv supports only .xlsx and .xlsm in this stage".to_string(),
        ));
    }

    let selected_sheet = select_sheet(&workbook.sheets, sheet)?;
    let sheet_name = selected_sheet.name.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.export_sheet_to_csv could not resolve a sheet name for the selected sheet"
                .to_string(),
        )
    })?;
    let sheet_part_path = selected_sheet.part_path.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.export_sheet_to_csv could not resolve a worksheet part path for the selected sheet"
                .to_string(),
        )
    })?;

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
    let export = build_csv_export(&worksheet_xml, &shared_strings)?;

    if let Some(parent) = output_path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(|err| {
            ExcelInspectionError::Message(format!(
                "failed to create csv output directory {}: {err}",
                parent.display()
            ))
        })?;
    }
    std::fs::write(output_path, export.csv.as_bytes()).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to write csv output {}: {err}",
            output_path.display()
        ))
    })?;

    Ok(ExportSheetToCsvResult {
        path: display_path.display().to_string(),
        sheet: SheetPreview {
            name: sheet_name,
            sheet_id: selected_sheet.sheet_id,
            part_path: sheet_part_path,
        },
        output_csv_path: output_display_path.display().to_string(),
        row_count: export.row_count,
        column_count: export.column_count,
        truncated: export.truncated,
        warnings: export.warnings,
    })
}

fn build_csv_export(
    worksheet_xml: &str,
    shared_strings: &[String],
) -> Result<CsvExport, ExcelInspectionError> {
    let mut reader = Reader::from_str(worksheet_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut csv = String::new();
    let mut row_count = 0usize;
    let mut column_count = 0usize;
    let mut truncated = false;
    let mut warnings = Vec::new();

    let mut in_sheet_data = false;
    let mut current_row = Vec::<(usize, String)>::new();
    let mut current_cell = None::<CellAccumulator>;
    let mut current_value = String::new();
    let mut current_inline = String::new();
    let mut capture_value = false;
    let mut capture_inline = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => match event.name().as_ref() {
                b"sheetData" => in_sheet_data = true,
                b"row" if in_sheet_data => {
                    current_row.clear();
                }
                b"c" if in_sheet_data => {
                    if current_row.len() >= MAX_EXPORT_COLUMNS {
                        current_cell = None;
                        truncated = true;
                    } else {
                        current_value.clear();
                        current_inline.clear();
                        current_cell = Some(CellAccumulator::from_event(&event)?);
                    }
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
                        && let Some(column_index) = cell.column_index()
                    {
                        let Some(value) = resolve_cell_value(
                            cell.cell_type.as_deref(),
                            current_value.as_str(),
                            current_inline.as_str(),
                            shared_strings,
                        ) else {
                            current_value.clear();
                            current_inline.clear();
                            capture_value = false;
                            capture_inline = false;
                            continue;
                        };
                        if column_index < MAX_EXPORT_COLUMNS {
                            current_row.push((
                                column_index,
                                bounded_text(value.as_str(), MAX_EXPORT_CELL_TEXT_CHARS),
                            ));
                        } else {
                            truncated = true;
                        }
                    }
                    current_value.clear();
                    current_inline.clear();
                    capture_value = false;
                    capture_inline = false;
                }
                b"row" if in_sheet_data => {
                    if row_count >= MAX_EXPORT_ROWS {
                        truncated = true;
                    } else {
                        current_row.sort_by_key(|(index, _)| *index);
                        let line = csv_line(current_row.as_slice(), &mut column_count);
                        if csv.len() + line.len() > MAX_EXPORT_CSV_BYTES {
                            truncated = true;
                        } else {
                            csv.push_str(line.as_str());
                            row_count += 1;
                        }
                    }
                    current_row.clear();
                }
                b"v" => capture_value = false,
                b"t" => capture_inline = false,
                _ => {}
            },
            Ok(Event::Text(text)) => {
                let decoded = text.decode().map_err(|err| {
                    ExcelInspectionError::Message(format!(
                        "failed to decode worksheet export text: {err}"
                    ))
                })?;
                if capture_value {
                    current_value.push_str(decoded.as_ref());
                } else if capture_inline {
                    current_inline.push_str(decoded.as_ref());
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse worksheet export: {err}"
                )));
            }
        }
        if truncated {
            break;
        }
        buf.clear();
    }

    if row_count >= MAX_EXPORT_ROWS {
        warnings.push(format!(
            "sheet export truncated to {MAX_EXPORT_ROWS} rows and {MAX_EXPORT_COLUMNS} columns"
        ));
    } else if truncated {
        warnings.push(format!(
            "sheet export truncated to {MAX_EXPORT_ROWS} rows, {MAX_EXPORT_COLUMNS} columns, or {MAX_EXPORT_CSV_BYTES} bytes"
        ));
    }

    Ok(CsvExport {
        csv,
        row_count,
        column_count,
        truncated,
        warnings,
    })
}

fn csv_line(cells: &[(usize, String)], column_count: &mut usize) -> String {
    let width = cells
        .last()
        .map(|(index, _)| index.saturating_add(1))
        .unwrap_or_default();
    *column_count = (*column_count).max(width);

    let mut values = vec![String::new(); width];
    for (index, value) in cells {
        values[*index] = escape_csv(value);
    }
    format!("{}\n", values.join(","))
}

fn escape_csv(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
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

    fn column_index(&self) -> Option<usize> {
        column_index_from_reference(self.reference.as_str())
    }
}

fn column_index_from_reference(reference: &str) -> Option<usize> {
    let mut index = 0usize;
    let mut found_letter = false;
    for ch in reference.chars() {
        if ch.is_ascii_alphabetic() {
            found_letter = true;
            let upper = ch.to_ascii_uppercase();
            index = index
                .checked_mul(26)?
                .checked_add(usize::from((upper as u8).checked_sub(b'A')?) + 1)?;
        } else {
            break;
        }
    }
    found_letter.then_some(index.saturating_sub(1))
}

struct CsvExport {
    csv: String,
    row_count: usize,
    column_count: usize,
    truncated: bool,
    warnings: Vec<String>,
}
