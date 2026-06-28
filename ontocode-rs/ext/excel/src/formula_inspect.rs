use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use calamine::Reader as CalamineReader;
use calamine::open_workbook_auto;
use quick_xml::Reader;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use zip::ZipArchive;

use crate::backend::ExcelInspectionError;
use crate::backend::inspect_workbook_with_display_path;
use crate::formula_ast::parse_formula_ast;
use crate::formula_sql::plan_formula_sql_preview;
use crate::preview::attr_value;
use crate::preview::bounded_text;
use crate::preview::calamine_cell_value;
use crate::preview::decode_general_ref;
use crate::preview::read_shared_strings;
use crate::preview::read_xml_entry;
use crate::preview::resolve_cell_value;
use crate::preview::select_sheet;
use crate::preview::select_xlsb_sheet;
use crate::tool::DefinedNameSummary;
use crate::tool::InspectSheetFormulasResult;
use crate::tool::SheetFormulaSummary;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;
use crate::tool::WorkbookFormat;

const DEFAULT_MAX_FORMULAS: usize = 128;
const MAX_FORMULAS: usize = 512;
const MAX_WORKBOOK_XML_BYTES: usize = 2 * 1024 * 1024;
const MAX_WORKSHEET_XML_BYTES: usize = 2 * 1024 * 1024;
const MAX_STYLES_XML_BYTES: usize = 2 * 1024 * 1024;
const MAX_FORMULA_TEXT_CHARS: usize = 512;
pub(crate) const MAX_DEFINED_NAMES: usize = 64;
pub(crate) const MAX_DEFINED_NAME_CHARS: usize = 512;

pub(crate) fn inspect_sheet_formulas_with_display_path(
    path: &Path,
    display_path: &Path,
    sheet: &SheetSelector,
    max_formulas: Option<usize>,
) -> Result<InspectSheetFormulasResult, ExcelInspectionError> {
    let workbook = inspect_workbook_with_display_path(path, display_path)?;
    match workbook.format {
        WorkbookFormat::Xlsx | WorkbookFormat::Xlsm => {
            inspect_openxml_sheet_formulas(path, display_path, sheet, max_formulas, &workbook)
        }
        WorkbookFormat::Xlsb => {
            inspect_xlsb_sheet_formulas(path, display_path, sheet, max_formulas, &workbook)
        }
        WorkbookFormat::Unknown => Err(ExcelInspectionError::Message(
            "excel.inspect_sheet_formulas supports only .xlsx, .xlsm, or .xlsb in this stage"
                .to_string(),
        )),
    }
}

fn inspect_openxml_sheet_formulas(
    path: &Path,
    display_path: &Path,
    sheet: &SheetSelector,
    max_formulas: Option<usize>,
    workbook: &crate::tool::InspectWorkbookResult,
) -> Result<InspectSheetFormulasResult, ExcelInspectionError> {
    let selected_sheet = select_sheet(&workbook.sheets, sheet)?;
    let sheet_name = selected_sheet.name.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.inspect_sheet_formulas could not resolve a sheet name for the selected sheet"
                .to_string(),
        )
    })?;
    let sheet_part_path = selected_sheet.part_path.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.inspect_sheet_formulas could not resolve a worksheet part path for the selected sheet"
                .to_string(),
        )
    })?;
    let max_formulas_applied = max_formulas
        .unwrap_or(DEFAULT_MAX_FORMULAS)
        .min(MAX_FORMULAS);

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
    let workbook_xml = read_xml_entry(&mut archive, "xl/workbook.xml", MAX_WORKBOOK_XML_BYTES)?;
    let styles = read_styles(&mut archive)?;
    let worksheet_xml = read_xml_entry(&mut archive, &sheet_part_path, MAX_WORKSHEET_XML_BYTES)?;
    let (mut formulas, truncated) = parse_formulas(
        &worksheet_xml,
        &shared_strings,
        &styles,
        max_formulas_applied,
    )?;
    let context = parse_workbook_context(&workbook_xml)?;
    for formula in &mut formulas {
        formula.sql_preview = plan_formula_sql_preview(
            sheet_name.as_str(),
            formula,
            &context.defined_names,
            workbook.markers.has_external_links,
        );
    }

    let mut warnings = Vec::new();
    if max_formulas.unwrap_or(DEFAULT_MAX_FORMULAS) > MAX_FORMULAS {
        warnings.push(format!(
            "max_formulas capped to {MAX_FORMULAS} for excel.inspect_sheet_formulas"
        ));
    }
    if truncated {
        warnings.push(format!(
            "formula inventory truncated to {max_formulas_applied} formulas"
        ));
    }

    Ok(InspectSheetFormulasResult {
        path: display_path.display().to_string(),
        sheet: SheetPreview {
            name: sheet_name,
            sheet_id: selected_sheet.sheet_id,
            part_path: sheet_part_path,
        },
        max_formulas_applied,
        formulas,
        calculation_mode: context.calculation_mode,
        full_calc_on_load: context.full_calc_on_load,
        force_full_calc: context.force_full_calc,
        defined_names: context.defined_names,
        defined_names_sample: context.defined_names_sample,
        has_external_links: workbook.markers.has_external_links,
        truncated,
        warnings,
    })
}

