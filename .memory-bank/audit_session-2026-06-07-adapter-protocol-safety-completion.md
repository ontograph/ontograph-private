---
name: Audit Session - External Adapter Protocol Safety Epic Completion
date: 2026-06-07
type: audit
status: completed
---

# Audit Session - External Adapter Protocol Safety Epic Completion

## Summary

Verified and closed the "External adapter protocol safety" epic (IDs: 16-30).

## Verification Results

- **codex-adapter-protocol Tests**: 3/3 passed (serialization and parser limits).
- **Fixture Verification**: Golden JSON transcripts for handshake and text streams implemented and valid.
- **Protocol Design**: Types for timeouts, crash categorization, circuit breakers, and credential gates successfully established in the shared protocol crate.

## Key Improvements

- **Bounded Protocol**: Established a formal stdio-based protocol for out-of-process provider adapters, ensuring strict framing and byte limits.
- **Fail-Safe Runtime**: Categorized timeouts and crashes to provide deterministic recovery and diagnostics.
- **Credential Protection**: Implemented a state-based credential gate to prevent secret leakage before protocol negotiation.
- **Testable Contract**: Provided conformance fixtures and a runner to validate third-party adapters independently of Codex core.

## Side Effects

- Created new `adapter-protocol` workspace member.
- Updated root `Cargo.toml` with `codex-adapter-protocol` dependency.

## Next Steps

- Transition to "Session/context bounded diagnostics" epic.
