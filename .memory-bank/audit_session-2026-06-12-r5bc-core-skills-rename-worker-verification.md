# R5BC Core Skills Rename Worker Verification

Date: 2026-06-12

## Outcome

- `codex-core-skills` -> `ontocode-core-skills` and `codex_core_skills` -> `ontocode_core_skills` are verified as an identity-only rename slice.
- Skill discovery roots, `SKILL.md` parsing, scope semantics, config rules, invocation behavior, available-skills rendering, app-server skills catalog behavior, core plugin loading, TUI skill mentions, persisted state, and the existing `core-skills` directory path are preserved.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-core-skills --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p codex-core-plugins --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-skills-extension --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server skills_list`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Scoped stale-reference search for `codex_core_skills|codex-core-skills`
- `cargo metadata --format-version 1 --no-deps` residual count: 18 `codex-*` workspace packages
- `git diff --check`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

## Notes

- Only intentional stale ref left in `ontocode-rs` source is the `codex-core-skills-isolated-` tempdir prefix in `core-skills/src/loader_tests.rs`.
- `detect-changes --repo codex` reported the expected broad dirty-tree high-risk result.
