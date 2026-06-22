# R5BP OTEL Recovery Closure

Date: 2026-06-12

## Outcome

R5BP is accepted after R5BP-U1 recovery.

## Recovery Scope

- Restored accepted Ontocode package identities after stale baseline regression.
- Preserved public command names, config keys, wire/generated names, persisted state, telemetry strings, and directory paths.
- Updated nextest package selectors for renamed packages.
- Added the missing `exec-server/src/main.rs` wrapper required by existing compatibility bin aliases.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `git diff --check`
- `cargo metadata --no-deps --format-version 1`: exactly `ontocode-app-server-protocol`, `codex-extension-api`, `codex-protocol`, `codex-state`, and `codex-tools` remain.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-otel ... -E 'package(ontocode-otel)' --no-tests=pass`: 44 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-features ... -E 'package(ontocode-features)' --no-tests=pass`: 50 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rollout ... -E 'package(ontocode-rollout)' --no-tests=pass`: 69 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-network-proxy ... -E 'package(ontocode-network-proxy)' --no-tests=pass`: 165 passed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-exec-server --tests`
- Stale-reference search for OTEL/features/rollout/network-proxy old package/crate names found only intentional docs/compat/comment surfaces.
- OntoIndex `detect-changes --repo codex` reports high risk for the known broad recovery diff.

## Remaining

- `codex-extension-api`
- `codex-state`
- `codex-tools`
- `ontocode-app-server-protocol`
- `codex-protocol`
