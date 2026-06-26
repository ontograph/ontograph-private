use anyhow::Context;
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

use crate::config::Config;
use crate::config::edit::ConfigEdit;
use crate::config::edit::ConfigEditsBuilder;
use crate::connectors;
use crate::guardian::GuardianApprovalRequest;
use crate::guardian::GuardianMcpAnnotations;
use crate::guardian::guardian_rejection_message;
use crate::guardian::guardian_timeout_message;
use crate::guardian::new_guardian_review_id;
use crate::guardian::review_approval_request;
use crate::guardian::routes_approval_to_guardian_with_reviewer;
use crate::hook_runtime::run_permission_request_hooks;
use crate::mcp_openai_file::rewrite_mcp_tool_arguments_for_openai_files;
use crate::mcp_tool_approval_templates::McpToolApprovalElicitationRequest;
use crate::mcp_tool_approval_templates::McpToolApprovalPromptOptions;
use crate::mcp_tool_approval_templates::build_mcp_tool_approval_display_params;
use crate::mcp_tool_approval_templates::build_mcp_tool_approval_elicitation_request;
use crate::mcp_tool_approval_templates::mcp_tool_approval_prompt_options;
use crate::mcp_tool_approval_templates::render_mcp_tool_approval_template;
#[cfg(test)]
pub(crate) use crate::mcp_tool_call_result_shape::MCP_TOOL_CALL_EVENT_RESULT_MAX_BYTES;
use crate::mcp_tool_call_result_shape::sanitize_mcp_tool_result_for_model;
use crate::mcp_tool_call_result_shape::truncate_mcp_tool_result_for_event;
#[cfg(test)]
pub(crate) use crate::mcp_tool_call_telemetry::MCP_RESULT_TELEMETRY_TARGET_ID_MAX_CHARS;
use crate::mcp_tool_call_telemetry::McpToolCallSpanFields;
use crate::mcp_tool_call_telemetry::emit_mcp_call_metrics;
use crate::mcp_tool_call_telemetry::mcp_tool_call_span;
use crate::mcp_tool_call_telemetry::record_mcp_result_span_telemetry;
#[cfg(test)]
pub(crate) use crate::mcp_tool_call_telemetry::truncate_str_to_char_boundary;
use crate::session::session::Session;
use crate::session::turn_context::TurnContext;
use crate::tools::hook_names::HookToolName;
use crate::tools::sandboxing::PermissionRequestPayload;
use crate::turn_metadata::McpTurnMetadataContext;
use ontocode_analytics::AppInvocation;
use ontocode_analytics::InvocationType;
use ontocode_analytics::build_track_events_context;
use ontocode_app_server_protocol::ConfigLayerSource;
use ontocode_app_server_protocol::McpServerElicitationRequest;
use ontocode_app_server_protocol::McpServerElicitationRequestParams;
use ontocode_config::types::AppToolApproval;
use ontocode_config::types::ApprovalsReviewer;
use ontocode_features::Feature;
use ontocode_hooks::PermissionRequestDecision;
use ontocode_mcp::CODEX_APPS_MCP_SERVER_NAME;
use ontocode_mcp::MCP_TOOL_CODEX_APPS_META_KEY;
use ontocode_mcp::McpPermissionPromptAutoApproveContext;
use ontocode_mcp::SandboxState;
use ontocode_mcp::auth_elicitation_completed_result;
use ontocode_mcp::build_auth_elicitation_plan;
use ontocode_mcp::declared_openai_file_input_param_names;
use ontocode_mcp::mcp_permission_prompt_is_auto_approved;
use ontocode_protocol::items::McpToolCallError;
use ontocode_protocol::items::McpToolCallItem;
use ontocode_protocol::items::McpToolCallStatus;
use ontocode_protocol::items::TurnItem;
use ontocode_protocol::mcp::CallToolResult;
use ontocode_protocol::mcp_approval_meta::PERSIST_ALWAYS as MCP_TOOL_APPROVAL_PERSIST_ALWAYS;
use ontocode_protocol::mcp_approval_meta::PERSIST_KEY as MCP_TOOL_APPROVAL_PERSIST_KEY;
use ontocode_protocol::mcp_approval_meta::PERSIST_SESSION as MCP_TOOL_APPROVAL_PERSIST_SESSION;
use ontocode_protocol::openai_models::InputModality;
use ontocode_protocol::protocol::AskForApproval;
use ontocode_protocol::protocol::McpInvocation;
use ontocode_protocol::protocol::ReviewDecision;
use ontocode_protocol::request_user_input::RequestUserInputAnswer;
use ontocode_protocol::request_user_input::RequestUserInputArgs;
use ontocode_protocol::request_user_input::RequestUserInputQuestion;
use ontocode_protocol::request_user_input::RequestUserInputQuestionOption;
use ontocode_protocol::request_user_input::RequestUserInputResponse;
use ontocode_rmcp_client::ElicitationAction;
use ontocode_rmcp_client::ElicitationResponse;
use ontocode_rollout::state_db;
use ontocode_utils_absolute_path::AbsolutePathBuf;
use rmcp::model::ToolAnnotations;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use toml_edit::value;
use tracing::Instrument;
use tracing::Span;
use tracing::error;

