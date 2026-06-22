# R5J Goal Extension Rename Worker Verification

Date: 2026-06-11

## Scope

- Renamed Cargo package `codex-goal-extension` to `ontocode-goal-extension`.
- Renamed Rust lib crate `codex_goal_extension` to `ontocode_goal_extension`.
- Updated root workspace metadata, `ext/goal` Bazel crate identity, goal extension tests/imports, and lock metadata.
- Preserved the existing `ontocode-rs/ext/goal` directory path.

## Preserved Surfaces

- Goal service/runtime/accounting/tool behavior.
- State/protocol semantics and templates.
- `codex-extension-api`, `codex-protocol`, `codex-state`, `codex-tools`, and `codex-otel` dependency identities.
- Env/config/wire/generated names, telemetry/product strings, and persisted state.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt` passed from `ontocode-rs`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-goal-extension --no-tests=pass` passed from `ontocode-rs`: 21 tests passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed from `ontocode-rs`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed from `ontocode-rs`.
- Active `ontocode-rs` stale-reference scan for `codex_goal_extension|codex-goal-extension` returned no matches.
- Cargo metadata reports 63 remaining `codex-*` packages.
- `git diff --check` passed via lean-ctx wrapper.
- OntoIndex CLI fallback `detect-changes --repo codex` completed and reported the known broad dirty-tree high-risk context rather than a scoped R5J-only verdict.

## Notes

- `ctx_shell` was allowlist-blocked for `just`, so required `just` checks were run through the terminal runner.
- The package test emitted pre-existing warnings for Windows sandbox dual binary targets and an `ontocode-core` dead-code warning.
