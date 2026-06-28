use std::sync::Arc;

use crate::agent::exceeds_thread_spawn_depth_limit;
use crate::agent::next_thread_spawn_depth;
use crate::session::turn_context::TurnContext;
use crate::tools::context::ToolInvocation;
use crate::tools::handlers::ApplyPatchHandler;
use crate::tools::handlers::CreateGoalHandler;
use crate::tools::handlers::ExecCommandHandler;
use crate::tools::handlers::ExecCommandHandlerOptions;
use crate::tools::handlers::GetGoalHandler;
use crate::tools::handlers::ListAvailablePluginsToInstallHandler;
use crate::tools::handlers::ManagerLoopNextHandler;
use crate::tools::handlers::PlanHandler;
use crate::tools::handlers::RequestPermissionsHandler;
use crate::tools::handlers::RequestPluginInstallHandler;
use crate::tools::handlers::RequestUserInputHandler;
use crate::tools::handlers::ShellCommandHandler;
use crate::tools::handlers::ShellCommandHandlerOptions;
use crate::tools::handlers::TestSyncHandler;
use crate::tools::handlers::UpdateGoalHandler;
use crate::tools::handlers::ViewImageHandler;
use crate::tools::handlers::WriteStdinHandler;
use crate::tools::handlers::agent_jobs::ReportAgentJobResultHandler;
use crate::tools::handlers::agent_jobs::SpawnAgentsOnCsvHandler;
use crate::tools::handlers::multi_agents::CloseAgentHandler;
use crate::tools::handlers::multi_agents::ResumeAgentHandler;
use crate::tools::handlers::multi_agents::SendInputHandler;
use crate::tools::handlers::multi_agents::SpawnAgentHandler;
use crate::tools::handlers::multi_agents::WaitAgentHandler;
use crate::tools::handlers::multi_agents_common::DEFAULT_WAIT_TIMEOUT_MS;
use crate::tools::handlers::multi_agents_common::MAX_WAIT_TIMEOUT_MS;
use crate::tools::handlers::multi_agents_common::MIN_WAIT_TIMEOUT_MS;
use crate::tools::handlers::multi_agents_spec::SpawnAgentToolOptions;
use crate::tools::handlers::multi_agents_spec::WaitAgentTimeoutOptions;
use crate::tools::handlers::multi_agents_v2::CloseAgentHandler as CloseAgentHandlerV2;
use crate::tools::handlers::multi_agents_v2::FollowupTaskHandler as FollowupTaskHandlerV2;
use crate::tools::handlers::multi_agents_v2::ListAgentsHandler as ListAgentsHandlerV2;
use crate::tools::handlers::multi_agents_v2::SendMessageHandler as SendMessageHandlerV2;
use crate::tools::handlers::multi_agents_v2::SpawnAgentHandler as SpawnAgentHandlerV2;
use crate::tools::handlers::multi_agents_v2::WaitAgentHandler as WaitAgentHandlerV2;
use crate::tools::handlers::view_image_spec::ViewImageToolOptions;
use crate::tools::planning::CoreToolPlanContext;
use crate::tools::planning::PlannedTools;
use crate::tools::registry::CoreToolRuntime;
use ontocode_features::Feature;
use ontocode_protocol::openai_models::ConfigShellToolType;
use ontocode_protocol::openai_models::ToolMode;
use ontocode_protocol::protocol::MultiAgentVersion;
use ontocode_protocol::protocol::SessionSource;
use ontocode_protocol::protocol::SubAgentSource;
use ontocode_tools::ResponsesApiNamespace;
use ontocode_tools::ResponsesApiNamespaceTool;
use ontocode_tools::ToolEnvironmentMode;
use ontocode_tools::ToolName;
use ontocode_tools::ToolOutput;
use ontocode_tools::ToolSearchInfo;
use ontocode_tools::ToolSpec;
use ontocode_tools::UnifiedExecShellMode;
use ontocode_tools::can_request_original_image_detail;
use ontocode_tools::collect_request_plugin_install_entries;
use ontocode_tools::request_user_input_available_modes;
use ontocode_tools::shell_command_backend_for_features;
use ontocode_tools::shell_type_for_model_and_features;

pub(crate) fn add_native_tools(
    context: &CoreToolPlanContext<'_>,
    planned_tools: &mut PlannedTools,
) {
    add_shell_tools(context, planned_tools);
    add_core_utility_tools(context, planned_tools);
    add_collaboration_tools(context, planned_tools);
}

fn namespace_tools_enabled(turn_context: &TurnContext) -> bool {
    turn_context.provider.capabilities().namespace_tools
}

