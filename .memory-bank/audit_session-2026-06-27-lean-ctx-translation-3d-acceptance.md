# Lean-ctx Translation 3D Acceptance

Date: 2026-06-27
Scope: `ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL.md`

## Decision

Accepted the plugin-only lean-ctx adapter path.

## OntoIndex Evidence

- `ontocode-rs/core-plugins/src/manifest.rs` already owns plugin manifest loading.
- `ontocode-rs/core-plugins/src/loader.rs` already owns plugin loading and plugin-bundled MCP server discovery.
- `ontocode-rs/core/src/mcp_tool_call.rs` already owns MCP tool-call lifecycle and provenance-bearing dispatch.

## Accepted Boundaries

- Keep lean-ctx external and authoritative for daemon behavior.
- Keep the first cut to `/health`, `/v1/manifest`, `/v1/tools`, and `/v1/tools/call`.
- Keep the first allowlist to `ctx_read`, `ctx_search`, and `ctx_summary`.
- No local cache, wrapper generator, native runtime, adapter-specific config family, app-server API, or SDK API in the first cut.

## Opened Task

- `L3D-P1` Phase 1 package skeleton is now the first active task in the tracking file.
