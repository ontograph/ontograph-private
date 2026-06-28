use std::fs;
use std::path::Path;

use pretty_assertions::assert_eq;
use tempfile::tempdir;

use crate::named_range_rewrite::NamedRangeRewriteDryRunFormulaResult;
use crate::named_range_rewrite::NamedRangeRewriteDryRunResult;
use crate::named_range_rewrite::NamedRangeRewriteMatch;
use crate::named_range_rewrite::named_range_rewrite_dry_run_with_display_path;
use crate::tests::write_zip_fixture;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;

fn write_row041_fixture(path: &Path, workbook_xml: &str, extra_entries: &[(&str, &str)]) {
    let mut entries = vec![
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
        ("xl/workbook.xml", workbook_xml),
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
              <dimension ref="A1:B3"/>
              <sheetData>
                <row r="1">
                  <c r="A1" t="inlineStr"><is><t>Metric</t></is></c>
                  <c r="B1" t="inlineStr"><is><t>Value</t></is></c>
                </row>
                <row r="2">
                  <c r="A2" t="inlineStr"><is><t>Total</t></is></c>
                  <c r="B2"><f>SUM(Data!$A$1:$A$3)</f><v>60</v></c>
                </row>
                <row r="3">
                  <c r="A3" t="inlineStr"><is><t>Average</t></is></c>
                  <c r="B3"><f>AVERAGE(Data!$A$1:$A$3)</f><v>20</v></c>
                </row>
              </sheetData>
            </worksheet>"#,
        ),
        (
            "xl/worksheets/sheet2.xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
              <dimension ref="A1:A3"/>
              <sheetData>
                <row r="1"><c r="A1"><v>10</v></c></row>
                <row r="2"><c r="A2"><v>20</v></c></row>
                <row r="3"><c r="A3"><v>30</v></c></row>
              </sheetData>
            </worksheet>"#,
        ),
    ];
    entries.extend_from_slice(extra_entries);
    write_zip_fixture(path, &entries);
}

fn write_row041_mapping(path: &Path, workbook_name: &str) {
    fs::write(
        path,
        format!(
            r#"[
              {{
                "workbook_path": "{workbook_name}",
                "sheet_name": "Main",
                "formula_targets": ["B2", "B3"],
                "from_ref": "Data!$A$1:$A$3",
                "to_name": "SalesData",
                "scope_expectation": "workbook",
                "sheet_name_for_scope": null,
                "max_replacements_per_formula": 1,
                "reference_mode": "exact_textual_match_only",
                "all_or_nothing": true,
                "notes": []
              }}
            ]"#
        ),
    )
    .expect("write mapping");
}

#[test]
fn named_range_rewrite_dry_run_reports_synthetic_positive_match() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("positive.xlsx");
    write_row041_fixture(
        &workbook_path,
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                  xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
          <sheets>
            <sheet name="Main" sheetId="1" r:id="rId1"/>
            <sheet name="Data" sheetId="2" r:id="rId2"/>
          </sheets>
          <definedNames>
            <definedName name="SalesData">Data!$A$1:$A$3</definedName>
          </definedNames>
          <calcPr calcId="191029"/>
        </workbook>"#,
        &[],
    );
    let mapping_path = dir.path().join("mapping.json");
    write_row041_mapping(&mapping_path, "positive.xlsx");

    let result = named_range_rewrite_dry_run_with_display_path(
        &workbook_path,
        Path::new("positive.xlsx"),
        &SheetSelector::Name {
            name: "Main".to_string(),
        },
        &mapping_path,
        Path::new("mapping.json"),
    )
    .expect("rewrite dry-run");

    assert_eq!(
        result,
        NamedRangeRewriteDryRunResult {
            path: "positive.xlsx".to_string(),
            mapping_path: "mapping.json".to_string(),
            sheet: SheetPreview {
                name: "Main".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            results: vec![
                NamedRangeRewriteDryRunFormulaResult {
                    formula_reference: "B2".to_string(),
                    original_formula: "SUM(Data!$A$1:$A$3)".to_string(),
                    proposed_rewritten_formula: Some("SUM(SalesData)".to_string()),
                    matched_mapping_entries: vec![NamedRangeRewriteMatch {
                        from_ref: "Data!$A$1:$A$3".to_string(),
                        to_name: "SalesData".to_string(),
                    }],
                    blocker_reasons: Vec::new(),
                    confidence: "high".to_string(),
                },
                NamedRangeRewriteDryRunFormulaResult {
                    formula_reference: "B3".to_string(),
                    original_formula: "AVERAGE(Data!$A$1:$A$3)".to_string(),
                    proposed_rewritten_formula: Some("AVERAGE(SalesData)".to_string()),
                    matched_mapping_entries: vec![NamedRangeRewriteMatch {
                        from_ref: "Data!$A$1:$A$3".to_string(),
                        to_name: "SalesData".to_string(),
                    }],
                    blocker_reasons: Vec::new(),
                    confidence: "high".to_string(),
                },
            ],
            warnings: Vec::new(),
        }
    );
}

