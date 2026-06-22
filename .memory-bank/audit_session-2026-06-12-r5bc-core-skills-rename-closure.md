# R5BC Core Skills Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-core-skills` -> `ontocode-core-skills`.
- Accepted `codex_core_skills` -> `ontocode_core_skills`.
- Scope remained identity-only: package, library, Bazel target, Cargo lock, and dependent imports.

## Guardrails Preserved

- Skill discovery roots and `SKILL.md` parsing.
- System, user, repo, and admin scope semantics.
- Symlink, max-depth, hidden-file, and plugin namespacing behavior.
- Skill config rules and disabled-skill resolution.
- Implicit and explicit skill invocation.
- Available-skills prompt rendering.
- App-server skills catalog behavior.
- Core plugin loading behavior.
- TUI skill mention and skill metadata behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `core-skills` directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-core-skills --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p codex-core-plugins --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-skills-extension --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server skills_list`.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Scoped stale-reference search for `codex_core_skills|codex-core-skills`: only intentional `codex-core-skills-isolated-` test tempdir prefix remains.
- Cargo metadata residual count: 18 `codex-*` packages.
- `git diff --check`: clean.
- OntoIndex `detect-changes --repo codex`: completed with the known broad high-risk dirty-tree envelope.

## Result

- R5BC accepted.
- Work completed on `gpt-5.4-mini` after Spark usage-limit fallback.
