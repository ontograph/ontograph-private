name: Claude Parked Row 073 Review
desc: Row 073 stays parked because memory/context token caps already span existing owners and central context assembly is CRITICAL impact
type: audit_session
date: 2026-06-20

# Claude Parked Row 073 Review

## Decision

Row 073 remains parked. No promotion packet.

## Evidence

- Parked ADR row 073 is `NARROW` and says to respect existing context-fragment architecture.
- Donor row 073 asks for a total session memory token limit in `core/src/context`.
- Duplicate gate keeps the row in the Gemini-overlapping context, memory, and prompt-cache bucket.
- Existing caps are already split across established owners:
  - `AdditionalContextUserFragment` and `AdditionalContextDeveloperFragment` cap individual additional-context values.
  - `DiagnosticFragment` caps diagnostic fragments.
  - Realtime startup context has per-section token budgets and a requested total budget.
  - AGENTS.md loading uses byte caps.
  - Memory stage-one input truncates rollout content to 70% of the effective context window, with a fallback token limit.
  - Compaction code already clamps token limits for selected messages.
- OntoIndex reports `Session.build_initial_context` has CRITICAL upstream impact: 44 impacted nodes, 29 direct callers, 7 affected modules, and the `run_turn` process affected.

## Closure

No exactly-one existing-owner failing test gap was found. Adding a total session memory limit would touch high-blast-radius context assembly or create a parallel injection path, so the row stays parked.
