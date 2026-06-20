name: Claude Parked Row 070 Review
desc: Row 070 stays parked because custom memory templates need a product boundary and no fresh DEFER promotion evidence was found
type: audit_session
date: 2026-06-20

# Claude Parked Row 070 Review

## Decision

Row 070 remains parked. No promotion packet.

## Evidence

- Parked ADR row 070 is `DEFER` and says personalization memory is risky without a product boundary.
- Donor row 070 asks to allow a custom memory template path in config / memory layer.
- Duplicate gate keeps this in parked memory scope: Gemini defers memory/config boundary work and Oh My Pi keeps memory rows owner-local unless a real gap appears.
- OntoIndex reports `ontocode-rs/memories/write/src/prompts.rs` as a 132-line memory prompt owner exposing `build_consolidation_prompt` and `build_stage_one_input_message`.
- The current memory consolidation prompt uses embedded templates plus memory extension blocks under the memory root, not arbitrary config paths.
- Config already supports unrelated compact-prompt overrides through `compact_prompt` and `experimental_compact_prompt_file`; that is not evidence for a custom memory-template product surface.
- Phase-2 memory consolidation disables recursive memory use for the internal worker.

## Closure

No fresh bug, regression, security, safety, or senior-approved product requirement was found. A custom memory template path would add config/product surface, so the row stays parked.
