use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use ontocode_extension_api::ConversationHistory;
use ontocode_extension_api::ExtensionData;
use ontocode_extension_api::ExtensionRegistryBuilder;
use ontocode_extension_api::NoopTurnItemEmitter;
use ontocode_extension_api::ToolCall;
use ontocode_extension_api::ToolName;
use ontocode_extension_api::ToolPayload;
use ontocode_extension_api::TurnInputContext;
use ontocode_extension_api::TurnInputEnvironment;
use ontocode_utils_output_truncation::TruncationPolicy;
use pretty_assertions::assert_eq;
use serde_json::json;
use tempfile::tempdir;
use zip::CompressionMethod;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use crate::backend::inspect_workbook;
use crate::export::export_sheet_to_csv_with_display_path;
use crate::extension::install;
use crate::powerquery_extract::EXTRACT_POWERQUERY_QUERIES_TOOL_NAME;
use crate::powerquery_extract::ExtractPowerQueryQueriesResult;
use crate::powerquery_extract::ExtractedPowerQueryQuery;
use crate::powerquery_translate::TranslatePowerQueryToSqlPreviewResult;
use crate::preview::read_sheet_preview_with_display_path;
use crate::tool::CellContentMode;
use crate::tool::DefinedNameSummary;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::EXPORT_SHEET_TO_CSV_TOOL_NAME;
use crate::tool::ExportSheetToCsvResult;
use crate::tool::INSPECT_SHEET_FORMULAS_TOOL_NAME;
use crate::tool::INSPECT_WORKBOOK_TOOL_NAME;
use crate::tool::InspectSheetFormulasResult;
use crate::tool::InspectWorkbookResult;
use crate::tool::READ_SHEET_PREVIEW_TOOL_NAME;
use crate::tool::ReadSheetPreviewResult;
use crate::tool::SheetDataValidationSummary;
use crate::tool::SheetDimension;
use crate::tool::SheetFormulaSummary;
use crate::tool::SheetPreview;
use crate::tool::SheetPreviewCell;
use crate::tool::SheetPreviewRow;
use crate::tool::SheetSelector;
use crate::tool::WorkbookFormat;
use crate::tool::workbook_path_from_model_arg;
use crate::vba_extract::EXTRACT_VBA_MODULES_TOOL_NAME;
use crate::vba_extract::ExtractVbaModulesResult;
use crate::vba_extract::ExtractedVbaModule;
use crate::vba_extract::VbaModuleKind;
use crate::vba_onlyoffice_analyze::ANALYZE_VBA_ONLYOFFICE_MIGRATION_TOOL_NAME;
use crate::vba_onlyoffice_analyze::AnalyzeVbaOnlyofficeMigrationResult;
use crate::vba_onlyoffice_analyze::AnalyzeVbaOperationSummary;
use crate::vba_onlyoffice_analyze::AnalyzeVbaProcedureSummary;
use crate::vba_onlyoffice_translate::TRANSLATE_VBA_TO_ONLYOFFICE_JS_PREVIEW_TOOL_NAME;
use crate::vba_onlyoffice_translate::TranslateVbaToOnlyofficeJsPreviewResult;
use crate::vba_onlyoffice_workbook_review::REVIEW_VBA_ONLYOFFICE_WORKBOOK_TOOL_NAME;
use crate::vba_onlyoffice_workbook_review::ReviewVbaOnlyofficeWorkbookResult;
use crate::vba_onlyoffice_workbook_review::ReviewedVbaOnlyofficeWorkbookModule;
use crate::vba_translate::MPreviewQuery;
use crate::vba_translate::TRANSLATE_VBA_TO_M_PREVIEW_TOOL_NAME;
use crate::vba_translate::TranslateVbaToMPreviewResult;
use ontocode_core::config::Config;
use ontocode_extension_api::FunctionCallError;

fn write_zip_fixture<N: AsRef<str>, C: AsRef<str>>(path: &std::path::Path, entries: &[(N, C)]) {
    let file = File::create(path).expect("create workbook fixture");
    let mut writer = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    for (name, contents) in entries {
        writer
            .start_file(name.as_ref(), options)
            .expect("start workbook entry");
        writer
            .write_all(contents.as_ref().as_bytes())
            .expect("write workbook entry");
    }
    writer.finish().expect("finish workbook fixture");
}

fn write_zip_fixture_bytes<N: AsRef<str>>(path: &std::path::Path, entries: &[(N, Vec<u8>)]) {
    let file = File::create(path).expect("create workbook fixture");
    let mut writer = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    for (name, contents) in entries {
        writer
            .start_file(name.as_ref(), options)
            .expect("start workbook entry");
        writer.write_all(contents).expect("write workbook entry");
    }
    writer.finish().expect("finish workbook fixture");
}

fn build_embedded_zip_bytes(entries: &[(&str, &str)]) -> Vec<u8> {
    let cursor = std::io::Cursor::new(Vec::<u8>::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    for (name, contents) in entries {
        writer
            .start_file(*name, options)
            .expect("start embedded zip entry");
        writer
            .write_all(contents.as_bytes())
            .expect("write embedded zip entry");
    }
    writer.finish().expect("finish embedded zip").into_inner()
}

fn build_data_mashup_xml_utf16(entries: &[(&str, &str)]) -> Vec<u8> {
    let mut payload = Vec::from([0, 0, 0, 0]);
    payload.extend(build_embedded_zip_bytes(entries));
    payload.extend(b"<metadata/>");
    let xml = format!(
        r#"<?xml version="1.0" encoding="utf-16"?><DataMashup xmlns="http://schemas.microsoft.com/DataMashup">{}</DataMashup>"#,
        BASE64_STANDARD.encode(payload)
    );
    let mut bytes = vec![0xFF, 0xFE];
    for unit in xml.encode_utf16() {
        bytes.extend(unit.to_le_bytes());
    }
    bytes
}

#[test]
fn inspect_workbook_decodes_openxml_metadata() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsx");
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                  <Default Extension="xml" ContentType="application/xml"/>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                  <Override PartName="/xl/worksheets/sheet2.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                  <sheets>
                    <sheet name="Summary" sheetId="1" r:id="rId1"/>
                    <sheet name="HiddenData" sheetId="2" state="hidden" r:id="rId2"/>
                  </sheets>
                  <definedNames>
                    <definedName name="Print_Area" localSheetId="1">HiddenData!$A$1:$B$2</definedName>
                  </definedNames>
                </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/>
                </Relationships>"#,
            ),
            ("xl/worksheets/sheet1.xml", "<worksheet/>"),
            ("xl/worksheets/sheet2.xml", "<worksheet/>"),
        ],
    );

    let result = inspect_workbook(&path).expect("inspect workbook");
    assert_eq!(
        result,
        InspectWorkbookResult {
            path: path.display().to_string(),
            format: WorkbookFormat::Xlsx,
            package_part_count: 5,
            package_parts_sample: vec![
                "[Content_Types].xml".to_string(),
                "xl/workbook.xml".to_string(),
                "xl/_rels/workbook.xml.rels".to_string(),
                "xl/worksheets/sheet1.xml".to_string(),
                "xl/worksheets/sheet2.xml".to_string(),
            ],
            sheets: vec![
                crate::tool::SheetSummary {
                    name: Some("Summary".to_string()),
                    sheet_id: Some(1),
                    relationship_id: Some("rId1".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    visibility: crate::tool::SheetVisibility::Visible,
                    kind: crate::tool::SheetKind::Worksheet,
                },
                crate::tool::SheetSummary {
                    name: Some("HiddenData".to_string()),
                    sheet_id: Some(2),
                    relationship_id: Some("rId2".to_string()),
                    part_path: Some("xl/worksheets/sheet2.xml".to_string()),
                    visibility: crate::tool::SheetVisibility::Hidden,
                    kind: crate::tool::SheetKind::Worksheet,
                },
            ],
            markers: crate::tool::WorkbookMarkers {
                has_vba_project: false,
                has_macro_enabled_package: false,
                has_power_query: false,
                has_connections: false,
                has_custom_xml: false,
                has_external_links: false,
                has_tables: false,
                has_comments: false,
                has_drawings: false,
                has_embedded_objects: false,
                has_charts: false,
                has_pivot_tables: false,
                has_formulas: false,
                has_xlsb_package: false,
            },
            marker_summaries: Vec::new(),
            warnings: Vec::new(),
        }
    );
}

#[test]
fn inspect_workbook_bounds_model_visible_strings() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsx");
    let long_sheet_name = "S".repeat(300);
    let workbook_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                  xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
          <sheets>
            <sheet name="{long_sheet_name}" sheetId="1" r:id="rId1"/>
          </sheets>
        </workbook>"#
    );
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml".to_string(),
                r#"<Types>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                </Types>"#
                    .to_string(),
            ),
            ("xl/workbook.xml".to_string(), workbook_xml),
            (
                "xl/_rels/workbook.xml.rels".to_string(),
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#
                    .to_string(),
            ),
            ("xl/worksheets/sheet1.xml".to_string(), "<worksheet/>".to_string()),
        ],
    );

    let result = inspect_workbook(&path).expect("inspect workbook");
    assert_eq!(
        result.sheets[0].name,
        Some(format!("{}...", "S".repeat(253)))
    );
}

#[test]
fn inspect_workbook_rejects_too_many_package_parts() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("too_many_parts.xlsx");
    let entries = (0..4097)
        .map(|index| (format!("xl/worksheets/sheet{index}.xml"), String::new()))
        .collect::<Vec<_>>();
    write_zip_fixture(&path, &entries);

    let err = inspect_workbook(&path).expect_err("too many parts should fail");
    assert_eq!(
        err.to_string(),
        "workbook package has 4097 entries; maximum supported is 4096"
    );
}

#[test]
fn inspect_workbook_rejects_oversized_xml_entry() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("oversized_entry.xlsx");
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml".to_string(),
                r#"<Types>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                </Types>"#
                    .to_string(),
            ),
            ("xl/workbook.xml".to_string(), "x".repeat(1024 * 1024 + 1)),
        ],
    );

    let err = inspect_workbook(&path).expect_err("oversized entry should fail");
    assert_eq!(
        err.to_string(),
        "workbook entry xl/workbook.xml exceeds 1048576 bytes"
    );
}

#[test]
fn inspect_workbook_rejects_too_many_xml_scan_entries() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("too_many_xml_entries.xlsx");
    let mut entries = vec![
        (
            "[Content_Types].xml".to_string(),
            r#"<Types>
              <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
            </Types>"#
                .to_string(),
        ),
        (
            "xl/workbook.xml".to_string(),
            r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets/></workbook>"#
                .to_string(),
        ),
    ];
    entries.extend((0..129).map(|index| {
        (
            format!("xl/worksheets/sheet{index}.xml"),
            "<worksheet/>".to_string(),
        )
    }));
    write_zip_fixture(&path, &entries);

    let err = inspect_workbook(&path).expect_err("too many XML scan entries should fail");
    assert_eq!(err.to_string(), "workbook XML scan exceeded 128 entries");
}

#[test]
fn inspect_workbook_rejects_total_xml_scan_bytes_over_budget() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("too_many_xml_bytes.xlsx");
    let mut entries = vec![
        (
            "[Content_Types].xml".to_string(),
            r#"<Types>
              <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
            </Types>"#
                .to_string(),
        ),
        (
            "xl/workbook.xml".to_string(),
            r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets/></workbook>"#
                .to_string(),
        ),
    ];
    entries.extend((0..8).map(|index| {
        (
            format!("xl/worksheets/sheet{index}.xml"),
            "x".repeat(1024 * 1024),
        )
    }));
    write_zip_fixture(&path, &entries);

    let err = inspect_workbook(&path).expect_err("total XML scan bytes should fail");
    assert_eq!(
        err.to_string(),
        "workbook XML scan exceeds 8388608 total bytes"
    );
}

#[test]
fn inspect_workbook_detects_xlsb_package_without_claiming_sheet_names() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsb");
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                  <Override PartName="/xl/workbook.bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                </Types>"#,
            ),
            ("xl/workbook.bin", "binary-workbook"),
            ("xl/worksheets/sheet1.bin", "binary-sheet"),
        ],
    );

    let result = inspect_workbook(&path).expect("inspect workbook");
    assert_eq!(result.format, WorkbookFormat::Xlsb);
    assert_eq!(result.sheets.len(), 1);
    assert_eq!(result.sheets[0].name, None);
    assert_eq!(
        result.sheets[0].part_path,
        Some("xl/worksheets/sheet1.bin".to_string())
    );
    assert!(result.markers.has_xlsb_package);
    assert_eq!(
        result.marker_summaries,
        vec![crate::tool::MarkerSummary {
            category: "xlsb_package".to_string(),
            count: 1,
            part_paths_sample: vec!["xl/workbook.bin".to_string()],
        }]
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("xlsb workbook names are not decoded"))
    );
}

