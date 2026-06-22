# R5AD Cloud Tasks Rename Worker Verification

Date: 2026-06-11

## Outcome

- `codex-cloud-tasks` -> `ontocode-cloud-tasks` and `codex_cloud_tasks` -> `ontocode_cloud_tasks` identity-only package/lib/Bazel/import rename is complete.
- Preserved cloud task CLI/TUI behavior, backend initialization, login/client behavior, task list/status/diff/apply/create flows, user-agent suffix telemetry strings, public command behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and folder path.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli cloud`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference scan for `codex_cloud_tasks|codex-cloud-tasks`
- `cargo metadata --format-version 1 --no-deps` residual count: 43 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Residual References

- `codex-cloud-tasks-client` dependency-path strings remain intentionally unchanged.
- `codex_cloud_tasks_*` telemetry/user-agent suffix strings remain intentionally unchanged.
- One historical TUI comment referencing `codex-cloud-tasks` remains unchanged.

## Notes

- `OntoIndex detect-changes` reported broad high risk because the worktree already contains unrelated dirty edits outside this slice.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
