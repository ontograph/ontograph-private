name: Claude Parked Row 049 Review
desc: Row 049 stays parked because merge babysitting and workflow-permission checks are external automation without fresh DEFER evidence
type: audit_session
date: 2026-06-20

# Claude Parked Row 049 Review

## Decision

Row 049 remains parked. No promotion packet.

## Evidence

- Parked ADR row 049 says merge babysitting is plugin behavior.
- Donor row 049 asks for a non-write-user CI check in `.github/workflows`, which is repository workflow/security automation rather than generated-code core behavior.
- Duplicate gate found no Gemini dispatchable slice and no Oh My Pi reopen lane for broader Claude plugin/runtime automation.
- Existing PR babysitting ownership lives in `.codex/skills/babysit-pr`, whose watcher checks CI, review comments, mergeability, and merge state.
- Workflow permission evidence stays in `.github/workflows`; write-token workflows are explicit, and `close-stale-contributor-prs.yml` already checks collaborator permission before acting on contributor PRs.
- OntoIndex semantic search did not return indexed workflow owners for this automation surface, so local workflow and skill files are the authoritative evidence for this row.
- No fresh bug, regression, security, safety, or product evidence was found.

## Closure

The row stays in the DEFER parking lot. Reopen only with specific evidence that existing PR babysitting or workflow-permission automation has a concrete unsafe behavior that belongs in an existing owner.
