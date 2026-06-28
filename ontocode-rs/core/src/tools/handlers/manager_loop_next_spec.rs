use ontocode_tools::JsonSchema;
use ontocode_tools::ResponsesApiTool;
use ontocode_tools::ToolSpec;
use serde_json::json;
use std::collections::BTreeMap;

pub const MANAGER_LOOP_NEXT_TOOL_NAME: &str = "manager_loop_next";

pub fn create_manager_loop_next_tool() -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "tracking_path".to_string(),
            JsonSchema::string(Some(
                "Workspace-relative tracking file under `.memory-bank/`. Markdown files only."
                    .to_string(),
            )),
        ),
        (
            "mode".to_string(),
            JsonSchema::string_enum(
                vec![json!("strict")],
                Some("Strict mode only for the first slice.".to_string()),
            ),
        ),
    ]);

    ToolSpec::Function(ResponsesApiTool {
        name: MANAGER_LOOP_NEXT_TOOL_NAME.to_string(),
        description: "Read a strict `.memory-bank` tracking file, parse the structured `manager_loop` YAML block, and return the next bounded manager-loop decision without generating a worker prompt."
            .to_string(),
        strict: true,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["tracking_path".to_string(), "mode".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    })
}
