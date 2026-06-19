# Ontocode Internal Crate Rename Cargo Inventory

Generated: 2026-06-09

## Scope

- Source of truth: `cargo metadata --format-version 1` run from `ontocode-rs`.
- Raw metadata: `ontocode_internal_crate_rename-cargo-metadata.json`.
- Included packages: every workspace Cargo package whose package name starts with `codex-`, plus any workspace package whose lib target starts with `codex_`.
- Stage 0 only: no implementation or rename proposal beyond preliminary inventory classification.

## Summary

- Workspace packages in metadata: 123
- R0A inventory packages: 120
- CLI/app: 8
- core/shared: 31
- deferred: 18
- helper/tool: 8
- leaf utility: 24
- protocol/generated: 5
- provider/auth/MCP: 17
- runtime-path/package-layout: 9

## First Three Safest Candidate Crates

Based only on Cargo inventory, the safest candidates are leaf utility crates with no bin target and zero or one direct `codex-*` workspace dependency.

| Rank | Package | Path | Lib crate | Dependency summary | Direct codex workspace deps |
| --- | --- | --- | --- | --- | --- |
| 1 | `codex-ansi-escape` | `ontocode-rs/ansi-escape/Cargo.toml` | `codex_ansi_escape` | 3 total; 0 codex workspace deps | `-` |
| 2 | `codex-async-utils` | `ontocode-rs/async-utils/Cargo.toml` | `codex_async_utils` | 4 total; 0 codex workspace deps | `-` |
| 3 | `codex-utils-absolute-path` | `ontocode-rs/utils/absolute-path/Cargo.toml` | `codex_utils_absolute_path` | 8 total; 0 codex workspace deps | `-` |

## Inventory

