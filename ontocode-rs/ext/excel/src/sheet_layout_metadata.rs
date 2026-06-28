use std::fs::File;
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
use quick_xml::events::Event;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_value;
use zip::ZipArchive;

use crate::backend::ExcelInspectionError;
use crate::backend::inspect_workbook_with_display_path;
use crate::preview::attr_value;
use crate::preview::bounded_text;
use crate::preview::read_xml_entry;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::ExcelThreadState;
use crate::tool::SheetKind;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;
use crate::tool::WorkbookFormat;
use crate::vba_extract::parse_tool_args;
use crate::vba_extract::resolve_workbook_path_from_model_arg;

pub(crate) const INSPECT_SHEET_LAYOUT_METADATA_TOOL_NAME: &str = "inspect_sheet_layout_metadata";

const INSPECT_SHEET_LAYOUT_METADATA_DESCRIPTION: &str = "Inspect bounded offline sheet layout metadata, including merged ranges, pane state, auto-filter metadata, and print-area hints.";
const MAX_WORKBOOK_XML_BYTES: usize = 512 * 1024;
const MAX_WORKSHEET_XML_BYTES: usize = 2 * 1024 * 1024;
const MAX_MERGED_RANGE_SAMPLE: usize = 16;
const MAX_REFERENCE_CHARS: usize = 64;
const MAX_PRINT_AREA_CHARS: usize = 256;
const MAX_WARNINGS: usize = 16;

type ParsedSheetLayoutMetadata = (
    usize,
    Vec<String>,
    bool,
    Option<SheetLayoutPaneSummary>,
    Option<String>,
);

