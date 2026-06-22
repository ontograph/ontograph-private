name: Claude Parked Row 066 Review
desc: Row 066 stays parked because autonomous memory extraction already has a bounded write pipeline and the donor idea is duplicate-blocked
type: audit_session
date: 2026-06-20

# Claude Parked Row 066 Review

## Decision

Row 066 remains parked. No promotion packet.

## Evidence

- Parked ADR row 066 says autonomous memory writes require explicit policy and caps.
- Donor row 066 asks to use a forked subagent for memory extraction in `core/src/context_manager`, but current ownership is `ontocode-rs/memories/write`.
- Duplicate gate blocks promotion because the Gemini pre-junior plan explicitly treats Claude memory and prompt-cache rows 066-078 as non-dispatchable duplicates.
- `start_memories_startup_task` skips ephemeral sessions, disabled memory feature, and sub-agent sessions before spawning the background memory pipeline.
- The memory pipeline already uses rate-limit checks, bounded startup claims, concurrency caps, retry backoff, redaction, and bounded Phase 2 selection.
- Phase 2 already claims a global lock, writes a bounded workspace diff, and spawns a locked-down internal consolidation sub-agent with memory disabled, no approvals, no network, local memory-root write access only, and collab disabled.
- Existing tests cover startup/root creation, workspace diff/pruning, prompt shape, and rate limiting.
- OntoIndex reports `ontocode-rs/memories/write/src/start.rs` exposes only `start_memories_startup_task` and is a thin 80-line owner.
- No exactly-one owner-local failing test gap was found.

## Closure

The row stays in the NARROW parking lot. Reopen only if a senior identifies one missing invariant inside the existing memory write pipeline without moving memory extraction into context-manager or adding unbounded autonomous writes.
