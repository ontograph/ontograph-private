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
use crate::formula_ast::FormulaAstBinaryOperator;
use crate::formula_ast::FormulaAstNode;
use crate::formula_ast::FormulaAstParseState;
use crate::formula_ast::FormulaAstSummary;
use crate::formula_cte_pipeline::INSPECT_FORMULA_CTE_PIPELINE_TOOL_NAME;
use crate::formula_cte_pipeline::InspectFormulaCtePipelineResult;
use crate::formula_sql_readiness::FormulaSqlReadinessCounts;
use crate::formula_sql_readiness::INSPECT_FORMULA_SQL_READINESS_TOOL_NAME;
use crate::formula_sql_readiness::InspectFormulaSqlReadinessResult;
use crate::named_range_rewrite::NAMED_RANGE_REWRITE_DRY_RUN_TOOL_NAME;
use crate::pivot_report_metadata::INSPECT_PIVOT_REPORT_METADATA_TOOL_NAME;
use crate::pivot_report_metadata::InspectPivotReportMetadataResult;
use crate::pivot_report_metadata::PivotCacheReportSummary;
use crate::pivot_report_metadata::PivotTableReportSummary;
use crate::pivot_report_metadata::inspect_pivot_report_metadata_from_workbook;
use crate::powerquery_extract::EXTRACT_POWERQUERY_QUERIES_TOOL_NAME;
use crate::powerquery_extract::ExtractPowerQueryQueriesResult;
use crate::powerquery_extract::ExtractedPowerQueryQuery;
use crate::powerquery_extract::PowerQueryDataModelLoadTarget;
use crate::powerquery_extract::PowerQueryDataModelPivotConsumer;
use crate::powerquery_extract::PowerQueryLexicalEvidenceKind;
use crate::powerquery_extract::PowerQueryLexicalReference;
use crate::powerquery_extract::PowerQueryLexicalReferenceKind;
use crate::powerquery_extract::PowerQueryLintCode;
use crate::powerquery_extract::PowerQueryLintFinding;
use crate::powerquery_extract::PowerQueryLoadTargetHint;
use crate::powerquery_extract::PowerQueryWorkbookConnectionSummary;
use crate::powerquery_extract::PowerQueryWorksheetLoadTarget;
use crate::powerquery_review_bundle::GENERATE_POWERQUERY_REVIEW_BUNDLE_TOOL_NAME;
use crate::powerquery_review_bundle::GeneratePowerQueryReviewBundleResult;
use crate::powerquery_review_bundle::PowerQueryBundleNormalizationStatus;
use crate::powerquery_review_bundle::PowerQueryReviewBundleManifest;
use crate::powerquery_review_bundle::PowerQueryReviewBundleQuerySummary;
use crate::powerquery_translate::TranslatePowerQueryToSqlPreviewResult;
use crate::preview::read_sheet_preview_with_display_path;
use crate::sheet_layout_metadata::INSPECT_SHEET_LAYOUT_METADATA_TOOL_NAME;
use crate::slider_query::GENERATE_SLIDER_QUERY_PACKAGE_TOOL_NAME;
use crate::slider_query::SCAN_SHEET_FORMULAS_DEPENDENCY_TOOL_NAME;
use crate::tool::CellContentMode;
use crate::tool::DefinedNameSummary;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::EXPORT_SHEET_TO_CSV_TOOL_NAME;
use crate::tool::ExportSheetToCsvResult;
use crate::tool::FormulaSqlPreviewState;
use crate::tool::FormulaSqlPreviewSummary;
use crate::tool::FormulaSqlReferenceKind;
use crate::tool::FormulaSqlReferenceSummary;
use crate::tool::INSPECT_SHEET_FORMULAS_TOOL_NAME;
use crate::tool::INSPECT_WORKBOOK_TOOL_NAME;
use crate::tool::InspectSheetFormulasResult;
use crate::tool::InspectWorkbookResult;
use crate::tool::MarkerSummary;
use crate::tool::READ_SHEET_PREVIEW_TOOL_NAME;
use crate::tool::ReadSheetPreviewResult;
use crate::tool::SheetDataValidationSummary;
use crate::tool::SheetDimension;
use crate::tool::SheetFormulaSummary;
use crate::tool::SheetKind;
use crate::tool::SheetPreview;
use crate::tool::SheetPreviewCell;
use crate::tool::SheetPreviewRow;
use crate::tool::SheetSelector;
use crate::tool::SheetSummary;
use crate::tool::SheetVisibility;
use crate::tool::WorkbookFormat;
use crate::tool::WorkbookMarkers;
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
use crate::vba_project_metadata::INSPECT_VBA_PROJECT_METADATA_TOOL_NAME;
use crate::vba_project_metadata::InspectVbaProjectMetadataResult;
use crate::vba_translate::MPreviewQuery;
use crate::vba_translate::TRANSLATE_VBA_TO_M_PREVIEW_TOOL_NAME;
use crate::vba_translate::TranslateVbaToMPreviewResult;
use crate::workbook_connections::INSPECT_WORKBOOK_CONNECTIONS_TOOL_NAME;
use crate::workbook_connections::InspectWorkbookConnectionsResult;
use crate::workbook_connections::inspect_workbook_connections_from_workbook;
use crate::workbook_defined_names::INSPECT_WORKBOOK_DEFINED_NAMES_TOOL_NAME;
use crate::workbook_defined_names::InspectWorkbookDefinedNamesResult;
use crate::workbook_defined_names::inspect_workbook_defined_names_from_workbook;
use crate::workbook_external_links::INSPECT_WORKBOOK_EXTERNAL_LINKS_TOOL_NAME;
use crate::workbook_external_links::InspectWorkbookExternalLinksResult;
use crate::workbook_external_links::WorkbookExternalLinkSummary;
use crate::workbook_external_links::inspect_workbook_external_links_from_workbook;
use crate::workbook_graph::INSPECT_WORKBOOK_GRAPH_TOOL_NAME;
use crate::workbook_migration_manifest::GENERATE_WORKBOOK_MIGRATION_MANIFEST_TOOL_NAME;
use crate::workbook_migration_manifest::GenerateWorkbookMigrationManifestResult;
use crate::workbook_migration_manifest::WorkbookMigrationFormulaSheetSummary;
use crate::workbook_migration_manifest::WorkbookMigrationFormulaSqlLineageEntry;
use crate::workbook_migration_manifest::WorkbookMigrationFormulaSqlLineageState;
use crate::workbook_migration_manifest::WorkbookMigrationManifest;
use crate::workbook_migration_manifest::WorkbookMigrationPivotSummary;
use crate::workbook_migration_manifest::WorkbookMigrationPowerQuerySummary;
use crate::workbook_migration_manifest::WorkbookMigrationVbaSummary;
use crate::workbook_tables::INSPECT_WORKBOOK_TABLES_TOOL_NAME;
use crate::workbook_used_ranges::INSPECT_WORKBOOK_USED_RANGES_TOOL_NAME;
use crate::workbook_used_ranges::InspectWorkbookUsedRangesResult;
use crate::workbook_used_ranges::WorkbookUsedRangeSummary;
use crate::workbook_used_ranges::inspect_workbook_used_ranges_from_workbook;
use ontocode_core::config::Config;
use ontocode_extension_api::FunctionCallError;

