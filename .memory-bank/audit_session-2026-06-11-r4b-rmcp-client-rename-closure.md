# R4B RMCP Client Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-rmcp-client` -> `ontocode-rmcp-client`.
- Accepted `codex_rmcp_client` -> `ontocode_rmcp_client` for active crate/import/Bazel identity.
- Preserved OAuth token parsing/refresh, credential/keyring behavior, MCP protocol behavior, streamable HTTP/SSE behavior, child-process transport, custom CA/TLS behavior, telemetry strings, env/config semantics, wire/generated names, and persisted state.

## Verification

- PASS: `CARGO_BUILD_JOBS=8 just fmt`
- PASS: `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client --no-tests=pass`
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-mcp --no-tests=pass`
- PARTIAL: `CARGO_BUILD_JOBS=8 just test -p codex-core --no-tests=pass` reached the full suite but failed unrelated `skills_use_aliases_in_developer_message_under_budget_pressure`; `snapshot_rollback_followup_turn_trims_context_updates` passed on exact rerun.
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-core mcp --no-tests=pass`
- PASS: `CARGO_BUILD_JOBS=8 just test -p ontocode-cli mcp --no-tests=pass`
- PASS: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server mcp --no-tests=pass`
- ZERO MATCH: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server oauth --no-tests=pass`
- PASS: `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- PASS: `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- PASS: active stale-reference search found no `codex-rmcp-client` or `codex_rmcp_client` refs under `ontocode-rs`.
- PASS: `git diff --check`
- PASS: OntoIndex scoped `gn_verify_diff` for the R4B file set.

## Notes

- The original sub-agent handle `019eb3c3-9e15-72e0-bb46-0eb5e170d738` was unavailable after session continuation, so manager recovery reviewed and verified the landed changes directly.
- Full app-server test execution exceeded the wrapper timeout and lost summary output; focused app-server MCP coverage passed after warm compile.
