# R5D API Rename Worker Verification

Date: 2026-06-11

## Scope

- Implemented identity-only rename `codex-api` -> `ontocode-api`.
- Implemented Rust crate import rename `codex_api` -> `ontocode_api`.
- Preserved directory path `ontocode-rs/codex-api`.

## Preserved Surfaces

- Request/response schema semantics.
- Auth header behavior.
- Responses, Realtime, WebSocket, and SSE streaming behavior.
- Model/list/search/image/file upload behavior.
- Telemetry strings, retry/error mapping, protocol/wire/generated names, env/config names, persisted state.

## Verification

- Passed: `CARGO_BUILD_JOBS=8 just fmt`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-api --no-tests=pass`.
- Failed with unrelated known fixture: `CARGO_BUILD_JOBS=8 just test -p codex-core client --no-tests=pass` (`skills_use_aliases_in_developer_message_under_budget_pressure`).
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-core responses --no-tests=pass`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-cli debug --no-tests=pass`.
- Failed full run with remote-control timing flake: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport --no-tests=pass`.
- Passed on exact rerun with nextest marking flaky: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport remote_control_waits_for_account_id_before_enrolling --no-tests=pass`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-backend-client --no-tests=pass`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-cloud-tasks --no-tests=pass`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`.
- Passed: `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- Passed: `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Passed: stale-reference classification for active `codex_api` and `codex-api` refs under `ontocode-rs`.
- Passed: `git diff --check`.
- OntoIndex MCP remained repo-miswired to `OntoIndex`; CLI fallback `detect-changes --repo codex` reported the known broad dirty-tree medium-risk context.

## Remaining Old-Name References

- `ontocode-rs/Cargo.toml` workspace member and dependency path strings preserve the existing `codex-api` directory path.
- Realtime wire log targets retain `codex_api::realtime_websocket::wire` to preserve telemetry/log target compatibility.
- `shell-command` parser fixture strings intentionally retain `codex_api` as sample command text.
- `cloud-tasks` path-style label intentionally retains `codex-api` because it is runtime classification text, not crate identity.

## Manager Action Needed

- Review and accept the R5D identity-only diff.
- Decide whether the unrelated `codex-core` skill-alias fixture failure and app-server transport timing flake should block acceptance or be tracked separately.