fn inspect_xlsb_sheet_formulas(
    path: &Path,
    display_path: &Path,
    sheet: &SheetSelector,
    max_formulas: Option<usize>,
    workbook: &crate::tool::InspectWorkbookResult,
) -> Result<InspectSheetFormulasResult, ExcelInspectionError> {
    let max_formulas_applied = max_formulas
        .unwrap_or(DEFAULT_MAX_FORMULAS)
        .min(MAX_FORMULAS);
    let mut xlsb = open_workbook_auto(path).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to open .xlsb workbook {} for excel.inspect_sheet_formulas: {err}",
            path.display()
        ))
    })?;
    let (sheet_index, sheet_name) =
        select_xlsb_sheet(&xlsb, sheet, "excel.inspect_sheet_formulas")?;
    let sheet_part_path = workbook
        .sheets
        .get(sheet_index)
        .and_then(|summary| summary.part_path.clone())
        .unwrap_or_default();
    let worksheet = xlsb.worksheet_range(&sheet_name).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to read .xlsb worksheet values for {sheet_name}: {err}"
        ))
    })?;
    let formula_range = xlsb.worksheet_formula(&sheet_name).map_err(|err| {
        ExcelInspectionError::Message(format!(
            "failed to read .xlsb worksheet formulas for {sheet_name}: {err}"
        ))
    })?;
    let (mut formulas, truncated) =
        build_xlsb_formula_inventory(&worksheet, &formula_range, max_formulas_applied);
    let defined_names = xlsb
        .defined_names()
        .iter()
        .take(MAX_DEFINED_NAMES)
        .map(|(name, target)| DefinedNameSummary {
            name: bounded_text(name, 128),
            sheet_scope: None,
            local_sheet_id: None,
            hidden: None,
            target: bounded_text(target, MAX_DEFINED_NAME_CHARS),
            truncated: target.chars().count() > MAX_DEFINED_NAME_CHARS,
        })
        .collect::<Vec<_>>();
    let defined_names_sample = xlsb
        .defined_names()
        .iter()
        .take(MAX_DEFINED_NAMES)
        .map(|(name, target)| bounded_text(&format!("{name}={target}"), MAX_DEFINED_NAME_CHARS))
        .collect::<Vec<_>>();

    for formula in &mut formulas {
        formula.sql_preview = plan_formula_sql_preview(
            sheet_name.as_str(),
            formula,
            &defined_names,
            workbook.markers.has_external_links,
        );
    }

    let mut warnings = Vec::new();
    if max_formulas.unwrap_or(DEFAULT_MAX_FORMULAS) > MAX_FORMULAS {
        warnings.push(format!(
            "max_formulas capped to {MAX_FORMULAS} for excel.inspect_sheet_formulas"
        ));
    }
    if truncated {
        warnings.push(format!(
            "formula inventory truncated to {max_formulas_applied} formulas"
        ));
    }
    warnings.push(
        "excel.inspect_sheet_formulas does not decode .xlsb calculation, style, or shared-formula metadata in this stage"
            .to_string(),
    );
    if sheet_part_path.is_empty() {
        warnings.push(
            "excel.inspect_sheet_formulas could not resolve an .xlsb worksheet part path in this stage"
                .to_string(),
        );
    }

    Ok(InspectSheetFormulasResult {
        path: display_path.display().to_string(),
        sheet: SheetPreview {
            name: sheet_name,
            sheet_id: workbook
                .sheets
                .get(sheet_index)
                .and_then(|summary| summary.sheet_id),
            part_path: sheet_part_path,
        },
        max_formulas_applied,
        formulas,
        calculation_mode: None,
        full_calc_on_load: None,
        force_full_calc: None,
        defined_names,
        defined_names_sample,
        has_external_links: workbook.markers.has_external_links,
        truncated,
        warnings,
    })
}