pub(crate) fn write_zip_fixture<N: AsRef<str>, C: AsRef<str>>(
    path: &std::path::Path,
    entries: &[(N, C)],
) {
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

pub(crate) fn write_zip_fixture_bytes<N: AsRef<str>>(
    path: &std::path::Path,
    entries: &[(N, Vec<u8>)],
) {
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

const SAMPLE_XLSB_BYTES: &[u8] = include_bytes!("../tests/fixtures/sample.xlsb");

fn write_embedded_xlsb_fixture(path: &std::path::Path) {
    std::fs::write(path, SAMPLE_XLSB_BYTES).expect("write embedded xlsb fixture");
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

pub(crate) fn build_data_mashup_xml_utf16(entries: &[(&str, &str)]) -> Vec<u8> {
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

pub(crate) fn write_workbook_migration_fixture(path: &Path) {
    write_zip_fixture_bytes(
        path,
        &[
            (
                "[Content_Types].xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                  <Default Extension="xml" ContentType="application/xml"/>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.ms-excel.sheet.macroEnabled.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                  <Override PartName="/xl/worksheets/sheet2.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                  <Override PartName="/xl/vbaProject.bin" ContentType="application/vnd.ms-office.vbaProject"/>
                </Types>"#
                    .to_vec(),
            ),
            (
                "xl/workbook.xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                  <sheets>
                    <sheet name="Summary" sheetId="1" r:id="rId1"/>
                    <sheet name="Data" sheetId="2" r:id="rId2"/>
                  </sheets>
                  <pivotCaches>
                    <pivotCache cacheId="1" r:id="rId3"/>
                  </pivotCaches>
                </workbook>"#
                    .to_vec(),
            ),
            (
                "xl/_rels/workbook.xml.rels",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/>
                  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" Target="pivotCache/pivotCacheDefinition1.xml"/>
                </Relationships>"#
                    .to_vec(),
            ),
            (
                "xl/worksheets/sheet1.xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="2">
                      <c r="A2"><v>7</v></c>
                      <c r="B2"><v>6</v></c>
                      <c r="D2"><f>A2+B2*2</f><v>19</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#
                    .to_vec(),
            ),
            (
                "xl/worksheets/sheet2.xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="1">
                      <c r="A1" t="inlineStr"><is><t>Region</t></is></c>
                      <c r="B1" t="inlineStr"><is><t>Revenue</t></is></c>
                    </row>
                    <row r="2">
                      <c r="A2" t="inlineStr"><is><t>North</t></is></c>
                      <c r="B2"><v>10</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#
                    .to_vec(),
            ),
            (
                "xl/worksheets/_rels/sheet1.xml.rels",
                br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable" Target="../pivotTables/pivotTable1.xml"/>
                  </Relationships>"#
                    .to_vec(),
            ),
            (
                "xl/pivotCache/pivotCacheDefinition1.xml",
                br#"<pivotCacheDefinition>
                    <cacheSource type="worksheet">
                      <worksheetSource sheet="Data" ref="A1:B4" name="SalesTable"/>
                    </cacheSource>
                    <cacheFields count="2">
                      <cacheField name="Region"/>
                      <cacheField name="Revenue"/>
                    </cacheFields>
                  </pivotCacheDefinition>"#
                    .to_vec(),
            ),
            (
                "xl/pivotTables/pivotTable1.xml",
                br#"<pivotTableDefinition name="PivotTable1" cacheId="1">
                    <location ref="D2:E5"/>
                    <rowFields count="1"><field x="0"/></rowFields>
                    <dataFields count="1"><dataField fld="1"/></dataFields>
                  </pivotTableDefinition>"#
                    .to_vec(),
            ),
            (
                "customXml/item1.xml",
                build_data_mashup_xml_utf16(&[(
                    "Formulas/Section1.m",
                    concat!(
                        "section Section1;\n",
                        "shared SalesData = let\n",
                        "    Source = Excel.CurrentWorkbook(){[Name=\"OrdersTable\"]}[Content]\n",
                        "in\n",
                        "    Source;\n",
                        "shared BrokenQuery = let\n",
                        "    Source = SalesData;\n",
                    ),
                )]),
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
          <dataValidations count="5">
            <dataValidation type="list" allowBlank="1" showDropDown="0" showErrorMessage="0" errorStyle="warning" sqref="A1 A2">
              <formula1>"Red,Blue"</formula1>
            </dataValidation>
            <dataValidation type="whole" operator="between" sqref="F1">
              <formula1>1</formula1>
              <formula2>10</formula2>
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
                ranges_sample: vec!["F1".to_string()],
                range_count: 1,
                validation_type: "whole".to_string(),
                operator: Some("between".to_string()),
                allow_blank: None,
                dropdown_visible: None,
                error_style: None,
                show_error_message: None,
                formula1: Some("1".to_string()),
                formula2: Some("10".to_string()),
                resolved_values_source: "none".to_string(),
                resolved_values_sample: Vec::new(),
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

pub(crate) fn write_formula_inventory_fixture(path: &Path) {
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

fn write_defined_name_formula_fixture(path: &Path) {
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
                    <definedName name="Threshold">42</definedName>
                  </definedNames>
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
                      <c r="A1"><f>Threshold+1</f><v>43</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

fn write_workbook_defined_names_fixture(path: &Path) {
    let long_target = "A".repeat(520);
    let mut defined_names = vec![
        r#"<definedName name="LocalName" localSheetId="0" hidden="1">Summary!$B$1</definedName>"#
            .to_string(),
        format!(r#"<definedName name="LongTarget">{long_target}</definedName>"#),
        r#"<definedName name="BrokenScope" localSheetId="9">Summary!$C$1</definedName>"#
            .to_string(),
    ];
    for index in 0..64 {
        defined_names.push(format!(
            r#"<definedName name="Name{index:02}">Summary!$A${}</definedName>"#,
            index + 1
        ));
    }
    let workbook_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                  xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
          <sheets>
            <sheet name="Summary" sheetId="1" r:id="rId1"/>
          </sheets>
          <definedNames>{}</definedNames>
        </workbook>"#,
        defined_names.join("")
    );
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
                </Types>"#,
            ),
            ("xl/workbook.xml", workbook_xml.as_str()),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#,
            ),
            ("xl/worksheets/sheet1.xml", r#"<worksheet/>"#),
        ],
    );
}

fn write_workbook_external_links_fixture(path: &Path) {
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
                  <Override PartName="/xl/externalLinks/externalLink1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"/>
                  <Override PartName="/xl/externalLinks/externalLink2.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"/>
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
                  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink" Target="externalLinks/externalLink1.xml"/>
                </Relationships>"#,
            ),
            ("xl/worksheets/sheet1.xml", r#"<worksheet/>"#),
            (
                "xl/externalLinks/externalLink1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <externalLink xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                              xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                  <externalBook r:id="rId1"/>
                </externalLink>"#,
            ),
            (
                "xl/externalLinks/_rels/externalLink1.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalBook" Target="file:///tmp/source.xlsx" TargetMode="External"/>
                </Relationships>"#,
            ),
            (
                "xl/externalLinks/externalLink2.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <externalLink xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <ddeLink/>
                </externalLink>"#,
            ),
        ],
    );
}

fn write_workbook_used_ranges_fixture(path: &Path) {
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
                    <sheet name="Data" sheetId="2" r:id="rId2"/>
                  </sheets>
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
            (
                "xl/worksheets/sheet1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <dimension ref="A1:D8"/>
                </worksheet>"#,
            ),
            (
                "xl/worksheets/sheet2.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="1"><c r="B2"><v>1</v></c></row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

fn write_scalar_formula_fixture(path: &Path) {
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
                    <row r="2">
                      <c r="A2"><v>7</v></c>
                      <c r="B2"><v>6</v></c>
                      <c r="D2"><f>A2+B2*2</f><v>19</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

fn write_vlookup_formula_fixture(path: &Path) {
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
                    <sheet name="Lookup" sheetId="2" r:id="rId2"/>
                  </sheets>
                  <definedNames>
                    <definedName name="KPI_Name">Lookup!$A$2:$B$4</definedName>
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
            (
                "xl/worksheets/sheet1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="2">
                      <c r="C2" t="inlineStr"><is><t>n</t></is></c>
                      <c r="D2"><f>VLOOKUP(C2,KPI_Name,2,FALSE)</f><v>North</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
            (
                "xl/worksheets/sheet2.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="2">
                      <c r="A2" t="inlineStr"><is><t>n</t></is></c>
                      <c r="B2" t="inlineStr"><is><t>North</t></is></c>
                    </row>
                    <row r="3">
                      <c r="A3" t="inlineStr"><is><t>s</t></is></c>
                      <c r="B3" t="inlineStr"><is><t>South</t></is></c>
                    </row>
                    <row r="4">
                      <c r="A4" t="inlineStr"><is><t>e</t></is></c>
                      <c r="B4" t="inlineStr"><is><t>East</t></is></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

fn write_lookup_vector_formula_fixture(path: &Path) {
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
                    <sheet name="Lookup" sheetId="2" r:id="rId2"/>
                  </sheets>
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
            (
                "xl/worksheets/sheet1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="2">
                      <c r="C2" t="inlineStr"><is><t>n</t></is></c>
                      <c r="D2"><f>XLOOKUP(C2,Lookup!$A$2:$A$4,Lookup!$B$2:$B$4)</f><v>North</v></c>
                      <c r="E2"><f>INDEX(Lookup!$B$2:$B$4,MATCH(C2,Lookup!$A$2:$A$4,0))</f><v>North</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
            (
                "xl/worksheets/sheet2.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="2">
                      <c r="A2" t="inlineStr"><is><t>n</t></is></c>
                      <c r="B2" t="inlineStr"><is><t>North</t></is></c>
                    </row>
                    <row r="3">
                      <c r="A3" t="inlineStr"><is><t>s</t></is></c>
                      <c r="B3" t="inlineStr"><is><t>South</t></is></c>
                    </row>
                    <row r="4">
                      <c r="A4" t="inlineStr"><is><t>e</t></is></c>
                      <c r="B4" t="inlineStr"><is><t>East</t></is></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

fn write_aggregate_formula_fixture(path: &Path) {
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
                    <sheet name="Lookup" sheetId="2" r:id="rId2"/>
                  </sheets>
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
            (
                "xl/worksheets/sheet1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="2">
                      <c r="C2" t="inlineStr"><is><t>North</t></is></c>
                      <c r="D2" t="inlineStr"><is><t>Retail</t></is></c>
                      <c r="E2"><f>SUMIFS(Lookup!$C$2:$C$4,Lookup!$A$2:$A$4,C2,Lookup!$B$2:$B$4,D2)</f><v>10</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
            (
                "xl/worksheets/sheet2.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="2">
                      <c r="A2" t="inlineStr"><is><t>North</t></is></c>
                      <c r="B2" t="inlineStr"><is><t>Retail</t></is></c>
                      <c r="C2"><v>10</v></c>
                    </row>
                    <row r="3">
                      <c r="A3" t="inlineStr"><is><t>North</t></is></c>
                      <c r="B3" t="inlineStr"><is><t>Wholesale</t></is></c>
                      <c r="C3"><v>7</v></c>
                    </row>
                    <row r="4">
                      <c r="A4" t="inlineStr"><is><t>South</t></is></c>
                      <c r="B4" t="inlineStr"><is><t>Retail</t></is></c>
                      <c r="C4"><v>5</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
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
                    parse: FormulaAstSummary {
                        state: FormulaAstParseState::Parsed,
                        root: Some(FormulaAstNode::FunctionCall {
                            name: "SUM".to_string(),
                            args: vec![FormulaAstNode::RangeReference {
                                start_reference: "B1".to_string(),
                                end_reference: "B2".to_string(),
                                sheet_name: None,
                            }],
                        }),
                        diagnostics: Vec::new(),
                        unsupported_reasons: Vec::new(),
                        truncated: false,
                    },
                    sql_preview: FormulaSqlPreviewSummary {
                        state: FormulaSqlPreviewState::Blocked,
                        sql_expression: None,
                        references: vec![FormulaSqlReferenceSummary {
                            kind: FormulaSqlReferenceKind::Range,
                            reference: "B1:B2".to_string(),
                            sheet_name: None,
                            same_row: None,
                            sql_identifier: None,
                        }],
                        blocker_reasons: vec![
                            "function_call_not_supported_in_scalar_phase".to_string(),
                            "range_reference_not_supported_in_scalar_phase".to_string(),
                            "workbook_has_external_links".to_string(),
                        ],
                        cached_value_present: true,
                    },
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
                    parse: FormulaAstSummary {
                        state: FormulaAstParseState::Parsed,
                        root: Some(FormulaAstNode::BinaryOperation {
                            operator: FormulaAstBinaryOperator::Concatenate,
                            left: Box::new(FormulaAstNode::CellReference {
                                reference: "A1".to_string(),
                                sheet_name: None,
                            }),
                            right: Box::new(FormulaAstNode::StringLiteral {
                                value: "x".to_string(),
                            }),
                        }),
                        diagnostics: Vec::new(),
                        unsupported_reasons: Vec::new(),
                        truncated: false,
                    },
                    sql_preview: FormulaSqlPreviewSummary {
                        state: FormulaSqlPreviewState::Blocked,
                        sql_expression: None,
                        references: vec![FormulaSqlReferenceSummary {
                            kind: FormulaSqlReferenceKind::Cell,
                            reference: "A1".to_string(),
                            sheet_name: None,
                            same_row: Some(true),
                            sql_identifier: Some("col_a".to_string()),
                        }],
                        blocker_reasons: vec!["workbook_has_external_links".to_string()],
                        cached_value_present: true,
                    },
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
                    parse: FormulaAstSummary {
                        state: FormulaAstParseState::Unsupported,
                        root: Some(FormulaAstNode::Unsupported {
                            reason: "volatile_function".to_string(),
                            text: "NOW".to_string(),
                        }),
                        diagnostics: Vec::new(),
                        unsupported_reasons: vec!["volatile_function".to_string()],
                        truncated: false,
                    },
                    sql_preview: FormulaSqlPreviewSummary {
                        state: FormulaSqlPreviewState::Blocked,
                        sql_expression: None,
                        references: Vec::new(),
                        blocker_reasons: vec![
                            "volatile_function".to_string(),
                            "workbook_has_external_links".to_string(),
                        ],
                        cached_value_present: true,
                    },
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
                    parse: FormulaAstSummary {
                        state: FormulaAstParseState::Missing,
                        root: None,
                        diagnostics: Vec::new(),
                        unsupported_reasons: Vec::new(),
                        truncated: false,
                    },
                    sql_preview: FormulaSqlPreviewSummary {
                        state: FormulaSqlPreviewState::Blocked,
                        sql_expression: None,
                        references: Vec::new(),
                        blocker_reasons: vec![
                            "formula_text_missing".to_string(),
                            "workbook_has_external_links".to_string(),
                        ],
                        cached_value_present: true,
                    },
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
fn inspect_sheet_formulas_parses_defined_name_references() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("defined-name-formulas.xlsx");
    write_defined_name_formula_fixture(&path);

    let result = crate::formula_inspect::inspect_sheet_formulas_with_display_path(
        &path,
        Path::new("defined-name-formulas.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect sheet formulas");

    assert_eq!(
        result.formulas,
        vec![SheetFormulaSummary {
            reference: "A1".to_string(),
            formula: "Threshold+1".to_string(),
            cached_value: Some("43".to_string()),
            parse: FormulaAstSummary {
                state: FormulaAstParseState::Parsed,
                root: Some(FormulaAstNode::BinaryOperation {
                    operator: FormulaAstBinaryOperator::Add,
                    left: Box::new(FormulaAstNode::DefinedNameReference {
                        name: "Threshold".to_string(),
                        sheet_name: None,
                    }),
                    right: Box::new(FormulaAstNode::NumberLiteral {
                        value: "1".to_string(),
                    }),
                }),
                diagnostics: Vec::new(),
                unsupported_reasons: Vec::new(),
                truncated: false,
            },
            sql_preview: FormulaSqlPreviewSummary {
                state: FormulaSqlPreviewState::Blocked,
                sql_expression: None,
                references: vec![FormulaSqlReferenceSummary {
                    kind: FormulaSqlReferenceKind::DefinedName,
                    reference: "Threshold".to_string(),
                    sheet_name: None,
                    same_row: None,
                    sql_identifier: None,
                }],
                blocker_reasons: vec![
                    "defined_name_reference_not_supported_in_scalar_phase".to_string(),
                ],
                cached_value_present: true,
            },
            warnings: Vec::new(),
            formula_type: None,
            shared_index: None,
            shared_range: None,
            style_index: None,
            number_format_id: None,
            number_format_code: None,
        }]
    );
}

#[test]
fn inspect_sheet_formulas_emits_review_only_scalar_sql_preview() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("scalar-formulas.xlsx");
    write_scalar_formula_fixture(&path);

    let result = crate::formula_inspect::inspect_sheet_formulas_with_display_path(
        &path,
        Path::new("scalar-formulas.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect sheet formulas");

    assert_eq!(
        result.formulas,
        vec![SheetFormulaSummary {
            reference: "D2".to_string(),
            formula: "A2+B2*2".to_string(),
            cached_value: Some("19".to_string()),
            parse: FormulaAstSummary {
                state: FormulaAstParseState::Parsed,
                root: Some(FormulaAstNode::BinaryOperation {
                    operator: FormulaAstBinaryOperator::Add,
                    left: Box::new(FormulaAstNode::CellReference {
                        reference: "A2".to_string(),
                        sheet_name: None,
                    }),
                    right: Box::new(FormulaAstNode::BinaryOperation {
                        operator: FormulaAstBinaryOperator::Multiply,
                        left: Box::new(FormulaAstNode::CellReference {
                            reference: "B2".to_string(),
                            sheet_name: None,
                        }),
                        right: Box::new(FormulaAstNode::NumberLiteral {
                            value: "2".to_string(),
                        }),
                    }),
                }),
                diagnostics: Vec::new(),
                unsupported_reasons: Vec::new(),
                truncated: false,
            },
            sql_preview: FormulaSqlPreviewSummary {
                state: FormulaSqlPreviewState::ReviewOnly,
                sql_expression: Some("([col_a] + ([col_b] * 2))".to_string()),
                references: vec![
                    FormulaSqlReferenceSummary {
                        kind: FormulaSqlReferenceKind::Cell,
                        reference: "A2".to_string(),
                        sheet_name: None,
                        same_row: Some(true),
                        sql_identifier: Some("col_a".to_string()),
                    },
                    FormulaSqlReferenceSummary {
                        kind: FormulaSqlReferenceKind::Cell,
                        reference: "B2".to_string(),
                        sheet_name: None,
                        same_row: Some(true),
                        sql_identifier: Some("col_b".to_string()),
                    },
                ],
                blocker_reasons: Vec::new(),
                cached_value_present: true,
            },
            warnings: Vec::new(),
            formula_type: None,
            shared_index: None,
            shared_range: None,
            style_index: None,
            number_format_id: None,
            number_format_code: None,
        }]
    );
}

#[test]
fn inspect_sheet_formulas_emits_review_only_exact_vlookup_sql_preview() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("vlookup-formulas.xlsx");
    write_vlookup_formula_fixture(&path);

    let result = crate::formula_inspect::inspect_sheet_formulas_with_display_path(
        &path,
        Path::new("vlookup-formulas.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect sheet formulas");

    assert_eq!(
        result.formulas,
        vec![SheetFormulaSummary {
            reference: "D2".to_string(),
            formula: "VLOOKUP(C2,KPI_Name,2,FALSE)".to_string(),
            cached_value: Some("North".to_string()),
            parse: FormulaAstSummary {
                state: FormulaAstParseState::Parsed,
                root: Some(FormulaAstNode::FunctionCall {
                    name: "VLOOKUP".to_string(),
                    args: vec![
                        FormulaAstNode::CellReference {
                            reference: "C2".to_string(),
                            sheet_name: None,
                        },
                        FormulaAstNode::DefinedNameReference {
                            name: "KPI_Name".to_string(),
                            sheet_name: None,
                        },
                        FormulaAstNode::NumberLiteral {
                            value: "2".to_string(),
                        },
                        FormulaAstNode::BooleanLiteral { value: false },
                    ],
                }),
                diagnostics: Vec::new(),
                unsupported_reasons: Vec::new(),
                truncated: false,
            },
            sql_preview: FormulaSqlPreviewSummary {
                state: FormulaSqlPreviewState::ReviewOnly,
                sql_expression: Some(
                    "(SELECT [lookup_col_2] FROM [lookup_kpi_name] WHERE [lookup_col_1] = [col_c])"
                        .to_string()
                ),
                references: vec![
                    FormulaSqlReferenceSummary {
                        kind: FormulaSqlReferenceKind::Cell,
                        reference: "C2".to_string(),
                        sheet_name: None,
                        same_row: Some(true),
                        sql_identifier: Some("col_c".to_string()),
                    },
                    FormulaSqlReferenceSummary {
                        kind: FormulaSqlReferenceKind::DefinedName,
                        reference: "KPI_Name".to_string(),
                        sheet_name: None,
                        same_row: None,
                        sql_identifier: Some("lookup_kpi_name".to_string()),
                    },
                    FormulaSqlReferenceSummary {
                        kind: FormulaSqlReferenceKind::Range,
                        reference: "$A$2:$B$4".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                        same_row: None,
                        sql_identifier: None,
                    },
                ],
                blocker_reasons: Vec::new(),
                cached_value_present: true,
            },
            warnings: Vec::new(),
            formula_type: None,
            shared_index: None,
            shared_range: None,
            style_index: None,
            number_format_id: None,
            number_format_code: None,
        }]
    );
}

#[test]
fn inspect_sheet_formulas_emits_review_only_exact_xlookup_and_index_match_sql_preview() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("lookup-vector-formulas.xlsx");
    write_lookup_vector_formula_fixture(&path);

    let result = crate::formula_inspect::inspect_sheet_formulas_with_display_path(
        &path,
        Path::new("lookup-vector-formulas.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect sheet formulas");

    assert_eq!(
        result.formulas,
        vec![
            SheetFormulaSummary {
                reference: "D2".to_string(),
                formula: "XLOOKUP(C2,Lookup!$A$2:$A$4,Lookup!$B$2:$B$4)".to_string(),
                cached_value: Some("North".to_string()),
                parse: FormulaAstSummary {
                    state: FormulaAstParseState::Parsed,
                    root: Some(FormulaAstNode::FunctionCall {
                        name: "XLOOKUP".to_string(),
                        args: vec![
                            FormulaAstNode::CellReference {
                                reference: "C2".to_string(),
                                sheet_name: None,
                            },
                            FormulaAstNode::RangeReference {
                                start_reference: "$A$2".to_string(),
                                end_reference: "$A$4".to_string(),
                                sheet_name: Some("Lookup".to_string()),
                            },
                            FormulaAstNode::RangeReference {
                                start_reference: "$B$2".to_string(),
                                end_reference: "$B$4".to_string(),
                                sheet_name: Some("Lookup".to_string()),
                            },
                        ],
                    }),
                    diagnostics: Vec::new(),
                    unsupported_reasons: Vec::new(),
                    truncated: false,
                },
                sql_preview: FormulaSqlPreviewSummary {
                    state: FormulaSqlPreviewState::ReviewOnly,
                    sql_expression: Some(
                        "(SELECT [lookup_return_1] FROM [lookup_pair_lookup_a_2_a_4_b_2_b_4] WHERE [lookup_key_1] = [col_c])"
                            .to_string()
                    ),
                    references: vec![
                        FormulaSqlReferenceSummary {
                            kind: FormulaSqlReferenceKind::Cell,
                            reference: "C2".to_string(),
                            sheet_name: None,
                            same_row: Some(true),
                            sql_identifier: Some("col_c".to_string()),
                        },
                        FormulaSqlReferenceSummary {
                            kind: FormulaSqlReferenceKind::Range,
                            reference: "$A$2:$A$4".to_string(),
                            sheet_name: Some("Lookup".to_string()),
                            same_row: None,
                            sql_identifier: None,
                        },
                        FormulaSqlReferenceSummary {
                            kind: FormulaSqlReferenceKind::Range,
                            reference: "$B$2:$B$4".to_string(),
                            sheet_name: Some("Lookup".to_string()),
                            same_row: None,
                            sql_identifier: None,
                        },
                    ],
                    blocker_reasons: Vec::new(),
                    cached_value_present: true,
                },
                warnings: Vec::new(),
                formula_type: None,
                shared_index: None,
                shared_range: None,
                style_index: None,
                number_format_id: None,
                number_format_code: None,
            },
            SheetFormulaSummary {
                reference: "E2".to_string(),
                formula: "INDEX(Lookup!$B$2:$B$4,MATCH(C2,Lookup!$A$2:$A$4,0))".to_string(),
                cached_value: Some("North".to_string()),
                parse: FormulaAstSummary {
                    state: FormulaAstParseState::Parsed,
                    root: Some(FormulaAstNode::FunctionCall {
                        name: "INDEX".to_string(),
                        args: vec![
                            FormulaAstNode::RangeReference {
                                start_reference: "$B$2".to_string(),
                                end_reference: "$B$4".to_string(),
                                sheet_name: Some("Lookup".to_string()),
                            },
                            FormulaAstNode::FunctionCall {
                                name: "MATCH".to_string(),
                                args: vec![
                                    FormulaAstNode::CellReference {
                                        reference: "C2".to_string(),
                                        sheet_name: None,
                                    },
                                    FormulaAstNode::RangeReference {
                                        start_reference: "$A$2".to_string(),
                                        end_reference: "$A$4".to_string(),
                                        sheet_name: Some("Lookup".to_string()),
                                    },
                                    FormulaAstNode::NumberLiteral {
                                        value: "0".to_string(),
                                    },
                                ],
                            },
                        ],
                    }),
                    diagnostics: Vec::new(),
                    unsupported_reasons: Vec::new(),
                    truncated: false,
                },
                sql_preview: FormulaSqlPreviewSummary {
                    state: FormulaSqlPreviewState::ReviewOnly,
                    sql_expression: Some(
                        "(SELECT [lookup_return_1] FROM [lookup_pair_lookup_a_2_a_4_b_2_b_4] WHERE [lookup_key_1] = [col_c])"
                            .to_string()
                    ),
                    references: vec![
                        FormulaSqlReferenceSummary {
                            kind: FormulaSqlReferenceKind::Range,
                            reference: "$B$2:$B$4".to_string(),
                            sheet_name: Some("Lookup".to_string()),
                            same_row: None,
                            sql_identifier: None,
                        },
                        FormulaSqlReferenceSummary {
                            kind: FormulaSqlReferenceKind::Cell,
                            reference: "C2".to_string(),
                            sheet_name: None,
                            same_row: Some(true),
                            sql_identifier: Some("col_c".to_string()),
                        },
                        FormulaSqlReferenceSummary {
                            kind: FormulaSqlReferenceKind::Range,
                            reference: "$A$2:$A$4".to_string(),
                            sheet_name: Some("Lookup".to_string()),
                            same_row: None,
                            sql_identifier: None,
                        },
                    ],
                    blocker_reasons: Vec::new(),
                    cached_value_present: true,
                },
                warnings: Vec::new(),
                formula_type: None,
                shared_index: None,
                shared_range: None,
                style_index: None,
                number_format_id: None,
                number_format_code: None,
            },
        ]
    );
}

#[test]
fn inspect_sheet_formulas_emits_review_only_aligned_sumifs_sql_preview() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("aggregate-formulas.xlsx");
    write_aggregate_formula_fixture(&path);

    let result = crate::formula_inspect::inspect_sheet_formulas_with_display_path(
        &path,
        Path::new("aggregate-formulas.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect sheet formulas");

    assert_eq!(
        result.formulas,
        vec![SheetFormulaSummary {
            reference: "E2".to_string(),
            formula: "SUMIFS(Lookup!$C$2:$C$4,Lookup!$A$2:$A$4,C2,Lookup!$B$2:$B$4,D2)"
                .to_string(),
            cached_value: Some("10".to_string()),
            parse: FormulaAstSummary {
                state: FormulaAstParseState::Parsed,
                root: Some(FormulaAstNode::FunctionCall {
                    name: "SUMIFS".to_string(),
                    args: vec![
                        FormulaAstNode::RangeReference {
                            start_reference: "$C$2".to_string(),
                            end_reference: "$C$4".to_string(),
                            sheet_name: Some("Lookup".to_string()),
                        },
                        FormulaAstNode::RangeReference {
                            start_reference: "$A$2".to_string(),
                            end_reference: "$A$4".to_string(),
                            sheet_name: Some("Lookup".to_string()),
                        },
                        FormulaAstNode::CellReference {
                            reference: "C2".to_string(),
                            sheet_name: None,
                        },
                        FormulaAstNode::RangeReference {
                            start_reference: "$B$2".to_string(),
                            end_reference: "$B$4".to_string(),
                            sheet_name: Some("Lookup".to_string()),
                        },
                        FormulaAstNode::CellReference {
                            reference: "D2".to_string(),
                            sheet_name: None,
                        },
                    ],
                }),
                diagnostics: Vec::new(),
                unsupported_reasons: Vec::new(),
                truncated: false,
            },
            sql_preview: FormulaSqlPreviewSummary {
                state: FormulaSqlPreviewState::ReviewOnly,
                sql_expression: Some(
                    "(SELECT SUM([aggregate_value_1]) FROM [aggregate_source_lookup_c_2_c_4_a_2_a_4_b_2_b_4] WHERE [criteria_col_1] = [col_c] AND [criteria_col_2] = [col_d])"
                        .to_string()
                ),
                references: vec![
                    FormulaSqlReferenceSummary {
                        kind: FormulaSqlReferenceKind::Range,
                        reference: "$C$2:$C$4".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                        same_row: None,
                        sql_identifier: None,
                    },
                    FormulaSqlReferenceSummary {
                        kind: FormulaSqlReferenceKind::Range,
                        reference: "$A$2:$A$4".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                        same_row: None,
                        sql_identifier: None,
                    },
                    FormulaSqlReferenceSummary {
                        kind: FormulaSqlReferenceKind::Cell,
                        reference: "C2".to_string(),
                        sheet_name: None,
                        same_row: Some(true),
                        sql_identifier: Some("col_c".to_string()),
                    },
                    FormulaSqlReferenceSummary {
                        kind: FormulaSqlReferenceKind::Range,
                        reference: "$B$2:$B$4".to_string(),
                        sheet_name: Some("Lookup".to_string()),
                        same_row: None,
                        sql_identifier: None,
                    },
                    FormulaSqlReferenceSummary {
                        kind: FormulaSqlReferenceKind::Cell,
                        reference: "D2".to_string(),
                        sheet_name: None,
                        same_row: Some(true),
                        sql_identifier: Some("col_d".to_string()),
                    },
                ],
                blocker_reasons: Vec::new(),
                cached_value_present: true,
            },
            warnings: Vec::new(),
            formula_type: None,
            shared_index: None,
            shared_range: None,
            style_index: None,
            number_format_id: None,
            number_format_code: None,
        }]
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
fn read_sheet_preview_reads_real_xlsb_values_with_bounded_warnings() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsb");
    write_embedded_xlsb_fixture(&path);

    let result = read_sheet_preview_with_display_path(
        &path,
        &path,
        &SheetSelector::Index { index: 0 },
        None,
        CellContentMode::Values,
    )
    .expect("xlsb preview should succeed");

    assert_eq!(result.sheet.name, "Лист1".to_string());
    assert_eq!(
        result.sheet.part_path,
        "xl/worksheets/sheet1.bin".to_string()
    );
    assert_eq!(
        result.dimension,
        Some(SheetDimension {
            reference: "A1:J3".to_string(),
        })
    );
    assert_eq!(
        result.rows,
        vec![
            SheetPreviewRow {
                row_index: 1,
                cells: vec![
                    SheetPreviewCell {
                        reference: "A1".to_string(),
                        value: Some("AST".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "C1".to_string(),
                        value: Some("id".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "D1".to_string(),
                        value: Some("AST".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "E1".to_string(),
                        value: Some("Наименование".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "F1".to_string(),
                        value: Some("БЕ".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "G1".to_string(),
                        value: Some("ОС".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "H1".to_string(),
                        value: Some("Инв".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "I1".to_string(),
                        value: Some("ТМЦ".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "J1".to_string(),
                        value: Some("KE_id".to_string()),
                        formula: None,
                    },
                ],
            },
            SheetPreviewRow {
                row_index: 2,
                cells: vec![
                    SheetPreviewCell {
                        reference: "A2".to_string(),
                        value: Some("AST0012741".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "C2".to_string(),
                        value: Some("166151358208687732".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "D2".to_string(),
                        value: Some("AST0010533".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "E2".to_string(),
                        value: Some("МФУ Ricoh IM 2702".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "F2".to_string(),
                        value: Some("3200".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "G2".to_string(),
                        value: Some("true".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "H2".to_string(),
                        value: Some("аренда Ricoh".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "I2".to_string(),
                        value: Some("".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "J2".to_string(),
                        value: Some("164622241490681986".to_string()),
                        formula: None,
                    },
                ],
            },
            SheetPreviewRow {
                row_index: 3,
                cells: vec![
                    SheetPreviewCell {
                        reference: "A3".to_string(),
                        value: Some("AST0012742".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "C3".to_string(),
                        value: Some("166151358208687796".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "D3".to_string(),
                        value: Some("AST0012742".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "E3".to_string(),
                        value: Some("МФУ Ricoh IM 550".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "F3".to_string(),
                        value: Some("3200".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "G3".to_string(),
                        value: Some("true".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "H3".to_string(),
                        value: Some("аренда Ricoh".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "I3".to_string(),
                        value: Some("".to_string()),
                        formula: None,
                    },
                    SheetPreviewCell {
                        reference: "J3".to_string(),
                        value: Some("164622241490681986".to_string()),
                        formula: None,
                    },
                ],
            },
        ]
    );
    assert_eq!(
        result.warnings,
        vec![
            "excel.read_sheet_preview does not decode .xlsb data validations in this stage"
                .to_string(),
        ]
    );
}

#[test]
fn inspect_sheet_formulas_reads_real_xlsb_sheet_metadata_without_stage_gate_error() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsb");
    write_embedded_xlsb_fixture(&path);

    let result = crate::formula_inspect::inspect_sheet_formulas_with_display_path(
        &path,
        &path,
        &SheetSelector::Index { index: 0 },
        None,
    )
    .expect("xlsb formula inventory should succeed");

    assert_eq!(result.sheet.name, "Лист1".to_string());
    assert_eq!(
        result.sheet.part_path,
        "xl/worksheets/sheet1.bin".to_string()
    );
    assert!(result.formulas.is_empty());
    assert_eq!(
        result.defined_names,
        vec![
            DefinedNameSummary {
                name: "RangeSelectionPrompt".to_string(),
                sheet_scope: None,
                local_sheet_id: None,
                hidden: None,
                target: "".to_string(),
                truncated: false,
            },
            DefinedNameSummary {
                name: "RequestRata".to_string(),
                sheet_scope: None,
                local_sheet_id: None,
                hidden: None,
                target: "".to_string(),
                truncated: false,
            },
        ]
    );
    assert_eq!(
        result.warnings,
        vec![
            "excel.inspect_sheet_formulas does not decode .xlsb calculation, style, or shared-formula metadata in this stage".to_string(),
        ]
    );
}

#[test]
fn inspect_workbook_defined_names_reports_bounded_inventory_and_warnings() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("defined-names.xlsx");
    write_workbook_defined_names_fixture(&path);

    let result =
        inspect_workbook_defined_names_from_workbook(&path, Path::new("defined-names.xlsx"))
            .expect("defined name inventory should succeed");

    assert_eq!(result.path, "defined-names.xlsx".to_string());
    assert_eq!(result.defined_name_count, 67);
    assert_eq!(result.defined_names.len(), 64);
    assert_eq!(
        result.defined_names[0],
        DefinedNameSummary {
            name: "LocalName".to_string(),
            sheet_scope: Some("Summary".to_string()),
            local_sheet_id: Some(0),
            hidden: Some(true),
            target: "Summary!$B$1".to_string(),
            truncated: false,
        }
    );
    assert_eq!(
        result.defined_names[1],
        DefinedNameSummary {
            name: "LongTarget".to_string(),
            sheet_scope: None,
            local_sheet_id: None,
            hidden: None,
            target: format!("{}...", "A".repeat(509)),
            truncated: true,
        }
    );
    assert_eq!(
        result.defined_names[2],
        DefinedNameSummary {
            name: "BrokenScope".to_string(),
            sheet_scope: None,
            local_sheet_id: Some(9),
            hidden: None,
            target: "Summary!$C$1".to_string(),
            truncated: false,
        }
    );
    assert_eq!(result.defined_names_sample.len(), 64);
    assert_eq!(
        result.defined_names_sample[0],
        "LocalName=Summary!$B$1".to_string()
    );
    assert_eq!(
        result.warnings,
        vec![
            "defined name inventory truncated to 64 of 67 names".to_string(),
            "1 defined name targets truncated to 512 characters".to_string(),
            "1 defined names kept unresolved localSheetId values without matching sheet names"
                .to_string(),
        ]
    );
}

#[test]
fn inspect_workbook_defined_names_rejects_xlsb_stage_gap() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsb");
    write_embedded_xlsb_fixture(&path);

    let err = inspect_workbook_defined_names_from_workbook(&path, Path::new("sample.xlsb"))
        .expect_err("xlsb defined names should stay unsupported");
    assert_eq!(
        err.to_string(),
        "excel.inspect_workbook_defined_names supports only .xlsx and .xlsm in this stage; .xlsb defined-name inventory remains unsupported"
            .to_string()
    );
}

#[test]
fn inspect_workbook_defined_names_fails_closed_on_malformed_workbook_xml() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("broken-defined-names.xlsx");
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
                </Types>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <definedNames>
                    <definedName name="Broken">Summary!$A$1
                  </definedNames>
                </workbook>"#,
            ),
        ],
    );

    let err =
        inspect_workbook_defined_names_from_workbook(&path, Path::new("broken-defined-names.xlsx"))
            .expect_err("malformed workbook.xml should fail closed");
    assert!(
        err.to_string()
            .starts_with("failed to parse workbook sheet list:"),
        "unexpected error: {err}"
    );
}

#[test]
fn inspect_workbook_defined_names_rejects_invalid_local_sheet_id() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("bad-local-sheet-id.xlsx");
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
                  <definedNames>
                    <definedName name="BrokenScope" localSheetId="oops">Summary!$A$1</definedName>
                  </definedNames>
                </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#,
            ),
        ],
    );

    let err =
        inspect_workbook_defined_names_from_workbook(&path, Path::new("bad-local-sheet-id.xlsx"))
            .expect_err("invalid localSheetId should fail closed");
    assert!(
        err.to_string().starts_with(
            "failed to parse workbook formula context: invalid definedName localSheetId `oops`:"
        ),
        "unexpected error: {err}"
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
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_DEFINED_NAMES_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_EXTERNAL_LINKS_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_USED_RANGES_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_FORMULA_SQL_READINESS_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_FORMULA_CTE_PIPELINE_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, SCAN_SHEET_FORMULAS_DEPENDENCY_TOOL_NAME,),
            ToolName::namespaced(EXCEL_NAMESPACE, GENERATE_SLIDER_QUERY_PACKAGE_TOOL_NAME,),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_PIVOT_REPORT_METADATA_TOOL_NAME,),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_CONNECTIONS_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_TABLES_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_SHEET_LAYOUT_METADATA_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_GRAPH_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, NAMED_RANGE_REWRITE_DRY_RUN_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, EXPORT_SHEET_TO_CSV_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, EXTRACT_POWERQUERY_QUERIES_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, GENERATE_POWERQUERY_REVIEW_BUNDLE_TOOL_NAME),
            ToolName::namespaced(
                EXCEL_NAMESPACE,
                GENERATE_WORKBOOK_MIGRATION_MANIFEST_TOOL_NAME,
            ),
            ToolName::namespaced(EXCEL_NAMESPACE, EXTRACT_VBA_MODULES_TOOL_NAME),
            ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_VBA_PROJECT_METADATA_TOOL_NAME),
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

#[tokio::test]
async fn inspect_vba_project_metadata_tool_resolves_relative_path_against_turn_cwd() {
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
            tool.tool_name()
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_VBA_PROJECT_METADATA_TOOL_NAME)
        })
        .expect("excel VBA project metadata tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "data/sample.xlsm" }).to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                INSPECT_VBA_PROJECT_METADATA_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect VBA project metadata");

    assert_eq!(
        serde_json::from_value::<InspectVbaProjectMetadataResult>(
            output
                .post_tool_use_response("call-1", &payload)
                .expect("json response")
        )
        .expect("deserialize metadata result")
        .path,
        "data/sample.xlsm".to_string()
    );
}

#[tokio::test]
async fn inspect_vba_project_metadata_tool_reports_own_path_errors() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("create workspace");

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
                    cwd: workspace,
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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_VBA_PROJECT_METADATA_TOOL_NAME)
        })
        .expect("excel VBA project metadata tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "../sample.xlsm" }).to_string(),
    };
    let result = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                INSPECT_VBA_PROJECT_METADATA_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload,
        })
        .await;

    let message = match result {
        Ok(_) => panic!("unscoped path should fail"),
        Err(FunctionCallError::RespondToModel(message)) => message,
        Err(other) => panic!("unexpected error variant: {other:?}"),
    };
    assert!(message.starts_with("excel.inspect_vba_project_metadata path"));
    assert!(!message.contains("excel.extract_vba_modules"));
}

#[derive(Clone, Copy)]
pub(crate) enum VbaFixtureModuleType {
    Procedural,
    DocClsDesigner,
}

pub(crate) fn build_vba_dir_stream(module_name: &str, code_page: u16) -> Vec<u8> {
    build_vba_dir_stream_with_modules(
        &[(module_name, VbaFixtureModuleType::Procedural)],
        code_page,
    )
}

pub(crate) fn build_vba_dir_stream_with_modules(
    modules: &[(&str, VbaFixtureModuleType)],
    code_page: u16,
) -> Vec<u8> {
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
    append_fixed_u16_record(&mut dir, 0x000F, modules.len() as u16);
    append_fixed_u16_record(&mut dir, 0x0013, 0);

    for (module_name, module_type) in modules {
        append_dir_record(&mut dir, 0x0019, module_name.as_bytes());
        append_dir_record(&mut dir, 0x001A, module_name.as_bytes());
        append_dir_record(&mut dir, 0x0032, &utf16le_bytes(module_name));
        append_dir_record(&mut dir, 0x001C, &[]);
        append_dir_record(&mut dir, 0x0048, &[]);
        append_fixed_u32_record(&mut dir, 0x0031, 0);
        append_fixed_u32_record(&mut dir, 0x001E, 0);
        append_fixed_u16_record(&mut dir, 0x002C, 0);
        match module_type {
            VbaFixtureModuleType::Procedural => append_reserved_record(&mut dir, 0x0021),
            VbaFixtureModuleType::DocClsDesigner => append_reserved_record(&mut dir, 0x0022),
        }
        append_reserved_record(&mut dir, 0x002B);
    }
    append_reserved_record(&mut dir, 0x0010);
    dir
}

pub(crate) fn compress_ovba_literal_only(data: &[u8]) -> Vec<u8> {
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

pub(crate) fn build_minimal_vba_project_bin(streams: &[(String, Vec<u8>)]) -> Vec<u8> {
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

#[test]
fn inspect_pivot_report_metadata_reads_worksheet_source_cache() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("pivot-worksheet.xlsx");
    write_zip_fixture(
        &path,
        &[
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                    <sheets>
                      <sheet name="Report" sheetId="1" r:id="rId1"/>
                    </sheets>
                    <pivotCaches>
                      <pivotCache cacheId="1" r:id="rId2"/>
                    </pivotCaches>
                  </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" Target="pivotCache/pivotCacheDefinition1.xml"/>
                  </Relationships>"#,
            ),
            ("xl/worksheets/sheet1.xml", r#"<worksheet/>"#),
            (
                "xl/worksheets/_rels/sheet1.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable" Target="../pivotTables/pivotTable1.xml"/>
                  </Relationships>"#,
            ),
            (
                "xl/pivotCache/pivotCacheDefinition1.xml",
                r#"<pivotCacheDefinition>
                    <cacheSource type="worksheet">
                      <worksheetSource sheet="Data" ref="A1:B4" name="SalesTable"/>
                    </cacheSource>
                    <cacheFields count="2">
                      <cacheField name="Region"/>
                      <cacheField name="Revenue"/>
                    </cacheFields>
                  </pivotCacheDefinition>"#,
            ),
            (
                "xl/pivotTables/pivotTable1.xml",
                r#"<pivotTableDefinition name="PivotTable1" cacheId="1">
                    <location ref="D2:E5"/>
                    <rowFields count="1"><field x="0"/></rowFields>
                    <dataFields count="1"><dataField fld="1"/></dataFields>
                  </pivotTableDefinition>"#,
            ),
        ],
    );

    let result =
        inspect_pivot_report_metadata_from_workbook(&path, Path::new("pivot-worksheet.xlsx"))
            .expect("inspect pivot metadata");

    assert_eq!(
        result,
        InspectPivotReportMetadataResult {
            mode: "openxml_package".to_string(),
            path: "pivot-worksheet.xlsx".to_string(),
            pivot_table_count: 1,
            pivot_cache_count: 1,
            pivot_tables: vec![PivotTableReportSummary {
                name: Some("PivotTable1".to_string()),
                worksheet_name: Some("Report".to_string()),
                part_path: Some("xl/pivotTables/pivotTable1.xml".to_string()),
                range_ref: Some("D2:E5".to_string()),
                cache_id: Some("1".to_string()),
                source_type: Some("worksheet".to_string()),
                source_name: Some("SalesTable".to_string()),
                source_range: Some("A1:B4".to_string()),
                connection_id: None,
                connection_name: None,
                connection_type: None,
                olap: false,
                data_model: false,
                stored_mdx_preview: None,
                stored_mdx_truncated: false,
                row_fields_sample: vec!["Region".to_string()],
                column_fields_sample: Vec::new(),
                data_fields_sample: vec!["Revenue".to_string()],
                page_fields_sample: Vec::new(),
                warnings: Vec::new(),
            }],
            pivot_caches: vec![PivotCacheReportSummary {
                cache_id: Some("1".to_string()),
                source_type: Some("worksheet".to_string()),
                source_name: Some("SalesTable".to_string()),
                source_range: Some("A1:B4".to_string()),
                connection_id: None,
                connection_name: None,
                connection_type: None,
                olap: false,
                data_model: false,
                cache_fields_sample: vec!["Region".to_string(), "Revenue".to_string()],
                warnings: Vec::new(),
            }],
            warnings: Vec::new(),
        }
    );
}

#[test]
fn inspect_pivot_report_metadata_reads_data_model_cache() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("pivot-model.xlsm");
    write_zip_fixture(
        &path,
        &[
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                    <sheets>
                      <sheet name="Total Revenue" sheetId="1" r:id="rId1"/>
                    </sheets>
                    <pivotCaches>
                      <pivotCache cacheId="107" r:id="rId2"/>
                    </pivotCaches>
                  </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet5.xml"/>
                    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" Target="pivotCache/pivotCacheDefinition1.xml"/>
                  </Relationships>"#,
            ),
            ("xl/worksheets/sheet5.xml", r#"<worksheet/>"#),
            (
                "xl/worksheets/_rels/sheet5.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable" Target="../pivotTables/pivotTable1.xml"/>
                  </Relationships>"#,
            ),
            (
                "xl/connections.xml",
                r#"<connections xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
                    <connection id="8" name="ThisWorkbookDataModel">
                      <dbPr connection="Data Model Connection" command="Model" commandType="1"/>
                      <olapPr/>
                      <extLst><ext><x15:connection model="1"/></ext></extLst>
                    </connection>
                  </connections>"#,
            ),
            (
                "xl/pivotCache/pivotCacheDefinition1.xml",
                r#"<pivotCacheDefinition>
                    <cacheSource type="external" connectionId="8"/>
                    <cacheFields count="0"/>
                    <cacheHierarchies count="2">
                      <cacheHierarchy caption="OrderDate (Month)"/>
                      <cacheHierarchy caption="Total Revenue"/>
                    </cacheHierarchies>
                  </pivotCacheDefinition>"#,
            ),
            (
                "xl/pivotTables/pivotTable1.xml",
                r#"<pivotTableDefinition name="PivotTable2" cacheId="107">
                    <location ref="B3:C16"/>
                    <rowFields count="1"><field x="0"/></rowFields>
                    <dataFields count="1"><dataField fld="1"/></dataFields>
                  </pivotTableDefinition>"#,
            ),
        ],
    );

    let result = inspect_pivot_report_metadata_from_workbook(&path, Path::new("pivot-model.xlsm"))
        .expect("inspect pivot metadata");

    assert_eq!(
        result,
        InspectPivotReportMetadataResult {
            mode: "openxml_package".to_string(),
            path: "pivot-model.xlsm".to_string(),
            pivot_table_count: 1,
            pivot_cache_count: 1,
            pivot_tables: vec![PivotTableReportSummary {
                name: Some("PivotTable2".to_string()),
                worksheet_name: Some("Total Revenue".to_string()),
                part_path: Some("xl/pivotTables/pivotTable1.xml".to_string()),
                range_ref: Some("B3:C16".to_string()),
                cache_id: Some("107".to_string()),
                source_type: Some("olap".to_string()),
                source_name: Some("ThisWorkbookDataModel".to_string()),
                source_range: None,
                connection_id: Some("8".to_string()),
                connection_name: Some("ThisWorkbookDataModel".to_string()),
                connection_type: Some("olap".to_string()),
                olap: true,
                data_model: true,
                stored_mdx_preview: None,
                stored_mdx_truncated: false,
                row_fields_sample: vec!["OrderDate (Month)".to_string()],
                column_fields_sample: Vec::new(),
                data_fields_sample: vec!["Total Revenue".to_string()],
                page_fields_sample: Vec::new(),
                warnings: Vec::new(),
            }],
            pivot_caches: vec![PivotCacheReportSummary {
                cache_id: Some("107".to_string()),
                source_type: Some("olap".to_string()),
                source_name: Some("ThisWorkbookDataModel".to_string()),
                source_range: None,
                connection_id: Some("8".to_string()),
                connection_name: Some("ThisWorkbookDataModel".to_string()),
                connection_type: Some("olap".to_string()),
                olap: true,
                data_model: true,
                cache_fields_sample: vec![
                    "OrderDate (Month)".to_string(),
                    "Total Revenue".to_string(),
                ],
                warnings: Vec::new(),
            }],
            warnings: Vec::new(),
        }
    );
}

#[test]
fn inspect_pivot_report_metadata_warns_on_missing_cache_relationship() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("pivot-missing.xlsx");
    write_zip_fixture(
        &path,
        &[
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                    <sheets><sheet name="Report" sheetId="1" r:id="rId1"/></sheets>
                    <pivotCaches><pivotCache cacheId="1" r:id="rId9"/></pivotCaches>
                  </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                  </Relationships>"#,
            ),
            ("xl/worksheets/sheet1.xml", r#"<worksheet/>"#),
        ],
    );

    let result =
        inspect_pivot_report_metadata_from_workbook(&path, Path::new("pivot-missing.xlsx"))
            .expect("inspect pivot metadata");

    assert_eq!(result.pivot_table_count, 0);
    assert_eq!(result.pivot_cache_count, 0);
    assert_eq!(
        result.warnings,
        vec!["pivot cache 1 is missing workbook relationship rId9".to_string()]
    );
}

#[tokio::test]
async fn inspect_pivot_report_metadata_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/sample.xlsm");
    write_zip_fixture(
        &workbook_path,
        &[
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                    <sheets><sheet name="Report" sheetId="1" r:id="rId1"/></sheets>
                  </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                  </Relationships>"#,
            ),
            ("xl/worksheets/sheet1.xml", r#"<worksheet/>"#),
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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_PIVOT_REPORT_METADATA_TOOL_NAME)
        })
        .expect("excel pivot report metadata tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "data/sample.xlsm" }).to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                INSPECT_PIVOT_REPORT_METADATA_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect pivot metadata");

    assert_eq!(
        serde_json::from_value::<InspectPivotReportMetadataResult>(
            output
                .post_tool_use_response("call-1", &payload)
                .expect("json response")
        )
        .expect("deserialize pivot metadata result")
        .path,
        "data/sample.xlsm".to_string()
    );
}

#[tokio::test]
async fn inspect_formula_sql_readiness_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/sample.xlsx");
    write_zip_fixture(
        &workbook_path,
        &[
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                    <sheets><sheet name="Summary" sheetId="1" r:id="rId1"/></sheets>
                  </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                  </Relationships>"#,
            ),
            (
                "xl/worksheets/sheet1.xml",
                r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                    <sheetData>
                      <row r="2">
                        <c r="A2"><v>7</v></c>
                        <c r="B2"><v>6</v></c>
                        <c r="D2"><f>A2+B2*2</f><v>19</v></c>
                      </row>
                    </sheetData>
                  </worksheet>"#,
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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_FORMULA_SQL_READINESS_TOOL_NAME)
        })
        .expect("excel formula sql readiness tool");
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "data/sample.xlsx",
            "sheet": { "type": "name", "name": "Summary" }
        })
        .to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                INSPECT_FORMULA_SQL_READINESS_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect formula sql readiness");

    let result = serde_json::from_value::<InspectFormulaSqlReadinessResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize formula sql readiness result");
    assert_eq!(result.path, "data/sample.xlsx".to_string());
    assert_eq!(
        result.readiness_counts,
        FormulaSqlReadinessCounts {
            scalar_row_local: 1,
            exact_lookup: 0,
            aligned_aggregate: 0,
            blocked: 0,
            malformed: 0,
            unsupported: 0,
        }
    );
    assert_eq!(result.formula_count, 1);
}

#[tokio::test]
async fn inspect_formula_cte_pipeline_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/sample.xlsx");
    write_formula_inventory_fixture(&workbook_path);

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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_FORMULA_CTE_PIPELINE_TOOL_NAME)
        })
        .expect("excel formula CTE pipeline tool");
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "data/sample.xlsx",
            "sheet": { "type": "name", "name": "Summary" }
        })
        .to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                INSPECT_FORMULA_CTE_PIPELINE_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect formula CTE pipeline");

    let result = serde_json::from_value::<InspectFormulaCtePipelineResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize formula CTE pipeline result");
    assert_eq!(result.path, "data/sample.xlsx".to_string());
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
            lint_finding_count: 0,
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
                lint_findings: Vec::new(),
                lexical_references: Vec::new(),
                connection_name: Some("Query - SalesQuery".to_string()),
                location: Some("SalesQuery".to_string()),
                command_preview: Some("SELECT * FROM [SalesQuery]".to_string()),
                command_type: None,
                workbook_connection_ids: Vec::new(),
                load_target_hint: PowerQueryLoadTargetHint::WorkbookConnection,
                worksheet_load_targets: Vec::new(),
                data_model_load_targets: Vec::new(),
            }],
            connections: vec![PowerQueryWorkbookConnectionSummary {
                id: None,
                name: Some("Query - SalesQuery".to_string()),
                connection_type: None,
                location: Some("SalesQuery".to_string()),
                command_preview: Some("SELECT * FROM [SalesQuery]".to_string()),
                command_type: None,
                query_name_hint: Some("SalesQuery".to_string()),
                load_target_hint: PowerQueryLoadTargetHint::WorkbookConnection,
            }],
            warnings: Vec::new(),
        }
    );
}

#[test]
fn extract_powerquery_queries_collects_lexical_reference_facts() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("lineage.xlsm");
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
                "customXml/item1.xml",
                build_data_mashup_xml_utf16(&[(
                    "Formulas/Section1.m",
                    concat!(
                        "section Section1;\n",
                        "shared SalesData = let\n",
                        "    Source = Excel.CurrentWorkbook(){[Name=\"OrdersTable\"]}[Content]\n",
                        "in\n",
                        "    Source;\n",
                        "shared #\"Staged Query\" = let\n",
                        "    Source = SalesData\n",
                        "in\n",
                        "    Source;\n",
                        "shared FinalQuery = let\n",
                        "    Source = #\"Staged Query\"\n",
                        "in\n",
                        "    Source;\n",
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
            query_count: 3,
            lint_finding_count: 0,
            queries: vec![
                ExtractedPowerQueryQuery {
                    name: "SalesData".to_string(),
                    source_part: "Formulas/Section1.m".to_string(),
                    source: concat!(
                        "shared SalesData = let\n",
                        "    Source = Excel.CurrentWorkbook(){[Name=\"OrdersTable\"]}[Content]\n",
                        "in\n",
                        "    Source;"
                    )
                    .to_string(),
                    source_truncated: false,
                    lint_findings: Vec::new(),
                    lexical_references: vec![PowerQueryLexicalReference {
                        kind: PowerQueryLexicalReferenceKind::WorkbookName,
                        target_name: "OrdersTable".to_string(),
                        evidence_kind: PowerQueryLexicalEvidenceKind::ExcelCurrentWorkbookName,
                        source_line: 2,
                        source_excerpt: "Excel.CurrentWorkbook(){[Name=\"OrdersTable\"]"
                            .to_string(),
                    }],
                    connection_name: None,
                    location: None,
                    command_preview: None,
                    command_type: None,
                    workbook_connection_ids: Vec::new(),
                    load_target_hint: PowerQueryLoadTargetHint::Unknown,
                    worksheet_load_targets: Vec::new(),
                    data_model_load_targets: Vec::new(),
                },
                ExtractedPowerQueryQuery {
                    name: "Staged Query".to_string(),
                    source_part: "Formulas/Section1.m".to_string(),
                    source: concat!(
                        "shared #\"Staged Query\" = let\n",
                        "    Source = SalesData\n",
                        "in\n",
                        "    Source;"
                    )
                    .to_string(),
                    source_truncated: false,
                    lint_findings: Vec::new(),
                    lexical_references: vec![PowerQueryLexicalReference {
                        kind: PowerQueryLexicalReferenceKind::QueryName,
                        target_name: "SalesData".to_string(),
                        evidence_kind: PowerQueryLexicalEvidenceKind::SharedQueryIdentifier,
                        source_line: 2,
                        source_excerpt: "SalesData".to_string(),
                    }],
                    connection_name: None,
                    location: None,
                    command_preview: None,
                    command_type: None,
                    workbook_connection_ids: Vec::new(),
                    load_target_hint: PowerQueryLoadTargetHint::Unknown,
                    worksheet_load_targets: Vec::new(),
                    data_model_load_targets: Vec::new(),
                },
                ExtractedPowerQueryQuery {
                    name: "FinalQuery".to_string(),
                    source_part: "Formulas/Section1.m".to_string(),
                    source: concat!(
                        "shared FinalQuery = let\n",
                        "    Source = #\"Staged Query\"\n",
                        "in\n",
                        "    Source;"
                    )
                    .to_string(),
                    source_truncated: false,
                    lint_findings: Vec::new(),
                    lexical_references: vec![PowerQueryLexicalReference {
                        kind: PowerQueryLexicalReferenceKind::QueryName,
                        target_name: "Staged Query".to_string(),
                        evidence_kind: PowerQueryLexicalEvidenceKind::SharedQueryIdentifier,
                        source_line: 2,
                        source_excerpt: "#\"Staged Query\"".to_string(),
                    }],
                    connection_name: None,
                    location: None,
                    command_preview: None,
                    command_type: None,
                    workbook_connection_ids: Vec::new(),
                    load_target_hint: PowerQueryLoadTargetHint::Unknown,
                    worksheet_load_targets: Vec::new(),
                    data_model_load_targets: Vec::new(),
                },
            ],
            connections: Vec::new(),
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
    assert_eq!(result.lint_finding_count, 0);
}

#[test]
fn extract_powerquery_queries_reports_bounded_lint_findings() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("lint.xlsm");
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
                "customXml/item1.xml",
                build_data_mashup_xml_utf16(&[(
                    "Formulas/Section1.m",
                    concat!(
                        "section Section1;\n",
                        "shared MissingIn = let\n",
                        "    Source = 1;\n",
                        "shared MissingLet = in 1;\n",
                        "shared EmptyBody = ;\n",
                    ),
                )]),
            ),
        ],
    );

    let result = crate::powerquery_extract::extract_powerquery_queries_from_workbook(&path, &path);

    assert_eq!(result.lint_finding_count, 3);
    assert_eq!(
        result.queries,
        vec![
            ExtractedPowerQueryQuery {
                name: "MissingIn".to_string(),
                source_part: "Formulas/Section1.m".to_string(),
                source: "shared MissingIn = let\n    Source = 1;".to_string(),
                source_truncated: false,
                lint_findings: vec![PowerQueryLintFinding {
                    code: PowerQueryLintCode::MissingInClauseForLetExpression,
                    message: "let expression is missing a matching `in` clause".to_string(),
                    source_line: Some(1),
                }],
                lexical_references: Vec::new(),
                connection_name: None,
                location: None,
                command_preview: None,
                command_type: None,
                workbook_connection_ids: Vec::new(),
                load_target_hint: PowerQueryLoadTargetHint::Unknown,
                worksheet_load_targets: Vec::new(),
                data_model_load_targets: Vec::new(),
            },
            ExtractedPowerQueryQuery {
                name: "MissingLet".to_string(),
                source_part: "Formulas/Section1.m".to_string(),
                source: "shared MissingLet = in 1;".to_string(),
                source_truncated: false,
                lint_findings: vec![PowerQueryLintFinding {
                    code: PowerQueryLintCode::MissingLetClauseForInExpression,
                    message: "`in` clause appears without a preceding `let` expression".to_string(),
                    source_line: Some(1),
                }],
                lexical_references: Vec::new(),
                connection_name: None,
                location: None,
                command_preview: None,
                command_type: None,
                workbook_connection_ids: Vec::new(),
                load_target_hint: PowerQueryLoadTargetHint::Unknown,
                worksheet_load_targets: Vec::new(),
                data_model_load_targets: Vec::new(),
            },
            ExtractedPowerQueryQuery {
                name: "EmptyBody".to_string(),
                source_part: "Formulas/Section1.m".to_string(),
                source: "shared EmptyBody = ;".to_string(),
                source_truncated: false,
                lint_findings: vec![PowerQueryLintFinding {
                    code: PowerQueryLintCode::EmptyQueryBody,
                    message: "shared query definition has no expression after `=`".to_string(),
                    source_line: Some(1),
                }],
                lexical_references: Vec::new(),
                connection_name: None,
                location: None,
                command_preview: None,
                command_type: None,
                workbook_connection_ids: Vec::new(),
                load_target_hint: PowerQueryLoadTargetHint::Unknown,
                worksheet_load_targets: Vec::new(),
                data_model_load_targets: Vec::new(),
            },
        ]
    );
}

#[test]
fn extract_powerquery_queries_maps_query_connections_and_data_model_hints() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("connections.xlsm");
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
                    <connection id="1" name="ModelConnection_ExternalData_1" type="5">
                      <dbPr connection="Data Model Connection" command="Customers" commandType="3"/>
                      <extLst>
                        <ext uri="{DE250136-89BD-433C-8126-D09CA5730AF9}" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
                          <x15:connection id="" model="1"/>
                        </ext>
                      </extLst>
                    </connection>
                    <connection id="4" name="Query - Customers" type="100">
                      <extLst>
                        <ext uri="{DE250136-89BD-433C-8126-D09CA5730AF9}" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
                          <x15:connection id="ff19990c-0640-4675-a1b3-e07f3be5840c"/>
                        </ext>
                      </extLst>
                    </connection>
                    <connection id="5" name="Query - SalesQuery" type="100">
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
                        "shared Customers = let\n",
                        "    Source = 1\n",
                        "in\n",
                        "    Source;\n",
                        "shared SalesQuery = let\n",
                        "    Source = 2\n",
                        "in\n",
                        "    Source;\n",
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
            query_count: 2,
            lint_finding_count: 0,
            queries: vec![
                ExtractedPowerQueryQuery {
                    name: "Customers".to_string(),
                    source_part: "Formulas/Section1.m".to_string(),
                    source: "shared Customers = let\n    Source = 1\nin\n    Source;".to_string(),
                    source_truncated: false,
                    lint_findings: Vec::new(),
                    lexical_references: Vec::new(),
                    connection_name: Some("Query - Customers".to_string()),
                    location: None,
                    command_preview: None,
                    command_type: None,
                    workbook_connection_ids: vec!["4".to_string(), "1".to_string()],
                    load_target_hint: PowerQueryLoadTargetHint::DataModel,
                    worksheet_load_targets: Vec::new(),
                    data_model_load_targets: Vec::new(),
                },
                ExtractedPowerQueryQuery {
                    name: "SalesQuery".to_string(),
                    source_part: "Formulas/Section1.m".to_string(),
                    source: "shared SalesQuery = let\n    Source = 2\nin\n    Source;".to_string(),
                    source_truncated: false,
                    lint_findings: Vec::new(),
                    lexical_references: Vec::new(),
                    connection_name: Some("Query - SalesQuery".to_string()),
                    location: Some("SalesQuery".to_string()),
                    command_preview: Some("SELECT * FROM [SalesQuery]".to_string()),
                    command_type: None,
                    workbook_connection_ids: vec!["5".to_string()],
                    load_target_hint: PowerQueryLoadTargetHint::WorkbookConnection,
                    worksheet_load_targets: Vec::new(),
                    data_model_load_targets: Vec::new(),
                },
            ],
            connections: vec![
                PowerQueryWorkbookConnectionSummary {
                    id: Some("1".to_string()),
                    name: Some("ModelConnection_ExternalData_1".to_string()),
                    connection_type: Some("5".to_string()),
                    location: None,
                    command_preview: Some("Customers".to_string()),
                    command_type: Some("3".to_string()),
                    query_name_hint: Some("Customers".to_string()),
                    load_target_hint: PowerQueryLoadTargetHint::DataModel,
                },
                PowerQueryWorkbookConnectionSummary {
                    id: Some("4".to_string()),
                    name: Some("Query - Customers".to_string()),
                    connection_type: Some("100".to_string()),
                    location: None,
                    command_preview: None,
                    command_type: None,
                    query_name_hint: Some("Customers".to_string()),
                    load_target_hint: PowerQueryLoadTargetHint::WorkbookConnection,
                },
                PowerQueryWorkbookConnectionSummary {
                    id: Some("5".to_string()),
                    name: Some("Query - SalesQuery".to_string()),
                    connection_type: Some("100".to_string()),
                    location: Some("SalesQuery".to_string()),
                    command_preview: Some("SELECT * FROM [SalesQuery]".to_string()),
                    command_type: None,
                    query_name_hint: Some("SalesQuery".to_string()),
                    load_target_hint: PowerQueryLoadTargetHint::WorkbookConnection,
                },
            ],
            warnings: Vec::new(),
        }
    );
}

