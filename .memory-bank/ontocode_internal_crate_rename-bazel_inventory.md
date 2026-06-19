# Ontocode Internal Crate Rename: Bazel And Lockfile Inventory

Date: 2026-06-09

Scope: R0B Stage 0 inventory only. No implementation edits.

## Summary

- OntoIndex status: `/opt/demodb/_workfolder/ontocode` is indexed and up to date at commit `73ba304`.
- Direct Bazel source is authoritative for this report.
- Found 119 `ontocode-rs/**/BUILD.bazel` `crate_name = "codex_*"` declarations.
- Found 5 Bazel target names that directly encode `codex-*`.
- Found 1 explicit Bazel label that directly encodes a `codex-*` target name.
- Found 120 `codex-*` package records in `ontocode-rs/Cargo.lock`.
- Found 0 direct `codex-*` / `codex_*` string hits in `MODULE.bazel.lock`; lockfile still must be regenerated because `MODULE.bazel` imports `//ontocode-rs:Cargo.lock` and `//ontocode-rs:Cargo.toml` through `crate.from_cargo`.

## Bazel Crate Name Inventory

Every entry below must move from `codex_*` to the matching `ontocode_*` crate name when its Cargo package/lib rename slice lands.

| BUILD file | Current `crate_name` |
| --- | --- |
| `ontocode-rs/agent-graph-store/BUILD.bazel` | `codex_agent_graph_store` |
| `ontocode-rs/agent-identity/BUILD.bazel` | `codex_agent_identity` |
| `ontocode-rs/analytics/BUILD.bazel` | `codex_analytics` |
| `ontocode-rs/ansi-escape/BUILD.bazel` | `codex_ansi_escape` |
| `ontocode-rs/apply-patch/BUILD.bazel` | `codex_apply_patch` |
| `ontocode-rs/app-server/BUILD.bazel` | `codex_app_server` |
| `ontocode-rs/app-server-client/BUILD.bazel` | `codex_app_server_client` |
| `ontocode-rs/app-server-daemon/BUILD.bazel` | `codex_app_server_daemon` |
| `ontocode-rs/app-server-protocol/BUILD.bazel` | `codex_app_server_protocol` |
| `ontocode-rs/app-server-test-client/BUILD.bazel` | `codex_app_server_test_client` |
| `ontocode-rs/app-server-transport/BUILD.bazel` | `codex_app_server_transport` |
| `ontocode-rs/arg0/BUILD.bazel` | `codex_arg0` |
| `ontocode-rs/async-utils/BUILD.bazel` | `codex_async_utils` |
| `ontocode-rs/aws-auth/BUILD.bazel` | `codex_aws_auth` |
| `ontocode-rs/backend-client/BUILD.bazel` | `codex_backend_client` |
| `ontocode-rs/bwrap/BUILD.bazel` | `codex_bwrap` |
| `ontocode-rs/chatgpt/BUILD.bazel` | `codex_chatgpt` |
| `ontocode-rs/cli/BUILD.bazel` | `codex_cli` |
| `ontocode-rs/cloud-config/BUILD.bazel` | `codex_cloud_config` |
| `ontocode-rs/cloud-tasks/BUILD.bazel` | `codex_cloud_tasks` |
| `ontocode-rs/cloud-tasks-client/BUILD.bazel` | `codex_cloud_tasks_client` |
| `ontocode-rs/cloud-tasks-mock-client/BUILD.bazel` | `codex_cloud_tasks_mock_client` |
| `ontocode-rs/code-mode/BUILD.bazel` | `codex_code_mode` |
| `ontocode-rs/codex-api/BUILD.bazel` | `codex_api` |
| `ontocode-rs/codex-backend-openapi-models/BUILD.bazel` | `codex_backend_openapi_models` |
| `ontocode-rs/codex-client/BUILD.bazel` | `codex_client` |
| `ontocode-rs/codex-experimental-api-macros/BUILD.bazel` | `codex_experimental_api_macros` |
| `ontocode-rs/codex-mcp/BUILD.bazel` | `codex_mcp` |
| `ontocode-rs/collaboration-mode-templates/BUILD.bazel` | `codex_collaboration_mode_templates` |
| `ontocode-rs/config/BUILD.bazel` | `codex_config` |
| `ontocode-rs/connectors/BUILD.bazel` | `codex_connectors` |
| `ontocode-rs/context-fragments/BUILD.bazel` | `codex_context_fragments` |
| `ontocode-rs/core-api/BUILD.bazel` | `codex_core_api` |
| `ontocode-rs/core/BUILD.bazel` | `codex_core` |
| `ontocode-rs/core-plugins/BUILD.bazel` | `codex_core_plugins` |
| `ontocode-rs/core-skills/BUILD.bazel` | `codex_core_skills` |
| `ontocode-rs/exec/BUILD.bazel` | `codex_exec` |
| `ontocode-rs/execpolicy/BUILD.bazel` | `codex_execpolicy` |
| `ontocode-rs/execpolicy-legacy/BUILD.bazel` | `codex_execpolicy_legacy` |
| `ontocode-rs/exec-server/BUILD.bazel` | `codex_exec_server` |
| `ontocode-rs/external-agent-migration/BUILD.bazel` | `codex_external_agent_migration` |
| `ontocode-rs/external-agent-sessions/BUILD.bazel` | `codex_external_agent_sessions` |
| `ontocode-rs/ext/extension-api/BUILD.bazel` | `codex_extension_api` |
| `ontocode-rs/ext/goal/BUILD.bazel` | `codex_goal_extension` |
| `ontocode-rs/ext/guardian/BUILD.bazel` | `codex_guardian` |
| `ontocode-rs/ext/image-generation/BUILD.bazel` | `codex_image_generation_extension` |
| `ontocode-rs/ext/memories/BUILD.bazel` | `codex_memories_extension` |
| `ontocode-rs/ext/skills/BUILD.bazel` | `codex_skills_extension` |
| `ontocode-rs/ext/web-search/BUILD.bazel` | `codex_web_search_extension` |
| `ontocode-rs/features/BUILD.bazel` | `codex_features` |
| `ontocode-rs/feedback/BUILD.bazel` | `codex_feedback` |
| `ontocode-rs/file-search/BUILD.bazel` | `codex_file_search` |
| `ontocode-rs/file-system/BUILD.bazel` | `codex_file_system` |
| `ontocode-rs/file-watcher/BUILD.bazel` | `codex_file_watcher` |
| `ontocode-rs/git-utils/BUILD.bazel` | `codex_git_utils` |
| `ontocode-rs/hooks/BUILD.bazel` | `codex_hooks` |
| `ontocode-rs/install-context/BUILD.bazel` | `codex_install_context` |
| `ontocode-rs/keyring-store/BUILD.bazel` | `codex_keyring_store` |
| `ontocode-rs/linux-sandbox/BUILD.bazel` | `codex_linux_sandbox` |
| `ontocode-rs/lmstudio/BUILD.bazel` | `codex_lmstudio` |
| `ontocode-rs/login/BUILD.bazel` | `codex_login` |
| `ontocode-rs/mcp-server/BUILD.bazel` | `codex_mcp_server` |
| `ontocode-rs/memories/read/BUILD.bazel` | `codex_memories_read` |
| `ontocode-rs/memories/write/BUILD.bazel` | `codex_memories_write` |
| `ontocode-rs/message-history/BUILD.bazel` | `codex_message_history` |
| `ontocode-rs/model-provider/BUILD.bazel` | `codex_model_provider` |
| `ontocode-rs/model-provider-info/BUILD.bazel` | `codex_model_provider_info` |
| `ontocode-rs/models-manager/BUILD.bazel` | `codex_models_manager` |
| `ontocode-rs/network-proxy/BUILD.bazel` | `codex_network_proxy` |
| `ontocode-rs/ollama/BUILD.bazel` | `codex_ollama` |
| `ontocode-rs/otel/BUILD.bazel` | `codex_otel` |
| `ontocode-rs/plugin/BUILD.bazel` | `codex_plugin` |
| `ontocode-rs/process-hardening/BUILD.bazel` | `codex_process_hardening` |
| `ontocode-rs/prompts/BUILD.bazel` | `codex_prompts` |
| `ontocode-rs/protocol/BUILD.bazel` | `codex_protocol` |
| `ontocode-rs/realtime-webrtc/BUILD.bazel` | `codex_realtime_webrtc` |
| `ontocode-rs/response-debug-context/BUILD.bazel` | `codex_response_debug_context` |
| `ontocode-rs/responses-api-proxy/BUILD.bazel` | `codex_responses_api_proxy` |
| `ontocode-rs/rmcp-client/BUILD.bazel` | `codex_rmcp_client` |
| `ontocode-rs/rollout/BUILD.bazel` | `codex_rollout` |
| `ontocode-rs/rollout-trace/BUILD.bazel` | `codex_rollout_trace` |
| `ontocode-rs/sandboxing/BUILD.bazel` | `codex_sandboxing` |
| `ontocode-rs/secrets/BUILD.bazel` | `codex_secrets` |
| `ontocode-rs/shell-command/BUILD.bazel` | `codex_shell_command` |
| `ontocode-rs/shell-escalation/BUILD.bazel` | `codex_shell_escalation` |
| `ontocode-rs/skills/BUILD.bazel` | `codex_skills` |
| `ontocode-rs/state/BUILD.bazel` | `codex_state` |
| `ontocode-rs/stdio-to-uds/BUILD.bazel` | `codex_stdio_to_uds` |
| `ontocode-rs/terminal-detection/BUILD.bazel` | `codex_terminal_detection` |
| `ontocode-rs/test-binary-support/BUILD.bazel` | `codex_test_binary_support` |
| `ontocode-rs/thread-manager-sample/BUILD.bazel` | `codex_thread_manager_sample` |
| `ontocode-rs/thread-store/BUILD.bazel` | `codex_thread_store` |
| `ontocode-rs/tools/BUILD.bazel` | `codex_tools` |
| `ontocode-rs/tui/BUILD.bazel` | `codex_tui` |
| `ontocode-rs/uds/BUILD.bazel` | `codex_uds` |
| `ontocode-rs/utils/absolute-path/BUILD.bazel` | `codex_utils_absolute_path` |
| `ontocode-rs/utils/approval-presets/BUILD.bazel` | `codex_utils_approval_presets` |
| `ontocode-rs/utils/cache/BUILD.bazel` | `codex_utils_cache` |
| `ontocode-rs/utils/cargo-bin/BUILD.bazel` | `codex_utils_cargo_bin` |
| `ontocode-rs/utils/cli/BUILD.bazel` | `codex_utils_cli` |
| `ontocode-rs/utils/elapsed/BUILD.bazel` | `codex_utils_elapsed` |
| `ontocode-rs/utils/fuzzy-match/BUILD.bazel` | `codex_utils_fuzzy_match` |
| `ontocode-rs/utils/home-dir/BUILD.bazel` | `codex_utils_home_dir` |
| `ontocode-rs/utils/image/BUILD.bazel` | `codex_utils_image` |
| `ontocode-rs/utils/json-to-toml/BUILD.bazel` | `codex_utils_json_to_toml` |
| `ontocode-rs/utils/oss/BUILD.bazel` | `codex_utils_oss` |
| `ontocode-rs/utils/output-truncation/BUILD.bazel` | `codex_utils_output_truncation` |
| `ontocode-rs/utils/path-utils/BUILD.bazel` | `codex_utils_path` |
| `ontocode-rs/utils/plugins/BUILD.bazel` | `codex_utils_plugins` |
| `ontocode-rs/utils/pty/BUILD.bazel` | `codex_utils_pty` |
| `ontocode-rs/utils/readiness/BUILD.bazel` | `codex_utils_readiness` |
| `ontocode-rs/utils/rustls-provider/BUILD.bazel` | `codex_utils_rustls_provider` |
| `ontocode-rs/utils/sandbox-summary/BUILD.bazel` | `codex_utils_sandbox_summary` |
| `ontocode-rs/utils/sleep-inhibitor/BUILD.bazel` | `codex_utils_sleep_inhibitor` |
| `ontocode-rs/utils/stream-parser/BUILD.bazel` | `codex_utils_stream_parser` |
| `ontocode-rs/utils/string/BUILD.bazel` | `codex_utils_string` |
| `ontocode-rs/utils/template/BUILD.bazel` | `codex_utils_template` |
| `ontocode-rs/v8-poc/BUILD.bazel` | `codex_v8_poc` |
| `ontocode-rs/windows-sandbox-rs/BUILD.bazel` | `codex_windows_sandbox` |

