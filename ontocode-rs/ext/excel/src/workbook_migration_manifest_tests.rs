use std::path::Path;

use pretty_assertions::assert_eq;
use tempfile::tempdir;

use crate::tests::write_workbook_migration_fixture;
use crate::tests::write_zip_fixture;
use crate::workbook_migration_manifest::WorkbookMigrationFormulaSqlLineageEntry;
use crate::workbook_migration_manifest::WorkbookMigrationFormulaSqlLineageState;
use crate::workbook_migration_manifest::generate_workbook_migration_manifest_with_display_path;

#[test]
fn generate_workbook_migration_manifest_rejects_out_of_range_formula_limit() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("review.xlsm");
    let bundle_path = dir.path().join("bundles/review");
    write_workbook_migration_fixture(&workbook_path);

    let err = generate_workbook_migration_manifest_with_display_path(
        &workbook_path,
        Path::new("review.xlsm"),
        &bundle_path,
        Path::new("bundles/review"),
        Some(0),
    )
    .expect_err("formula limit 0 should fail closed");

    assert_eq!(
        err.to_string(),
        "excel.generate_workbook_migration_manifest max_formulas_per_sheet must be between 1 and 512"
            .to_string()
    );
}

#[test]
fn generate_workbook_migration_manifest_emits_blocked_formula_lineage() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("blocked.xlsx");
    let bundle_path = dir.path().join("bundles/blocked");
    write_zip_fixture(
        &workbook_path,
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
                      <c r="B2"><f>A2#</f><v>7</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );

    let result = generate_workbook_migration_manifest_with_display_path(
        &workbook_path,
        Path::new("blocked.xlsx"),
        &bundle_path,
        Path::new("bundles/blocked"),
        None,
    )
    .expect("generate workbook migration manifest");

    assert_eq!(
        result.manifest.formula_sql_lineage,
        vec![WorkbookMigrationFormulaSqlLineageEntry {
            source_id: "Summary!B2".to_string(),
            sheet: "Summary".to_string(),
            reference: "B2".to_string(),
            family: crate::formula_sql_readiness::FormulaSqlReadinessFamily::Unknown,
            readiness_state: WorkbookMigrationFormulaSqlLineageState::Blocked,
            sql_expression: None,
            blocker_reasons: vec!["dynamic_array_or_spill_marker".to_string()],
            warnings: vec!["uses_dynamic_array_or_spill_marker".to_string()],
        }]
    );
}

#[test]
fn generate_workbook_migration_manifest_degrades_pivot_parse_failures() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("broken-pivot.xlsx");
    let bundle_path = dir.path().join("bundles/broken-pivot");
    write_zip_fixture(
        &workbook_path,
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
                  <pivotCaches>
                    <pivotCache cacheId="1" r:id="rId2"/>
                  </pivotCaches>
                </workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" Target="pivotCache/pivotCacheDefinition1.xml"/>
                </Relationships>"#,
            ),
            (
                "xl/worksheets/sheet1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="2">
                      <c r="A2"><v>7</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
            (
                "xl/pivotCache/pivotCacheDefinition1.xml",
                "<pivotCacheDefinition",
            ),
        ],
    );

    let result = generate_workbook_migration_manifest_with_display_path(
        &workbook_path,
        Path::new("broken-pivot.xlsx"),
        &bundle_path,
        Path::new("bundles/broken-pivot"),
        None,
    )
    .expect("generate workbook migration manifest");

    assert_eq!(result.manifest.pivot.pivot_table_count, 0);
    assert_eq!(result.manifest.pivot.pivot_cache_count, 0);
    assert!(
        result
            .manifest
            .unsupported_sections
            .contains(&"pivot_metadata_unavailable".to_string())
    );
    assert!(!result.manifest.pivot.warnings.is_empty());
}
