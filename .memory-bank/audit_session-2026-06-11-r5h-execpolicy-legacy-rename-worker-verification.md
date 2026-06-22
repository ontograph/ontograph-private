# R5H Execpolicy Legacy Rename Worker Verification

Date: 2026-06-11

## Scope

- Renamed Cargo package and legacy binary target `ontocode-execpolicy-legacy` to `ontocode-execpolicy-legacy`.
- Renamed Rust lib crate `codex_execpolicy_legacy` to `ontocode_execpolicy_legacy`.
- Updated legacy execpolicy Bazel crate identity, README `cargo run -p` commands, active non-legacy README cross-reference, and package-local imports/tests.
- Preserved the existing directory path `ontocode-rs/execpolicy-legacy`.

## Preserved Surfaces

- Legacy policy parser/checker behavior.
- Default policy semantics.
- CLI argument behavior except binary/package identity.
- JSON output shape.
- Env/config/wire/generated names.
- Telemetry/product strings.
- Persisted state.

## Verification

- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-execpolicy-legacy --no-tests=pass`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_execpolicy_legacy` and `ontocode-execpolicy-legacy`
- `git diff --check`
- `cd /opt/demodb/_workfolder/ontocode && /usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

## Notes

- OntoIndex MCP impact was repo-miswired to `OntoIndex`; CLI impact for `ExecCall` and `PolicyParser` reported the known ambiguous candidates.
- Remaining old-name references are historical memory-bank inventory/tracking artifacts, not active source references.
- Cargo metadata reports 65 remaining `codex-*` packages after R5H.
