# R1J CLI Utility Rename Risk Review

Date: 2026-06-10

Decision:
- Approve `codex-utils-cli` -> `ontocode-utils-cli` as the next exact R1 identity-only slice.
- Scope is limited to Cargo package identity, Rust lib crate identity, Bazel crate name, workspace dependency keys, dependent manifests/imports, active README/test command refs, lockfiles, and verification metadata.
- No runtime package-layout, public command-name, protocol/schema, telemetry, persisted-state, `.codex`, or `CODEX_*` rename authority is granted.

Evidence:
- Current metadata shows nine remaining `codex-utils-*` crates.
- `codex-utils-cli` has 8 direct reverse dependencies: app-server, app-server-test-client, chatgpt, cli, cloud-tasks, exec, mcp-server, and tui.
- Exported surface includes `CliConfigOverrides`, `SharedCliOptions`, `ApprovalModeCliArg`, `SandboxModeCliArg`, `ProfileV2Name`, `format_env_display`, `resume_command`, and `resume_hint`.
- OntoIndex impact for `resume_hint`: HIGH, 9 impacted nodes, direct CLI/TUI resume hint paths.
- OntoIndex impact for `CliConfigOverrides` struct: HIGH, 5 impacted nodes across MCP/config/test-client paths.
- OntoIndex impact for `SharedCliOptions` struct: LOW in graph, but direct import inventory shows CLI/TUI/exec use.

Rejected alternatives:
- `codex-utils-rustls-provider`: CRITICAL impact through TLS/auth/websocket/proxy/exec-server paths.
- `codex-utils-pty`: direct search shows command exec, core exec/unified-exec, exec-server, rmcp-client stdio, tools, and Windows sandbox usage; OntoIndex LOW result is unreliable because it resolved type aliases without import callers.
- `codex-utils-string`: HIGH/CRITICAL through telemetry, metrics, tool previews, Windows sandbox, and TUI/app-server/exec process paths.
- `codex-utils-output-truncation`: CRITICAL through shell/tool-output/context/hook output paths.

Required R1J verification:
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cloud-tasks`
- `CARGO_BUILD_JOBS=8 just test -p codex-chatgpt`
- `CARGO_BUILD_JOBS=8 PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 PATH=/home/evrasyuk/.local/node_modules/.bin:$PATH just bazel-lock-check`
- Stale-reference search for `codex-utils-cli` and `codex_utils_cli`.
- `git diff --check`
- OntoIndex `gn_verify_diff` scoped to R1J files and tests.