#[test]
fn inspect_workbook_connections_reports_bounded_summaries_and_warnings() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("connections-inventory.xlsm");
    let long_command = "X".repeat(520);
    let workbook_xml = format!(
        r#"<connections xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                <connection id="1" name="ModelConnection_ExternalData_1" type="5">
                  <dbPr connection="Data Model Connection" command="Customers" commandType="3"/>
                  <extLst>
                    <ext uri="{{DE250136-89BD-433C-8126-D09CA5730AF9}}" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
                      <x15:connection id="" model="1"/>
                    </ext>
                  </extLst>
                </connection>
                <connection id="4" name="Query - Customers" type="100">
                  <dbPr connection="Provider=Microsoft.Mashup.OleDb.1;Data Source=$Workbook$;Location=&quot;Customers&quot;" command="{long_command}"/>
                </connection>
                <connection id="9" name="Legacy ODBC" type="2">
                  <dbPr connection="DSN=LegacyWarehouse" command="SELECT 1" commandType="2"/>
                </connection>
              </connections>"#
    );
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
            ("xl/connections.xml", workbook_xml.into_bytes()),
        ],
    );

    let result = inspect_workbook_connections_from_workbook(&path, &path);

    assert_eq!(
        result,
        InspectWorkbookConnectionsResult {
            mode: "read_only_inspection".to_string(),
            path: path.display().to_string(),
            connection_count: 3,
            connections: vec![
                PowerQueryWorkbookConnectionSummary {
                    id: Some("1".to_string()),
                    name: Some("ModelConnection_ExternalData_1".to_string()),
                    connection_type: Some("5".to_string()),
                    location: None,
                    command_preview: Some("Customers".to_string()),
                    command_type: Some("3".to_string()),
                    query_name_hint: Some("Customers".to_string()),
                    load_target_hint: PowerQueryLoadTargetHint::DataModel,
                },
                PowerQueryWorkbookConnectionSummary {
                    id: Some("4".to_string()),
                    name: Some("Query - Customers".to_string()),
                    connection_type: Some("100".to_string()),
                    location: Some("Customers".to_string()),
                    command_preview: Some("X".repeat(512)),
                    command_type: None,
                    query_name_hint: Some("Customers".to_string()),
                    load_target_hint: PowerQueryLoadTargetHint::WorkbookConnection,
                },
                PowerQueryWorkbookConnectionSummary {
                    id: Some("9".to_string()),
                    name: Some("Legacy ODBC".to_string()),
                    connection_type: Some("2".to_string()),
                    location: None,
                    command_preview: Some("SELECT 1".to_string()),
                    command_type: Some("2".to_string()),
                    query_name_hint: None,
                    load_target_hint: PowerQueryLoadTargetHint::WorkbookConnection,
                },
            ],
            warnings: vec![
                "command text for workbook connection `Query - Customers` truncated to 512 characters"
                    .to_string(),
                "query-name hint `Customers` maps to both workbook_connection and data_model load targets"
                    .to_string(),
                "workbook connection `Legacy ODBC` uses unsupported connection kind `2`; only bounded offline metadata is reported"
                    .to_string(),
            ],
        }
    );
}

