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
use crate::sheet_layout_metadata::INSPECT_SHEET_LAYOUT_METADATA_TOOL_NAME;
use crate::sheet_layout_metadata::InspectSheetLayoutMetadataResult;
use crate::sheet_layout_metadata::SheetLayoutPaneKind;
use crate::sheet_layout_metadata::SheetLayoutPaneSummary;
use crate::sheet_layout_metadata::inspect_sheet_layout_metadata_with_display_path;
use crate::tests::write_zip_fixture;
use crate::tool::EXCEL_NAMESPACE;
use crate::tool::SheetPreview;
use crate::tool::SheetSelector;

fn write_sheet_layout_fixture(path: &std::path::Path) {
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
                    <definedName name="_xlnm.Print_Area" localSheetId="0">'Summary'!$A$1:$D$20</definedName>
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
                  <sheetViews>
                    <sheetView workbookViewId="0">
                      <pane state="frozen" xSplit="1" ySplit="1" topLeftCell="B2" activePane="bottomRight"/>
                    </sheetView>
                  </sheetViews>
                  <autoFilter ref="$A$1:$D$20"/>
                  <mergeCells count="18">
                    <mergeCell ref="A1:B1"/>
                    <mergeCell ref="C1:D1"/>
                    <mergeCell ref="A3:A4"/>
                    <mergeCell ref="B3:B4"/>
                    <mergeCell ref="C3:C4"/>
                    <mergeCell ref="D3:D4"/>
                    <mergeCell ref="A6:B6"/>
                    <mergeCell ref="C6:D6"/>
                    <mergeCell ref="A8:B8"/>
                    <mergeCell ref="C8:D8"/>
                    <mergeCell ref="A10:B10"/>
                    <mergeCell ref="C10:D10"/>
                    <mergeCell ref="A12:B12"/>
                    <mergeCell ref="C12:D12"/>
                    <mergeCell ref="A14:B14"/>
                    <mergeCell ref="C14:D14"/>
                    <mergeCell ref="A16:B16"/>
                    <mergeCell ref="C16:D16"/>
                  </mergeCells>
                </worksheet>"#,
            ),
        ],
    );
}

#[test]
fn inspect_sheet_layout_metadata_reports_bounded_layout_summary() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("layout.xlsx");
    write_sheet_layout_fixture(&path);

    let result = inspect_sheet_layout_metadata_with_display_path(
        &path,
        &path,
        &SheetSelector::Name {
            name: "Summary".to_string(),
        },
    )
    .expect("inspect sheet layout metadata");

    assert_eq!(
        result,
        InspectSheetLayoutMetadataResult {
            mode: "read_only_inspection".to_string(),
            path: path.display().to_string(),
            sheet: SheetPreview {
                name: "Summary".to_string(),
                sheet_id: Some(1),
                part_path: "xl/worksheets/sheet1.xml".to_string(),
            },
            merged_range_count: 18,
            merged_ranges_sample: vec![
                "A1:B1".to_string(),
                "C1:D1".to_string(),
                "A3:A4".to_string(),
                "B3:B4".to_string(),
                "C3:C4".to_string(),
                "D3:D4".to_string(),
                "A6:B6".to_string(),
                "C6:D6".to_string(),
                "A8:B8".to_string(),
                "C8:D8".to_string(),
                "A10:B10".to_string(),
                "C10:D10".to_string(),
                "A12:B12".to_string(),
                "C12:D12".to_string(),
                "A14:B14".to_string(),
                "C14:D14".to_string(),
            ],
            pane: Some(SheetLayoutPaneSummary {
                kind: SheetLayoutPaneKind::Freeze,
                top_left_cell: Some("B2".to_string()),
                x_split: Some("1".to_string()),
                y_split: Some("1".to_string()),
                active_pane: Some("bottomRight".to_string()),
            }),
            auto_filter_range: Some("$A$1:$D$20".to_string()),
            print_area: Some("'Summary'!$A$1:$D$20".to_string()),
            warnings: vec![
                "merged range sample truncated to 16 entries for sheet Summary".to_string()
            ],
        }
    );
}

#[tokio::test]
async fn inspect_sheet_layout_metadata_tool_resolves_relative_path_against_turn_cwd() {
    let dir = tempdir().expect("temp dir");
    let workspace = dir.path().join("workspace");
    std::fs::create_dir_all(workspace.join("data")).expect("create workspace");
    let workbook_path = workspace.join("data/layout.xlsx");
    write_sheet_layout_fixture(&workbook_path);

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
                == ToolName::namespaced(EXCEL_NAMESPACE, INSPECT_SHEET_LAYOUT_METADATA_TOOL_NAME)
        })
        .expect("excel sheet layout metadata tool");
    let payload = ToolPayload::Function {
        arguments: json!({
            "path": "data/layout.xlsx",
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
                INSPECT_SHEET_LAYOUT_METADATA_TOOL_NAME,
            ),
            model: "gpt-test".to_string(),
            truncation_policy: TruncationPolicy::Bytes(1024),
            conversation_history: ConversationHistory::default(),
            turn_item_emitter: Arc::new(NoopTurnItemEmitter),
            payload: payload.clone(),
        })
        .await
        .expect("tool should inspect sheet layout metadata");

    let result = serde_json::from_value::<InspectSheetLayoutMetadataResult>(
        output
            .post_tool_use_response("call-1", &payload)
            .expect("json response"),
    )
    .expect("deserialize layout metadata result");
    assert_eq!(result.path, "data/layout.xlsx".to_string());
    assert_eq!(result.sheet.name, "Summary".to_string());
}
