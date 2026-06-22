# R5C Client Rename Worker Verification

Date: 2026-06-11

## Scope

- Implemented identity-only rename `codex-client` -> `ontocode-client`.
- Implemented crate import rename `codex_client` -> `ontocode_client`.
- Kept directory path `ontocode-rs/codex-client`.
- Did not change HTTP transport behavior, custom CA/TLS/root handling, proxy/cookie behavior, retry/backoff, SSE/streaming, telemetry semantics, auth/login/client behavior, env/config/wire names, persisted state, or `custom_ca_probe` behavior.

## OntoIndex

- MCP impact calls were miswired to repo `OntoIndex`; CLI status confirmed `/opt/demodb/_workfolder/ontocode` is indexed and up to date.
- CLI impact confirmed `build_reqwest_client_with_custom_ca` is CRITICAL impact: 35 impacted nodes, 10 direct, 13 modules.
- CLI context confirmed direct callers in login auth, exec-server HTTP client construction, `codex-api` file upload, cloud-tasks environment detection, and backend-client construction.
- CLI impact confirmed `HttpTransport` is MEDIUM impact per the approved risk review.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-client --no-tests=pass`: passed, 27 tests.
- `CARGO_BUILD_JOBS=8 just test -p codex-api --no-tests=pass`: passed, 124 tests.
- `CARGO_BUILD_JOBS=8 just test -p codex-backend-client --no-tests=pass`: passed, 12 tests.
- `CARGO_BUILD_JOBS=8 just test -p codex-cloud-tasks --no-tests=pass`: passed, 13 tests, 1 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server --no-tests=pass`: passed, 196 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`: passed, 50 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client --no-tests=pass`: passed, 64 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login --no-tests=pass`: passed, 118 tests.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed with existing rules_rs annotation warnings.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Stale search under `ontocode-rs`: no `codex_client` refs; remaining `codex-client` refs are the workspace member/path strings preserving `ontocode-rs/codex-client`.
- `git diff --check`: passed.
- OntoIndex CLI `detect-changes --repo codex`: completed as MCP fallback; broad dirty-tree caveat applies because unrelated edits pre-existed in this worktree.

## Notes

- The Rust test runs emitted pre-existing warnings for duplicate Windows sandbox binary target paths and a `codex-core` dead-code warning in `context_manager/history.rs`.
- No manager action is required beyond normal R5C review/acceptance.
