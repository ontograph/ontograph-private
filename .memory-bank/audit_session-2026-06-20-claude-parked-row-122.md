# Claude Parked Row 122 Review

Date: 2026-06-20

## Decision

Row 122 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 122 says to add metadata to existing MCP resources only if missing.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 122 proposes adding a list-tools MCP endpoint with source metadata under `codex-mcp`.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `ontocode-rs/rmcp-client/src/rmcp_client.rs` already implements MCP `tools/list`.
- `ontocode-rs/codex-mcp/src/connection_manager.rs` already aggregates tools through `McpConnectionManager::list_all_tools` and attaches server metadata.
- `ontocode-rs/core/src/tools/handlers/mcp_resource_spec.rs` and its tests already define the current MCP resource list/read tools.
- `ontocode-rs/core/src/mcp_tool_call.rs` already looks up connector id/name, plugin id, tool title, tool description, annotations, MCP app resource URI, and app metadata for MCP tool calls.
- No exactly-one missing metadata fixture gap was found in the existing MCP resource/tool owners.
- Adding source metadata would create a new source-browsing/API surface and risks bypassing OntoIndex/security policy.

## Outcome

No implementation dispatch. Row 122 remains parked unless a precise existing MCP metadata field gap is proven without adding source-browsing API surface.
