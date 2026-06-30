use std::sync::Arc;
use std::sync::Weak;

use ontocode_app_server_protocol::ServerNotification;
use ontocode_app_server_protocol::ThreadGoalUpdatedNotification;
use ontocode_core::NewThread;
use ontocode_core::StartThreadOptions;
use ontocode_core::ThreadManager;
use ontocode_core::config::Config;
use ontocode_extension_api::AgentSpawnFuture;
use ontocode_extension_api::AgentSpawner;
use ontocode_extension_api::ExtensionEventSink;
use ontocode_extension_api::ExtensionRegistry;
use ontocode_extension_api::ExtensionRegistryBuilder;
use ontocode_login::AuthManager;
use ontocode_protocol::ThreadId;
use ontocode_protocol::error::CodexErr;
use ontocode_protocol::protocol::Event;
use ontocode_protocol::protocol::EventMsg;

use crate::outgoing_message::OutgoingMessageSender;

pub(crate) fn thread_extensions<S>(
    guardian_agent_spawner: S,
    event_sink: Arc<dyn ExtensionEventSink>,
    auth_manager: Arc<AuthManager>,
) -> Arc<ExtensionRegistry<Config>>
where
    S: AgentSpawner<StartThreadOptions, Spawned = NewThread, Error = CodexErr> + 'static,
{
    let mut builder = ExtensionRegistryBuilder::<Config>::with_event_sink(event_sink);
    ontocode_guardian::install(&mut builder, guardian_agent_spawner);
    ontocode_excel_extension::install(&mut builder);
    ontocode_lctx_extension::install(&mut builder);
    ontocode_memories_extension::install(&mut builder, ontocode_otel::global());
    ontocode_ontograph_extension::install(&mut builder);
    ontocode_web_search_extension::install(&mut builder, auth_manager.clone());
    ontocode_image_generation_extension::install(&mut builder, auth_manager);
    Arc::new(builder.build())
}

pub(crate) fn app_server_extension_event_sink(
    outgoing: Arc<OutgoingMessageSender>,
) -> Arc<dyn ExtensionEventSink> {
    Arc::new(AppServerExtensionEventSink { outgoing })
}

struct AppServerExtensionEventSink {
    outgoing: Arc<OutgoingMessageSender>,
}

impl ExtensionEventSink for AppServerExtensionEventSink {
    fn emit(&self, event: Event) {
        match event.msg {
            EventMsg::ThreadGoalUpdated(thread_goal_event) => {
                self.outgoing
                    .try_send_server_notification(ServerNotification::ThreadGoalUpdated(
                        ThreadGoalUpdatedNotification {
                            thread_id: thread_goal_event.thread_id.to_string(),
                            turn_id: thread_goal_event.turn_id,
                            goal: thread_goal_event.goal.into(),
                        },
                    ));
            }
            msg => {
                tracing::debug!(event_id = %event.id, ?msg, "dropping unsupported extension event");
            }
        }
    }
}

