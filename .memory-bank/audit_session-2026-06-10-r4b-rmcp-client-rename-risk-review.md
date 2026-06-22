# R4B RMCP Client Rename Risk Review

Date: 2026-06-10

## Decision

- Approve exactly one identity-only slice: `codex-rmcp-client` -> `ontocode-rmcp-client` and `codex_rmcp_client` -> `ontocode_rmcp_client`.
- Treat this as auth/MCP sensitive despite LOW graph impact.

## OntoIndex Impact

- `RmcpClient` struct in `ontocode-rs/rmcp-client/src/rmcp_client.rs`: LOW, no upstream graph callers reported.

## Direct Inventory

- `ontocode-rs/rmcp-client/Cargo.toml`: package name.
- `ontocode-rs/rmcp-client/BUILD.bazel`: Bazel crate name.
- `ontocode-rs/Cargo.toml`: workspace dependency key.
- `ontocode-rs/codex-mcp/Cargo.toml`
- `ontocode-rs/core/Cargo.toml`
- `ontocode-rs/cli/Cargo.toml`
- `ontocode-rs/app-server/Cargo.toml`

## Allowed Changes

- Cargo package name, derived Rust crate name, Bazel crate name, workspace/dependent manifest keys, lockfiles, and active imports/selectors if any are discovered.

## Forbidden Changes

- OAuth token parsing/refresh behavior.
- Credential/keyring behavior.
- MCP handshake/tool/resource/list behavior.
- Streamable HTTP/SSE behavior.
- Child-process transport behavior.
- Custom CA/TLS behavior.
- CLI/app-server/core behavior.
- Telemetry/product strings.
- Env/config semantics.
- Wire/generated names.
- Persisted state.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client --no-tests=pass`
- Direct dependent package checks: `codex-mcp`, `codex-core`, `ontocode-cli`, `ontocode-app-server`.
- Focused OAuth/MCP tests if available from package filters.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale-reference search for old package/lib refs.
- `git diff --check`
- OntoIndex scoped `gn_verify_diff`
