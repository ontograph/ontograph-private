use super::*;
use ontocode_protocol::models::BaseInstructions;
use ontocode_protocol::models::ContentItem;
use ontocode_protocol::models::FunctionCallOutputContentItem;
use ontocode_protocol::models::FunctionCallOutputPayload;
use ontocode_protocol::models::ResponseItem;
use ontocode_protocol::openai_models::ModelInfo;
use ontocode_tools::JsonSchema;
use ontocode_tools::ResponsesApiTool;
use ontocode_tools::ToolSpec;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::collections::BTreeMap;

fn model_info() -> ModelInfo {
    serde_json::from_value(json!({
        "slug": "claude-sonnet-4-5",
        "display_name": "Claude Sonnet 4.5",
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
        "context_window": 200000,
        "max_context_window": 200000,
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
fn builds_basic_messages_request() {
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
            text: String::new(),
        },
        ..Prompt::default()
    };

    let request = build_messages_request(&prompt, &model_info()).expect("request should build");

    assert_eq!(
        serde_json::to_value(request).expect("request should serialize"),
        json!({
            "model": "claude-sonnet-4-5",
            "max_tokens": 4096,
            "stream": true,
            "messages": [
                {"role": "user", "content": [{"type": "text", "text": "hello"}]},
                {"role": "assistant", "content": [{"type": "text", "text": "hi"}]}
            ]
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

    let err = build_messages_request(&prompt, &model_info()).expect_err("image should fail");

    assert!(err.to_string().contains("multimodal input"));
}

#[test]
fn serializes_function_tool_declarations() {
    let prompt = Prompt {
        input: vec![ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: "lookup weather".to_string(),
            }],
            phase: None,
        }],
        tools: vec![lookup_tool()],
        base_instructions: BaseInstructions {
            text: String::new(),
        },
        ..Prompt::default()
    };

    let request = build_messages_request(&prompt, &model_info()).expect("request should build");

    assert_eq!(
        serde_json::to_value(request).expect("request should serialize"),
        json!({
            "model": "claude-sonnet-4-5",
            "max_tokens": 4096,
            "stream": true,
            "tools": [{
                "name": "lookup",
                "description": "Look up a value.",
                "input_schema": {
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
            }],
            "messages": [
                {
                    "role": "user",
                    "content": [{"type": "text", "text": "lookup weather"}]
                }
            ]
        })
    );
}

#[test]
fn translates_function_call_and_result_history() {
    let prompt = Prompt {
        input: vec![
            ResponseItem::FunctionCall {
                id: None,
                name: "lookup".to_string(),
                namespace: None,
                arguments: r#"{"query":"weather"}"#.to_string(),
                call_id: "toolu_1".to_string(),
            },
            ResponseItem::FunctionCallOutput {
                call_id: "toolu_1".to_string(),
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
        base_instructions: BaseInstructions {
            text: String::new(),
        },
        ..Prompt::default()
    };

    let request = build_messages_request(&prompt, &model_info()).expect("request should build");

    assert_eq!(
        serde_json::to_value(request).expect("request should serialize"),
        json!({
            "model": "claude-sonnet-4-5",
            "max_tokens": 4096,
            "stream": true,
            "messages": [
                {
                    "role": "assistant",
                    "content": [{
                        "type": "tool_use",
                        "id": "toolu_1",
                        "name": "lookup",
                        "input": {"query": "weather"}
                    }]
                },
                {
                    "role": "user",
                    "content": [{
                        "type": "tool_result",
                        "tool_use_id": "toolu_1",
                        "content": "sunny\n72F"
                    }]
                }
            ]
        })
    );
}

#[test]
fn rejects_non_object_tool_arguments() {
    let prompt = Prompt {
        input: vec![ResponseItem::FunctionCall {
            id: None,
            name: "shell".to_string(),
            namespace: None,
            arguments: "[]".to_string(),
            call_id: "call-1".to_string(),
        }],
        ..Prompt::default()
    };

    let err =
        build_messages_request(&prompt, &model_info()).expect_err("non-object args should fail");

    assert!(err.to_string().contains("object tool-call arguments"));
}

#[test]
fn translates_text_stream_to_response_events() {
    let mut state = AnthropicStreamState::default();

    assert!(matches!(
        state
            .process_data(
                r#"{"type":"message_start","message":{"id":"msg_1","usage":{"input_tokens":3,"output_tokens":1}}}"#
            )
            .expect("message_start should parse")
            .as_slice(),
        [ResponseEvent::Created]
    ));
    assert!(matches!(
        state
            .process_data(
                r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"hel"}}"#
            )
            .expect("delta should parse")
            .as_slice(),
        [ResponseEvent::OutputTextDelta(delta)] if delta == "hel"
    ));
    assert!(matches!(
        state
            .process_data(r#"{"type":"message_delta","usage":{"output_tokens":2}}"#)
            .expect("message_delta should parse")
            .as_slice(),
        []
    ));

    let events = state
        .process_data(r#"{"type":"message_stop"}"#)
        .expect("message_stop should parse");

    assert!(matches!(
        &events[0],
        ResponseEvent::OutputItemDone(ResponseItem::Message { role, content, .. })
            if role == "assistant"
                && content == &vec![ContentItem::OutputText { text: "hel".to_string() }]
    ));
    assert!(matches!(
        &events[1],
        ResponseEvent::Completed {
            response_id,
            token_usage: Some(TokenUsage {
                input_tokens: 3,
                output_tokens: 2,
                total_tokens: 5,
                ..
            }),
            end_turn: Some(true),
        } if response_id == "msg_1"
    ));
}

#[test]
fn translates_tool_use_stream_to_response_events() {
    let mut state = AnthropicStreamState::default();

    assert!(matches!(
        state
            .process_data(r#"{"type":"message_start","message":{"id":"msg_1"}}"#)
            .expect("message_start should parse")
            .as_slice(),
        [ResponseEvent::Created]
    ));

    let added = state
        .process_data(
            r#"{"type":"content_block_start","index":0,"content_block":{"type":"tool_use","id":"toolu_1","name":"lookup","input":{}}}"#,
        )
        .expect("tool_use start should parse");
    assert!(matches!(
        &added[0],
        ResponseEvent::OutputItemAdded(ResponseItem::FunctionCall {
            id: Some(id),
            name,
            namespace: None,
            arguments,
            call_id,
        }) if id == "toolu_1" && name == "lookup" && arguments.is_empty() && call_id == "toolu_1"
    ));

    assert!(matches!(
        state
            .process_data(
                r#"{"type":"content_block_delta","index":0,"delta":{"type":"input_json_delta","partial_json":"{\"query\":\"wea"}}"#
            )
            .expect("first input delta should parse")
            .as_slice(),
        [ResponseEvent::ToolCallInputDelta {
            item_id,
            call_id: Some(call_id),
            delta,
        }] if item_id == "toolu_1" && call_id == "toolu_1" && delta == "{\"query\":\"wea"
    ));
    assert!(matches!(
        state
            .process_data(
                r#"{"type":"content_block_delta","index":0,"delta":{"type":"input_json_delta","partial_json":"ther\"}"}}"#
            )
            .expect("second input delta should parse")
            .as_slice(),
        [ResponseEvent::ToolCallInputDelta {
            item_id,
            call_id: Some(call_id),
            delta,
        }] if item_id == "toolu_1" && call_id == "toolu_1" && delta == "ther\"}"
    ));

    let done = state
        .process_data(r#"{"type":"content_block_stop","index":0}"#)
        .expect("tool_use stop should parse");
    assert!(matches!(
        &done[0],
        ResponseEvent::OutputItemDone(ResponseItem::FunctionCall {
            id: Some(id),
            name,
            namespace: None,
            arguments,
            call_id,
        }) if id == "toolu_1"
            && name == "lookup"
            && arguments == r#"{"query":"weather"}"#
            && call_id == "toolu_1"
    ));

    assert!(matches!(
        state
            .process_data(r#"{"type":"message_stop"}"#)
            .expect("message_stop should parse")
            .as_slice(),
        [ResponseEvent::Completed {
            response_id,
            end_turn: Some(false),
            ..
        }] if response_id == "msg_1"
    ));
}

#[test]
fn translates_error_event_to_stream_error() {
    let mut state = AnthropicStreamState::default();

    let err = state
        .process_data(
            r#"{"type":"error","error":{"type":"authentication_error","message":"bad key"}}"#,
        )
        .expect_err("error event should fail");

    assert!(err.to_string().contains("authentication_error"));
}
