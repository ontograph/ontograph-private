use std::time::Duration;

use crate::session::session::Session;
use crate::session::turn_context::TurnContext;
use ontocode_otel::sanitize_metric_tag_value;
use ontocode_protocol::mcp::CallToolResult;
use serde_json::Value as JsonValue;
use tracing::Span;
use tracing::field::Empty;
use url::Url;

const MCP_CALL_COUNT_METRIC: &str = "codex.mcp.call";
const MCP_CALL_DURATION_METRIC: &str = "codex.mcp.call.duration_ms";
const MCP_RESULT_TELEMETRY_META_KEY: &str = "codex/telemetry";
const MCP_RESULT_TELEMETRY_SPAN_KEY: &str = "span";
const MCP_RESULT_TELEMETRY_TARGET_ID_KEY: &str = "target_id";
const MCP_RESULT_TELEMETRY_DID_TRIGGER_SERVER_USER_FLOW_KEY: &str = "did_trigger_server_user_flow";
const MCP_RESULT_TELEMETRY_TARGET_ID_SPAN_ATTR: &str = "codex.mcp.target.id";
const MCP_RESULT_TELEMETRY_SERVER_USER_FLOW_SPAN_ATTR: &str =
    "codex.mcp.server_user_flow.triggered";
pub(crate) const MCP_RESULT_TELEMETRY_TARGET_ID_MAX_CHARS: usize = 256;

pub(crate) fn emit_mcp_call_metrics(
    turn_context: &TurnContext,
    status: &str,
    tool_name: &str,
    connector_id: Option<&str>,
    connector_name: Option<&str>,
    duration: Option<Duration>,
) {
    let tags = mcp_call_metric_tags(status, tool_name, connector_id, connector_name);
    let tag_refs: Vec<(&str, &str)> = tags
        .iter()
        .map(|(key, value)| (*key, value.as_str()))
        .collect();
    turn_context
        .session_telemetry
        .counter(MCP_CALL_COUNT_METRIC, /*inc*/ 1, &tag_refs);
    if let Some(duration) = duration {
        turn_context.session_telemetry.record_duration(
            MCP_CALL_DURATION_METRIC,
            duration,
            &tag_refs,
        );
    }
}

fn mcp_call_metric_tags(
    status: &str,
    tool_name: &str,
    connector_id: Option<&str>,
    connector_name: Option<&str>,
) -> Vec<(&'static str, String)> {
    let mut tags = vec![
        ("status", sanitize_metric_tag_value(status)),
        ("tool", sanitize_metric_tag_value(tool_name)),
    ];
    if let Some(connector_id) = connector_id.filter(|connector_id| !connector_id.is_empty()) {
        tags.push(("connector_id", sanitize_metric_tag_value(connector_id)));
    }
    if let Some(connector_name) = connector_name.filter(|connector_name| !connector_name.is_empty())
    {
        tags.push(("connector_name", sanitize_metric_tag_value(connector_name)));
    }
    tags
}

pub(crate) fn mcp_tool_call_span(
    session: &Session,
    turn_context: &TurnContext,
    fields: McpToolCallSpanFields<'_>,
) -> Span {
    let transport = match fields.server_origin {
        Some("stdio") => "stdio",
        Some("in_process") => "in_process",
        Some(_) => "streamable_http",
        None => "",
    };
    let span = tracing::info_span!(
        "mcp.tools.call",
        otel.kind = "client",
        rpc.system = "jsonrpc",
        rpc.method = "tools/call",
        mcp.server.name = fields.server_name,
        mcp.server.origin = fields.server_origin.unwrap_or(""),
        mcp.transport = transport,
        mcp.connector.id = fields.connector_id.unwrap_or(""),
        mcp.connector.name = fields.connector_name.unwrap_or(""),
        tool.name = fields.tool_name,
        tool.call_id = fields.call_id,
        conversation.id = %session.thread_id,
        session.id = %session.thread_id,
        turn.id = turn_context.sub_id.as_str(),
        server.address = Empty,
        server.port = Empty,
        codex.mcp.target.id = Empty,
        codex.mcp.server_user_flow.triggered = Empty,
    );
    record_server_fields(&span, fields.server_origin);
    span
}

pub(crate) struct McpToolCallSpanFields<'a> {
    pub(crate) server_name: &'a str,
    pub(crate) tool_name: &'a str,
    pub(crate) call_id: &'a str,
    pub(crate) server_origin: Option<&'a str>,
    pub(crate) connector_id: Option<&'a str>,
    pub(crate) connector_name: Option<&'a str>,
}

fn record_server_fields(span: &Span, url: Option<&str>) {
    let Some(url) = url else {
        return;
    };
    let Ok(parsed) = Url::parse(url) else {
        return;
    };
    if let Some(host) = parsed.host_str() {
        span.record("server.address", host);
    }
    if let Some(port) = parsed.port_or_known_default() {
        span.record("server.port", port as i64);
    }
}

pub(crate) fn record_mcp_result_span_telemetry(span: &Span, result: Option<&CallToolResult>) {
    let Some(span_telemetry) = result
        .and_then(|result| result.meta.as_ref())
        .and_then(JsonValue::as_object)
        .and_then(|meta| meta.get(MCP_RESULT_TELEMETRY_META_KEY))
        .and_then(JsonValue::as_object)
        .and_then(|telemetry| telemetry.get(MCP_RESULT_TELEMETRY_SPAN_KEY))
        .and_then(JsonValue::as_object)
    else {
        return;
    };

    if let Some(target_id) = span_telemetry
        .get(MCP_RESULT_TELEMETRY_TARGET_ID_KEY)
        .and_then(JsonValue::as_str)
        .filter(|target_id| !target_id.is_empty())
    {
        span.record(
            MCP_RESULT_TELEMETRY_TARGET_ID_SPAN_ATTR,
            truncate_str_to_char_boundary(target_id, MCP_RESULT_TELEMETRY_TARGET_ID_MAX_CHARS),
        );
    }

    if let Some(did_trigger_server_user_flow) = span_telemetry
        .get(MCP_RESULT_TELEMETRY_DID_TRIGGER_SERVER_USER_FLOW_KEY)
        .and_then(JsonValue::as_bool)
    {
        span.record(
            MCP_RESULT_TELEMETRY_SERVER_USER_FLOW_SPAN_ATTR,
            did_trigger_server_user_flow,
        );
    }
}

pub(crate) fn truncate_str_to_char_boundary(value: &str, max_chars: usize) -> &str {
    match value.char_indices().nth(max_chars) {
        Some((index, _)) => &value[..index],
        None => value,
    }
}