#[derive(Clone, Default)]
pub(crate) struct ExcelInspectSheetLayoutMetadataTool {
    thread_state: Arc<ExcelThreadState>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct InspectSheetLayoutMetadataArgs {
    pub path: String,
    pub sheet: SheetSelector,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SheetLayoutPaneKind {
    Freeze,
    Split,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct SheetLayoutPaneSummary {
    pub kind: SheetLayoutPaneKind,
    pub top_left_cell: Option<String>,
    pub x_split: Option<String>,
    pub y_split: Option<String>,
    pub active_pane: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub(crate) struct InspectSheetLayoutMetadataResult {
    pub mode: String,
    pub path: String,
    pub sheet: SheetPreview,
    pub merged_range_count: usize,
    pub merged_ranges_sample: Vec<String>,
    pub pane: Option<SheetLayoutPaneSummary>,
    pub auto_filter_range: Option<String>,
    pub print_area: Option<String>,
    pub warnings: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ExcelInspectSheetLayoutMetadataTool {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_SHEET_LAYOUT_METADATA_TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let input_schema =
            serde_json::to_value(schemars::schema_for!(InspectSheetLayoutMetadataArgs))
                .unwrap_or_else(|err| {
                    panic!("inspect_sheet_layout_metadata args schema should serialize: {err}")
                });
        let output_schema =
            serde_json::to_value(schemars::schema_for!(InspectSheetLayoutMetadataResult))
                .unwrap_or_else(|err| {
                    panic!("inspect_sheet_layout_metadata result schema should serialize: {err}")
                });
        ToolSpec::Namespace(ontocode_tools::ResponsesApiNamespace {
            name: EXCEL_NAMESPACE.to_string(),
            description: ontocode_tools::default_namespace_description(EXCEL_NAMESPACE),
            tools: vec![ontocode_tools::ResponsesApiNamespaceTool::Function(
                ontocode_tools::ResponsesApiTool {
                    name: INSPECT_SHEET_LAYOUT_METADATA_TOOL_NAME.to_string(),
                    description: INSPECT_SHEET_LAYOUT_METADATA_DESCRIPTION.to_string(),
                    strict: false,
                    defer_loading: None,
                    parameters: parse_tool_input_schema(&input_schema).unwrap_or_else(|err| {
                        panic!("inspect_sheet_layout_metadata args schema should parse: {err}")
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
        let args = parse_tool_args::<InspectSheetLayoutMetadataArgs>(
            &call,
            "excel.inspect_sheet_layout_metadata",
        )?;
        let cwd = self.thread_state.current_cwd().ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "excel.inspect_sheet_layout_metadata workspace context is unavailable for this turn".to_string(),
            )
        })?;
        let workbook_path = resolve_workbook_path_from_model_arg(
            "excel.inspect_sheet_layout_metadata",
            &args.path,
            &cwd,
        )?;
        let result = inspect_sheet_layout_metadata_with_display_path(
            &workbook_path,
            Path::new(args.path.trim()),
            &args.sheet,
        )?;
        let value = to_value(result).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to serialize sheet layout metadata: {err}"
            ))
        })?;
        Ok(Box::new(JsonToolOutput::new(value)))
    }
}

impl ExcelInspectSheetLayoutMetadataTool {
    pub(crate) fn new(thread_state: Arc<ExcelThreadState>) -> Self {
        Self { thread_state }
    }
}

pub(crate) fn inspect_sheet_layout_metadata_with_display_path(
    path: &Path,
    display_path: &Path,
    sheet: &SheetSelector,
) -> Result<InspectSheetLayoutMetadataResult, ExcelInspectionError> {
    let workbook = inspect_workbook_with_display_path(path, display_path)?;
    if !matches!(workbook.format, WorkbookFormat::Xlsx | WorkbookFormat::Xlsm) {
        return Err(ExcelInspectionError::Message(
            "excel.inspect_sheet_layout_metadata supports only .xlsx and .xlsm in this stage"
                .to_string(),
        ));
    }
    let (sheet_index, selected_sheet) = select_sheet_summary(&workbook.sheets, sheet)?;
    let sheet_name = selected_sheet.name.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.inspect_sheet_layout_metadata could not resolve a sheet name for the selected sheet"
                .to_string(),
        )
    })?;
    let sheet_part_path = selected_sheet.part_path.clone().ok_or_else(|| {
        ExcelInspectionError::Message(
            "excel.inspect_sheet_layout_metadata could not resolve a worksheet part path for the selected sheet"
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
    let workbook_xml = read_xml_entry(&mut archive, "xl/workbook.xml", MAX_WORKBOOK_XML_BYTES)?;
    let worksheet_xml = read_xml_entry(&mut archive, &sheet_part_path, MAX_WORKSHEET_XML_BYTES)?;

    let mut warnings = workbook.warnings.clone();
    let (
        merged_range_count,
        merged_ranges_sample,
        merged_ranges_truncated,
        pane,
        auto_filter_range,
    ) = parse_sheet_layout_metadata(&worksheet_xml)?;
    if merged_ranges_truncated {
        push_warning(
            &mut warnings,
            format!(
                "merged range sample truncated to {MAX_MERGED_RANGE_SAMPLE} entries for sheet {sheet_name}"
            ),
        );
    }
    let print_area = parse_print_area(&workbook_xml, sheet_index, &sheet_name)?;

    Ok(InspectSheetLayoutMetadataResult {
        mode: "read_only_inspection".to_string(),
        path: display_path.display().to_string(),
        sheet: SheetPreview {
            name: sheet_name,
            sheet_id: selected_sheet.sheet_id,
            part_path: sheet_part_path,
        },
        merged_range_count,
        merged_ranges_sample,
        pane,
        auto_filter_range,
        print_area,
        warnings,
    })
}

fn select_sheet_summary<'a>(
    sheets: &'a [crate::tool::SheetSummary],
    selector: &SheetSelector,
) -> Result<(usize, &'a crate::tool::SheetSummary), ExcelInspectionError> {
    match selector {
        SheetSelector::Name { name } => sheets
            .iter()
            .enumerate()
            .find(|(_, sheet)| {
                sheet.kind == SheetKind::Worksheet && sheet.name.as_deref() == Some(name.as_str())
            })
            .ok_or_else(|| {
                ExcelInspectionError::Message(format!(
                    "excel.inspect_sheet_layout_metadata could not find worksheet named {name}"
                ))
            }),
        SheetSelector::Index { index } => sheets
            .iter()
            .enumerate()
            .filter(|(_, sheet)| sheet.kind == SheetKind::Worksheet)
            .nth(*index)
            .ok_or_else(|| {
                ExcelInspectionError::Message(format!(
                    "excel.inspect_sheet_layout_metadata could not find worksheet index {index}"
                ))
            }),
    }
}

fn parse_sheet_layout_metadata(
    worksheet_xml: &str,
) -> Result<ParsedSheetLayoutMetadata, ExcelInspectionError> {
    let mut reader = Reader::from_str(worksheet_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut merged_range_count = 0usize;
    let mut merged_ranges_sample = Vec::new();
    let mut merged_ranges_truncated = false;
    let mut pane = None;
    let mut auto_filter_range = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"mergeCell" =>
            {
                merged_range_count += 1;
                if let Some(reference) = attr_value(&event, b"ref")? {
                    if merged_ranges_sample.len() < MAX_MERGED_RANGE_SAMPLE {
                        merged_ranges_sample.push(bounded_text(&reference, MAX_REFERENCE_CHARS));
                    } else {
                        merged_ranges_truncated = true;
                    }
                }
            }
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"pane" && pane.is_none() =>
            {
                pane = parse_pane_summary(&event)?;
            }
            Ok(Event::Empty(event)) | Ok(Event::Start(event))
                if event.name().as_ref() == b"autoFilter" && auto_filter_range.is_none() =>
            {
                auto_filter_range = attr_value(&event, b"ref")?
                    .map(|value| bounded_text(&value, MAX_REFERENCE_CHARS));
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse worksheet layout metadata: {err}"
                )));
            }
        }
        buf.clear();
    }

    Ok((
        merged_range_count,
        merged_ranges_sample,
        merged_ranges_truncated,
        pane,
        auto_filter_range,
    ))
}

