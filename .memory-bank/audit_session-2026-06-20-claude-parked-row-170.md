# Claude Parked Row 170 Review

Date: 2026-06-20

## Decision

Row 170 stays parked.

## Source

- ADR row 170: `New` / `Non-core` / `DEFER` / `Theme changes are cosmetic.`
- Donor row 170: `Add offscreen freeze for expensive components. | TUI | Reduces render cost. | Performance smoke.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-rs/tui/src/pager_overlay.rs` already owns cached live-tail overlay rendering and no-op recomputation when the cache key is unchanged.
- `ontocode-rs/tui/src/app/resize_reflow.rs` and `resize_reflow_cap.rs` already own source-backed resize reflow and terminal-specific row caps.
- `ontocode-rs/tui/src/streaming/chunking.rs` and `streaming/commit_tick.rs` already own adaptive queue-pressure catch-up behavior.
- `ontocode-rs/tui/src/streaming/controller.rs` already owns table holdback and stable-prefix caching for dense streamed tables.
- Existing tests cover live-tail snapshots/no-op behavior, resize reflow smoke/regressions, and streaming table-tail reflow behavior.

## Outcome

No implementation dispatch. No fresh bug, regression, security, safety, product evidence, or concrete existing-owner performance smoke gap was found. Offscreen freeze would add a new subsystem, retained component tree, scheduler, theme system, or UI architecture. No Rust tests were run because no product code changed.
