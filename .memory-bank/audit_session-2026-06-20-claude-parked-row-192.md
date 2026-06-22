# Claude Parked Row 192 Review

Date: 2026-06-20

## Decision

Row 192 stays parked.

## Source

- ADR row 192: `Partial | Conditional | NARROW | Plugin permission checks can extend existing plugin manager.`
- Donor row 192: `Add MCP test script. | scripts/tests | Quick MCP smoke. | MCP script smoke.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-rs/codex-mcp/src/mcp/mod.rs` already defines the MCP permission auto-approval gate over approval policy, permission profile, and tool approval mode.
- `ontocode-rs/codex-mcp/src/mcp/mod_tests.rs` already tests unrestricted managed profiles, read-only profiles, non-never approval policy, `AppToolApproval::Approve`, and `AppToolApproval::Auto`.
- `ontocode-rs/core-plugins/src/loader.rs` already applies per-plugin MCP server policy into the existing plugin manager path.
- `ontocode-rs/core-plugins/src/manager_tests.rs` already exercises plugin `.mcp.json` loading plus config overlay of enabled state, default tool approval, enabled/disabled tools, and per-tool approval.
- Existing CLI MCP, MCP-server process, RMCP client, and connection-manager tests cover MCP smoke behavior without a separate script harness.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
