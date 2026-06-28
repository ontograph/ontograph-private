use std::sync::Arc;

use ontocode_core::config::Config;
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

use crate::extension::install;
use crate::tests::build_data_mashup_xml_utf16;
use crate::tests::write_formula_inventory_fixture;
use crate::tests::write_zip_fixture;
use crate::tests::write_zip_fixture_bytes;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;
use crate::workbook_graph::INSPECT_WORKBOOK_GRAPH_TOOL_NAME;
use crate::workbook_graph::InspectWorkbookGraphResult;
use crate::workbook_graph::WorkbookGraphEdge;
use crate::workbook_graph::WorkbookGraphEdgeKind;
use crate::workbook_graph::WorkbookGraphEvidence;
use crate::workbook_graph::WorkbookGraphEvidenceKind;
use crate::workbook_graph::WorkbookGraphFormulaInventoryScope;
use crate::workbook_graph::WorkbookGraphMode;
use crate::workbook_graph::WorkbookGraphNode;
use crate::workbook_graph::WorkbookGraphNodeKind;
use crate::workbook_graph::inspect_workbook_graph_with_display_path;

fn write_cross_sheet_graph_fixture(path: &std::path::Path) {
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
                  <sheetData>
                    <row r="1">
                      <c r="A1"><f>SUM(Data!$A$1:$A$3)+Data!$B$2</f><v>7</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
            (
                "xl/worksheets/sheet2.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="1">
                      <c r="A1"><v>1</v></c>
                    </row>
                    <row r="2">
                      <c r="B2"><v>2</v></c>
                    </row>
                    <row r="3">
                      <c r="A3"><v>3</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

fn write_defined_name_graph_fixture(path: &std::path::Path) {
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

fn write_local_defined_name_target_graph_fixture(path: &std::path::Path) {
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
                    <definedName name="Threshold" localSheetId="0">$B$1</definedName>
                    <definedName name="Totals" localSheetId="0">$B$1:$B$2</definedName>
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
                      <c r="A1"><f>Threshold+1</f><v>8</v></c>
                      <c r="B1"><v>7</v></c>
                    </row>
                    <row r="2">
                      <c r="A2"><f>SUM(Totals)</f><v>11</v></c>
                      <c r="B2"><v>4</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

fn write_ambiguous_defined_name_graph_fixture(path: &std::path::Path) {
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
                    <definedName name="Threshold">$B$1</definedName>
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
                      <c r="A1"><f>Threshold+1</f><v>8</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

fn write_powerquery_graph_fixture(path: &std::path::Path) {
    write_zip_fixture_bytes(
        path,
        &[
            (
                "[Content_Types].xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                  <Default Extension="xml" ContentType="application/xml"/>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
                  <Override PartName="/xl/tables/table1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"/>
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
                  </sheets>
                  <definedNames>
                    <definedName name="Threshold">Summary!$D$1</definedName>
                  </definedNames>
                </workbook>"#
                    .to_vec(),
            ),
            (
                "xl/_rels/workbook.xml.rels",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#
                    .to_vec(),
            ),
            (
                "xl/worksheets/sheet1.xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="1">
                      <c r="A1" t="inlineStr"><is><t>Category</t></is></c>
                      <c r="B1" t="inlineStr"><is><t>Amount</t></is></c>
                      <c r="D1"><v>7</v></c>
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
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rIdTable1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/table" Target="../tables/table1.xml"/>
                </Relationships>"#
                    .to_vec(),
            ),
            (
                "xl/tables/table1.xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <table xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" id="1" name="Sales" displayName="Sales" ref="$A$1:$B$2">
                  <autoFilter ref="$A$1:$B$2"/>
                  <tableColumns count="2">
                    <tableColumn id="1" name="Category"/>
                    <tableColumn id="2" name="Amount"/>
                  </tableColumns>
                </table>"#
                    .to_vec(),
            ),
            (
                "customXml/item1.xml",
                build_data_mashup_xml_utf16(&[(
                    "Formulas/Section1.m",
                    concat!(
                        "section Section1;\n",
                        "shared ThresholdQuery = let\n",
                        "    Source = Excel.CurrentWorkbook(){[Name=\"Threshold\"]}[Content]\n",
                        "in\n",
                        "    Source;\n",
                        "shared TableQuery = let\n",
                        "    Source = Excel.CurrentWorkbook(){[Name=\"Sales\"]}[Content]\n",
                        "in\n",
                        "    Source;\n",
                        "shared FinalQuery = let\n",
                        "    Source = TableQuery\n",
                        "in\n",
                        "    Source;\n",
                    ),
                )]),
            ),
        ],
    );
}

fn write_unresolved_powerquery_graph_fixture(path: &std::path::Path) {
    write_zip_fixture_bytes(
        path,
        &[
            (
                "[Content_Types].xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
                  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
                  <Default Extension="xml" ContentType="application/xml"/>
                  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
                  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
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
                  </sheets>
                </workbook>"#
                    .to_vec(),
            ),
            (
                "xl/_rels/workbook.xml.rels",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
                </Relationships>"#
                    .to_vec(),
            ),
            (
                "xl/worksheets/sheet1.xml",
                br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
                  <sheetData>
                    <row r="1">
                      <c r="A1"><v>1</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#
                    .to_vec(),
            ),
            (
                "customXml/item1.xml",
                build_data_mashup_xml_utf16(&[(
                    "Formulas/Section1.m",
                    concat!(
                        "section Section1;\n",
                        "shared MissingWorkbookName = let\n",
                        "    Source = Excel.CurrentWorkbook(){[Name=\"MissingRange\"]}[Content]\n",
                        "in\n",
                        "    Source;\n",
                    ),
                )]),
            ),
        ],
    );
}

fn write_unresolved_defined_name_graph_fixture(path: &std::path::Path) {
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
                    <row r="1">
                      <c r="A1"><f>MissingName+1</f><v>8</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

fn write_table_graph_fixture(path: &std::path::Path) {
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
                  <Override PartName="/xl/tables/table1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"/>
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
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                           xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
                  <sheetData>
                    <row r="1">
                      <c r="A1" t="inlineStr"><is><t>Region</t></is></c>
                      <c r="B1" t="inlineStr"><is><t>Amount</t></is></c>
                      <c r="D1"><f>SUM(Sales[Amount])</f><v>11</v></c>
                    </row>
                    <row r="2">
                      <c r="A2" t="inlineStr"><is><t>North</t></is></c>
                      <c r="B2"><v>7</v></c>
                    </row>
                    <row r="3">
                      <c r="A3" t="inlineStr"><is><t>South</t></is></c>
                      <c r="B3"><v>4</v></c>
                    </row>
                  </sheetData>
                  <tableParts count="1">
                    <tablePart r:id="rIdTable1"/>
                  </tableParts>
                </worksheet>"#,
            ),
            (
                "xl/worksheets/_rels/sheet1.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rIdTable1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/table" Target="../tables/table1.xml"/>
                </Relationships>"#,
            ),
            (
                "xl/tables/table1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <table xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                       id="1"
                       name="Sales"
                       displayName="Sales"
                       ref="$A$1:$B$3">
                  <autoFilter ref="$A$1:$B$3"/>
                  <tableColumns count="2">
                    <tableColumn id="1" name="Region"/>
                    <tableColumn id="2" name="Amount"/>
                  </tableColumns>
                </table>"#,
            ),
        ],
    );
}

fn write_missing_table_reference_graph_fixture(path: &std::path::Path) {
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
                    <row r="1">
                      <c r="A1"><f>SUM(Missing[Amount])</f><v>0</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
        ],
    );
}

fn write_unresolved_table_metadata_graph_fixture(path: &std::path::Path) {
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
                  <Override PartName="/xl/tables/table1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"/>
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
                      <c r="A1"><v>1</v></c>
                    </row>
                  </sheetData>
                </worksheet>"#,
            ),
            (
                "xl/tables/table1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <table xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                       id="1"
                       name="Sales"
                       displayName="Sales"
                       ref="$A$1:$B$3">
                  <autoFilter ref="$A$1:$B$3"/>
                </table>"#,
            ),
        ],
    );
}

