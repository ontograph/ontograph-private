# R5AG External-Agent Sessions Rename Risk Review

Date: 2026-06-11

## Candidate

- `codex-external-agent-sessions` -> `ontocode-external-agent-sessions`
- `codex_external_agent_sessions` -> `ontocode_external_agent_sessions`

## Inventory

- Cargo metadata direct reverse dependencies: `ontocode-app-server`
- Active direct refs: 12
- Ref locations: root workspace metadata, app-server dependency/import usage, and external-agent-sessions manifest/Bazel identity.

## OntoIndex Impact

- `Function:ontocode-rs/external-agent-sessions/src/detect.rs:detect_recent_sessions`: MEDIUM, 7 impacted, 6 direct, 2 modules, 0 processes.
- `Function:ontocode-rs/external-agent-sessions/src/lib.rs:prepare_pending_session_imports`: LOW, 2 impacted, 2 direct, 1 module, 0 processes.
- `Function:ontocode-rs/external-agent-sessions/src/lib.rs:prepare_validated_session_imports`: LOW, 0 impacted, 0 processes.
- `Function:ontocode-rs/external-agent-sessions/src/ledger.rs:record_imported_session`: LOW, 3 impacted, 3 direct, 2 modules, 0 processes.
- `Struct:ontocode-rs/external-agent-sessions/src/lib.rs:ExternalAgentSessionMigration`: HIGH, 9 impacted, 2 direct, 3 modules, 0 processes.

## Decision

- Proceed as an identity-only package/lib/Bazel/import rename.
- The HIGH impact is accepted only because exported struct behavior, fields, serialization boundaries, and app-server session flow must remain unchanged.

## Guardrails

- Preserve external session scanning, recency limits, title selection, imported-history rollout construction, tool-call/tool-result bounded tags, token accounting, import ledger hashing/canonicalization, duplicate-import semantics, and app-server external-agent session APIs/processors.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `external-agent-sessions` directory path.
- Verify with the session package tests, app-server external-agent session/request-processor checks, fmt, Bazel lock checks, active-source stale-reference search, metadata count, diff check, and OntoIndex CLI fallback verification.
