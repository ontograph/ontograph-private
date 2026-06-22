name: Claude Parked Row 048 Review
desc: Row 048 stays parked because reviewer/issue lifecycle automation belongs outside core and no fresh evidence was found
type: audit_session
date: 2026-06-20

# Claude Parked Row 048 Review

## Decision

Row 048 remains parked. No promotion packet.

## Evidence

- Parked ADR row 048 says reviewer assignment is workflow automation, not core.
- Donor row 048 asks for issue lifecycle comment automation in `.github/workflows`, which is external repository automation rather than generated-code core behavior.
- Duplicate gate found no Gemini dispatchable slice and only external-workflow-level overlap with Oh My Pi.
- Existing owners are `.github/workflows/issue-deduplicator.yml`, `.github/workflows/issue-labeler.yml`, and `.codex/skills/babysit-pr/scripts/gh_pr_watch.py`.
- Worker evidence found `fetch_new_review_items` already reads issue-comment, review-comment, and review endpoints before CI checks in the PR babysitting path.
- Existing tests cover review-comment prioritization and review intake before CI checks; no fresh bug, regression, security, safety, or product evidence was found for a new lifecycle-comment workflow step.

## Closure

The row stays in the DEFER parking lot. Reopen only with specific evidence of a concrete failure in the existing issue or reviewer workflow path.