#[test]
fn inspect_workbook_graph_reports_phase1_structure_and_formula_membership() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("formulas.xlsx");
    write_formula_inventory_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("formulas.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result,
        InspectWorkbookGraphResult {
            path: "formulas.xlsx".to_string(),
            mode: WorkbookGraphMode::PackageStructurePlusPerSheetFormulaMembership,
            is_partial: true,
            formula_inventory_scope: WorkbookGraphFormulaInventoryScope::SelectedSheetOnly,
            formula_inventory_sheet: SheetPreview {
                name: "Summary".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            max_formulas_applied: 10,
            formula_inventory_truncated: false,
            nodes: vec![
                WorkbookGraphNode {
                    id: "workbook".to_string(),
                    kind: WorkbookGraphNodeKind::Workbook,
                    label: "formulas.xlsx".to_string(),
                    sheet_name: None,
                    part_path: None,
                    cell_reference: None,
                    formula: None,
                    cached_value: None,
                },
                WorkbookGraphNode {
                    id: "worksheet:0".to_string(),
                    kind: WorkbookGraphNodeKind::Worksheet,
                    label: "Summary".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: None,
                    formula: None,
                    cached_value: None,
                },
                WorkbookGraphNode {
                    id: "formula:0:A1".to_string(),
                    kind: WorkbookGraphNodeKind::CellFormula,
                    label: "Summary!A1".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: Some("A1".to_string()),
                    formula: Some("SUM(B1:B2)".to_string()),
                    cached_value: Some("7".to_string()),
                },
                WorkbookGraphNode {
                    id: "ref-range:0:B1:B2".to_string(),
                    kind: WorkbookGraphNodeKind::ReferencedRange,
                    label: "Summary!B1:B2".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: Some("B1:B2".to_string()),
                    formula: None,
                    cached_value: None,
                },
                WorkbookGraphNode {
                    id: "formula:0:B1".to_string(),
                    kind: WorkbookGraphNodeKind::CellFormula,
                    label: "Summary!B1".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: Some("B1".to_string()),
                    formula: Some("A1&\"x\"".to_string()),
                    cached_value: Some("North".to_string()),
                },
                WorkbookGraphNode {
                    id: "ref-cell:0:A1".to_string(),
                    kind: WorkbookGraphNodeKind::ReferencedCell,
                    label: "Summary!A1".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: Some("A1".to_string()),
                    formula: None,
                    cached_value: None,
                },
                WorkbookGraphNode {
                    id: "formula:0:C1".to_string(),
                    kind: WorkbookGraphNodeKind::CellFormula,
                    label: "Summary!C1".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: Some("C1".to_string()),
                    formula: Some("NOW()".to_string()),
                    cached_value: Some("45900".to_string()),
                },
                WorkbookGraphNode {
                    id: "formula:0:B2".to_string(),
                    kind: WorkbookGraphNodeKind::CellFormula,
                    label: "Summary!B2".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: Some("B2".to_string()),
                    formula: Some(String::new()),
                    cached_value: Some("9".to_string()),
                },
            ],
            edges: vec![
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::WorkbookContainsWorksheet,
                    from: "workbook".to_string(),
                    to: "worksheet:0".to_string(),
                    evidence: vec![
                        WorkbookGraphEvidence {
                            kind: WorkbookGraphEvidenceKind::WorkbookSheetEntry,
                            part_path: "xl/workbook.xml".to_string(),
                            cell_reference: None,
                            detail: "sheet entry name=Summary sheet_id=1 relationship_id=rId1"
                                .to_string(),
                        },
                        WorkbookGraphEvidence {
                            kind: WorkbookGraphEvidenceKind::WorkbookRelationship,
                            part_path: "xl/_rels/workbook.xml.rels".to_string(),
                            cell_reference: None,
                            detail: "worksheet part target xl/worksheets/sheet1.xml".to_string(),
                        },
                    ],
                },
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::WorksheetContainsFormula,
                    from: "worksheet:0".to_string(),
                    to: "formula:0:A1".to_string(),
                    evidence: vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::WorksheetFormulaCell,
                        part_path: "xl/worksheets/sheet1.xml".to_string(),
                        cell_reference: Some("A1".to_string()),
                        detail: "selected-sheet formula inventory entry".to_string(),
                    }],
                },
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::FormulaReferencesRange,
                    from: "formula:0:A1".to_string(),
                    to: "ref-range:0:B1:B2".to_string(),
                    evidence: vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::FormulaAstRangeReference,
                        part_path: "xl/worksheets/sheet1.xml".to_string(),
                        cell_reference: Some("A1".to_string()),
                        detail: "ast range reference Summary!B1:B2".to_string(),
                    }],
                },
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::WorksheetContainsFormula,
                    from: "worksheet:0".to_string(),
                    to: "formula:0:B1".to_string(),
                    evidence: vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::WorksheetFormulaCell,
                        part_path: "xl/worksheets/sheet1.xml".to_string(),
                        cell_reference: Some("B1".to_string()),
                        detail: "selected-sheet formula inventory entry".to_string(),
                    }],
                },
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::FormulaReferencesCell,
                    from: "formula:0:B1".to_string(),
                    to: "ref-cell:0:A1".to_string(),
                    evidence: vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::FormulaAstCellReference,
                        part_path: "xl/worksheets/sheet1.xml".to_string(),
                        cell_reference: Some("B1".to_string()),
                        detail: "ast cell reference Summary!A1".to_string(),
                    }],
                },
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::WorksheetContainsFormula,
                    from: "worksheet:0".to_string(),
                    to: "formula:0:C1".to_string(),
                    evidence: vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::WorksheetFormulaCell,
                        part_path: "xl/worksheets/sheet1.xml".to_string(),
                        cell_reference: Some("C1".to_string()),
                        detail: "selected-sheet formula inventory entry".to_string(),
                    }],
                },
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::WorksheetContainsFormula,
                    from: "worksheet:0".to_string(),
                    to: "formula:0:B2".to_string(),
                    evidence: vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::WorksheetFormulaCell,
                        part_path: "xl/worksheets/sheet1.xml".to_string(),
                        cell_reference: Some("B2".to_string()),
                        detail: "selected-sheet formula inventory entry".to_string(),
                    }],
                },
            ],
            warnings: vec![
                "selected sheet contains formulas without parsed ASTs; graph omits dependency edges for missing formulas".to_string(),
                "selected sheet contains unsupported formulas; graph omits dependency edges for unsupported constructs".to_string(),
                "workbook graph is partial: it emits workbook structure, selected-sheet formula membership, and AST-backed dependency edges only where proven".to_string(),
                "workbook has external links; phase 1 graph omits external-link lineage".to_string(),
            ],
        }
    );
}