fn build_xlsb_formula_inventory(
    worksheet: &calamine::Range<calamine::Data>,
    formulas: &calamine::Range<String>,
    max_formulas: usize,
) -> (Vec<SheetFormulaSummary>, bool) {
    let mut summaries = Vec::new();
    let mut truncated = false;
    let formula_start = formulas.start().unwrap_or((0, 0));

    for (relative_row, relative_col, formula) in formulas.used_cells() {
        if formula.is_empty() {
            continue;
        }
        if summaries.len() >= max_formulas {
            truncated = true;
            break;
        }
        let absolute_row = formula_start.0 + relative_row as u32;
        let absolute_col = formula_start.1 + relative_col as u32;
        let formula_was_truncated = formula.chars().count() > MAX_FORMULA_TEXT_CHARS;
        let bounded_formula = bounded_text(formula, MAX_FORMULA_TEXT_CHARS);
        summaries.push(SheetFormulaSummary {
            reference: crate::preview::zero_based_cell_reference(absolute_row, absolute_col),
            formula: bounded_formula.clone(),
            cached_value: worksheet
                .get_value((absolute_row, absolute_col))
                .and_then(calamine_cell_value),
            parse: parse_formula_ast(&bounded_formula, formula_was_truncated),
            sql_preview: Default::default(),
            warnings: formula_warnings(&bounded_formula),
            formula_type: None,
            shared_index: None,
            shared_range: None,
            style_index: None,
            number_format_id: None,
            number_format_code: None,
        });
    }

    (summaries, truncated)
}

fn read_styles(archive: &mut ZipArchive<File>) -> Result<StyleLookup, ExcelInspectionError> {
    let Ok(xml) = read_xml_entry(archive, "xl/styles.xml", MAX_STYLES_XML_BYTES) else {
        return Ok(StyleLookup::default());
    };
    parse_styles(&xml)
}

#[derive(Default)]
struct StyleLookup {
    style_num_fmt_ids: Vec<u32>,
    format_codes: HashMap<u32, String>,
}

impl StyleLookup {
    fn number_format(&self, style_index: Option<u32>) -> (Option<u32>, Option<String>) {
        let Some(style_index) = style_index else {
            return (None, None);
        };
        let Some(number_format_id) = usize::try_from(style_index)
            .ok()
            .and_then(|index| self.style_num_fmt_ids.get(index))
            .copied()
        else {
            return (None, None);
        };
        (
            Some(number_format_id),
            self.format_codes.get(&number_format_id).cloned(),
        )
    }
}

fn parse_styles(xml: &str) -> Result<StyleLookup, ExcelInspectionError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut lookup = StyleLookup::default();
    let mut in_cell_xfs = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => match event.name().as_ref() {
                b"cellXfs" => in_cell_xfs = true,
                b"numFmt" => push_number_format(&event, &mut lookup)?,
                b"xf" if in_cell_xfs => push_style_format(&event, &mut lookup)?,
                _ => {}
            },
            Ok(Event::Empty(event)) => match event.name().as_ref() {
                b"numFmt" => push_number_format(&event, &mut lookup)?,
                b"xf" if in_cell_xfs => push_style_format(&event, &mut lookup)?,
                _ => {}
            },
            Ok(Event::End(event)) if event.name().as_ref() == b"cellXfs" => in_cell_xfs = false,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse workbook styles: {err}"
                )));
            }
        }
        buf.clear();
    }

    Ok(lookup)
}