/// Handles the specified tool call and dispatches the appropriate MCP tool-call
/// item lifecycle events to the `Session`.
pub(crate) async fn handle_mcp_tool_call(
    sess: Arc<Session>,
    turn_context: &Arc<TurnContext>,
    call_id: String,
    server: String,
    tool_name: String,
    hook_tool_name: HookToolName,
    arguments: String,
) -> HandledMcpToolCall {
    // Parse the `arguments` as JSON. An empty string is OK, but invalid JSON
    // is not.
    let arguments_value = if arguments.trim().is_empty() {
        None
    } else {
        match serde_json::from_str::<serde_json::Value>(&arguments) {
            Ok(value) => Some(value),
            Err(e) => {
                error!("failed to parse tool call arguments: {e}");
                return HandledMcpToolCall {
                    result: CallToolResult::from_error_text(format!("err: {e}")),
                    tool_input: JsonValue::Object(serde_json::Map::new()),
                };
            }
        }
    };

    let invocation = McpInvocation {
        server: server.clone(),
        tool: tool_name.clone(),
        arguments: arguments_value.clone(),
    };

    let metadata =
        lookup_mcp_tool_metadata(sess.as_ref(), turn_context.as_ref(), &server, &tool_name).await;
    let item_metadata = McpToolCallItemMetadata {
        mcp_app_resource_uri: metadata
            .as_ref()
            .and_then(|metadata| metadata.mcp_app_resource_uri.clone()),
        plugin_id: metadata
            .as_ref()
            .and_then(|metadata| metadata.plugin_id.clone()),
    };
    let app_tool_policy = if server == CODEX_APPS_MCP_SERVER_NAME {
        connectors::app_tool_policy(
            &turn_context.config,
            metadata
                .as_ref()
                .and_then(|metadata| metadata.connector_id.as_deref()),
            &tool_name,
            metadata
                .as_ref()
                .and_then(|metadata| metadata.tool_title.as_deref()),
            metadata
                .as_ref()
                .and_then(|metadata| metadata.annotations.as_ref()),
        )
    } else {
        connectors::AppToolPolicy::default()
    };
    let approval_mode = if server == CODEX_APPS_MCP_SERVER_NAME {
        app_tool_policy.approval
    } else {
        custom_mcp_tool_approval_mode(sess.as_ref(), turn_context.as_ref(), &server, &tool_name)
            .await
    };

    if server == CODEX_APPS_MCP_SERVER_NAME && !app_tool_policy.enabled {
        let result = notify_mcp_tool_call_skip(
            sess.as_ref(),
            turn_context.as_ref(),
            &call_id,
            invocation,
            item_metadata.clone(),
            "MCP tool call blocked by app configuration".to_string(),
            /*already_started*/ false,
        )
        .await;
        let status = if result.is_ok() { "ok" } else { "error" };
        turn_context.session_telemetry.counter(
            "codex.mcp.call",
            /*inc*/ 1,
            &[("status", status)],
        );
        return HandledMcpToolCall {
            result: CallToolResult::from_result(result),
            tool_input: arguments_value
                .unwrap_or_else(|| JsonValue::Object(serde_json::Map::new())),
        };
    }
    let connector_id = metadata
        .as_ref()
        .and_then(|metadata| metadata.connector_id.clone());
    let connector_name = metadata
        .as_ref()
        .and_then(|metadata| metadata.connector_name.clone());

    notify_mcp_tool_call_started(
        sess.as_ref(),
        turn_context.as_ref(),
        &call_id,
        invocation.clone(),
        item_metadata.clone(),
    )
    .await;

    if let Some(decision) = maybe_request_mcp_tool_approval(
        &sess,
        turn_context,
        &call_id,
        &invocation,
        &hook_tool_name,
        metadata.as_ref(),
        approval_mode,
    )
    .await
    {
        let result = match decision {
            McpToolApprovalDecision::Accept
            | McpToolApprovalDecision::AcceptForSession
            | McpToolApprovalDecision::AcceptAndRemember => {
                return handle_approved_mcp_tool_call(
                    sess.as_ref(),
                    turn_context.as_ref(),
                    &call_id,
                    invocation,
                    metadata.as_ref(),
                    item_metadata,
                )
                .await;
            }
            McpToolApprovalDecision::Decline { message } => {
                let message = message.unwrap_or_else(|| "user rejected MCP tool call".to_string());
                notify_mcp_tool_call_skip(
                    sess.as_ref(),
                    turn_context.as_ref(),
                    &call_id,
                    invocation,
                    item_metadata.clone(),
                    message,
                    /*already_started*/ true,
                )
                .await
            }
            McpToolApprovalDecision::Cancel => {
                let message = "user cancelled MCP tool call".to_string();
                notify_mcp_tool_call_skip(
                    sess.as_ref(),
                    turn_context.as_ref(),
                    &call_id,
                    invocation,
                    item_metadata.clone(),
                    message,
                    /*already_started*/ true,
                )
                .await
            }
        };

        let status = if result.is_ok() { "ok" } else { "error" };
        emit_mcp_call_metrics(
            turn_context.as_ref(),
            status,
            &tool_name,
            connector_id.as_deref(),
            connector_name.as_deref(),
            /*duration*/ None,
        );

        return HandledMcpToolCall {
            result: CallToolResult::from_result(result),
            tool_input: arguments_value
                .unwrap_or_else(|| JsonValue::Object(serde_json::Map::new())),
        };
    }

    handle_approved_mcp_tool_call(
        sess.as_ref(),
        turn_context.as_ref(),
        &call_id,
        invocation,
        metadata.as_ref(),
        item_metadata,
    )
    .await
}

pub(crate) struct HandledMcpToolCall {
    pub(crate) result: CallToolResult,
    pub(crate) tool_input: JsonValue,
}

#[derive(Clone)]
struct McpToolCallItemMetadata {
    mcp_app_resource_uri: Option<String>,
    plugin_id: Option<String>,
}

async fn handle_approved_mcp_tool_call(
    sess: &Session,
    turn_context: &TurnContext,
    call_id: &str,
    invocation: McpInvocation,
    metadata: Option<&McpToolApprovalMetadata>,
    item_metadata: McpToolCallItemMetadata,
) -> HandledMcpToolCall {
    let server = invocation.server.clone();
    maybe_mark_thread_memory_mode_polluted(sess, turn_context, &server).await;
    let tool_name = invocation.tool.clone();
    let arguments_value = invocation.arguments.clone();
    let connector_id = metadata.and_then(|metadata| metadata.connector_id.as_deref());
    let connector_name = metadata.and_then(|metadata| metadata.connector_name.as_deref());
    let server_origin = sess
        .services
        .mcp_connection_manager
        .read()
        .await
        .server_origin(&server)
        .map(str::to_string);

    let start = Instant::now();
    let rewrite = rewrite_mcp_tool_arguments_for_openai_files(
        sess,
        turn_context,
        arguments_value.clone(),
        metadata.and_then(|metadata| metadata.openai_file_input_params.as_deref()),
    )
    .await;
    let tool_input = match &rewrite {
        Ok(Some(rewritten_arguments)) => rewritten_arguments.clone(),
        Ok(None) | Err(_) => arguments_value
            .clone()
            .unwrap_or_else(|| JsonValue::Object(serde_json::Map::new())),
    };
    let result = async {
        let rewritten_arguments = rewrite?;
        let request_meta =
            build_mcp_tool_call_request_meta(turn_context, &server, call_id, metadata);
        let result = execute_mcp_tool_call(
            sess,
            turn_context,
            call_id,
            &invocation,
            rewritten_arguments,
            metadata,
            request_meta,
        )
        .await;
        record_mcp_result_span_telemetry(&Span::current(), result.as_ref().ok());
        result
    }
    .instrument(mcp_tool_call_span(
        sess,
        turn_context,
        McpToolCallSpanFields {
            server_name: &server,
            tool_name: &tool_name,
            call_id,
            server_origin: server_origin.as_deref(),
            connector_id,
            connector_name,
        },
    ))
    .await;
    if let Err(error) = &result {
        tracing::warn!("MCP tool call error: {error:?}");
    }
    let duration = start.elapsed();
    notify_mcp_tool_call_completed(
        sess,
        turn_context,
        call_id,
        invocation,
        item_metadata,
        duration,
        truncate_mcp_tool_result_for_event(&result),
    )
    .await;
    maybe_track_codex_app_used(sess, turn_context, &server, &tool_name).await;

    let status = if result.is_ok() { "ok" } else { "error" };
    emit_mcp_call_metrics(
        turn_context,
        status,
        &tool_name,
        connector_id,
        connector_name,
        Some(duration),
    );

    HandledMcpToolCall {
        result: CallToolResult::from_result(result),
        tool_input,
    }
}

