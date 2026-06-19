# R5AO Cloud Tasks Client Rename Worker Verification

Date: 2026-06-12
Status: worker verification complete; manager review pending
Model fallback: `gpt-5.4-mini` because `gpt-5.3-codex-spark` was unavailable or usage-limited.

## Scope

- Rename Cargo package `codex-cloud-tasks-client` to `ontocode-cloud-tasks-client`.
- Rename Rust crate import `codex_cloud_tasks_client` to `ontocode_cloud_tasks_client`.
- Update workspace metadata, cloud-tasks-client manifest/Bazel identity, and direct dependent imports/dependencies in cloud-tasks and cloud-tasks-mock-client.
- Preserve the existing `cloud-tasks-client` folder path.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks-client --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks-mock-client --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n 'codex_cloud_tasks_client|codex-cloud-tasks-client' ontocode-rs --glob '!target' || true`
- `cargo metadata --format-version 1 --no-deps` residual count: 32 `codex-*` packages
- `git diff --check`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

## Result

- Active old refs are clean in the targeted cloud-tasks slice.
- Cargo metadata reflects the new `ontocode-cloud-tasks-client` package identity.
- OntoIndex detect-changes reported high risk from pre-existing unrelated dirty-tree churn, not from this identity-only slice.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