fn parse_pane_summary(
    event: &quick_xml::events::BytesStart<'_>,
) -> Result<Option<SheetLayoutPaneSummary>, ExcelInspectionError> {
    let state = attr_value(event, b"state")?;
    let x_split = attr_value(event, b"xSplit")?;
    let y_split = attr_value(event, b"ySplit")?;
    let top_left_cell = attr_value(event, b"topLeftCell")?;
    let active_pane = attr_value(event, b"activePane")?;

    let kind = match state.as_deref() {
        Some("frozen") | Some("frozenSplit") => Some(SheetLayoutPaneKind::Freeze),
        Some("split") => Some(SheetLayoutPaneKind::Split),
        _ if x_split.is_some() || y_split.is_some() => Some(SheetLayoutPaneKind::Split),
        _ => None,
    };

    Ok(kind.map(|kind| SheetLayoutPaneSummary {
        kind,
        top_left_cell: top_left_cell.map(|value| bounded_text(&value, MAX_REFERENCE_CHARS)),
        x_split: x_split.map(|value| bounded_text(&value, MAX_REFERENCE_CHARS)),
        y_split: y_split.map(|value| bounded_text(&value, MAX_REFERENCE_CHARS)),
        active_pane: active_pane.map(|value| bounded_text(&value, MAX_REFERENCE_CHARS)),
    }))
}

fn parse_print_area(
    workbook_xml: &str,
    sheet_index: usize,
    sheet_name: &str,
) -> Result<Option<String>, ExcelInspectionError> {
    let mut reader = Reader::from_str(workbook_xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut capture_text = false;
    let mut matches_sheet = false;
    let mut current_text = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) if event.name().as_ref() == b"definedName" => {
                current_text.clear();
                capture_text = matches!(
                    attr_value(&event, b"name")?.as_deref(),
                    Some("_xlnm.Print_Area")
                );
                matches_sheet = capture_text
                    && attr_value(&event, b"localSheetId")?
                        .and_then(|value| value.parse::<usize>().ok())
                        == Some(sheet_index);
            }
            Ok(Event::Text(text)) if capture_text => {
                let decoded = text.decode().map_err(|err| {
                    ExcelInspectionError::Message(format!(
                        "failed to decode workbook print-area text: {err}"
                    ))
                })?;
                current_text.push_str(decoded.as_ref());
            }
            Ok(Event::End(event)) if event.name().as_ref() == b"definedName" => {
                if capture_text
                    && (matches_sheet || current_text.starts_with(&format!("'{sheet_name}'!")))
                {
                    return Ok(Some(bounded_text(&current_text, MAX_PRINT_AREA_CHARS)));
                }
                capture_text = false;
                matches_sheet = false;
                current_text.clear();
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => {
                return Err(ExcelInspectionError::Message(format!(
                    "failed to parse workbook print-area metadata: {err}"
                )));
            }
        }
        buf.clear();
    }

    Ok(None)
}

fn push_warning(warnings: &mut Vec<String>, warning: String) {
    if warnings.len() < MAX_WARNINGS && !warnings.iter().any(|existing| existing == &warning) {
        warnings.push(warning);
    }
}