fn push_number_format(
    event: &BytesStart<'_>,
    lookup: &mut StyleLookup,
) -> Result<(), ExcelInspectionError> {
    if let Some(id) = attr_value(event, b"numFmtId")?.and_then(|value| value.parse().ok())
        && let Some(code) = attr_value(event, b"formatCode")?
    {
        lookup.format_codes.insert(id, bounded_text(&code, 256));
    }
    Ok(())
}

fn push_style_format(
    event: &BytesStart<'_>,
    lookup: &mut StyleLookup,
) -> Result<(), ExcelInspectionError> {
    lookup.style_num_fmt_ids.push(
        attr_value(event, b"numFmtId")?
            .and_then(|value| value.parse().ok())
            .unwrap_or_default(),
    );
    Ok(())
}

pub(crate) struct WorkbookContext {
    pub(crate) calculation_mode: Option<String>,
    pub(crate) full_calc_on_load: Option<bool>,
    pub(crate) force_full_calc: Option<bool>,
    pub(crate) defined_name_count: usize,
    pub(crate) defined_names: Vec<DefinedNameSummary>,
    pub(crate) defined_names_sample: Vec<String>,
    pub(crate) truncated_defined_name_targets: usize,
    pub(crate) unresolved_sheet_scope_count: usize,
}

pub(crate) fn parse_workbook_context(xml: &str) -> Result<WorkbookContext, ExcelInspectionError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut context = WorkbookContext {
        calculation_mode: None,
        full_calc_on_load: None,
        force_full_calc: None,
        defined_name_count: 0,
        defined_names: Vec::new(),
        defined_names_sample: Vec::new(),
        truncated_defined_name_targets: 0,
        unresolved_sheet_scope_count: 0,
    };
    let mut sheet_names = Vec::new();
    let mut current_defined_name = None;
    let mut current_defined_name_local_sheet_id = None;
    let mut current_defined_name_hidden = None;
    let mut current_defined_name_text = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => match event.name().as_ref() {
                b"sheet" => push_sheet_name(&event, &mut sheet_names)?,
                b"calcPr" => read_calc_pr(&event, &mut context)?,
                b"definedName" => {
                    current_defined_name = attr_value(&event, b"name")?;
                    current_defined_name_local_sheet_id = attr_value(&event, b"localSheetId")?
                        .map(|value| {
                            value.parse().map_err(|err| {
                                ExcelInspectionError::Message(format!(
                                    "failed to parse workbook formula context: invalid definedName localSheetId `{value}`: {err}"
                                ))
                            })
                        })
                        .transpose()?;
                    current_defined_name_hidden =
                        attr_value(&event, b"hidden")?.and_then(|value| parse_bool(&value));
                    current_defined_name_text.clear();
                }
                _ => {}
            },
            Ok(Event::Empty(event)) if event.name().as_ref() == b"sheet" => {
                push_sheet_name(&event, &mut sheet_names)?;
            }
            Ok(Event::Empty(event)) if event.name().as_ref() == b"calcPr" => {
                read_calc_pr(&event, &mut context)?;
            }
            Ok(Event::Text(text)) if current_defined_name.is_some() => {
                current_defined_name_text.push_str(
                    text.decode()
                        .map_err(|err| {
                            ExcelInspectionError::Message(format!(
                                "failed to decode defined name text: {err}"
                            ))
                        })?
                        .as_ref(),
                );
            }
            Ok(Event::GeneralRef(reference)) if current_defined_name.is_some() => {
                current_defined_name_text.push_str(&decode_general_ref(&reference)?);
            }
            Ok(Event::End(event)) if event.name().as_ref() == b"definedName" => {
                context.defined_name_count += 1;
                let name = current_defined_name.take().unwrap_or_default();
                let target = bounded_text(&current_defined_name_text, MAX_DEFINED_NAME_CHARS);
                let local_sheet_id = current_defined_name_local_sheet_id.take();
                let sheet_scope = local_sheet_id
                    .and_then(|index: u32| usize::try_from(index).ok())
                    .and_then(|index| sheet_names.get(index).cloned());
                if local_sheet_id.is_some() && sheet_scope.is_none() {
                    context.unresolved_sheet_scope_count += 1;
                }
                let truncated = current_defined_name_text.chars().count() > MAX_DEFINED_NAME_CHARS;
                if truncated {
                    context.truncated_defined_name_targets += 1;
                }
                if context.defined_names.len() < MAX_DEFINED_NAMES {
                    context.defined_names.push(DefinedNameSummary {
                        name: bounded_text(&name, 128),
                        sheet_scope,
                        local_sheet_id,
                        hidden: current_defined_name_hidden.take(),
                        target: target.clone(),
                        truncated,
                    });
                    context.defined_names_sample.push(bounded_text(
                        &format!("{name}={target}"),
                        MAX_DEFINED_NAME_CHARS,
                    ));
                }
                current_defined_name_local_sheet_id = None;
                current_defined_name_hidden = None;
                current_defined_name_text.clear();
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse workbook formula context: {err}"
                )));
            }
        }
        buf.clear();
    }

    Ok(context)
}