#[test]
fn inspect_workbook_reports_marker_counts_and_paths() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsm");
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml",
                r#"<Types>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.ms-excel.sheet.macroEnabled.main+xml"/>
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets/></workbook>"#,
            ),
            ("xl/vbaProject.bin", "vba"),
            (
                "xl/connections.xml",
                r#"<connections><connection name="Query - A"><dbPr connection="Provider=Microsoft.Mashup"/></connection></connections>"#,
            ),
            ("customXml/item1.xml", "<DataMashup>payload</DataMashup>"),
            ("xl/comments1.xml", "<comments/>"),
            ("xl/drawings/drawing1.xml", "<drawing/>"),
            ("xl/charts/chart1.xml", "<chart/>"),
            (
                "xl/worksheets/sheet1.xml",
                "<worksheet><sheetData><row><c><f>A1+1</f></c></row></sheetData></worksheet>",
            ),
        ],
    );

    let result = inspect_workbook(&path).expect("inspect workbook");

    assert_eq!(result.format, WorkbookFormat::Xlsm);
    assert!(result.markers.has_vba_project);
    assert!(result.markers.has_power_query);
    assert!(result.markers.has_comments);
    assert!(result.markers.has_drawings);
    assert!(result.markers.has_charts);
    assert!(result.markers.has_formulas);
    assert_eq!(
        result
            .marker_summaries
            .iter()
            .map(|summary| summary.category.as_str())
            .collect::<Vec<_>>(),
        vec![
            "vba_project",
            "power_query",
            "connections",
            "custom_xml",
            "comments",
            "drawings",
            "charts",
            "formulas",
        ]
    );
}

#[test]
fn inspect_workbook_handles_utf16_power_query_custom_xml() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("utf16_custom_xml.xlsm");
    write_zip_fixture_bytes(
        &path,
        &[
            (
                "[Content_Types].xml",
                br#"<Types>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.ms-excel.sheet.macroEnabled.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                </Types>"#
                    .to_vec(),
            ),
            (
                "xl/workbook.xml",
                br#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Summary" sheetId="1" r:id="rId1"/></sheets></workbook>"#
                    .to_vec(),
            ),
            (
                "xl/_rels/workbook.xml.rels",
                br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#
                    .to_vec(),
            ),
            ("xl/connections.xml", b"<connections><connection name=\"Query - Sales\"><dbPr connection=\"Provider=Microsoft.Mashup\"/></connection></connections>".to_vec()),
            (
                "customXml/item1.xml",
                build_data_mashup_xml_utf16(&[("Formulas/Section1.m", "section Section1;\nshared Sales = 1;")]),
            ),
            ("xl/worksheets/sheet1.xml", b"<worksheet/>".to_vec()),
        ],
    );

    let result = inspect_workbook(&path).expect("inspect workbook");

    assert_eq!(
        result,
        InspectWorkbookResult {
            path: path.display().to_string(),
            format: WorkbookFormat::Xlsm,
            package_part_count: 6,
            package_parts_sample: vec![
                "[Content_Types].xml".to_string(),
                "xl/workbook.xml".to_string(),
                "xl/_rels/workbook.xml.rels".to_string(),
                "xl/connections.xml".to_string(),
                "customXml/item1.xml".to_string(),
                "xl/worksheets/sheet1.xml".to_string(),
            ],
            sheets: vec![crate::tool::SheetSummary {
                name: Some("Summary".to_string()),
                sheet_id: Some(1),
                relationship_id: Some("rId1".to_string()),
                part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                visibility: crate::tool::SheetVisibility::Visible,
                kind: crate::tool::SheetKind::Worksheet,
            }],
            markers: crate::tool::WorkbookMarkers {
                has_vba_project: false,
                has_macro_enabled_package: true,
                has_power_query: true,
                has_connections: true,
                has_custom_xml: true,
                has_external_links: false,
                has_tables: false,
                has_comments: false,
                has_drawings: false,
                has_embedded_objects: false,
                has_charts: false,
                has_pivot_tables: false,
                has_formulas: false,
                has_xlsb_package: false,
            },
            marker_summaries: vec![
                crate::tool::MarkerSummary {
                    category: "power_query".to_string(),
                    count: 2,
                    part_paths_sample: vec![
                        "xl/connections.xml".to_string(),
                        "customXml/item1.xml".to_string(),
                    ],
                },
                crate::tool::MarkerSummary {
                    category: "connections".to_string(),
                    count: 1,
                    part_paths_sample: vec!["xl/connections.xml".to_string()],
                },
                crate::tool::MarkerSummary {
                    category: "custom_xml".to_string(),
                    count: 1,
                    part_paths_sample: vec!["customXml/item1.xml".to_string()],
                },
            ],
            warnings: Vec::new(),
        }
    );
}

#[test]
fn read_sheet_preview_decodes_shared_strings_and_formulas() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsx");
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                  <Default Extension="xml" ContentType="application/xml"/>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                  <Override PartName="/xl/sharedStrings.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"/>
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                  <sheets>
                    <sheet name="Summary" sheetId="1" r:id="rId1"/>
                  </sheets>
                </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#,
            ),
            (
                "xl/sharedStrings.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <si><t>Shared text</t></si>
                </sst>"#,
            ),
            (
                "xl/worksheets/sheet1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <dimension ref="A1:D2"/>
                  <sheetData>
                    <row r="1">
                      <c r="A1" t="s"><v>0</v></c>
                      <c r="B1" t="inlineStr"><is><t>Inline value</t></is></c>
                      <c r="C1"><f>SUM(D2:D3)</f><v>3</v></c>
                    </row>
                    <row r="2">
                      <c r="D2"><v>1</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );

    let result = read_sheet_preview_with_display_path(
        &path,
        &path,
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
        CellContentMode::ValuesAndFormulas,
    )
    .expect("read sheet preview");

    assert_eq!(
        result,
        ReadSheetPreviewResult {
            path: path.display().to_string(),
            sheet: SheetPreview {
                name: "Summary".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            dimension: Some(SheetDimension {
                reference: "A1:D2".to_string(),
            }),
            max_rows_applied: 10,
            cell_content: CellContentMode::ValuesAndFormulas,
            rows: vec![
                SheetPreviewRow {
                    row_index: 1,
                    cells: vec![
                        SheetPreviewCell {
                            reference: "A1".to_string(),
                            value: Some("Shared text".to_string()),
                            formula: None,
                        },
                        SheetPreviewCell {
                            reference: "B1".to_string(),
                            value: Some("Inline value".to_string()),
                            formula: None,
                        },
                        SheetPreviewCell {
                            reference: "C1".to_string(),
                            value: Some("3".to_string()),
                            formula: Some("SUM(D2:D3)".to_string()),
                        },
                    ],
                },
                SheetPreviewRow {
                    row_index: 2,
                    cells: vec![SheetPreviewCell {
                        reference: "D2".to_string(),
                        value: Some("1".to_string()),
                        formula: None,
                    }],
                },
            ],
            data_validations: Vec::new(),
            truncated: false,
            warnings: Vec::new(),
        }
    );
}

#[test]
fn read_sheet_preview_preserves_formula_xml_entities() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("formula_entities.xlsx");
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                  <Default Extension="xml" ContentType="application/xml"/>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                  <sheets>
                    <sheet name="Summary" sheetId="1" r:id="rId1"/>
                  </sheets>
                </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#,
            ),
            (
                "xl/worksheets/sheet1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="1">
                      <c r="A1"><f>VLOOKUP(C21,KPI_Name,2,FALSE)&amp;" for "&amp;C19</f><v>Sales for North</v></c>
                      <c r="B1"><f>C24&lt;&gt;""</f><v>TRUE</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );

    let result = read_sheet_preview_with_display_path(
        &path,
        &path,
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
        CellContentMode::ValuesAndFormulas,
    )
    .expect("read sheet preview");

    assert_eq!(
        result,
        ReadSheetPreviewResult {
            path: path.display().to_string(),
            sheet: SheetPreview {
                name: "Summary".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            dimension: None,
            max_rows_applied: 10,
            cell_content: CellContentMode::ValuesAndFormulas,
            rows: vec![SheetPreviewRow {
                row_index: 1,
                cells: vec![
                    SheetPreviewCell {
                        reference: "A1".to_string(),
                        value: Some("Sales for North".to_string()),
                        formula: Some("VLOOKUP(C21,KPI_Name,2,FALSE)&\" for \"&C19".to_string()),
                    },
                    SheetPreviewCell {
                        reference: "B1".to_string(),
                        value: Some("TRUE".to_string()),
                        formula: Some("C24<>\"\"".to_string()),
                    },
                ],
            }],
            data_validations: Vec::new(),
            truncated: false,
            warnings: Vec::new(),
        }
    );
}

#[test]
fn read_sheet_preview_reports_bounded_data_validations() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("data_validations.xlsx");
    let source_rows = (1..=300)
        .map(|row| format!(r#"<row r="{row}"><c r="D{row}"><v>Choice{row}</v></c></row>"#))
        .collect::<String>();
    let worksheet_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
          <sheetData>
            <row r="1"><c r="A1"><v>7</v></c></row>
            {source_rows}
          </sheetData>
          <dataValidations count="4">
            <dataValidation type="list" allowBlank="1" showDropDown="0" showErrorMessage="0" errorStyle="warning" sqref="A1 A2">
              <formula1>"Red,Blue"</formula1>
            </dataValidation>
            <dataValidation type="list" sqref="B1">
              <formula1>$D$1:$D$300</formula1>
            </dataValidation>
            <dataValidation type="list" sqref="C1">
              <formula1>INDIRECT(&quot;Choices&quot;)</formula1>
            </dataValidation>
            <dataValidation type="list" sqref="E1">
              <formula1>$D$1:$D$300</formula1>
            </dataValidation>
          </dataValidations>
        </worksheet>"#
    );
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml".to_string(),
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                  <Default Extension="xml" ContentType="application/xml"/>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                </Types>"#
                    .to_string(),
            ),
            (
                "xl/workbook.xml".to_string(),
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                  <sheets>
                    <sheet name="Summary" sheetId="1" r:id="rId1"/>
                  </sheets>
                </workbook>"#
                    .to_string(),
            ),
            (
                "xl/_rels/workbook.xml.rels".to_string(),
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#
                    .to_string(),
            ),
            ("xl/worksheets/sheet1.xml".to_string(), worksheet_xml),
        ],
    );

    let result = read_sheet_preview_with_display_path(
        &path,
        &path,
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(1),
        CellContentMode::Values,
    )
    .expect("read sheet preview");

    assert_eq!(
        result.data_validations,
        vec![
            SheetDataValidationSummary {
                ranges_sample: vec!["A1".to_string(), "A2".to_string()],
                range_count: 2,
                validation_type: "list".to_string(),
                operator: None,
                allow_blank: Some(true),
                dropdown_visible: Some(true),
                error_style: Some("warning".to_string()),
                show_error_message: Some(false),
                formula1: Some("\"Red,Blue\"".to_string()),
                formula2: None,
                resolved_values_source: "inline_list".to_string(),
                resolved_values_sample: vec!["Red".to_string(), "Blue".to_string()],
                resolved_values_truncated: false,
            },
            SheetDataValidationSummary {
                ranges_sample: vec!["B1".to_string()],
                range_count: 1,
                validation_type: "list".to_string(),
                operator: None,
                allow_blank: None,
                dropdown_visible: None,
                error_style: None,
                show_error_message: None,
                formula1: Some("$D$1:$D$300".to_string()),
                formula2: None,
                resolved_values_source: "same_sheet_range".to_string(),
                resolved_values_sample: (1..=128).map(|index| format!("Choice{index}")).collect(),
                resolved_values_truncated: true,
            },
            SheetDataValidationSummary {
                ranges_sample: vec!["C1".to_string()],
                range_count: 1,
                validation_type: "list".to_string(),
                operator: None,
                allow_blank: None,
                dropdown_visible: None,
                error_style: None,
                show_error_message: None,
                formula1: Some("INDIRECT(\"Choices\")".to_string()),
                formula2: None,
                resolved_values_source: "unresolved".to_string(),
                resolved_values_sample: Vec::new(),
                resolved_values_truncated: false,
            },
            SheetDataValidationSummary {
                ranges_sample: vec!["E1".to_string()],
                range_count: 1,
                validation_type: "list".to_string(),
                operator: None,
                allow_blank: None,
                dropdown_visible: None,
                error_style: None,
                show_error_message: None,
                formula1: Some("$D$1:$D$300".to_string()),
                formula2: None,
                resolved_values_source: "same_sheet_range".to_string(),
                resolved_values_sample: (1..=126).map(|index| format!("Choice{index}")).collect(),
                resolved_values_truncated: true,
            },
        ]
    );
    assert_eq!(
        result.warnings,
        vec![
            "sheet preview truncated to 1 rows and 32 columns".to_string(),
            "data validation resolved values truncated for formula: $D$1:$D$300".to_string(),
            "data validation formula could not be resolved: INDIRECT(\"Choices\")".to_string(),
            "data validation resolved values truncated for formula: $D$1:$D$300".to_string(),
        ]
    );
}

