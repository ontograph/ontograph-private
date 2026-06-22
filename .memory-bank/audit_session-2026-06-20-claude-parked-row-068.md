name: Claude Parked Row 068 Review
desc: Row 068 stays parked because current memory reads already use existing memory-read and context-fragment owners without a single promotable test gap
type: audit_session
date: 2026-06-20

# Claude Parked Row 068 Review

## Decision

Row 068 remains parked. No promotion packet.

## Evidence

- Parked ADR row 068 is `NARROW` and says to keep the idea as a verification rule, not new storage.
- Donor row 068 asks to read current memory through the existing file-read path in `core/src/context`.
- Duplicate gate blocks promotion because the parked ADR groups rows 066-078 as Gemini-overlapping context, memory, and prompt-cache scope.
- `ontocode-rs/memories/read/src/lib.rs` owns memory read-path helpers, memory citation parsing, and telemetry classification without depending on the write pipeline.
- `ontocode-rs/memories/read/src/usage.rs` classifies safe read/search commands for `MEMORY.md`, `memory_summary.md`, `raw_memories.md`, rollout summaries, and memory skills.
- Existing context owners already constrain injected hidden context: `InternalModelContextFragment` validates source labels, contextual-user-fragment tests reject arbitrary context tags, and `DiagnosticFragment` demonstrates a hard-cap pattern.

## Closure

No exactly-one existing-owner failing test gap was found. A new current-memory read or injection path would duplicate existing memory-read and bounded context-fragment boundaries, so the row stays parked.