Notes:

- `ontocode-rs/adapter-protocol/Cargo.toml` appears in Cargo lock/package inventory but no `ontocode-rs/adapter-protocol/BUILD.bazel` was found by direct file inventory.
- `ontocode-rs/core/tests/common/BUILD.bazel`, `ontocode-rs/app-server/tests/common/BUILD.bazel`, and `ontocode-rs/mcp-server/tests/common/BUILD.bazel` use non-`codex_*` test support crate names and are not counted above.

## Bazel Labels, Macros, And Encoded Names

### Macro Surface

- Root macro: `defs.bzl::codex_rust_crate`.
- Every counted crate loads and invokes `codex_rust_crate`; renaming the macro itself is global Bazel churn and should not happen inside crate-family rename slices unless separately approved.
- `codex_rust_crate` derives:
  - library labels from `name`.
  - binary labels from Cargo `[[bin]]` data in `@crates//:data.bzl`.
  - unit test labels as `<name>-unit-tests`.
  - integration test labels as `<name>-<test-stem>-test`.
  - Windows cross-test labels as `<name>-<test-stem>-test-windows-cross`.
  - `CARGO_BIN_EXE_*` env keys and runfile mappings from binary target names.
  - deps from `all_crate_deps()` generated by `@crates`.

### Direct `codex-*` Bazel Target Names

