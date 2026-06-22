# Claude Parked Row 171 Review

Date: 2026-06-20

## Decision

Row 171 stays parked.

## Source

- ADR row 171: `New / Non-core / DEFER / Layout experiments need design task.`
- Donor row 171: `Add FPS metrics context for TUI. | TUI | Diagnoses terminal rendering. | Metrics test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `/debug-config` already owns a dedicated TUI diagnostics surface.
- TUI frame requesting already hard-caps redraw scheduling at 120fps and has rate-limit tests.
- Runtime metrics logging, OTEL metrics tests, and config schema coverage already cover adjacent metrics behavior.
- No fresh bug, regression, safety, security, product-demand, or owner-local metrics test gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
