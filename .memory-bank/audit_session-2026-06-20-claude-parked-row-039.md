name: Claude Parked Row 039 Review
desc: Row 039 stays parked because the parked git-helper idea and donor branch-diff prompt idea are different surfaces
type: audit_session
date: 2026-06-20

# Claude Parked Row 039 Review

## Decision

Row 039 remains parked. No promotion packet.

## Evidence

- Parked ADR row 039 says git commit helpers should stay plugin/skill unless core state changes.
- Donor row 039 says to add a branch diff preamble to review prompts in `core/src/tasks/review.rs`.
- These sources describe different owner surfaces: git-helper workflow policy versus review prompt generation.
- Duplicate gate found no Gemini dispatchable parked-row slice and no Oh My Pi reopen signal.
- OntoIndex reports `ontocode-rs/prompts/src/review_request.rs` exports `resolve_review_request`, `review_prompt`, `user_facing_hint`, and `REVIEW_PROMPT`; the file is 138 lines.
- Current review prompt code already includes base-branch merge-base and `git diff` preambles.
- Existing prompt tests cover both base-branch prompt variants, and skill-loader tests cover nearby plugin skill namespacing behavior.

## Closure

No single owner-local failing test gap was found. Promoting this row would either duplicate existing prompt behavior or pull git-helper workflow policy into the wrong core boundary.
