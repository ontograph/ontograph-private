# R5J Goal Extension Rename Risk Review

Date: 2026-06-11

## Candidate

- `codex-goal-extension` -> `ontocode-goal-extension`.
- `codex_goal_extension` -> `ontocode_goal_extension`.
- Identity-only package/lib/Bazel/test rename.

## Evidence

- Cargo metadata reports zero direct reverse dependencies.
- Direct text inventory has 11 active refs confined to root workspace metadata, `ext/goal/Cargo.toml`, `ext/goal/BUILD.bazel`, and `ext/goal/tests/goal_extension_backend.rs`.
- OntoIndex CLI `impact --repo codex install_with_backend` reports LOW risk with zero direct impacted nodes.
- OntoIndex CLI `impact --repo codex 'Struct:ontocode-rs/ext/goal/src/api.rs:GoalService'` reports LOW risk with zero direct impacted nodes.

## Guardrails

- Preserve goal service/runtime/accounting/tool behavior.
- Preserve state/protocol semantics and templates.
- Preserve `codex-extension-api`, `codex-protocol`, `codex-state`, `codex-tools`, and `codex-otel` dependency identities.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `ext/goal` directory path.

## Decision

R5J is approved for dispatch as a bounded residual package identity slice. No behavior edits or compatibility-surface renames are approved.
