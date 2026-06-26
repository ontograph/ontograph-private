use std::collections::BTreeMap;
use std::collections::HashSet;
use std::sync::LazyLock;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
use tracing::warn;

use crate::mcp_tool_call::McpToolApprovalKey;
use crate::mcp_tool_call::McpToolApprovalMetadata;
use crate::session::session::Session;
use crate::session::turn_context::TurnContext;
use ontocode_app_server_protocol::McpElicitationObjectType;
use ontocode_app_server_protocol::McpElicitationSchema;
use ontocode_app_server_protocol::McpServerElicitationRequest;
use ontocode_app_server_protocol::McpServerElicitationRequestParams;
use ontocode_mcp::CODEX_APPS_MCP_SERVER_NAME;
use ontocode_protocol::mcp_approval_meta::APPROVAL_KIND_KEY as MCP_TOOL_APPROVAL_KIND_KEY;
use ontocode_protocol::mcp_approval_meta::APPROVAL_KIND_MCP_TOOL_CALL as MCP_TOOL_APPROVAL_KIND_MCP_TOOL_CALL;
use ontocode_protocol::mcp_approval_meta::CONNECTOR_DESCRIPTION_KEY as MCP_TOOL_APPROVAL_CONNECTOR_DESCRIPTION_KEY;
use ontocode_protocol::mcp_approval_meta::CONNECTOR_ID_KEY as MCP_TOOL_APPROVAL_CONNECTOR_ID_KEY;
use ontocode_protocol::mcp_approval_meta::CONNECTOR_NAME_KEY as MCP_TOOL_APPROVAL_CONNECTOR_NAME_KEY;
use ontocode_protocol::mcp_approval_meta::PERSIST_ALWAYS as MCP_TOOL_APPROVAL_PERSIST_ALWAYS;
use ontocode_protocol::mcp_approval_meta::PERSIST_KEY as MCP_TOOL_APPROVAL_PERSIST_KEY;
use ontocode_protocol::mcp_approval_meta::PERSIST_SESSION as MCP_TOOL_APPROVAL_PERSIST_SESSION;
use ontocode_protocol::mcp_approval_meta::SOURCE_CONNECTOR as MCP_TOOL_APPROVAL_SOURCE_CONNECTOR;
use ontocode_protocol::mcp_approval_meta::SOURCE_KEY as MCP_TOOL_APPROVAL_SOURCE_KEY;
use ontocode_protocol::mcp_approval_meta::TOOL_DESCRIPTION_KEY as MCP_TOOL_APPROVAL_TOOL_DESCRIPTION_KEY;
use ontocode_protocol::mcp_approval_meta::TOOL_PARAMS_DISPLAY_KEY as MCP_TOOL_APPROVAL_TOOL_PARAMS_DISPLAY_KEY;
use ontocode_protocol::mcp_approval_meta::TOOL_PARAMS_KEY as MCP_TOOL_APPROVAL_TOOL_PARAMS_KEY;
use ontocode_protocol::mcp_approval_meta::TOOL_TITLE_KEY as MCP_TOOL_APPROVAL_TOOL_TITLE_KEY;
use ontocode_protocol::request_user_input::RequestUserInputQuestion;

const CONSEQUENTIAL_TOOL_MESSAGE_TEMPLATES_SCHEMA_VERSION: u8 = 4;
const CONNECTOR_NAME_TEMPLATE_VAR: &str = "{connector_name}";

static CONSEQUENTIAL_TOOL_MESSAGE_TEMPLATES: LazyLock<
    Option<Vec<ConsequentialToolMessageTemplate>>,