fn write_formula_inventory_fixture(path: &Path) {
    write_zip_fixture(
        path,
        &[
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                  <Default Extension="xml" ContentType="application/xml"/>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                  <Override PartName="/xl/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"/>
                  <Override PartName="/xl/sharedStrings.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"/>
                  <Override PartName="/xl/externalLinks/externalLink1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"/>
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                  <sheets>
                    <sheet name="Summary" sheetId="1" r:id="rId1"/>
                  </sheets>
                  <definedNames>
                    <definedName name="Threshold" localSheetId="0" hidden="1">Summary!$B$1</definedName>
                    <definedName name="Friendly">"A&amp;B"</definedName>
                  </definedNames>
                  <calcPr calcMode="manual" fullCalcOnLoad="1" forceFullCalc="0"/>
                </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#,
            ),
            (
                "xl/styles.xml",
                r##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <numFmts count="1">
                    <numFmt numFmtId="165" formatCode="#,##0.00"/>
                  </numFmts>
                  <cellXfs count="2">
                    <xf numFmtId="0"/>
                    <xf numFmtId="165"/>
                  </cellXfs>
                </styleSheet>"##,
            ),
            (
                "xl/sharedStrings.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <si><t>North</t></si>
                </sst>"#,
            ),
            (
                "xl/worksheets/sheet1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="1">
                      <c r="A1"><f>SUM(B1:B2)</f><v>7</v></c>
                      <c r="B1" t="s"><f t="shared" si="2" ref="B1:B2">A1&amp;&quot;x&quot;</f><v>0</v></c>
                      <c r="C1" s="1"><f>NOW()</f><v>45900</v></c>
                    </row>
                    <row r="2">
                      <c r="B2"><f t="shared" si="2"/><v>9</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
            ("xl/externalLinks/externalLink1.xml", "<externalLink/>"),
        ],
    );
}

#[test]
fn inspect_sheet_formulas_reports_bounded_formula_inventory() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("formulas.xlsx");
    write_formula_inventory_fixture(&path);

    let result = crate::formula_inspect::inspect_sheet_formulas_with_display_path(
        &path,
        Path::new("formulas.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect sheet formulas");

    assert_eq!(
        result,
        InspectSheetFormulasResult {
            path: "formulas.xlsx".to_string(),
            sheet: SheetPreview {
                name: "Summary".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            max_formulas_applied: 10,
            formulas: vec![
                SheetFormulaSummary {
                    reference: "A1".to_string(),
                    formula: "SUM(B1:B2)".to_string(),
                    cached_value: Some("7".to_string()),
                    warnings: Vec::new(),
                    formula_type: None,
                    shared_index: None,
                    shared_range: None,
                    style_index: None,
                    number_format_id: None,
                    number_format_code: None,
                },
                SheetFormulaSummary {
                    reference: "B1".to_string(),
                    formula: "A1&\"x\"".to_string(),
                    cached_value: Some("North".to_string()),
                    warnings: Vec::new(),
                    formula_type: Some("shared".to_string()),
                    shared_index: Some(2),
                    shared_range: Some("B1:B2".to_string()),
                    style_index: None,
                    number_format_id: None,
                    number_format_code: None,
                },
                SheetFormulaSummary {
                    reference: "C1".to_string(),
                    formula: "NOW()".to_string(),
                    cached_value: Some("45900".to_string()),
                    warnings: vec!["uses_volatile_function".to_string()],
                    formula_type: None,
                    shared_index: None,
                    shared_range: None,
                    style_index: Some(1),
                    number_format_id: Some(165),
                    number_format_code: Some("#,##0.00".to_string()),
                },
                SheetFormulaSummary {
                    reference: "B2".to_string(),
                    formula: String::new(),
                    cached_value: Some("9".to_string()),
                    warnings: Vec::new(),
                    formula_type: Some("shared".to_string()),
                    shared_index: Some(2),
                    shared_range: None,
                    style_index: None,
                    number_format_id: None,
                    number_format_code: None,
                },
            ],
            calculation_mode: Some("manual".to_string()),
            full_calc_on_load: Some(true),
            force_full_calc: Some(false),
            defined_names: vec![
                DefinedNameSummary {
                    name: "Threshold".to_string(),
                    sheet_scope: Some("Summary".to_string()),
                    local_sheet_id: Some(0),
                    hidden: Some(true),
                    target: "Summary!$B$1".to_string(),
                    truncated: false,
                },
                DefinedNameSummary {
                    name: "Friendly".to_string(),
                    sheet_scope: None,
                    local_sheet_id: None,
                    hidden: None,
                    target: "\"A&B\"".to_string(),
                    truncated: false,
                },
            ],
            defined_names_sample: vec![
                "Threshold=Summary!$B$1".to_string(),
                "Friendly=\"A&B\"".to_string(),
            ],
            has_external_links: true,
            truncated: false,
            warnings: Vec::new(),
        }
    );
}

#[test]
fn inspect_sheet_formulas_reports_lexical_risk_warnings() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("risky_formulas.xlsx");
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                  <Default Extension="xml" ContentType="application/xml"/>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                  <sheets>
                    <sheet name="Summary" sheetId="1" r:id="rId1"/>
                  </sheets>
                </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#,
            ),
            (
                "xl/worksheets/sheet1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="1">
                      <c r="A1"><f>INDIRECT("A1")</f><v>1</v></c>
                      <c r="B1"><f>OFFSET(A1,1,1)</f><v>2</v></c>
                      <c r="C1"><f>[Book2.xlsx]Sheet1!A1</f><v>3</v></c>
                      <c r="D1"><f>FILTER(A1:A3,A1:A3&gt;0)#</f><v>4</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );

    let result = crate::formula_inspect::inspect_sheet_formulas_with_display_path(
        &path,
        Path::new("risky_formulas.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect sheet formulas");

    assert_eq!(
        result
            .formulas
            .iter()
            .map(|formula| formula.warnings.clone())
            .collect::<Vec<_>>(),
        vec![
            vec!["uses_indirect_reference".to_string()],
            vec!["uses_offset_reference".to_string()],
            vec!["references_external_workbook_or_url".to_string()],
            vec!["uses_dynamic_array_or_spill_marker".to_string()],
        ]
    );
}

#[test]
fn inspect_sheet_formulas_truncates_at_requested_limit() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("formulas.xlsx");
    write_formula_inventory_fixture(&path);

    let result = crate::formula_inspect::inspect_sheet_formulas_with_display_path(
        &path,
        Path::new("formulas.xlsx"),
        &SheetSelector::Index { index: 0 },
        Some(2),
    )
    .expect("inspect sheet formulas");

    assert_eq!(result.max_formulas_applied, 2);
    assert_eq!(result.formulas.len(), 2);
    assert_eq!(result.truncated, true);
    assert_eq!(
        result.warnings,
        vec!["formula inventory truncated to 2 formulas".to_string()]
    );
}

#[test]
fn read_sheet_preview_rejects_xlsb_packages_in_stage_2() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsb");
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                  <Override PartName="/xl/workbook.bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                </Types>"#,
            ),
            ("xl/workbook.bin", "binary-workbook"),
            ("xl/worksheets/sheet1.bin", "binary-sheet"),
        ],
    );

    let err = read_sheet_preview_with_display_path(
        &path,
        &path,
        &SheetSelector::Index { index: 0 },
        None,
        CellContentMode::Values,
    )
    .expect_err("xlsb preview should fail");

    assert_eq!(
        err.to_string(),
        "excel.read_sheet_preview supports only .xlsx and .xlsm in this stage"
    );
}

#[test]
fn export_sheet_to_csv_writes_selected_sheet() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsx");
    let output_path = dir.path().join("exports/summary.csv");
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                  <Default Extension="xml" ContentType="application/xml"/>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                  <sheets>
                    <sheet name="Summary" sheetId="1" r:id="rId1"/>
                  </sheets>
                </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#,
            ),
            (
                "xl/worksheets/sheet1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="1">
                      <c r="A1" t="inlineStr"><is><t>name</t></is></c>
                      <c r="B1" t="inlineStr"><is><t>score</t></is></c>
                    </row>
                    <row r="2">
                      <c r="A2" t="inlineStr"><is><t>Alice</t></is></c>
                      <c r="B2"><v>7</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );

    let result = export_sheet_to_csv_with_display_path(
        &path,
        &path,
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        &output_path,
        std::path::Path::new("exports/summary.csv"),
    )
    .expect("export sheet to csv");

    assert_eq!(
        result,
        ExportSheetToCsvResult {
            path: path.display().to_string(),
            sheet: SheetPreview {
                name: "Summary".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            output_csv_path: "exports/summary.csv".to_string(),
            row_count: 2,
            column_count: 2,
            truncated: false,
            warnings: Vec::new(),
        }
    );
    assert_eq!(
        std::fs::read_to_string(&output_path).expect("read exported csv"),
        "name,score\nAlice,7\n"
    );
}

#[test]
fn workbook_path_from_model_arg_rejects_unscoped_paths() {
    assert_eq!(
        function_error_message(workbook_path_from_model_arg(
            "/tmp/book.xlsx",
            std::path::Path::new("/workspace")
        )),
        "excel.inspect_workbook path must be relative and stay within the current working directory"
    );
    assert_eq!(
        function_error_message(workbook_path_from_model_arg(
            "../book.xlsx",
            std::path::Path::new("/workspace")
        )),
        "excel.inspect_workbook path must be relative and stay within the current working directory"
    );
    assert_eq!(
        function_error_message(workbook_path_from_model_arg(
            "https://example.test/book.xlsx",
            std::path::Path::new("/workspace")
        )),
        "excel.inspect_workbook path must be a local workbook path"
    );
    assert_eq!(
        function_error_message(workbook_path_from_model_arg(
            "book.csv",
            std::path::Path::new("/workspace")
        )),
        "excel.inspect_workbook path must end in .xlsx, .xlsm, or .xlsb"
    );
    assert_eq!(
        workbook_path_from_model_arg("data/book.xlsx", std::path::Path::new("/workspace"))
            .expect("relative workbook path"),
        std::path::PathBuf::from("/workspace/data/book.xlsx")
    );
}

#[test]
fn installed_extension_contributes_excel_tools() {
    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("11111111-1111-4111-8111-111111111111");

    let tool_names = registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .map(|tool| tool.tool_name())
        .collect::<Vec<_>>();

    assert_eq!(
        tool_names,
        vec![
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, READ_SHEET_PREVIEW_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_SHEET_FORMULAS_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, EXPORT_SHEET_TO_CSV_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, EXTRACT_POWERQUERY_QUERIES_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, EXTRACT_VBA_MODULES_TOOL_NAME),
            ToolName::namespaced(
                EXCEL_NAMESPACE,
                crate::powerquery_translate::TRANSLATE_POWERQUERY_TO_SQL_PREVIEW_TOOL_NAME,
            ),
            ToolName::namespaced(EXCEL_NAMESPACE, ANALYZE_VBA_ONLYOFFICE_MIGRATION_TOOL_NAME,),
            ToolName::namespaced(
                EXCEL_NAMESPACE,
                TRANSLATE_VBA_TO_ONLYOFFICE_JS_PREVIEW_TOOL_NAME,
            ),
            ToolName::namespaced(EXCEL_NAMESPACE, REVIEW_VBA_ONLYOFFICE_WORKBOOK_TOOL_NAME,),
            ToolName::namespaced(EXCEL_NAMESPACE, TRANSLATE_VBA_TO_M_PREVIEW_TOOL_NAME),
        ]
    );
}

#[test]
fn extract_vba_modules_reads_module_source_end_to_end() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsm");
    write_zip_fixture_bytes(
        &path,
        &[
            (
                "[Content_Types].xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                  <Override PartName="/xl/vbaProject.bin" ContentType="application/vnd.ms-office.vbaProject"/>
                </Types>"#
                .to_vec(),
            ),
            (
                "xl/workbook.xml",
                br#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"/>"#
                    .to_vec(),
            ),
            (
                "xl/vbaProject.bin",
                build_minimal_vba_project_bin(&[
                    (
                        "dir".to_string(),
                        compress_ovba_literal_only(&build_vba_dir_stream("MyModule", 0x04E4)),
                    ),
                    (
                        "MyModule".to_string(),
                        compress_ovba_literal_only(
                            b"Attribute VB_Name = \"MyModule\"\r\nFunction Square(x As Double) As Double\r\n  Square = x * x\r\nEnd Function\r\n",
                        ),
                    ),
                ]),
            ),
        ],
    );

    let result = crate::vba_extract::extract_vba_modules_from_workbook(&path, &path);

    assert_eq!(
        result,
        ExtractVbaModulesResult {
            mode: "read_only_extraction".to_string(),
            path: path.display().to_string(),
            has_vba_project: true,
            code_page: Some(1252),
            module_count: 1,
            modules: vec![ExtractedVbaModule {
                name: "MyModule".to_string(),
                stream_name: "MyModule".to_string(),
                module_kind: VbaModuleKind::Procedural,
                text_offset: 0,
                read_only: false,
                private: false,
                doc_string: String::new(),
                source: "Attribute VB_Name = \"MyModule\"\r\nFunction Square(x As Double) As Double\r\n  Square = x * x\r\nEnd Function\r\n".to_string(),
                source_truncated: false,
            }],
            warnings: Vec::new(),
        }
    );
}