These `name = "codex-*"` targets are separate from `crate_name` and require explicit Bazel label migration if the corresponding package target is renamed:

| BUILD file | Encoded target name |
| --- | --- |
| `ontocode-rs/codex-api/BUILD.bazel` | `codex-api` |
| `ontocode-rs/codex-backend-openapi-models/BUILD.bazel` | `codex-backend-openapi-models` |
| `ontocode-rs/codex-client/BUILD.bazel` | `codex-client` |
| `ontocode-rs/codex-experimental-api-macros/BUILD.bazel` | `codex-experimental-api-macros` |
| `ontocode-rs/codex-mcp/BUILD.bazel` | `codex-mcp` |

### Direct Encoded Label

- `ontocode-rs/core/BUILD.bazel` uses `//ontocode-rs/windows-sandbox-rs:ontocode-command-runner` in `extra_binaries`.
- This label feeds test runfiles and `CARGO_BIN_EXE_ontocode-command-runner`; if renamed, core integration tests must update both the Bazel label and any Rust-side binary lookup expectations.

### Other Visible Bazel Surfaces

- `MODULE.bazel` has `module(name = "codex")`; this is a repository/module identity surface, not a crate name, and should not be changed by internal crate rename slices without a separate Bazel-module compatibility decision.
- `justfile` has `bazel-codex` recipes that run `bazel run //ontocode-rs/cli:codex`; current working tree already has active CLI hard-cutover changes, so treat this as a separate public/runtime command surface and do not fold it into crate rename slices without manager review.
- `ontocode-rs/cli/BUILD.bazel` already declares `multiplatform_binaries(name = "ontocode")`; binary target churn here is high risk because release target listing and package scripts depend on it.
- `defs.bzl` keeps `--remap-path-prefix=../ontocode-rs=` and `--remap-path-prefix=ontocode-rs=`; this is repository path normalization, not crate identity.

