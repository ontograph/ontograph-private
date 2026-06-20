name: Claude Parked Row 038 Review
desc: Row 038 stays parked because structured review metadata already exists and local-only review would add a second review service
type: audit_session
date: 2026-06-20

# Claude Parked Row 038 Review

## Decision

Row 038 remains parked. No promotion packet.

## Evidence

- Parked ADR row 038 says to keep only structured review metadata, not a new review service.
- Donor row 038 asks for local-only review mode separate from remote review in `core/src/tasks/review.rs`.
- Duplicate gate found no Gemini dispatchable slice and no Oh My Pi reopen signal beyond narrow test-first hardening.
- OntoIndex reports `ontocode-rs/core/src/tasks/review.rs` has public `new` and `exit_review_mode`; the file is 279 lines.
- OntoIndex found `process_review_events`, `parse_review_output_event`, and `exit_review_mode` as existing structured review-output owners.
- Worker review found `exit_review_mode` has one direct upstream caller, `ReviewTask.run`, emits `ExitedReviewModeEvent`, and has LOW impact.
- Existing review tests cover lifecycle/output, custom review model selection, session-model fallback, event filtering, and parent-history isolation.

## Closure

No existing-owner failing test proves a missing local-only-vs-remote review split. Adding that split would introduce a second review service, so the row stays parked.
