# R5D API Rename Closure

Date: 2026-06-11

## Scope

- Accepted identity-only rename: `codex-api` -> `ontocode-api`.
- Accepted Rust crate import rename: `codex_api` -> `ontocode_api`.
- Preserved `ontocode-rs/codex-api` directory path.
- Preserved request/response schema semantics, auth header behavior, Responses/Realtime/WebSocket/SSE behavior, model/list/search/image/file upload behavior, telemetry, retry/error mapping, protocol/wire/generated names, env/config names, and persisted state.

## Manager Verification

- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`.
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-api --no-tests=pass` (124 tests).
- Passed after manager test invariant fix: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-core skills_use_aliases_in_developer_message_under_budget_pressure --no-tests=pass`.
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-core client --no-tests=pass` (106 tests).
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-core responses --no-tests=pass` (59 tests).
- Passed on exact rerun: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport remote_control_waits_for_account_id_before_enrolling --no-tests=pass`.
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Passed: stale search for `codex_api` under `ontocode-rs`; remaining refs are intentional realtime wire log targets and shell-command parser fixtures.
- Passed: stale search for `codex-api` under `ontocode-rs`; remaining refs are intentional workspace member/path refs and a cloud-tasks runtime label.
- Passed: `git diff --check`.
- OntoIndex MCP `gn_verify_diff` remains repo-miswired to `OntoIndex`; CLI fallback `detect-changes --repo codex` completed and reported only the known broad dirty-tree medium-risk context.

## Decision

- R5D is accepted as done.
- The final R5 implementation slice is `codex-core` -> `ontocode-core`; it requires fresh senior risk review before dispatch.