async fn execute_mcp_tool_call(
    sess: &Session,
    turn_context: &TurnContext,
    call_id: &str,
    invocation: &McpInvocation,
    rewritten_arguments: Option<JsonValue>,
    metadata: Option<&McpToolApprovalMetadata>,
    request_meta: Option<JsonValue>,
) -> Result<CallToolResult, String> {
    let request_meta = with_mcp_tool_call_thread_id_meta(request_meta, &sess.thread_id.to_string());
    let request_meta = augment_mcp_tool_request_meta_with_sandbox_state(
        sess,
        turn_context,
        &invocation.server,
        request_meta,
    )
    .await
    .map_err(|e| format!("failed to build MCP tool request metadata: {e:#}"))?;
    let mcp_call_trace = sess
        .services
        .rollout_thread_trace
        .start_mcp_call_trace(call_id);
    let request_meta = mcp_call_trace.add_request_meta(request_meta);
    let result = sess
        .call_tool(
            &invocation.server,
            &invocation.tool,
            rewritten_arguments,
            request_meta,
        )
        .await
        .map_err(|e| format!("tool call error: {e:?}"))?;
    let result = sanitize_mcp_tool_result_for_model(
        turn_context
            .model_info
            .input_modalities
            .contains(&InputModality::Image),
        Ok(result),
    )?;
    Ok(maybe_request_codex_apps_auth_elicitation(
        sess,
        turn_context,
        call_id,
        &invocation.server,
        metadata,
        result,
    )
    .await)
}

async fn maybe_request_codex_apps_auth_elicitation(
    sess: &Session,
    turn_context: &TurnContext,
    call_id: &str,
    server: &str,
    metadata: Option<&McpToolApprovalMetadata>,
    result: CallToolResult,
) -> CallToolResult {
    if !sess
        .services
        .mcp_connection_manager
        .read()
        .await
        .is_host_owned_codex_apps_server(server)
    {
        return result;
    }

    if !turn_context.features.enabled(Feature::AuthElicitation) {
        return result;
    }

    match turn_context.approval_policy.value() {
        AskForApproval::Never => return result,
        AskForApproval::Granular(granular_config) if !granular_config.allows_mcp_elicitations() => {
            return result;
        }
        AskForApproval::OnFailure
        | AskForApproval::OnRequest
        | AskForApproval::UnlessTrusted
        | AskForApproval::Granular(_) => {}
    }

    let connector_id = metadata.and_then(|metadata| metadata.connector_id.as_deref());
    let connector_name = metadata.and_then(|metadata| metadata.connector_name.as_deref());
    let install_url = connector_id.map(|connector_id| {
        ontocode_connectors::metadata::connector_install_url(
            connector_name.unwrap_or(connector_id),
            connector_id,
        )
    });
    let Some(plan) =
        build_auth_elicitation_plan(call_id, &result, connector_id, connector_name, install_url)
    else {
        return result;
    };

    let request_id = rmcp::model::RequestId::String(plan.elicitation.elicitation_id.clone().into());
    let params = McpServerElicitationRequestParams {
        thread_id: sess.thread_id.to_string(),
        turn_id: Some(turn_context.sub_id.clone()),
        server_name: CODEX_APPS_MCP_SERVER_NAME.to_string(),
        request: McpServerElicitationRequest::Url {
            meta: Some(plan.elicitation.meta),
            message: plan.elicitation.message,
            url: plan.elicitation.url,
            elicitation_id: plan.elicitation.elicitation_id,
        },
    };
    let response = sess
        .request_mcp_server_elicitation(turn_context, request_id, params)
        .await
        .response;
    if !response
        .as_ref()
        .is_some_and(|response| response.action == ElicitationAction::Accept)
    {
        return result;
    }

    refresh_codex_apps_after_connector_auth(sess, turn_context).await;
    auth_elicitation_completed_result(&plan.auth_failure, result.meta)
}

#[expect(
    clippy::await_holding_invalid_type,
    reason = "Codex Apps cache refresh reads through the session-owned manager guard"
)]
async fn refresh_codex_apps_after_connector_auth(sess: &Session, turn_context: &TurnContext) {
    let mcp_tools_result = {
        let manager = sess.services.mcp_connection_manager.read().await;
        manager.hard_refresh_codex_apps_tools_cache().await
    };

    match mcp_tools_result {
        Ok(mcp_tools) => {
            let auth = sess.services.auth_manager.auth().await;
            connectors::refresh_accessible_connectors_cache_from_mcp_tools(
                &turn_context.config,
                auth.as_ref(),
                &mcp_tools,
            );
        }
        Err(err) => {
            tracing::warn!("failed to refresh Codex Apps tools after connector auth: {err:#}");
        }
    }
}

#[expect(
    clippy::await_holding_invalid_type,
    reason = "MCP sandbox metadata reads through the session-owned manager guard"
)]
async fn augment_mcp_tool_request_meta_with_sandbox_state(
    sess: &Session,
    turn_context: &TurnContext,
    server: &str,
    mut meta: Option<serde_json::Value>,
) -> anyhow::Result<Option<serde_json::Value>> {
    let supports_sandbox_state_meta = sess
        .services
        .mcp_connection_manager
        .read()
        .await
        .server_supports_sandbox_state_meta_capability(server)
        .await
        .unwrap_or(false);
    if !supports_sandbox_state_meta {
        return Ok(meta);
    }

    let sandbox_state = serde_json::to_value(SandboxState {
        permission_profile: Some(turn_context.permission_profile()),
        sandbox_policy: turn_context.sandbox_policy(),
        codex_linux_sandbox_exe: turn_context.codex_linux_sandbox_exe.clone(),
        #[allow(deprecated)]
        sandbox_cwd: turn_context.cwd.to_path_buf(),
        use_legacy_landlock: turn_context.features.use_legacy_landlock(),
    })?;

    match meta.as_mut() {
        Some(serde_json::Value::Object(map)) => {
            map.insert(
                ontocode_mcp::MCP_SANDBOX_STATE_META_CAPABILITY.to_string(),
                sandbox_state,
            );
        }
        Some(_) => {}
        None => {
            let mut map = serde_json::Map::new();
            map.insert(
                ontocode_mcp::MCP_SANDBOX_STATE_META_CAPABILITY.to_string(),
                sandbox_state,
            );
            meta = Some(serde_json::Value::Object(map));
        }
    }

    Ok(meta)
}

async fn maybe_mark_thread_memory_mode_polluted(
    sess: &Session,
    turn_context: &TurnContext,
    server: &str,
) {
    if !turn_context.config.memories.disable_on_external_context {
        return;
    }
    let pollutes_memory = sess
        .services
        .mcp_connection_manager
        .read()
        .await
        .server_pollutes_memory(server);
    if !pollutes_memory {
        return;
    }
    state_db::mark_thread_memory_mode_polluted(
        sess.services.state_db.as_deref(),
        sess.thread_id,
        "mcp_tool_call",
    )
    .await;
}

async fn notify_mcp_tool_call_started(
    sess: &Session,
    turn_context: &TurnContext,
    call_id: &str,
    invocation: McpInvocation,
    item_metadata: McpToolCallItemMetadata,
) {
    let McpInvocation {
        server,
        tool,
        arguments,
    } = invocation;
    let item = TurnItem::McpToolCall(McpToolCallItem {
        id: call_id.to_string(),
        server,
        tool,
        arguments: arguments.unwrap_or(JsonValue::Null),
        mcp_app_resource_uri: item_metadata.mcp_app_resource_uri,
        plugin_id: item_metadata.plugin_id,
        status: McpToolCallStatus::InProgress,
        result: None,
        error: None,
        duration: None,
    });
    sess.emit_turn_item_started(turn_context, &item).await;
}

