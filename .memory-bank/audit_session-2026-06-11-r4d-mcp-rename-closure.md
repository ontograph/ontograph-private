# R4D MCP Manager Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-mcp` -> `ontocode-mcp` and `codex_mcp` -> `ontocode_mcp`.
- Scope stayed identity-only: Cargo package, Rust lib crate, Bazel crate/import wiring, dependent imports, and lockfiles.
- Preserved MCP protocol/tool/resource/template/list/call behavior, Codex Apps MCP names/paths/cache keys, legacy `mcp__` prefix behavior, OAuth/auth elicitation behavior, provenance/metadata, app-server MCP status/catalog behavior, CLI/TUI/core behavior, telemetry/product strings, env/config semantics, wire/generated names, persisted state, and the existing `codex-mcp` directory/workspace member path.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-mcp`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core mcp --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server mcp --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli mcp --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference scans for `codex_mcp::`, `\bcodex_mcp\b`, and `codex-mcp`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff` passed with explicit R4D changed files and executed tests.

## Result

- R4D is accepted.
- Remaining R4 provider/auth/support crates require a fresh one-slice senior risk review before dispatch.
