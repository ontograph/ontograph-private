name: Claude Parked Row 071 Review
desc: Row 071 stays parked because cross-project memory sharing and custom memory update prompts are mismatched DEFER surfaces without fresh evidence
type: audit_session
date: 2026-06-20

# Claude Parked Row 071 Review

## Decision

Row 071 remains parked. No promotion packet.

## Evidence

- Parked ADR row 071 is `DEFER` and says cross-project memory sharing should not be core now.
- Donor row 071 asks to allow a custom memory update prompt in config / memory layer.
- The ADR row and donor row are mismatched; they do not describe one exact feature to promote.
- Duplicate gate keeps the row in parked memory, context, and prompt-cache scope.
- `ontocode-rs/memories/README.md` documents `memories/read` as the read-path owner and `memories/write` as the write-path owner for Phase 1/2 prompt rendering and templates.
- `ontocode-rs/memories/write/src/prompts.rs` owns embedded memory prompt templates through `build_consolidation_prompt` and `build_stage_one_input_message`.
- Current config prompt overrides cover compact and guardian prompts, not a memory-update prompt key.
- TUI memory settings only persist `use_memories` and `generate_memories`, and `/memory update` is still reported as unavailable.

## Closure

No fresh bug, regression, security, safety, or senior-approved product requirement was found. Cross-project sharing and custom memory-update prompts remain product/config decisions, so the row stays parked.
