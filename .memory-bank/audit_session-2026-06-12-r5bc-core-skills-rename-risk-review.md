# R5BC Core Skills Rename Risk Review

Date: 2026-06-12

## Decision

- Approve exactly one residual slice: `codex-core-skills` -> `ontocode-core-skills`.
- Approve crate import rename: `codex_core_skills` -> `ontocode_core_skills`.
- Scope is identity-only: package metadata, library crate name, Bazel crate name, Cargo lock, and dependent imports.

## OntoIndex Impact

- Exact `load_skills_from_roots`: CRITICAL.
- Impacted nodes: 59.
- Direct impacted nodes: 13.
- Affected modules: 16.
- Affected processes: 20.

## Direct Active References

- Root workspace dependency metadata.
- `core-skills` manifest and Bazel identity.
- `core` skill imports/re-exports and available-skills context wiring.
- `core-plugins` loader and manager imports.
- `skills-extension` catalog/render/selection imports.
- `tui` skill mention, composer, chatwidget, and helper imports.
- Intentional test tempdir prefix `codex-core-skills-isolated-`.

## Guardrails

- Preserve skill discovery roots and `SKILL.md` parsing.
- Preserve system, user, repo, and admin scope semantics.
- Preserve symlink, max-depth, hidden-file, and plugin namespacing behavior.
- Preserve skill config rules and disabled-skill resolution.
- Preserve implicit and explicit skill invocation.
- Preserve available-skills prompt rendering.
- Preserve app-server skills catalog behavior.
- Preserve core plugin loading behavior.
- Preserve TUI skill mention and skill metadata behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `core-skills` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-core-skills --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p codex-core-plugins --tests` while that package remains unrenamed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-skills-extension --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`.
- Focused app-server skills catalog check if available.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Scoped stale-reference search for `codex_core_skills|codex-core-skills`.
- Metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