async fn notify_mcp_tool_call_completed(
    sess: &Session,
    turn_context: &TurnContext,
    call_id: &str,
    invocation: McpInvocation,
    item_metadata: McpToolCallItemMetadata,
    duration: Duration,
    result: Result<CallToolResult, String>,
) {
    let (status, result, error) = match result {
        Ok(result) if result.is_error.unwrap_or(false) => {
            (McpToolCallStatus::Failed, Some(result), None)
        }
        Ok(result) => (McpToolCallStatus::Completed, Some(result), None),
        Err(message) => (
            McpToolCallStatus::Failed,
            None,
            Some(McpToolCallError { message }),
        ),
    };
    let McpInvocation {
        server,
        tool,
        arguments,
    } = invocation;
    let item = TurnItem::McpToolCall(McpToolCallItem {
        id: call_id.to_string(),
        server,
        tool,
        arguments: arguments.unwrap_or(JsonValue::Null),
        mcp_app_resource_uri: item_metadata.mcp_app_resource_uri,
        plugin_id: item_metadata.plugin_id,
        status,
        result,
        error,
        duration: Some(duration),
    });
    sess.emit_turn_item_completed(turn_context, item).await;
}

struct McpAppUsageMetadata {
    connector_id: Option<String>,
    app_name: Option<String>,
}

