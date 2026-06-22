# R5A Core API Rename Worker Verification

Date: 2026-06-11

Scope:
- Renamed Cargo package/lib/Bazel/import identity only:
  - `codex-core-api` -> `ontocode-core-api`
  - `codex_core_api` -> `ontocode_core_api`
- Kept directory path `ontocode-rs/core-api` unchanged.
- Updated root workspace dependency key, `core-api` manifest/lib/Bazel identity, `thread-manager-sample` dependency/import/README references, and Cargo lockfile entries.

Preserved:
- All `core-api/src/lib.rs` facade exports and re-export semantics.
- `thread-manager-sample` runtime behavior.
- Core/config/protocol type semantics.
- Public command/config/schema/wire names.
- Telemetry/product strings.
- Persisted state and compatibility names.

Verification:
- PASS: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- PASS: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-core-api --no-tests=pass`
- PASS: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-thread-manager-sample --no-tests=pass`
- PASS: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-update`
- PASS: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check`
- PASS: `rg -n "\bcodex_core_api\b|\bcodex-core-api\b" ontocode-rs --glob "!target" || true` returned no remaining refs.
- PASS: `git diff --check`
- PASS: OntoIndex CLI `detect-changes --repo codex --scope staged` on the seven-file R5A staged set reported 7 files, 1 symbol, 0 affected processes, LOW risk; files were unstaged afterward.

Notes:
- `MODULE.bazel.lock` did not change after `just bazel-lock-update`.
- `ontocode-core-api` check ran zero tests as expected for this facade crate.
- `codex-thread-manager-sample` check ran zero tests across one binary after compiling the sample.
- Cargo emitted unrelated pre-existing warnings for duplicate windows-sandbox binary target source files and a `codex-core` dead-code warning in `context_manager/history.rs`.
