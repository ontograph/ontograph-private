# R5BR State Rename Risk Review

Date: 2026-06-12

## Scope

- Rename `codex-state` to `ontocode-state`.
- Rename `codex_state` to `ontocode_state`.
- Keep the existing `state` directory path.

## OntoIndex

- `install_process_db_telemetry`: CRITICAL; reaches core OTEL init and CLI/TUI/app-server/exec/mcp-server entrypoints.
- `AgentJob`: LOW, no upstream nodes reported.
- `StateRuntime`, `ThreadMetadata`, and generic `state`: UNKNOWN/ambiguous.
- `StateService`, `StateStore`, and `LatestSession`: UNKNOWN/not found.

## Guardrails

- Do not change SQLite schema migrations.
- Do not change DB filenames or `CODEX_SQLITE_HOME`.
- Do not change metric names or telemetry behavior.
- Do not change rollout metadata extraction, backfill/runtime behavior, thread/goal/memory/log/agent-job models, remote-control enrollment state, public config/wire/generated/schema names, persisted state, telemetry/product strings, or directory paths.

## Verification Required

- State package compile/tests.
- Focused migration/runtime/model/telemetry checks where available.
- Core/app-server/CLI/TUI/exec/mcp-server compile checks if imports changed.
- `just fmt`.
- `just bazel-lock-update` and `just bazel-lock-check`.
- Stale-reference search.
- Cargo metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
