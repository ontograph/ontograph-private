# R5K V8 POC Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-v8-poc` -> `ontocode-v8-poc`.
- Accepted `codex_v8_poc` -> `ontocode_v8_poc`.
- Preserved `ontocode-rs/v8-poc` directory path.
- Preserved existing Bazel target label `//ontocode-rs/v8-poc:v8-poc`.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-v8-poc --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_v8_poc` and `codex-v8-poc`.
- `git diff --check`
- OntoIndex CLI fallback: `detect-changes --repo codex`.

## Decision

R5K is accepted. Active old V8 POC package refs are clean. Cargo metadata reports 62 remaining `codex-*` packages, so residual package stages continue and R6 cleanup remains blocked.
