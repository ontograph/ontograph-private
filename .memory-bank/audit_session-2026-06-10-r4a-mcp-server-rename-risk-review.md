# R4A MCP Server Rename Risk Review

Date: 2026-06-10

## Decision

- Approve exactly one R4 identity-only slice: `ontocode-mcp-server` -> `ontocode-mcp-server` and `codex_mcp_server` -> `ontocode_mcp_server`.
- Preserve the standalone `ontocode-mcp-server` binary name unless a separate command-surface ADR approves removing or renaming it.

## OntoIndex Impact

- `ontocode-rs/mcp-server/src/lib.rs::run_main`: LOW.
- Upstream impact reaches `cli_main` and `main` through CLI dispatch.

## Direct Inventory

- `ontocode-rs/mcp-server/Cargo.toml`: package name, binary name, and lib crate name.
- `ontocode-rs/mcp-server/BUILD.bazel`: Bazel crate name.
- `ontocode-rs/Cargo.toml`: workspace dependency key.
- `ontocode-rs/cli/Cargo.toml`: direct CLI dependency key.
- `justfile`: package selector for the MCP server helper recipe.

## Allowed Changes

- Cargo package name, lib crate name, workspace/dependent manifest keys, Rust imports, Bazel crate name, lockfiles, and active package selectors.
- Test names or package selectors needed to run the renamed package.

## Forbidden Changes

- Standalone `ontocode-mcp-server` binary name.
- MCP protocol/tool behavior.
- CLI dispatch behavior.
- Shell approval behavior.
- Telemetry/product strings.
- Env/config semantics.
- Protocol/wire/generated names.
- Persisted state.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server --no-tests=pass`
- Focused CLI MCP-server dispatch test if available, otherwise `CARGO_BUILD_JOBS=8 just test -p ontocode-cli mcp`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale-reference search for old package/lib refs, with standalone binary name classified as intentional compatibility.
- `git diff --check`
- OntoIndex scoped `gn_verify_diff`