fn push_sheet_name(
    event: &BytesStart<'_>,
    sheet_names: &mut Vec<String>,
) -> Result<(), ExcelInspectionError> {
    if let Some(name) = attr_value(event, b"name")? {
        sheet_names.push(bounded_text(&name, 128));
    }
    Ok(())
}

fn read_calc_pr(
    event: &BytesStart<'_>,
    context: &mut WorkbookContext,
) -> Result<(), ExcelInspectionError> {
    context.calculation_mode = attr_value(event, b"calcMode")?;
    context.full_calc_on_load =
        attr_value(event, b"fullCalcOnLoad")?.and_then(|value| parse_bool(&value));
    context.force_full_calc =
        attr_value(event, b"forceFullCalc")?.and_then(|value| parse_bool(&value));
    Ok(())
}

fn parse_bool(value: &str) -> Option<bool> {
    matches!(value, "1" | "true" | "TRUE")
        .then_some(true)
        .or_else(|| matches!(value, "0" | "false" | "FALSE").then_some(false))
}

fn parse_formulas(
    xml: &str,
    shared_strings: &[String],
    styles: &StyleLookup,
    max_formulas: usize,
) -> Result<(Vec<SheetFormulaSummary>, bool), ExcelInspectionError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut formulas = Vec::new();
    let mut truncated = false;
    let mut current = None;
    let mut capture_formula = false;
    let mut capture_value = false;
    let mut in_sheet_data = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => match event.name().as_ref() {
                b"sheetData" => in_sheet_data = true,
                b"c" if in_sheet_data => current = Some(FormulaAccumulator::from_event(&event)?),
                b"f" if current.is_some() => {
                    if let Some(accumulator) = current.as_mut() {
                        accumulator.formula_type = attr_value(&event, b"t")?;
                        accumulator.shared_index =
                            attr_value(&event, b"si")?.and_then(|value| value.parse::<u32>().ok());
                        accumulator.shared_range = attr_value(&event, b"ref")?;
                        accumulator.has_formula = true;
                    }
                    capture_formula = true;
                }
                b"v" if current.is_some() => capture_value = true,
                _ => {}
            },
            Ok(Event::Empty(event)) if event.name().as_ref() == b"f" && current.is_some() => {
                if let Some(accumulator) = current.as_mut() {
                    accumulator.formula_type = attr_value(&event, b"t")?;
                    accumulator.shared_index =
                        attr_value(&event, b"si")?.and_then(|value| value.parse::<u32>().ok());
                    accumulator.shared_range = attr_value(&event, b"ref")?;
                    accumulator.has_formula = true;
                }
            }
            Ok(Event::End(event)) => match event.name().as_ref() {
                b"sheetData" => in_sheet_data = false,
                b"c" => {
                    if let Some(accumulator) = current.take()
                        && accumulator.has_formula
                    {
                        if formulas.len() >= max_formulas {
                            truncated = true;
                        } else {
                            formulas.push(accumulator.finish(shared_strings, styles));
                        }
                    }
                    capture_formula = false;
                    capture_value = false;
                }
                b"f" => capture_formula = false,
                b"v" => capture_value = false,
                _ => {}
            },
            Ok(Event::Text(text)) => {
                if let Some(accumulator) = current.as_mut() {
                    let decoded = text.decode().map_err(|err| {
                        ExcelInspectionError::Message(format!(
                            "failed to decode formula inventory text: {err}"
                        ))
                    })?;
                    if capture_formula {
                        accumulator.formula.push_str(decoded.as_ref());
                    } else if capture_value {
                        accumulator.cached_value.push_str(decoded.as_ref());
                    }
                }
            }
            Ok(Event::GeneralRef(reference)) => {
                if let Some(accumulator) = current.as_mut() {
                    let decoded = decode_general_ref(&reference)?;
                    if capture_formula {
                        accumulator.formula.push_str(&decoded);
                    } else if capture_value {
                        accumulator.cached_value.push_str(&decoded);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse worksheet formulas: {err}"
                )));
            }
        }
        buf.clear();
    }

    Ok((formulas, truncated))
}

