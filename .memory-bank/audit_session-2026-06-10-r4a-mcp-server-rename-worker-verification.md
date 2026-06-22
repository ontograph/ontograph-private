# R4A MCP Server Rename Worker Verification

Date: 2026-06-10

## Scope

- Implemented identity-only rename: `ontocode-mcp-server` -> `ontocode-mcp-server`.
- Implemented lib/Bazel crate rename: `codex_mcp_server` -> `ontocode_mcp_server`.
- Updated workspace and CLI dependency keys, MCP-server test support dependency key/imports, lockfile package references, and the active `justfile` MCP-server package selector.

## Preserved Compatibility

- Standalone binary name remains `ontocode-mcp-server`.
- MCP implementation name remains `ontocode-mcp-server`.
- OTEL service/tag compatibility remains `codex_mcp_server`.
- MCP protocol/tool behavior, CLI dispatch behavior, shell approval behavior, env/config semantics, wire/generated names, and persisted state were not changed.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server --no-tests=pass`: passed, 14 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli mcp`: passed, 6 tests with 255 skipped by filter.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active stale-reference search classified remaining `ontocode-mcp-server` / `codex_mcp_server` references as binary, implementation, OTEL, docs, or process context compatibility strings.
- `git diff --check`: passed.
- Scoped OntoIndex `gn_verify_diff`: passed.

## Notes

- OntoIndex impact was run for the CLI `cli_main` body reference update and returned LOW upstream impact with direct caller `main`.
- No blockers.
