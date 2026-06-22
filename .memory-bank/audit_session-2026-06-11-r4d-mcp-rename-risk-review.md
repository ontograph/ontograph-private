# R4D MCP Manager Rename Risk Review

Date: 2026-06-11

## Decision

- Approve exactly one identity-only slice: `codex-mcp` -> `ontocode-mcp` and `codex_mcp` -> `ontocode_mcp`.
- Treat as CRITICAL risk because MCP tool listing is used by session context, tool routing, connectors, plugin install verification, app-server app listing, and runtime MCP tool calls.

## OntoIndex Evidence

- CLI index status: `/opt/demodb/_workfolder/ontocode` is indexed and up to date at commit `73ba304`.
- MCP facade defaults to repository id `OntoIndex`, so R4D impact evidence uses the CLI command with `--repo codex`.
- `McpConnectionManager` struct: LOW impact.
- `McpConnectionManager::list_all_tools`: CRITICAL impact, 72 upstream impacted nodes, 16 direct callers, 18 modules, and session `run_turn` affected.
- `call_tool`: ambiguous without UID; runtime behavior remains forbidden for this identity-only slice.

## Direct Inventory

- `ontocode-rs/codex-mcp/Cargo.toml`: package and lib crate identity.
- `ontocode-rs/codex-mcp/BUILD.bazel`: Bazel target and crate identity.
- `ontocode-rs/Cargo.toml`: workspace dependency key; keep the workspace member path string `"codex-mcp"` because the directory is not being renamed.
- Direct dependent manifests: `ontocode-rs/app-server/Cargo.toml`, `ontocode-rs/cli/Cargo.toml`, `ontocode-rs/core/Cargo.toml`, `ontocode-rs/tui/Cargo.toml`.

## Allowed Changes

- Cargo package name, Rust crate name, Bazel crate identity, workspace/dependent manifest keys, Cargo/Bazel lockfiles, and active Rust imports/selectors if discovered.

## Forbidden Changes

- MCP protocol behavior.
- Tool/resource/template list behavior.
- MCP tool call behavior.
- Codex Apps MCP server names, URL paths, cache keys, and metadata keys.
- Legacy `mcp__` tool prefix behavior.
- OAuth/auth elicitation behavior.
- Tool provenance and tool metadata behavior.
- App-server MCP status/catalog behavior.
- CLI/TUI/core behavior.
- Telemetry/product strings.
- Env/config semantics.
- Wire/generated names.
- Persisted state.
- Existing `codex-mcp` directory path and workspace member path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core mcp --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server mcp --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli mcp --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale-reference search for `codex-mcp` and `codex_mcp`; classify the preserved folder path/workspace member and MCP-server package-name compatibility refs.
- `git diff --check`
- OntoIndex scoped `gn_verify_diff` or CLI `detect-changes`.
