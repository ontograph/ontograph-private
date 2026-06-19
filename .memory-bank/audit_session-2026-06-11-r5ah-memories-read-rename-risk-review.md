# R5AH Memories Read Rename Risk Review

Date: 2026-06-11

## Candidate

- `codex-memories-read` -> `ontocode-memories-read`
- `codex_memories_read` -> `ontocode_memories_read`

## Inventory

- Cargo metadata direct reverse dependencies: `ontocode-core`
- Active direct refs: 11
- Ref locations: root workspace metadata, core dependency/import usage, memories README identity text, and memories/read manifest/Bazel identity.

## OntoIndex Impact

- `Function:ontocode-rs/memories/read/src/lib.rs:memory_root`: LOW, 0 impacted, 0 processes.
- `Function:ontocode-rs/memories/read/src/citations.rs:parse_memory_citation`: CRITICAL, 11 impacted, 5 direct, 5 modules, 0 processes.
- `Function:ontocode-rs/memories/read/src/citations.rs:thread_ids_from_memory_citation`: LOW, 5 impacted, 1 direct, 2 modules, 0 processes.
- `Function:ontocode-rs/memories/read/src/usage.rs:memories_usage_kinds_from_command`: LOW, 4 impacted, 1 direct, 2 modules, 0 processes.

## Decision

- Proceed as an identity-only package/lib/Bazel/import rename.
- The CRITICAL impact is accepted only because citation parsing, hidden markup handling, and stream-event behavior must remain unchanged.

## Guardrails

- Preserve memory root path semantics, citation parsing, rollout/thread id extraction, hidden assistant markup handling through core stream events, and memory usage command classification/metric tags.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `memories/read` directory path.
- Verify with memories-read package tests, focused core memory/stream checks, fmt, Bazel lock checks, active-source stale-reference search, metadata count, diff check, and OntoIndex CLI fallback verification.
