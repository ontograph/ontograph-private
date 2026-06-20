# Claude Parked Row 172 Review

Date: 2026-06-20

## Decision

Row 172 stays parked.

## Source

- ADR row 172: `Partial / Conditional / NARROW / Error topology display can surface existing structured errors.`
- Donor row 172: `Add token warning component. | TUI / context manager | Warns before compaction pressure. | Snapshot test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Core session turn logic already computes auto-compaction token status and context-window exhaustion.
- TUI bottom-pane and status-card owners already expose context-window usage/remaining state.
- Existing tests cover context indicator reset/unknown-window behavior, runtime context-window updates, context-window error handling, and pre-turn compaction request shape.
- No exactly-one owner-local missing test or doc gap was found for a token warning or existing structured-error display.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
