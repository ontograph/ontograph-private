# R5U Skills Rename Closure

Date: 2026-06-11

## Scope

- Accepted the identity-only rename `codex-skills` -> `ontocode-skills`.
- Accepted the crate/import rename `codex_skills` -> `ontocode_skills`.
- Preserved embedded system skill assets, `CODEX_HOME/skills/.system` path semantics, marker filename/salt/fingerprint behavior, install/remove/cache-root behavior, core-skills loader/manager behavior, include_dir compile data, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `skills` directory path.

## Risk

- OntoIndex exact impact for `install_system_skills`: LOW, 4 impacted nodes, 1 direct, no affected processes.
- OntoIndex exact impact for `system_cache_root_dir`: CRITICAL, 10 impacted nodes, 3 direct, 6 affected modules, no affected processes.
- The CRITICAL risk was accepted only because the implemented change was confined to package/lib/Bazel/import identity and did not change runtime logic or cache-root semantics.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-skills --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-skills --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_skills|codex-skills`: clean.
- `git diff --check`: clean.
- OntoIndex CLI fallback `detect-changes --repo codex`: reported the known broad dirty-tree high-risk context, not a scoped skills blocker.

## Result

- Cargo metadata reports 52 remaining `codex-*` workspace packages.
- Worker ran on `gpt-5.4-mini` with high reasoning after Spark usage-limit fallback.
