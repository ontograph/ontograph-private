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
use crate::tests::write_zip_fixture;
use crate::tool::EXCEL_NAMESPACE;
use crate::workbook_tables::INSPECT_WORKBOOK_TABLES_TOOL_NAME;
use crate::workbook_tables::InspectWorkbookTablesResult;
use crate::workbook_tables::WorkbookTableSummary;
use crate::workbook_tables::inspect_workbook_tables_from_workbook;

fn write_workbook_tables_fixture(path: &std::path::Path) {
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
                      <c r="A1" t="inlineStr"><is><t>Category</t></is></c>
                      <c r="B1" t="inlineStr"><is><t>Amount</t></is></c>
                    </row>
                  </sheetData>
                  <tableParts count="3">
                    <tablePart r:id="rIdTable1" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>
                    <tablePart r:id="rIdTable2" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>
                    <tablePart r:id="rIdMissing" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>
                  </tableParts>
                </worksheet>"#,
            ),
            (
                "xl/worksheets/_rels/sheet1.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                  <Relationship Id="rIdTable1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/table" Target="../tables/table1.xml"/>
                  <Relationship Id="rIdTable2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/table" Target="../tables/table2.xml"/>
                  <Relationship Id="rIdMissing" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/table" Target="../tables/missing.xml"/>
                </Relationships>"#,
            ),
            (
                "xl/tables/table1.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <table xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                       id="1"
                       name="Sales_Internal"
                       displayName="Sales"
                       ref="$A$1:$J$4"
                       headerRowCount="1"
                       totalsRowShown="1">
                  <tableColumns count="10">
                    <tableColumn id="1" name="Category"/>
                    <tableColumn id="2" name="Amount"/>
                    <tableColumn id="3" name="Jan"/>
                    <tableColumn id="4" name="Feb"/>
                    <tableColumn id="5" name="Mar"/>
                    <tableColumn id="6" name="Apr"/>
                    <tableColumn id="7" name="May"/>
                    <tableColumn id="8" name="Jun"/>
                    <tableColumn id="9" name="Jul"/>
                    <tableColumn id="10" name="Aug"/>
                  </tableColumns>
                </table>"#,
            ),
            (
                "xl/tables/table2.xml",
                r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <table xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                       id="2"
                       name="Lookup_Internal"
                       displayName="Lookup"
                       ref="$M$2:$N$4"
                       headerRowCount="0"
                       totalsRowCount="0">
                  <tableColumns count="2">
                    <tableColumn id="1" name="Key"/>
                    <tableColumn id="2" name="Value"/>
                  </tableColumns>
                </table>"#,
            ),
        ],
    );
}

#[test]
fn inspect_workbook_tables_reports_bounded_inventory_and_warnings() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("tables.xlsx");
    write_workbook_tables_fixture(&path);

    let result = inspect_workbook_tables_from_workbook(&path, &path);

    assert_eq!(
        result,
        InspectWorkbookTablesResult {
            mode: "read_only_inspection".to_string(),
            path: path.display().to_string(),
            table_count: 2,
            tables: vec![
                WorkbookTableSummary {
                    name: "Sales".to_string(),
                    alt_name: Some("Sales_Internal".to_string()),
                    sheet_name: "Summary".to_string(),
                    part_path: "xl/tables/table1.xml".to_string(),
                    range_reference: "$A$1:$J$4".to_string(),
                    has_header_row: Some(true),
                    has_totals_row: Some(true),
                    column_names_sample: vec![
                        "Category".to_string(),
                        "Amount".to_string(),
                        "Jan".to_string(),
                        "Feb".to_string(),
                        "Mar".to_string(),
                        "Apr".to_string(),
                        "May".to_string(),
                        "Jun".to_string(),
                    ],
                },
                WorkbookTableSummary {
                    name: "Lookup".to_string(),
                    alt_name: Some("Lookup_Internal".to_string()),
                    sheet_name: "Summary".to_string(),
                    part_path: "xl/tables/table2.xml".to_string(),
                    range_reference: "$M$2:$N$4".to_string(),
                    has_header_row: Some(false),
                    has_totals_row: Some(false),
                    column_names_sample: vec!["Key".to_string(), "Value".to_string()],
                },
            ],
            warnings: vec![
                "column-name sample for table `Summary!Sales` truncated to 8 entries".to_string(),
                "workbook contains unresolved table metadata; only tables with proven worksheet ownership and range metadata are reported".to_string(),
            ],
        }
    );
}

#[tokio::test]
async fn inspect_workbook_tables_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/tables.xlsx");
    write_workbook_tables_fixture(&workbook_path);

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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_TABLES_TOOL_NAME)
        })
        .expect("excel workbook tables tool");
    let payload = ToolPayload::Function {
        arguments: json!({ "path": "data/tables.xlsx" }).to_string(),
    };
    let output = tool
        .handle(ToolCall {
            turn_id: "turn-1".to_string(),
            call_id: "call-1".to_string(),
            tool_name: ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_WORKBOOK_TABLES_TOOL_NAME),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect workbook tables");

    let result = serde_json::from_value::<InspectWorkbookTablesResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize workbook tables result");
    assert_eq!(result.path, "data/tables.xlsx".to_string());
    assert_eq!(result.table_count, 2);
}
