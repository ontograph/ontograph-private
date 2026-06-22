# Claude Parked Row 126 Review

Date: 2026-06-20

## Decision

Row 126 stays rejected.

## Source

- ADR row 126: `Existing | Non-core | REJECT | File search should use existing shell/search/OntoIndex.`
- Donor row 126: `Add read-source-file MCP endpoint with path allowlist. | codex-mcp | Safe code exploration. | Path traversal test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Sub-agent `019ee538-db74-7843-b04e-117f5446af02` recommended rejected and made no edits.
- `ontocode-rs/core/src/tools/handlers/mcp_resource_spec.rs`, `ontocode-rs/core/src/tools/handlers/mcp_resource/read_mcp_resource.rs`, `ontocode-rs/core/src/codex_thread.rs`, and `ontocode-rs/codex-mcp/src/connection_manager.rs` already own MCP resource reading as server/URI-scoped resource access.
- `ontocode-rs/core/src/tools/handlers/tool_search_spec.rs` identifies `tool_search` as the model-facing MCP discovery path.
- `ontocode-rs/file-search/src/lib.rs` owns repository file search.
- `ontocode-rs/tui/src/history_cell/tests.rs` covers shell search rendering for commands such as `rg "foo" src`.
- No concrete existing-owner path-traversal or allowlist gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