#[test]
fn extract_powerquery_queries_reports_worksheet_table_load_targets() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("worksheet-loads.xlsx");
    write_zip_fixture_bytes(
        &path,
        &[
            (
                "[Content_Types].xml",
                br#"<Types>
                    <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  </Types>"#
                    .to_vec(),
            ),
            (
                "xl/workbook.xml",
                br#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                    <definedNames>
                      <definedName name="ExternalData_3">'Customers'!$A$1:$G$11</definedName>
                    </definedNames>
                  </workbook>"#
                    .to_vec(),
            ),
            (
                "xl/connections.xml",
                br#"<connections xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                    <connection id="2" name="Query - Customers" type="5">
                      <dbPr connection="Provider=Microsoft.Mashup.OleDb.1;Data Source=$Workbook$;Location=Customers;Extended Properties=&quot;&quot;" command="SELECT * FROM [Customers]"/>
                    </connection>
                  </connections>"#
                    .to_vec(),
            ),
            (
                "xl/queryTables/queryTable3.xml",
                br#"<queryTable xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" name="ExternalData_3" connectionId="2"/>"#
                    .to_vec(),
            ),
            (
                "xl/tables/table3.xml",
                br#"<table xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" name="Customers" displayName="Customers" ref="A1:G11" tableType="queryTable"/>"#
                    .to_vec(),
            ),
            (
                "xl/tables/_rels/table3.xml.rels",
                br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable" Target="../queryTables/queryTable3.xml"/>
                  </Relationships>"#
                    .to_vec(),
            ),
            (
                "customXml/item1.xml",
                build_data_mashup_xml_utf16(&[(
                    "Formulas/Section1.m",
                    concat!(
                        "section Section1;\n",
                        "shared Customers = let\n",
                        "    Source = 1\n",
                        "in\n",
                        "    Source;\n",
                    ),
                )]),
            ),
        ],
    );

    let result = crate::powerquery_extract::extract_powerquery_queries_from_workbook(&path, &path);

    assert_eq!(
        result.queries,
        vec![ExtractedPowerQueryQuery {
            name: "Customers".to_string(),
            source_part: "Formulas/Section1.m".to_string(),
            source: "shared Customers = let\n    Source = 1\nin\n    Source;".to_string(),
            source_truncated: false,
            lint_findings: Vec::new(),
            lexical_references: Vec::new(),
            connection_name: Some("Query - Customers".to_string()),
            location: Some("Customers".to_string()),
            command_preview: Some("SELECT * FROM [Customers]".to_string()),
            command_type: None,
            workbook_connection_ids: vec!["2".to_string()],
            load_target_hint: PowerQueryLoadTargetHint::WorkbookConnection,
            worksheet_load_targets: vec![PowerQueryWorksheetLoadTarget {
                external_data_name: Some("ExternalData_3".to_string()),
                table_name: Some("Customers".to_string()),
                sheet_name: Some("Customers".to_string()),
                range_ref: Some("$A$1:$G$11".to_string()),
            }],
            data_model_load_targets: Vec::new(),
        }]
    );
}

