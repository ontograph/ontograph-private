# Lefties: Lean-ctx Project Tool Extensions

## Status

Deferred

## Date

2026-06-07

## Source

Moved from `.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` during review/challenge.

## Reason

These items may be useful for agent workflow polish, but they do not naturally extend the current project core. They are release-management or final-response helpers rather than architecture, provider, OAuth, MCP, shell, context, GitNexus, memory-bank, or test/diff enforcement tools.

They should not be implemented as part of the lean-ctx project tool extension ADR unless a later workflow ADR defines a release-management automation layer.

## Moved Items

197. `onto_release_readiness_report`: combine tests, GitNexus, tracking, and residual risks.
198. `onto_release_rollback_notes`: generate rollback notes for risky changes.
199. `onto_release_changelog_candidate`: draft concise changelog entries for approved changes.
200. `onto_release_final_answer`: generate final user-facing completion summary with evidence.

## Challenge Notes

- `onto_release_readiness_report` overlaps with task readiness and final review, but the release framing is too broad for this ADR.
- `onto_release_rollback_notes` belongs to release/process management, not project core extension tooling.
- `onto_release_changelog_candidate` belongs to changelog/release policy and should not be hidden in lean-ctx tooling.
- `onto_release_final_answer` is assistant behavior, not a codebase tool.

## Reconsideration Criteria

Reconsider only if the project creates an explicit release automation ADR with:

- release-stage ownership
- changelog policy
- rollback policy
- evidence retention requirements
- compatibility with GitNexus and memory-bank tracking
