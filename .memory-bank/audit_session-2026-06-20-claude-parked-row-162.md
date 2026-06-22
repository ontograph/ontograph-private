# Claude Parked Row 162 Review

Date: 2026-06-20

## Decision

Row 162 stays parked.

## Source

- ADR row 162: `Partial / Non-core / NARROW / TUI review panes need snapshot-backed UI task.`
- Donor row 162: `Add MCP server approval dialog copy tests. | TUI / MCP | Prevents confusing trust prompts. | Snapshot test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Core MCP tool-call tests already assert both server-named and app-labeled approval copy variants.
- TUI MCP server elicitation snapshots already cover approval fallback, session-persist, and parameter-summary approval variants.
- No single MCP trust/approval dialog copy snapshot gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
