# Claude Parked Row 082 Review

Date: 2026-06-20

## Scope

- Source ADR row: `ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 082.
- Donor source row: `CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 082.
- Tracking row: `CLAUDE_CODE_DONOR_DEFERRED_NARROW_REJECT_PRE_JUNIOR_PROJECT_PLAN.md` row 082.

## Decision

Row 082 stays parked. No promotion packet.

The parked ADR classifies the item as `DEFER` because background speculation needs a measured latency win first. The donor proposal asks to cap speculative background turns, but no current background-speculation implementation was found to bound.

## Evidence

- Duplicate gates keep rows 081-095 in the parked context, memory, prompt-cache, and speculation bucket.
- `ontocode-rs/core/src/context_manager` currently exposes history, normalization, and update modules rather than a background speculation runtime.
- OntoIndex exploration for background speculation and turn limits surfaced compaction and generic tool-suggestion paths, not a prompt-speculation owner.
- No measured latency, product, bug, regression, security, or safety evidence was found to justify promoting the deferred item.

## Outcome

No core implementation task is created. Reopen only if a measured latency/product requirement names one existing owner and a failing max-turn regression test without adding a speculative cache or new background generation stack.
