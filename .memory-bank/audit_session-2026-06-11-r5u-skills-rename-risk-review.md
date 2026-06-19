# R5U Skills Rename Risk Review

Date: 2026-06-11

## Decision

- Dispatch `codex-skills` -> `ontocode-skills`.
- Dispatch `codex_skills` -> `ontocode_skills`.
- Limit the slice to package/lib/Bazel/import identity only.

## Inventory

- Cargo metadata before R5U reports 53 remaining `codex-*` workspace packages.
- Direct reverse dependencies: 1, `codex-core-skills`.
- Direct active refs: 7.
- Ref scope: root workspace metadata, skills manifest/Bazel identity, and core-skills dependency/import re-exports.

## OntoIndex

- `install_system_skills`: LOW impact, 4 impacted nodes, 1 direct, 0 affected processes.
- `system_cache_root_dir`: CRITICAL impact, 10 impacted nodes, 3 direct, 6 affected modules, 0 affected processes.
- Repo path reported by the CLI fallback is `/opt/demodb/_workfolder/ontocode`.

## Guardrails

- Preserve embedded system skill assets.
- Preserve `CODEX_HOME/skills/.system` path semantics.
- Preserve marker filename, marker salt, and fingerprint behavior.
- Preserve install/remove/cache-root behavior.
- Preserve core-skills loader and manager behavior.
- Preserve include_dir compile data and the existing `skills` directory path.
- Preserve env/config/wire/generated names, telemetry/product strings, and persisted state.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-skills --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-skills --no-tests=pass`
- Focused core-skills system/loader checks if available.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core --no-tests=pass` because OntoIndex links cache-root through core thread-manager construction.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_skills|codex-skills`.
- `git diff --check`.
- OntoIndex CLI fallback `detect-changes --repo codex`.
