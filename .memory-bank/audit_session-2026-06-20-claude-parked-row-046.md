name: Claude Parked Row 046 Review
desc: Row 046 stays parked because changelog workflows and PR creation skills are non-core workflow/plugin territory
type: audit_session
date: 2026-06-20

# Claude Parked Row 046 Review

## Decision

Row 046 remains parked. No promotion packet.

## Evidence

- Parked ADR row 046 says changelog workflows are not core.
- Donor row 046 says to add PR creation command as plugin/skill in `core-skills` / GitHub plugin.
- Both are non-core workflow/plugin territory and not the same behavior.
- Duplicate gate found no dispatchable Gemini or Oh My Pi slice.
- GitHub PR creation already has a plugin skill owner in `github:yeet`; PR-body editing lives in `codex-pr-body`.
- Worker review found release-note/changelog-adjacent ownership in `cli/src/doctor/output.rs:notes_for_report` and release workflow scripts.
- Adjacent coverage exists for Changelog Writer skill display and release-notes display.

## Closure

DEFER promotion requires fresh bug, regression, security/safety, or senior-approved product evidence. None was found, so row 046 stays parked.