#[test]
fn extract_powerquery_queries_reports_data_model_load_targets_and_pivot_consumers() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("data-model-routing.xlsm");
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
                br#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                    xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                    xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
                    <pivotCaches>
                      <pivotCache cacheId="107" r:id="rId1"/>
                    </pivotCaches>
                    <extLst>
                      <ext uri="{FCE2AD5D-F65C-4FA6-A056-5C36A1767C68}">
                        <x15:dataModel>
                          <x15:modelTables>
                            <x15:modelTable id="Sales_model_id" name="Sales" connection="Query - Sales"/>
                          </x15:modelTables>
                        </x15:dataModel>
                      </ext>
                    </extLst>
                  </workbook>"#
                    .to_vec(),
            ),
            (
                "xl/_rels/workbook.xml.rels",
                br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" Target="pivotCache/pivotCacheDefinition1.xml"/>
                  </Relationships>"#
                    .to_vec(),
            ),
            (
                "xl/connections.xml",
                br#"<connections xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                    <connection id="7" name="Query - Sales" type="100">
                      <extLst>
                        <ext uri="{DE250136-89BD-433C-8126-D09CA5730AF9}" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
                          <x15:connection id="query-guid"/>
                        </ext>
                      </extLst>
                    </connection>
                    <connection id="8" name="ThisWorkbookDataModel" type="5">
                      <dbPr connection="Data Model Connection" command="Model" commandType="1"/>
                      <extLst>
                        <ext uri="{DE250136-89BD-433C-8126-D09CA5730AF9}" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
                          <x15:connection id="" model="1"/>
                        </ext>
                      </extLst>
                    </connection>
                  </connections>"#
                    .to_vec(),
            ),
            (
                "xl/pivotCache/pivotCacheDefinition1.xml",
                br#"<pivotCacheDefinition xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                    <cacheSource type="external" connectionId="8"/>
                  </pivotCacheDefinition>"#
                    .to_vec(),
            ),
            (
                "xl/pivotTables/pivotTable1.xml",
                br#"<pivotTableDefinition xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" name="PivotTable2" cacheId="107">
                    <location ref="A3:D12"/>
                  </pivotTableDefinition>"#
                    .to_vec(),
            ),
            (
                "customXml/item1.xml",
                build_data_mashup_xml_utf16(&[(
                    "Formulas/Section1.m",
                    concat!(
                        "section Section1;\n",
                        "shared Sales = let\n",
                        "    Source = 1\n",
                        "in\n",
                        "    Source;\n",
                    ),
                )]),
            ),
        ],
    );

    let result = crate::powerquery_extract::extract_powerquery_queries_from_workbook(&path, &path);

    assert_eq!(
        result.queries,
        vec![ExtractedPowerQueryQuery {
            name: "Sales".to_string(),
            source_part: "Formulas/Section1.m".to_string(),
            source: "shared Sales = let\n    Source = 1\nin\n    Source;".to_string(),
            source_truncated: false,
            lint_findings: Vec::new(),
            lexical_references: Vec::new(),
            connection_name: Some("Query - Sales".to_string()),
            location: None,
            command_preview: None,
            command_type: None,
            workbook_connection_ids: vec!["7".to_string()],
            load_target_hint: PowerQueryLoadTargetHint::WorkbookConnection,
            worksheet_load_targets: Vec::new(),
            data_model_load_targets: vec![PowerQueryDataModelLoadTarget {
                model_table_name: Some("Sales".to_string()),
                model_table_id: Some("Sales_model_id".to_string()),
                source_connection_name: Some("Query - Sales".to_string()),
                source_workbook_connection_ids: vec!["7".to_string()],
                model_connection_ids: vec!["8".to_string()],
                pivot_consumers: vec![PowerQueryDataModelPivotConsumer {
                    pivot_table_name: Some("PivotTable2".to_string()),
                    pivot_table_part: Some("xl/pivotTables/pivotTable1.xml".to_string()),
                    pivot_cache_id: Some("107".to_string()),
                    source_connection_id: Some("8".to_string()),
                    location_ref: Some("A3:D12".to_string()),
                }],
            }],
        }]
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
fn generate_powerquery_review_bundle_writes_manifest_and_reports() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("review.xlsm");
    let bundle_path = dir.path().join("bundles/review");
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
                        "shared SalesData = let\n",
                        "    Source = Excel.CurrentWorkbook(){[Name=\"OrdersTable\"]}[Content]\n",
                        "in\n",
                        "    Source;\n",
                        "shared BrokenQuery = let\n",
                        "    Source = SalesData;\n",
                    ),
                )]),
            ),
        ],
    );

    let result =
        crate::powerquery_review_bundle::generate_powerquery_review_bundle_with_display_path(
            &workbook_path,
            Path::new("review.xlsm"),
            &bundle_path,
            Path::new("bundles/review"),
        )
        .expect("generate review bundle");

    assert_eq!(
        result,
        GeneratePowerQueryReviewBundleResult {
            path: "review.xlsm".to_string(),
            bundle_path: "bundles/review".to_string(),
            manifest_path: "bundles/review/manifest.json".to_string(),
            manifest: PowerQueryReviewBundleManifest {
                bundle_name: "review_powerquery_review_bundle".to_string(),
                query_count: 2,
                queries: vec![
                    PowerQueryReviewBundleQuerySummary {
                        name: "SalesData".to_string(),
                        source_path: "queries/01_salesdata.m".to_string(),
                        normalized_source_path: Some("normalized_m/01_salesdata.m".to_string(),),
                        lint_finding_count: 0,
                        lineage_reference_count: 1,
                    },
                    PowerQueryReviewBundleQuerySummary {
                        name: "BrokenQuery".to_string(),
                        source_path: "queries/02_brokenquery.m".to_string(),
                        normalized_source_path: Some("normalized_m/02_brokenquery.m".to_string(),),
                        lint_finding_count: 1,
                        lineage_reference_count: 1,
                    },
                ],
                lint_summary_path: "reports/lint-summary.json".to_string(),
                lineage_summary_path: "reports/lineage-summary.json".to_string(),
                normalization_status: PowerQueryBundleNormalizationStatus::CopyArtifacts,
            },
            warnings: Vec::new(),
        }
    );

    assert_eq!(
        std::fs::read_to_string(bundle_path.join("manifest.json")).expect("read manifest"),
        serde_json::to_string_pretty(&result.manifest).expect("serialize manifest")
    );
    assert_eq!(
        std::fs::read_to_string(bundle_path.join("queries/01_salesdata.m"))
            .expect("read first query file"),
        concat!(
            "shared SalesData = let\n",
            "    Source = Excel.CurrentWorkbook(){[Name=\"OrdersTable\"]}[Content]\n",
            "in\n",
            "    Source;"
        )
    );
    assert_eq!(
        std::fs::read_to_string(bundle_path.join("normalized_m/01_salesdata.m"))
            .expect("read first normalized query file"),
        concat!(
            "shared SalesData = let\n",
            "    Source = Excel.CurrentWorkbook(){[Name=\"OrdersTable\"]}[Content]\n",
            "in\n",
            "    Source;\n"
        )
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(
            &std::fs::read_to_string(bundle_path.join("reports/lint-summary.json"))
                .expect("read lint summary")
        )
        .expect("parse lint summary")[1]["findings"]
            .as_array()
            .expect("lint findings array")
            .len(),
        1
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(
            &std::fs::read_to_string(bundle_path.join("reports/lineage-summary.json"))
                .expect("read lineage summary")
        )
        .expect("parse lineage summary")[0]["references"]
            .as_array()
            .expect("lineage references array")
            .len(),
        1
    );
}

