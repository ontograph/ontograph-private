# Claude Parked Row 095 Review

Date: 2026-06-20

## Decision

Row 095 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 095 says to avoid frequent prompt mutation and enforce stability.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 095 proposes adding a model-visible nudge message for budget continuation under prompts / context fragments.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `ontocode-rs/prompts/src/goals_tests.rs` already covers continuation prompt and budget-limit prompt behavior.
- `ontocode-rs/prompts/templates/goals/budget_limit.md` already contains the budget-limit wrap-up nudge.
- `ontocode-rs/core/src/goals.rs` injects goal steering through the existing hidden `goal` internal context item.
- Session coverage already verifies budget-limited steering without aborting the active turn.

## Outcome

No implementation dispatch. Adding another model-visible nudge or context fragment would create prompt churn rather than harden an uncovered fixture.
