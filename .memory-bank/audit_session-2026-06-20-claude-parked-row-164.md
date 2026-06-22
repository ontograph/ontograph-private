# Claude Parked Row 164 Review

Date: 2026-06-20

## Decision

Row 164 stays parked.

## Source

- ADR row 164: `Partial / Non-core / DEFER / Conversation decorations are not core.`
- Donor row 164: `Add desktop MCP import dialog. | app-server / MCP | Helps migrate desktop configs. | Import fixture.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing app-server external-agent import already handles MCP config migration from repo and home settings into `mcp_servers`.
- Existing tests cover MCP config import, settings toggles, project config, and invalid settings behavior.
- No fresh bug, regression, security, safety, or product evidence was found for adding a desktop MCP import dialog.
- No concrete desktop-import fixture gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