#[tokio::test]
async fn generate_powerquery_review_bundle_tool_resolves_relative_paths_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/review.xlsm");
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
                        "in\n",
                        "    Source;\n",
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
                == ToolName::namespaced(
                    EXCEL_NAMESPACE,
                    GENERATE_POWERQUERY_REVIEW_BUNDLE_TOOL_NAME,
                )
        })
        .expect("excel Power Query review bundle tool");
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "data/review.xlsm",
            "output_bundle_path": "bundles/review"
        })
        .to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                GENERATE_POWERQUERY_REVIEW_BUNDLE_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should generate review bundle");

    let result = serde_json::from_value::<GeneratePowerQueryReviewBundleResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize review bundle result");
    assert_eq!(result.path, "data/review.xlsm".to_string());
    assert_eq!(result.bundle_path, "bundles/review".to_string());
}

#[test]
fn inspect_workbook_connections_reports_connections_without_powerquery_absence_warning() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("connections.xlsx");
    write_zip_fixture(
        &workbook_path,
        &[
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                    <sheets><sheet name="Summary" sheetId="1" r:id="rId1"/></sheets>
                  </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                  </Relationships>"#,
            ),
            ("xl/worksheets/sheet1.xml", r#"<worksheet/>"#),
            (
                "xl/connections.xml",
                r#"<connections xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                    <connection id="5" name="Query - SalesQuery" type="100">
                      <dbPr connection="Provider=Microsoft.Mashup.OleDb.1;Data Source=$Workbook$;Location=&quot;SalesQuery&quot;" command="SELECT * FROM [SalesQuery]"/>
                    </connection>
                  </connections>"#,
            ),
        ],
    );

    let result =
        inspect_workbook_connections_from_workbook(&workbook_path, Path::new("connections.xlsx"));

    assert_eq!(
        result,
        InspectWorkbookConnectionsResult {
            mode: "read_only_inspection".to_string(),
            path: "connections.xlsx".to_string(),
            connection_count: 1,
            connections: vec![PowerQueryWorkbookConnectionSummary {
                id: Some("5".to_string()),
                name: Some("Query - SalesQuery".to_string()),
                connection_type: Some("100".to_string()),
                location: Some("SalesQuery".to_string()),
                command_preview: Some("SELECT * FROM [SalesQuery]".to_string()),
                command_type: None,
                query_name_hint: Some("SalesQuery".to_string()),
                load_target_hint: PowerQueryLoadTargetHint::WorkbookConnection,
            }],
            warnings: Vec::new(),
        }
    );
}

