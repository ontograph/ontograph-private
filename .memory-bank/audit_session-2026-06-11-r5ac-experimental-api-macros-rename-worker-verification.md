# R5AC Experimental API Macros Rename Worker Verification

Date: 2026-06-11

## Scope

- `codex-experimental-api-macros` -> `ontocode-experimental-api-macros`
- `codex_experimental_api_macros` -> `ontocode_experimental_api_macros`

## Result

- Identity-only proc-macro package/lib/Bazel/import rename completed.
- Macro expansion, experimental attribute parsing, nested experimental propagation, inventory registration, serialized field-name conversion, and app-server v2 experimental gating/schema behavior were preserved.
- Active-source stale refs are clean for `codex_experimental_api_macros`; the only remaining `codex-experimental-api-macros` content hit is the intentional workspace path string.
- `cargo metadata --format-version 1 --no-deps` reports 44 remaining `codex-*` workspace packages.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-experimental-api-macros --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- active-source stale-reference search for `codex_experimental_api_macros|codex-experimental-api-macros`
- `cargo metadata --format-version 1 --no-deps`
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

## Notes

- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
- OntoIndex impact remained HIGH for exact helper `experimental_reason`; no macro semantics changed.
