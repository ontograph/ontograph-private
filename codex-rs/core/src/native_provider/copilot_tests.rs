use super::*;
use codex_protocol::models::BaseInstructions;
use codex_protocol::models::ContentItem;
use codex_protocol::models::FunctionCallOutputContentItem;
use codex_protocol::models::FunctionCallOutputPayload;
use codex_protocol::models::ResponseItem;
use codex_protocol::openai_models::ModelInfo;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::collections::BTreeMap;
use wiremock::Mock;
use wiremock::MockServer;
use wiremock::ResponseTemplate;
use wiremock::matchers::header;
use wiremock::matchers::method;
use wiremock::matchers::path;

fn model_info() -> ModelInfo {
    serde_json::from_value(json!({
        "slug": "gpt-4o-copilot",
        "display_name": "GPT-4o Copilot",
        "description": null,
        "default_reasoning_level": "medium",
        "supported_reasoning_levels": [],
        "shell_type": "shell_command",
        "visibility": "list",
        "supported_in_api": true,
        "priority": 0,
        "upgrade": null,
        "base_instructions": "base instructions",
        "supports_reasoning_summaries": false,
        "support_verbosity": false,
        "default_verbosity": null,
        "apply_patch_tool_type": null,
        "truncation_policy": {"mode": "bytes", "limit": 10000},
        "supports_parallel_tool_calls": false,
        "supports_image_detail_original": false,
        "context_window": 128000,
        "max_context_window": 128000,
        "experimental_supported_tools": []
    }))
    .expect("valid model")
}

fn lookup_tool() -> ToolSpec {
    ToolSpec::Function(ResponsesApiTool {
        name: "lookup".to_string(),
        description: "Look up a value.".to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            BTreeMap::from([(
                "query".to_string(),
                JsonSchema::string(Some("Search query.".to_string())),
            )]),
            Some(vec!["query".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    })
}

#[test]
fn builds_basic_chat_completions_request() {
    let prompt = Prompt {
        input: vec![
            ResponseItem::Message {
                id: None,
                role: "user".to_string(),
                content: vec![ContentItem::InputText {
                    text: "hello".to_string(),
                }],
                phase: None,
            },
            ResponseItem::Message {
                id: None,
                role: "assistant".to_string(),
                content: vec![ContentItem::OutputText {
                    text: "hi".to_string(),
                }],
                phase: None,
            },
        ],
        base_instructions: BaseInstructions {
            text: "answer briefly".to_string(),
        },
        ..Prompt::default()
    };

    let request =
        build_chat_completions_request(&prompt, &model_info()).expect("request should build");

    assert_eq!(
        serde_json::to_value(request).expect("request should serialize"),
        json!({
            "model": "gpt-4o-copilot",
            "messages": [
                {"role": "system", "content": "answer briefly"},
                {"role": "user", "content": "hello"},
                {"role": "assistant", "content": "hi"}
            ],
            "stream": true
        })
    );
}

#[test]
fn serializes_function_tool_declarations_and_history() {
    let prompt = Prompt {
        input: vec![
            ResponseItem::FunctionCall {
                id: None,
                name: "lookup".to_string(),
                namespace: None,
                arguments: r#"{"query":"weather"}"#.to_string(),
                call_id: "call_1".to_string(),
            },
            ResponseItem::FunctionCallOutput {
                call_id: "call_1".to_string(),
                output: FunctionCallOutputPayload::from_content_items(vec![
                    FunctionCallOutputContentItem::InputText {
                        text: "sunny".to_string(),
                    },
                    FunctionCallOutputContentItem::InputText {
                        text: "72F".to_string(),
                    },
                ]),
            },
        ],
        tools: vec![lookup_tool()],
        base_instructions: BaseInstructions {
            text: String::new(),
        },
        ..Prompt::default()
    };

    let request =
        build_chat_completions_request(&prompt, &model_info()).expect("request should build");

    assert_eq!(
        serde_json::to_value(request).expect("request should serialize"),
        json!({
            "model": "gpt-4o-copilot",
            "messages": [
                {
                    "role": "assistant",
                    "tool_calls": [{
                        "id": "call_1",
                        "type": "function",
                        "function": {
                            "name": "lookup",
                            "arguments": "{\"query\":\"weather\"}"
                        }
                    }]
                },
                {
                    "role": "tool",
                    "content": "sunny\n72F",
                    "tool_call_id": "call_1"
                }
            ],
            "stream": true,
            "tools": [{
                "type": "function",
                "function": {
                    "name": "lookup",
                    "description": "Look up a value.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "Search query."
                            }
                        },
                        "required": ["query"],
                        "additionalProperties": false
                    }
                }
            }]
        })
    );
}

