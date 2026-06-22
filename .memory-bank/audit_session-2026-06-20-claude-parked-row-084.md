# Claude Parked Row 084 Review

Date: 2026-06-20

## Scope

- Source ADR row: `ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 084.
- Donor source row: `CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 084.
- Tracking row: `CLAUDE_CODE_DONOR_DEFERRED_NARROW_REJECT_PRE_JUNIOR_PROJECT_PLAN.md` row 084.

## Decision

Row 084 stays parked. No promotion packet.

The parked ADR allows only deterministic prompt-cache constraints for this row. The donor proposal asks for a safe read-only tool list in speculation, but current Ontocode has no speculation runtime to constrain.

## Evidence

- Duplicate gates keep rows 081-095 in the parked context, memory, prompt-cache, and speculation bucket.
- `ontocode-rs/core/src/tools/spec_plan.rs` is the generic tool-router planner, not a speculation-specific owner.
- `spec_plan_tests.rs` already covers deterministic visible-tool names, registered tool names, namespace grouping, and exposure gating.
- Promoting this would require creating speculation-specific tool behavior rather than adding one owner-local failing test.

## Outcome

No core implementation task is created. Reopen only if an existing prompt-cache or tool-router regression is proven without introducing speculative runtime behavior.
