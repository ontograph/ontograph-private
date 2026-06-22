# R5J Goal Extension Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-goal-extension` -> `ontocode-goal-extension`.
- Accepted `codex_goal_extension` -> `ontocode_goal_extension`.
- Preserved `ontocode-rs/ext/goal` directory path.
- Preserved goal extension behavior and dependency compatibility surfaces.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-goal-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_goal_extension` and `codex-goal-extension`.
- `git diff --check`
- OntoIndex CLI fallback: `detect-changes --repo codex`.

## Decision

R5J is accepted. Active old goal extension package refs are clean. Cargo metadata reports 63 remaining `codex-*` packages, so residual package stages continue and R6 cleanup remains blocked.
