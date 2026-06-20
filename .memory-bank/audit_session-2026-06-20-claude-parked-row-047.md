name: Claude Parked Row 047 Review
desc: Row 047 stays parked because duplicate issue triage is already external automation and no fresh DEFER evidence exists
type: audit_session
date: 2026-06-20

# Claude Parked Row 047 Review

## Decision

Row 047 remains parked. No promotion packet.

## Evidence

- Parked ADR row 047 says marketplace-style triage belongs outside core.
- Donor row 047 says to add duplicate issue triage workflow as external automation in `.github/workflows` / scripts.
- Duplicate gate found Gemini rejects issue-triage automation as product/runtime behavior and Oh My Pi has no reopen lane.
- OntoIndex reports `.codex/skills/codex-issue-digest/SKILL.md` is a 128-line external issue-digest owner.
- OntoIndex surfaced the issue-digest script family and digest-row/ranking tests.
- Worker review found live `.github/workflows/issue-deduplicator.yml` and `issue-labeler.yml` owners.
- Existing script coverage is `pytest .codex/skills/codex-issue-digest/scripts/test_collect_issue_digest.py`.

## Closure

DEFER promotion requires fresh bug, regression, security/safety, or senior-approved product evidence. None was found, so duplicate issue triage remains parked outside core.