## Lockfile And Generated Surface Inventory

### Direct Lockfiles

- `ontocode-rs/Cargo.lock`: must update after each Cargo package rename. It currently contains 120 `name = "codex-*"` package records.
- `MODULE.bazel.lock`: must be refreshed after each Cargo/Cargo.lock package rename even though it currently has no direct `codex-*` or `codex_*` strings.
- `tools/argument-comment-lint/Cargo.lock`: separate lockfile imported by `MODULE.bazel`; no R0B evidence that internal `ontocode-rs` crate renames should edit it.

### Bazel Cargo Import Wiring

- `MODULE.bazel` imports Rust workspace crates with:
  - `crate.from_cargo(cargo_lock = "//ontocode-rs:Cargo.lock", cargo_toml = "//ontocode-rs:Cargo.toml", ...)`
  - `use_repo(crate, "crates")`
- `defs.bzl` loads `@crates//:data.bzl` and `@crates//:defs.bzl`.
- Generated `@crates` data drives `DEP_DATA`, `all_crate_deps`, binary discovery, and test env/runfile generation.
- Package rename slices must assume generated Bazel repository metadata changes even when checked-in `MODULE.bazel.lock` has no human-readable `codex-*` strings.

### Generated/Derived Outputs To Watch

- `MODULE.bazel.lock`, via `just bazel-lock-update`.
- Generated external repo metadata under Bazel output/cache for `@crates`; not checked in, but used by analysis.
- `CARGO_BIN_EXE_*` env names generated by `codex_rust_crate` for binaries.
- Test labels generated by `codex_rust_crate`.
- Release filegroups from `multiplatform_binaries`, especially `//ontocode-rs/cli:release_binaries`.