async fn maybe_track_codex_app_used(
    sess: &Session,
    turn_context: &TurnContext,
    server: &str,
    tool_name: &str,
) {
    if server != CODEX_APPS_MCP_SERVER_NAME {
        return;
    }
    let metadata = lookup_mcp_app_usage_metadata(sess, server, tool_name).await;
    let (connector_id, app_name) = metadata
        .map(|metadata| (metadata.connector_id, metadata.app_name))
        .unwrap_or((None, None));
    let invocation_type = if let Some(connector_id) = connector_id.as_deref() {
        let mentioned_connector_ids = sess.get_connector_selection().await;
        if mentioned_connector_ids.contains(connector_id) {
            InvocationType::Explicit
        } else {
            InvocationType::Implicit
        }
    } else {
        InvocationType::Implicit
    };

    let tracking = build_track_events_context(
        turn_context.model_info.slug.clone(),
        sess.thread_id.to_string(),
        turn_context.sub_id.clone(),
    );
    sess.services.analytics_events_client.track_app_used(
        tracking,
        AppInvocation {
            connector_id,
            app_name,
            invocation_type: Some(invocation_type),
        },
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum McpToolApprovalDecision {
    Accept,
    AcceptForSession,
    AcceptAndRemember,
    Decline { message: Option<String> },
    Cancel,
}

pub(crate) struct McpToolApprovalMetadata {
    pub(crate) annotations: Option<ToolAnnotations>,
    pub(crate) connector_id: Option<String>,
    pub(crate) connector_name: Option<String>,
    pub(crate) connector_description: Option<String>,
    pub(crate) plugin_id: Option<String>,
    pub(crate) tool_title: Option<String>,
    pub(crate) tool_description: Option<String>,
    pub(crate) mcp_app_resource_uri: Option<String>,
    pub(crate) codex_apps_meta: Option<serde_json::Map<String, serde_json::Value>>,
    pub(crate) openai_file_input_params: Option<Vec<String>>,
}

const MCP_TOOL_OPENAI_OUTPUT_TEMPLATE_META_KEY: &str = "openai/outputTemplate";
const MCP_TOOL_UI_RESOURCE_URI_META_KEY: &str = "ui/resourceUri";
const MCP_TOOL_PLUGIN_ID_META_KEY: &str = "plugin_id";
const MCP_TOOL_THREAD_ID_META_KEY: &str = "threadId";

async fn custom_mcp_tool_approval_mode(
    sess: &Session,
    turn_context: &TurnContext,
    server: &str,
    tool_name: &str,
) -> AppToolApproval {
    let user_configured_mode = turn_context
        .config
        .config_layer_stack
        .effective_config()
        .as_table()
        .and_then(|table| table.get("mcp_servers"))
        .cloned()
        .and_then(|value| {
            HashMap::<String, ontocode_config::types::McpServerConfig>::deserialize(value).ok()
        })
        .and_then(|servers| {
            let server_config = servers.get(server)?;
            Some(
                server_config
                    .tools
                    .get(tool_name)
                    .and_then(|tool| tool.approval_mode)
                    .or(server_config.default_tools_approval_mode)
                    .unwrap_or_default(),
            )
        });
    if let Some(user_configured_mode) = user_configured_mode {
        return user_configured_mode;
    }

    sess.services
        .plugins_manager
        .plugins_for_config(&turn_context.config.plugins_config_input())
        .await
        .plugins()
        .iter()
        .filter(|plugin| plugin.is_active())
        .find_map(|plugin| {
            let server_config = plugin.mcp_servers.get(server)?;
            server_config
                .tools
                .get(tool_name)
                .and_then(|tool| tool.approval_mode)
                .or(server_config.default_tools_approval_mode)
        })
        .unwrap_or_default()
}

fn build_mcp_tool_call_request_meta(
    turn_context: &TurnContext,
    server: &str,
    call_id: &str,
    metadata: Option<&McpToolApprovalMetadata>,
) -> Option<serde_json::Value> {
    let mut request_meta = serde_json::Map::new();

    if let Some(turn_metadata) = turn_context
        .turn_metadata_state
        .current_meta_value_for_mcp_request(McpTurnMetadataContext {
            model: turn_context.model_info.slug.as_str(),
            reasoning_effort: turn_context.effective_reasoning_effort(),
        })
    {
        request_meta.insert(
            crate::X_CODEX_TURN_METADATA_HEADER.to_string(),
            turn_metadata,
        );
    }

    if server == CODEX_APPS_MCP_SERVER_NAME {
        let mut codex_apps_meta = metadata
            .and_then(|metadata| metadata.codex_apps_meta.clone())
            .unwrap_or_default();
        codex_apps_meta.insert(
            "call_id".to_string(),
            serde_json::Value::String(call_id.to_string()),
        );
        request_meta.insert(
            MCP_TOOL_CODEX_APPS_META_KEY.to_string(),
            serde_json::Value::Object(codex_apps_meta),
        );
    }
    if let Some(plugin_id) = metadata.and_then(|metadata| metadata.plugin_id.as_ref()) {
        request_meta.insert(
            MCP_TOOL_PLUGIN_ID_META_KEY.to_string(),
            serde_json::Value::String(plugin_id.clone()),
        );
    }

    (!request_meta.is_empty()).then_some(serde_json::Value::Object(request_meta))
}

fn with_mcp_tool_call_thread_id_meta(
    meta: Option<serde_json::Value>,
    thread_id: &str,
) -> Option<serde_json::Value> {
    match meta {
        Some(serde_json::Value::Object(mut map)) => {
            map.insert(
                MCP_TOOL_THREAD_ID_META_KEY.to_string(),
                serde_json::Value::String(thread_id.to_string()),
            );
            Some(serde_json::Value::Object(map))
        }
        None => {
            let mut map = serde_json::Map::new();
            map.insert(
                MCP_TOOL_THREAD_ID_META_KEY.to_string(),
                serde_json::Value::String(thread_id.to_string()),
            );
            Some(serde_json::Value::Object(map))
        }
        other => other,
    }
}

pub(crate) const MCP_TOOL_APPROVAL_QUESTION_ID_PREFIX: &str = "mcp_tool_call_approval";
pub(crate) const MCP_TOOL_APPROVAL_ACCEPT: &str = "Allow";
pub(crate) const MCP_TOOL_APPROVAL_ACCEPT_FOR_SESSION: &str = "Allow for this session";
// Internal-only token used when guardian auto-reviews delegated MCP approvals on the
// RequestUserInput compatibility path. That legacy MCP prompt has allow/cancel labels but no
// real "Decline" answer, so this lets guardian denials round-trip distinctly from user cancel.
// This is not a user-facing option.
pub(crate) const MCP_TOOL_APPROVAL_DECLINE_SYNTHETIC: &str = "__codex_mcp_decline__";
const MCP_TOOL_APPROVAL_ACCEPT_AND_REMEMBER: &str = "Allow and don't ask me again";
const MCP_TOOL_APPROVAL_CANCEL: &str = "Cancel";

pub(crate) fn is_mcp_tool_approval_question_id(question_id: &str) -> bool {
    question_id
        .strip_prefix(MCP_TOOL_APPROVAL_QUESTION_ID_PREFIX)
        .is_some_and(|suffix| suffix.starts_with('_'))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct McpToolApprovalKey {
    server: String,
    connector_id: Option<String>,
    tool_name: String,
}

async fn maybe_request_mcp_tool_approval(
    sess: &Arc<Session>,
    turn_context: &Arc<TurnContext>,
    call_id: &str,
    invocation: &McpInvocation,
    hook_tool_name: &HookToolName,
    metadata: Option<&McpToolApprovalMetadata>,
    approval_mode: AppToolApproval,
) -> Option<McpToolApprovalDecision> {
    let approvals_reviewer = mcp_approvals_reviewer(turn_context, &invocation.server, metadata);
    if mcp_permission_prompt_is_auto_approved(
        turn_context.approval_policy.value(),
        &turn_context.permission_profile(),
        McpPermissionPromptAutoApproveContext {
            tool_approval_mode: Some(approval_mode),
        },
    ) {
        return None;
    }

    let annotations = metadata.and_then(|metadata| metadata.annotations.as_ref());
    let approval_required = requires_mcp_tool_approval(annotations);
    if !approval_required && approval_mode != AppToolApproval::Prompt {
        return None;
    }

    let session_approval_key = session_mcp_tool_approval_key(invocation, metadata, approval_mode);
    let persistent_approval_key =
        persistent_mcp_tool_approval_key(invocation, metadata, approval_mode);
    if let Some(key) = session_approval_key.as_ref()
        && mcp_tool_approval_is_remembered(sess, key).await
    {
        return Some(McpToolApprovalDecision::Accept);
    }

    match run_permission_request_hooks(
        sess,
        turn_context,
        call_id,
        PermissionRequestPayload {
            tool_name: hook_tool_name.clone(),
            tool_input: invocation
                .arguments
                .clone()
                .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new())),
        },
    )
    .await
    {
        Some(PermissionRequestDecision::Allow) => {
            return Some(McpToolApprovalDecision::Accept);
        }
        Some(PermissionRequestDecision::Deny { message }) => {
            return Some(McpToolApprovalDecision::Decline {
                message: Some(message),
            });
        }
        None => {}
    }

    let tool_call_mcp_elicitation_enabled = turn_context
        .config
        .features
        .enabled(Feature::ToolCallMcpElicitation);

    if routes_approval_to_guardian_with_reviewer(turn_context, approvals_reviewer) {
        let review_id = new_guardian_review_id();
        let decision = review_approval_request(
            sess,
            turn_context,
            review_id.clone(),
            build_guardian_mcp_tool_review_request(call_id, invocation, metadata),
            /*retry_reason*/ None,
        )
        .await;
        let decision = mcp_tool_approval_decision_from_guardian(sess, &review_id, decision).await;
        apply_mcp_tool_approval_decision(
            sess,
            turn_context,
            &decision,
            session_approval_key,
            persistent_approval_key,
        )
        .await;
        return Some(decision);
    }

    let prompt_options = mcp_tool_approval_prompt_options(
        session_approval_key.as_ref(),
        persistent_approval_key.as_ref(),
        tool_call_mcp_elicitation_enabled,
    );
    let question_id = format!("{MCP_TOOL_APPROVAL_QUESTION_ID_PREFIX}_{call_id}");
    let rendered_template = render_mcp_tool_approval_template(
        &invocation.server,
        metadata.and_then(|metadata| metadata.connector_id.as_deref()),
        metadata.and_then(|metadata| metadata.connector_name.as_deref()),
        metadata.and_then(|metadata| metadata.tool_title.as_deref()),
        invocation.arguments.as_ref(),
    );
    let tool_params_display = rendered_template
        .as_ref()
        .map(|rendered_template| rendered_template.tool_params_display.clone())
        .or_else(|| build_mcp_tool_approval_display_params(invocation.arguments.as_ref()));
    let question = build_mcp_tool_approval_question(
        question_id.clone(),
        &invocation.server,
        &invocation.tool,
        metadata.and_then(|metadata| metadata.connector_name.as_deref()),
        prompt_options,
        rendered_template
            .as_ref()
            .map(|rendered_template| rendered_template.question.as_str()),
    );
    if tool_call_mcp_elicitation_enabled {
        let request_id = rmcp::model::RequestId::String(
            format!("{MCP_TOOL_APPROVAL_QUESTION_ID_PREFIX}_{call_id}").into(),
        );
        let params = build_mcp_tool_approval_elicitation_request(
            sess.as_ref(),
            turn_context.as_ref(),
            McpToolApprovalElicitationRequest {
                server: &invocation.server,
                metadata,
                tool_params: rendered_template
                    .as_ref()
                    .and_then(|rendered_template| rendered_template.tool_params.as_ref())
                    .or(invocation.arguments.as_ref()),
                tool_params_display: tool_params_display.as_deref(),
                question,
                message_override: rendered_template
                    .as_ref()
                    .map(|rendered_template| rendered_template.elicitation_message.as_str()),
                prompt_options,
            },
        );
        let decision = parse_mcp_tool_approval_elicitation_response(
            sess.request_mcp_server_elicitation(turn_context.as_ref(), request_id, params)
                .await
                .response,
            &question_id,
        );
        let decision = normalize_approval_decision_for_mode(decision, approval_mode);
        apply_mcp_tool_approval_decision(
            sess,
            turn_context,
            &decision,
            session_approval_key,
            persistent_approval_key,
        )
        .await;
        return Some(decision);
    }

    let args = RequestUserInputArgs {
        questions: vec![question],
    };
    let response = sess
        .request_user_input(turn_context.as_ref(), call_id.to_string(), args)
        .await;
    let decision = normalize_approval_decision_for_mode(
        parse_mcp_tool_approval_response(response, &question_id),
        approval_mode,
    );
    apply_mcp_tool_approval_decision(
        sess,
        turn_context,
        &decision,
        session_approval_key,
        persistent_approval_key,
    )
    .await;
    Some(decision)
}

pub(crate) fn mcp_approvals_reviewer(
    turn_context: &TurnContext,
    server_name: &str,
    metadata: Option<&McpToolApprovalMetadata>,
) -> ApprovalsReviewer {
    connectors::mcp_approvals_reviewer(
        turn_context.config.as_ref(),
        server_name,
        metadata.and_then(|metadata| metadata.connector_id.as_deref()),
    )
}

fn session_mcp_tool_approval_key(
    invocation: &McpInvocation,
    metadata: Option<&McpToolApprovalMetadata>,
    approval_mode: AppToolApproval,
) -> Option<McpToolApprovalKey> {
    if approval_mode != AppToolApproval::Auto {
        return None;
    }

    let connector_id = metadata.and_then(|metadata| metadata.connector_id.clone());
    if invocation.server == CODEX_APPS_MCP_SERVER_NAME && connector_id.is_none() {
        return None;
    }

    Some(McpToolApprovalKey {
        server: invocation.server.clone(),
        connector_id,
        tool_name: invocation.tool.clone(),
    })
}

fn persistent_mcp_tool_approval_key(
    invocation: &McpInvocation,
    metadata: Option<&McpToolApprovalMetadata>,
    approval_mode: AppToolApproval,
) -> Option<McpToolApprovalKey> {
    session_mcp_tool_approval_key(invocation, metadata, approval_mode)
}

pub(crate) fn build_guardian_mcp_tool_review_request(
    call_id: &str,
    invocation: &McpInvocation,
    metadata: Option<&McpToolApprovalMetadata>,
) -> GuardianApprovalRequest {
    GuardianApprovalRequest::McpToolCall {
        id: call_id.to_string(),
        server: invocation.server.clone(),
        tool_name: invocation.tool.clone(),
        arguments: invocation.arguments.clone(),
        connector_id: metadata.and_then(|metadata| metadata.connector_id.clone()),
        connector_name: metadata.and_then(|metadata| metadata.connector_name.clone()),
        connector_description: metadata.and_then(|metadata| metadata.connector_description.clone()),
        tool_title: metadata.and_then(|metadata| metadata.tool_title.clone()),
        tool_description: metadata.and_then(|metadata| metadata.tool_description.clone()),
        annotations: metadata
            .and_then(|metadata| metadata.annotations.as_ref())
            .map(|annotations| GuardianMcpAnnotations {
                destructive_hint: annotations.destructive_hint,
                open_world_hint: annotations.open_world_hint,
                read_only_hint: annotations.read_only_hint,
            }),
    }
}

async fn mcp_tool_approval_decision_from_guardian(
    sess: &Session,
    review_id: &str,
    decision: ReviewDecision,
) -> McpToolApprovalDecision {
    match decision {
        ReviewDecision::Approved
        | ReviewDecision::ApprovedExecpolicyAmendment { .. }
        | ReviewDecision::NetworkPolicyAmendment { .. } => McpToolApprovalDecision::Accept,
        ReviewDecision::ApprovedForSession => McpToolApprovalDecision::AcceptForSession,
        ReviewDecision::Denied => McpToolApprovalDecision::Decline {
            message: Some(guardian_rejection_message(sess, review_id).await),
        },
        ReviewDecision::TimedOut => McpToolApprovalDecision::Decline {
            message: Some(guardian_timeout_message()),
        },
        ReviewDecision::Abort => McpToolApprovalDecision::Decline { message: None },
    }
}

#[expect(
    clippy::await_holding_invalid_type,
    reason = "MCP approval metadata reads through the session-owned manager guard"
)]
pub(crate) async fn lookup_mcp_tool_metadata(
    sess: &Session,
    turn_context: &TurnContext,
    server: &str,
    tool_name: &str,
) -> Option<McpToolApprovalMetadata> {
    let manager = sess.services.mcp_connection_manager.read().await;
    let plugin_id = manager
        .plugin_id_for_mcp_server_name(server)
        .map(str::to_string);
    let tools = manager.list_all_tools().await;
    let tool_info = tools
        .into_iter()
        .find(|tool_info| tool_info.server_name == server && tool_info.tool.name == tool_name)?;
    let connector_description = if server == CODEX_APPS_MCP_SERVER_NAME {
        let connectors = match connectors::list_cached_accessible_connectors_from_mcp_tools(
            turn_context.config.as_ref(),
        )
        .await
        {
            Some(connectors) => Some(connectors),
            None => {
                connectors::list_accessible_connectors_from_mcp_tools(turn_context.config.as_ref())
                    .await
                    .ok()
            }
        };
        connectors.and_then(|connectors| {
            let connector_id = tool_info.connector_id.as_deref()?;
            connectors
                .into_iter()
                .find(|connector| connector.id == connector_id)
                .and_then(|connector| connector.description)
        })
    } else {
        None
    };

    Some(McpToolApprovalMetadata {
        annotations: tool_info.tool.annotations,
        connector_id: tool_info.connector_id,
        connector_name: tool_info.connector_name,
        connector_description,
        plugin_id,
        tool_title: tool_info.tool.title,
        tool_description: tool_info.tool.description.map(std::borrow::Cow::into_owned),
        mcp_app_resource_uri: get_mcp_app_resource_uri(tool_info.tool.meta.as_deref()),
        codex_apps_meta: tool_info
            .tool
            .meta
            .as_ref()
            .and_then(|meta| meta.get(MCP_TOOL_CODEX_APPS_META_KEY))
            .and_then(serde_json::Value::as_object)
            .cloned(),
        // Disallow custom MCPs from uploading files via fileParams.
        openai_file_input_params: openai_file_input_params_for_server(
            server,
            tool_info.tool.meta.as_deref(),
        ),
    })
}

