# R5AO Cloud Tasks Client Rename Risk Review

Date: 2026-06-12
Status: approved for identity-only dispatch with HIGH cloud-task guardrails
Model fallback: `gpt-5.4-mini` because the required Spark model is unavailable or usage-limited.

## Scope

- Rename Cargo package `codex-cloud-tasks-client` to `ontocode-cloud-tasks-client`.
- Rename Rust crate import `codex_cloud_tasks_client` to `ontocode_cloud_tasks_client`.
- Update workspace metadata, cloud-tasks-client manifest/Bazel identity, and direct dependent imports/dependencies.
- Preserve the existing `cloud-tasks-client` folder path.

## Direct Inventory

- Direct reverse dependencies: `ontocode-cloud-tasks`, `ontocode-cloud-tasks-mock-client`.
- Active refs are in workspace metadata, the cloud-tasks-client manifest/Bazel identity, cloud-tasks CLI/TUI imports/usages/tests, and cloud-tasks-mock-client imports/usages.

## OntoIndex Impact

- `CloudBackend`: LOW, 2 impacted symbols, no affected processes.
- `TaskSummary`: HIGH, 7 impacted symbols, 6 direct, 4 modules, no affected processes.
- `ApplyOutcome`: LOW, 3 impacted symbols, 1 module, no affected processes.
- `Struct:ontocode-rs/cloud-tasks-client/src/http.rs:HttpClient`: LOW, 0 impacted symbols, no affected processes.

## Guardrails

- Identity-only rename: do not change Cloud task API models, serde shapes, status mapping, HTTP request paths, backend-client calls, git apply behavior, user-agent behavior, or CLI/TUI behavior.
- Preserve task list/status/diff/apply/create flows and cloud task telemetry/user-agent compatibility strings.
- Preserve public command/config/wire/generated names, persisted state, and folder path.
- Do not rename `codex-backend-client`, `codex-git-utils`, or other residual packages in this slice.
- Run package/dependent cloud-task checks, fmt, Bazel lock update/check, stale-reference classification, `git diff --check`, and OntoIndex diff detection before closure.
