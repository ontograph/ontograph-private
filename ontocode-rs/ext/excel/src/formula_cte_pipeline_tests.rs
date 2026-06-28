use std::path::Path;

use pretty_assertions::assert_eq;
use tempfile::tempdir;

use crate::formula_cte_pipeline::FormulaCteStage;
use crate::formula_cte_pipeline::inspect_formula_cte_pipeline_from_workbook;
use crate::formula_sql_readiness::FormulaSqlReadinessFamily;
use crate::tests::write_zip_fixture;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;

fn write_pipeline_fixture(path: &Path) {
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

fn write_ambiguous_stage_fixture(path: &Path) {
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
fn inspect_formula_cte_pipeline_stages_ready_formulas_and_blocks_cycles() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("phase2.xlsx");
    write_pipeline_fixture(&workbook_path);

    let result = inspect_formula_cte_pipeline_from_workbook(
        &workbook_path,
        Path::new("phase2.xlsx"),
        &SheetSelector::Name {
            name: "Sheet1".to_string(),
        },
        Some(10),
    )
    .expect("inspect formula cte pipeline");

    assert_eq!(result.path, "phase2.xlsx".to_string());
    assert_eq!(
        result.sheet,
        SheetPreview {
            name: "Sheet1".to_string(),
            sheet_id: Some(1),
            part_path: "xl/worksheets/sheet1.xml".to_string(),
        }
    );
    assert_eq!(result.formula_count, 6);
    assert_eq!(result.ready_formula_count, 2);
    assert_eq!(result.blocked_formula_count, 4);
    assert_eq!(
        result.cycles_detected,
        vec![vec!["A2".to_string(), "B2".to_string()]]
    );
    assert_eq!(
        result.stages,
        vec![
            FormulaCteStage {
                stage_index: 0,
                depends_on_stages: Vec::new(),
                candidate_groups: vec![crate::formula_cte_pipeline::FormulaCteCandidateGroup {
                    group_name: "stage_0_scalar_row_local".to_string(),
                    family: FormulaSqlReadinessFamily::ScalarRowLocal,
                    formula_references: vec!["B1".to_string()],
                    formulas: vec![crate::formula_cte_pipeline::FormulaCteCandidateFormula {
                        reference: "B1".to_string(),
                        formula: "A1*2".to_string(),
                        sql_expression: "([col_a] * 2)".to_string(),
                        dependencies: vec!["A1".to_string()],
                    }],
                }],
                warnings: Vec::new(),
            },
            FormulaCteStage {
                stage_index: 1,
                depends_on_stages: vec![0],
                candidate_groups: vec![crate::formula_cte_pipeline::FormulaCteCandidateGroup {
                    group_name: "stage_1_scalar_row_local".to_string(),
                    family: FormulaSqlReadinessFamily::ScalarRowLocal,
                    formula_references: vec!["C1".to_string()],
                    formulas: vec![crate::formula_cte_pipeline::FormulaCteCandidateFormula {
                        reference: "C1".to_string(),
                        formula: "B1+5".to_string(),
                        sql_expression: "([col_b] + 5)".to_string(),
                        dependencies: vec!["B1".to_string()],
                    }],
                }],
                warnings: Vec::new(),
            },
        ]
    );
    assert_eq!(
        result
            .blocked_formulas
            .iter()
            .map(|formula| formula.reference.clone())
            .collect::<Vec<_>>(),
        vec![
            "D1".to_string(),
            "E1".to_string(),
            "A2".to_string(),
            "B2".to_string(),
        ]
    );
    assert_eq!(
        result
            .blocked_formulas
            .iter()
            .find(|formula| formula.reference == "E1")
            .expect("blocked E1")
            .blocked_by,
        vec!["D1".to_string()]
    );
    assert!(
        !result
            .blocked_formulas
            .iter()
            .find(|formula| formula.reference == "D1")
            .expect("blocked D1")
            .blocker_reasons
            .is_empty()
    );
    assert_eq!(result.warnings, Vec::<String>::new());
}

#[test]
fn inspect_formula_cte_pipeline_warns_when_one_stage_mixes_families() {
    let dir = tempdir().expect("temp dir");
    let workbook_path = dir.path().join("readiness.xlsx");
    write_ambiguous_stage_fixture(&workbook_path);

    let result = inspect_formula_cte_pipeline_from_workbook(
        &workbook_path,
        Path::new("readiness.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect formula cte pipeline");

    assert_eq!(result.ready_formula_count, 4);
    assert_eq!(result.blocked_formula_count, 2);
    assert_eq!(result.stage_count, 1);
    assert_eq!(result.stages[0].depends_on_stages, Vec::<usize>::new());
    assert_eq!(
        result.stages[0]
            .candidate_groups
            .iter()
            .map(|group| group.group_name.clone())
            .collect::<Vec<_>>(),
        vec![
            "stage_0_aligned_aggregate".to_string(),
            "stage_0_exact_lookup".to_string(),
            "stage_0_scalar_row_local".to_string(),
        ]
    );
    assert_eq!(
        result.stages[0].warnings,
        vec![
            "stage_0 mixes multiple readiness families; candidate CTE grouping is heuristic"
                .to_string(),
        ]
    );
    assert!(
        result.warnings.contains(
            &"stage_0 mixes multiple readiness families; candidate CTE grouping is heuristic"
                .to_string()
        )
    );
    assert_eq!(
        result
            .blocked_formulas
            .iter()
            .map(|formula| formula.reference.clone())
            .collect::<Vec<_>>(),
        vec!["H2".to_string(), "I2".to_string()]
    );
}
