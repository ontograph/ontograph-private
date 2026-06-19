# R4A MCP Server Rename Closure

Date: 2026-06-10

## Scope

- Accepted `ontocode-mcp-server` -> `ontocode-mcp-server` and `codex_mcp_server` -> `ontocode_mcp_server` as an identity-only package/lib/Bazel/import rename.
- Preserved the standalone `ontocode-mcp-server` binary name.
- Preserved MCP implementation name, OTEL compatibility, MCP protocol/tool behavior, CLI dispatch behavior, shell approval behavior, env/config semantics, wire/generated names, and persisted state.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server --no-tests=pass` passed: 14 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli mcp` passed: 6 passed, 255 skipped.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed in worker verification.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- Active stale-reference search found only intentional standalone binary, implementation-name, test-process, and OTEL compatibility strings.
- `git diff --check` passed.
- OntoIndex scoped `gn_verify_diff` passed.

## Next

- Continue R4 with a fresh one-slice risk review before dispatching another provider/auth/MCP support crate.
