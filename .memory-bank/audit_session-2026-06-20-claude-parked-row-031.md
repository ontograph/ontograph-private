name: Claude Parked Row 031 Review
desc: Row 031 stays parked because review command behavior already exists in the review prompt, TUI, and session owners
type: audit_session
date: 2026-06-20

# Claude Parked Row 031 Review

## Decision

Row 031 remains parked. No promotion packet.

## Evidence

- Parked ADR row 031 says review command ideas belong in the existing review prompt/skill path.
- Donor row 031 asks for a first-class review prompt command in `core/src/tasks/review.rs`.
- Current code already exposes `/review` through the TUI slash-command path, including popup presets and inline custom instructions.
- OntoIndex reports `ontocode-rs/core/src/tasks/review.rs` has public `new` and `exit_review_mode`; the file is 279 lines.
- Worker review found `core/src/session/handlers.rs:review` has LOW upstream impact through `submission_loop`, `resolve_review_request`, and `spawn_review_thread`.
- Existing tests cover review prompt rendering, queued `/review` with args, and core review lifecycle/output behavior.

## Closure

This is not reducible to one missing existing-owner test. Promoting it would reopen or rename existing command architecture rather than extend a proven gap, so the row stays parked.
