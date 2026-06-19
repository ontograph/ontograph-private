# R5K V8 POC Rename Worker Verification

Date: 2026-06-11

## Scope

- Renamed Cargo package `codex-v8-poc` to `ontocode-v8-poc`.
- Renamed Rust lib crate and Bazel crate identity `codex_v8_poc` to `ontocode_v8_poc`.
- Preserved existing `ontocode-rs/v8-poc` directory path and Bazel target label `//ontocode-rs/v8-poc:v8-poc`.

## Verification

- OntoIndex CLI impact for `Function:ontocode-rs/v8-poc/src/lib.rs:bazel_target`: LOW risk, zero direct impacted nodes, zero affected processes/modules, repo path `/opt/demodb/_workfolder/ontocode`.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-v8-poc --no-tests=pass`: passed; 6 tests passed, covering Bazel target exposure, embedded V8 version, sandbox feature/link check, integer/string V8 evaluation, and CRDTP dispatchable parsing.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed; emitted existing rules_rs well-known crate annotation warnings.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active-source stale-reference search for `codex_v8_poc|codex-v8-poc` under `ontocode-rs`: clean.
- `git diff --check`: passed.
- OntoIndex CLI fallback `detect-changes --repo codex`: completed and reported the known broad dirty-tree high-risk context, 200 files, 326 symbols, and 8 affected execution flows rather than a scoped R5K-only verdict.
- Cargo metadata reports 62 remaining `codex-*` packages.

## Notes

- Preserved V8 proof-of-concept behavior, V8 sandbox feature semantics, embedded V8/version checks, CRDTP tests, env/config/wire/generated names, telemetry/product strings, and persisted state.
- Cargo test emitted pre-existing duplicate-target warnings from `windows-sandbox-rs`; no R5K files were involved.
- `Cargo.lock` was already part of broad dirty-tree rename churn; the R5K-relevant lock entry is `ontocode-v8-poc`.
