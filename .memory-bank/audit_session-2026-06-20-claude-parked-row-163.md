# Claude Parked Row 163 Review

Date: 2026-06-20

## Decision

Row 163 stays parked.

## Source

- ADR row 163: `Partial / Non-core / DEFER / UI-only command palette changes need product demand.`
- Donor row 163: `Add MCP multiselect dialog for import. | TUI / MCP | Safer bulk server selection. | Snapshot test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing external-agent MCP import service and app-server tests cover import behavior without a TUI multiselect.
- The reusable TUI `MultiSelectPicker` already has snapshot coverage in other setup flows.
- No fresh bug, regression, security, safety, or product evidence was found for adding an MCP import multiselect dialog.
- No concrete MCP-import multiselect snapshot gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
