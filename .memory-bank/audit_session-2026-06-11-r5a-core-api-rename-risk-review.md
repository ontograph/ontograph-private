# R5A Core API Rename Risk Review

Date: 2026-06-11

Candidate:
- `codex-core-api` -> `ontocode-core-api`
- `codex_core_api` -> `ontocode_core_api`

Direct inventory:
- `codex-core-api` package refs under `ontocode-rs`: 7.
- `codex_core_api` crate refs under `ontocode-rs`: 55.
- Active dependent is `thread-manager-sample`; `core-api` itself is a facade re-export crate.

Approved scope:
- Identity-only Cargo package/lib/Bazel/import rename.
- Preserve existing `core-api` folder path.
- Preserve all facade exports and re-export semantics.
- Preserve `thread-manager-sample` behavior.
- Preserve core/config/protocol type semantics, public command/config/schema/wire names, telemetry/product strings, and persisted state.

Rejected scope:
- No core runtime behavior changes.
- No facade export additions/removals.
- No protocol/config/schema/generated-wire rename.
- No public command or persisted-state rename.
- No broad find-and-replace outside active package/lib/Bazel/import references.

Required verification:
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core-api --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-thread-manager-sample --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for active `codex_core_api` / `codex-core-api` refs under `ontocode-rs`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff` or CLI `detect-changes`.
