# R5C Client Rename Closure

Date: 2026-06-11

## Scope

- Accepted identity-only rename: `codex-client` -> `ontocode-client`.
- Accepted Rust crate import rename: `codex_client` -> `ontocode_client`.
- Preserved `ontocode-rs/codex-client` directory path.
- Preserved HTTP transport, custom CA/TLS/root handling, proxy/cookie behavior, retry/backoff, SSE/streaming, telemetry semantics, auth/login/client behavior, env/config/wire names, persisted state, and test-only `custom_ca_probe` behavior.

## Manager Verification

- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-client --no-tests=pass` (27 tests).
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-api --no-tests=pass` (124 tests).
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Passed: stale search for `codex_client` under `ontocode-rs`.
- Passed: stale search for `codex-client` under `ontocode-rs`; only workspace member/path refs remain intentionally.
- Passed: `git diff --check`.
- OntoIndex MCP `gn_verify_diff` remains repo-miswired to `OntoIndex`; CLI fallback `detect-changes --repo codex` completed and reported only the known broad dirty-tree medium-risk context.

## Decision

- R5C is accepted as done.
- Next R5 slice requires a fresh one-slice risk review before dispatch.
