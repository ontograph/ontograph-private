# R5AD Cloud Tasks Rename Risk Review

Date: 2026-06-11

## Candidate

- `codex-cloud-tasks` -> `ontocode-cloud-tasks`
- `codex_cloud_tasks` -> `ontocode_cloud_tasks`

## Current Inventory

- Cargo metadata direct reverse dependency: `ontocode-cli`.
- Exact active refs: 16.
- Ref scope: root lock metadata, CLI dependency/import/call sites, cloud-tasks manifest/Bazel identity, telemetry/user-agent suffix compatibility strings, and one TUI doc comment.

## OntoIndex CLI Fallback Impact

- `Function:ontocode-rs/cloud-tasks/src/lib.rs:run_main`: LOW, 0 impacted nodes, 0 affected processes.
- `init_backend`: MEDIUM, 6 impacted nodes, 0 affected processes.
- `run_exec_command`: LOW, 1 impacted node, 0 affected processes.
- `load_tasks`: LOW, 1 impacted node, 0 affected processes.

## Guardrails

- Only package/lib/Bazel/import identity may change.
- Preserve cloud task CLI/TUI behavior.
- Preserve backend initialization, login/client behavior, and cloud backend calls.
- Preserve task list/status/diff/apply/create flows.
- Preserve user-agent suffix telemetry strings such as `codex_cloud_tasks_exec`, `codex_cloud_tasks_status`, `codex_cloud_tasks_list`, `codex_cloud_tasks_diff`, `codex_cloud_tasks_apply`, and `codex_cloud_tasks_tui`.
- Preserve public command behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and folder path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli cloud`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_cloud_tasks|codex-cloud-tasks`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Decision

- Approved as R5AD only because it is a bounded identity-only rename with one direct dependent.
- Work must run on fallback `gpt-5.4-mini` after Spark usage-limit fallback and record that fallback in output/tracking.