#[test]
fn inspect_workbook_graph_emits_cross_sheet_formula_dependency_edges() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("cross_sheet.xlsx");
    write_cross_sheet_graph_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("cross_sheet.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result
            .nodes
            .iter()
            .filter(|node| {
                matches!(
                    node.kind,
                    WorkbookGraphNodeKind::ReferencedCell | WorkbookGraphNodeKind::ReferencedRange
                )
            })
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            WorkbookGraphNode {
                id: "ref-range:1:$A$1:$A$3".to_string(),
                kind: WorkbookGraphNodeKind::ReferencedRange,
                label: "Data!$A$1:$A$3".to_string(),
                sheet_name: Some("Data".to_string()),
                part_path: Some("xl/worksheets/sheet2.xml".to_string()),
                cell_reference: Some("$A$1:$A$3".to_string()),
                formula: None,
                cached_value: None,
            },
            WorkbookGraphNode {
                id: "ref-cell:1:$B$2".to_string(),
                kind: WorkbookGraphNodeKind::ReferencedCell,
                label: "Data!$B$2".to_string(),
                sheet_name: Some("Data".to_string()),
                part_path: Some("xl/worksheets/sheet2.xml".to_string()),
                cell_reference: Some("$B$2".to_string()),
                formula: None,
                cached_value: None,
            },
        ]
    );
    assert_eq!(
        result
            .edges
            .iter()
            .filter(|edge| edge.from == "formula:0:A1")
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::FormulaReferencesWorksheet,
                from: "formula:0:A1".to_string(),
                to: "worksheet:1".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::FormulaAstWorksheetReference,
                    part_path: "xl/worksheets/sheet2.xml".to_string(),
                    cell_reference: Some("A1".to_string()),
                    detail: "ast reference resolved to worksheet Data".to_string(),
                }],
            },
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::FormulaReferencesRange,
                from: "formula:0:A1".to_string(),
                to: "ref-range:1:$A$1:$A$3".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::FormulaAstRangeReference,
                    part_path: "xl/worksheets/sheet2.xml".to_string(),
                    cell_reference: Some("A1".to_string()),
                    detail: "ast range reference Data!$A$1:$A$3".to_string(),
                }],
            },
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::FormulaReferencesCell,
                from: "formula:0:A1".to_string(),
                to: "ref-cell:1:$B$2".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::FormulaAstCellReference,
                    part_path: "xl/worksheets/sheet2.xml".to_string(),
                    cell_reference: Some("A1".to_string()),
                    detail: "ast cell reference Data!$B$2".to_string(),
                }],
            },
        ]
    );
}

