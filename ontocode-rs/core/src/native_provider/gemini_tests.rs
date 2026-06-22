use super::*;
use http::HeaderMap;
use http::HeaderValue;
use ontocode_api::AuthProvider;
use ontocode_api::SharedAuthProvider;
use ontocode_protocol::models::BaseInstructions;
use ontocode_protocol::models::ContentItem;
use ontocode_protocol::models::FunctionCallOutputContentItem;
use ontocode_protocol::models::FunctionCallOutputPayload;
use ontocode_protocol::models::ResponseItem;
use ontocode_protocol::openai_models::ModelInfo;
use ontocode_tools::JsonSchema;
use ontocode_tools::ResponsesApiNamespace;
use ontocode_tools::ResponsesApiTool;
use ontocode_tools::ToolSpec;
use pretty_assertions::assert_eq;
use serde_json::json;
use serial_test::serial;
use std::collections::BTreeMap;
use std::sync::Arc;
use wiremock::Mock;
use wiremock::MockServer;
use wiremock::ResponseTemplate;
use wiremock::matchers::header;
use wiremock::matchers::method;
use wiremock::matchers::path;

fn model_info() -> ModelInfo {
    serde_json::from_value(json!({
        "slug": "gemini-2.5-pro",
        "display_name": "Gemini 2.5 Pro",
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
        "context_window": 1048576,
        "max_context_window": 1048576,
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

fn namespace_tool(name: &str) -> ToolSpec {
    ToolSpec::Namespace(ResponsesApiNamespace {
        name: name.to_string(),
        description: format!("Tools in the {name} namespace."),
        tools: Vec::new(),
    })
}

#[derive(Clone, Debug)]
struct StaticAuthProvider {
    headers: HeaderMap,
}

impl AuthProvider for StaticAuthProvider {
    fn add_auth_headers(&self, headers: &mut HeaderMap) {
        headers.extend(self.headers.clone());
    }
}

fn bearer_auth_provider(token: &str) -> SharedAuthProvider {
    let mut headers = HeaderMap::new();
    headers.insert(
        http::header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}")).expect("valid bearer auth header"),
    );
    Arc::new(StaticAuthProvider { headers })
}

#[test]
fn builds_basic_generate_content_request() {
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
        build_generate_content_request(&prompt, &model_info()).expect("request should build");

    assert_eq!(
        serde_json::to_value(request).expect("request should serialize"),
        json!({
            "contents": [
                {"role": "user", "parts": [{"text": "hello"}]},
                {"role": "model", "parts": [{"text": "hi"}]}
            ],
            "systemInstruction": {
                "parts": [{"text": "answer briefly"}]
            }
        })
    );
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

    let request =
        build_generate_content_request(&prompt, &model_info()).expect("request should build");

    assert_eq!(
        serde_json::to_value(request).expect("request should serialize"),
        json!({
            "contents": [
                {"role": "user", "parts": [{"text": "lookup weather"}]}
            ],
            "tools": [{
                "functionDeclarations": [{
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
                }]
            }]
        })
    );
}

#[test]
fn skips_namespace_tool_declarations() {
    let prompt = Prompt {
        input: vec![ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: "lookup weather".to_string(),
            }],
            phase: None,
        }],
        tools: vec![
            namespace_tool("multi_agent_v1"),
            namespace_tool("mcp__ontoindex"),
            lookup_tool(),
        ],
        base_instructions: BaseInstructions {
            text: String::new(),
        },
        ..Prompt::default()
    };

    let request =
        build_generate_content_request(&prompt, &model_info()).expect("request should build");

    assert_eq!(
        serde_json::to_value(request).expect("request should serialize"),
        json!({
            "contents": [
                {"role": "user", "parts": [{"text": "lookup weather"}]}
            ],
            "tools": [{
                "functionDeclarations": [{
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
                }]
            }]
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
        base_instructions: BaseInstructions {
            text: String::new(),
        },
        ..Prompt::default()
    };

    let request =
        build_generate_content_request(&prompt, &model_info()).expect("request should build");

    assert_eq!(
        serde_json::to_value(request).expect("request should serialize"),
        json!({
            "contents": [
                {
                    "role": "model",
                    "parts": [{
                        "functionCall": {
                            "name": "lookup",
                            "args": {"query": "weather"}
                        }
                    }]
                },
                {
                    "role": "user",
                    "parts": [{
                        "functionResponse": {
                            "name": "lookup",
                            "response": {"content": "sunny\n72F"}
                        }
                    }]
                }
            ]
        })
    );
}

#[test]
fn rejects_tool_result_without_prior_tool_call() {
    let prompt = Prompt {
        input: vec![ResponseItem::FunctionCallOutput {
            call_id: "call_1".to_string(),
            output: FunctionCallOutputPayload::from_text("orphan".to_string()),
        }],
        ..Prompt::default()
    };

    let err =
        build_generate_content_request(&prompt, &model_info()).expect_err("orphan result fails");

    assert!(err.to_string().contains("without prior tool call"));
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

    let err = build_generate_content_request(&prompt, &model_info()).expect_err("image fails");

    assert!(err.to_string().contains("multimodal input"));
}

#[tokio::test]
#[serial]
async fn native_runtime_requires_api_key_or_bearer_auth() {
    let request = GeminiGenerateContentRequest {
        contents: vec![GeminiContent {
            role: "user".to_string(),
            parts: vec![GeminiPart {
                text: Some("hello".to_string()),
                function_call: None,
                function_response: None,
            }],
        }],
        system_instruction: None,
        tools: None,
    };
    let provider_info = ModelProviderInfo {
        name: "gemini".to_string(),
        env_key: None,
        ..Default::default()
    };

    let err = match stream_generate_content(
        reqwest::Client::new(),
        &provider_info,
        &model_info(),
        None,
        request,
        http::HeaderMap::new(),
    )
    .await
    {
        Ok(_) => panic!("missing api-key auth should fail"),
        Err(err) => err,
    };

    assert!(
        err.to_string().contains(
            "Gemini native runtime requires either GEMINI_API_KEY or provider bearer auth"
        )
    );
}

#[tokio::test]
#[serial]
async fn native_runtime_prefers_x_goog_api_key_header_over_bearer_auth() {
    let server = MockServer::start().await;
    let request = GeminiGenerateContentRequest {
        contents: vec![GeminiContent {
            role: "user".to_string(),
            parts: vec![GeminiPart {
                text: Some("hello".to_string()),
                function_call: None,
                function_response: None,
            }],
        }],
        system_instruction: None,
        tools: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1beta/models/gemini-2.5-pro:streamGenerateContent"))
        .and(header("x-goog-api-key", "gemini-test-key"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_raw(
                    concat!(
                        "data: {\"responseId\":\"resp-1\",\"candidates\":[",
                        "{\"content\":{\"role\":\"model\",\"parts\":[{\"text\":\"ok\"}]},",
                        "\"finishReason\":\"STOP\"}]}",
                        "\n\n"
                    ),
                    "text/event-stream",
                ),
        )
        .expect(1)
        .mount(&server)
        .await;

    unsafe {
        std::env::set_var("GEMINI_API_KEY", "gemini-test-key");
    }
    let mut provider_info = ModelProviderInfo::create_gemini_provider();
    provider_info.base_url = Some(format!("{}/v1beta", server.uri()));
    provider_info.env_key = Some("GEMINI_API_KEY".to_string());
    let auth = bearer_auth_provider("bearer-test-token");

    let response = stream_generate_content(
        reqwest::Client::new(),
        &provider_info,
        &model_info(),
        Some(auth),
        request,
        http::HeaderMap::new(),
    )
    .await
    .expect("gemini request should succeed");

    drop(response);
    let request = server
        .received_requests()
        .await
        .expect("mock server should return captured requests")
        .pop()
        .expect("request should be captured");
    assert_eq!(
        request
            .headers
            .get(http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok()),
        None
    );
    assert_eq!(
        request
            .headers
            .get("x-goog-api-key")
            .and_then(|value| value.to_str().ok()),
        Some("gemini-test-key")
    );
    unsafe {
        std::env::remove_var("GEMINI_API_KEY");
    }
}

#[tokio::test]
#[serial]
async fn native_runtime_sends_authorization_header_when_api_key_is_missing() {
    let server = MockServer::start().await;
    let request = GeminiGenerateContentRequest {
        contents: vec![GeminiContent {
            role: "user".to_string(),
            parts: vec![GeminiPart {
                text: Some("hello".to_string()),
                function_call: None,
                function_response: None,
            }],
        }],
        system_instruction: None,
        tools: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1beta/models/gemini-2.5-pro:streamGenerateContent"))
        .and(header("authorization", "Bearer bearer-test-token"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_raw(
                    concat!(
                        "data: {\"responseId\":\"resp-1\",\"candidates\":[",
                        "{\"content\":{\"role\":\"model\",\"parts\":[{\"text\":\"ok\"}]},",
                        "\"finishReason\":\"STOP\"}]}",
                        "\n\n"
                    ),
                    "text/event-stream",
                ),
        )
        .expect(1)
        .mount(&server)
        .await;

    let mut provider_info = ModelProviderInfo::create_gemini_provider();
    provider_info.base_url = Some(format!("{}/v1beta", server.uri()));
    provider_info.env_key = Some("TEST_GEMINI_API_KEY_MISSING".to_string());

    let response = stream_generate_content(
        reqwest::Client::new(),
        &provider_info,
        &model_info(),
        Some(bearer_auth_provider("bearer-test-token")),
        request,
        http::HeaderMap::new(),
    )
    .await
    .expect("gemini request should succeed");

    drop(response);
    let request = server
        .received_requests()
        .await
        .expect("mock server should return captured requests")
        .pop()
        .expect("request should be captured");
    assert_eq!(
        request
            .headers
            .get(http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok()),
        Some("Bearer bearer-test-token")
    );
    assert_eq!(request.headers.get("x-goog-api-key"), None);
}

#[test]
fn translates_text_stream_to_response_events() {
    let mut state = GeminiStreamState::default();

    assert!(matches!(
        state
            .process_data(
                r#"{"responseId":"resp_1","candidates":[{"content":{"role":"model","parts":[{"text":"hel"}]}}],"usageMetadata":{"promptTokenCount":3}}"#
            )
            .expect("chunk should parse")
            .as_slice(),
        [ResponseEvent::Created, ResponseEvent::OutputTextDelta(delta)] if delta == "hel"
    ));
    assert!(matches!(
        state
            .process_data(
                r#"{"candidates":[{"content":{"role":"model","parts":[{"text":"lo"}]},"finishReason":"STOP"}],"usageMetadata":{"candidatesTokenCount":2,"totalTokenCount":5}}"#
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
fn translates_function_call_stream_to_response_events() {
    let mut state = GeminiStreamState::default();

    let events = state
        .process_data(
            r#"{"responseId":"resp_1","candidates":[{"content":{"role":"model","parts":[{"functionCall":{"name":"lookup","args":{"query":"weather"}}}]},"finishReason":"STOP"}]}"#,
        )
        .expect("function call chunk should parse");

    assert!(matches!(&events[0], ResponseEvent::Created));
    assert!(matches!(
        &events[1],
        ResponseEvent::OutputItemAdded(ResponseItem::FunctionCall {
            id: Some(id),
            name,
            namespace: None,
            arguments,
            call_id,
        }) if id == "gemini-call-0" && name == "lookup" && arguments.is_empty() && call_id == "gemini-call-0"
    ));
    assert!(matches!(
        &events[2],
        ResponseEvent::ToolCallInputDelta {
            item_id,
            call_id: Some(call_id),
            delta,
        } if item_id == "gemini-call-0"
            && call_id == "gemini-call-0"
            && delta == r#"{"query":"weather"}"#
    ));
    assert!(matches!(
        &events[3],
        ResponseEvent::OutputItemDone(ResponseItem::FunctionCall {
            id: Some(id),
            name,
            namespace: None,
            arguments,
            call_id,
        }) if id == "gemini-call-0"
            && name == "lookup"
            && arguments == r#"{"query":"weather"}"#
            && call_id == "gemini-call-0"
    ));
    assert!(matches!(
        &events[4],
        ResponseEvent::Completed {
            response_id,
            end_turn: Some(false),
            ..
        } if response_id == "resp_1"
    ));
}

#[test]
fn translates_error_event_to_stream_error() {
    let mut state = GeminiStreamState::default();

    let err = state
        .process_data(r#"{"error":{"code":401,"message":"bad key","status":"UNAUTHENTICATED"}}"#)
        .expect_err("error event should fail");

    assert!(err.to_string().contains("UNAUTHENTICATED"));
}