fn openai_file_input_params_for_server(
    server: &str,
    meta: Option<&serde_json::Map<String, serde_json::Value>>,
) -> Option<Vec<String>> {
    (server == CODEX_APPS_MCP_SERVER_NAME)
        .then_some(declared_openai_file_input_param_names(meta))
        .filter(|params| !params.is_empty())
}

fn get_mcp_app_resource_uri(
    meta: Option<&serde_json::Map<String, serde_json::Value>>,
) -> Option<String> {
    meta.and_then(|meta| {
        meta.get("ui")
            .and_then(serde_json::Value::as_object)
            .and_then(|ui| ui.get("resourceUri"))
            .and_then(serde_json::Value::as_str)
            .or_else(|| {
                meta.get(MCP_TOOL_UI_RESOURCE_URI_META_KEY)
                    .and_then(serde_json::Value::as_str)
            })
            .or_else(|| {
                meta.get(MCP_TOOL_OPENAI_OUTPUT_TEMPLATE_META_KEY)
                    .and_then(serde_json::Value::as_str)
            })
            .map(str::to_string)
    })
}

#[expect(
    clippy::await_holding_invalid_type,
    reason = "MCP app metadata reads through the session-owned manager guard"
)]
async fn lookup_mcp_app_usage_metadata(
    sess: &Session,
    server: &str,
    tool_name: &str,
) -> Option<McpAppUsageMetadata> {
    let tools = sess
        .services
        .mcp_connection_manager
        .read()
        .await
        .list_all_tools()
        .await;

    tools.into_iter().find_map(|tool_info| {
        if tool_info.server_name == server && tool_info.tool.name == tool_name {
            Some(McpAppUsageMetadata {
                connector_id: tool_info.connector_id,
                app_name: tool_info.connector_name,
            })
        } else {
            None
        }
    })
}

