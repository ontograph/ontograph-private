# R5G Thread Manager Sample Rename Worker Verification

Date: 2026-06-11

## Scope

- Renamed only `codex-thread-manager-sample` to `ontocode-thread-manager-sample`.
- Renamed only the sample Bazel crate identity and originator string from `codex_thread_manager_sample` to `ontocode_thread_manager_sample`.
- Preserved the existing `ontocode-rs/thread-manager-sample` directory path, `ontocode-core-api` dependency boundary, sample behavior, thread-manager API semantics, env/config/wire/generated names, telemetry schemas outside the sample originator, persisted state, and protocol/generated names.

## Verification

- PASS: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`.
- PASS: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-thread-manager-sample --no-tests=pass`.
- PASS: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- PASS: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- PASS: active-source stale-reference search found no `codex-thread-manager-sample` or `codex_thread_manager_sample` refs under `ontocode-rs`.
- PASS: `git diff --check`.
- PASS with caveat: `ontoindex detect-changes --repo codex` ran via CLI fallback and reported the known broad dirty-tree high-risk context rather than a scoped R5G-only verdict.

## Notes

- `ontocode-thread-manager-sample` has zero tests; nextest ran with `--no-tests=pass` and reported `0 tests run`.
- Cargo metadata reports 66 remaining `codex-*` packages after R5G.
- Repository-wide old-name refs that remain are historical/planning memory-bank entries and generated inventory snapshots, not active source refs.
