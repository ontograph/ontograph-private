# Claude Parked Row 169 Review

Date: 2026-06-20

## Decision

Row 169 stays parked.

## Source

- ADR row 169: `Existing` / `Non-core` / `DEFER` / `Rendering abstractions already exist; avoid churn.`
- Donor row 169: `Add virtual message list. | TUI | Handles long sessions efficiently. | Render regression test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-rs/tui/src/history_cell/mod.rs` already defines the history-cell contract, transcript render modes, desired-height measurement, and transcript overlay line generation.
- `ontocode-rs/tui/src/pager_overlay.rs` already owns transcript overlay scroll behavior and cached live-tail rendering.
- `ontocode-rs/tui/src/app/resize_reflow.rs`, `transcript_reflow.rs`, and `resize_reflow_cap.rs` already own source-backed scrollback reflow, debounce/final stream repair, and terminal-specific row caps.
- `ontocode-rs/tui/src/streaming/controller.rs` and `streaming/chunking.rs` already own streaming chunking, table holdback, and queue-pressure draining.
- Existing tests cover capped and uncapped resize reflow, thread-switch replay tail mode, transcript wrapping, source-backed consolidation, and overlay live-tail behavior.

## Outcome

No implementation dispatch. No fresh bug, regression, security, safety, product evidence, or concrete existing-owner performance regression gap was found. A virtual message list would add a new rendering abstraction, transcript storage model, rendering owner, or UI architecture. No Rust tests were run because no product code changed.