#[tokio::test]
async fn inspect_workbook_connections_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/connections.xlsx");
    write_zip_fixture(
        &workbook_path,
        &[
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                    <sheets><sheet name="Summary" sheetId="1" r:id="rId1"/></sheets>
                  </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                  </Relationships>"#,
            ),
            ("xl/worksheets/sheet1.xml", r#"<worksheet/>"#),
            (
                "xl/connections.xml",
                r#"<connections xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                    <connection id="8" name="LegacyImport" type="2"/>
                  </connections>"#,
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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_CONNECTIONS_TOOL_NAME)
        })
        .expect("excel workbook connections tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "data/connections.xlsx" }).to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                INSPECT_WORKBOOK_CONNECTIONS_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect workbook connections");

    let result = serde_json::from_value::<InspectWorkbookConnectionsResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize workbook connections result");
    assert_eq!(result.path, "data/connections.xlsx".to_string());
    assert_eq!(result.connection_count, 1);
    assert_eq!(
        result.warnings,
        vec![
            "workbook connection `LegacyImport` uses unsupported connection kind `2`; only bounded offline metadata is reported"
                .to_string()
        ]
    );
}

#[tokio::test]
async fn inspect_workbook_defined_names_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/defined-names.xlsx");
    write_defined_name_formula_fixture(&workbook_path);

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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_DEFINED_NAMES_TOOL_NAME)
        })
        .expect("excel workbook defined names tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "data/defined-names.xlsx" }).to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                INSPECT_WORKBOOK_DEFINED_NAMES_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect workbook defined names");

    let result = serde_json::from_value::<InspectWorkbookDefinedNamesResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize workbook defined names result");
    assert_eq!(result.path, "data/defined-names.xlsx".to_string());
    assert_eq!(result.defined_name_count, 1);
    assert_eq!(
        result.defined_names,
        vec![DefinedNameSummary {
            name: "Threshold".to_string(),
            sheet_scope: None,
            local_sheet_id: None,
            hidden: None,
            target: "42".to_string(),
            truncated: false,
        }]
    );
    assert!(result.warnings.is_empty());
}

#[test]
fn inspect_workbook_external_links_reports_bounded_inventory_and_warnings() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("external-links.xlsx");
    write_workbook_external_links_fixture(&path);

    let result =
        inspect_workbook_external_links_from_workbook(&path, Path::new("external-links.xlsx"))
            .expect("external link inventory should succeed");

    assert_eq!(
        result,
        InspectWorkbookExternalLinksResult {
            mode: "read_only_inspection".to_string(),
            path: "external-links.xlsx".to_string(),
            external_link_count: 2,
            external_links: vec![
                WorkbookExternalLinkSummary {
                    part_path: "xl/externalLinks/externalLink1.xml".to_string(),
                    workbook_relationship_id: Some("rId2".to_string()),
                    workbook_relationship_target: Some(
                        "xl/externalLinks/externalLink1.xml".to_string()
                    ),
                    detail_kind: "external_book".to_string(),
                    detail_relationship_id: Some("rId1".to_string()),
                    detail_relationship_target: Some("file:///tmp/source.xlsx".to_string()),
                },
                WorkbookExternalLinkSummary {
                    part_path: "xl/externalLinks/externalLink2.xml".to_string(),
                    workbook_relationship_id: None,
                    workbook_relationship_target: None,
                    detail_kind: "dde_link".to_string(),
                    detail_relationship_id: None,
                    detail_relationship_target: None,
                },
            ],
            inventory_truncated: false,
            warnings: vec![
                "external-link part `xl/externalLinks/externalLink2.xml` has no matching workbook relationship entry"
                    .to_string(),
                "external-link part `xl/externalLinks/externalLink2.xml` uses unsupported detail kind `dde_link`; only bounded metadata is reported"
                    .to_string(),
            ],
        }
    );
}

