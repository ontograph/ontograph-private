name: Claude Parked Row 076 Review
desc: Row 076 stays parked because failed-approach tracking already belongs to memory templates and repo-only docs/process hygiene
type: audit_session
date: 2026-06-20

# Claude Parked Row 076 Review

## Decision

Row 076 remains parked. No promotion packet.

## Evidence

- Parked ADR row 076 is `NARROW` and says command guidance belongs in prompt/docs.
- Donor row 076 asks to track failed approaches in memory as `.memory-bank` practice.
- Duplicate gate keeps the row in the Gemini-overlapping context, memory, and prompt-cache bucket.
- The memory stage-one template already asks for hard-won shortcuts, failure shields, what failed, what worked instead, and how future agents should do it differently.
- The memory consolidation template already says reusable knowledge should include failure shields and recurring failure modes.
- Lean-ctx memory-bank tooling remains scoped to repository-only scripts and markdown checks, not runtime core behavior.

## Closure

Failed-approach tracking is already prompt/docs/process guidance. No core implementation gap was found, so the row stays parked.
