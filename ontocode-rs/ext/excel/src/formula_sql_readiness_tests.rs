use std::fs::File;
use std::io::Write;
use std::path::Path;

use pretty_assertions::assert_eq;
use tempfile::tempdir;
use zip::CompressionMethod;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use crate::formula_ast::FormulaAstParseState;
use crate::formula_sql_readiness::FormulaSqlBlockedFormula;
use crate::formula_sql_readiness::FormulaSqlBlockedReasonCount;
use crate::formula_sql_readiness::FormulaSqlReadinessCounts;
use crate::formula_sql_readiness::FormulaSqlReadinessFamily;
use crate::formula_sql_readiness::FormulaSqlReadyFormula;
use crate::formula_sql_readiness::InspectFormulaSqlReadinessResult;
use crate::formula_sql_readiness::inspect_formula_sql_readiness_from_workbook;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;

fn write_zip_fixture(path: &Path, entries: &[(&str, &str)]) {
    let file = File::create(path).expect("create workbook fixture");
    let mut writer = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    for (name, contents) in entries {
        writer.start_file(*name, options).expect("start entry");
        writer
            .write_all(contents.as_bytes())
            .expect("write workbook entry");
    }
    writer.finish().expect("finish workbook fixture");
}

fn write_readiness_fixture(path: &Path) {
    write_zip_fixture(
        path,
        &[
            (
                "xl/workbook.xml",
                r#"<workbook xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
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
                r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/>
                  </Relationships>"#,
            ),
            (
                "xl/worksheets/sheet1.xml",
                r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                    <sheetData>
                      <row r="2">
                        <c r="A2"><v>7</v></c>
                        <c r="B2"><v>6</v></c>
                        <c r="C2"><v>North</v></c>
                        <c r="D2"><f>A2+B2*2</f><v>19</v></c>
                        <c r="E2"><f>VLOOKUP(C2,KPI_Name,2,FALSE)</f><v>North</v></c>
                        <c r="F2"><f>XLOOKUP(C2,Lookup!$A$2:$A$4,Lookup!$B$2:$B$4)</f><v>North</v></c>
                        <c r="G2"><f>SUMIFS(Lookup!$C$2:$C$4,Lookup!$A$2:$A$4,C2)</f><v>10</v></c>
                        <c r="H2"><f>A2#</f><v>7</v></c>
                        <c r="I2"><f>A2+</f><v>7</v></c>
                      </row>
                    </sheetData>
                  </worksheet>"#,
            ),
            (
                "xl/worksheets/sheet2.xml",
                r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                    <sheetData>
                      <row r="2"><c r="A2"><v>North</v></c><c r="B2"><v>Retail</v></c><c r="C2"><v>10</v></c></row>
                      <row r="3"><c r="A3"><v>South</v></c><c r="B3"><v>Wholesale</v></c><c r="C3"><v>20</v></c></row>
                      <row r="4"><c r="A4"><v>West</v></c><c r="B4"><v>Retail</v></c><c r="C4"><v>30</v></c></row>
                    </sheetData>
                  </worksheet>"#,
            ),
        ],
    );
}

#[test]
fn inspect_formula_sql_readiness_summarizes_supported_and_blocked_formulas() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("readiness.xlsx");
    write_readiness_fixture(&path);

    let result = inspect_formula_sql_readiness_from_workbook(
        &path,
        Path::new("readiness.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect formula SQL readiness");

    assert_eq!(
        result,
        InspectFormulaSqlReadinessResult {
            path: "readiness.xlsx".to_string(),
            sheet: SheetPreview {
                name: "Summary".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            max_formulas_applied: 10,
            formula_count: 6,
            readiness_counts: FormulaSqlReadinessCounts {
                scalar_row_local: 1,
                exact_lookup: 2,
                aligned_aggregate: 1,
                blocked: 2,
                malformed: 1,
                unsupported: 1,
            },
            blocked_reason_counts: vec![
                FormulaSqlBlockedReasonCount {
                    reason: "dynamic_array_or_spill_marker".to_string(),
                    count: 1,
                },
                FormulaSqlBlockedReasonCount {
                    reason: "formula_parse_malformed".to_string(),
                    count: 1,
                },
            ],
            ready_formulas: vec![
                FormulaSqlReadyFormula {
                    reference: "D2".to_string(),
                    formula: "A2+B2*2".to_string(),
                    family: FormulaSqlReadinessFamily::ScalarRowLocal,
                    sql_expression: "([col_a] + ([col_b] * 2))".to_string(),
                    parse_state: FormulaAstParseState::Parsed,
                    warnings: Vec::new(),
                },
                FormulaSqlReadyFormula {
                    reference: "E2".to_string(),
                    formula: "VLOOKUP(C2,KPI_Name,2,FALSE)".to_string(),
                    family: FormulaSqlReadinessFamily::ExactLookup,
                    sql_expression:
                        "(SELECT [lookup_col_2] FROM [lookup_kpi_name] WHERE [lookup_col_1] = [col_c])"
                            .to_string(),
                    parse_state: FormulaAstParseState::Parsed,
                    warnings: Vec::new(),
                },
                FormulaSqlReadyFormula {
                    reference: "F2".to_string(),
                    formula: "XLOOKUP(C2,Lookup!$A$2:$A$4,Lookup!$B$2:$B$4)".to_string(),
                    family: FormulaSqlReadinessFamily::ExactLookup,
                    sql_expression:
                        "(SELECT [lookup_return_1] FROM [lookup_pair_lookup_a_2_a_4_b_2_b_4] WHERE [lookup_key_1] = [col_c])"
                            .to_string(),
                    parse_state: FormulaAstParseState::Parsed,
                    warnings: Vec::new(),
                },
                FormulaSqlReadyFormula {
                    reference: "G2".to_string(),
                    formula: "SUMIFS(Lookup!$C$2:$C$4,Lookup!$A$2:$A$4,C2)".to_string(),
                    family: FormulaSqlReadinessFamily::AlignedAggregate,
                    sql_expression:
                        "(SELECT SUM([aggregate_value_1]) FROM [aggregate_source_lookup_c_2_c_4_a_2_a_4] WHERE [criteria_col_1] = [col_c])"
                            .to_string(),
                    parse_state: FormulaAstParseState::Parsed,
                    warnings: Vec::new(),
                },
            ],
            blocked_formulas: vec![
                FormulaSqlBlockedFormula {
                    reference: "H2".to_string(),
                    formula: "A2#".to_string(),
                    family_hint: FormulaSqlReadinessFamily::Unknown,
                    parse_state: FormulaAstParseState::Unsupported,
                    blocker_reasons: vec!["dynamic_array_or_spill_marker".to_string()],
                    warnings: vec!["uses_dynamic_array_or_spill_marker".to_string()],
                },
                FormulaSqlBlockedFormula {
                    reference: "I2".to_string(),
                    formula: "A2+".to_string(),
                    family_hint: FormulaSqlReadinessFamily::Unknown,
                    parse_state: FormulaAstParseState::Malformed,
                    blocker_reasons: vec!["formula_parse_malformed".to_string()],
                    warnings: Vec::new(),
                },
            ],
            truncated: false,
            warnings: Vec::new(),
        }
    );
}
