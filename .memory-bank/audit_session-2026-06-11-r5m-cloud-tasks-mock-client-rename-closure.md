# R5M Cloud Tasks Mock Client Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-cloud-tasks-mock-client` -> `ontocode-cloud-tasks-mock-client`.
- Accepted `codex_cloud_tasks_mock_client` -> `ontocode_cloud_tasks_mock_client`.
- Preserved `ontocode-rs/cloud-tasks-mock-client` directory path.
- Preserved `codex-cloud-tasks-client` dependency identity.

## Manager Recovery

- Worker Fermat `019eb770-99fb-7820-bf47-209066781c66` was stale-closed after writing the scoped patch.
- Manager completed verification and closure locally.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks-mock-client --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-cloud-tasks --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_cloud_tasks_mock_client` and `codex-cloud-tasks-mock-client`.
- `git diff --check`
- OntoIndex CLI fallback: `detect-changes --repo codex`.

## Decision

R5M is accepted. Active old cloud-tasks mock-client package refs are clean. Cargo metadata reports 60 remaining `codex-*` packages, so residual package stages continue and R6 cleanup remains blocked.