#[test]
fn translate_vba_to_m_preview_translates_source_first() {
    let source = concat!(
        "Sub LoadEmployeeData()\n",
        "    Dim conn As New ADODB.Connection\n",
        "    Dim rs As New ADODB.Recordset\n",
        "    Dim sql As String\n",
        "    conn.Open \"Provider=SQLOLEDB;Data Source=SQLSERVER01;Initial Catalog=HRDatabase;Integrated Security=SSPI\"\n",
        "    sql = \"SELECT FirstName, LastName, Department, Salary \"\n",
        "    sql = sql & \"FROM Employees \"\n",
        "    sql = sql & \"WHERE IsActive = 1\"\n",
        "    rs.Open sql, conn\n",
        "End Sub"
    );

    let result = crate::vba_translate::translate_vba_to_m_preview(source, Some("sample.bas"));

    assert_eq!(
        result,
        TranslateVbaToMPreviewResult {
            mode: "heuristic_preview".to_string(),
            source_name: "sample.bas".to_string(),
            source_line_count: 10,
            source_truncated: false,
            m_queries: vec![MPreviewQuery {
                name: "LoadEmployeeData_Query".to_string(),
                source_procedure: "LoadEmployeeData".to_string(),
                native_sql: "SELECT FirstName, LastName, Department, Salary FROM Employees WHERE IsActive = 1".to_string(),
                code: concat!(
                    "let\n",
                    "    Source = Sql.Database(\"SQLSERVER01\", \"HRDatabase\"),\n",
                    "    QueryResult = Value.NativeQuery(Source, \"SELECT FirstName, LastName, Department, Salary FROM Employees WHERE IsActive = 1\"),\n",
                    "    SelectColumns = Table.SelectColumns(QueryResult, {\"FirstName\", \"LastName\", \"Department\", \"Salary\"})\n",
                    "in\n",
                    "    SelectColumns"
                ).to_string(),
            }],
            modified_vba: concat!(
                "Sub LoadEmployeeData()\n",
                "    Dim conn As New ADODB.Connection\n",
                "    Dim rs As New ADODB.Recordset\n",
                "    Dim sql As String\n",
                "    ' VBA to M preview: data-access block was moved to generated Power Query M.\n",
                "    ' VBA to M preview replaced: conn.Open \"Provider=SQLOLEDB;Data Source=SQLSERVER01;Initial Catalog=HRDatabase;Integrated Security=SSPI\"\n",
                "    ' VBA to M preview replaced: sql = \"SELECT FirstName, LastName, Department, Salary \"\n",
                "    ' VBA to M preview replaced: sql = sql & \"FROM Employees \"\n",
                "    ' VBA to M preview replaced: sql = sql & \"WHERE IsActive = 1\"\n",
                "    ' VBA to M preview replaced: rs.Open sql, conn\n",
                "End Sub\n"
            ).to_string(),
            warnings: vec![
                "Preview is heuristic and source-first; review the generated M before reuse.".to_string(),
            ],
        }
    );
}

#[test]
fn translate_vba_to_m_preview_warns_for_unsupported_patterns() {
    let source = concat!(
        "Sub BuildCommand()\n",
        "    Set cmd = CreateObject(\"ADODB.Command\")\n",
        "    cmd.Parameters.Append cmd.CreateParameter(\"@Id\", 3, 1, , 42)\n",
        "    rs.OpenRecordset \"Employees\"\n",
        "    sql = \"SELECT * \" _\n",
        "End Sub\n"
    );

    let result = crate::vba_translate::translate_vba_to_m_preview(source, Some("warn.bas"));

    assert!(
        result
            .warnings
            .contains(&"CreateObject automation is not translated in this preview.".to_string())
    );
    assert!(result.warnings.contains(
        &"Parameterized ADO command bindings are not translated in this preview.".to_string()
    ));
    assert!(
        result
            .warnings
            .contains(&"DAO OpenRecordset flows are not translated in this preview.".to_string())
    );
    assert!(
        result.warnings.contains(
            &"VBA line continuations may hide SQL assembly that this preview does not reconstruct."
                .to_string()
        )
    );
}

#[tokio::test]
async fn analyze_vba_onlyoffice_migration_tool_analyzes_supported_subset() {
    let source = concat!(
        "Sub FormatSelection()\n",
        "    ActiveCell.Value = \"Ready\"\n",
        "    Selection.Font.Bold = True\n",
        "    Selection.Font.Color = RGB(10, 20, 30)\n",
        "End Sub\n"
    );

    let dir = tempdir().expect("temp dir");
    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");
    registry.turn_input_contributors()[0]
        .contribute(
            TurnInputContext {
                turn_id: "turn-1".to_string(),
                user_input: Vec::new(),
                environments: vec![TurnInputEnvironment {
                    environment_id: "local".to_string(),
                    cwd: dir.path().to_path_buf(),
                    is_primary: true,
                }],
            },
            &session_store,
            &thread_store,
            &ExtensionData::new("turn"),
        )
        .await;

    let tool = registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .find(|tool| {
            tool.tool_name()
                == ToolName::namespaced(EXCEL_NAMESPACE, ANALYZE_VBA_ONLYOFFICE_MIGRATION_TOOL_NAME)
        })
        .expect("excel VBA analyzer tool");
    let payload = ToolPayload::Function {
        arguments: json!({
            "source_text": source,
            "source_name": "sample.bas",
        })
        .to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                ANALYZE_VBA_ONLYOFFICE_MIGRATION_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should analyze VBA");

    assert_eq!(
        serde_json::from_value::<AnalyzeVbaOnlyofficeMigrationResult>(
            output
                .post_tool_use_response("call-1", &payload)
                .expect("json response")
        )
        .expect("deserialize analyzer result"),
        AnalyzeVbaOnlyofficeMigrationResult {
            procedures: vec![AnalyzeVbaProcedureSummary {
                name: "FormatSelection".to_string(),
                kind: "sub".to_string(),
                start_line: 1,
                end_line: 5,
                supported_operation_count: 3,
                unsupported_operation_count: 0,
                requires_manual_rewrite: false,
            }],
            supported_operations: vec![
                AnalyzeVbaOperationSummary {
                    procedure: "FormatSelection".to_string(),
                    line: 2,
                    operation: "SetCellValue".to_string(),
                    target: "ActiveCell.Value".to_string(),
                    value: Some("\"Ready\"".to_string()),
                    reason: None,
                },
                AnalyzeVbaOperationSummary {
                    procedure: "FormatSelection".to_string(),
                    line: 3,
                    operation: "SetFontBold".to_string(),
                    target: "Selection.Font.Bold".to_string(),
                    value: Some("true".to_string()),
                    reason: None,
                },
                AnalyzeVbaOperationSummary {
                    procedure: "FormatSelection".to_string(),
                    line: 4,
                    operation: "SetTextColor".to_string(),
                    target: "Selection.Font.Color".to_string(),
                    value: Some("RGB(10, 20, 30)".to_string()),
                    reason: None,
                },
            ],
            unsupported_operations: Vec::new(),
            requires_manual_rewrite: false,
            warnings: Vec::new(),
            success: true,
        }
    );
}

#[test]
fn analyze_vba_onlyoffice_migration_reports_blockers() {
    let source = concat!(
        "Sub BuildCommand()\n",
        "    On Error Resume Next\n",
        "    Set cmd = CreateObject(\"ADODB.Command\")\n",
        "    cmd.Parameters.Append cmd.CreateParameter(\"@Id\", 3, 1, , 42)\n",
        "    Selection.Merge\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration(
        source,
        Some("blockers.bas"),
    );

    assert_eq!(
        result,
        AnalyzeVbaOnlyofficeMigrationResult {
            procedures: vec![AnalyzeVbaProcedureSummary {
                name: "BuildCommand".to_string(),
                kind: "sub".to_string(),
                start_line: 1,
                end_line: 6,
                supported_operation_count: 0,
                unsupported_operation_count: 4,
                requires_manual_rewrite: true,
            }],
            supported_operations: Vec::new(),
            unsupported_operations: vec![
                AnalyzeVbaOperationSummary {
                    procedure: "BuildCommand".to_string(),
                    line: 2,
                    operation: "OnError".to_string(),
                    target: "On Error Resume Next".to_string(),
                    value: None,
                    reason: Some(
                        "error handling is not supported by the first ONLYOFFICE analyzer slice."
                            .to_string(),
                    ),
                },
                AnalyzeVbaOperationSummary {
                    procedure: "BuildCommand".to_string(),
                    line: 3,
                    operation: "CreateObject".to_string(),
                    target: "Set cmd = CreateObject(\"ADODB.Command\")".to_string(),
                    value: None,
                    reason: Some(
                        "COM automation is not supported by the first ONLYOFFICE analyzer slice."
                            .to_string(),
                    ),
                },
                AnalyzeVbaOperationSummary {
                    procedure: "BuildCommand".to_string(),
                    line: 4,
                    operation: "UnsupportedStatement".to_string(),
                    target: "cmd.Parameters.Append cmd.CreateParameter(\"@Id\", 3, 1, , 42)"
                        .to_string(),
                    value: None,
                    reason: Some(
                        "unrecognized executable statements are not supported by the first ONLYOFFICE analyzer slice."
                            .to_string(),
                    ),
                },
                AnalyzeVbaOperationSummary {
                    procedure: "BuildCommand".to_string(),
                    line: 5,
                    operation: "Merge".to_string(),
                    target: "Selection.Merge".to_string(),
                    value: None,
                    reason: Some(
                        "this spreadsheet operation is deferred from the first ONLYOFFICE analyzer slice."
                            .to_string(),
                    ),
                },
            ],
            requires_manual_rewrite: true,
            warnings: vec![
                "No supported spreadsheet operations were detected in the source preview."
                    .to_string(),
            ],
            success: false,
        }
    );
}

#[test]
fn analyze_vba_onlyoffice_migration_supports_formulalocal_literal() {
    let source = concat!(
        "Sub WriteFormula()\n",
        "    Cells(2, 3).FormulaLocal = \"=SUM(A1:A3)\"\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration(
        source,
        Some("formulalocal.bas"),
    );

    assert_eq!(
        result,
        AnalyzeVbaOnlyofficeMigrationResult {
            procedures: vec![AnalyzeVbaProcedureSummary {
                name: "WriteFormula".to_string(),
                kind: "sub".to_string(),
                start_line: 1,
                end_line: 3,
                supported_operation_count: 1,
                unsupported_operation_count: 0,
                requires_manual_rewrite: false,
            }],
            supported_operations: vec![AnalyzeVbaOperationSummary {
                procedure: "WriteFormula".to_string(),
                line: 2,
                operation: "SetCellFormula".to_string(),
                target: "Cells(2, 3).FormulaLocal".to_string(),
                value: Some("\"=SUM(A1:A3)\"".to_string()),
                reason: None,
            }],
            unsupported_operations: Vec::new(),
            requires_manual_rewrite: false,
            warnings: Vec::new(),
            success: true,
        }
    );
}

#[test]
fn analyze_vba_onlyoffice_migration_supports_wrap_and_alignment_literals() {
    let source = concat!(
        "Sub FormatSelection()\n",
        "    Selection.WrapText = True\n",
        "    Selection.HorizontalAlignment = -4108\n",
        "    Selection.VerticalAlignment = \"center\"\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration(
        source,
        Some("wrap-alignment.bas"),
    );

    assert_eq!(
        result,
        AnalyzeVbaOnlyofficeMigrationResult {
            procedures: vec![AnalyzeVbaProcedureSummary {
                name: "FormatSelection".to_string(),
                kind: "sub".to_string(),
                start_line: 1,
                end_line: 5,
                supported_operation_count: 3,
                unsupported_operation_count: 0,
                requires_manual_rewrite: false,
            }],
            supported_operations: vec![
                AnalyzeVbaOperationSummary {
                    procedure: "FormatSelection".to_string(),
                    line: 2,
                    operation: "SetWrap".to_string(),
                    target: "Selection.WrapText".to_string(),
                    value: Some("true".to_string()),
                    reason: None,
                },
                AnalyzeVbaOperationSummary {
                    procedure: "FormatSelection".to_string(),
                    line: 3,
                    operation: "SetAlignHorizontal".to_string(),
                    target: "Selection.HorizontalAlignment".to_string(),
                    value: Some("-4108".to_string()),
                    reason: None,
                },
                AnalyzeVbaOperationSummary {
                    procedure: "FormatSelection".to_string(),
                    line: 4,
                    operation: "SetAlignVertical".to_string(),
                    target: "Selection.VerticalAlignment".to_string(),
                    value: Some("\"center\"".to_string()),
                    reason: None,
                },
            ],
            unsupported_operations: Vec::new(),
            requires_manual_rewrite: false,
            warnings: Vec::new(),
            success: true,
        }
    );
}

#[test]
fn analyze_vba_onlyoffice_migration_keeps_formulalocal_variable_rhs_fail_closed() {
    let source = concat!(
        "Sub AddComment()\n",
        "    Cells(r, erc).FormulaLocal = er\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration(
        source,
        Some("formulalocal-variable.bas"),
    );

    assert_eq!(
        result,
        AnalyzeVbaOnlyofficeMigrationResult {
            procedures: vec![AnalyzeVbaProcedureSummary {
                name: "AddComment".to_string(),
                kind: "sub".to_string(),
                start_line: 1,
                end_line: 3,
                supported_operation_count: 0,
                unsupported_operation_count: 1,
                requires_manual_rewrite: true,
            }],
            supported_operations: Vec::new(),
            unsupported_operations: vec![AnalyzeVbaOperationSummary {
                procedure: "AddComment".to_string(),
                line: 2,
                operation: "SetCellFormula".to_string(),
                target: "Cells(r, erc).FormulaLocal".to_string(),
                value: None,
                reason: Some(
                    "cell formulas must use a string literal that starts with =.".to_string(),
                ),
            }],
            requires_manual_rewrite: true,
            warnings: vec![
                "No supported spreadsheet operations were detected in the source preview."
                    .to_string(),
            ],
            success: false,
        }
    );
}

#[test]
fn analyze_vba_onlyoffice_migration_redacts_secret_literals() {
    let source = concat!(
        "Sub Secrets()\n",
        "    Selection.Font.Name = \"C:\\Users\\alice\\Desktop\\secret-font.ttf\"\n",
        "    Selection.Font.Size = 12\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration(
        source,
        Some("secrets.bas"),
    );
    let serialized = serde_json::to_string(&result).expect("serialize analyzer result");

    assert!(serialized.contains("<redacted>"));
    assert!(!serialized.contains("C:\\Users\\alice\\Desktop\\secret-font.ttf"));
    assert_eq!(
        result.supported_operations,
        vec![
            AnalyzeVbaOperationSummary {
                procedure: "Secrets".to_string(),
                line: 2,
                operation: "SetFontName".to_string(),
                target: "Selection.Font.Name".to_string(),
                value: Some("<redacted>".to_string()),
                reason: None,
            },
            AnalyzeVbaOperationSummary {
                procedure: "Secrets".to_string(),
                line: 3,
                operation: "SetFontSize".to_string(),
                target: "Selection.Font.Size".to_string(),
                value: Some("12".to_string()),
                reason: None,
            },
        ]
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("redacted"))
    );
}