#[test]
fn inspect_workbook_graph_emits_defined_name_formula_text_edges() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("defined_name_graph.xlsx");
    write_defined_name_graph_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("defined_name_graph.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result
            .nodes
            .iter()
            .filter(|node| {
                matches!(
                    node.kind,
                    WorkbookGraphNodeKind::DefinedName
                        | WorkbookGraphNodeKind::DefinedNameFormulaText
                )
            })
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            WorkbookGraphNode {
                id: "defined-name:workbook:threshold".to_string(),
                kind: WorkbookGraphNodeKind::DefinedName,
                label: "Threshold".to_string(),
                sheet_name: None,
                part_path: Some("xl/workbook.xml".to_string()),
                cell_reference: None,
                formula: None,
                cached_value: None,
            },
            WorkbookGraphNode {
                id: "defined-name-formula:defined-name:workbook:threshold".to_string(),
                kind: WorkbookGraphNodeKind::DefinedNameFormulaText,
                label: "Threshold=42".to_string(),
                sheet_name: None,
                part_path: Some("xl/workbook.xml".to_string()),
                cell_reference: None,
                formula: Some("42".to_string()),
                cached_value: None,
            },
        ]
    );
    assert_eq!(
        result
            .edges
            .iter()
            .filter(|edge| {
                matches!(
                    edge.kind,
                    WorkbookGraphEdgeKind::FormulaReferencesDefinedName
                        | WorkbookGraphEdgeKind::DefinedNameTargetsFormulaText
                )
            })
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::FormulaReferencesDefinedName,
                from: "formula:0:A1".to_string(),
                to: "defined-name:workbook:threshold".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::FormulaAstDefinedNameReference,
                    part_path: "xl/worksheets/sheet1.xml".to_string(),
                    cell_reference: Some("A1".to_string()),
                    detail: "ast defined-name reference Threshold".to_string(),
                }],
            },
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::DefinedNameTargetsFormulaText,
                from: "defined-name:workbook:threshold".to_string(),
                to: "defined-name-formula:defined-name:workbook:threshold".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::WorkbookDefinedNameTarget,
                    part_path: "xl/workbook.xml".to_string(),
                    cell_reference: None,
                    detail: "defined name Threshold keeps opaque formula text 42".to_string(),
                }],
            },
        ]
    );
    assert_eq!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("defined-name references; graph omits")),
        false
    );
}