## Required Verification Commands Per Rename Slice

Run from repo root unless noted.

1. Format and package tests:
   - `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
   - `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p <new-package>`
2. Bazel lock regeneration and check:
   - `just bazel-lock-update`
   - `just bazel-lock-check`
3. Bazel package analysis/build checks:
   - `bazel build //ontocode-rs/<crate-path>:<bazel-name>`
   - `bazel test //ontocode-rs/<crate-path>:<bazel-name>-unit-tests` when a unit-test target is generated.
   - `bazel test //ontocode-rs/<crate-path>:<bazel-name>-<test-stem>-test` for each visible `tests/*.rs` integration test target.
4. Binary/runfile checks when the crate owns or exposes binaries:
   - `bazel build //ontocode-rs/<crate-path>:<binary-name>`
   - `bazel test` on dependent crates that list the binary in `extra_binaries`.
5. Release target checks when CLI/runtime binary surfaces are touched:
   - `bazel build //ontocode-rs/cli:release_binaries`
   - `scripts/list-bazel-release-targets.sh` if release-target enumeration changes.
6. Diff verification:
   - Prefer OntoIndex `gn_verify_diff` for the exact slice.
   - Fallback: `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`.

## Package-Specific Bazel Checks Visible From R0B

These are examples for the early proposed slices and high-risk families; each actual slice should derive the exact labels from that crate's `BUILD.bazel` and `tests/*.rs`.

| Slice/crate | Bazel checks |
| --- | --- |
| `utils/absolute-path` | `bazel build //ontocode-rs/utils/absolute-path:absolute-path`; `bazel test //ontocode-rs/utils/absolute-path:absolute-path-unit-tests` |
| `utils/cargo-bin` | `bazel build //ontocode-rs/utils/cargo-bin:cargo-bin`; `bazel test //ontocode-rs/utils/cargo-bin:cargo-bin-unit-tests`; dependent runfile tests using `//ontocode-rs/utils/cargo-bin:repo_root.marker` |
| `install-context` | `bazel build //ontocode-rs/install-context:install-context`; `bazel test //ontocode-rs/install-context:install-context-unit-tests` |
| `linux-sandbox` | `bazel build //ontocode-rs/linux-sandbox:linux-sandbox`; test dependent crates that use `//ontocode-rs/linux-sandbox:ontocode-linux-sandbox` as an `extra_binaries` runfile |
| `windows-sandbox-rs` | `bazel build //ontocode-rs/windows-sandbox-rs:windows-sandbox-rs`; update/check `//ontocode-rs/windows-sandbox-rs:ontocode-command-runner` consumers if the binary label changes |
| `cli` | `bazel build //ontocode-rs/cli:cli`; `bazel build //ontocode-rs/cli:ontocode`; `bazel build //ontocode-rs/cli:release_binaries`; dependent `extra_binaries` tests in `core`, `tui`, and `rmcp-client` |
| `core` | `bazel test //ontocode-rs/core:core-unit-tests`; `bazel test //ontocode-rs/core:core-all-test`; check `extra_binaries`, `compile_data`, and `CARGO_MANIFEST_DIR` behavior |
| `tui` | `bazel test //ontocode-rs/tui:tui-unit-tests`; check `compile_data`, snapshots, and `//ontocode-rs/cli:ontocode` runfile |
| `app-server` | `bazel test //ontocode-rs/app-server:app-server-unit-tests`; check sharded app-server integration labels and `extra_binaries` |
| `app-server-protocol` | `bazel build //ontocode-rs/app-server-protocol:app-server-protocol`; `bazel test //ontocode-rs/app-server-protocol:app-server-protocol-unit-tests`; schema fixture names are a separate protocol gate |
| `protocol` | `bazel build //ontocode-rs/protocol:protocol`; `bazel test //ontocode-rs/protocol:protocol-unit-tests`; protocol/generated identities are a separate gate |
| `v8-poc` | `bazel build //ontocode-rs/v8-poc:v8-poc`; `bazel build //ontocode-rs/v8-poc:v8-poc-rusty-v8`; check `@crates//:v8` and V8 artifact wiring |

## High-Risk Bazel Crates

