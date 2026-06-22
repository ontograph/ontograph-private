name: Claude Parked Row 050 Review
desc: Row 050 stays parked because duplicate issue auto-close and branch hygiene are repo automation without fresh DEFER evidence
type: audit_session
date: 2026-06-20

# Claude Parked Row 050 Review

## Decision

Row 050 remains parked. No promotion packet.

## Evidence

- Parked ADR row 050 says branch hygiene automation is outside generated-code core.
- Donor row 050 asks for auto-close duplicate issue flow with audit log in `.github/workflows` / scripts, which is repository operations automation rather than runtime core behavior.
- Duplicate gate found no Gemini dispatchable slice and no Oh My Pi reopen lane for issue-triage automation.
- Existing duplicate issue ownership lives in `.github/workflows/issue-deduplicator.yml`; it already fetches all/open issues, runs duplicate-identification prompts, normalizes output, comments with potential duplicates, and removes the trigger label.
- Issue summarization remains in `.codex/skills/codex-issue-digest`, with script-backed collection and tests.
- Branch hygiene remains separate workflow automation in `.github/workflows/close-stale-contributor-prs.yml`.
- OntoIndex surfaced issue-digest script/test owners but local workflow files were the authoritative evidence for the workflow-only donor request.
- No fresh bug, regression, security, safety, or product evidence was found.

## Closure

The row stays in the DEFER parking lot. Reopen only with specific evidence that existing repository automation has a concrete unsafe or broken behavior and that the fix belongs in an existing owner.