#[test]
fn inspect_workbook_graph_emits_local_defined_name_target_edges() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("local_defined_name_targets.xlsx");
    write_local_defined_name_target_graph_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("local_defined_name_targets.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result
            .edges
            .iter()
            .filter(|edge| {
                matches!(
                    edge.kind,
                    WorkbookGraphEdgeKind::FormulaReferencesDefinedName
                        | WorkbookGraphEdgeKind::DefinedNameTargetsCell
                        | WorkbookGraphEdgeKind::DefinedNameTargetsRange
                )
            })
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::FormulaReferencesDefinedName,
                from: "formula:0:A1".to_string(),
                to: "defined-name:summary:threshold".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::FormulaAstDefinedNameReference,
                    part_path: "xl/worksheets/sheet1.xml".to_string(),
                    cell_reference: Some("A1".to_string()),
                    detail: "ast defined-name reference Summary!Threshold".to_string(),
                }],
            },
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::DefinedNameTargetsCell,
                from: "defined-name:summary:threshold".to_string(),
                to: "ref-cell:0:$B$1".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::WorkbookDefinedNameTarget,
                    part_path: "xl/workbook.xml".to_string(),
                    cell_reference: None,
                    detail: "defined name Summary!Threshold targets cell Summary!$B$1".to_string(),
                }],
            },
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::FormulaReferencesDefinedName,
                from: "formula:0:A2".to_string(),
                to: "defined-name:summary:totals".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::FormulaAstDefinedNameReference,
                    part_path: "xl/worksheets/sheet1.xml".to_string(),
                    cell_reference: Some("A2".to_string()),
                    detail: "ast defined-name reference Summary!Totals".to_string(),
                }],
            },
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::DefinedNameTargetsRange,
                from: "defined-name:summary:totals".to_string(),
                to: "ref-range:0:$B$1:$B$2".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::WorkbookDefinedNameTarget,
                    part_path: "xl/workbook.xml".to_string(),
                    cell_reference: None,
                    detail: "defined name Summary!Totals targets range Summary!$B$1:$B$2"
                        .to_string(),
                }],
            },
        ]
    );
    assert_eq!(
        result.warnings.contains(
            &"workbook contains defined names with unqualified workbook-scope cell or range targets; graph keeps those targets as opaque formula text"
                .to_string()
        ),
        false
    );
}

#[test]
fn inspect_workbook_graph_warns_for_ambiguous_workbook_scope_defined_name_targets() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("ambiguous_defined_name_targets.xlsx");
    write_ambiguous_defined_name_graph_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("ambiguous_defined_name_targets.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result.warnings.contains(
            &"workbook contains defined names with unqualified workbook-scope cell or range targets; graph keeps those targets as opaque formula text"
                .to_string()
        ),
        true
    );
    assert_eq!(
        result
            .edges
            .iter()
            .filter(|edge| {
                matches!(
                    edge.kind,
                    WorkbookGraphEdgeKind::DefinedNameTargetsCell
                        | WorkbookGraphEdgeKind::DefinedNameTargetsRange
                        | WorkbookGraphEdgeKind::DefinedNameTargetsFormulaText
                )
            })
            .cloned()
            .collect::<Vec<_>>(),
        vec![WorkbookGraphEdge {
            kind: WorkbookGraphEdgeKind::DefinedNameTargetsFormulaText,
            from: "defined-name:workbook:threshold".to_string(),
            to: "defined-name-formula:defined-name:workbook:threshold".to_string(),
            evidence: vec![WorkbookGraphEvidence {
                kind: WorkbookGraphEvidenceKind::WorkbookDefinedNameTarget,
                part_path: "xl/workbook.xml".to_string(),
                cell_reference: None,
                detail: "defined name Threshold keeps opaque formula text $B$1".to_string(),
            }],
        }]
    );
}

#[test]
fn inspect_workbook_graph_warns_for_unresolved_defined_name_references() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("unresolved_defined_name.xlsx");
    write_unresolved_defined_name_graph_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("unresolved_defined_name.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result.warnings.contains(
            &"selected sheet contains unresolved defined-name references; graph omits dependency edges for unresolved names"
                .to_string()
        ),
        true
    );
    assert_eq!(
        result
            .edges
            .iter()
            .filter(|edge| edge.kind == WorkbookGraphEdgeKind::FormulaReferencesDefinedName)
            .count(),
        0
    );
}