fn tool_suggest_enabled(turn_context: &TurnContext) -> bool {
    let features = turn_context.features.get();
    features.enabled(Feature::ToolSuggest)
        && features.enabled(Feature::Apps)
        && features.enabled(Feature::Plugins)
}

fn multi_agent_v2_enabled(turn_context: &TurnContext) -> bool {
    turn_context.multi_agent_version == MultiAgentVersion::V2
}

fn collab_tools_enabled(turn_context: &TurnContext) -> bool {
    match turn_context.multi_agent_version {
        MultiAgentVersion::Disabled => false,
        MultiAgentVersion::V1 => !exceeds_thread_spawn_depth_limit(
            next_thread_spawn_depth(&turn_context.session_source),
            turn_context.config.agent_max_depth,
        ),
        MultiAgentVersion::V2 => true,
    }
}

fn goal_tools_enabled(turn_context: &TurnContext) -> bool {
    turn_context.goal_tools_enabled()
        && !matches!(
            turn_context.session_source,
            SessionSource::SubAgent(SubAgentSource::Review)
        )
}

fn agent_jobs_tools_enabled(turn_context: &TurnContext) -> bool {
    turn_context.features.get().enabled(Feature::SpawnCsv) && collab_tools_enabled(turn_context)
}

fn agent_jobs_worker_tools_enabled(turn_context: &TurnContext) -> bool {
    agent_jobs_tools_enabled(turn_context)
        && matches!(
            &turn_context.session_source,
            SessionSource::SubAgent(SubAgentSource::Other(label)) if label.starts_with("agent_job:")
        )
}

pub(crate) fn wait_agent_timeout_options(turn_context: &TurnContext) -> WaitAgentTimeoutOptions {
    if multi_agent_v2_enabled(turn_context) {
        return WaitAgentTimeoutOptions {
            default_timeout_ms: turn_context.config.multi_agent_v2.default_wait_timeout_ms,
            min_timeout_ms: turn_context.config.multi_agent_v2.min_wait_timeout_ms,
            max_timeout_ms: turn_context.config.multi_agent_v2.max_wait_timeout_ms,
        };
    }

    WaitAgentTimeoutOptions {
        default_timeout_ms: DEFAULT_WAIT_TIMEOUT_MS,
        min_timeout_ms: MIN_WAIT_TIMEOUT_MS,
        max_timeout_ms: MAX_WAIT_TIMEOUT_MS,
    }
}

fn max_concurrent_threads_per_session(turn_context: &TurnContext) -> Option<usize> {
    multi_agent_v2_enabled(turn_context).then_some(
        turn_context
            .config
            .multi_agent_v2
            .max_concurrent_threads_per_session,
    )
}

fn agent_type_description(
    turn_context: &TurnContext,
    default_agent_type_description: &str,
) -> String {
    let agent_type_description =
        crate::agent::role::spawn_tool_spec::build(&turn_context.config.agent_roles);
    if agent_type_description.is_empty() {
        default_agent_type_description.to_string()
    } else {
        agent_type_description
    }
}

fn add_shell_tools(context: &CoreToolPlanContext<'_>, planned_tools: &mut PlannedTools) {
    let turn_context = context.turn_context;
    let features = turn_context.features.get();
    let environment_mode = turn_context.tool_environment_mode();
    if !environment_mode.has_environment() {
        return;
    }

    let allow_login_shell = turn_context.config.permissions.allow_login_shell;
    let exec_permission_approvals_enabled = features.enabled(Feature::ExecPermissionApprovals);
    let include_environment_id = matches!(environment_mode, ToolEnvironmentMode::Multiple);
    let shell_command_options = ShellCommandHandlerOptions {
        backend_config: shell_command_backend_for_features(features),
        allow_login_shell,
        exec_permission_approvals_enabled,
    };

    match shell_type_for_model_and_features(&turn_context.model_info, features) {
        ConfigShellToolType::UnifiedExec => {
            planned_tools.add(ExecCommandHandler::new(ExecCommandHandlerOptions {
                allow_login_shell,
                exec_permission_approvals_enabled,
                include_environment_id,
                include_shell_parameter: unified_exec_should_include_shell_parameter(turn_context),
            }));
            planned_tools.add(WriteStdinHandler);
            planned_tools.add_dispatch_only(ShellCommandHandler::new(shell_command_options));
        }
        ConfigShellToolType::Disabled => {}
        ConfigShellToolType::Default
        | ConfigShellToolType::Local
        | ConfigShellToolType::ShellCommand => {
            planned_tools.add(ShellCommandHandler::new(shell_command_options));
        }
    }
}

