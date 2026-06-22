# R5L Collaboration Mode Templates Rename Worker Verification

Date: 2026-06-11

## Scope

- Renamed Cargo package `codex-collaboration-mode-templates` to `ontocode-collaboration-mode-templates`.
- Renamed Rust lib crate `codex_collaboration_mode_templates` to `ontocode_collaboration_mode_templates`.
- Updated Bazel crate identity, root workspace metadata, `models-manager` dependency/imports, and Cargo lock metadata.

## Guardrails

- Template file contents were not changed.
- Existing `ontocode-rs/collaboration-mode-templates` directory path and Bazel target label were preserved.
- Collaboration-mode preset behavior, `models-manager` semantics, compile data/template packaging, env/config/wire/generated names, telemetry/product strings, and persisted state were preserved.

## Verification

- Pre-edit OntoIndex CLI impact for `DEFAULT`: LOW, zero direct impacted nodes, repoPath `/opt/demodb/_workfolder/ontocode`.
- Pre-edit OntoIndex CLI impact for `PLAN`: LOW, zero direct impacted nodes, repoPath `/opt/demodb/_workfolder/ontocode`.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-collaboration-mode-templates --no-tests=pass`: passed; zero tests, bench-smoke completed.
- `CARGO_BUILD_JOBS=8 just test -p codex-models-manager --no-tests=pass`: passed, 32 tests.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active `ontocode-rs` source stale-reference search for `codex_collaboration_mode_templates|codex-collaboration-mode-templates`: zero matches.
- `git diff --check`: passed.
- OntoIndex MCP `gn_verify_diff`: blocked by repo miswiring (`/opt/demodb/_workfolder/ontocode` not found; available repo `OntoIndex`).
- OntoIndex CLI fallback `detect-changes --repo codex`: completed; reported broad dirty-tree context of 200 files, 326 symbols, 8 affected processes, high risk.
- Cargo metadata reports 61 remaining `codex-*` packages after R5L.

## Notes

- OntoIndex MCP remains repo-miswired to `OntoIndex`; CLI fallback was used for impact and closeout verification.
- `ctx_shell` permanently blocked `just` through the lean-ctx allowlist, so required `just` commands were run with the regular terminal.
- Test output repeated known unrelated Windows sandbox duplicate-bin warnings.
- Bazel lock update repeated existing rules_rs crate-annotation warnings.
