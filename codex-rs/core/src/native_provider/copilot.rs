use crate::client_common::Prompt;
use codex_api::ApiError;
use codex_api::ResponseEvent;
use codex_api::ResponseStream as ApiResponseStream;
use codex_model_provider_info::ModelProviderInfo;
use codex_protocol::error::CodexErr;
use codex_protocol::error::Result;
use codex_protocol::models::ContentItem;
use codex_protocol::models::FunctionCallOutputBody;
use codex_protocol::models::FunctionCallOutputContentItem;
use codex_protocol::models::FunctionCallOutputPayload;
use codex_protocol::models::ResponseItem;
use codex_protocol::openai_models::ModelInfo;
use codex_protocol::protocol::TokenUsage;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use http::HeaderMap;
use http::HeaderValue;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use tokio::sync::mpsc;
use tokio::time::timeout;
use tracing::trace;

const COPILOT_DEFAULT_BASE_URL: &str = "https://api.githubcopilot.com";
const COPILOT_TOKEN_ENDPOINT: &str = "https://api.github.com/copilot_internal/v2/token";
const COPILOT_USER_AGENT: &str = "Codex/0.0.0";
const COPILOT_EDITOR_VERSION: &str = "codex/0.0.0";
const COPILOT_EDITOR_PLUGIN_VERSION: &str = "codex/0.0.0";
const COPILOT_INTEGRATION_ID: &str = "vscode-chat";

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct CopilotChatCompletionsRequest {
    model: String,
    messages: Vec<CopilotChatMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<CopilotTool>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct CopilotChatMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<CopilotToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct CopilotToolCall {
    id: String,
    #[serde(rename = "type")]
    tool_type: &'static str,
    function: CopilotToolCallFunction,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct CopilotToolCallFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct CopilotTool {
    #[serde(rename = "type")]
    tool_type: &'static str,
    function: CopilotToolFunction,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct CopilotToolFunction {
    name: String,
    description: String,
    parameters: JsonSchema,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CopilotToken {
    token: String,
    expires_at: Option<i64>,
}

pub(crate) fn build_chat_completions_request(
    prompt: &Prompt,
    model_info: &ModelInfo,
) -> Result<CopilotChatCompletionsRequest> {
    if prompt.parallel_tool_calls {
        return Err(unsupported(
            "Copilot native runtime does not support parallel tool calls",
        ));
    }
    if prompt.output_schema.is_some() {
        return Err(unsupported(
            "Copilot native runtime does not support output schemas yet",
        ));
    }

    let tools = copilot_tools(&prompt.tools)?;
    let mut messages = Vec::new();
    if !prompt.base_instructions.text.trim().is_empty() {
        messages.push(text_message(
            "system",
            prompt.base_instructions.text.clone(),
        ));
    }

    for item in prompt.get_formatted_input() {
        match item {
            ResponseItem::Message { role, content, .. } => {
                let text = text_content(content)?;
                match role.as_str() {
                    "user" | "assistant" | "system" | "developer" => {
                        messages.push(text_message(&role, text));
                    }
                    other => {
                        return Err(unsupported(format!(
                            "Copilot native runtime does not support message role `{other}`"
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
                        "Copilot native runtime does not support namespaced tool-call history yet",
                    ));
                }
                let arguments = parse_tool_arguments(&arguments)?;
                messages.push(CopilotChatMessage {
                    role: "assistant".to_string(),
                    content: None,
                    tool_calls: Some(vec![CopilotToolCall {
                        id: call_id,
                        tool_type: "function",
                        function: CopilotToolCallFunction { name, arguments },
                    }]),
                    tool_call_id: None,
                });
            }
            ResponseItem::FunctionCallOutput { call_id, output } => {
                messages.push(CopilotChatMessage {
                    role: "tool".to_string(),
                    content: Some(tool_result_content(output)?),
                    tool_calls: None,
                    tool_call_id: Some(call_id),
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
                    "Copilot native runtime does not support this tool or non-text history yet",
                ));
            }
        }
    }

    Ok(CopilotChatCompletionsRequest {
        model: model_info.slug.clone(),
        messages,
        stream: true,
        tools,
    })
}

pub(crate) async fn stream_chat_completions(
    client: Client,
    provider_info: &ModelProviderInfo,
    request: CopilotChatCompletionsRequest,
    mut extra_headers: HeaderMap,
) -> Result<ApiResponseStream> {
    let github_token = copilot_github_token(provider_info)?;
    let copilot_token =
        exchange_github_token(client.clone(), COPILOT_TOKEN_ENDPOINT, &github_token)
            .await?
            .token;
    let url = copilot_chat_url(provider_info)?;
    add_configured_headers(provider_info, &mut extra_headers);
    add_copilot_headers(&mut extra_headers);
    extra_headers.insert(
        http::header::ACCEPT,
        HeaderValue::from_static("text/event-stream"),
    );
    extra_headers.insert(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    extra_headers.insert(
        http::header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {copilot_token}")).map_err(|err| {
            CodexErr::UnsupportedOperation(format!("invalid Copilot bearer token: {err}"))
        })?,
    );

    let response = client
        .post(url)
        .headers(extra_headers)
        .json(&request)
        .send()
        .await
        .map_err(|err| CodexErr::UnsupportedOperation(format!("Copilot request failed: {err}")))?;

    let request_id = response
        .headers()
        .get("x-request-id")
        .or_else(|| response.headers().get("github-request-id"))
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let status = response.status();
    if !status.is_success() {
        let message = response
            .text()
            .await
            .unwrap_or_else(|err| format!("failed to read Copilot error response: {err}"));
        return Err(CodexErr::UnsupportedOperation(format!(
            "Copilot request failed with status {status}: {message}"
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

pub(crate) async fn exchange_github_token(
    client: Client,
    token_endpoint: &str,
    github_bearer_token: &str,
) -> Result<CopilotToken> {
    if github_bearer_token.trim().is_empty() {
        return Err(unsupported(
            "Copilot native runtime requires a non-empty GitHub bearer token",
        ));
    }

    let mut headers = HeaderMap::new();
    add_copilot_headers(&mut headers);
    headers.insert(
        http::header::ACCEPT,
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        http::header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", github_bearer_token.trim())).map_err(
            |err| CodexErr::UnsupportedOperation(format!("invalid GitHub bearer token: {err}")),
        )?,
    );

    let response = client
        .get(token_endpoint)
        .headers(headers)
        .send()
        .await
        .map_err(|err| {
            CodexErr::UnsupportedOperation(format!("Copilot token exchange failed: {err}"))
        })?;
    let status = response.status();
    if !status.is_success() {
        let message = response
            .text()
            .await
            .unwrap_or_else(|err| format!("failed to read Copilot token error response: {err}"));
        return Err(CodexErr::UnsupportedOperation(format!(
            "Copilot token exchange failed with status {status}: {message}"
        )));
    }

    let body: CopilotTokenResponse = response.json().await.map_err(|err| {
        CodexErr::UnsupportedOperation(format!("failed to parse Copilot token response: {err}"))
    })?;
    if body.token.trim().is_empty() {
        return Err(unsupported(
            "Copilot token exchange returned an empty token",
        ));
    }

    Ok(CopilotToken {
        token: body.token,
        expires_at: body.expires_at,
    })
}

fn text_message(role: &str, content: String) -> CopilotChatMessage {
    CopilotChatMessage {
        role: role.to_string(),
        content: Some(content),
        tool_calls: None,
        tool_call_id: None,
    }
}

fn copilot_tools(tools: &[ToolSpec]) -> Result<Option<Vec<CopilotTool>>> {
    if tools.is_empty() {
        return Ok(None);
    }

    let mut declarations = Vec::with_capacity(tools.len());
    for tool in tools {
        match tool {
            ToolSpec::Function(tool) => declarations.push(copilot_function_tool(tool)),
            ToolSpec::Namespace(_)
            | ToolSpec::ToolSearch { .. }
            | ToolSpec::ImageGeneration { .. }
            | ToolSpec::WebSearch { .. }
            | ToolSpec::Freeform(_) => {
                return Err(unsupported(format!(
                    "Copilot native runtime does not support `{}` tool declarations yet",
                    tool.name()
                )));
            }
        }
    }

    Ok(Some(declarations))
}

fn copilot_function_tool(tool: &ResponsesApiTool) -> CopilotTool {
    CopilotTool {
        tool_type: "function",
        function: CopilotToolFunction {
            name: tool.name.clone(),
            description: tool.description.clone(),
            parameters: tool.parameters.clone(),
        },
    }
}

fn parse_tool_arguments(arguments: &str) -> Result<String> {
    let input: Value = serde_json::from_str(arguments).map_err(|err| {
        unsupported(format!(
            "Copilot native runtime cannot translate invalid tool-call arguments: {err}"
        ))
    })?;
    if !input.is_object() {
        return Err(unsupported(
            "Copilot native runtime only supports object tool-call arguments",
        ));
    }
    serde_json::to_string(&input).map_err(|err| {
        unsupported(format!(
            "Copilot native runtime cannot serialize tool-call arguments: {err}"
        ))
    })
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
                            "Copilot native runtime does not support image tool results yet",
                        ));
                    }
                    FunctionCallOutputContentItem::EncryptedContent { .. } => {
                        return Err(unsupported(
                            "Copilot native runtime does not support encrypted tool results",
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
                    "Copilot native runtime does not support multimodal input yet",
                ));
            }
        }
    }
    Ok(parts.join("\n"))
}

fn copilot_github_token(provider_info: &ModelProviderInfo) -> Result<String> {
    provider_info
        .env_key
        .as_deref()
        .and_then(|env_key| std::env::var(env_key).ok())
        .filter(|token| !token.trim().is_empty())
        .ok_or_else(|| {
            CodexErr::UnsupportedOperation(
                "Copilot native runtime requires provider env_key to contain a GitHub OAuth/access token"
                    .to_string(),
            )
        })
}

fn copilot_chat_url(provider_info: &ModelProviderInfo) -> Result<String> {
    let base_url = provider_info
        .base_url
        .as_deref()
        .unwrap_or(COPILOT_DEFAULT_BASE_URL)
        .trim_end_matches('/');
    let mut url = if base_url.ends_with("/chat/completions") {
        url::Url::parse(base_url)
    } else {
        url::Url::parse(&format!("{base_url}/chat/completions"))
    }
    .map_err(|err| CodexErr::UnsupportedOperation(format!("invalid Copilot base_url: {err}")))?;
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

fn add_copilot_headers(headers: &mut HeaderMap) {
    headers.insert(
        http::header::USER_AGENT,
        HeaderValue::from_static(COPILOT_USER_AGENT),
    );
    headers.insert(
        "editor-version",
        HeaderValue::from_static(COPILOT_EDITOR_VERSION),
    );
    headers.insert(
        "editor-plugin-version",
        HeaderValue::from_static(COPILOT_EDITOR_PLUGIN_VERSION),
    );
    headers.insert(
        "copilot-integration-id",
        HeaderValue::from_static(COPILOT_INTEGRATION_ID),
    );
    headers.insert(
        "openai-intent",
        HeaderValue::from_static("conversation-panel"),
    );
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
    let mut state = CopilotStreamState::default();
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
                        "Copilot stream closed before completion".to_string(),
                    )))
                    .await;
                return;
            }
            Err(_) => {
                let _ = tx_event
                    .send(Err(ApiError::Stream(
                        "idle timeout waiting for Copilot SSE".to_string(),
                    )))
                    .await;
                return;
            }
        };

        trace!("Copilot SSE event: {}", event.data);
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
struct CopilotStreamState {
    response_id: Option<String>,
    text: String,
    emitted_created: bool,
    completed: bool,
    tool_calls: BTreeMap<usize, StreamingToolCall>,
    input_tokens: i64,
    output_tokens: i64,
    total_tokens: i64,
}

impl CopilotStreamState {
    fn process_data(&mut self, data: &str) -> std::result::Result<Vec<ResponseEvent>, ApiError> {
        if self.completed || data.trim().is_empty() {
            return Ok(Vec::new());
        }
        if data.trim() == "[DONE]" {
            return Ok(self.complete_events());
        }

        let chunk: CopilotStreamChunk = serde_json::from_str(data).map_err(|err| {
            ApiError::Stream(format!("failed to parse Copilot stream event: {err}"))
        })?;
        if let Some(error) = chunk.error {
            return Err(ApiError::Stream(format!(
                "Copilot stream error {}: {}",
                error.code.unwrap_or_else(|| "unknown".to_string()),
                error.message
            )));
        }
        if let Some(id) = chunk.id {
            self.response_id = Some(id);
        }
        if let Some(usage) = chunk.usage {
            self.input_tokens = usage.prompt_tokens.unwrap_or(self.input_tokens);
            self.output_tokens = usage.completion_tokens.unwrap_or(self.output_tokens);
            self.total_tokens = usage
                .total_tokens
                .unwrap_or_else(|| self.input_tokens + self.output_tokens);
        }

        let mut events = Vec::new();
        if !self.emitted_created {
            self.emitted_created = true;
            events.push(ResponseEvent::Created);
        }

        let mut should_complete = false;
        for choice in chunk.choices {
            if let Some(delta) = choice.delta {
                if let Some(content) = delta.content {
                    self.text.push_str(&content);
                    events.push(ResponseEvent::OutputTextDelta(content));
                }
                for tool_call in delta.tool_calls {
                    events.extend(self.tool_call_delta_events(tool_call)?);
                }
            }
            if choice.finish_reason.is_some() {
                should_complete = true;
            }
        }

        if should_complete {
            events.extend(self.complete_events());
        }

        Ok(events)
    }

    fn tool_call_delta_events(
        &mut self,
        delta: CopilotStreamToolCallDelta,
    ) -> std::result::Result<Vec<ResponseEvent>, ApiError> {
        let index = delta.index;
        let call = self
            .tool_calls
            .entry(index)
            .or_insert_with(|| StreamingToolCall {
                id: format!("copilot-call-{index}"),
                name: String::new(),
                arguments: String::new(),
                emitted_added: false,
            });
        if let Some(id) = delta.id {
            call.id = id;
        }
        let mut argument_delta = None;
        if let Some(function) = delta.function {
            if let Some(name) = function.name {
                call.name = name;
            }
            if let Some(arguments) = function.arguments {
                call.arguments.push_str(&arguments);
                argument_delta = Some(arguments);
            }
        }

        let mut events = Vec::new();
        if !call.emitted_added && !call.name.is_empty() {
            call.emitted_added = true;
            events.push(ResponseEvent::OutputItemAdded(ResponseItem::FunctionCall {
                id: Some(call.id.clone()),
                name: call.name.clone(),
                namespace: None,
                arguments: String::new(),
                call_id: call.id.clone(),
            }));
        }
        if let Some(arguments) = argument_delta
            && !arguments.is_empty()
        {
            events.push(ResponseEvent::ToolCallInputDelta {
                item_id: call.id.clone(),
                call_id: Some(call.id.clone()),
                delta: arguments,
            });
        }
        Ok(events)
    }

    fn complete_events(&mut self) -> Vec<ResponseEvent> {
        if self.completed {
            return Vec::new();
        }
        self.completed = true;
        let response_id = self
            .response_id
            .clone()
            .unwrap_or_else(|| "copilot-response".to_string());
        let mut events = Vec::new();
        if !self.text.is_empty() || self.tool_calls.is_empty() {
            events.push(ResponseEvent::OutputItemDone(ResponseItem::Message {
                id: Some(response_id.clone()),
                role: "assistant".to_string(),
                content: vec![ContentItem::OutputText {
                    text: self.text.clone(),
                }],
                phase: None,
            }));
        }
        for call in self.tool_calls.values() {
            events.push(ResponseEvent::OutputItemDone(ResponseItem::FunctionCall {
                id: Some(call.id.clone()),
                name: call.name.clone(),
                namespace: None,
                arguments: call.arguments.clone(),
                call_id: call.id.clone(),
            }));
        }
        events.push(ResponseEvent::Completed {
            response_id,
            token_usage: Some(TokenUsage {
                input_tokens: self.input_tokens,
                cached_input_tokens: 0,
                output_tokens: self.output_tokens,
                reasoning_output_tokens: 0,
                total_tokens: self.total_tokens,
            }),
            end_turn: Some(self.tool_calls.is_empty()),
        });
        events
    }
}

#[derive(Debug, Default)]
struct StreamingToolCall {
    id: String,
    name: String,
    arguments: String,
    emitted_added: bool,
}

#[derive(Debug, Deserialize)]
struct CopilotTokenResponse {
    token: String,
    expires_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct CopilotStreamChunk {
    id: Option<String>,
    #[serde(default)]
    choices: Vec<CopilotStreamChoice>,
    usage: Option<CopilotUsage>,
    error: Option<CopilotError>,
}

#[derive(Debug, Deserialize)]
struct CopilotStreamChoice {
    delta: Option<CopilotStreamDelta>,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CopilotStreamDelta {
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<CopilotStreamToolCallDelta>,
}

#[derive(Debug, Deserialize)]
struct CopilotStreamToolCallDelta {
    index: usize,
    id: Option<String>,
    function: Option<CopilotStreamFunctionDelta>,
}

#[derive(Debug, Deserialize)]
struct CopilotStreamFunctionDelta {
    name: Option<String>,
    arguments: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CopilotUsage {
    prompt_tokens: Option<i64>,
    completion_tokens: Option<i64>,
    total_tokens: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct CopilotError {
    message: String,
    code: Option<String>,
}

fn unsupported(message: impl Into<String>) -> CodexErr {
    CodexErr::UnsupportedOperation(message.into())
}

#[cfg(test)]
#[path = "copilot_tests.rs"]
mod tests;
