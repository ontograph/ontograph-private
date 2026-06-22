# R4D MCP Manager Rename Worker Verification

Date: 2026-06-11

## Scope

- Implemented exactly `codex-mcp` -> `ontocode-mcp` and `codex_mcp` -> `ontocode_mcp` for active Cargo package/lib, Bazel crate identity, direct dependent manifests, lockfile package references, and active Rust imports/selectors.
- Preserved the existing `ontocode-rs/codex-mcp/` folder path and workspace member path string `"codex-mcp"`.
- No MCP protocol, tool/resource/template list, tool call, Codex Apps compatibility, legacy `mcp__` prefix, auth elicitation, provenance, app-server/CLI/TUI/core behavior, telemetry/product string, env/config, wire/generated name, or persisted-state behavior was changed.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp --no-tests=pass` passed: 71 tests.
- `CARGO_BUILD_JOBS=8 just test -p codex-core mcp --no-tests=pass` passed: 220 tests, 2442 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server mcp --no-tests=pass` passed: 28 tests, 783 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli mcp --no-tests=pass` passed: 6 tests, 255 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass` passed: 2772 tests, 4 skipped.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed with existing crate-annotation warnings.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-mcp` passed.
- `/home/evrasyuk/.local/bin/lean-ctx -c 'git diff --check'` passed.
- OntoIndex CLI `detect-changes --repo codex` ran after implementation; output is noisy from pre-existing unrelated dirty files and reports 200 changed files / 307 symbols / medium risk.

## Stale Reference Classification

- Active manifest/Bazel/lockfile old package refs are clean except the intentionally preserved workspace member/path string `codex-mcp`.
- Active Rust import/selectors are clean: no `codex_mcp::` remains under app-server, CLI, core, TUI, or `codex-mcp`.
- Remaining old-name hits are intentional compatibility surfaces: `codex-mcp` config/debug command examples, `codex-mcp-client` MCP implementation name, `ontocode-mcp-server` binary/package compatibility, `codex_mcp_server` telemetry tags, `codex_mcp_tool_call_id` wire/test field names, `codex_mcp_tool_call_event` telemetry event type, `__codex_mcp_decline__` synthetic persisted/result marker, and `/tmp/codex-mcp` config serialization fixtures.

## Notes

- OntoIndex CLI status confirmed `/opt/demodb/_workfolder/ontocode` indexed at commit `73ba304`.
- OntoIndex CLI impact for `Function:ontocode-rs/codex-mcp/src/connection_manager.rs:McpConnectionManager.list_all_tools` is CRITICAL with 92 impacted nodes, 16 direct callers, 20 modules, and `run_turn` affected; implementation stayed identity-only.