fn unified_exec_should_include_shell_parameter(turn_context: &TurnContext) -> bool {
    !matches!(
        &turn_context.unified_exec_shell_mode,
        UnifiedExecShellMode::ZshFork(_)
    ) || turn_context
        .environments
        .turn_environments
        .iter()
        .any(|environment| environment.environment.is_remote())
}

fn add_core_utility_tools(context: &CoreToolPlanContext<'_>, planned_tools: &mut PlannedTools) {
    let turn_context = context.turn_context;
    let features = turn_context.features.get();
    let environment_mode = turn_context.tool_environment_mode();

    planned_tools.add(PlanHandler);
    if environment_mode.has_environment() {
        planned_tools.add(ManagerLoopNextHandler);
    }
    if goal_tools_enabled(turn_context) {
        planned_tools.add(GetGoalHandler);
        planned_tools.add(CreateGoalHandler);
        planned_tools.add(UpdateGoalHandler);
    }

    if turn_context.config.experimental_request_user_input_enabled {
        planned_tools.add(RequestUserInputHandler {
            available_modes: request_user_input_available_modes(features),
        });
    }

    if features.enabled(Feature::RequestPermissionsTool) {
        planned_tools.add(RequestPermissionsHandler);
    }

    if tool_suggest_enabled(turn_context)
        && let Some(discoverable_tools) =
            context.discoverable_tools.filter(|tools| !tools.is_empty())
    {
        planned_tools.add(ListAvailablePluginsToInstallHandler::new(
            collect_request_plugin_install_entries(discoverable_tools),
        ));
        planned_tools.add(RequestPluginInstallHandler::new(
            discoverable_tools.to_vec(),
        ));
    }

    if environment_mode.has_environment() && turn_context.model_info.apply_patch_tool_type.is_some()
    {
        let include_environment_id = matches!(environment_mode, ToolEnvironmentMode::Multiple);
        planned_tools.add(ApplyPatchHandler::new(include_environment_id));
    }

    if turn_context
        .model_info
        .experimental_supported_tools
        .iter()
        .any(|tool| tool == "test_sync_tool")
    {
        planned_tools.add(TestSyncHandler);
    }

    if environment_mode.has_environment() {
        let include_environment_id = matches!(environment_mode, ToolEnvironmentMode::Multiple);
        planned_tools.add(ViewImageHandler::new(ViewImageToolOptions {
            can_request_original_image_detail: can_request_original_image_detail(
                &turn_context.model_info,
            ),
            include_environment_id,
        }));
    }
}