#[test]
fn inspect_workbook_graph_emits_table_edges_and_structured_reference_links() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("table_graph.xlsx");
    write_table_graph_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("table_graph.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result
            .nodes
            .iter()
            .filter(|node| matches!(node.kind, WorkbookGraphNodeKind::Table))
            .cloned()
            .collect::<Vec<_>>(),
        vec![WorkbookGraphNode {
            id: "table:0:sales".to_string(),
            kind: WorkbookGraphNodeKind::Table,
            label: "Summary!Sales".to_string(),
            sheet_name: Some("Summary".to_string()),
            part_path: Some("xl/tables/table1.xml".to_string()),
            cell_reference: Some("$A$1:$B$3".to_string()),
            formula: None,
            cached_value: None,
        }]
    );
    assert_eq!(
        result
            .edges
            .iter()
            .filter(|edge| {
                matches!(
                    edge.kind,
                    WorkbookGraphEdgeKind::FormulaReferencesTable
                        | WorkbookGraphEdgeKind::TableHasRange
                )
            })
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::TableHasRange,
                from: "table:0:sales".to_string(),
                to: "ref-range:0:$A$1:$B$3".to_string(),
                evidence: vec![
                    WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::WorksheetTableRelationship,
                        part_path: "xl/worksheets/_rels/sheet1.xml.rels".to_string(),
                        cell_reference: None,
                        detail:
                            "worksheet Summary table relationship rIdTable1 -> xl/tables/table1.xml"
                                .to_string(),
                    },
                    WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::TableXmlRange,
                        part_path: "xl/tables/table1.xml".to_string(),
                        cell_reference: None,
                        detail: "table Sales range $A$1:$B$3".to_string(),
                    },
                ],
            },
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::FormulaReferencesTable,
                from: "formula:0:D1".to_string(),
                to: "table:0:sales".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::FormulaAstStructuredReference,
                    part_path: "xl/worksheets/sheet1.xml".to_string(),
                    cell_reference: Some("D1".to_string()),
                    detail: "structured reference Sales[Amount] resolved to table Summary!Sales"
                        .to_string(),
                }],
            },
        ]
    );
    assert_eq!(
        result.warnings.contains(
            &"selected sheet contains unresolved structured table references; graph omits dependency edges for unresolved table targets"
                .to_string()
        ),
        false
    );
}

#[test]
fn inspect_workbook_graph_emits_powerquery_lineage_edges_from_lexical_proof() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("powerquery_graph.xlsm");
    write_powerquery_graph_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("powerquery_graph.xlsm"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result
            .nodes
            .iter()
            .filter(|node| node.kind == WorkbookGraphNodeKind::PowerQueryQuery)
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            WorkbookGraphNode {
                id: "power-query:thresholdquery".to_string(),
                kind: WorkbookGraphNodeKind::PowerQueryQuery,
                label: "ThresholdQuery".to_string(),
                sheet_name: None,
                part_path: Some("Formulas/Section1.m".to_string()),
                cell_reference: None,
                formula: Some(
                    concat!(
                        "shared ThresholdQuery = let\n",
                        "    Source = Excel.CurrentWorkbook(){[Name=\"Threshold\"]}[Content]\n",
                        "in\n",
                        "    Source;"
                    )
                    .to_string(),
                ),
                cached_value: None,
            },
            WorkbookGraphNode {
                id: "power-query:tablequery".to_string(),
                kind: WorkbookGraphNodeKind::PowerQueryQuery,
                label: "TableQuery".to_string(),
                sheet_name: None,
                part_path: Some("Formulas/Section1.m".to_string()),
                cell_reference: None,
                formula: Some(
                    concat!(
                        "shared TableQuery = let\n",
                        "    Source = Excel.CurrentWorkbook(){[Name=\"Sales\"]}[Content]\n",
                        "in\n",
                        "    Source;"
                    )
                    .to_string(),
                ),
                cached_value: None,
            },
            WorkbookGraphNode {
                id: "power-query:finalquery".to_string(),
                kind: WorkbookGraphNodeKind::PowerQueryQuery,
                label: "FinalQuery".to_string(),
                sheet_name: None,
                part_path: Some("Formulas/Section1.m".to_string()),
                cell_reference: None,
                formula: Some(
                    concat!(
                        "shared FinalQuery = let\n",
                        "    Source = TableQuery\n",
                        "in\n",
                        "    Source;"
                    )
                    .to_string(),
                ),
                cached_value: None,
            },
        ]
    );
    assert_eq!(
        result
            .edges
            .iter()
            .filter(|edge| {
                matches!(
                    edge.kind,
                    WorkbookGraphEdgeKind::PowerQueryReferencesDefinedName
                        | WorkbookGraphEdgeKind::PowerQueryReferencesTable
                        | WorkbookGraphEdgeKind::PowerQueryReferencesQuery
                        | WorkbookGraphEdgeKind::DefinedNameTargetsCell
                )
            })
            .cloned()
            .collect::<Vec<_>>(),
        vec![
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::PowerQueryReferencesDefinedName,
                from: "power-query:thresholdquery".to_string(),
                to: "defined-name:workbook:threshold".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::PowerQueryLexicalWorkbookNameReference,
                    part_path: "Formulas/Section1.m".to_string(),
                    cell_reference: None,
                    detail: "Power Query ThresholdQuery resolved Excel.CurrentWorkbook name Threshold to defined name Threshold on line 2".to_string(),
                }],
            },
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::DefinedNameTargetsCell,
                from: "defined-name:workbook:threshold".to_string(),
                to: "ref-cell:0:$D$1".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::WorkbookDefinedNameTarget,
                    part_path: "xl/workbook.xml".to_string(),
                    cell_reference: None,
                    detail: "defined name Threshold targets cell Summary!$D$1".to_string(),
                }],
            },
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::PowerQueryReferencesTable,
                from: "power-query:tablequery".to_string(),
                to: "table:0:sales".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::PowerQueryLexicalWorkbookNameReference,
                    part_path: "Formulas/Section1.m".to_string(),
                    cell_reference: None,
                    detail: "Power Query TableQuery resolved Excel.CurrentWorkbook name Sales to table Summary!Sales on line 2".to_string(),
                }],
            },
            WorkbookGraphEdge {
                kind: WorkbookGraphEdgeKind::PowerQueryReferencesQuery,
                from: "power-query:finalquery".to_string(),
                to: "power-query:tablequery".to_string(),
                evidence: vec![WorkbookGraphEvidence {
                    kind: WorkbookGraphEvidenceKind::PowerQueryLexicalQueryReference,
                    part_path: "Formulas/Section1.m".to_string(),
                    cell_reference: None,
                    detail: "Power Query FinalQuery lexically references query TableQuery on line 2".to_string(),
                }],
            },
        ]
    );
    assert_eq!(
        result.warnings.contains(
            &"workbook contains unresolved Power Query Excel.CurrentWorkbook name references; graph omits lineage edges for unresolved workbook-name targets"
                .to_string()
        ),
        false
    );
}