#[test]
fn analyze_vba_onlyoffice_migration_caps_procedures_and_operations() {
    let mut source = String::new();
    for index in 0..18 {
        source.push_str(&format!(
            "Sub Proc{index}()\n    ActiveCell.Value = \"Value{index}\"\n    Selection.Font.Bold = True\n    Selection.Font.Color = RGB({index}, {index}, {index})\nEnd Sub\n"
        ));
    }

    let result = crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration(&source, None);

    assert_eq!(result.procedures.len(), 16);
    assert_eq!(result.supported_operations.len(), 32);
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("procedure analysis truncated"))
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("supported operations truncated"))
    );
    assert!(!result.success);
    assert!(result.requires_manual_rewrite);
}

#[test]
fn analyze_vba_onlyoffice_migration_marks_workbook_open_as_event_sub() {
    let source = concat!(
        "Private Sub Workbook_Open()\n",
        "    Sheets(\"Lookup\").Visible = xlHidden\n",
        "    ActiveSheet.Protect Password:=\"REDACTED_PASSWORD\"\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration(
        source,
        Some("workbook_open.bas"),
    );

    assert_eq!(
        result.procedures,
        vec![AnalyzeVbaProcedureSummary {
            name: "Workbook_Open".to_string(),
            kind: "event_sub".to_string(),
            start_line: 1,
            end_line: 4,
            supported_operation_count: 0,
            unsupported_operation_count: 2,
            requires_manual_rewrite: true,
        }]
    );
    assert_eq!(
        result.unsupported_operations,
        vec![
            AnalyzeVbaOperationSummary {
                procedure: "Workbook_Open".to_string(),
                line: 1,
                operation: "EventProcedure".to_string(),
                target: "Private Sub Workbook_Open()".to_string(),
                value: None,
                reason: Some(
                    "event procedures are not supported by the first ONLYOFFICE analyzer slice."
                        .to_string(),
                ),
            },
            AnalyzeVbaOperationSummary {
                procedure: "Workbook_Open".to_string(),
                line: 2,
                operation: "UnsupportedStatement".to_string(),
                target: "Sheets(\"Lookup\").Visible = xlHidden".to_string(),
                value: None,
                reason: Some(
                    "unrecognized executable statements are not supported by the first ONLYOFFICE analyzer slice."
                        .to_string(),
                ),
            },
            AnalyzeVbaOperationSummary {
                procedure: "Workbook_Open".to_string(),
                line: 3,
                operation: "Protect".to_string(),
                target: "ActiveSheet.Protect Password:=<redacted>".to_string(),
                value: None,
                reason: Some(
                    "this spreadsheet operation is deferred from the first ONLYOFFICE analyzer slice."
                        .to_string(),
                ),
            },
        ]
    );
    assert!(!result.success);
    assert!(result.requires_manual_rewrite);
}

#[test]
fn analyze_vba_onlyoffice_migration_fails_closed_for_environment_reads() {
    let source = concat!(
        "Sub GreetUser()\n",
        "    userName = Environ(\"UserName\")\n",
        "    ActiveCell.Value = \"Ready\"\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration(
        source,
        Some("environment.bas"),
    );

    assert_eq!(
        result.procedures,
        vec![AnalyzeVbaProcedureSummary {
            name: "GreetUser".to_string(),
            kind: "sub".to_string(),
            start_line: 1,
            end_line: 4,
            supported_operation_count: 1,
            unsupported_operation_count: 1,
            requires_manual_rewrite: true,
        }]
    );
    assert_eq!(
        result.supported_operations,
        vec![AnalyzeVbaOperationSummary {
            procedure: "GreetUser".to_string(),
            line: 3,
            operation: "SetCellValue".to_string(),
            target: "ActiveCell.Value".to_string(),
            value: Some("\"Ready\"".to_string()),
            reason: None,
        }]
    );
    assert_eq!(
        result.unsupported_operations,
        vec![AnalyzeVbaOperationSummary {
            procedure: "GreetUser".to_string(),
            line: 2,
            operation: "EnvironmentRead".to_string(),
            target: "userName = Environ(\"UserName\")".to_string(),
            value: None,
            reason: Some(
                "environment reads are not supported by the first ONLYOFFICE analyzer slice."
                    .to_string(),
            ),
        }]
    );
    assert!(!result.success);
    assert!(result.requires_manual_rewrite);
}

#[test]
fn translate_vba_to_onlyoffice_js_preview_supports_line_continuations() {
    let source = concat!(
        "Sub FillCell()\n",
        "    ActiveCell.Value = _\n",
        "        \"Ready\"\n",
        "    ActiveCell.Formula = _\n",
        "        \"=SUM(A1:A3)\"\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(
        source,
        Some("continuation.bas"),
    );

    assert!(result.success);
    assert_eq!(
        result.unsupported_operations,
        Vec::<AnalyzeVbaOperationSummary>::new()
    );
    assert_eq!(
        result.procedure_summaries,
        vec![AnalyzeVbaProcedureSummary {
            name: "FillCell".to_string(),
            kind: "sub".to_string(),
            start_line: 1,
            end_line: 6,
            supported_operation_count: 2,
            unsupported_operation_count: 0,
            requires_manual_rewrite: false,
        }]
    );
    assert!(
        result
            .macro_value
            .contains("worksheet.GetActiveCell().SetValue(\"Ready\");")
    );
    assert!(
        result
            .macro_value
            .contains("worksheet.GetActiveCell().SetFormulaArray(\"=SUM(A1:A3)\");")
    );
}

#[test]
fn translate_vba_to_onlyoffice_js_preview_emits_value_and_formula() {
    let source = concat!(
        "Sub FillCell()\n",
        "    ActiveCell.Value = \"Ready\"\n",
        "    ActiveCell.Formula = \"=SUM(A1:A3)\"\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(
        source,
        Some("fill.bas"),
    );

    assert_eq!(
        result,
        TranslateVbaToOnlyofficeJsPreviewResult {
            macro_value: concat!(
                "(function()\n",
                "{\n",
                "    let worksheet = Api.GetActiveSheet();\n",
                "    let workbook = Api.GetActiveWorkbook();\n",
                "    worksheet.GetActiveCell().SetValue(\"Ready\");\n",
                "    worksheet.GetActiveCell().SetFormulaArray(\"=SUM(A1:A3)\");\n",
                "})();"
            )
            .to_string(),
            function_body: concat!(
                "    let worksheet = Api.GetActiveSheet();\n",
                "    let workbook = Api.GetActiveWorkbook();\n",
                "    worksheet.GetActiveCell().SetValue(\"Ready\");\n",
                "    worksheet.GetActiveCell().SetFormulaArray(\"=SUM(A1:A3)\");"
            )
            .to_string(),
            procedure_summaries: vec![AnalyzeVbaProcedureSummary {
                name: "FillCell".to_string(),
                kind: "sub".to_string(),
                start_line: 1,
                end_line: 4,
                supported_operation_count: 2,
                unsupported_operation_count: 0,
                requires_manual_rewrite: false,
            }],
            unsupported_operations: Vec::new(),
            redactions: Vec::new(),
            warnings: Vec::new(),
            success: true,
        }
    );
}

#[test]
fn translate_vba_to_onlyoffice_js_preview_emits_formulalocal_literal() {
    let source = concat!(
        "Sub WriteFormula()\n",
        "    Cells(2, 3).FormulaLocal = \"=SUM(A1:A3)\"\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(
        source,
        Some("formulalocal.bas"),
    );

    assert_eq!(
        result.function_body,
        concat!(
            "    let worksheet = Api.GetActiveSheet();\n",
            "    let workbook = Api.GetActiveWorkbook();\n",
            "    worksheet.GetActiveCell().SetFormulaArray(\"=SUM(A1:A3)\");"
        )
    );
    assert!(result.success);
    assert_eq!(result.warnings, Vec::<String>::new());
}

#[test]
fn translate_vba_to_onlyoffice_js_preview_emits_formatting_with_rgb() {
    let source = concat!(
        "Sub PaintSelection()\n",
        "    Selection.Font.Bold = True\n",
        "    Selection.Font.Italic = False\n",
        "    Selection.Font.Name = \"Arial\"\n",
        "    Selection.Font.Size = 12\n",
        "    Selection.Font.Color = RGB(10, 20, 30)\n",
        "    Selection.Interior.Color = RGB(200, 210, 220)\n",
        "    Selection.NumberFormat = \"0.00\"\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(
        source,
        Some("format.bas"),
    );

    assert_eq!(
        result.macro_value,
        concat!(
            "(function()\n",
            "{\n",
            "    let worksheet = Api.GetActiveSheet();\n",
            "    let workbook = Api.GetActiveWorkbook();\n",
            "    Api.GetSelection().SetBold(true);\n",
            "    Api.GetSelection().SetItalic(false);\n",
            "    Api.GetSelection().SetFontName(\"Arial\");\n",
            "    Api.GetSelection().SetFontSize(\"12\");\n",
            "    Api.GetSelection().SetFontColor(Api.CreateColorFromRGB(10, 20, 30));\n",
            "    Api.GetSelection().SetBackgroundColor(Api.CreateColorFromRGB(200, 210, 220));\n",
            "    Api.GetSelection().SetNumberFormat(\"0.00\");\n",
            "})();"
        )
    );
    assert!(result.success);
    assert_eq!(result.redactions, Vec::<String>::new());
    assert_eq!(result.warnings, Vec::<String>::new());
}