#[test]
fn rejects_multimodal_input() {
    let prompt = Prompt {
        input: vec![ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputImage {
                image_url: "data:image/png;base64,abc".to_string(),
                detail: None,
            }],
            phase: None,
        }],
        ..Prompt::default()
    };

    let err = build_chat_completions_request(&prompt, &model_info()).expect_err("image fails");

    assert!(err.to_string().contains("multimodal input"));
}

#[tokio::test]
async fn exchanges_github_token_for_copilot_token() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/copilot_internal/v2/token"))
        .and(header("authorization", "Bearer github-token"))
        .and(header("copilot-integration-id", "vscode-chat"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "token": "copilot-token",
            "expires_at": 123
        })))
        .mount(&server)
        .await;

    let token = exchange_github_token(
        reqwest::Client::new(),
        &format!("{}/copilot_internal/v2/token", server.uri()),
        "github-token",
    )
    .await
    .expect("token exchange should succeed");

    assert_eq!(
        token,
        CopilotToken {
            token: "copilot-token".to_string(),
            expires_at: Some(123),
        }
    );
}

#[test]
fn translates_text_stream_to_response_events() {
    let mut state = CopilotStreamState::default();

    assert!(matches!(
        state
            .process_data(
                r#"{"id":"resp_1","choices":[{"delta":{"content":"hel"}}],"usage":{"prompt_tokens":3}}"#
            )
            .expect("chunk should parse")
            .as_slice(),
        [ResponseEvent::Created, ResponseEvent::OutputTextDelta(delta)] if delta == "hel"
    ));
    assert!(matches!(
        state
            .process_data(
                r#"{"choices":[{"delta":{"content":"lo"},"finish_reason":"stop"}],"usage":{"completion_tokens":2,"total_tokens":5}}"#
            )
            .expect("final chunk should parse")
            .as_slice(),
        [
            ResponseEvent::OutputTextDelta(delta),
            ResponseEvent::OutputItemDone(ResponseItem::Message { role, content, .. }),
            ResponseEvent::Completed {
                response_id,
                token_usage: Some(TokenUsage {
                    input_tokens: 3,
                    output_tokens: 2,
                    total_tokens: 5,
                    ..
                }),
                end_turn: Some(true),
            },
        ] if delta == "lo"
            && role == "assistant"
            && content == &vec![ContentItem::OutputText { text: "hello".to_string() }]
            && response_id == "resp_1"
    ));
}

#[test]
fn translates_tool_call_stream_to_response_events() {
    let mut state = CopilotStreamState::default();

    let events = state
        .process_data(
            r#"{"id":"resp_1","choices":[{"delta":{"tool_calls":[{"index":0,"id":"call_1","function":{"name":"lookup","arguments":"{\"query\""}}]}}]}"#,
        )
        .expect("tool call chunk should parse");
    assert!(matches!(&events[0], ResponseEvent::Created));
    assert!(matches!(
        &events[1],
        ResponseEvent::OutputItemAdded(ResponseItem::FunctionCall {
            id: Some(id),
            name,
            namespace: None,
            arguments,
            call_id,
        }) if id == "call_1" && name == "lookup" && arguments.is_empty() && call_id == "call_1"
    ));
    assert!(matches!(
        &events[2],
        ResponseEvent::ToolCallInputDelta {
            item_id,
            call_id: Some(call_id),
            delta,
        } if item_id == "call_1" && call_id == "call_1" && delta == "{\"query\""
    ));

    let events = state
        .process_data(
            r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"function":{"arguments":":\"weather\"}"}}]},"finish_reason":"tool_calls"}]}"#,
        )
        .expect("tool call final chunk should parse");

    assert!(matches!(
        &events[0],
        ResponseEvent::ToolCallInputDelta {
            item_id,
            call_id: Some(call_id),
            delta,
        } if item_id == "call_1" && call_id == "call_1" && delta == ":\"weather\"}"
    ));
    assert!(matches!(
        &events[1],
        ResponseEvent::OutputItemDone(ResponseItem::FunctionCall {
            id: Some(id),
            name,
            namespace: None,
            arguments,
            call_id,
        }) if id == "call_1"
            && name == "lookup"
            && arguments == "{\"query\":\"weather\"}"
            && call_id == "call_1"
    ));
    assert!(matches!(
        &events[2],
        ResponseEvent::Completed {
            response_id,
            end_turn: Some(false),
            ..
        } if response_id == "resp_1"
    ));
}

#[test]
fn translates_error_event_to_stream_error() {
    let mut state = CopilotStreamState::default();

    let err = state
        .process_data(r#"{"error":{"message":"bad token","code":"unauthorized"}}"#)
        .expect_err("error event should fail");

    assert!(err.to_string().contains("unauthorized"));
}
