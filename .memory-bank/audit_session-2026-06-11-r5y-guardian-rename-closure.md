# R5Y Guardian Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-guardian` -> `ontocode-guardian`.
- Accepted `codex_guardian` -> `ontocode_guardian`.
- Scope was identity-only package/lib/Bazel/import rename.

## Preserved Surfaces

- Guardian review behavior.
- Guardian subagent spawn/fork context behavior.
- Thread lifecycle contribution behavior.
- App-server extension registration behavior.
- Extension API/protocol semantics.
- `codex_guardian_review` analytics event strings.
- Telemetry, env/config/wire/generated names, persisted state, and folder path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-guardian --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_guardian|codex-guardian`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5Y.
- Active old refs are clean except the intentional `codex_guardian_review` analytics event strings.
- Cargo metadata reports 48 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5Y-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
