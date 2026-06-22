name: Claude Parked Row 041 Review
desc: Row 041 stays parked because PR-body automation is a GitHub skill and donor commit automation is a different runtime surface
type: audit_session
date: 2026-06-20

# Claude Parked Row 041 Review

## Decision

Row 041 remains parked. No promotion packet.

## Evidence

- Parked ADR row 041 says PR-body automation belongs in GitHub skill/plugin.
- Donor row 041 says to add a commit command with scoped git allowlist in the TUI command layer / core task.
- These sources describe different behaviors.
- Duplicate gate found no matching Gemini or Oh My Pi pre-junior slice.
- OntoIndex reports `.codex/skills/codex-pr-body/SKILL.md` is a 62-line PR-body skill owner.
- Worker review found `tui/src/branch_summary.rs` owns TUI git/GitHub probes and `git-utils/src/baseline.rs` only has internal baseline commit scaffolding.
- Existing PR lookup tests cover current-branch view, parent-repo fallback, and open-PR parsing.

## Closure

No single owner-local failing test gap was found. A scoped commit command would add runtime commit automation rather than extend the existing GitHub PR-body skill boundary.