fn add_collaboration_tools(context: &CoreToolPlanContext<'_>, planned_tools: &mut PlannedTools) {
    let turn_context = context.turn_context;
    if collab_tools_enabled(turn_context) {
        if multi_agent_v2_enabled(turn_context) {
            let exposure = if turn_context.config.multi_agent_v2.non_code_mode_only {
                crate::tools::registry::ToolExposure::DirectModelOnly
            } else {
                crate::tools::registry::ToolExposure::Direct
            };
            let tool_namespace = namespace_tools_enabled(turn_context)
                .then_some(turn_context.config.multi_agent_v2.tool_namespace.as_deref())
                .flatten();
            let agent_type_description =
                agent_type_description(turn_context, context.default_agent_type_description);
            let hide_spawn_agent_for_coding_subagent =
                matches!(
                    turn_context.tool_mode,
                    ToolMode::CodeMode | ToolMode::CodeModeOnly
                ) && matches!(turn_context.session_source, SessionSource::SubAgent(_));
            if !hide_spawn_agent_for_coding_subagent {
                planned_tools.add_arc(crate::tools::registry::override_tool_exposure(
                    multi_agent_v2_handler(
                        SpawnAgentHandlerV2::new(SpawnAgentToolOptions {
                            available_models: turn_context.available_models.clone(),
                            agent_type_description,
                            hide_agent_type_model_reasoning: turn_context
                                .config
                                .multi_agent_v2
                                .hide_spawn_agent_metadata,
                            include_usage_hint: turn_context
                                .config
                                .multi_agent_v2
                                .usage_hint_enabled,
                            usage_hint_text: turn_context
                                .config
                                .multi_agent_v2
                                .usage_hint_text
                                .clone(),
                            max_concurrent_threads_per_session: max_concurrent_threads_per_session(
                                turn_context,
                            ),
                        }),
                        tool_namespace,
                    ),
                    exposure,
                ));
            }
            planned_tools.add_arc(crate::tools::registry::override_tool_exposure(
                multi_agent_v2_handler(SendMessageHandlerV2, tool_namespace),
                exposure,
            ));
            planned_tools.add_arc(crate::tools::registry::override_tool_exposure(
                multi_agent_v2_handler(FollowupTaskHandlerV2, tool_namespace),
                exposure,
            ));
            planned_tools.add_arc(crate::tools::registry::override_tool_exposure(
                multi_agent_v2_handler(
                    WaitAgentHandlerV2::new(context.wait_agent_timeouts),
                    tool_namespace,
                ),
                exposure,
            ));
            planned_tools.add_arc(crate::tools::registry::override_tool_exposure(
                multi_agent_v2_handler(CloseAgentHandlerV2, tool_namespace),
                exposure,
            ));
            planned_tools.add_arc(crate::tools::registry::override_tool_exposure(
                multi_agent_v2_handler(ListAgentsHandlerV2, tool_namespace),
                exposure,
            ));
        } else {
            let agent_type_description =
                agent_type_description(turn_context, context.default_agent_type_description);
            let exposure = if crate::tools::spec_plan::search_tool_enabled(turn_context)
                && namespace_tools_enabled(turn_context)
            {
                crate::tools::registry::ToolExposure::Deferred
            } else {
                crate::tools::registry::ToolExposure::Direct
            };
            planned_tools.add_with_exposure(
                SpawnAgentHandler::new(SpawnAgentToolOptions {
                    available_models: turn_context.available_models.clone(),
                    agent_type_description,
                    hide_agent_type_model_reasoning: turn_context
                        .config
                        .multi_agent_v2
                        .hide_spawn_agent_metadata,
                    include_usage_hint: turn_context.config.multi_agent_v2.usage_hint_enabled,
                    usage_hint_text: turn_context.config.multi_agent_v2.usage_hint_text.clone(),
                    max_concurrent_threads_per_session: max_concurrent_threads_per_session(
                        turn_context,
                    ),
                }),
                exposure,
            );
            planned_tools.add_with_exposure(SendInputHandler, exposure);
            planned_tools.add_with_exposure(ResumeAgentHandler, exposure);
            planned_tools
                .add_with_exposure(WaitAgentHandler::new(context.wait_agent_timeouts), exposure);
            planned_tools.add_with_exposure(CloseAgentHandler, exposure);
        }
    }

    if agent_jobs_tools_enabled(turn_context) {
        planned_tools.add(SpawnAgentsOnCsvHandler);
        if agent_jobs_worker_tools_enabled(turn_context) {
            planned_tools.add(ReportAgentJobResultHandler);
        }
    }
}

fn multi_agent_v2_handler(
    handler: impl CoreToolRuntime + 'static,
    namespace: Option<&str>,
) -> Arc<dyn CoreToolRuntime> {
    match namespace {
        Some(namespace) => Arc::new(MultiAgentV2NamespaceOverride {
            handler: Arc::new(handler),
            namespace: namespace.to_string(),
        }),
        None => Arc::new(handler),
    }
}

struct MultiAgentV2NamespaceOverride {
    handler: Arc<dyn CoreToolRuntime>,
    namespace: String,
}

#[async_trait::async_trait]
impl ontocode_tools::ToolExecutor<ToolInvocation> for MultiAgentV2NamespaceOverride {
    fn tool_name(&self) -> ToolName {
        ToolName::namespaced(self.namespace.clone(), self.handler.tool_name().name)
    }

    fn spec(&self) -> ToolSpec {
        match self.handler.spec() {
            ToolSpec::Function(tool) => ToolSpec::Namespace(ResponsesApiNamespace {
                name: self.namespace.clone(),
                description: "Tools for spawning and managing sub-agents.".to_string(),
                tools: vec![ResponsesApiNamespaceTool::Function(tool)],
            }),
            spec => spec,
        }
    }

    fn exposure(&self) -> crate::tools::registry::ToolExposure {
        self.handler.exposure()
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        self.handler.supports_parallel_tool_calls()
    }

    fn search_info(&self) -> Option<ToolSearchInfo> {
        self.handler.search_info()
    }

    async fn handle(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, ontocode_tools::FunctionCallError> {
        self.handler.handle(invocation).await
    }
}

impl CoreToolRuntime for MultiAgentV2NamespaceOverride {
    fn matches_kind(&self, payload: &crate::tools::context::ToolPayload) -> bool {
        self.handler.matches_kind(payload)
    }

    fn create_diff_consumer(
        &self,
    ) -> Option<Box<dyn crate::tools::registry::ToolArgumentDiffConsumer>> {
        self.handler.create_diff_consumer()
    }
}
