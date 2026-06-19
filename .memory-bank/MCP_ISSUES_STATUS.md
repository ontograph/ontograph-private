# MCP Issues Status

Date: 2026-06-15

## Current Status

No open MCP reliability tasks are listed in the current memory-bank plan. The MCP reliability and auth hardening epic is recorded as complete in `project_pending-tasks.md` and closed by `audit_session-2026-06-07-mcp-reliability-completion.md`.

## Completed Fix Areas

- Paginated MCP tool, resource, and resource-template discovery for large servers.
- Improved malformed `mcp_servers` configuration diagnostics.
- Image result sanitization for non-vision models in MCP tool-call output.
- Proactive OAuth token refresh in `RmcpClient` with a 60s timeout.
- Hook/MCP interaction coverage for blocking and rewriting MCP tool calls before execution.

## Verified At Closure

- `codex-mcp` tests: 71/71 passed.
- `codex-core` MCP tool-call tests: 72/72 passed.
- `codex-rmcp-client` tests: 63/63 passed.
- `just bench-smoke` passed.

## Current Risk Areas

- Claude MCP OAuth live validation remains blocked until a real redacted credential sample is available.
- MCP runtime changes must reuse the existing owners rather than adding a parallel manager:
  - `ontocode-rs/codex-mcp/src/connection_manager.rs`
  - `ontocode-rs/codex-mcp/src/mcp/mod.rs`
  - `ontocode-rs/rmcp-client/src/oauth.rs`
  - existing app-server MCP processors
- New slash-command or agent-management work should not introduce a second MCP tool registry, status pipeline, OAuth store, or credential broker.

## Relevant Tests

- `ontocode-rs/core/tests/suite/hooks_mcp.rs`
- `ontocode-rs/core/tests/suite/code_mode.rs`
- `ontocode-rs/core/tests/common/responses.rs`

