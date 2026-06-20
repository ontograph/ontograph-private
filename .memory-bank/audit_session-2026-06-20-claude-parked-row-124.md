# Claude Parked Row 124 Review

Date: 2026-06-20

## Decision

Row 124 stays rejected.

## Source

- ADR row 124: `Existing | Non-core | REJECT | Raw code exposure duplicates existing tools and risks leakage.`
- Donor row 124: `Add get-tool-source MCP endpoint gated to dev mode. | codex-mcp | Improves local debugging. | Dev-only permission test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Sub-agent `019ee533-048a-72d2-b819-60db89634031` recommended rejected and made no edits.
- `ontocode-rs/core/src/tools/handlers/tool_search_spec.rs` tells callers to use `tool_search` for MCP discovery.
- `ontocode-rs/core/src/tools/handlers/mcp_resource/list_mcp_resources.rs` and `ontocode-rs/core/src/tools/handlers/mcp_resource/read_mcp_resource.rs` own the standard MCP resource list/read tools.
- `ontocode-rs/core/src/tools/handlers/mcp_resource_spec_tests.rs` covers the MCP resource list/read tool specs.
- `ontocode-rs/codex-mcp/src/tools.rs` keeps raw identities for protocol routing while exposing sanitized model-facing tool names.
- `ontocode-rs/codex-mcp/src/mcp/mod.rs` contains approval-prompt gating, not a raw-source publication path.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