#[test]
fn named_range_rewrite_dry_run_blocks_external_links() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("external.xlsx");
    write_row041_fixture(
        &workbook_path,
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                  xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
          <sheets>
            <sheet name="Main" sheetId="1" r:id="rId1"/>
            <sheet name="Data" sheetId="2" r:id="rId2"/>
          </sheets>
          <externalReferences><externalReference r:id="rId3"/></externalReferences>
          <definedNames>
            <definedName name="SalesData">Data!$A$1:$A$3</definedName>
          </definedNames>
          <calcPr calcId="191029"/>
        </workbook>"#,
        &[("xl/externalLinks/externalLink1.xml", "<externalLink/>")],
    );
    let mapping_path = dir.path().join("mapping.json");
    write_row041_mapping(&mapping_path, "external.xlsx");

    let result = named_range_rewrite_dry_run_with_display_path(
        &workbook_path,
        Path::new("external.xlsx"),
        &SheetSelector::Name {
            name: "Main".to_string(),
        },
        &mapping_path,
        Path::new("mapping.json"),
    )
    .expect("rewrite dry-run");

    assert_eq!(
        result.results[0].blocker_reasons,
        vec!["external-link-blocked".to_string()]
    );
    assert_eq!(result.results[0].proposed_rewritten_formula, None);
}

#[test]
fn named_range_rewrite_dry_run_blocks_ambiguous_sheet_scope() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("ambiguous.xlsx");
    write_row041_fixture(
        &workbook_path,
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                  xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
          <sheets>
            <sheet name="Main" sheetId="1" r:id="rId1"/>
            <sheet name="Data" sheetId="2" r:id="rId2"/>
          </sheets>
          <definedNames>
            <definedName name="SalesData">Data!$A$1:$A$3</definedName>
            <definedName name="SalesData" localSheetId="0">Main!$C$1:$C$3</definedName>
          </definedNames>
          <calcPr calcId="191029"/>
        </workbook>"#,
        &[],
    );
    let mapping_path = dir.path().join("mapping.json");
    write_row041_mapping(&mapping_path, "ambiguous.xlsx");

    let result = named_range_rewrite_dry_run_with_display_path(
        &workbook_path,
        Path::new("ambiguous.xlsx"),
        &SheetSelector::Name {
            name: "Main".to_string(),
        },
        &mapping_path,
        Path::new("mapping.json"),
    )
    .expect("rewrite dry-run");

    assert_eq!(
        result.results[0].blocker_reasons,
        vec!["ambiguous-sheet-scope".to_string()]
    );
    assert_eq!(result.results[0].proposed_rewritten_formula, None);
}

#[test]
fn named_range_rewrite_dry_run_blocks_r1c1_workbooks() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("r1c1.xlsx");
    write_row041_fixture(
        &workbook_path,
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                  xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
          <sheets>
            <sheet name="Main" sheetId="1" r:id="rId1"/>
            <sheet name="Data" sheetId="2" r:id="rId2"/>
          </sheets>
          <definedNames>
            <definedName name="SalesData">Data!$A$1:$A$3</definedName>
          </definedNames>
          <calcPr calcId="191029" refMode="R1C1"/>
        </workbook>"#,
        &[],
    );
    let mapping_path = dir.path().join("mapping.json");
    write_row041_mapping(&mapping_path, "r1c1.xlsx");

    let result = named_range_rewrite_dry_run_with_display_path(
        &workbook_path,
        Path::new("r1c1.xlsx"),
        &SheetSelector::Name {
            name: "Main".to_string(),
        },
        &mapping_path,
        Path::new("mapping.json"),
    )
    .expect("rewrite dry-run");

    assert_eq!(
        result.results[0].blocker_reasons,
        vec!["unsupported-reference-mode".to_string()]
    );
    assert_eq!(result.results[0].proposed_rewritten_formula, None);
}
