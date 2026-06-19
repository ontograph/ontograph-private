# R5AD Cloud Tasks Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-cloud-tasks` -> `ontocode-cloud-tasks`.
- Accepted `codex_cloud_tasks` -> `ontocode_cloud_tasks`.
- Scope was identity-only package/lib/Bazel/import rename.

## Preserved Surfaces

- Cloud task CLI/TUI behavior.
- Backend initialization, login/client behavior, and cloud backend calls.
- Task list/status/diff/apply/create flows.
- User-agent suffix telemetry strings.
- Public command behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and folder path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-cloud-tasks --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli cloud`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_cloud_tasks|codex-cloud-tasks`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5AD.
- Active old crate refs are clean except intentional telemetry/user-agent suffix strings and one historical TUI comment.
- Cargo metadata reports 43 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5AD-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