pub(crate) fn guardian_agent_spawner(
    thread_manager: Weak<ThreadManager>,
) -> impl AgentSpawner<StartThreadOptions, Spawned = NewThread, Error = CodexErr> {
    move |forked_from_thread_id: ThreadId,
          options: StartThreadOptions|
          -> AgentSpawnFuture<'static, NewThread, CodexErr> {
        let thread_manager = thread_manager.clone();
        Box::pin(async move {
            let thread_manager = thread_manager.upgrade().ok_or_else(|| {
                CodexErr::UnsupportedOperation("thread manager dropped".to_string())
            })?;
            thread_manager
                .spawn_subagent(forked_from_thread_id, options)
                .await
        })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ontocode_analytics::AnalyticsEventsClient;
    use ontocode_app_server_protocol::ServerNotification;
    use ontocode_app_server_protocol::ThreadGoal as AppServerThreadGoal;
    use ontocode_app_server_protocol::ThreadGoalStatus as AppServerThreadGoalStatus;
    use ontocode_extension_api::ExtensionData;
    use ontocode_extension_api::ToolName;
    use ontocode_login::AuthManager;
    use ontocode_login::CodexAuth;
    use ontocode_protocol::protocol::ThreadGoal;
    use ontocode_protocol::protocol::ThreadGoalStatus;
    use ontocode_protocol::protocol::ThreadGoalUpdatedEvent;
    use pretty_assertions::assert_eq;
    use tokio::sync::mpsc;
    use tokio::time::timeout;

    use super::*;
    use crate::outgoing_message::OutgoingEnvelope;
    use crate::outgoing_message::OutgoingMessage;

    struct NoopTestEventSink;

    impl ExtensionEventSink for NoopTestEventSink {
        fn emit(&self, _event: Event) {}
    }

    fn unused_test_spawner(
        _forked_from_thread_id: ThreadId,
        _options: StartThreadOptions,
    ) -> AgentSpawnFuture<'static, NewThread, CodexErr> {
        Box::pin(async { panic!("spawner not used") })
    }

    #[test]
    fn thread_extensions_register_ontograph_tool() {
        let registry = thread_extensions(
            unused_test_spawner,
            Arc::new(NoopTestEventSink),
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("dummy")),
        );

        assert_eq!(
            namespace_tool_names(&registry, "ontograph"),
            vec![
                ToolName::namespaced("ontograph", "discover"),
                ToolName::namespaced("ontograph", "explain_module"),
                ToolName::namespaced("ontograph", "impact"),
                ToolName::namespaced("ontograph", "inspect"),
                ToolName::namespaced("ontograph", "search"),
            ]
        );
    }

    #[test]
    fn thread_extensions_register_lctx_tool() {
        let registry = thread_extensions(
            unused_test_spawner,
            Arc::new(NoopTestEventSink),
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("dummy")),
        );

        assert_eq!(
            namespace_tool_names(&registry, "lctx"),
            vec![ToolName::namespaced("lctx", "read")]
        );
    }

    #[test]
    fn thread_extensions_register_excel_tool() {
        let registry = thread_extensions(
            unused_test_spawner,
            Arc::new(NoopTestEventSink),
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("dummy")),
        );

        let mut excel_builder = ExtensionRegistryBuilder::<Config>::new();
        ontocode_excel_extension::install(&mut excel_builder);
        let excel_registry = excel_builder.build();

        assert_eq!(
            namespace_tool_names(&registry, "excel"),
            namespace_tool_names(&excel_registry, "excel")
        );
    }

    fn namespace_tool_names(
        registry: &ExtensionRegistry<Config>,
        namespace: &str,
    ) -> Vec<ToolName> {
        registry
            .tool_contributors()
            .iter()
            .flat_map(|contributor| {
                contributor.tools(
                    &ExtensionData::new("session"),
                    &ExtensionData::new("thread"),
                )
            })
            .map(|tool| tool.tool_name())
            .filter(|tool_name| tool_name.namespace.as_deref() == Some(namespace))
            .collect()
    }

    #[tokio::test]
    async fn app_server_event_sink_forwards_thread_goal_updates() {
        let (outgoing_tx, mut outgoing_rx) = mpsc::channel(4);
        let outgoing = Arc::new(OutgoingMessageSender::new(
            outgoing_tx,
            AnalyticsEventsClient::disabled(),
        ));
        let sink = app_server_extension_event_sink(outgoing);
        let thread_id = ThreadId::default();

        sink.emit(Event {
            id: "call-1".to_string(),
            msg: EventMsg::ThreadGoalUpdated(ThreadGoalUpdatedEvent {
                thread_id,
                turn_id: Some("turn-1".to_string()),
                goal: ThreadGoal {
                    thread_id,
                    objective: "wire extension events".to_string(),
                    status: ThreadGoalStatus::Active,
                    token_budget: Some(123),
                    tokens_used: 45,
                    time_used_seconds: 6,
                    created_at: 7,
                    updated_at: 8,
                },
            }),
        });

        let envelope = timeout(Duration::from_secs(1), outgoing_rx.recv())
            .await
            .expect("timed out waiting for forwarded extension event")
            .expect("outgoing channel closed unexpectedly");
        let OutgoingEnvelope::Broadcast { message } = envelope else {
            panic!("expected broadcast notification");
        };
        let OutgoingMessage::AppServerNotification(ServerNotification::ThreadGoalUpdated(
            notification,
        )) = message
        else {
            panic!("expected thread goal updated notification");
        };

        assert_eq!(
            ThreadGoalUpdatedNotification {
                thread_id: thread_id.to_string(),
                turn_id: Some("turn-1".to_string()),
                goal: AppServerThreadGoal {
                    thread_id: thread_id.to_string(),
                    objective: "wire extension events".to_string(),
                    status: AppServerThreadGoalStatus::Active,
                    token_budget: Some(123),
                    tokens_used: 45,
                    time_used_seconds: 6,
                    created_at: 7,
                    updated_at: 8,
                },
            },
            notification
        );
    }
}