fn build_mcp_tool_approval_question(
    question_id: String,
    server: &str,
    tool_name: &str,
    connector_name: Option<&str>,
    prompt_options: McpToolApprovalPromptOptions,
    question_override: Option<&str>,
) -> RequestUserInputQuestion {
    let question = question_override
        .map(ToString::to_string)
        .unwrap_or_else(|| {
            build_mcp_tool_approval_fallback_message(server, tool_name, connector_name)
        });
    let question = format!("{}?", question.trim_end_matches('?'));

    let mut options = vec![RequestUserInputQuestionOption {
        label: MCP_TOOL_APPROVAL_ACCEPT.to_string(),
        description: "Run the tool and continue.".to_string(),
    }];
    if prompt_options.allow_session_remember {
        options.push(RequestUserInputQuestionOption {
            label: MCP_TOOL_APPROVAL_ACCEPT_FOR_SESSION.to_string(),
            description: "Run the tool and remember this choice for this session.".to_string(),
        });
    }
    if prompt_options.allow_persistent_approval {
        options.push(RequestUserInputQuestionOption {
            label: MCP_TOOL_APPROVAL_ACCEPT_AND_REMEMBER.to_string(),
            description: "Run the tool and remember this choice for future tool calls.".to_string(),
        });
    }
    options.push(RequestUserInputQuestionOption {
        label: MCP_TOOL_APPROVAL_CANCEL.to_string(),
        description: "Cancel this tool call.".to_string(),
    });

    RequestUserInputQuestion {
        id: question_id,
        header: "Approve app tool call?".to_string(),
        question,
        is_other: false,
        is_secret: false,
        options: Some(options),
    }
}

fn build_mcp_tool_approval_fallback_message(
    server: &str,
    tool_name: &str,
    connector_name: Option<&str>,
) -> String {
    let actor = connector_name
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| {
            if server == CODEX_APPS_MCP_SERVER_NAME {
                "this app".to_string()
            } else {
                format!("the {server} MCP server")
            }
        });
    format!("Allow {actor} to run tool \"{tool_name}\"?")
}

fn parse_mcp_tool_approval_elicitation_response(
    response: Option<ElicitationResponse>,
    question_id: &str,
) -> McpToolApprovalDecision {
    let Some(response) = response else {
        return McpToolApprovalDecision::Cancel;
    };
    match response.action {
        ElicitationAction::Accept => {
            match response
                .meta
                .as_ref()
                .and_then(serde_json::Value::as_object)
                .and_then(|meta| meta.get(MCP_TOOL_APPROVAL_PERSIST_KEY))
                .and_then(serde_json::Value::as_str)
            {
                Some(MCP_TOOL_APPROVAL_PERSIST_SESSION) => {
                    return McpToolApprovalDecision::AcceptForSession;
                }
                Some(MCP_TOOL_APPROVAL_PERSIST_ALWAYS) => {
                    return McpToolApprovalDecision::AcceptAndRemember;
                }
                _ => {}
            }

            match parse_mcp_tool_approval_response(
                request_user_input_response_from_elicitation_content(response.content),
                question_id,
            ) {
                McpToolApprovalDecision::Cancel => McpToolApprovalDecision::Accept,
                decision => decision,
            }
        }
        ElicitationAction::Decline => McpToolApprovalDecision::Decline { message: None },
        ElicitationAction::Cancel => McpToolApprovalDecision::Cancel,
    }
}

fn request_user_input_response_from_elicitation_content(
    content: Option<serde_json::Value>,
) -> Option<RequestUserInputResponse> {
    let Some(content) = content else {
        return Some(RequestUserInputResponse {
            answers: std::collections::HashMap::new(),
        });
    };
    let content = content.as_object()?;
    let answers = content
        .iter()
        .filter_map(|(question_id, value)| {
            let answers = match value {
                serde_json::Value::String(answer) => vec![answer.clone()],
                serde_json::Value::Array(values) => values
                    .iter()
                    .filter_map(|value| value.as_str().map(ToString::to_string))
                    .collect(),
                _ => return None,
            };
            Some((question_id.clone(), RequestUserInputAnswer { answers }))
        })
        .collect();

    Some(RequestUserInputResponse { answers })
}

fn parse_mcp_tool_approval_response(
    response: Option<RequestUserInputResponse>,
    question_id: &str,
) -> McpToolApprovalDecision {
    let Some(response) = response else {
        return McpToolApprovalDecision::Cancel;
    };
    let answers = response
        .answers
        .get(question_id)
        .map(|answer| answer.answers.as_slice());
    let Some(answers) = answers else {
        return McpToolApprovalDecision::Cancel;
    };
    if answers
        .iter()
        .any(|answer| answer == MCP_TOOL_APPROVAL_DECLINE_SYNTHETIC)
    {
        McpToolApprovalDecision::Decline { message: None }
    } else if answers
        .iter()
        .any(|answer| answer == MCP_TOOL_APPROVAL_ACCEPT_FOR_SESSION)
    {
        McpToolApprovalDecision::AcceptForSession
    } else if answers
        .iter()
        .any(|answer| answer == MCP_TOOL_APPROVAL_ACCEPT_AND_REMEMBER)
    {
        McpToolApprovalDecision::AcceptAndRemember
    } else if answers
        .iter()
        .any(|answer| answer == MCP_TOOL_APPROVAL_ACCEPT)
    {
        McpToolApprovalDecision::Accept
    } else {
        McpToolApprovalDecision::Cancel
    }
}

fn normalize_approval_decision_for_mode(
    decision: McpToolApprovalDecision,
    approval_mode: AppToolApproval,
) -> McpToolApprovalDecision {
    if approval_mode == AppToolApproval::Prompt
        && matches!(
            decision,
            McpToolApprovalDecision::AcceptForSession | McpToolApprovalDecision::AcceptAndRemember
        )
    {
        McpToolApprovalDecision::Accept
    } else {
        decision
    }
}

async fn mcp_tool_approval_is_remembered(sess: &Session, key: &McpToolApprovalKey) -> bool {
    let store = sess.services.tool_approvals.lock().await;
    matches!(store.get(key), Some(ReviewDecision::ApprovedForSession))
}

async fn remember_mcp_tool_approval(sess: &Session, key: McpToolApprovalKey) {
    let mut store = sess.services.tool_approvals.lock().await;
    store.put(key, ReviewDecision::ApprovedForSession);
}

async fn apply_mcp_tool_approval_decision(
    sess: &Session,
    turn_context: &TurnContext,
    decision: &McpToolApprovalDecision,
    session_approval_key: Option<McpToolApprovalKey>,
    persistent_approval_key: Option<McpToolApprovalKey>,
) {
    match decision {
        McpToolApprovalDecision::AcceptForSession => {
            if let Some(key) = session_approval_key {
                remember_mcp_tool_approval(sess, key).await;
            }
        }
        McpToolApprovalDecision::AcceptAndRemember => {
            if let Some(key) = persistent_approval_key {
                maybe_persist_mcp_tool_approval(sess, turn_context, key).await;
            } else if let Some(key) = session_approval_key {
                remember_mcp_tool_approval(sess, key).await;
            }
        }
        McpToolApprovalDecision::Accept
        | McpToolApprovalDecision::Decline { .. }
        | McpToolApprovalDecision::Cancel => {}
    }
}

