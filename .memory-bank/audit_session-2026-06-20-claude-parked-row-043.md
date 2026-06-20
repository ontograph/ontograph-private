name: Claude Parked Row 043 Review
desc: Row 043 stays parked because release-note automation and commit secret warnings are different non-core surfaces
type: audit_session
date: 2026-06-20

# Claude Parked Row 043 Review

## Decision

Row 043 remains parked. No promotion packet.

## Evidence

- Parked ADR row 043 says release note generation is non-core automation.
- Donor row 043 says to warn before committing likely secrets in Git command prompt rules.
- These sources describe different behaviors.
- Duplicate gate found Gemini strips release/UI/eval/plugin overlap and Oh My Pi excludes release automation.
- OntoIndex search surfaced existing secret redaction owners in provider-auth, doctor output, external-agent import, and `secrets/src/sanitizer.rs`.
- Worker review found release-note ownership in `cli/src/doctor/output.rs` notes rendering and TUI update-popup snapshots.
- The only nearby git rule found was the generic no-commit instruction, not a dedicated secret-warning commit rule.

## Closure

No single owner-local failing test gap was found. Promoting this row would create commit-policy/runtime-core automation while the parked row is about non-core release-note generation.
