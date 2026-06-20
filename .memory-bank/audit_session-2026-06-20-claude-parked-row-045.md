name: Claude Parked Row 045 Review
desc: Row 045 stays parked because review templates and recent-commit commit prompts are split non-core surfaces
type: audit_session
date: 2026-06-20

# Claude Parked Row 045 Review

## Decision

Row 045 remains parked. No promotion packet.

## Evidence

- Parked ADR row 045 says review templates are useful as memory-bank/prompt assets.
- Donor row 045 says to include recent commit style in commit prompt rules.
- These sources describe different behaviors.
- Duplicate gate found no exact Gemini or Oh My Pi duplicate.
- OntoIndex reports `ontocode-rs/git-utils/src/info.rs` exports `recent_commits` and other git-info helpers; the file is 890 lines.
- Worker review found `review_prompt` is owned by `prompts/src/review_request.rs`, `recent_commits` by `git-utils/src/info.rs`, and commit-request assembly by `exec/src/lib.rs`.
- Existing tests cover review prompt rendering, commit request shape, and recent-commit collection, but not a shared commit-prompt seam.

## Closure

No single owner-local failing test gap was found. Promoting this row would add commit prompt/automation behavior into runtime core while the parked row is about review templates as assets.
