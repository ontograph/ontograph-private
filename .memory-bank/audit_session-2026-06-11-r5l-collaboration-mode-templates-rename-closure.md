# R5L Collaboration Mode Templates Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-collaboration-mode-templates` -> `ontocode-collaboration-mode-templates`.
- Accepted `codex_collaboration_mode_templates` -> `ontocode_collaboration_mode_templates`.
- Preserved template file contents and `ontocode-rs/collaboration-mode-templates` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-collaboration-mode-templates --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-models-manager --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_collaboration_mode_templates` and `codex-collaboration-mode-templates`.
- `git diff --check`
- OntoIndex CLI fallback: `detect-changes --repo codex`.

## Decision

R5L is accepted. Active old collaboration-mode template package refs are clean. Cargo metadata reports 61 remaining `codex-*` packages, so residual package stages continue and R6 cleanup remains blocked.