#[test]
fn translate_vba_to_onlyoffice_js_preview_emits_wrap_and_alignment_literals() {
    let source = concat!(
        "Sub FormatSelection()\n",
        "    Selection.WrapText = True\n",
        "    Selection.HorizontalAlignment = -4108\n",
        "    Selection.VerticalAlignment = \"center\"\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(
        source,
        Some("wrap-alignment.bas"),
    );

    assert_eq!(
        result.macro_value,
        concat!(
            "(function()\n",
            "{\n",
            "    let worksheet = Api.GetActiveSheet();\n",
            "    let workbook = Api.GetActiveWorkbook();\n",
            "    Api.GetSelection().SetWrap(true);\n",
            "    Api.GetSelection().SetAlignHorizontal(-4108);\n",
            "    Api.GetSelection().SetAlignVertical(\"center\");\n",
            "})();"
        )
    );
    assert_eq!(
        result.function_body,
        concat!(
            "    let worksheet = Api.GetActiveSheet();\n",
            "    let workbook = Api.GetActiveWorkbook();\n",
            "    Api.GetSelection().SetWrap(true);\n",
            "    Api.GetSelection().SetAlignHorizontal(-4108);\n",
            "    Api.GetSelection().SetAlignVertical(\"center\");"
        )
    );
    assert!(result.success);
    assert_eq!(result.redactions, Vec::<String>::new());
    assert_eq!(result.warnings, Vec::<String>::new());
}

#[test]
fn translate_vba_to_onlyoffice_js_preview_fails_closed_for_unsupported_constructs() {
    let source = concat!(
        "Sub BuildCommand()\n",
        "    On Error Resume Next\n",
        "    Selection.Merge\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(
        source,
        Some("blockers.bas"),
    );

    assert!(!result.success);
    assert_eq!(result.macro_value, "");
    assert_eq!(result.function_body, "");
    assert_eq!(result.unsupported_operations.len(), 2);
}

#[test]
fn analyze_and_translate_vba_onlyoffice_js_preview_fail_closed_for_msgbox() {
    let source = concat!("Sub WarnUser()\n", "    MsgBox \"Hello\"\n", "End Sub\n");

    let analysis =
        crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration(source, Some("msgbox.bas"));

    assert_eq!(
        analysis,
        AnalyzeVbaOnlyofficeMigrationResult {
            procedures: vec![AnalyzeVbaProcedureSummary {
                name: "WarnUser".to_string(),
                kind: "sub".to_string(),
                start_line: 1,
                end_line: 3,
                supported_operation_count: 0,
                unsupported_operation_count: 1,
                requires_manual_rewrite: true,
            }],
            supported_operations: Vec::new(),
            unsupported_operations: vec![AnalyzeVbaOperationSummary {
                procedure: "WarnUser".to_string(),
                line: 2,
                operation: "UnsupportedStatement".to_string(),
                target: "MsgBox \"Hello\"".to_string(),
                value: None,
                reason: Some(
                    "unrecognized executable statements are not supported by the first ONLYOFFICE analyzer slice."
                        .to_string(),
                ),
            }],
            requires_manual_rewrite: true,
            warnings: vec![
                "No supported spreadsheet operations were detected in the source preview."
                    .to_string(),
            ],
            success: false,
        }
    );

    let result = crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(
        source,
        Some("msgbox.bas"),
    );

    assert!(!result.success);
    assert_eq!(result.macro_value, "");
    assert_eq!(result.function_body, "");
}

#[test]
fn analyze_and_translate_vba_onlyoffice_js_preview_fail_closed_for_alignment_constants() {
    let source = concat!(
        "Sub AlignSelection()\n",
        "    Selection.HorizontalAlignment = xlHAlignCenter\n",
        "End Sub\n"
    );

    let analysis = crate::vba_onlyoffice_analyze::analyze_vba_onlyoffice_migration(
        source,
        Some("alignment.bas"),
    );

    assert_eq!(
        analysis,
        AnalyzeVbaOnlyofficeMigrationResult {
            procedures: vec![AnalyzeVbaProcedureSummary {
                name: "AlignSelection".to_string(),
                kind: "sub".to_string(),
                start_line: 1,
                end_line: 3,
                supported_operation_count: 0,
                unsupported_operation_count: 1,
                requires_manual_rewrite: true,
            }],
            supported_operations: Vec::new(),
            unsupported_operations: vec![AnalyzeVbaOperationSummary {
                procedure: "AlignSelection".to_string(),
                line: 2,
                operation: "SetAlignHorizontal".to_string(),
                target: "Selection.HorizontalAlignment".to_string(),
                value: None,
                reason: Some("alignment expects a numeric literal or quoted string.".to_string(),),
            }],
            requires_manual_rewrite: true,
            warnings: vec![
                "No supported spreadsheet operations were detected in the source preview."
                    .to_string(),
            ],
            success: false,
        }
    );

    let result = crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(
        source,
        Some("alignment.bas"),
    );

    assert!(!result.success);
    assert_eq!(result.macro_value, "");
    assert_eq!(result.function_body, "");
}

#[test]
fn translate_vba_to_onlyoffice_js_preview_fails_closed_for_truncation() {
    let source = format!(
        "Sub Huge()\n    ActiveCell.Value = \"{}\"\nEnd Sub\n",
        "A".repeat(33_000)
    );

    let result =
        crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(&source, None);

    assert!(!result.success);
    assert_eq!(result.macro_value, "");
    assert_eq!(result.function_body, "");
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("truncated"))
    );
}

#[test]
fn translate_vba_to_onlyoffice_js_preview_fails_closed_for_redaction() {
    let source = concat!(
        "Sub Secrets()\n",
        "    Selection.Font.Name = \"C:\\Users\\alice\\Desktop\\secret-font.ttf\"\n",
        "End Sub\n"
    );

    let result = crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(
        source,
        Some("secrets.bas"),
    );

    assert!(!result.success);
    assert_eq!(result.macro_value, "");
    assert_eq!(result.function_body, "");
    assert_eq!(result.redactions, vec!["Secrets:2".to_string()]);
}

#[test]
fn translate_vba_to_onlyoffice_js_preview_fails_closed_for_unknown_operation() {
    let analysis = AnalyzeVbaOnlyofficeMigrationResult {
        procedures: vec![AnalyzeVbaProcedureSummary {
            name: "Unknown".to_string(),
            kind: "sub".to_string(),
            start_line: 1,
            end_line: 3,
            supported_operation_count: 1,
            unsupported_operation_count: 0,
            requires_manual_rewrite: false,
        }],
        supported_operations: vec![AnalyzeVbaOperationSummary {
            procedure: "Unknown".to_string(),
            line: 2,
            operation: "SetBorder".to_string(),
            target: "Selection.Borders".to_string(),
            value: Some("true".to_string()),
            reason: None,
        }],
        unsupported_operations: Vec::new(),
        requires_manual_rewrite: false,
        warnings: Vec::new(),
        success: true,
    };

    let result =
        crate::vba_onlyoffice_translate::translate_analyzed_vba_to_onlyoffice_js_preview(analysis);

    assert!(!result.success);
    assert_eq!(result.macro_value, "");
    assert_eq!(result.function_body, "");
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("SetBorder"))
    );
}

#[test]
fn translate_vba_to_onlyoffice_js_preview_fails_closed_for_unmapped_value_expression() {
    let source = concat!(
        "Sub BadValue()\n",
        "    ActiveCell.Value = RGB(1, 2, 3)\n",
        "End Sub\n"
    );

    let result =
        crate::vba_onlyoffice_translate::translate_vba_to_onlyoffice_js_preview(source, None);

    assert!(!result.success);
    assert_eq!(result.macro_value, "");
    assert_eq!(result.function_body, "");
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("SetCellValue"))
    );
}

#[test]
fn extract_vba_modules_uses_display_path_in_open_errors() {
    let missing_path = std::path::Path::new("/tmp/private/workbook.xlsm");
    let result = crate::vba_extract::extract_vba_modules_from_workbook(
        missing_path,
        std::path::Path::new("workbooks/example.xlsm"),
    );

    assert_eq!(result.has_vba_project, false);
    assert_eq!(result.warnings.len(), 1);
    assert!(result.warnings[0].contains("workbooks/example.xlsm"));
    assert!(!result.warnings[0].contains("/tmp/private/workbook.xlsm"));
}

#[tokio::test]
async fn extract_vba_modules_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/sample.xlsm");
    write_zip_fixture_bytes(
        &workbook_path,
        &[
            (
                "[Content_Types].xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                  <Override PartName="/xl/vbaProject.bin" ContentType="application/vnd.ms-office.vbaProject"/>
                </Types>"#
                .to_vec(),
            ),
            (
                "xl/vbaProject.bin",
                build_minimal_vba_project_bin(&[
                    (
                        "dir".to_string(),
                        compress_ovba_literal_only(&build_vba_dir_stream("MyModule", 0x04E4)),
                    ),
                    (
                        "MyModule".to_string(),
                        compress_ovba_literal_only(
                            b"Attribute VB_Name = \"MyModule\"\r\nFunction Square(x As Double) As Double\r\n  Square = x * x\r\nEnd Function\r\n",
                        ),
                    ),
                ]),
            ),
        ],
    );

    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");
    registry.turn_input_contributors()[0]
        .contribute(
            TurnInputContext {
                turn_id: "turn-1".to_string(),
                user_input: Vec::new(),
                environments: vec![TurnInputEnvironment {
                    environment_id: "local".to_string(),
                    cwd: workspace.clone(),
                    is_primary: true,
                }],
            },
            &session_store,
            &thread_store,
            &ExtensionData::new("turn"),
        )
        .await;

    let tool = registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .find(|tool| {
            tool.tool_name() == ToolName::namespaced(EXCEL_NAMESPACE, EXTRACT_VBA_MODULES_TOOL_NAME)
        })
        .expect("excel VBA extractor tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "data/sample.xlsm" }).to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(EXCEL_NAMESPACE, EXTRACT_VBA_MODULES_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should extract VBA");

    assert_eq!(
        serde_json::from_value::<ExtractVbaModulesResult>(
            output
                .post_tool_use_response("call-1", &payload)
                .expect("json response")
        )
        .expect("deserialize VBA extraction result"),
        ExtractVbaModulesResult {
            mode: "read_only_extraction".to_string(),
            path: "data/sample.xlsm".to_string(),
            has_vba_project: true,
            code_page: Some(1252),
            module_count: 1,
            modules: vec![ExtractedVbaModule {
                name: "MyModule".to_string(),
                stream_name: "MyModule".to_string(),
                module_kind: VbaModuleKind::Procedural,
                text_offset: 0,
                read_only: false,
                private: false,
                doc_string: String::new(),
                source: "Attribute VB_Name = \"MyModule\"\r\nFunction Square(x As Double) As Double\r\n  Square = x * x\r\nEnd Function\r\n".to_string(),
                source_truncated: false,
            }],
            warnings: Vec::new(),
        }
    );
}

fn build_vba_dir_stream(module_name: &str, code_page: u16) -> Vec<u8> {
    let mut dir = Vec::new();
    append_fixed_u32_record(&mut dir, 0x0001, 0x0000_0001);
    append_fixed_u32_record(&mut dir, 0x0002, 0x0000_0409);
    append_fixed_u32_record(&mut dir, 0x0014, 0x0000_0409);
    append_fixed_u16_record(&mut dir, 0x0003, code_page);
    append_dir_record(&mut dir, 0x0004, b"Project1");
    append_dir_record(&mut dir, 0x0005, b"");
    append_dir_record(&mut dir, 0x0040, &[]);
    append_dir_record(&mut dir, 0x0006, b"");
    append_dir_record(&mut dir, 0x003D, &[]);
    append_fixed_u32_record(&mut dir, 0x0007, 0);
    append_fixed_u32_record(&mut dir, 0x0008, 0);
    append_version_record(&mut dir, 1, 0);
    append_fixed_u16_record(&mut dir, 0x000F, 1);
    append_fixed_u16_record(&mut dir, 0x0013, 0);

    append_dir_record(&mut dir, 0x0019, module_name.as_bytes());
    append_dir_record(&mut dir, 0x001A, module_name.as_bytes());
    append_dir_record(&mut dir, 0x0032, &utf16le_bytes(module_name));
    append_dir_record(&mut dir, 0x001C, &[]);
    append_dir_record(&mut dir, 0x0048, &[]);
    append_fixed_u32_record(&mut dir, 0x0031, 0);
    append_fixed_u32_record(&mut dir, 0x001E, 0);
    append_fixed_u16_record(&mut dir, 0x002C, 0);
    append_reserved_record(&mut dir, 0x0021);
    append_reserved_record(&mut dir, 0x002B);
    append_reserved_record(&mut dir, 0x0010);
    dir
}

fn compress_ovba_literal_only(data: &[u8]) -> Vec<u8> {
    let mut output = vec![0x01];

    let mut pos = 0usize;
    while pos < data.len() {
        let chunk_len = std::cmp::min(4096, data.len() - pos);
        let mut tokens = Vec::new();
        let mut written = 0usize;
        while written < chunk_len {
            let group_count = std::cmp::min(8, chunk_len - written);
            tokens.push(0x00);
            for index in 0..group_count {
                tokens.push(data[pos + written + index]);
            }
            written += group_count;
        }

        let size = tokens.len() + 2;
        let header = (0b011 << 12) | 0x8000 | ((size - 3) & 0x0FFF);
        output.push((header & 0xFF) as u8);
        output.push((header >> 8) as u8);
        output.extend_from_slice(&tokens);
        pos += chunk_len;
    }

    output
}

