# R5M Cloud Tasks Mock Client Rename Risk Review

Date: 2026-06-11

## Candidate

- `codex-cloud-tasks-mock-client` -> `ontocode-cloud-tasks-mock-client`.
- `codex_cloud_tasks_mock_client` -> `ontocode_cloud_tasks_mock_client`.
- Identity-only package/lib/Bazel/import rename.

## Evidence

- Cargo metadata reports one direct reverse dependency: `codex-cloud-tasks`.
- Direct text inventory has nine active refs confined to root workspace metadata, `cloud-tasks-mock-client/Cargo.toml`, `cloud-tasks-mock-client/BUILD.bazel`, `cloud-tasks/Cargo.toml`, `cloud-tasks/src/lib.rs`, and `cloud-tasks/tests/env_filter.rs`.
- OntoIndex CLI impact for exact `Struct:ontocode-rs/cloud-tasks-mock-client/src/mock.rs:MockClient` reports LOW risk with zero direct impacted nodes.

## Guardrails

- Preserve mock `CloudBackend` behavior.
- Preserve cloud-tasks mock wiring and env-filter behavior.
- Preserve `codex-cloud-tasks-client` dependency identity.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `cloud-tasks-mock-client` directory path.

## Decision

R5M is approved for dispatch as a bounded residual package identity slice. No behavior edits or public surface rewrites are approved.
