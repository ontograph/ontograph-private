# Claude Parked Row 121 Review

Date: 2026-06-20

## Decision

Row 121 stays rejected.

## Source

- ADR row 121: `Existing | Non-core | REJECT | OntoIndex already handles code exploration; do not clone it.`
- Donor row 121: `Add MCP explorer server for source browsing. | codex-mcp / dev tooling | Useful for donor/codebase exploration. | MCP server smoke test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Sub-agent `019ee52e-edb7-7113-8f26-46f90daec48b` recommended rejected and made no edits.
- AGENTS.md defines OntoIndex as the code-intelligence path for exploration, impact, and context.
- `ontocode-rs/codex-mcp/src/connection_manager.rs` already owns MCP tool listing through `list_all_tools`.
- `ontocode-rs/core/src/tools/handlers/tool_search_spec.rs` tells model-facing callers to use `tool_search` for MCP tool discovery.
- `ontocode-rs/core/src/tools/handlers/mcp_resource_spec.rs` already defines MCP resource list/template/read tool specs.
- `ontocode-rs/codex-mcp/src/connection_manager_tests.rs` covers cached MCP tool metadata while a client is pending.
- `ontocode-rs/core/tests/suite/search_tool.rs` covers deferring MCP tools behind `tool_search` rather than exposing them directly.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
