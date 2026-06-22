# Claude Parked Row 081 Review

Date: 2026-06-20

## Scope

- Source ADR row: `ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 081.
- Donor source row: `CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 081.
- Tracking row: `CLAUDE_CODE_DONOR_DEFERRED_NARROW_REJECT_PRE_JUNIOR_PROJECT_PLAN.md` row 081.

## Decision

Row 081 stays parked. No promotion packet.

The parked ADR classifies the item as `DEFER` because speculative generation is risky and cache-hostile by default. The donor proposal asks for read-only prompt suggestion/speculation overlay behavior, which would add new product/runtime behavior rather than harden one current owner.

## Evidence

- Duplicate gates keep rows 081-095 in the parked context, memory, prompt-cache, and speculation bucket.
- OntoIndex exploration did not identify a clean existing owner for speculative prompt suggestions.
- Local source search found no current `ontocode-rs/core/src/next_prompt_suggestion.rs` or `ontocode-rs/core/src/context/next_prompt_suggestion.rs`; those names appear only in markdown-render fixture text.
- Existing nearby owners are broad session/model-client prompt-cache behavior and TUI overlay rendering, not a narrow owner-local failing test surface.
- No fresh bug, regression, security, safety, latency, or product evidence was found that would justify promoting the deferred item.

## Outcome

No core implementation task is created. Reopen only with a measured product need or regression that names one existing owner, one failing test, and a bounded no-cache-churn implementation path.
