name: Claude Parked Row 042 Review
desc: Row 042 stays parked because CI diagnostics and commit-amend prompt policy are different non-core surfaces
type: audit_session
date: 2026-06-20

# Claude Parked Row 042 Review

## Decision

Row 042 remains parked. No promotion packet.

## Evidence

- Parked ADR row 042 says CI diagnostics belong in GitHub skill/plugin first.
- Donor row 042 says to forbid commit amend unless the user explicitly asks.
- These sources describe different behaviors.
- Duplicate gate found no Gemini or Oh My Pi reopen slice.
- GitHub CI diagnostics already have a plugin skill owner in `gh-fix-ci`; `babysit-pr` is the companion PR watcher workflow.
- OntoIndex surfaced `permissions_instructions.rs` prompt assembly and `exec_policy.rs` amendment-rule tests as nearby git-policy owners.
- Worker review found the live on-request permission template warns about destructive actions such as `rm` and `git reset`, but no dedicated `commit --amend` prompt test.

## Closure

This is not one stable existing-owner failing test gap. Promoting it would move commit workflow policy into runtime core while the parked row is about CI diagnostics in GitHub skills/plugins.