#[test]
fn inspect_workbook_graph_warns_for_unresolved_powerquery_workbook_names() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("missing_powerquery_target.xlsm");
    write_unresolved_powerquery_graph_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("missing_powerquery_target.xlsm"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result.warnings.contains(
            &"workbook contains unresolved Power Query Excel.CurrentWorkbook name references; graph omits lineage edges for unresolved workbook-name targets"
                .to_string()
        ),
        true
    );
    assert_eq!(
        result
            .edges
            .iter()
            .filter(|edge| {
                matches!(
                    edge.kind,
                    WorkbookGraphEdgeKind::PowerQueryReferencesDefinedName
                        | WorkbookGraphEdgeKind::PowerQueryReferencesTable
                )
            })
            .count(),
        0
    );
}

#[test]
fn inspect_workbook_graph_warns_for_unresolved_structured_table_references() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("missing_table_reference.xlsx");
    write_missing_table_reference_graph_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("missing_table_reference.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result.warnings.contains(
            &"selected sheet contains unresolved structured table references; graph omits dependency edges for unresolved table targets"
                .to_string()
        ),
        true
    );
    assert_eq!(
        result
            .edges
            .iter()
            .filter(|edge| edge.kind == WorkbookGraphEdgeKind::FormulaReferencesTable)
            .count(),
        0
    );
}

#[test]
fn inspect_workbook_graph_warns_for_unresolved_table_metadata() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("unresolved_table_metadata.xlsx");
    write_unresolved_table_metadata_graph_fixture(&path);

    let result = inspect_workbook_graph_with_display_path(
        &path,
        std::path::Path::new("unresolved_table_metadata.xlsx"),
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
        Some(10),
    )
    .expect("inspect workbook graph");

    assert_eq!(
        result.warnings.contains(
            &"workbook contains unresolved table metadata; graph omits table edges unless worksheet ownership and table ranges are proven"
                .to_string()
        ),
        true
    );
    assert_eq!(
        result
            .edges
            .iter()
            .filter(|edge| edge.kind == WorkbookGraphEdgeKind::TableHasRange)
            .count(),
        0
    );
}

