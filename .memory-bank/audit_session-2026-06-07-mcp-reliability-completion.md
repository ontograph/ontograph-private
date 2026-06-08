---
name: Audit Session - MCP Reliability Epic Completion
date: 2026-06-07
type: audit
status: completed
---

# Audit Session - MCP Reliability Epic Completion

## Summary

Verified and closed the "MCP reliability and auth hardening" epic (IDs: 141-146, 149-151, 155-157).

## Verification Results

- **codex-mcp Tests**: 71/71 passed.
- **codex-core (mcp_tool_call) Tests**: 72/72 passed.
- **codex-rmcp-client Tests**: 63/63 passed.
- **Bench Smoke**: `just bench-smoke` passed.
- **Refactor Verification**: Custom `run_paginated_operation` helper in `RmcpClient` correctly handles cursors and duplicate detection.

## Key Improvements

- **Scalable Discovery**: Tools, resources, and templates are now fetched using pagination, supporting large MCP servers.
- **Enhanced Security**: Images are automatically stripped from tool results if the active model does not support image input.
- **Better UX**: Improved error reporting when `mcp_servers` configuration is malformed.
- **Robust Auth**: OAuth tokens are proactively refreshed with a 60s timeout to prevent session hangs.

## Side Effects

- Refactored `list_resources` and `list_resource_templates` in `connection_manager.rs` and `Session` to remove redundant `params`.
- Added `tokio-util` dependency (initially tried with `sync` feature, but reverted to manual timeout to avoid workspace-wide conflicts).

## Next Steps

- Transition to "Hook and shell permission safety" epic.
