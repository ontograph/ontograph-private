# Claude Parked Row 083 Review

Date: 2026-06-20

## Scope

- Source ADR row: `ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 083.
- Donor source row: `CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 083.
- Tracking row: `CLAUDE_CODE_DONOR_DEFERRED_NARROW_REJECT_PRE_JUNIOR_PROJECT_PLAN.md` row 083.

## Decision

Row 083 stays parked. No promotion packet.

The parked ADR classifies the item as `DEFER` because prediction cache should not be introduced without telemetry. The donor proposal asks to cap speculative messages, but no current prediction/speculation cache owner or row-specific telemetry exists.

## Evidence

- Duplicate gates keep rows 081-095 in the parked context, memory, prompt-cache, and speculation bucket.
- `ontocode-rs/core/src/context_manager` owns history, normalization, update plumbing, and token accounting, not prediction caching.
- The nearby cache found by the worker is image-byte estimate caching, unrelated to speculative prediction.
- OntoIndex surfaced prompt-cache-key and generic API telemetry paths, but not prediction-cache telemetry or a max-message speculation owner.
- No fresh bug, regression, security, safety, product, or telemetry evidence was found to justify promoting the deferred item.

## Outcome

No core implementation task is created. Reopen only with telemetry-backed product evidence and one existing owner that can express a failing max-message test without adding a new speculative cache stack.
