# R5AC Experimental API Macros Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-experimental-api-macros` -> `ontocode-experimental-api-macros`.
- Accepted `codex_experimental_api_macros` -> `ontocode_experimental_api_macros`.
- Scope was identity-only package/proc-macro lib/Bazel/import rename.

## Preserved Surfaces

- `#[derive(ExperimentalApi)]` expansion behavior.
- `#[experimental(...)]` parsing and nested experimental propagation.
- Inventory registration and serialized field-name conversion.
- App-server v2 experimental gating and schema behavior.
- Telemetry, env/config/wire/generated names, persisted state, and folder path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-experimental-api-macros --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_experimental_api_macros|codex-experimental-api-macros`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5AC.
- Active old crate refs are clean except intentional workspace path strings in `ontocode-rs/Cargo.toml`.
- Cargo metadata reports 44 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5AC-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
