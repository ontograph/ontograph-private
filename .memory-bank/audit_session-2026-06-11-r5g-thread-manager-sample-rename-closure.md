# R5G Thread Manager Sample Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-thread-manager-sample` -> `ontocode-thread-manager-sample`.
- Accepted `codex_thread_manager_sample` -> `ontocode_thread_manager_sample` for the sample Bazel/originator identity.
- Preserved `ontocode-rs/thread-manager-sample` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-thread-manager-sample --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_thread_manager_sample` and `codex-thread-manager-sample`.
- `git diff --check`
- OntoIndex CLI fallback: `detect-changes --repo codex`.

## Decision

R5G is accepted. Active old sample package refs are clean. Cargo metadata reports 66 remaining `codex-*` packages, so residual package stages continue and R6 cleanup remains blocked.
