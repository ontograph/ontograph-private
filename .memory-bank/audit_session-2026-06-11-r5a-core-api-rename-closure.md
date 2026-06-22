# R5A Core API Rename Closure

Date: 2026-06-11

## Scope

- Accepted identity-only rename: `codex-core-api` -> `ontocode-core-api`.
- Accepted Rust crate import rename: `codex_core_api` -> `ontocode_core_api`.
- Preserved `ontocode-rs/core-api` directory path and facade export semantics.
- Preserved `thread-manager-sample` behavior, core/config/protocol semantics, compatibility names, telemetry/product strings, and persisted state.

## Manager Verification

- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-core-api --no-tests=pass`.
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-thread-manager-sample --no-tests=pass`.
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Passed: stale search for `codex_core_api` and `codex-core-api` under `ontocode-rs`.
- Passed: `git diff --check`.
- OntoIndex MCP `gn_verify_diff` remains repo-miswired to `OntoIndex`; CLI fallback `detect-changes --repo codex` completed and reported only the known broad dirty-tree medium-risk context.

## Decision

- R5A is accepted as done.
- Next R5 slice requires a fresh one-slice risk review before dispatch.
