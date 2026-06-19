# R5I Skills Extension Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-skills-extension` -> `ontocode-skills-extension`.
- Accepted `codex_skills_extension` -> `ontocode_skills_extension`.
- Preserved `ontocode-rs/ext/skills` directory path.
- Preserved `codex-core-skills` dependency identity and extension API/protocol compatibility surfaces.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-skills-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_skills_extension` and `codex-skills-extension`.
- `git diff --check`
- OntoIndex CLI fallback: `detect-changes --repo codex`.

## Decision

R5I is accepted. Active old skills extension package refs are clean. Cargo metadata reports 64 remaining `codex-*` packages, so residual package stages continue and R6 cleanup remains blocked.
