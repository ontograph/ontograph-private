# Claude Parked Row 085 Review

Date: 2026-06-20

## Scope

- Source ADR row: `ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 085.
- Donor source row: `CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 085.
- Tracking row: `CLAUDE_CODE_DONOR_DEFERRED_NARROW_REJECT_PRE_JUNIOR_PROJECT_PLAN.md` row 085.

## Decision

Row 085 stays parked. No promotion packet.

The parked ADR classifies the item as `DEFER` because multi-branch generation is expensive and hard to verify. The donor proposal asks for speculative overlay write adoption, which would create new workspace/write behavior rather than harden an existing owner.

## Evidence

- Duplicate gates keep rows 081-095 in the parked context, memory, prompt-cache, and speculation bucket.
- OntoIndex exploration found no overlay/copy-back symbols for speculative workspace write adoption.
- Existing `exec-server` copy behavior is generic filesystem copy plumbing.
- Existing process env-overlay behavior is process-environment setup, not workspace overlay write adoption.
- No fresh bug, regression, security, safety, or product evidence was found to justify promoting the deferred item.

## Outcome

No core implementation task is created. Reopen only with explicit product evidence and a current owner that can express a failing test without adding speculative workspace branches or copy-back runtime behavior.
