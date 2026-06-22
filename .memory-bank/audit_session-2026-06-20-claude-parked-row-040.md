name: Claude Parked Row 040 Review
desc: Row 040 stays parked because issue triage remains non-core and the donor PR fallback is a different review-command surface
type: audit_session
date: 2026-06-20

# Claude Parked Row 040 Review

## Decision

Row 040 remains parked. No promotion packet.

## Evidence

- Parked ADR row 040 says issue triage workflow is useful but not core.
- Donor row 040 says to add PR selection fallback when PR number is missing in app-server / TUI review command.
- These sources describe different behaviors.
- Duplicate gate found Gemini keeps issue-triage automation external and Oh My Pi has no reopen lane for Claude command/UI/plugin overlaps.
- OntoIndex reports `ontocode-rs/app-server-protocol/src/protocol/v2/review.rs` is 66 lines.
- OntoIndex reports `ontocode-rs/tui/src/chatwidget/review_popups.rs` exposes `open_review_popup`, branch picker, commit picker, and custom prompt owners.
- Worker review found `tui/src/branch_summary.rs` owns PR resolution with current-branch lookup and head-commit fallback; `open_pull_request` has LOW impact and two direct callers.
- Existing PR lookup and base-branch review tests cover nearby behavior.

## Closure

DEFER promotion requires fresh bug, regression, security/safety, or senior-approved product evidence. None was found, so issue triage remains parked and the donor PR fallback does not create a promotion packet.