fn build_minimal_vba_project_bin(streams: &[(String, Vec<u8>)]) -> Vec<u8> {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("vbaProject.bin");
    let mut compound = cfb::create(&path).expect("create compound file");
    compound.create_storage("/VBA").expect("create VBA storage");

    for (name, bytes) in streams {
        let stream_path = format!("/VBA/{name}");
        let mut stream = compound
            .create_stream(&stream_path)
            .expect("create CFB stream");
        stream.write_all(bytes).expect("write CFB stream");
    }

    drop(compound);
    std::fs::read(path).expect("read vbaProject.bin")
}

fn append_dir_record(buffer: &mut Vec<u8>, id: u16, data: &[u8]) {
    buffer.extend_from_slice(&id.to_le_bytes());
    buffer.extend_from_slice(&(data.len() as u32).to_le_bytes());
    buffer.extend_from_slice(data);
}

fn append_fixed_u16_record(buffer: &mut Vec<u8>, id: u16, value: u16) {
    buffer.extend_from_slice(&id.to_le_bytes());
    buffer.extend_from_slice(&2u32.to_le_bytes());
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn append_fixed_u32_record(buffer: &mut Vec<u8>, id: u16, value: u32) {
    buffer.extend_from_slice(&id.to_le_bytes());
    buffer.extend_from_slice(&4u32.to_le_bytes());
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn append_reserved_record(buffer: &mut Vec<u8>, id: u16) {
    buffer.extend_from_slice(&id.to_le_bytes());
    buffer.extend_from_slice(&0u32.to_le_bytes());
}

fn append_version_record(buffer: &mut Vec<u8>, major: u32, minor: u16) {
    buffer.extend_from_slice(&0x0009u16.to_le_bytes());
    buffer.extend_from_slice(&4u32.to_le_bytes());
    buffer.extend_from_slice(&major.to_le_bytes());
    buffer.extend_from_slice(&minor.to_le_bytes());
}

fn utf16le_bytes(value: &str) -> Vec<u8> {
    value
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>()
}

#[tokio::test]
async fn inspect_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/sample.xlsx");
    write_zip_fixture(
        &workbook_path,
        &[
            (
                "[Content_Types].xml",
                r#"<Types>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets/></workbook>"#,
            ),
        ],
    );

    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");
    registry.turn_input_contributors()[0]
        .contribute(
            TurnInputContext {
                turn_id: "turn-1".to_string(),
                user_input: Vec::new(),
                environments: vec![TurnInputEnvironment {
                    environment_id: "local".to_string(),
                    cwd: workspace.clone(),
                    is_primary: true,
                }],
            },
            &session_store,
            &thread_store,
            &ExtensionData::new("turn"),
        )
        .await;

    let tool = registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .next()
        .expect("excel inspection tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "data/sample.xlsx" }).to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect workbook");

    assert_eq!(
        serde_json::from_value::<InspectWorkbookResult>(
            output
                .post_tool_use_response("call-1", &payload)
                .expect("json response")
        )
        .expect("deserialize inspection result")
        .path,
        "data/sample.xlsx"
    );
}

#[tokio::test]
async fn read_sheet_preview_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/sample.xlsx");
    write_zip_fixture(
        &workbook_path,
        &[
            (
                "[Content_Types].xml",
                r#"<Types>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Summary" sheetId="1" r:id="rId1"/></sheets></workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#,
            ),
            (
                "xl/worksheets/sheet1.xml",
                r#"<worksheet><sheetData><row r="1"><c r="A1"><v>7</v></c></row></sheetData></worksheet>"#,
            ),
        ],
    );

    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");
    registry.turn_input_contributors()[0]
        .contribute(
            TurnInputContext {
                turn_id: "turn-1".to_string(),
                user_input: Vec::new(),
                environments: vec![TurnInputEnvironment {
                    environment_id: "local".to_string(),
                    cwd: workspace.clone(),
                    is_primary: true,
                }],
            },
            &session_store,
            &thread_store,
            &ExtensionData::new("turn"),
        )
        .await;

    let tool = registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .find(|tool| {
            tool.tool_name() == ToolName::namespaced(EXCEL_NAMESPACE, READ_SHEET_PREVIEW_TOOL_NAME)
        })
        .expect("excel preview tool");
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "data/sample.xlsx",
            "sheet": { "type": "index", "index": 0 },
        })
        .to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(EXCEL_NAMESPACE, READ_SHEET_PREVIEW_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should preview workbook");

    assert_eq!(
        serde_json::from_value::<ReadSheetPreviewResult>(
            output
                .post_tool_use_response("call-1", &payload)
                .expect("json response")
        )
        .expect("deserialize preview result"),
        ReadSheetPreviewResult {
            path: "data/sample.xlsx".to_string(),
            sheet: SheetPreview {
                name: "Summary".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            dimension: None,
            max_rows_applied: 20,
            cell_content: CellContentMode::Values,
            rows: vec![SheetPreviewRow {
                row_index: 1,
                cells: vec![SheetPreviewCell {
                    reference: "A1".to_string(),
                    value: Some("7".to_string()),
                    formula: None,
                }],
            }],
            data_validations: Vec::new(),
            truncated: false,
            warnings: Vec::new(),
        }
    );
}

#[tokio::test]
async fn export_sheet_to_csv_tool_resolves_relative_paths_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/sample.xlsx");
    write_zip_fixture(
        &workbook_path,
        &[
            (
                "[Content_Types].xml",
                r#"<Types>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Summary" sheetId="1" r:id="rId1"/></sheets></workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#,
            ),
            (
                "xl/worksheets/sheet1.xml",
                r#"<worksheet><sheetData><row r="1"><c r="A1" t="inlineStr"><is><t>name</t></is></c><c r="B1" t="inlineStr"><is><t>score</t></is></c></row><row r="2"><c r="A2" t="inlineStr"><is><t>Alice</t></is></c><c r="B2"><v>7</v></c></row></sheetData></worksheet>"#,
            ),
        ],
    );

    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");
    registry.turn_input_contributors()[0]
        .contribute(
            TurnInputContext {
                turn_id: "turn-1".to_string(),
                user_input: Vec::new(),
                environments: vec![TurnInputEnvironment {
                    environment_id: "local".to_string(),
                    cwd: workspace.clone(),
                    is_primary: true,
                }],
            },
            &session_store,
            &thread_store,
            &ExtensionData::new("turn"),
        )
        .await;

    let tool = registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .find(|tool| {
            tool.tool_name() == ToolName::namespaced(EXCEL_NAMESPACE, EXPORT_SHEET_TO_CSV_TOOL_NAME)
        })
        .expect("excel export tool");
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "data/sample.xlsx",
            "sheet": { "type": "index", "index": 0 },
            "output_csv_path": "exports/report.csv",
        })
        .to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(EXCEL_NAMESPACE, EXPORT_SHEET_TO_CSV_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should export workbook");

    assert_eq!(
        serde_json::from_value::<ExportSheetToCsvResult>(
            output
                .post_tool_use_response("call-1", &payload)
                .expect("json response")
        )
        .expect("deserialize export result"),
        ExportSheetToCsvResult {
            path: "data/sample.xlsx".to_string(),
            sheet: SheetPreview {
                name: "Summary".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            output_csv_path: "exports/report.csv".to_string(),
            row_count: 2,
            column_count: 2,
            truncated: false,
            warnings: Vec::new(),
        }
    );
    assert_eq!(
        std::fs::read_to_string(workspace.join("exports/report.csv")).expect("read exported csv"),
        "name,score\nAlice,7\n"
    );
}

#[test]
fn extract_powerquery_queries_reads_data_mashup_queries() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsm");
    write_zip_fixture_bytes(
        &path,
        &[
            (
                "[Content_Types].xml",
                br#"<Types>
                    <Override PartName="/xl/workbook.xml" ContentType="application/vnd.ms-excel.sheet.macroEnabled.main+xml"/>
                  </Types>"#
                    .to_vec(),
            ),
            (
                "xl/workbook.xml",
                br#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets/></workbook>"#
                    .to_vec(),
            ),
            (
                "xl/connections.xml",
                br#"<connections xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                    <connection name="Query - SalesQuery">
                      <dbPr connection="Provider=Microsoft.Mashup.OleDb.1;Data Source=$Workbook$;Location=&quot;SalesQuery&quot;" command="SELECT * FROM [SalesQuery]"/>
                    </connection>
                  </connections>"#
                    .to_vec(),
            ),
            (
                "customXml/item1.xml",
                build_data_mashup_xml_utf16(&[(
                    "Formulas/Section1.m",
                    concat!(
                        "section Section1;\n",
                        "shared SalesQuery = let\n",
                        "    Source = Sql.Database(\"demo\", \"warehouse\"),\n",
                        "    dbo_Sales = Source{[Schema=\"dbo\",Item=\"Sales\"]}[Data],\n",
                        "    SelectedColumns = Table.SelectColumns(dbo_Sales, {\"OrderId\", \"Amount\"})\n",
                        "in\n",
                        "    SelectedColumns;\n",
                    ),
                )]),
            ),
        ],
    );

    let result = crate::powerquery_extract::extract_powerquery_queries_from_workbook(&path, &path);

    assert_eq!(
        result,
        ExtractPowerQueryQueriesResult {
            mode: "read_only_extraction".to_string(),
            path: path.display().to_string(),
            has_power_query: true,
            query_count: 1,
            queries: vec![ExtractedPowerQueryQuery {
                name: "SalesQuery".to_string(),
                source_part: "Formulas/Section1.m".to_string(),
                source: concat!(
                    "shared SalesQuery = let\n",
                    "    Source = Sql.Database(\"demo\", \"warehouse\"),\n",
                    "    dbo_Sales = Source{[Schema=\"dbo\",Item=\"Sales\"]}[Data],\n",
                    "    SelectedColumns = Table.SelectColumns(dbo_Sales, {\"OrderId\", \"Amount\"})\n",
                    "in\n",
                    "    SelectedColumns;"
                )
                .to_string(),
                source_truncated: false,
                connection_name: Some("Query - SalesQuery".to_string()),
                location: Some("SalesQuery".to_string()),
                command_preview: Some("SELECT * FROM [SalesQuery]".to_string()),
            }],
            warnings: Vec::new(),
        }
    );
}

#[test]
fn extract_powerquery_queries_keeps_powerquery_marker_on_corrupted_payload() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("broken.xlsm");
    write_zip_fixture(
        &path,
        &[
            (
                "[Content_Types].xml",
                r#"<Types>
                    <Override PartName="/xl/workbook.xml" ContentType="application/vnd.ms-excel.sheet.macroEnabled.main+xml"/>
                  </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets/></workbook>"#,
            ),
            ("customXml/item1.xml", "<DataMashup>not-base64</DataMashup>"),
        ],
    );

    let result = crate::powerquery_extract::extract_powerquery_queries_from_workbook(&path, &path);

    assert_eq!(result.has_power_query, true);
    assert_eq!(result.query_count, 0);
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("failed to decode DataMashup payload"))
    );
}

#[tokio::test]
async fn extract_powerquery_queries_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/sample.xlsm");
    write_zip_fixture_bytes(
        &workbook_path,
        &[
            (
                "[Content_Types].xml",
                br#"<Types>
                    <Override PartName="/xl/workbook.xml" ContentType="application/vnd.ms-excel.sheet.macroEnabled.main+xml"/>
                  </Types>"#
                    .to_vec(),
            ),
            (
                "xl/workbook.xml",
                br#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets/></workbook>"#
                    .to_vec(),
            ),
            (
                "customXml/item1.xml",
                build_data_mashup_xml_utf16(&[(
                    "Formulas/Section1.m",
                    concat!(
                        "section Section1;\n",
                        "shared NativeQuery = let\n",
                        "    Source = Sql.Database(\"demo\", \"warehouse\"),\n",
                        "    QueryResult = Value.NativeQuery(Source, \"SELECT 1\")\n",
                        "in\n",
                        "    QueryResult;\n",
                    ),
                )]),
            ),
        ],
    );

    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");
    registry.turn_input_contributors()[0]
        .contribute(
            TurnInputContext {
                turn_id: "turn-1".to_string(),
                user_input: Vec::new(),
                environments: vec![TurnInputEnvironment {
                    environment_id: "local".to_string(),
                    cwd: workspace.clone(),
                    is_primary: true,
                }],
            },
            &session_store,
            &thread_store,
            &ExtensionData::new("turn"),
        )
        .await;

    let tool = registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .find(|tool| {
            tool.tool_name()
                == ToolName::namespaced(EXCEL_NAMESPACE, EXTRACT_POWERQUERY_QUERIES_TOOL_NAME)
        })
        .expect("excel Power Query extractor tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "data/sample.xlsm" }).to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(EXCEL_NAMESPACE, EXTRACT_POWERQUERY_QUERIES_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should extract Power Query");

    assert_eq!(
        serde_json::from_value::<ExtractPowerQueryQueriesResult>(
            output
                .post_tool_use_response("call-1", &payload)
                .expect("json response")
        )
        .expect("deserialize extraction result")
        .path,
        "data/sample.xlsm".to_string()
    );
}