| Family | Path | Package | Lib crate name | Bin names | Dependency summary | Direct codex workspace deps |
| --- | --- | --- | --- | --- | --- | --- |
| protocol/generated | `ontocode-rs/adapter-protocol/Cargo.toml` | `codex-adapter-protocol` | `codex_adapter_protocol` | `-` | 4 total; 0 codex workspace deps | - |
| deferred | `ontocode-rs/agent-graph-store/Cargo.toml` | `codex-agent-graph-store` | `codex_agent_graph_store` | `-` | 9 total; 2 codex workspace deps | `codex-protocol, codex-state` |
| deferred | `ontocode-rs/agent-identity/Cargo.toml` | `codex-agent-identity` | `codex_agent_identity` | `-` | 13 total; 1 codex workspace deps | `codex-protocol` |
| core/shared | `ontocode-rs/analytics/Cargo.toml` | `codex-analytics` | `codex_analytics` | `-` | 14 total; 7 codex workspace deps | `ontocode-app-server-protocol, codex-git-utils, codex-login, codex-model-provider, codex-plugin, codex-protocol, codex-utils-absolute-path` |
| leaf utility | `ontocode-rs/ansi-escape/Cargo.toml` | `codex-ansi-escape` | `codex_ansi_escape` | `-` | 3 total; 0 codex workspace deps | - |
| core/shared | `ontocode-rs/codex-api/Cargo.toml` | `codex-api` | `codex_api` | `-` | 30 total; 3 codex workspace deps | `codex-client, codex-protocol, codex-utils-rustls-provider` |
| CLI/app | `ontocode-rs/app-server/Cargo.toml` | `ontocode-app-server` | `codex_app_server` | `ontocode-app-server, ontocode-app-server-test-notify-capture, test_notify_capture` | 84 total; 46 codex workspace deps | `codex-analytics, ontocode-app-server-protocol, ontocode-app-server-transport, codex-arg0, codex-backend-client, codex-chatgpt, codex-cloud-config, codex-config, +38 more` |
| CLI/app | `ontocode-rs/app-server-client/Cargo.toml` | `ontocode-app-server-client` | `codex_app_server_client` | `-` | 23 total; 11 codex workspace deps | `ontocode-app-server, ontocode-app-server-protocol, codex-arg0, codex-config, codex-core, ontocode-exec-server, codex-feedback, codex-protocol, +3 more` |
| CLI/app | `ontocode-rs/app-server-daemon/Cargo.toml` | `ontocode-app-server-daemon` | `codex_app_server_daemon` | `-` | 15 total; 4 codex workspace deps | `ontocode-app-server-protocol, ontocode-app-server-transport, codex-uds, codex-utils-home-dir` |
| protocol/generated | `ontocode-rs/app-server-protocol/Cargo.toml` | `ontocode-app-server-protocol` | `codex_app_server_protocol` | `export, write_schema_fixtures` | 22 total; 5 codex workspace deps | `codex-experimental-api-macros, codex-protocol, codex-shell-command, codex-utils-absolute-path, codex-utils-cargo-bin` |
| CLI/app | `ontocode-rs/app-server-test-client/Cargo.toml` | `ontocode-app-server-test-client` | `codex_app_server_test_client` | `ontocode-app-server-test-client` | 15 total; 5 codex workspace deps | `ontocode-app-server-protocol, codex-core, codex-otel, codex-protocol, codex-utils-cli` |
| CLI/app | `ontocode-rs/app-server-transport/Cargo.toml` | `ontocode-app-server-transport` | `codex_app_server_transport` | `-` | 33 total; 10 codex workspace deps | `codex-api, ontocode-app-server-protocol, codex-config, codex-core, codex-login, codex-model-provider, codex-state, codex-uds, +2 more` |
| helper/tool | `ontocode-rs/apply-patch/Cargo.toml` | `codex-apply-patch` | `codex_apply_patch` | `apply_patch` | 13 total; 3 codex workspace deps | `ontocode-exec-server, codex-utils-absolute-path, codex-utils-cargo-bin` |
| runtime-path/package-layout | `ontocode-rs/arg0/Cargo.toml` | `codex-arg0` | `codex_arg0` | `-` | 13 total; 8 codex workspace deps | `codex-apply-patch, ontocode-exec-server, codex-install-context, codex-linux-sandbox, codex-sandboxing, codex-shell-escalation, codex-utils-absolute-path, codex-utils-home-dir` |
| leaf utility | `ontocode-rs/async-utils/Cargo.toml` | `codex-async-utils` | `codex_async_utils` | `-` | 4 total; 0 codex workspace deps | - |
| provider/auth/MCP | `ontocode-rs/aws-auth/Cargo.toml` | `codex-aws-auth` | `codex_aws_auth` | `-` | 9 total; 0 codex workspace deps | - |
| core/shared | `ontocode-rs/backend-client/Cargo.toml` | `codex-backend-client` | `codex_backend_client` | `-` | 11 total; 6 codex workspace deps | `codex-api, codex-backend-openapi-models, codex-client, codex-login, codex-model-provider, codex-protocol` |
| protocol/generated | `ontocode-rs/codex-backend-openapi-models/Cargo.toml` | `codex-backend-openapi-models` | `codex_backend_openapi_models` | `-` | 3 total; 0 codex workspace deps | - |
| helper/tool | `ontocode-rs/bwrap/Cargo.toml` | `codex-bwrap` | `-` | `bwrap` | 3 total; 0 codex workspace deps | - |
| provider/auth/MCP | `ontocode-rs/chatgpt/Cargo.toml` | `codex-chatgpt` | `codex_chatgpt` | `-` | 17 total; 10 codex workspace deps | `ontocode-app-server-protocol, codex-connectors, codex-core, codex-core-plugins, codex-git-utils, codex-login, codex-model-provider, codex-plugin, +2 more` |
| CLI/app | `ontocode-rs/cli/Cargo.toml` | `codex-cli` | `codex_cli` | `ontocode` | 68 total; 39 codex workspace deps | `codex-api, ontocode-app-server, ontocode-app-server-daemon, ontocode-app-server-protocol, ontocode-app-server-test-client, codex-arg0, codex-chatgpt, codex-cloud-tasks, +31 more` |
| core/shared | `ontocode-rs/codex-client/Cargo.toml` | `codex-client` | `codex_client` | `custom_ca_probe` | 25 total; 2 codex workspace deps | `codex-utils-cargo-bin, codex-utils-rustls-provider` |
| deferred | `ontocode-rs/cloud-config/Cargo.toml` | `codex-cloud-config` | `codex_cloud_config` | `-` | 18 total; 6 codex workspace deps | `codex-backend-client, codex-config, codex-core, codex-login, codex-otel, codex-protocol` |
| deferred | `ontocode-rs/cloud-tasks/Cargo.toml` | `codex-cloud-tasks` | `codex_cloud_tasks` | `-` | 26 total; 9 codex workspace deps | `codex-client, codex-cloud-tasks-client, codex-cloud-tasks-mock-client, codex-core, codex-git-utils, codex-login, codex-model-provider, ontocode-tui, +1 more` |
| deferred | `ontocode-rs/cloud-tasks-client/Cargo.toml` | `codex-cloud-tasks-client` | `codex_cloud_tasks_client` | `-` | 9 total; 3 codex workspace deps | `codex-api, codex-backend-client, codex-git-utils` |
| deferred | `ontocode-rs/cloud-tasks-mock-client/Cargo.toml` | `codex-cloud-tasks-mock-client` | `codex_cloud_tasks_mock_client` | `-` | 4 total; 1 codex workspace deps | `codex-cloud-tasks-client` |
| core/shared | `ontocode-rs/code-mode/Cargo.toml` | `codex-code-mode` | `codex_code_mode` | `-` | 9 total; 1 codex workspace deps | `codex-protocol` |
| deferred | `ontocode-rs/collaboration-mode-templates/Cargo.toml` | `codex-collaboration-mode-templates` | `codex_collaboration_mode_templates` | `-` | 0 total; 0 codex workspace deps | - |
| core/shared | `ontocode-rs/config/Cargo.toml` | `codex-config` | `codex_config` | `-` | 44 total; 10 codex workspace deps | `ontocode-app-server-protocol, ontocode-execpolicy, codex-features, codex-file-system, codex-git-utils, codex-model-provider-info, codex-network-proxy, codex-protocol, +2 more` |
| provider/auth/MCP | `ontocode-rs/connectors/Cargo.toml` | `codex-connectors` | `codex_connectors` | `-` | 10 total; 1 codex workspace deps | `ontocode-app-server-protocol` |
| core/shared | `ontocode-rs/context-fragments/Cargo.toml` | `codex-context-fragments` | `codex_context_fragments` | `-` | 2 total; 2 codex workspace deps | `codex-protocol, codex-utils-string` |
| core/shared | `ontocode-rs/core/Cargo.toml` | `codex-core` | `codex_core` | `ontocode-write-config-schema` | 117 total; 54 codex workspace deps | `codex-analytics, codex-api, ontocode-app-server-protocol, codex-apply-patch, codex-async-utils, codex-code-mode, codex-config, codex-connectors, +46 more` |
| core/shared | `ontocode-rs/core-api/Cargo.toml` | `codex-core-api` | `codex_core_api` | `-` | 13 total; 13 codex workspace deps | `codex-analytics, ontocode-app-server-protocol, codex-arg0, codex-config, codex-core, ontocode-exec-server, codex-extension-api, codex-features, +5 more` |
| core/shared | `ontocode-rs/core-plugins/Cargo.toml` | `codex-core-plugins` | `codex_core_plugins` | `-` | 35 total; 14 codex workspace deps | `codex-analytics, ontocode-app-server-protocol, codex-config, codex-core-skills, ontocode-exec-server, codex-git-utils, codex-hooks, codex-login, +6 more` |
| deferred | `ontocode-rs/core-skills/Cargo.toml` | `codex-core-skills` | `codex_core_skills` | `-` | 26 total; 13 codex workspace deps | `codex-analytics, ontocode-app-server-protocol, codex-config, codex-context-fragments, ontocode-exec-server, codex-login, codex-model-provider, codex-otel, +5 more` |
| runtime-path/package-layout | `ontocode-rs/exec/Cargo.toml` | `ontocode-exec` | `codex_exec` | `ontocode-exec` | 41 total; 18 codex workspace deps | `ontocode-app-server-client, ontocode-app-server-protocol, codex-apply-patch, codex-arg0, codex-cloud-config, codex-config, codex-core, codex-feedback, +10 more` |
| runtime-path/package-layout | `ontocode-rs/exec-server/Cargo.toml` | `ontocode-exec-server` | `codex_exec_server` | `-` | 35 total; 10 codex workspace deps | `codex-api, ontocode-app-server-protocol, codex-client, codex-file-system, codex-protocol, codex-sandboxing, codex-test-binary-support, codex-utils-absolute-path, +2 more` |
| core/shared | `ontocode-rs/execpolicy/Cargo.toml` | `ontocode-execpolicy` | `codex_execpolicy` | `ontocode-execpolicy` | 11 total; 1 codex workspace deps | `codex-utils-absolute-path` |
| core/shared | `ontocode-rs/execpolicy-legacy/Cargo.toml` | `ontocode-execpolicy-legacy` | `codex_execpolicy_legacy` | `ontocode-execpolicy-legacy` | 14 total; 0 codex workspace deps | - |
| protocol/generated | `ontocode-rs/codex-experimental-api-macros/Cargo.toml` | `codex-experimental-api-macros` | `-` | `-` | 3 total; 0 codex workspace deps | - |
| deferred | `ontocode-rs/ext/extension-api/Cargo.toml` | `codex-extension-api` | `codex_extension_api` | `-` | 4 total; 3 codex workspace deps | `codex-context-fragments, codex-protocol, codex-tools` |
| deferred | `ontocode-rs/external-agent-migration/Cargo.toml` | `codex-external-agent-migration` | `codex_external_agent_migration` | `-` | 6 total; 1 codex workspace deps | `codex-hooks` |
| deferred | `ontocode-rs/external-agent-sessions/Cargo.toml` | `codex-external-agent-sessions` | `codex_external_agent_sessions` | `-` | 8 total; 3 codex workspace deps | `ontocode-app-server-protocol, codex-protocol, codex-utils-output-truncation` |
| core/shared | `ontocode-rs/features/Cargo.toml` | `codex-features` | `codex_features` | `-` | 7 total; 2 codex workspace deps | `codex-otel, codex-protocol` |
| core/shared | `ontocode-rs/feedback/Cargo.toml` | `codex-feedback` | `codex_feedback` | `-` | 7 total; 2 codex workspace deps | `codex-login, codex-protocol` |
| core/shared | `ontocode-rs/file-search/Cargo.toml` | `ontocode-file-search` | `codex_file_search` | `ontocode-file-search` | 10 total; 0 codex workspace deps | - |
| core/shared | `ontocode-rs/file-system/Cargo.toml` | `codex-file-system` | `codex_file_system` | `-` | 4 total; 2 codex workspace deps | `codex-protocol, codex-utils-absolute-path` |
| core/shared | `ontocode-rs/file-watcher/Cargo.toml` | `codex-file-watcher` | `codex_file_watcher` | `-` | 5 total; 0 codex workspace deps | - |
| core/shared | `ontocode-rs/git-utils/Cargo.toml` | `codex-git-utils` | `codex_git_utils` | `-` | 18 total; 3 codex workspace deps | `codex-file-system, codex-protocol, codex-utils-absolute-path` |
| deferred | `ontocode-rs/ext/goal/Cargo.toml` | `codex-goal-extension` | `codex_goal_extension` | `-` | 17 total; 7 codex workspace deps | `codex-core, codex-extension-api, codex-otel, codex-protocol, codex-state, codex-tools, codex-utils-template` |
| core/shared | `ontocode-rs/ext/guardian/Cargo.toml` | `codex-guardian` | `codex_guardian` | `-` | 4 total; 3 codex workspace deps | `codex-core, codex-extension-api, codex-protocol` |
| core/shared | `ontocode-rs/hooks/Cargo.toml` | `codex-hooks` | `codex_hooks` | `write_hooks_schema_fixtures` | 19 total; 5 codex workspace deps | `codex-config, codex-plugin, codex-protocol, codex-utils-absolute-path, codex-utils-output-truncation` |
| deferred | `ontocode-rs/ext/image-generation/Cargo.toml` | `codex-image-generation-extension` | `codex_image_generation_extension` | `-` | 15 total; 9 codex workspace deps | `codex-api, codex-core, codex-extension-api, codex-features, codex-login, codex-model-provider, codex-model-provider-info, codex-protocol, +1 more` |
| runtime-path/package-layout | `ontocode-rs/install-context/Cargo.toml` | `codex-install-context` | `codex_install_context` | `-` | 4 total; 2 codex workspace deps | `codex-utils-absolute-path, codex-utils-home-dir` |
| provider/auth/MCP | `ontocode-rs/keyring-store/Cargo.toml` | `codex-keyring-store` | `codex_keyring_store` | `-` | 6 total; 0 codex workspace deps | - |
| helper/tool | `ontocode-rs/linux-sandbox/Cargo.toml` | `codex-linux-sandbox` | `codex_linux_sandbox` | `ontocode-linux-sandbox` | 18 total; 6 codex workspace deps | `codex-core, codex-install-context, codex-process-hardening, codex-protocol, codex-sandboxing, codex-utils-absolute-path` |
| provider/auth/MCP | `ontocode-rs/lmstudio/Cargo.toml` | `codex-lmstudio` | `codex_lmstudio` | `-` | 9 total; 2 codex workspace deps | `codex-core, codex-model-provider-info` |
| provider/auth/MCP | `ontocode-rs/login/Cargo.toml` | `codex-login` | `codex_login` | `-` | 36 total; 10 codex workspace deps | `codex-agent-identity, ontocode-app-server-protocol, codex-client, codex-config, codex-keyring-store, codex-model-provider-info, codex-otel, codex-protocol, +2 more` |
| provider/auth/MCP | `ontocode-rs/codex-mcp/Cargo.toml` | `codex-mcp` | `codex_mcp` | `-` | 27 total; 11 codex workspace deps | `codex-api, codex-async-utils, codex-config, ontocode-exec-server, codex-login, codex-model-provider, codex-otel, codex-plugin, +3 more` |
| provider/auth/MCP | `ontocode-rs/mcp-server/Cargo.toml` | `ontocode-mcp-server` | `codex_mcp_server` | `ontocode-mcp-server` | 26 total; 11 codex workspace deps | `codex-arg0, codex-config, codex-core, ontocode-exec-server, codex-extension-api, codex-login, codex-protocol, codex-shell-command, +3 more` |
| deferred | `ontocode-rs/ext/memories/Cargo.toml` | `codex-memories-extension` | `codex_memories_extension` | `-` | 17 total; 8 codex workspace deps | `codex-core, codex-extension-api, codex-features, codex-otel, codex-tools, codex-utils-absolute-path, codex-utils-output-truncation, codex-utils-template` |
| deferred | `ontocode-rs/memories/read/Cargo.toml` | `codex-memories-read` | `codex_memories_read` | `-` | 4 total; 3 codex workspace deps | `codex-protocol, codex-shell-command, codex-utils-absolute-path` |
| deferred | `ontocode-rs/memories/write/Cargo.toml` | `codex-memories-write` | `codex_memories_write` | `-` | 30 total; 17 codex workspace deps | `codex-backend-client, codex-config, codex-core, codex-features, codex-git-utils, codex-login, codex-models-manager, codex-otel, +9 more` |
| core/shared | `ontocode-rs/message-history/Cargo.toml` | `codex-message-history` | `codex_message_history` | `-` | 8 total; 1 codex workspace deps | `codex-config` |
| provider/auth/MCP | `ontocode-rs/model-provider/Cargo.toml` | `codex-model-provider` | `codex_model_provider` | `-` | 19 total; 11 codex workspace deps | `codex-agent-identity, codex-api, codex-aws-auth, codex-client, codex-feedback, codex-login, codex-model-provider-info, codex-models-manager, +3 more` |
| provider/auth/MCP | `ontocode-rs/model-provider-info/Cargo.toml` | `codex-model-provider-info` | `codex_model_provider_info` | `-` | 11 total; 4 codex workspace deps | `codex-api, ontocode-app-server-protocol, codex-protocol, codex-utils-absolute-path` |
| core/shared | `ontocode-rs/models-manager/Cargo.toml` | `codex-models-manager` | `codex_models_manager` | `-` | 16 total; 7 codex workspace deps | `ontocode-app-server-protocol, codex-collaboration-mode-templates, codex-login, codex-otel, codex-protocol, codex-utils-output-truncation, codex-utils-template` |
| provider/auth/MCP | `ontocode-rs/network-proxy/Cargo.toml` | `codex-network-proxy` | `codex_network_proxy` | `-` | 28 total; 3 codex workspace deps | `codex-utils-absolute-path, codex-utils-home-dir, codex-utils-rustls-provider` |
| provider/auth/MCP | `ontocode-rs/ollama/Cargo.toml` | `codex-ollama` | `codex_ollama` | `-` | 13 total; 2 codex workspace deps | `codex-core, codex-model-provider-info` |
| core/shared | `ontocode-rs/otel/Cargo.toml` | `codex-otel` | `codex_otel` | `-` | 27 total; 5 codex workspace deps | `codex-api, ontocode-app-server-protocol, codex-protocol, codex-utils-absolute-path, codex-utils-string` |
| core/shared | `ontocode-rs/plugin/Cargo.toml` | `codex-plugin` | `codex_plugin` | `-` | 4 total; 3 codex workspace deps | `codex-config, codex-utils-absolute-path, codex-utils-plugins` |
| runtime-path/package-layout | `ontocode-rs/process-hardening/Cargo.toml` | `codex-process-hardening` | `codex_process_hardening` | `-` | 2 total; 0 codex workspace deps | - |
| core/shared | `ontocode-rs/prompts/Cargo.toml` | `codex-prompts` | `codex_prompts` | `-` | 8 total; 6 codex workspace deps | `codex-context-fragments, ontocode-execpolicy, codex-git-utils, codex-protocol, codex-utils-absolute-path, codex-utils-template` |
| protocol/generated | `ontocode-rs/protocol/Cargo.toml` | `codex-protocol` | `codex_protocol` | `-` | 34 total; 6 codex workspace deps | `codex-async-utils, ontocode-execpolicy, codex-network-proxy, codex-utils-absolute-path, codex-utils-image, codex-utils-string` |
| provider/auth/MCP | `ontocode-rs/realtime-webrtc/Cargo.toml` | `codex-realtime-webrtc` | `codex_realtime_webrtc` | `-` | 3 total; 0 codex workspace deps | - |
| core/shared | `ontocode-rs/response-debug-context/Cargo.toml` | `codex-response-debug-context` | `codex_response_debug_context` | `-` | 5 total; 1 codex workspace deps | `codex-api` |
| provider/auth/MCP | `ontocode-rs/responses-api-proxy/Cargo.toml` | `ontocode-responses-api-proxy` | `codex_responses_api_proxy` | `ontocode-responses-api-proxy` | 11 total; 1 codex workspace deps | `codex-process-hardening` |
| provider/auth/MCP | `ontocode-rs/rmcp-client/Cargo.toml` | `codex-rmcp-client` | `codex_rmcp_client` | `rmcp_test_server, test_stdio_server, test_streamable_http_server` | 36 total; 9 codex workspace deps | `codex-api, codex-client, codex-config, ontocode-exec-server, codex-keyring-store, codex-protocol, codex-utils-cargo-bin, codex-utils-home-dir, +1 more` |
| core/shared | `ontocode-rs/rollout/Cargo.toml` | `codex-rollout` | `codex_rollout` | `-` | 20 total; 7 codex workspace deps | `ontocode-file-search, codex-git-utils, codex-login, codex-otel, codex-protocol, codex-state, codex-utils-path` |
| core/shared | `ontocode-rs/rollout-trace/Cargo.toml` | `codex-rollout-trace` | `codex_rollout_trace` | `-` | 10 total; 2 codex workspace deps | `codex-code-mode, codex-protocol` |
| runtime-path/package-layout | `ontocode-rs/sandboxing/Cargo.toml` | `codex-sandboxing` | `codex_sandboxing` | `-` | 15 total; 3 codex workspace deps | `codex-network-proxy, codex-protocol, codex-utils-absolute-path` |
| provider/auth/MCP | `ontocode-rs/secrets/Cargo.toml` | `codex-secrets` | `codex_secrets` | `-` | 15 total; 2 codex workspace deps | `codex-git-utils, codex-keyring-store` |
| core/shared | `ontocode-rs/shell-command/Cargo.toml` | `codex-shell-command` | `codex_shell_command` | `-` | 15 total; 2 codex workspace deps | `codex-protocol, codex-utils-absolute-path` |
| helper/tool | `ontocode-rs/shell-escalation/Cargo.toml` | `codex-shell-escalation` | `codex_shell_escalation` | `ontocode-execve-wrapper` | 15 total; 2 codex workspace deps | `codex-protocol, codex-utils-absolute-path` |
| deferred | `ontocode-rs/skills/Cargo.toml` | `codex-skills` | `codex_skills` | `-` | 3 total; 1 codex workspace deps | `codex-utils-absolute-path` |
| deferred | `ontocode-rs/ext/skills/Cargo.toml` | `codex-skills-extension` | `codex_skills_extension` | `-` | 7 total; 4 codex workspace deps | `codex-core, codex-core-skills, codex-extension-api, codex-protocol` |
| core/shared | `ontocode-rs/state/Cargo.toml` | `codex-state` | `codex_state` | `logs_client` | 17 total; 2 codex workspace deps | `codex-git-utils, codex-protocol` |
| runtime-path/package-layout | `ontocode-rs/stdio-to-uds/Cargo.toml` | `ontocode-stdio-to-uds` | `codex_stdio_to_uds` | `ontocode-stdio-to-uds` | 6 total; 2 codex workspace deps | `codex-uds, codex-utils-cargo-bin` |
| runtime-path/package-layout | `ontocode-rs/terminal-detection/Cargo.toml` | `codex-terminal-detection` | `codex_terminal_detection` | `-` | 2 total; 0 codex workspace deps | - |
| helper/tool | `ontocode-rs/test-binary-support/Cargo.toml` | `codex-test-binary-support` | `codex_test_binary_support` | `-` | 2 total; 1 codex workspace deps | `codex-arg0` |
| CLI/app | `ontocode-rs/thread-manager-sample/Cargo.toml` | `codex-thread-manager-sample` | `-` | `codex-thread-manager-sample` | 5 total; 1 codex workspace deps | `codex-core-api` |
| core/shared | `ontocode-rs/thread-store/Cargo.toml` | `codex-thread-store` | `codex_thread_store` | `-` | 17 total; 6 codex workspace deps | `codex-git-utils, codex-install-context, codex-protocol, codex-rollout, codex-state, codex-utils-path` |
| helper/tool | `ontocode-rs/tools/Cargo.toml` | `codex-tools` | `codex_tools` | `-` | 18 total; 9 codex workspace deps | `ontocode-app-server-protocol, codex-code-mode, codex-features, codex-protocol, codex-utils-absolute-path, codex-utils-cargo-bin, codex-utils-output-truncation, codex-utils-pty, +1 more` |
| CLI/app | `ontocode-rs/tui/Cargo.toml` | `ontocode-tui` | `codex_tui` | `ontocode-tui, md-events` | 107 total; 45 codex workspace deps | `codex-ansi-escape, ontocode-app-server-client, ontocode-app-server-protocol, codex-arg0, codex-cli, codex-cloud-config, codex-config, codex-connectors, +37 more` |
| runtime-path/package-layout | `ontocode-rs/uds/Cargo.toml` | `codex-uds` | `codex_uds` | `-` | 7 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/absolute-path/Cargo.toml` | `codex-utils-absolute-path` | `codex_utils_absolute_path` | `-` | 8 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/approval-presets/Cargo.toml` | `codex-utils-approval-presets` | `codex_utils_approval_presets` | `-` | 1 total; 1 codex workspace deps | `codex-protocol` |
| leaf utility | `ontocode-rs/utils/cache/Cargo.toml` | `codex-utils-cache` | `codex_utils_cache` | `-` | 4 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/cargo-bin/Cargo.toml` | `codex-utils-cargo-bin` | `codex_utils_cargo_bin` | `-` | 3 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/cli/Cargo.toml` | `codex-utils-cli` | `codex_utils_cli` | `-` | 6 total; 2 codex workspace deps | `codex-protocol, codex-shell-command` |
| leaf utility | `ontocode-rs/utils/elapsed/Cargo.toml` | `codex-utils-elapsed` | `codex_utils_elapsed` | `-` | 0 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/fuzzy-match/Cargo.toml` | `codex-utils-fuzzy-match` | `codex_utils_fuzzy_match` | `-` | 0 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/home-dir/Cargo.toml` | `codex-utils-home-dir` | `codex_utils_home_dir` | `-` | 4 total; 1 codex workspace deps | `codex-utils-absolute-path` |
| leaf utility | `ontocode-rs/utils/image/Cargo.toml` | `codex-utils-image` | `codex_utils_image` | `-` | 8 total; 1 codex workspace deps | `codex-utils-cache` |
| leaf utility | `ontocode-rs/utils/json-to-toml/Cargo.toml` | `codex-utils-json-to-toml` | `codex_utils_json_to_toml` | `-` | 3 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/oss/Cargo.toml` | `codex-utils-oss` | `codex_utils_oss` | `-` | 4 total; 4 codex workspace deps | `codex-core, codex-lmstudio, codex-model-provider-info, codex-ollama` |
| leaf utility | `ontocode-rs/utils/output-truncation/Cargo.toml` | `codex-utils-output-truncation` | `codex_utils_output_truncation` | `-` | 3 total; 2 codex workspace deps | `codex-protocol, codex-utils-string` |
| leaf utility | `ontocode-rs/utils/path-utils/Cargo.toml` | `codex-utils-path` | `codex_utils_path` | `-` | 5 total; 1 codex workspace deps | `codex-utils-absolute-path` |
| leaf utility | `ontocode-rs/utils/plugins/Cargo.toml` | `codex-utils-plugins` | `codex_utils_plugins` | `-` | 7 total; 3 codex workspace deps | `ontocode-exec-server, codex-login, codex-utils-absolute-path` |
| leaf utility | `ontocode-rs/utils/pty/Cargo.toml` | `codex-utils-pty` | `codex_utils_pty` | `-` | 10 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/readiness/Cargo.toml` | `codex-utils-readiness` | `codex_utils_readiness` | `-` | 6 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/rustls-provider/Cargo.toml` | `codex-utils-rustls-provider` | `codex_utils_rustls_provider` | `-` | 1 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/sandbox-summary/Cargo.toml` | `codex-utils-sandbox-summary` | `codex_utils_sandbox_summary` | `-` | 6 total; 4 codex workspace deps | `codex-core, codex-model-provider-info, codex-protocol, codex-utils-absolute-path` |
| leaf utility | `ontocode-rs/utils/sleep-inhibitor/Cargo.toml` | `codex-utils-sleep-inhibitor` | `codex_utils_sleep_inhibitor` | `-` | 4 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/stream-parser/Cargo.toml` | `codex-utils-stream-parser` | `codex_utils_stream_parser` | `-` | 1 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/string/Cargo.toml` | `codex-utils-string` | `codex_utils_string` | `-` | 4 total; 0 codex workspace deps | - |
| leaf utility | `ontocode-rs/utils/template/Cargo.toml` | `codex-utils-template` | `codex_utils_template` | `-` | 1 total; 0 codex workspace deps | - |
| helper/tool | `ontocode-rs/v8-poc/Cargo.toml` | `codex-v8-poc` | `codex_v8_poc` | `-` | 2 total; 0 codex workspace deps | - |
| provider/auth/MCP | `ontocode-rs/ext/web-search/Cargo.toml` | `codex-web-search-extension` | `codex_web_search_extension` | `-` | 15 total; 9 codex workspace deps | `codex-api, codex-core, codex-extension-api, codex-features, codex-login, codex-model-provider, codex-model-provider-info, codex-protocol, +1 more` |
| helper/tool | `ontocode-rs/windows-sandbox-rs/Cargo.toml` | `ontocode-windows-sandbox` | `codex_windows_sandbox` | `ontocode-command-runner, ontocode-command-runner, ontocode-windows-sandbox, ontocode-windows-sandbox-setup` | 20 total; 5 codex workspace deps | `codex-otel, codex-protocol, codex-utils-absolute-path, codex-utils-pty, codex-utils-string` |

## Notes

- Dependency counts are direct manifest dependencies from Cargo metadata; target/platform activation is not expanded here.
- Direct codex workspace dependency lists include only direct dependencies whose Cargo package names start with `codex-`, truncated after eight names per row.
- Preliminary family classification is intentionally conservative and must be reviewed before any implementation slice.
- Package/lib inventory is Cargo-authoritative; Bazel labels, command refs, Rust imports, and protocol/schema surfaces are owned by separate Stage 0 outputs.