async fn maybe_persist_mcp_tool_approval(
    sess: &Session,
    turn_context: &TurnContext,
    key: McpToolApprovalKey,
) {
    let tool_name = key.tool_name.clone();

    let persist_result = if key.server == CODEX_APPS_MCP_SERVER_NAME {
        let Some(connector_id) = key.connector_id.clone() else {
            remember_mcp_tool_approval(sess, key).await;
            return;
        };
        persist_codex_app_tool_approval(&turn_context.config, &connector_id, &tool_name).await
    } else {
        persist_non_app_mcp_tool_approval(sess, &turn_context.config, &key.server, &tool_name).await
    };

    if let Err(err) = persist_result {
        error!(
            error = %err,
            server = key.server,
            tool_name,
            "failed to persist MCP tool approval"
        );
        remember_mcp_tool_approval(sess, key).await;
        return;
    }

    sess.reload_user_config_layer().await;
    remember_mcp_tool_approval(sess, key).await;
}

async fn persist_codex_app_tool_approval(
    config: &Config,
    connector_id: &str,
    tool_name: &str,
) -> anyhow::Result<()> {
    ConfigEditsBuilder::for_config(config)
        .with_edits([ConfigEdit::SetPath {
            segments: vec![
                "apps".to_string(),
                connector_id.to_string(),
                "tools".to_string(),
                tool_name.to_string(),
                "approval_mode".to_string(),
            ],
            value: value("approve"),
        }])
        .apply()
        .await
}

#[cfg(test)]
async fn persist_custom_mcp_tool_approval(
    config: &Config,
    server: &str,
    tool_name: &str,
) -> anyhow::Result<()> {
    let Some(config_edits_builder) = custom_mcp_tool_approval_config_builder(config, server)?
    else {
        anyhow::bail!("MCP server `{server}` is not configured in config.toml");
    };

    persist_custom_mcp_tool_approval_with(config_edits_builder, server, tool_name).await
}

async fn persist_non_app_mcp_tool_approval(
    sess: &Session,
    config: &Config,
    server: &str,
    tool_name: &str,
) -> anyhow::Result<()> {
    if let Some(config_edits_builder) = custom_mcp_tool_approval_config_builder(config, server)? {
        return persist_custom_mcp_tool_approval_with(config_edits_builder, server, tool_name)
            .await;
    }

    let plugin_config_name = sess
        .services
        .plugins_manager
        .plugins_for_config(&config.plugins_config_input())
        .await
        .plugins()
        .iter()
        .filter(|plugin| plugin.is_active())
        .find(|plugin| plugin.mcp_servers.contains_key(server))
        .map(|plugin| plugin.config_name.clone());

    if let Some(plugin_config_name) = plugin_config_name {
        return ConfigEditsBuilder::for_config(config)
            .with_edits([ConfigEdit::SetPath {
                segments: vec![
                    "plugins".to_string(),
                    plugin_config_name,
                    "mcp_servers".to_string(),
                    server.to_string(),
                    "tools".to_string(),
                    tool_name.to_string(),
                    "approval_mode".to_string(),
                ],
                value: value("approve"),
            }])
            .apply()
            .await;
    }

    anyhow::bail!("MCP server `{server}` is not configured in config.toml or an enabled plugin")
}

fn custom_mcp_tool_approval_config_builder(
    config: &Config,
    server: &str,
) -> anyhow::Result<Option<ConfigEditsBuilder>> {
    if let Some(project_config_folder) = project_mcp_tool_approval_config_folder(config, server) {
        return Ok(Some(ConfigEditsBuilder::new(&project_config_folder)));
    }

    Ok(user_mcp_server_is_configured(config, server)?
        .then(|| ConfigEditsBuilder::for_config(config)))
}

async fn persist_custom_mcp_tool_approval_with(
    config_edits_builder: ConfigEditsBuilder,
    server: &str,
    tool_name: &str,
) -> anyhow::Result<()> {
    config_edits_builder
        .with_edits([ConfigEdit::SetPath {
            segments: vec![
                "mcp_servers".to_string(),
                server.to_string(),
                "tools".to_string(),
                tool_name.to_string(),
                "approval_mode".to_string(),
            ],
            value: value("approve"),
        }])
        .apply()
        .await
}

fn user_mcp_server_is_configured(config: &Config, server: &str) -> anyhow::Result<bool> {
    let Some(mcp_servers_toml) = config
        .config_layer_stack
        .effective_user_config()
        .as_ref()
        .and_then(|user_config| user_config.get("mcp_servers"))
        .cloned()
    else {
        return Ok(false);
    };
    let servers = HashMap::<String, ontocode_config::types::McpServerConfig>::deserialize(
        mcp_servers_toml,
    )
    .context("failed to parse `mcp_servers` configuration in config.toml; please check for invalid server names or malformed transport settings")?;
    Ok(servers.contains_key(server))
}

fn project_mcp_tool_approval_config_folder(
    config: &Config,
    server: &str,
) -> Option<AbsolutePathBuf> {
    config
        .config_layer_stack
        .layers_high_to_low()
        .into_iter()
        .find_map(|layer| {
            if !matches!(layer.name, ConfigLayerSource::Project { .. }) {
                return None;
            }

            let servers = layer
                .config
                .as_table()
                .and_then(|table| table.get("mcp_servers"))
                .cloned()
                .and_then(|value| {
                    HashMap::<String, ontocode_config::types::McpServerConfig>::deserialize(value)
                        .ok()
                })?;
            if servers.contains_key(server) {
                layer.config_folder()
            } else {
                None
            }
        })
}

fn requires_mcp_tool_approval(annotations: Option<&ToolAnnotations>) -> bool {
    let destructive_hint = annotations.and_then(|annotations| annotations.destructive_hint);
    if destructive_hint == Some(true) {
        return true;
    }

    let read_only_hint = annotations
        .and_then(|annotations| annotations.read_only_hint)
        .unwrap_or(false);
    if read_only_hint {
        return false;
    }

    destructive_hint.unwrap_or(true)
        || annotations
            .and_then(|annotations| annotations.open_world_hint)
            .unwrap_or(true)
}

async fn notify_mcp_tool_call_skip(
    sess: &Session,
    turn_context: &TurnContext,
    call_id: &str,
    invocation: McpInvocation,
    item_metadata: McpToolCallItemMetadata,
    message: String,
    already_started: bool,
) -> Result<CallToolResult, String> {
    if !already_started {
        notify_mcp_tool_call_started(
            sess,
            turn_context,
            call_id,
            invocation.clone(),
            item_metadata.clone(),
        )
        .await;
    }

    notify_mcp_tool_call_completed(
        sess,
        turn_context,
        call_id,
        invocation,
        item_metadata,
        Duration::ZERO,
        truncate_mcp_tool_result_for_event(&Err(message.clone())),
    )
    .await;
    Err(message)
}

#[cfg(test)]
#[path = "mcp_tool_call_tests.rs"]
mod tests;
