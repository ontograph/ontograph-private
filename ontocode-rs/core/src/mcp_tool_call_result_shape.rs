use ontocode_protocol::mcp::CallToolResult;
use ontocode_utils_output_truncation::TruncationPolicy;
use ontocode_utils_output_truncation::truncate_text;

pub(crate) const MCP_TOOL_CALL_EVENT_RESULT_MAX_BYTES: usize = 64 * 1024;

pub(crate) fn sanitize_mcp_tool_result_for_model(
    supports_image_input: bool,
    result: Result<CallToolResult, String>,
) -> Result<CallToolResult, String> {
    if supports_image_input {
        return result;
    }

    result.map(|call_tool_result| CallToolResult {
        content: call_tool_result
            .content
            .iter()
            .map(|block| {
                if let Some(content_type) = block.get("type").and_then(serde_json::Value::as_str)
                    && content_type == "image"
                {
                    return serde_json::json!({
                        "type": "text",
                        "text": "<image content omitted because you do not support image input>",
                    });
                }

                block.clone()
            })
            .collect::<Vec<_>>(),
        structured_content: call_tool_result.structured_content,
        is_error: call_tool_result.is_error,
        meta: call_tool_result.meta,
    })
}

pub(crate) fn truncate_mcp_tool_result_for_event(
    result: &Result<CallToolResult, String>,
) -> Result<CallToolResult, String> {
    match result {
        Ok(call_tool_result) => {
            // The app-server rebuilds `ThreadItem::McpToolCall` from this item,
            // so avoid persisting multi-megabyte results in rollout storage.
            let Ok(serialized) = serde_json::to_string(call_tool_result) else {
                return Ok(call_tool_result.clone());
            };
            if serialized.len() <= MCP_TOOL_CALL_EVENT_RESULT_MAX_BYTES {
                return Ok(call_tool_result.clone());
            }

            // A huge MCP result can put bytes in `content`, `structuredContent`,
            // or `_meta`. Collapse the event copy to a text preview of the whole
            // serialized result so the UI still has useful context without
            // preserving a multi-megabyte structured payload.
            //
            // This budget applies to the preview text, not the final event JSON.
            // The preview is itself serialized into a JSON string, so quotes and
            // backslashes can be escaped again and the stored event may end up
            // somewhat larger than this byte budget.
            let truncated = truncate_text(
                &serialized,
                TruncationPolicy::Bytes(MCP_TOOL_CALL_EVENT_RESULT_MAX_BYTES),
            );
            Ok(CallToolResult {
                content: vec![serde_json::json!({
                    "type": "text",
                    "text": truncated,
                })],
                structured_content: None,
                is_error: call_tool_result.is_error,
                meta: None,
            })
        }
        Err(message) => Err(truncate_text(
            message,
            TruncationPolicy::Bytes(MCP_TOOL_CALL_EVENT_RESULT_MAX_BYTES),
        )),
    }
}
