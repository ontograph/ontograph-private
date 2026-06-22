# Claude Parked Row 166 Review

Date: 2026-06-20

## Decision

Row 166 stays parked.

## Source

- ADR row 166: `Partial` / `Non-core` / `NARROW` / `Diagnostics display can improve supportability.`
- Donor row 166: `Add IDE selection context indicator. | TUI / context | Makes active selection visible. | Snapshot test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-rs/tui/src/chatwidget/ide_context.rs` already owns `/ide` enable, disable, status, prompt injection, and footer indicator synchronization.
- `ontocode-rs/tui/src/ide_context/prompt.rs` already renders active file, active selection ranges/content, and open tabs with focused tests for format, truncation, and tab omission.
- `ontocode-rs/tui/src/bottom_pane/footer.rs` already renders the `IDE context` right-side footer indicator and snapshots the active indicator case.
- `ontocode-rs/tui/src/chatwidget/tests/composer_submission.rs` already verifies IDE context prompt prefixes remain hidden from the user-facing transcript.

## Outcome

No implementation dispatch. No exactly-one existing TUI/context diagnostics display snapshot or status gap was found without adding a new IDE integration, context fragment, app-server API, status pane, or UI surface. No Rust tests were run because no product code changed.
