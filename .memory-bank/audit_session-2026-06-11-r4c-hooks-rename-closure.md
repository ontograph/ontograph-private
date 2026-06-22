# R4C Hooks Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-hooks` -> `ontocode-hooks`.
- Accepted `codex_hooks` -> `ontocode_hooks` only for active crate/import/Bazel identity.
- Preserved hook schema/config semantics, persisted hook-state keys, deprecated `[features].codex_hooks` compatibility, matcher/trust/command/stop-loop/plugin/app-server behavior, telemetry, env/config semantics, wire/generated names, and persisted state.

## Verification

- PASS: worker `CARGO_BUILD_JOBS=8 just fmt`.
- PASS: manager `CARGO_BUILD_JOBS=8 just test -p ontocode-hooks --no-tests=pass` (117 passed).
- PASS: manager `CARGO_BUILD_JOBS=8 just test -p codex-core hooks --no-tests=pass` (58 passed).
- PASS: worker `CARGO_BUILD_JOBS=8 just test -p codex-core hook --no-tests=pass`.
- PASS: manager `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server hooks --no-tests=pass` (13 passed).
- PASS: worker `CARGO_BUILD_JOBS=8 just test -p codex-core-plugins --no-tests=pass`.
- PASS: worker `CARGO_BUILD_JOBS=8 just test -p codex-external-agent-migration --no-tests=pass`.
- PASS: worker `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- PASS: manager `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- PASS: no `codex-hooks` refs remain under `ontocode-rs`.
- PASS: remaining `codex_hooks` refs are intentional deprecated feature/schema/test compatibility strings.
- PASS: manager `git diff --check`.
- PASS: scoped OntoIndex `gn_verify_diff` for the R4C file set.

## Notes

- R4C was HIGH risk only through hook-key and runtime dispatch blast radius; no function or behavior edits were made.
