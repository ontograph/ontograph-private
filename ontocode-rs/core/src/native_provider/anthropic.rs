use crate::client_common::Prompt;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use http::HeaderMap;
use http::HeaderValue;
use ontocode_api::ApiError;
use ontocode_api::ResponseEvent;
use ontocode_api::ResponseStream as ApiResponseStream;
use ontocode_model_provider_info::ModelProviderInfo;
use ontocode_protocol::error::CodexErr;
use ontocode_protocol::error::Result;
use ontocode_protocol::models::ContentItem;
use ontocode_protocol::models::FunctionCallOutputBody;
use ontocode_protocol::models::FunctionCallOutputContentItem;
use ontocode_protocol::models::FunctionCallOutputPayload;
use ontocode_protocol::models::ResponseItem;
use ontocode_protocol::openai_models::ModelInfo;
use ontocode_protocol::protocol::TokenUsage;
use ontocode_tools::JsonSchema;
use ontocode_tools::ResponsesApiTool;
use ontocode_tools::ToolSpec;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use tokio::sync::mpsc;
use tokio::time::timeout;
use tracing::trace;

const ANTHROPIC_DEFAULT_BASE_URL: &str = "https://api.anthropic.com/v1";
const ANTHROPIC_MESSAGES_PATH: &str = "messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_MAX_TOKENS: u32 = 4096;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct AnthropicMessagesRequest {
    model: String,
    max_tokens: u32,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
    messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct AnthropicMessage {
    role: AnthropicRole,
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum AnthropicRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicContent {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    ToolResult {
        tool_use_id: String,
        content: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct AnthropicTool {
    name: String,
    description: String,
    input_schema: JsonSchema,
}

pub(crate) fn build_messages_request(
    prompt: &Prompt,
    model_info: &ModelInfo,
) -> Result<AnthropicMessagesRequest> {
    if prompt.parallel_tool_calls {
        return Err(unsupported(
            "Claude native runtime does not support parallel tool calls",
        ));
    }
    if prompt.output_schema.is_some() {
        return Err(unsupported(
            "Claude native runtime does not support output schemas yet",
        ));
    }

    let tools = anthropic_tools(&prompt.tools)?;
    let mut system_parts = Vec::new();
    if !prompt.base_instructions.text.trim().is_empty() {
        system_parts.push(prompt.base_instructions.text.clone());
    }

    let mut messages = Vec::new();
    for item in prompt.get_formatted_input() {
        match item {
            ResponseItem::Message { role, content, .. } => {
                let text = text_content(content)?;
                match role.as_str() {
                    "user" => messages.push(text_message(AnthropicRole::User, text)),
                    "assistant" => messages.push(text_message(AnthropicRole::Assistant, text)),
                    "system" | "developer" => system_parts.push(text),
                    other => {
                        return Err(unsupported(format!(
                            "Claude native runtime does not support message role `{other}`"
                        )));
                    }
                }
            }
            ResponseItem::Reasoning { .. } => {}
            ResponseItem::FunctionCall {
                id: _,
                name,
                namespace,
                arguments,
                call_id,
            } => {
                if namespace.is_some() {
                    return Err(unsupported(
                        "Claude native runtime does not support namespaced tool-call history yet",
                    ));
                }
                messages.push(AnthropicMessage {
                    role: AnthropicRole::Assistant,
                    content: vec![AnthropicContent::ToolUse {
                        id: call_id,
                        name,
                        input: parse_tool_arguments(&arguments)?,
                    }],
                });
            }
            ResponseItem::FunctionCallOutput { call_id, output } => {
                messages.push(AnthropicMessage {
                    role: AnthropicRole::User,
                    content: vec![AnthropicContent::ToolResult {
                        tool_use_id: call_id,
                        content: tool_result_content(output)?,
                    }],
                });
            }
            ResponseItem::CustomToolCall { .. }
            | ResponseItem::CustomToolCallOutput { .. }
            | ResponseItem::LocalShellCall { .. }
            | ResponseItem::ToolSearchCall { .. }
            | ResponseItem::ToolSearchOutput { .. }
            | ResponseItem::WebSearchCall { .. }
            | ResponseItem::ImageGenerationCall { .. }
            | ResponseItem::Compaction { .. }
            | ResponseItem::CompactionTrigger
            | ResponseItem::ContextCompaction { .. }
            | ResponseItem::Other => {
                return Err(unsupported(
                    "Claude native runtime does not support this tool or non-text history yet",
                ));
            }
        }
    }

    Ok(AnthropicMessagesRequest {
        model: model_info.slug.clone(),
        max_tokens: DEFAULT_MAX_TOKENS,
        stream: true,
        system: (!system_parts.is_empty()).then(|| system_parts.join("\n\n")),
        tools,
        messages,
    })
}

pub(crate) async fn stream_messages(
    client: Client,
    provider_info: &ModelProviderInfo,
    request: AnthropicMessagesRequest,
    mut extra_headers: HeaderMap,
) -> Result<ApiResponseStream> {
    let api_key = provider_info.api_key()?.ok_or_else(|| {
        CodexErr::UnsupportedOperation(
            "Claude native runtime requires provider env_key API-key auth".to_string(),
        )
    })?;
    let url = anthropic_url(provider_info)?;
    add_configured_headers(provider_info, &mut extra_headers);
    extra_headers.insert(
        "anthropic-version",
        HeaderValue::from_static(ANTHROPIC_VERSION),
    );
    extra_headers.insert(
        http::header::ACCEPT,
        HeaderValue::from_static("text/event-stream"),
    );
    extra_headers.insert(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    extra_headers.insert(
        "x-api-key",
        HeaderValue::from_str(&api_key).map_err(|err| {
            CodexErr::UnsupportedOperation(format!("invalid Claude API key: {err}"))
        })?,
    );

    let response = client
        .post(url)
        .headers(extra_headers)
        .json(&request)
        .send()
        .await
        .map_err(|err| CodexErr::UnsupportedOperation(format!("Claude request failed: {err}")))?;

    let request_id = response
        .headers()
        .get("request-id")
        .or_else(|| response.headers().get("x-request-id"))
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let status = response.status();
    if !status.is_success() {
        let message = response
            .text()
            .await
            .unwrap_or_else(|err| format!("failed to read Claude error response: {err}"));
        return Err(CodexErr::UnsupportedOperation(format!(
            "Claude request failed with status {status}: {message}"
        )));
    }

    let stream = response.bytes_stream();
    let (tx_event, rx_event) = mpsc::channel(1600);
    let idle_timeout = provider_info.stream_idle_timeout();
    tokio::spawn(async move {
        process_sse(stream.eventsource(), tx_event, idle_timeout).await;
    });

    Ok(ApiResponseStream {
        rx_event,
        upstream_request_id: request_id,
    })
}

fn text_message(role: AnthropicRole, text: String) -> AnthropicMessage {
    AnthropicMessage {
        role,
        content: vec![AnthropicContent::Text { text }],
    }
}

fn anthropic_tools(tools: &[ToolSpec]) -> Result<Option<Vec<AnthropicTool>>> {
    if tools.is_empty() {
        return Ok(None);
    }

    let mut translated = Vec::with_capacity(tools.len());
    for tool in tools {
        match tool {
            ToolSpec::Function(tool) => translated.push(anthropic_function_tool(tool)),
            ToolSpec::Namespace(_)
            | ToolSpec::ToolSearch { .. }
            | ToolSpec::ImageGeneration { .. }
            | ToolSpec::WebSearch { .. }
            | ToolSpec::Freeform(_) => {
                return Err(unsupported(format!(
                    "Claude native runtime does not support `{}` tool declarations yet",
                    tool.name()
                )));
            }
        }
    }

    Ok(Some(translated))
}

fn anthropic_function_tool(tool: &ResponsesApiTool) -> AnthropicTool {
    AnthropicTool {
        name: tool.name.clone(),
        description: tool.description.clone(),
        input_schema: tool.parameters.clone(),
    }
}

fn parse_tool_arguments(arguments: &str) -> Result<Value> {
    let input: Value = serde_json::from_str(arguments).map_err(|err| {
        unsupported(format!(
            "Claude native runtime cannot translate invalid tool-call arguments: {err}"
        ))
    })?;
    if !input.is_object() {
        return Err(unsupported(
            "Claude native runtime only supports object tool-call arguments",
        ));
    }
    Ok(input)
}

fn tool_result_content(output: FunctionCallOutputPayload) -> Result<String> {
    match output.body {
        FunctionCallOutputBody::Text(content) => Ok(content),
        FunctionCallOutputBody::ContentItems(items) => {
            let mut parts = Vec::new();
            for item in items {
                match item {
                    FunctionCallOutputContentItem::InputText { text } => parts.push(text),
                    FunctionCallOutputContentItem::InputImage { .. } => {
                        return Err(unsupported(
                            "Claude native runtime does not support image tool results yet",
                        ));
                    }
                    FunctionCallOutputContentItem::EncryptedContent { .. } => {
                        return Err(unsupported(
                            "Claude native runtime does not support encrypted tool results",
                        ));
                    }
                }
            }
            Ok(parts.join("\n"))
        }
    }
}

fn text_content(content: Vec<ContentItem>) -> Result<String> {
    let mut parts = Vec::new();
    for item in content {
        match item {
            ContentItem::InputText { text } | ContentItem::OutputText { text } => {
                parts.push(text);
            }
            ContentItem::InputImage { .. } => {
                return Err(unsupported(
                    "Claude native runtime does not support multimodal input yet",
                ));
            }
        }
    }
    Ok(parts.join("\n"))
}

fn anthropic_url(provider_info: &ModelProviderInfo) -> Result<String> {
    let base_url = provider_info
        .base_url
        .as_deref()
        .unwrap_or(ANTHROPIC_DEFAULT_BASE_URL)
        .trim_end_matches('/');
    let mut url = url::Url::parse(&format!("{base_url}/{ANTHROPIC_MESSAGES_PATH}"))
        .map_err(|err| CodexErr::UnsupportedOperation(format!("invalid Claude base_url: {err}")))?;
    if let Some(query_params) = &provider_info.query_params {
        url.query_pairs_mut().extend_pairs(query_params);
    }
    Ok(url.to_string())
}

fn add_configured_headers(provider_info: &ModelProviderInfo, headers: &mut HeaderMap) {
    if let Some(extra) = &provider_info.http_headers {
        for (name, value) in extra {
            if let Ok(name) = http::HeaderName::try_from(name)
                && let Ok(value) = HeaderValue::from_str(value)
            {
                headers.insert(name, value);
            }
        }
    }

    if let Some(env_headers) = &provider_info.env_http_headers {
        for (name, env_var) in env_headers {
            if let Ok(value) = std::env::var(env_var)
                && !value.trim().is_empty()
                && let Ok(name) = http::HeaderName::try_from(name)
                && let Ok(value) = HeaderValue::from_str(&value)
            {
                headers.insert(name, value);
            }
        }
    }
}

async fn process_sse<S>(
    mut stream: S,
    tx_event: mpsc::Sender<std::result::Result<ResponseEvent, ApiError>>,
    idle_timeout: std::time::Duration,
) where
    S: futures::Stream<
            Item = std::result::Result<
                eventsource_stream::Event,
                eventsource_stream::EventStreamError<reqwest::Error>,
            >,
        > + Unpin,
{
    let mut state = AnthropicStreamState::default();
    loop {
        let event = match timeout(idle_timeout, stream.next()).await {
            Ok(Some(Ok(event))) => event,
            Ok(Some(Err(err))) => {
                let _ = tx_event.send(Err(ApiError::Stream(err.to_string()))).await;
                return;
            }
            Ok(None) => {
                let _ = tx_event
                    .send(Err(ApiError::Stream(
                        "Claude stream closed before message_stop".to_string(),
                    )))
                    .await;
                return;
            }
            Err(_) => {
                let _ = tx_event
                    .send(Err(ApiError::Stream(
                        "idle timeout waiting for Claude SSE".to_string(),
                    )))
                    .await;
                return;
            }
        };

        trace!("Claude SSE event: {}", event.data);
        match state.process_data(&event.data) {
            Ok(events) => {
                let completed = events
                    .iter()
                    .any(|event| matches!(event, ResponseEvent::Completed { .. }));
                for event in events {
                    if tx_event.send(Ok(event)).await.is_err() {
                        return;
                    }
                }
                if completed {
                    return;
                }
            }
            Err(err) => {
                let _ = tx_event.send(Err(err)).await;
                return;
            }
        }
    }
}

#[derive(Debug, Default)]
struct AnthropicStreamState {
    response_id: Option<String>,
    text: String,
    tool_uses: BTreeMap<i64, StreamingToolUse>,
    emitted_tool_call: bool,
    input_tokens: i64,
    output_tokens: i64,
}

impl AnthropicStreamState {
    fn process_data(&mut self, data: &str) -> std::result::Result<Vec<ResponseEvent>, ApiError> {
        let event: AnthropicStreamEvent = serde_json::from_str(data).map_err(|err| {
            ApiError::Stream(format!("failed to parse Claude stream event: {err}"))
        })?;
        match event {
            AnthropicStreamEvent::MessageStart { message } => {
                self.response_id = Some(message.id);
                if let Some(usage) = message.usage {
                    self.input_tokens = usage.input_tokens.unwrap_or(0);
                    self.output_tokens = usage.output_tokens.unwrap_or(0);
                }
                Ok(vec![ResponseEvent::Created])
            }
            AnthropicStreamEvent::ContentBlockStart {
                index,
                content_block,
            } => match content_block {
                AnthropicContentBlock::ToolUse { id, name, input } => {
                    if !input.is_object() {
                        return Err(ApiError::Stream(
                            "Claude tool_use input must be a JSON object".to_string(),
                        ));
                    }
                    let mut tool_use = StreamingToolUse::new(id, name);
                    let mut events = vec![ResponseEvent::OutputItemAdded(tool_use.to_item())];
                    if input.as_object().is_some_and(|input| !input.is_empty()) {
                        let delta = serde_json::to_string(&input).map_err(|err| {
                            ApiError::Stream(format!(
                                "failed to serialize Claude tool_use input: {err}"
                            ))
                        })?;
                        tool_use.arguments.push_str(&delta);
                        events.push(tool_use.to_delta(delta));
                    }
                    self.tool_uses.insert(index, tool_use);
                    Ok(events)
                }
                AnthropicContentBlock::Other => Ok(Vec::new()),
            },
            AnthropicStreamEvent::ContentBlockDelta { index, delta } => match delta {
                AnthropicDelta::TextDelta { text } => {
                    self.text.push_str(&text);
                    Ok(vec![ResponseEvent::OutputTextDelta(text)])
                }
                AnthropicDelta::InputJsonDelta { partial_json } => {
                    let Some(tool_use) = self.tool_uses.get_mut(&index) else {
                        return Err(ApiError::Stream(
                            "Claude tool input delta arrived before tool_use start".to_string(),
                        ));
                    };
                    tool_use.arguments.push_str(&partial_json);
                    Ok(vec![tool_use.to_delta(partial_json)])
                }
                AnthropicDelta::Other => Ok(Vec::new()),
            },
            AnthropicStreamEvent::ContentBlockStop { index } => {
                let Some(tool_use) = self.tool_uses.remove(&index) else {
                    return Ok(Vec::new());
                };
                self.emitted_tool_call = true;
                Ok(vec![ResponseEvent::OutputItemDone(
                    tool_use.to_done_item()?,
                )])
            }
            AnthropicStreamEvent::MessageDelta { usage } => {
                if let Some(usage) = usage
                    && let Some(output_tokens) = usage.output_tokens
                {
                    self.output_tokens = output_tokens;
                }
                Ok(Vec::new())
            }
            AnthropicStreamEvent::MessageStop => {
                let response_id = self
                    .response_id
                    .clone()
                    .unwrap_or_else(|| "claude-message".to_string());
                let mut events = Vec::new();
                if !self.text.is_empty() || !self.emitted_tool_call {
                    let message = ResponseItem::Message {
                        id: Some(response_id.clone()),
                        role: "assistant".to_string(),
                        content: vec![ContentItem::OutputText {
                            text: self.text.clone(),
                        }],
                        phase: None,
                    };
                    events.push(ResponseEvent::OutputItemDone(message));
                }
                events.push(ResponseEvent::Completed {
                    response_id,
                    token_usage: Some(TokenUsage {
                        input_tokens: self.input_tokens,
                        cached_input_tokens: 0,
                        output_tokens: self.output_tokens,
                        reasoning_output_tokens: 0,
                        total_tokens: self.input_tokens + self.output_tokens,
                    }),
                    end_turn: Some(!self.emitted_tool_call),
                });
                Ok(events)
            }
            AnthropicStreamEvent::Error { error } => Err(ApiError::Stream(format!(
                "Claude stream error {}: {}",
                error.kind, error.message
            ))),
            AnthropicStreamEvent::Other => Ok(Vec::new()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicStreamEvent {
    MessageStart {
        message: AnthropicMessageStart,
    },
    ContentBlockStart {
        index: i64,
        content_block: AnthropicContentBlock,
    },
    ContentBlockDelta {
        #[serde(default)]
        index: i64,
        delta: AnthropicDelta,
    },
    ContentBlockStop {
        index: i64,
    },
    MessageDelta {
        usage: Option<AnthropicUsage>,
    },
    MessageStop,
    Error {
        error: AnthropicError,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicContentBlock {
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
struct AnthropicMessageStart {
    id: String,
    usage: Option<AnthropicUsage>,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: Option<i64>,
    output_tokens: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicDelta {
    TextDelta {
        text: String,
    },
    InputJsonDelta {
        partial_json: String,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug)]
struct StreamingToolUse {
    id: String,
    name: String,
    arguments: String,
}

impl StreamingToolUse {
    fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            arguments: String::new(),
        }
    }

    fn to_item(&self) -> ResponseItem {
        ResponseItem::FunctionCall {
            id: Some(self.id.clone()),
            name: self.name.clone(),
            namespace: None,
            arguments: self.arguments.clone(),
            call_id: self.id.clone(),
        }
    }

    fn to_delta(&self, delta: String) -> ResponseEvent {
        ResponseEvent::ToolCallInputDelta {
            item_id: self.id.clone(),
            call_id: Some(self.id.clone()),
            delta,
        }
    }

    fn to_done_item(&self) -> std::result::Result<ResponseItem, ApiError> {
        let arguments = if self.arguments.is_empty() {
            "{}".to_string()
        } else {
            let input: Value = serde_json::from_str(&self.arguments).map_err(|err| {
                ApiError::Stream(format!("failed to parse Claude tool_use input: {err}"))
            })?;
            if !input.is_object() {
                return Err(ApiError::Stream(
                    "Claude tool_use input must be a JSON object".to_string(),
                ));
            }
            serde_json::to_string(&input).map_err(|err| {
                ApiError::Stream(format!("failed to serialize Claude tool_use input: {err}"))
            })?
        };

        Ok(ResponseItem::FunctionCall {
            id: Some(self.id.clone()),
            name: self.name.clone(),
            namespace: None,
            arguments,
            call_id: self.id.clone(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct AnthropicError {
    #[serde(rename = "type")]
    kind: String,
    message: String,
}

fn unsupported(message: impl Into<String>) -> CodexErr {
    CodexErr::UnsupportedOperation(message.into())
}

#[cfg(test)]
#[path = "anthropic_tests.rs"]
mod tests;