#[test]
fn translate_powerquery_to_sql_preview_uses_native_query_sql() {
    let source = concat!(
        "section Section1;\n",
        "shared SalesQuery = let\n",
        "    Source = Sql.Database(\"server\", \"warehouse\"),\n",
        "    QueryResult = Value.NativeQuery(Source, \"SELECT OrderId, Amount FROM Sales WHERE Amount > 100\")\n",
        "in\n",
        "    QueryResult;\n",
    );

    let result = crate::powerquery_translate::translate_powerquery_to_sql_preview(
        source,
        Some("Section1.m"),
    );

    assert_eq!(
        result,
        TranslatePowerQueryToSqlPreviewResult {
            mode: "heuristic_preview".to_string(),
            source_name: "Section1.m".to_string(),
            source_line_count: 6,
            source_truncated: false,
            success: true,
            sql: "SELECT OrderId, Amount FROM Sales WHERE Amount > 100".to_string(),
            unsupported_functions: Vec::new(),
            warnings: vec![
                "SQL preview was derived directly from Value.NativeQuery().".to_string(),
                "Preview is heuristic and source-first; review the generated SQL before execution."
                    .to_string(),
            ],
        }
    );
}

#[test]
fn translate_powerquery_to_sql_preview_translates_basic_table_pipeline() {
    let source = concat!(
        "shared DimDate = let\n",
        "    Source = Sql.Database(\"server\", \"warehouse\"),\n",
        "    dbo_DimDate = Source{[Schema=\"dbo\",Item=\"DimDate\"]}[Data],\n",
        "    FilteredRows = Table.SelectRows(dbo_DimDate, each [CalendarYear] = 2024 and [MonthNumberOfYear] > 1),\n",
        "    SelectedColumns = Table.SelectColumns(FilteredRows, {\"DateKey\", \"CalendarYear\"}),\n",
        "    SortedRows = Table.Sort(SelectedColumns, {{\"DateKey\", Order.Descending}})\n",
        "in\n",
        "    SortedRows;\n",
    );

    let result =
        crate::powerquery_translate::translate_powerquery_to_sql_preview(source, Some("DimDate.m"));

    assert_eq!(
        result,
        TranslatePowerQueryToSqlPreviewResult {
            mode: "heuristic_preview".to_string(),
            source_name: "DimDate.m".to_string(),
            source_line_count: 8,
            source_truncated: false,
            success: true,
            sql: "SELECT [DateKey], [CalendarYear] FROM [dbo].[DimDate] WHERE [CalendarYear] = 2024 AND [MonthNumberOfYear] > 1 ORDER BY [DateKey] DESC".to_string(),
            unsupported_functions: Vec::new(),
            warnings: vec![
                "Preview is heuristic and source-first; review the generated SQL before execution."
                    .to_string(),
            ],
        }
    );
}

#[test]
fn translate_powerquery_to_sql_preview_warns_for_unsupported_functions() {
    let source = concat!(
        "shared SalesQuery = let\n",
        "    Source = Sql.Database(\"server\", \"warehouse\"),\n",
        "    dbo_Sales = Source{[Schema=\"dbo\",Item=\"Sales\"]}[Data],\n",
        "    Promoted = Table.PromoteHeaders(dbo_Sales),\n",
        "    AddedCustom = Table.AddColumn(Promoted, \"Flag\", each 1)\n",
        "in\n",
        "    AddedCustom;\n",
    );

    let result = crate::powerquery_translate::translate_powerquery_to_sql_preview(
        source,
        Some("SalesQuery.m"),
    );

    assert_eq!(result.success, true);
    assert_eq!(result.sql, "SELECT * FROM [dbo].[Sales]".to_string());
    assert_eq!(
        result.unsupported_functions,
        vec!["Table.AddColumn".to_string()]
    );
    assert!(
        result
            .warnings
            .contains(&"Function 'Table.PromoteHeaders' was ignored in SQL preview.".to_string())
    );
    assert!(
        result.warnings.contains(
            &"Function 'Table.AddColumn' is not fully supported and was ignored in SQL preview."
                .to_string()
        )
    );
}

#[test]
fn translate_powerquery_to_sql_preview_drops_unsupported_predicates() {
    let source = concat!(
        "shared SalesQuery = let\n",
        "    Source = Sql.Database(\"server\", \"warehouse\"),\n",
        "    dbo_Sales = Source{[Schema=\"dbo\",Item=\"Sales\"]}[Data],\n",
        "    FilteredRows = Table.SelectRows(dbo_Sales, each Text.Contains([Region], \"North\"))\n",
        "in\n",
        "    FilteredRows;\n",
    );

    let result = crate::powerquery_translate::translate_powerquery_to_sql_preview(
        source,
        Some("SalesQuery.m"),
    );

    assert_eq!(result.success, true);
    assert_eq!(result.sql, "SELECT * FROM [dbo].[Sales]".to_string());
    assert!(
        result
            .warnings
            .contains(&"Table.SelectRows predicate was not translated in SQL preview.".to_string())
    );
}

#[test]
fn review_vba_onlyoffice_workbook_filters_modules_and_emits_safe_previews() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsm");
    write_zip_fixture_bytes(
        &path,
        &[
            (
                "[Content_Types].xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                  <Override PartName="/xl/vbaProject.bin" ContentType="application/vnd.ms-office.vbaProject"/>
                </Types>"#
                .to_vec(),
            ),
            (
                "xl/vbaProject.bin",
                build_minimal_vba_project_bin(&[
                    (
                        "dir".to_string(),
                        compress_ovba_literal_only(&build_vba_dir_stream("SafeModule", 0x04E4)),
                    ),
                    (
                        "SafeModule".to_string(),
                        compress_ovba_literal_only(
                            b"Attribute VB_Name = \"SafeModule\"\r\nSub FillCell()\r\n  ActiveCell.Value = \"Ready\"\r\nEnd Sub\r\n",
                        ),
                    ),
                ]),
            ),
        ],
    );

    let result = crate::vba_onlyoffice_workbook_review::review_vba_onlyoffice_workbook(
        &path,
        &path,
        Some(&["SafeModule".to_string(), "MissingModule".to_string()]),
    );

    assert_eq!(
        result,
        ReviewVbaOnlyofficeWorkbookResult {
            mode: "read_only_workbook_review".to_string(),
            path: path.display().to_string(),
            has_vba_project: true,
            code_page: Some(1252),
            extracted_module_count: 1,
            reviewed_module_count: 1,
            requested_module_names: vec!["SafeModule".to_string(), "MissingModule".to_string()],
            modules: vec![ReviewedVbaOnlyofficeWorkbookModule {
                name: "SafeModule".to_string(),
                stream_name: "SafeModule".to_string(),
                module_kind: VbaModuleKind::Procedural,
                source_truncated: false,
                analysis: AnalyzeVbaOnlyofficeMigrationResult {
                    procedures: vec![AnalyzeVbaProcedureSummary {
                        name: "FillCell".to_string(),
                        kind: "sub".to_string(),
                        start_line: 2,
                        end_line: 4,
                        supported_operation_count: 1,
                        unsupported_operation_count: 0,
                        requires_manual_rewrite: false,
                    }],
                    supported_operations: vec![AnalyzeVbaOperationSummary {
                        procedure: "FillCell".to_string(),
                        line: 3,
                        operation: "SetCellValue".to_string(),
                        target: "ActiveCell.Value".to_string(),
                        value: Some("\"Ready\"".to_string()),
                        reason: None,
                    }],
                    unsupported_operations: Vec::new(),
                    requires_manual_rewrite: false,
                    warnings: Vec::new(),
                    success: true,
                },
                warnings: Vec::new(),
                macro_value: concat!(
                    "(function()\n",
                    "{\n",
                    "    let worksheet = Api.GetActiveSheet();\n",
                    "    let workbook = Api.GetActiveWorkbook();\n",
                    "    worksheet.GetActiveCell().SetValue(\"Ready\");\n",
                    "})();"
                )
                .to_string(),
                function_body: concat!(
                    "    let worksheet = Api.GetActiveSheet();\n",
                    "    let workbook = Api.GetActiveWorkbook();\n",
                    "    worksheet.GetActiveCell().SetValue(\"Ready\");"
                )
                .to_string(),
            }],
            warnings: vec![
                "requested module MissingModule was not found in the workbook review set"
                    .to_string(),
            ],
            success: true,
        }
    );
}

#[test]
fn review_vba_onlyoffice_workbook_fails_closed_for_blocked_module() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("blocked.xlsm");
    write_zip_fixture_bytes(
        &path,
        &[
            (
                "[Content_Types].xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                  <Override PartName="/xl/vbaProject.bin" ContentType="application/vnd.ms-office.vbaProject"/>
                </Types>"#
                .to_vec(),
            ),
            (
                "xl/vbaProject.bin",
                build_minimal_vba_project_bin(&[
                    (
                        "dir".to_string(),
                        compress_ovba_literal_only(&build_vba_dir_stream("BlockedModule", 0x04E4)),
                    ),
                    (
                        "BlockedModule".to_string(),
                        compress_ovba_literal_only(
                            b"Attribute VB_Name = \"BlockedModule\"\r\nSub WarnUser()\r\n  MsgBox \"Hello\"\r\nEnd Sub\r\n",
                        ),
                    ),
                ]),
            ),
        ],
    );

    let result =
        crate::vba_onlyoffice_workbook_review::review_vba_onlyoffice_workbook(&path, &path, None);

    assert_eq!(result.reviewed_module_count, 1);
    assert_eq!(result.modules[0].name, "BlockedModule");
    assert_eq!(result.modules[0].macro_value, "");
    assert_eq!(result.modules[0].function_body, "");
    assert_eq!(result.modules[0].analysis.success, false);
    assert!(
        result.modules[0]
            .warnings
            .iter()
            .any(|warning| warning.contains("No supported spreadsheet operations"))
    );
    assert!(result.success);
}

#[tokio::test]
async fn review_vba_onlyoffice_workbook_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/sample.xlsm");
    write_zip_fixture_bytes(
        &workbook_path,
        &[
            (
                "[Content_Types].xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="bin" ContentType="application/vnd.ms-excel.sheet.binary.macroEnabled.main"/>
                  <Override PartName="/xl/vbaProject.bin" ContentType="application/vnd.ms-office.vbaProject"/>
                </Types>"#
                .to_vec(),
            ),
            (
                "xl/vbaProject.bin",
                build_minimal_vba_project_bin(&[
                    (
                        "dir".to_string(),
                        compress_ovba_literal_only(&build_vba_dir_stream("SafeModule", 0x04E4)),
                    ),
                    (
                        "SafeModule".to_string(),
                        compress_ovba_literal_only(
                            b"Attribute VB_Name = \"SafeModule\"\r\nSub FillCell()\r\n  ActiveCell.Value = \"Ready\"\r\nEnd Sub\r\n",
                        ),
                    ),
                ]),
            ),
        ],
    );

    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    install(&mut builder);
    let registry = builder.build();
    let session_store = ExtensionData::new("session");
    let thread_store = ExtensionData::new("thread");
    registry.turn_input_contributors()[0]
        .contribute(
            TurnInputContext {
                turn_id: "turn-1".to_string(),
                user_input: Vec::new(),
                environments: vec![TurnInputEnvironment {
                    environment_id: "local".to_string(),
                    cwd: workspace.clone(),
                    is_primary: true,
                }],
            },
            &session_store,
            &thread_store,
            &ExtensionData::new("turn"),
        )
        .await;

    let tool = registry
        .tool_contributors()
        .iter()
        .flat_map(|contributor| contributor.tools(&session_store, &thread_store))
        .find(|tool| {
            tool.tool_name()
                == ToolName::namespaced(EXCEL_NAMESPACE, REVIEW_VBA_ONLYOFFICE_WORKBOOK_TOOL_NAME)
        })
        .expect("excel VBA workbook review tool");
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "data/sample.xlsm",
            "module_names": ["SafeModule"],
        })
        .to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                REVIEW_VBA_ONLYOFFICE_WORKBOOK_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should review workbook");

    let result = serde_json::from_value::<ReviewVbaOnlyofficeWorkbookResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize workbook review result");
    assert_eq!(result.reviewed_module_count, 1);
    assert_eq!(result.modules[0].name, "SafeModule");
    assert_eq!(result.modules[0].macro_value.is_empty(), false);
    assert!(result.success);
}

fn function_error_message(result: Result<std::path::PathBuf, FunctionCallError>) -> String {
    match result.expect_err("expected error") {
        FunctionCallError::RespondToModel(message) => message,
        err => panic!("unexpected function call error: {err:?}"),
    }
}