#[test]
fn inspect_workbook_external_links_rejects_xlsb_stage_gap() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsb");
    write_embedded_xlsb_fixture(&path);

    let err = inspect_workbook_external_links_from_workbook(&path, Path::new("sample.xlsb"))
        .expect_err("xlsb external links should stay unsupported");
    assert_eq!(
        err.to_string(),
        "excel.inspect_workbook_external_links supports only .xlsx and .xlsm in this stage; .xlsb external-link inventory remains unsupported"
            .to_string()
    );
}

#[tokio::test]
async fn inspect_workbook_external_links_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/external-links.xlsx");
    write_workbook_external_links_fixture(&workbook_path);

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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_EXTERNAL_LINKS_TOOL_NAME)
        })
        .expect("excel workbook external links tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "data/external-links.xlsx" }).to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                INSPECT_WORKBOOK_EXTERNAL_LINKS_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect workbook external links");

    let result = serde_json::from_value::<InspectWorkbookExternalLinksResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize workbook external links result");
    assert_eq!(result.path, "data/external-links.xlsx".to_string());
    assert_eq!(result.external_link_count, 2);
}

#[test]
fn inspect_workbook_used_ranges_reports_openxml_sheet_ranges() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("used-ranges.xlsx");
    write_workbook_used_ranges_fixture(&path);

    let result = inspect_workbook_used_ranges_from_workbook(&path, Path::new("used-ranges.xlsx"))
        .expect("used range inventory should succeed");

    assert_eq!(
        result,
        InspectWorkbookUsedRangesResult {
            mode: "read_only_inspection".to_string(),
            path: "used-ranges.xlsx".to_string(),
            sheet_count: 2,
            sheet_used_ranges: vec![
                WorkbookUsedRangeSummary {
                    sheet_name: "Summary".to_string(),
                    sheet_id: Some(1),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    used_range: Some(SheetDimension {
                        reference: "A1:D8".to_string(),
                    }),
                },
                WorkbookUsedRangeSummary {
                    sheet_name: "Data".to_string(),
                    sheet_id: Some(2),
                    part_path: Some("xl/worksheets/sheet2.xml".to_string()),
                    used_range: None,
                },
            ],
            inventory_truncated: false,
            warnings: Vec::new(),
        }
    );
}

#[test]
fn inspect_workbook_used_ranges_reads_real_xlsb_dimensions() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("sample.xlsb");
    write_embedded_xlsb_fixture(&path);

    let result = inspect_workbook_used_ranges_from_workbook(&path, Path::new("sample.xlsb"))
        .expect("xlsb used range inventory should succeed");

    assert_eq!(result.mode, "read_only_inspection".to_string());
    assert_eq!(result.path, "sample.xlsb".to_string());
    assert_eq!(result.sheet_count, 1);
    assert_eq!(
        result.sheet_used_ranges,
        vec![WorkbookUsedRangeSummary {
            sheet_name: "Лист1".to_string(),
            sheet_id: None,
            part_path: Some("xl/worksheets/sheet1.bin".to_string()),
            used_range: Some(SheetDimension {
                reference: "A1:J3".to_string(),
            }),
        }]
    );
    assert!(result.warnings.is_empty());
}

#[tokio::test]
async fn inspect_workbook_used_ranges_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/used-ranges.xlsx");
    write_workbook_used_ranges_fixture(&workbook_path);

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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_USED_RANGES_TOOL_NAME)
        })
        .expect("excel workbook used ranges tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "data/used-ranges.xlsx" }).to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                INSPECT_WORKBOOK_USED_RANGES_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect workbook used ranges");

    let result = serde_json::from_value::<InspectWorkbookUsedRangesResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize workbook used ranges result");
    assert_eq!(result.path, "data/used-ranges.xlsx".to_string());
    assert_eq!(result.sheet_count, 2);
}

#[test]
fn generate_workbook_migration_manifest_writes_deterministic_manifest() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("review.xlsm");
    let bundle_path = dir.path().join("bundles/review");
    write_workbook_migration_fixture(&workbook_path);

    let result =
        crate::workbook_migration_manifest::generate_workbook_migration_manifest_with_display_path(
            &workbook_path,
            Path::new("review.xlsm"),
            &bundle_path,
            Path::new("bundles/review"),
            None,
        )
        .expect("generate workbook migration manifest");

    assert_eq!(
        result,
        GenerateWorkbookMigrationManifestResult {
            path: "review.xlsm".to_string(),
            bundle_path: "bundles/review".to_string(),
            manifest_path: "bundles/review/manifest.json".to_string(),
            manifest: WorkbookMigrationManifest {
                bundle_name: "review".to_string(),
                workbook: InspectWorkbookResult {
                    path: "review.xlsm".to_string(),
                    format: WorkbookFormat::Xlsm,
                    package_part_count: 10,
                    package_parts_sample: vec![
                        "[Content_Types].xml".to_string(),
                        "xl/workbook.xml".to_string(),
                        "xl/_rels/workbook.xml.rels".to_string(),
                        "xl/worksheets/sheet1.xml".to_string(),
                        "xl/worksheets/sheet2.xml".to_string(),
                        "xl/worksheets/_rels/sheet1.xml.rels".to_string(),
                        "xl/pivotCache/pivotCacheDefinition1.xml".to_string(),
                        "xl/pivotTables/pivotTable1.xml".to_string(),
                        "customXml/item1.xml".to_string(),
                        "xl/vbaProject.bin".to_string(),
                    ],
                    sheets: vec![
                        SheetSummary {
                            name: Some("Summary".to_string()),
                            sheet_id: Some(1),
                            relationship_id: Some("rId1".to_string()),
                            part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                            visibility: SheetVisibility::Visible,
                            kind: SheetKind::Worksheet,
                        },
                        SheetSummary {
                            name: Some("Data".to_string()),
                            sheet_id: Some(2),
                            relationship_id: Some("rId2".to_string()),
                            part_path: Some("xl/worksheets/sheet2.xml".to_string()),
                            visibility: SheetVisibility::Visible,
                            kind: SheetKind::Worksheet,
                        },
                    ],
                    markers: WorkbookMarkers {
                        has_vba_project: true,
                        has_macro_enabled_package: true,
                        has_power_query: true,
                        has_connections: false,
                        has_custom_xml: true,
                        has_external_links: false,
                        has_tables: false,
                        has_comments: false,
                        has_drawings: false,
                        has_embedded_objects: false,
                        has_charts: false,
                        has_pivot_tables: true,
                        has_formulas: true,
                        has_xlsb_package: false,
                    },
                    marker_summaries: vec![
                        MarkerSummary {
                            category: "vba_project".to_string(),
                            count: 1,
                            part_paths_sample: vec!["xl/vbaProject.bin".to_string()],
                        },
                        MarkerSummary {
                            category: "power_query".to_string(),
                            count: 1,
                            part_paths_sample: vec!["customXml/item1.xml".to_string()],
                        },
                        MarkerSummary {
                            category: "custom_xml".to_string(),
                            count: 1,
                            part_paths_sample: vec!["customXml/item1.xml".to_string()],
                        },
                        MarkerSummary {
                            category: "pivot_tables".to_string(),
                            count: 2,
                            part_paths_sample: vec![
                                "xl/pivotCache/pivotCacheDefinition1.xml".to_string(),
                                "xl/pivotTables/pivotTable1.xml".to_string(),
                            ],
                        },
                        MarkerSummary {
                            category: "formulas".to_string(),
                            count: 1,
                            part_paths_sample: vec!["xl/worksheets/sheet1.xml".to_string()],
                        },
                    ],
                    warnings: Vec::new(),
                },
                formula_sheets: vec![
                    WorkbookMigrationFormulaSheetSummary {
                        sheet: SheetPreview {
                            name: "Summary".to_string(),
                            sheet_id: Some(1),
                            part_path: "xl/worksheets/sheet1.xml".to_string(),
                        },
                        formula_count: Some(1),
                        readiness_counts: Some(FormulaSqlReadinessCounts {
                            scalar_row_local: 1,
                            exact_lookup: 0,
                            aligned_aggregate: 0,
                            blocked: 0,
                            malformed: 0,
                            unsupported: 0,
                        }),
                        blocked_reason_counts: Vec::new(),
                        dependency_node_count: Some(1),
                        dependency_cycle_count: Some(0),
                        dependency_unsupported_formula_count: Some(0),
                        readiness_truncated: Some(false),
                        dependency_truncated: Some(false),
                        readiness_unavailable_reason: None,
                        dependency_unavailable_reason: None,
                        warnings: Vec::new(),
                    },
                    WorkbookMigrationFormulaSheetSummary {
                        sheet: SheetPreview {
                            name: "Data".to_string(),
                            sheet_id: Some(2),
                            part_path: "xl/worksheets/sheet2.xml".to_string(),
                        },
                        formula_count: Some(0),
                        readiness_counts: Some(FormulaSqlReadinessCounts::default()),
                        blocked_reason_counts: Vec::new(),
                        dependency_node_count: Some(0),
                        dependency_cycle_count: Some(0),
                        dependency_unsupported_formula_count: Some(0),
                        readiness_truncated: Some(false),
                        dependency_truncated: Some(false),
                        readiness_unavailable_reason: None,
                        dependency_unavailable_reason: None,
                        warnings: Vec::new(),
                    },
                ],
                formula_sql_lineage: vec![WorkbookMigrationFormulaSqlLineageEntry {
                    source_id: "Summary!D2".to_string(),
                    sheet: "Summary".to_string(),
                    reference: "D2".to_string(),
                    family: crate::formula_sql_readiness::FormulaSqlReadinessFamily::ScalarRowLocal,
                    readiness_state: WorkbookMigrationFormulaSqlLineageState::Ready,
                    sql_expression: Some("([col_a] + ([col_b] * 2))".to_string()),
                    blocker_reasons: Vec::new(),
                    warnings: Vec::new(),
                }],
                power_query: WorkbookMigrationPowerQuerySummary {
                    has_power_query: true,
                    query_count: 2,
                    lint_finding_count: 1,
                    connection_count: 0,
                    query_names: vec!["SalesData".to_string(), "BrokenQuery".to_string()],
                    warnings: Vec::new(),
                },
                pivot: WorkbookMigrationPivotSummary {
                    pivot_table_count: 1,
                    pivot_cache_count: 1,
                    pivot_table_names: vec!["PivotTable1".to_string()],
                    warnings: Vec::new(),
                },
                vba: WorkbookMigrationVbaSummary {
                    has_vba_project: true,
                    module_count: 1,
                    module_names: vec!["MyModule".to_string()],
                    warnings: Vec::new(),
                },
                unsupported_sections: vec![
                    "full_workbook_translation_or_apply_not_supported".to_string(),
                    "offline_read_only_manifest_no_live_excel_or_write_surfaces".to_string(),
                ],
                warnings: Vec::new(),
            },
            warnings: Vec::new(),
        }
    );
    assert_eq!(
        std::fs::read_to_string(bundle_path.join("manifest.json")).expect("read manifest"),
        serde_json::to_string_pretty(&result.manifest).expect("serialize manifest")
    );
}

#[tokio::test]
async fn generate_workbook_migration_manifest_tool_resolves_relative_paths_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/review.xlsm");
    write_workbook_migration_fixture(&workbook_path);

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
                == ToolName::namespaced(
                    EXCEL_NAMESPACE,
                    GENERATE_WORKBOOK_MIGRATION_MANIFEST_TOOL_NAME,
                )
        })
        .expect("excel workbook migration manifest tool");
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "data/review.xlsm",
            "output_bundle_path": "bundles/review"
        })
        .to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(
                EXCEL_NAMESPACE,
                GENERATE_WORKBOOK_MIGRATION_MANIFEST_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should generate workbook migration manifest");

    let result = serde_json::from_value::<GenerateWorkbookMigrationManifestResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize manifest result");
    assert_eq!(result.path, "data/review.xlsm".to_string());
    assert_eq!(result.bundle_path, "bundles/review".to_string());
    assert_eq!(
        result.manifest_path,
        "bundles/review/manifest.json".to_string()
    );
}

#[tokio::test]
async fn generate_workbook_migration_manifest_tool_rejects_unscoped_output_bundle_path() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/review.xlsm");
    write_workbook_migration_fixture(&workbook_path);

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
                == ToolName::namespaced(
                    EXCEL_NAMESPACE,
                    GENERATE_WORKBOOK_MIGRATION_MANIFEST_TOOL_NAME,
                )
        })
        .expect("excel workbook migration manifest tool");

    for invalid_output_bundle_path in ["../escape", "https://example.test/bundles/review"] {
        let payload = ToolPayload::Function {
            arguments: json!({
                "path": "data/review.xlsm",
                "output_bundle_path": invalid_output_bundle_path,
            })
            .to_string(),
        };
        let err = tool
            .handle(ToolCall {
                turn_id: "turn-1".to_string(),
                call_id: "call-1".to_string(),
                tool_name: ToolName::namespaced(
                    EXCEL_NAMESPACE,
                    GENERATE_WORKBOOK_MIGRATION_MANIFEST_TOOL_NAME,
                ),
                model: "gpt-test".to_string(),
                truncation_policy: TruncationPolicy::Bytes(1024),
                conversation_history: ConversationHistory::default(),
                turn_item_emitter: Arc::new(NoopTurnItemEmitter),
                payload,
            })
            .await;

        let message = match err {
            Ok(_) => panic!("invalid output bundle path should fail"),
            Err(FunctionCallError::RespondToModel(message)) => message,
            Err(other) => panic!("unexpected error variant: {other:?}"),
        };
        assert!(
            message.contains("output_bundle_path"),
            "expected output_bundle_path error, got: {message}"
        );
    }
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
