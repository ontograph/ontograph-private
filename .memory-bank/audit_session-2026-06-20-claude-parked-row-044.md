name: Claude Parked Row 044 Review
desc: Row 044 stays parked because repo workflow assistant scope and HEREDOC commit prompt rules are different surfaces
type: audit_session
date: 2026-06-20

# Claude Parked Row 044 Review

## Decision

Row 044 remains parked. No promotion packet.

## Evidence

- Parked ADR row 044 says repo workflow assistant should not enter core runtime.
- Donor row 044 says to use HEREDOC commit messages in Git command prompt rules.
- These sources describe different behaviors.
- Duplicate gate found no exact Gemini or Oh My Pi duplicate and no reopen lane.
- OntoIndex reports `ontocode-rs/core/src/exec_policy.rs` owns `create_exec_approval_requirement_for_command`; the file is 1050 lines.
- Current HEREDOC-related tests cover amendment suppression and prefix parsing cases.
- Worker review found HEREDOC handling is already a shell/exec-policy concern, not a missing repo-workflow runtime feature.

## Closure

DEFER promotion requires fresh bug, regression, security/safety, or senior-approved product evidence. None was found, so the row stays parked.