#[tokio::test]
async fn inspect_workbook_graph_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/formulas.xlsx");
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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_GRAPH_TOOL_NAME)
        })
        .expect("excel workbook graph tool");
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "data/formulas.xlsx",
            "sheet": { "type": "name", "name": "Summary" },
            "max_formulas": 2,
        })
        .to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_GRAPH_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect workbook graph");

    assert_eq!(
        serde_json::from_value::<InspectWorkbookGraphResult>(
            output
                .post_tool_use_response("call-1", &payload)
                .expect("json response")
        )
        .expect("deserialize workbook graph result"),
        InspectWorkbookGraphResult {
            path: "data/formulas.xlsx".to_string(),
            mode: WorkbookGraphMode::PackageStructurePlusPerSheetFormulaMembership,
            is_partial: true,
            formula_inventory_scope: WorkbookGraphFormulaInventoryScope::SelectedSheetOnly,
            formula_inventory_sheet: SheetPreview {
                name: "Summary".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            max_formulas_applied: 2,
            formula_inventory_truncated: true,
            nodes: vec![
                WorkbookGraphNode {
                    id: "workbook".to_string(),
                    kind: WorkbookGraphNodeKind::Workbook,
                    label: "data/formulas.xlsx".to_string(),
                    sheet_name: None,
                    part_path: None,
                    cell_reference: None,
                    formula: None,
                    cached_value: None,
                },
                WorkbookGraphNode {
                    id: "worksheet:0".to_string(),
                    kind: WorkbookGraphNodeKind::Worksheet,
                    label: "Summary".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: None,
                    formula: None,
                    cached_value: None,
                },
                WorkbookGraphNode {
                    id: "formula:0:A1".to_string(),
                    kind: WorkbookGraphNodeKind::CellFormula,
                    label: "Summary!A1".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: Some("A1".to_string()),
                    formula: Some("SUM(B1:B2)".to_string()),
                    cached_value: Some("7".to_string()),
                },
                WorkbookGraphNode {
                    id: "ref-range:0:B1:B2".to_string(),
                    kind: WorkbookGraphNodeKind::ReferencedRange,
                    label: "Summary!B1:B2".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: Some("B1:B2".to_string()),
                    formula: None,
                    cached_value: None,
                },
                WorkbookGraphNode {
                    id: "formula:0:B1".to_string(),
                    kind: WorkbookGraphNodeKind::CellFormula,
                    label: "Summary!B1".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: Some("B1".to_string()),
                    formula: Some("A1&\"x\"".to_string()),
                    cached_value: Some("North".to_string()),
                },
                WorkbookGraphNode {
                    id: "ref-cell:0:A1".to_string(),
                    kind: WorkbookGraphNodeKind::ReferencedCell,
                    label: "Summary!A1".to_string(),
                    sheet_name: Some("Summary".to_string()),
                    part_path: Some("xl/worksheets/sheet1.xml".to_string()),
                    cell_reference: Some("A1".to_string()),
                    formula: None,
                    cached_value: None,
                },
            ],
            edges: vec![
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::WorkbookContainsWorksheet,
                    from: "workbook".to_string(),
                    to: "worksheet:0".to_string(),
                    evidence: vec![
                        WorkbookGraphEvidence {
                            kind: WorkbookGraphEvidenceKind::WorkbookSheetEntry,
                            part_path: "xl/workbook.xml".to_string(),
                            cell_reference: None,
                            detail: "sheet entry name=Summary sheet_id=1 relationship_id=rId1"
                                .to_string(),
                        },
                        WorkbookGraphEvidence {
                            kind: WorkbookGraphEvidenceKind::WorkbookRelationship,
                            part_path: "xl/_rels/workbook.xml.rels".to_string(),
                            cell_reference: None,
                            detail: "worksheet part target xl/worksheets/sheet1.xml".to_string(),
                        },
                    ],
                },
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::WorksheetContainsFormula,
                    from: "worksheet:0".to_string(),
                    to: "formula:0:A1".to_string(),
                    evidence: vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::WorksheetFormulaCell,
                        part_path: "xl/worksheets/sheet1.xml".to_string(),
                        cell_reference: Some("A1".to_string()),
                        detail: "selected-sheet formula inventory entry".to_string(),
                    }],
                },
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::FormulaReferencesRange,
                    from: "formula:0:A1".to_string(),
                    to: "ref-range:0:B1:B2".to_string(),
                    evidence: vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::FormulaAstRangeReference,
                        part_path: "xl/worksheets/sheet1.xml".to_string(),
                        cell_reference: Some("A1".to_string()),
                        detail: "ast range reference Summary!B1:B2".to_string(),
                    }],
                },
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::WorksheetContainsFormula,
                    from: "worksheet:0".to_string(),
                    to: "formula:0:B1".to_string(),
                    evidence: vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::WorksheetFormulaCell,
                        part_path: "xl/worksheets/sheet1.xml".to_string(),
                        cell_reference: Some("B1".to_string()),
                        detail: "selected-sheet formula inventory entry".to_string(),
                    }],
                },
                WorkbookGraphEdge {
                    kind: WorkbookGraphEdgeKind::FormulaReferencesCell,
                    from: "formula:0:B1".to_string(),
                    to: "ref-cell:0:A1".to_string(),
                    evidence: vec![WorkbookGraphEvidence {
                        kind: WorkbookGraphEvidenceKind::FormulaAstCellReference,
                        part_path: "xl/worksheets/sheet1.xml".to_string(),
                        cell_reference: Some("B1".to_string()),
                        detail: "ast cell reference Summary!A1".to_string(),
                    }],
                },
            ],
            warnings: vec![
                "formula inventory truncated to 2 formulas".to_string(),
                "workbook graph is partial: it emits workbook structure, selected-sheet formula membership, and AST-backed dependency edges only where proven".to_string(),
                "workbook has external links; phase 1 graph omits external-link lineage".to_string(),
            ],
        }
    );
}
