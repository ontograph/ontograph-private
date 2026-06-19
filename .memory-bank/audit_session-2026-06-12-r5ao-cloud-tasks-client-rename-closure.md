# R5AO Cloud Tasks Client Rename Closure

Date: 2026-06-12
Status: accepted
Model fallback: `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.

## Outcome

- Accepted `codex-cloud-tasks-client` as `ontocode-cloud-tasks-client`.
- Accepted `codex_cloud_tasks_client` as `ontocode_cloud_tasks_client`.
- Preserved Cloud task API models, serde shapes, status mapping, HTTP request paths, backend-client calls, git apply behavior, user-agent behavior, task list/status/diff/apply/create flows, cloud task telemetry/user-agent compatibility strings, public command/config/wire/generated names, persisted state, and the existing `cloud-tasks-client` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks-client --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks-mock-client --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n 'codex_cloud_tasks_client|codex-cloud-tasks-client' ontocode-rs --glob '!target' || true`
- `cargo metadata --format-version 1 --no-deps`
- `git diff --check`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

## Notes

- Active old refs in `ontocode-rs` are clean.
- Cargo metadata reports 32 remaining `codex-*` packages.
- OntoIndex fallback still reports the known broad dirty-tree high-risk context: 200 files, 312 symbols, 8 affected processes.
