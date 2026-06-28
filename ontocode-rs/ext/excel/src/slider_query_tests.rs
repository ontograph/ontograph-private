use std::path::Path;

use pretty_assertions::assert_eq;
use tempfile::tempdir;

use crate::slider_query::FormulaDependencySummary;
use crate::slider_query::GenerateSliderQueryPackageResult;
use crate::slider_query::ScanSheetFormulasDependencyResult;
use crate::slider_query::SliderQueryBlockedFormulaSummary;
use crate::slider_query::SliderQueryGeneratedQuerySummary;
use crate::slider_query::SliderQueryGeneratedQueryType;
use crate::slider_query::SliderQueryPackageManifest;
use crate::slider_query::generate_slider_query_package_with_display_path;
use crate::slider_query::scan_sheet_formulas_dependency_with_display_path;
use crate::tests::write_zip_fixture;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;

fn write_phase2_fixture(path: &Path) {
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
                    <sheet name="Sheet1" sheetId="1" r:id="rId1"/>
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
                      <c r="A1"><v>10</v></c>
                      <c r="B1"><f>A1*2</f><v>20</v></c>
                      <c r="C1"><f>B1+5</f><v>25</v></c>
                      <c r="D1"><f>OFFSET(A1,1,1)</f></c>
                      <c r="E1"><f>D1+1</f></c>
                    </row>
                    <row r="2">
                      <c r="A2"><f>B2</f></c>
                      <c r="B2"><f>A2</f></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

#[test]
fn scan_sheet_formulas_dependency_reports_dependencies_cycles_and_blockers() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("phase2.xlsx");
    write_phase2_fixture(&workbook_path);

    let result = scan_sheet_formulas_dependency_with_display_path(
        &workbook_path,
        Path::new("phase2.xlsx"),
        &SheetSelector::Name {
            name: "Sheet1".to_string(),
        },
        Some(10),
    )
    .expect("scan worksheet dependencies");

    assert_eq!(
        result,
        ScanSheetFormulasDependencyResult {
            path: "phase2.xlsx".to_string(),
            sheet: SheetPreview {
                name: "Sheet1".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            max_formulas_applied: 10,
            nodes: vec![
                FormulaDependencySummary {
                    cell: "B1".to_string(),
                    formula: "A1*2".to_string(),
                    dependencies: vec!["A1".to_string()],
                    has_cycle: false,
                    is_supported: true,
                    unsupported_reason: None,
                },
                FormulaDependencySummary {
                    cell: "C1".to_string(),
                    formula: "B1+5".to_string(),
                    dependencies: vec!["B1".to_string()],
                    has_cycle: false,
                    is_supported: true,
                    unsupported_reason: None,
                },
                FormulaDependencySummary {
                    cell: "D1".to_string(),
                    formula: "OFFSET(A1,1,1)".to_string(),
                    dependencies: vec!["A1".to_string()],
                    has_cycle: false,
                    is_supported: false,
                    unsupported_reason: Some("volatile_function_offset".to_string()),
                },
                FormulaDependencySummary {
                    cell: "E1".to_string(),
                    formula: "D1+1".to_string(),
                    dependencies: vec!["D1".to_string()],
                    has_cycle: false,
                    is_supported: true,
                    unsupported_reason: None,
                },
                FormulaDependencySummary {
                    cell: "A2".to_string(),
                    formula: "B2".to_string(),
                    dependencies: vec!["B2".to_string()],
                    has_cycle: true,
                    is_supported: true,
                    unsupported_reason: None,
                },
                FormulaDependencySummary {
                    cell: "B2".to_string(),
                    formula: "A2".to_string(),
                    dependencies: vec!["A2".to_string()],
                    has_cycle: true,
                    is_supported: true,
                    unsupported_reason: None,
                },
            ],
            cycles_detected: vec![vec!["A2".to_string(), "B2".to_string()]],
            truncated: false,
            warnings: Vec::new(),
        }
    );
}

#[test]
fn generate_slider_query_package_writes_manifest_and_clean_query_files() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("phase2.xlsx");
    let package_path = dir.path().join("packages/sheet1");
    write_phase2_fixture(&workbook_path);

    let result = generate_slider_query_package_with_display_path(
        &workbook_path,
        Path::new("phase2.xlsx"),
        &SheetSelector::Name {
            name: "Sheet1".to_string(),
        },
        &package_path,
        Path::new("packages/sheet1"),
        Some(10),
    )
    .expect("generate slider query package");

    assert_eq!(
        result,
        GenerateSliderQueryPackageResult {
            path: "phase2.xlsx".to_string(),
            sheet: SheetPreview {
                name: "Sheet1".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            package_path: "packages/sheet1".to_string(),
            manifest_path: "packages/sheet1/manifest.json".to_string(),
            manifest: SliderQueryPackageManifest {
                package_name: "sheet1_slider_package".to_string(),
                generated_queries: vec![SliderQueryGeneratedQuerySummary {
                    name: "sheet1_prepared".to_string(),
                    r#type: SliderQueryGeneratedQueryType::PreparedColumns,
                    sql_path: "queries/sheet1_prepared.sql".to_string(),
                    variable_path: "variables/sheet1_prepared.json".to_string(),
                    blocked_path: "queries/sheet1_blocked.json".to_string(),
                }],
                blocked_formulas: vec![
                    SliderQueryBlockedFormulaSummary {
                        cell: "D1".to_string(),
                        formula: "OFFSET(A1,1,1)".to_string(),
                        reason: "volatile_function_offset".to_string(),
                        blocked_by: Vec::new(),
                        dependencies: vec!["A1".to_string()],
                    },
                    SliderQueryBlockedFormulaSummary {
                        cell: "E1".to_string(),
                        formula: "D1+1".to_string(),
                        reason: "blocked_dependency".to_string(),
                        blocked_by: vec!["D1".to_string()],
                        dependencies: vec!["D1".to_string()],
                    },
                    SliderQueryBlockedFormulaSummary {
                        cell: "A2".to_string(),
                        formula: "B2".to_string(),
                        reason: "circular_dependency".to_string(),
                        blocked_by: Vec::new(),
                        dependencies: vec!["B2".to_string()],
                    },
                    SliderQueryBlockedFormulaSummary {
                        cell: "B2".to_string(),
                        formula: "A2".to_string(),
                        reason: "circular_dependency".to_string(),
                        blocked_by: Vec::new(),
                        dependencies: vec!["A2".to_string()],
                    },
                ],
            },
            warnings: Vec::new(),
        }
    );

    assert_eq!(
        std::fs::read_to_string(package_path.join("manifest.json")).expect("read manifest"),
        serde_json::to_string_pretty(&result.manifest).expect("serialize manifest")
    );
    assert_eq!(
        std::fs::read_to_string(package_path.join("queries/sheet1_prepared.sql"))
            .expect("read prepared sql"),
        "-- Source: Sheet1, cells B1, C1\n-- Inputs: A\n-- Confidence: high\n\nWITH base_source AS (\n    SELECT *\n    FROM raw.\"sheet1\"\n)\nSELECT\n    *,\n    (col_a * 2) AS col_b,\n    ((col_a * 2) + 5) AS col_c\nFROM base_source;"
    );
    assert_eq!(
        std::fs::read_to_string(package_path.join("queries/sheet1_blocked.json"))
            .expect("read blocked json"),
        serde_json::to_string_pretty(&result.manifest.blocked_formulas)
            .expect("serialize blocked formulas")
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(
            &std::fs::read_to_string(package_path.join("variables/sheet1_prepared.json"))
                .expect("read variables file")
        )
        .expect("parse variables file")["preparedColumns"]
            .as_array()
            .expect("prepared columns array")
            .len(),
        2
    );
}
