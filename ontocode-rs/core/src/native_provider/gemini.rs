use crate::client_common::Prompt;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use http::HeaderMap;
use http::HeaderValue;
use ontocode_api::ApiError;
use ontocode_api::ResponseEvent;
use ontocode_api::ResponseStream as ApiResponseStream;
use ontocode_api::SharedAuthProvider;
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
use serde_json::json;
use std::collections::BTreeMap;
use tokio::sync::mpsc;
use tokio::time::timeout;
use tracing::trace;

const GEMINI_DEFAULT_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeminiGenerateContentRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiSystemInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<GeminiTool>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiSystemInstruction {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_call: Option<GeminiFunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_response: Option<GeminiFunctionResponse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GeminiFunctionCall {
    name: String,
    args: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GeminiFunctionResponse {
    name: String,
    response: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiTool {
    function_declarations: Vec<GeminiFunctionDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct GeminiFunctionDeclaration {
    name: String,
    description: String,
    parameters: JsonSchema,
}

pub(crate) fn build_generate_content_request(
    prompt: &Prompt,
    model_info: &ModelInfo,
) -> Result<GeminiGenerateContentRequest> {
    let _ = model_info;
    if prompt.parallel_tool_calls {
        return Err(unsupported(
            "Gemini native runtime does not support parallel tool calls",
        ));
    }
    if prompt.output_schema.is_some() {
        return Err(unsupported(
            "Gemini native runtime does not support output schemas yet",
        ));
    }

    let tools = gemini_tools(&prompt.tools)?;
    let mut system_parts = Vec::new();
    if !prompt.base_instructions.text.trim().is_empty() {
        system_parts.push(prompt.base_instructions.text.clone());
    }

    let mut tool_call_names = BTreeMap::new();
    let mut contents = Vec::new();
    for item in prompt.get_formatted_input() {
        match item {
            ResponseItem::Message { role, content, .. } => {
                let text = text_content(content)?;
                match role.as_str() {
                    "user" => contents.push(text_content_message("user", text)),
                    "assistant" => contents.push(text_content_message("model", text)),
                    "system" | "developer" => system_parts.push(text),
                    other => {
                        return Err(unsupported(format!(
                            "Gemini native runtime does not support message role `{other}`"
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
                        "Gemini native runtime does not support namespaced tool-call history yet",
                    ));
                }
                let args = parse_tool_arguments(&arguments)?;
                tool_call_names.insert(call_id, name.clone());
                contents.push(GeminiContent {
                    role: "model".to_string(),
                    parts: vec![GeminiPart {
                        text: None,
                        function_call: Some(GeminiFunctionCall { name, args }),
                        function_response: None,
                    }],
                });
            }
            ResponseItem::FunctionCallOutput { call_id, output } => {
                let Some(name) = tool_call_names.get(&call_id) else {
                    return Err(unsupported(
                        "Gemini native runtime cannot translate tool result without prior tool call",
                    ));
                };
                contents.push(GeminiContent {
                    role: "user".to_string(),
                    parts: vec![GeminiPart {
                        text: None,
                        function_call: None,
                        function_response: Some(GeminiFunctionResponse {
                            name: name.clone(),
                            response: json!({ "content": tool_result_content(output)? }),
                        }),
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
                    "Gemini native runtime does not support this tool or non-text history yet",
                ));
            }
        }
    }

    Ok(GeminiGenerateContentRequest {
        contents,
        system_instruction: (!system_parts.is_empty()).then(|| GeminiSystemInstruction {
            parts: vec![GeminiPart {
                text: Some(system_parts.join("\n\n")),
                function_call: None,
                function_response: None,
            }],
        }),
        tools,
    })
}

pub(crate) async fn stream_generate_content(
    client: Client,
    provider_info: &ModelProviderInfo,
    model_info: &ModelInfo,
    api_auth: Option<SharedAuthProvider>,
    request: GeminiGenerateContentRequest,
    mut extra_headers: HeaderMap,
) -> Result<ApiResponseStream> {
    let api_key = provider_info
        .env_key
        .as_deref()
        .and_then(|env_key| std::env::var(env_key).ok())
        .filter(|api_key| !api_key.trim().is_empty());
    let url = gemini_url(provider_info, model_info)?;
    add_configured_headers(provider_info, &mut extra_headers);
    extra_headers.insert(
        http::header::ACCEPT,
        HeaderValue::from_static("text/event-stream"),
    );
    extra_headers.insert(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    if let Some(api_key) = api_key {
        extra_headers.insert(
            "x-goog-api-key",
            HeaderValue::from_str(&api_key).map_err(|err| {
                CodexErr::UnsupportedOperation(format!("invalid Gemini API key: {err}"))
            })?,
        );
    } else {
        let Some(api_auth) = api_auth else {
            return Err(CodexErr::UnsupportedOperation(
                "Gemini native runtime requires either GEMINI_API_KEY or provider bearer auth"
                    .to_string(),
            ));
        };
        api_auth.add_auth_headers(&mut extra_headers);
        if !extra_headers.contains_key(http::header::AUTHORIZATION) {
            return Err(CodexErr::UnsupportedOperation(
                "Gemini native runtime requires either GEMINI_API_KEY or provider bearer auth"
                    .to_string(),
            ));
        }
    }

    let response = client
        .post(url)
        .headers(extra_headers)
        .json(&request)
        .send()
        .await
        .map_err(|err| CodexErr::UnsupportedOperation(format!("Gemini request failed: {err}")))?;

    let request_id = response
        .headers()
        .get("x-request-id")
        .or_else(|| response.headers().get("x-guploader-uploadid"))
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let status = response.status();
    if !status.is_success() {
        let message = response
            .text()
            .await
            .unwrap_or_else(|err| format!("failed to read Gemini error response: {err}"));
        return Err(CodexErr::UnsupportedOperation(format!(
            "Gemini request failed with status {status}: {message}"
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

fn text_content_message(role: &str, text: String) -> GeminiContent {
    GeminiContent {
        role: role.to_string(),
        parts: vec![GeminiPart {
            text: Some(text),
            function_call: None,
            function_response: None,
        }],
    }
}

fn gemini_tools(tools: &[ToolSpec]) -> Result<Option<Vec<GeminiTool>>> {
    if tools.is_empty() {
        return Ok(None);
    }

    let mut declarations = Vec::with_capacity(tools.len());
    for tool in tools {
        match tool {
            ToolSpec::Function(tool) => declarations.push(gemini_function_declaration(tool)),
            ToolSpec::Namespace(_) => {}
            ToolSpec::ToolSearch { .. }
            | ToolSpec::ImageGeneration { .. }
            | ToolSpec::WebSearch { .. }
            | ToolSpec::Freeform(_) => {
                return Err(unsupported(format!(
                    "Gemini native runtime does not support `{}` tool declarations yet",
                    tool.name()
                )));
            }
        }
    }

    Ok((!declarations.is_empty()).then(|| {
        vec![GeminiTool {
            function_declarations: declarations,
        }]
    }))
}

fn gemini_function_declaration(tool: &ResponsesApiTool) -> GeminiFunctionDeclaration {
    GeminiFunctionDeclaration {
        name: tool.name.clone(),
        description: tool.description.clone(),
        parameters: tool.parameters.clone(),
    }
}

fn parse_tool_arguments(arguments: &str) -> Result<Value> {
    let input: Value = serde_json::from_str(arguments).map_err(|err| {
        unsupported(format!(
            "Gemini native runtime cannot translate invalid tool-call arguments: {err}"
        ))
    })?;
    if !input.is_object() {
        return Err(unsupported(
            "Gemini native runtime only supports object tool-call arguments",
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
                            "Gemini native runtime does not support image tool results yet",
                        ));
                    }
                    FunctionCallOutputContentItem::EncryptedContent { .. } => {
                        return Err(unsupported(
                            "Gemini native runtime does not support encrypted tool results",
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
                    "Gemini native runtime does not support multimodal input yet",
                ));
            }
        }
    }
    Ok(parts.join("\n"))
}

fn gemini_url(provider_info: &ModelProviderInfo, model_info: &ModelInfo) -> Result<String> {
    let base_url = provider_info
        .base_url
        .as_deref()
        .unwrap_or(GEMINI_DEFAULT_BASE_URL)
        .trim_end_matches('/');
    let model_path = if model_info.slug.starts_with("models/") {
        format!("{}:streamGenerateContent", model_info.slug)
    } else {
        format!("models/{}:streamGenerateContent", model_info.slug)
    };
    let mut url = url::Url::parse(&format!("{base_url}/{model_path}"))
        .map_err(|err| CodexErr::UnsupportedOperation(format!("invalid Gemini base_url: {err}")))?;
    url.query_pairs_mut().append_pair("alt", "sse");
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
    let mut state = GeminiStreamState::default();
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
                        "Gemini stream closed before completion".to_string(),
                    )))
                    .await;
                return;
            }
            Err(_) => {
                let _ = tx_event
                    .send(Err(ApiError::Stream(
                        "idle timeout waiting for Gemini SSE".to_string(),
                    )))
                    .await;
                return;
            }
        };

        trace!("Gemini SSE event: {}", event.data);
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
struct GeminiStreamState {
    response_id: Option<String>,
    text: String,
    emitted_created: bool,
    emitted_tool_call: bool,
    next_tool_call_index: usize,
    input_tokens: i64,
    output_tokens: i64,
    reasoning_output_tokens: i64,
    total_tokens: i64,
}

impl GeminiStreamState {
    fn process_data(&mut self, data: &str) -> std::result::Result<Vec<ResponseEvent>, ApiError> {
        if data.trim().is_empty() || data.trim() == "[DONE]" {
            return Ok(Vec::new());
        }

        let chunk: GeminiStreamChunk = serde_json::from_str(data).map_err(|err| {
            ApiError::Stream(format!("failed to parse Gemini stream event: {err}"))
        })?;
        if let Some(error) = chunk.error {
            return Err(ApiError::Stream(format!(
                "Gemini stream error {}: {}",
                error.status.unwrap_or_else(|| error.code.to_string()),
                error.message
            )));
        }
        if let Some(response_id) = chunk.response_id {
            self.response_id = Some(response_id);
        }
        if let Some(usage) = chunk.usage_metadata {
            self.input_tokens = usage.prompt_token_count.unwrap_or(self.input_tokens);
            self.output_tokens = usage.candidates_token_count.unwrap_or(self.output_tokens);
            self.reasoning_output_tokens = usage.thoughts_token_count.unwrap_or(0);
            self.total_tokens = usage.total_token_count.unwrap_or_else(|| {
                self.input_tokens + self.output_tokens + self.reasoning_output_tokens
            });
        }

        let mut events = Vec::new();
        if !self.emitted_created {
            self.emitted_created = true;
            events.push(ResponseEvent::Created);
        }

        let mut completed = false;
        for candidate in chunk.candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        self.text.push_str(&text);
                        events.push(ResponseEvent::OutputTextDelta(text));
                    }
                    if let Some(function_call) = part.function_call {
                        events.extend(self.function_call_events(function_call)?);
                    }
                }
            }
            if candidate.finish_reason.is_some() {
                completed = true;
            }
        }

        if completed {
            let response_id = self
                .response_id
                .clone()
                .unwrap_or_else(|| "gemini-response".to_string());
            if !self.text.is_empty() || !self.emitted_tool_call {
                events.push(ResponseEvent::OutputItemDone(ResponseItem::Message {
                    id: Some(response_id.clone()),
                    role: "assistant".to_string(),
                    content: vec![ContentItem::OutputText {
                        text: self.text.clone(),
                    }],
                    phase: None,
                }));
            }
            events.push(ResponseEvent::Completed {
                response_id,
                token_usage: Some(TokenUsage {
                    input_tokens: self.input_tokens,
                    cached_input_tokens: 0,
                    output_tokens: self.output_tokens,
                    reasoning_output_tokens: self.reasoning_output_tokens,
                    total_tokens: self.total_tokens,
                }),
                end_turn: Some(!self.emitted_tool_call),
            });
        }

        Ok(events)
    }

    fn function_call_events(
        &mut self,
        function_call: GeminiFunctionCall,
    ) -> std::result::Result<Vec<ResponseEvent>, ApiError> {
        if !function_call.args.is_object() {
            return Err(ApiError::Stream(
                "Gemini functionCall args must be a JSON object".to_string(),
            ));
        }
        let call_id = format!("gemini-call-{}", self.next_tool_call_index);
        self.next_tool_call_index += 1;
        let arguments = serde_json::to_string(&function_call.args).map_err(|err| {
            ApiError::Stream(format!(
                "failed to serialize Gemini functionCall args: {err}"
            ))
        })?;
        self.emitted_tool_call = true;

        let added_item = ResponseItem::FunctionCall {
            id: Some(call_id.clone()),
            name: function_call.name.clone(),
            namespace: None,
            arguments: String::new(),
            call_id: call_id.clone(),
        };
        let done_item = ResponseItem::FunctionCall {
            id: Some(call_id.clone()),
            name: function_call.name,
            namespace: None,
            arguments: arguments.clone(),
            call_id: call_id.clone(),
        };

        let mut events = vec![ResponseEvent::OutputItemAdded(added_item)];
        if !arguments.is_empty() {
            events.push(ResponseEvent::ToolCallInputDelta {
                item_id: call_id.clone(),
                call_id: Some(call_id),
                delta: arguments,
            });
        }
        events.push(ResponseEvent::OutputItemDone(done_item));
        Ok(events)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiStreamChunk {
    #[serde(default)]
    candidates: Vec<GeminiCandidate>,
    usage_metadata: Option<GeminiUsageMetadata>,
    response_id: Option<String>,
    error: Option<GeminiError>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiCandidate {
    content: Option<GeminiContent>,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiUsageMetadata {
    prompt_token_count: Option<i64>,
    candidates_token_count: Option<i64>,
    thoughts_token_count: Option<i64>,
    total_token_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct GeminiError {
    code: i64,
    message: String,
    status: Option<String>,
}

fn unsupported(message: impl Into<String>) -> CodexErr {
    CodexErr::UnsupportedOperation(message.into())
}

#[cfg(test)]
#[path = "gemini_tests.rs"]
mod tests;
