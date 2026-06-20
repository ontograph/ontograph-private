name: Claude Parked Row 067 Review
desc: Row 067 stays parked because strict memory-file permissions need explicit cross-platform policy and security evidence rather than memory-bank docs discipline
type: audit_session
date: 2026-06-20

# Claude Parked Row 067 Review

## Decision

Row 067 remains parked. No promotion packet.

## Evidence

- Parked ADR row 067 is `NARROW` and says the current memory-bank process can absorb the idea as docs discipline.
- Donor row 067 asks for strict file permissions for memory files in `thread-store` / memory layer, with a permission-mode test.
- Duplicate gate blocks promotion because the parked ADR groups rows 066-078 as Gemini-overlapping context, memory, and prompt-cache scope.
- OntoIndex reports `ontocode-rs/memories/write/src/workspace.rs` as a 118-line memory workspace owner exposing `prepare_memory_workspace`, `memory_workspace_diff`, `write_workspace_diff`, and baseline reset APIs.
- Current memory workspace and thread-store evidence uses default file creation for generated memory artifacts and thread files; no explicit permission-mode policy owner was found.

## Closure

Strict memory-file permissions are broader than one existing-owner failing test unless a senior first defines cross-platform policy, compatibility expectations, and concrete security evidence. The row stays parked.