> = LazyLock::new(load_consequential_tool_message_templates);

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RenderedMcpToolApprovalTemplate {
    pub(crate) question: String,
    pub(crate) elicitation_message: String,
    pub(crate) tool_params: Option<Value>,
    pub(crate) tool_params_display: Vec<RenderedMcpToolApprovalParam>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct RenderedMcpToolApprovalParam {
    pub(crate) name: String,
    pub(crate) value: Value,
    pub(crate) display_name: String,
}

#[derive(Clone, Copy)]
pub(crate) struct McpToolApprovalPromptOptions {
    pub(crate) allow_session_remember: bool,
    pub(crate) allow_persistent_approval: bool,
}

pub(crate) struct McpToolApprovalElicitationRequest<'a> {
    pub(crate) server: &'a str,
    pub(crate) metadata: Option<&'a McpToolApprovalMetadata>,
    pub(crate) tool_params: Option<&'a Value>,
    pub(crate) tool_params_display: Option<&'a [RenderedMcpToolApprovalParam]>,
    pub(crate) question: RequestUserInputQuestion,
    pub(crate) message_override: Option<&'a str>,
    pub(crate) prompt_options: McpToolApprovalPromptOptions,
}

#[derive(Debug, Deserialize)]
struct ConsequentialToolMessageTemplatesFile {
    schema_version: u8,
    templates: Vec<ConsequentialToolMessageTemplate>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct ConsequentialToolMessageTemplate {
    connector_id: String,
    server_name: String,
    tool_title: String,
    template: String,
    template_params: Vec<ConsequentialToolTemplateParam>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct ConsequentialToolTemplateParam {
    name: String,
    label: String,
}

pub(crate) fn render_mcp_tool_approval_template(
    server_name: &str,
    connector_id: Option<&str>,
    connector_name: Option<&str>,
    tool_title: Option<&str>,
    tool_params: Option<&Value>,
) -> Option<RenderedMcpToolApprovalTemplate> {
    let templates = CONSEQUENTIAL_TOOL_MESSAGE_TEMPLATES.as_ref()?;
    render_mcp_tool_approval_template_from_templates(
        templates,
        server_name,
        connector_id,
        connector_name,
        tool_title,
        tool_params,
    )
}

pub(crate) fn mcp_tool_approval_prompt_options(
    session_approval_key: Option<&McpToolApprovalKey>,
    persistent_approval_key: Option<&McpToolApprovalKey>,
    tool_call_mcp_elicitation_enabled: bool,
) -> McpToolApprovalPromptOptions {
    McpToolApprovalPromptOptions {
        allow_session_remember: session_approval_key.is_some(),
        allow_persistent_approval: tool_call_mcp_elicitation_enabled
            && persistent_approval_key.is_some(),
    }
}

fn load_consequential_tool_message_templates() -> Option<Vec<ConsequentialToolMessageTemplate>> {
    let templates = match serde_json::from_str::<ConsequentialToolMessageTemplatesFile>(
        include_str!("consequential_tool_message_templates.json"),
    ) {
        Ok(templates) => templates,
        Err(err) => {
            warn!(error = %err, "failed to parse consequential tool approval templates");
            return None;
        }
    };

    if templates.schema_version != CONSEQUENTIAL_TOOL_MESSAGE_TEMPLATES_SCHEMA_VERSION {
        warn!(
            found_schema_version = templates.schema_version,
            expected_schema_version = CONSEQUENTIAL_TOOL_MESSAGE_TEMPLATES_SCHEMA_VERSION,
            "unexpected consequential tool approval templates schema version"
        );
        return None;
    }

    Some(templates.templates)
}

fn render_mcp_tool_approval_template_from_templates(
    templates: &[ConsequentialToolMessageTemplate],
    server_name: &str,
    connector_id: Option<&str>,
    connector_name: Option<&str>,
    tool_title: Option<&str>,
    tool_params: Option<&Value>,
) -> Option<RenderedMcpToolApprovalTemplate> {
    let connector_id = connector_id?;
    let tool_title = tool_title.map(str::trim).filter(|name| !name.is_empty())?;
    let template = templates.iter().find(|template| {
        template.server_name == server_name
            && template.connector_id == connector_id
            && template.tool_title == tool_title
    })?;
    let elicitation_message = render_question_template(&template.template, connector_name)?;
    let (tool_params, tool_params_display) = match tool_params {
        Some(Value::Object(tool_params)) => {
            render_tool_params(tool_params, &template.template_params)?
        }
        Some(_) => return None,
        None => (None, Vec::new()),
    };

    Some(RenderedMcpToolApprovalTemplate {
        question: elicitation_message.clone(),
        elicitation_message,
        tool_params,
        tool_params_display,
    })
}

fn render_question_template(template: &str, connector_name: Option<&str>) -> Option<String> {
    let template = template.trim();
    if template.is_empty() {
        return None;
    }

    if template.contains(CONNECTOR_NAME_TEMPLATE_VAR) {
        let connector_name = connector_name
            .map(str::trim)
            .filter(|name| !name.is_empty())?;
        return Some(template.replace(CONNECTOR_NAME_TEMPLATE_VAR, connector_name));
    }

    Some(template.to_string())
}

fn render_tool_params(
    tool_params: &Map<String, Value>,
    template_params: &[ConsequentialToolTemplateParam],
) -> Option<(Option<Value>, Vec<RenderedMcpToolApprovalParam>)> {
    let mut display_params = Vec::new();
    let mut display_names = HashSet::new();
    let mut handled_names = HashSet::new();

    for template_param in template_params {
        let label = template_param.label.trim();
        if label.is_empty() {
            return None;
        }
        let Some(value) = tool_params.get(&template_param.name) else {
            continue;
        };
        if !display_names.insert(label.to_string()) {
            return None;
        }
        display_params.push(RenderedMcpToolApprovalParam {
            name: template_param.name.clone(),
            value: value.clone(),
            display_name: label.to_string(),
        });
        handled_names.insert(template_param.name.as_str());
    }

    let mut remaining_params = tool_params
        .iter()
        .filter(|(name, _)| !handled_names.contains(name.as_str()))
        .collect::<Vec<_>>();
    remaining_params.sort_by_key(|(name, _)| *name);

    for (name, value) in remaining_params {
        if handled_names.contains(name.as_str()) {
            continue;
        }
        if !display_names.insert(name.clone()) {
            return None;
        }
        display_params.push(RenderedMcpToolApprovalParam {
            name: name.clone(),
            value: value.clone(),
            display_name: name.clone(),
        });
    }

    Some((Some(Value::Object(tool_params.clone())), display_params))
}

pub(crate) fn build_mcp_tool_approval_elicitation_request(
    sess: &Session,
    turn_context: &TurnContext,
    request: McpToolApprovalElicitationRequest<'_>,
) -> McpServerElicitationRequestParams {
    let message = request
        .message_override
        .map(ToString::to_string)
        .unwrap_or_else(|| request.question.question.clone());

    McpServerElicitationRequestParams {
        thread_id: sess.thread_id.to_string(),
        turn_id: Some(turn_context.sub_id.clone()),
        server_name: request.server.to_string(),
        request: McpServerElicitationRequest::Form {
            meta: build_mcp_tool_approval_elicitation_meta(
                request.server,
                request.metadata,
                request.tool_params,
                request.tool_params_display,
                request.prompt_options,
            ),
            message,
            requested_schema: McpElicitationSchema {
                schema_uri: None,
                type_: McpElicitationObjectType::Object,
                properties: BTreeMap::new(),
                required: None,
            },
        },
    }
}

pub(crate) fn build_mcp_tool_approval_elicitation_meta(
    server: &str,
    metadata: Option<&McpToolApprovalMetadata>,
    tool_params: Option<&Value>,
    tool_params_display: Option<&[RenderedMcpToolApprovalParam]>,
    prompt_options: McpToolApprovalPromptOptions,
) -> Option<Value> {
    let mut meta = serde_json::Map::new();
    meta.insert(
        MCP_TOOL_APPROVAL_KIND_KEY.to_string(),
        serde_json::Value::String(MCP_TOOL_APPROVAL_KIND_MCP_TOOL_CALL.to_string()),
    );
    match (
        prompt_options.allow_session_remember,
        prompt_options.allow_persistent_approval,
    ) {
        (true, true) => {
            meta.insert(
                MCP_TOOL_APPROVAL_PERSIST_KEY.to_string(),
                serde_json::json!([
                    MCP_TOOL_APPROVAL_PERSIST_SESSION,
                    MCP_TOOL_APPROVAL_PERSIST_ALWAYS,
                ]),
            );
        }
        (true, false) => {
            meta.insert(
                MCP_TOOL_APPROVAL_PERSIST_KEY.to_string(),
                serde_json::Value::String(MCP_TOOL_APPROVAL_PERSIST_SESSION.to_string()),
            );
        }
        (false, true) => {
            meta.insert(
                MCP_TOOL_APPROVAL_PERSIST_KEY.to_string(),
                serde_json::Value::String(MCP_TOOL_APPROVAL_PERSIST_ALWAYS.to_string()),
            );
        }
        (false, false) => {}
    }
    if let Some(metadata) = metadata {
        if let Some(tool_title) = metadata.tool_title.as_ref() {
            meta.insert(
                MCP_TOOL_APPROVAL_TOOL_TITLE_KEY.to_string(),
                serde_json::Value::String(tool_title.clone()),
            );
        }
        if let Some(tool_description) = metadata.tool_description.as_ref() {
            meta.insert(
                MCP_TOOL_APPROVAL_TOOL_DESCRIPTION_KEY.to_string(),
                serde_json::Value::String(tool_description.clone()),
            );
        }
        if server == CODEX_APPS_MCP_SERVER_NAME
            && (metadata.connector_id.is_some()
                || metadata.connector_name.is_some()
                || metadata.connector_description.is_some())
        {
            meta.insert(
                MCP_TOOL_APPROVAL_SOURCE_KEY.to_string(),
                serde_json::Value::String(MCP_TOOL_APPROVAL_SOURCE_CONNECTOR.to_string()),
            );
            if let Some(connector_id) = metadata.connector_id.as_deref() {
                meta.insert(
                    MCP_TOOL_APPROVAL_CONNECTOR_ID_KEY.to_string(),
                    serde_json::Value::String(connector_id.to_string()),
                );
            }
            if let Some(connector_name) = metadata.connector_name.as_ref() {
                meta.insert(
                    MCP_TOOL_APPROVAL_CONNECTOR_NAME_KEY.to_string(),
                    serde_json::Value::String(connector_name.clone()),
                );
            }
            if let Some(connector_description) = metadata.connector_description.as_ref() {
                meta.insert(
                    MCP_TOOL_APPROVAL_CONNECTOR_DESCRIPTION_KEY.to_string(),
                    serde_json::Value::String(connector_description.clone()),
                );
            }
        }
    }
    if let Some(tool_params) = tool_params {
        meta.insert(
            MCP_TOOL_APPROVAL_TOOL_PARAMS_KEY.to_string(),
            tool_params.clone(),
        );
    }
    if let Some(tool_params_display) = tool_params_display
        && let Ok(tool_params_display) = serde_json::to_value(tool_params_display)
    {
        meta.insert(
            MCP_TOOL_APPROVAL_TOOL_PARAMS_DISPLAY_KEY.to_string(),
            tool_params_display,
        );
    }
    (!meta.is_empty()).then_some(serde_json::Value::Object(meta))
}

pub(crate) fn build_mcp_tool_approval_display_params(
    tool_params: Option<&Value>,
) -> Option<Vec<RenderedMcpToolApprovalParam>> {
    let tool_params = tool_params?.as_object()?;
    let mut display_params = tool_params
        .iter()
        .map(|(name, value)| RenderedMcpToolApprovalParam {
            name: name.clone(),
            value: value.clone(),
            display_name: name.clone(),
        })
        .collect::<Vec<_>>();
    display_params.sort_by(|left, right| left.name.cmp(&right.name));
    Some(display_params)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn renders_exact_match_with_readable_param_labels() {
        let templates = vec![ConsequentialToolMessageTemplate {
            connector_id: "calendar".to_string(),
            server_name: "codex_apps".to_string(),
            tool_title: "create_event".to_string(),
            template: "Allow {connector_name} to create an event?".to_string(),
            template_params: vec![
                ConsequentialToolTemplateParam {
                    name: "calendar_id".to_string(),
                    label: "Calendar".to_string(),
                },
                ConsequentialToolTemplateParam {
                    name: "title".to_string(),
                    label: "Title".to_string(),
                },
            ],
        }];

        let rendered = render_mcp_tool_approval_template_from_templates(
            &templates,
            "codex_apps",
            Some("calendar"),
            Some("Calendar"),
            Some("create_event"),
            Some(&json!({
                "title": "Roadmap review",
                "calendar_id": "primary",
                "timezone": "UTC",
            })),
        );

        assert_eq!(
            rendered,
            Some(RenderedMcpToolApprovalTemplate {
                question: "Allow Calendar to create an event?".to_string(),
                elicitation_message: "Allow Calendar to create an event?".to_string(),
                tool_params: Some(json!({
                    "title": "Roadmap review",
                    "calendar_id": "primary",
                    "timezone": "UTC",
                })),
                tool_params_display: vec![
                    RenderedMcpToolApprovalParam {
                        name: "calendar_id".to_string(),
                        value: json!("primary"),
                        display_name: "Calendar".to_string(),
                    },
                    RenderedMcpToolApprovalParam {
                        name: "title".to_string(),
                        value: json!("Roadmap review"),
                        display_name: "Title".to_string(),
                    },
                    RenderedMcpToolApprovalParam {
                        name: "timezone".to_string(),
                        value: json!("UTC"),
                        display_name: "timezone".to_string(),
                    },
                ],
            })
        );
    }

    #[test]
    fn returns_none_when_no_exact_match_exists() {
        let templates = vec![ConsequentialToolMessageTemplate {
            connector_id: "calendar".to_string(),
            server_name: "codex_apps".to_string(),
            tool_title: "create_event".to_string(),
            template: "Allow {connector_name} to create an event?".to_string(),
            template_params: Vec::new(),
        }];

        assert_eq!(
            render_mcp_tool_approval_template_from_templates(
                &templates,
                "codex_apps",
                Some("calendar"),
                Some("Calendar"),
                Some("delete_event"),
                Some(&json!({})),
            ),
            None
        );
    }

    #[test]
    fn returns_none_when_relabeling_would_collide() {
        let templates = vec![ConsequentialToolMessageTemplate {
            connector_id: "calendar".to_string(),
            server_name: "codex_apps".to_string(),
            tool_title: "create_event".to_string(),
            template: "Allow {connector_name} to create an event?".to_string(),
            template_params: vec![ConsequentialToolTemplateParam {
                name: "calendar_id".to_string(),
                label: "timezone".to_string(),
            }],
        }];

        assert_eq!(
            render_mcp_tool_approval_template_from_templates(
                &templates,
                "codex_apps",
                Some("calendar"),
                Some("Calendar"),
                Some("create_event"),
                Some(&json!({
                    "calendar_id": "primary",
                    "timezone": "UTC",
                })),
            ),
            None
        );
    }

    #[test]
    fn bundled_templates_load() {
        assert_eq!(CONSEQUENTIAL_TOOL_MESSAGE_TEMPLATES.is_some(), true);
    }

    #[test]
    fn renders_literal_template_without_connector_substitution() {
        let templates = vec![ConsequentialToolMessageTemplate {
            connector_id: "github".to_string(),
            server_name: "codex_apps".to_string(),
            tool_title: "add_comment".to_string(),
            template: "Allow GitHub to add a comment to a pull request?".to_string(),
            template_params: Vec::new(),
        }];

        let rendered = render_mcp_tool_approval_template_from_templates(
            &templates,
            "codex_apps",
            Some("github"),
            /*connector_name*/ None,
            Some("add_comment"),
            Some(&json!({})),
        );

        assert_eq!(
            rendered,
            Some(RenderedMcpToolApprovalTemplate {
                question: "Allow GitHub to add a comment to a pull request?".to_string(),
                elicitation_message: "Allow GitHub to add a comment to a pull request?".to_string(),
                tool_params: Some(json!({})),
                tool_params_display: Vec::new(),
            })
        );
    }

    #[test]
    fn returns_none_when_connector_placeholder_has_no_value() {
        let templates = vec![ConsequentialToolMessageTemplate {
            connector_id: "calendar".to_string(),
            server_name: "codex_apps".to_string(),
            tool_title: "create_event".to_string(),
            template: "Allow {connector_name} to create an event?".to_string(),
            template_params: Vec::new(),
        }];

        assert_eq!(
            render_mcp_tool_approval_template_from_templates(
                &templates,
                "codex_apps",
                Some("calendar"),
                /*connector_name*/ None,
                Some("create_event"),
                Some(&json!({})),
            ),
            None
        );
    }
}
