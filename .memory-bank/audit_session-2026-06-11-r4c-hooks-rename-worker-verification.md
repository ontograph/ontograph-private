# R4C Hooks Rename Worker Verification

Date: 2026-06-11

## Scope

- Renamed only `codex-hooks` -> `ontocode-hooks`.
- Renamed only active Rust crate/import/Bazel identity `codex_hooks` -> `ontocode_hooks`.
- Updated active workspace/dependent manifests, Cargo lockfile entries, Bazel crate identity, and active Rust imports/selectors.

## Preserved

- Hook JSON/config schema semantics.
- Persisted hook-state key strings produced by `hook_key`.
- Deprecated `[features].codex_hooks` compatibility behavior and diagnostics.
- Hook matcher, trust/hash, command execution, stop-loop, plugin hook declaration, and app-server hook catalog behavior.
- Telemetry/product strings, env/config semantics, wire/generated names, and persisted state.

## Verification

- PASS: `CARGO_BUILD_JOBS=8 just fmt`.
- PASS: `CARGO_BUILD_JOBS=8 just test -p ontocode-hooks --no-tests=pass`.
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-core hooks --no-tests=pass`.
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-core hook --no-tests=pass`.
- PASS: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server hooks --no-tests=pass`.
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-core-plugins --no-tests=pass`.
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-external-agent-migration --no-tests=pass`.
- PASS: `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- PASS: `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- PASS: active stale-reference searches found no `codex-hooks` refs in `ontocode-rs`; remaining `codex_hooks` refs are intentional deprecated feature-key/schema/diagnostic compatibility refs.
- PASS: `git diff --check`.
- PASS with noisy unrelated worktree context: `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex` ran against the indexed `codex` repo and reported medium risk across 200 dirty files.

## Notes

- Pre-edit OntoIndex CLI status confirmed `/opt/demodb/_workfolder/ontocode` is indexed and up to date.
- Pre-edit impact remained HIGH for `hook_key`, MEDIUM for `execute_handlers`, and LOW for `Hooks::dispatch`; edits stayed metadata/import-only.
- OntoIndex MCP `gn_verify_diff` was not used because the MCP facade exposed only the default `OntoIndex` repository id instead of `/opt/demodb/_workfolder/ontocode`; CLI verification used `--repo codex`.