struct FormulaAccumulator {
    reference: String,
    cell_type: Option<String>,
    style_index: Option<u32>,
    formula: String,
    cached_value: String,
    formula_type: Option<String>,
    shared_index: Option<u32>,
    shared_range: Option<String>,
    has_formula: bool,
}

impl FormulaAccumulator {
    fn from_event(event: &BytesStart<'_>) -> Result<Self, ExcelInspectionError> {
        Ok(Self {
            reference: attr_value(event, b"r")?.unwrap_or_default(),
            cell_type: attr_value(event, b"t")?,
            style_index: attr_value(event, b"s")?.and_then(|value| value.parse().ok()),
            formula: String::new(),
            cached_value: String::new(),
            formula_type: None,
            shared_index: None,
            shared_range: None,
            has_formula: false,
        })
    }

    fn finish(self, shared_strings: &[String], styles: &StyleLookup) -> SheetFormulaSummary {
        let (number_format_id, number_format_code) = styles.number_format(self.style_index);
        let formula_was_truncated = self.formula.chars().count() > MAX_FORMULA_TEXT_CHARS;
        let formula = bounded_text(&self.formula, MAX_FORMULA_TEXT_CHARS);
        let parse = parse_formula_ast(&formula, formula_was_truncated);
        let warnings = formula_warnings(&formula);
        SheetFormulaSummary {
            reference: bounded_text(&self.reference, 32),
            formula,
            cached_value: resolve_cell_value(
                self.cell_type.as_deref(),
                &self.cached_value,
                "",
                shared_strings,
            ),
            parse,
            sql_preview: Default::default(),
            warnings,
            formula_type: self.formula_type,
            shared_index: self.shared_index,
            shared_range: self.shared_range,
            style_index: self.style_index,
            number_format_id,
            number_format_code,
        }
    }
}

fn formula_warnings(formula: &str) -> Vec<String> {
    let upper = formula.to_ascii_uppercase();
    let mut warnings = Vec::new();
    if contains_function(&upper, "INDIRECT") {
        warnings.push("uses_indirect_reference".to_string());
    }
    if contains_function(&upper, "OFFSET") {
        warnings.push("uses_offset_reference".to_string());
    }
    if ["NOW", "TODAY", "RAND", "RANDBETWEEN"]
        .iter()
        .any(|name| contains_function(&upper, name))
    {
        warnings.push("uses_volatile_function".to_string());
    }
    if formula.contains('[') || upper.contains("HTTP://") || upper.contains("HTTPS://") {
        warnings.push("references_external_workbook_or_url".to_string());
    }
    if formula.contains('#')
        || [
            "_XLFN.",
            "FILTER(",
            "UNIQUE(",
            "SORT(",
            "SORTBY(",
            "SEQUENCE(",
            "RANDARRAY(",
        ]
        .iter()
        .any(|marker| upper.contains(marker))
    {
        warnings.push("uses_dynamic_array_or_spill_marker".to_string());
    }
    warnings
}

fn contains_function(formula: &str, name: &str) -> bool {
    formula.contains(&format!("{name}("))
}