- `ontocode-rs/core`: high risk due `compile_data`, `CARGO_MANIFEST_DIR`, sharded tests, `no-sandbox`, long timeouts, and many `extra_binaries`.
- `ontocode-rs/cli`: high risk due release binary/filegroup generation and external scripts that may call Bazel CLI targets.
- `ontocode-rs/tui`: high risk due snapshots, compile/test data, and `//ontocode-rs/cli:ontocode` binary runfile dependency.
- `ontocode-rs/app-server`: high risk due sharded tests and runtime helper binaries.
- `ontocode-rs/app-server-protocol` and `ontocode-rs/protocol`: high risk because schema/generated/wire identifiers are explicitly out of scope unless separately approved.
- `ontocode-rs/windows-sandbox-rs`: high risk due explicit `ontocode-command-runner` label consumed by core tests.
- `ontocode-rs/linux-sandbox`, `ontocode-rs/bwrap`, `ontocode-rs/sandboxing`: high risk due sandbox/helper binary runfiles and platform-specific Bazel constraints.
- `ontocode-rs/exec-server` and `ontocode-rs/exec`: high risk due helper/runtime behavior and integration compile data.
- `ontocode-rs/rmcp-client`: medium-high risk because it depends on `//ontocode-rs/cli:ontocode` test server/binary runfiles.
- `ontocode-rs/v8-poc` and `ontocode-rs/realtime-webrtc`: high risk due custom Bazel deps, `@crates` overrides, and platform link flags.

## Bazel-Specific No-Go Risks

- Do not rename the root `codex_rust_crate` macro as part of a crate slice; it is shared by nearly every Rust Bazel package.
- Do not rename `MODULE.bazel` `module(name = "codex")` without a separate Bazel module identity decision.
- Do not rename protocol/schema generated names just because `crate_name` changes.
- Do not rename package-manager identities or package layout files such as `codex-package.json` from Bazel slices.
- Do not change binary labels consumed through `extra_binaries` without updating dependent `CARGO_BIN_EXE_*` expectations and runfiles.
- Do not skip `just bazel-lock-update` and `just bazel-lock-check`; `@crates` metadata is generated from Cargo inputs and can drift without obvious source string matches.
- Do not rely only on `MODULE.bazel.lock` string search; it currently has no direct `codex-*` strings but is still part of the lockfile surface.
- Do not batch `core`, `cli`, protocol, app-server, and sandbox/runtime helpers together; their Bazel runfile/test surfaces overlap and make failures hard to isolate.

## Commands Run

- `ctx_read .memory-bank/MEMORY.md`
- `ctx_read .memory-bank/ONTOCODE_INTERNAL_CRATE_RENAME_PROJECT_PLAN.md`
- `ctx_read .memory-bank/ONTOCODE_INTERNAL_CRATE_RENAME_TRACKING.md`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js status`
- `rg -n 'crate_name\s*=\s*"codex_[^"]+"' ontocode-rs -g 'BUILD.bazel'`
- `rg -n '\bname\s*=\s*"[^"]*codex[^"]*"' ontocode-rs -g 'BUILD.bazel'`
- `rg -n '(@crates//:[^"[:space:],\]]*codex[^"[:space:],\]]*|//ontocode-rs/[^"[:space:],\]]*:[^"[:space:],\]]*codex[^"[:space:],\]]*|:[^"[:space:],\]]*codex[^"[:space:],\]]*)' ontocode-rs -g 'BUILD.bazel'`
- `rg -n 'codex_rust_crate|multiplatform_binaries|workspace_root_test|CARGO_BIN_EXE_|package-files|release_binaries|repo_root\.marker|ontocode-rs=|ontocode-rs/' defs.bzl ontocode-rs -g 'BUILD.bazel'`
- `awk '/^name = "codex-/{print FILENAME ":" FNR ":" $0}' ontocode-rs/Cargo.lock`
- `rg -n 'codex-|codex_' MODULE.bazel.lock`
- `rg --files | rg '(^|/)(Cargo\.lock|MODULE\.bazel\.lock|MODULE\.bazel|Cargo\.toml|BUILD\.bazel|defs\.bzl|.*lock.*|.*bazel.*)$'`
- `git status --short`

## Blockers And Caveats

- Python heredoc analysis is blocked by local command policy; the inventory used `rg`, `awk`, and direct file reads instead.
- One broad Bazel/script search timed out after 120 seconds; the key Bazel source inventory completed through narrower direct searches.
- Working tree already has many unrelated modifications from other work; this R0B task only adds this owned output file.
- No build/test/lock regeneration commands were run because this is inventory-only and no crate rename was implemented.
