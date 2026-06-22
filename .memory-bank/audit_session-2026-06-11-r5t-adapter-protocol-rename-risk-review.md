# R5T Adapter Protocol Rename Risk Review

Date: 2026-06-11

## Decision

- Dispatch `codex-adapter-protocol` -> `ontocode-adapter-protocol`.
- Dispatch `codex_adapter_protocol` -> `ontocode_adapter_protocol` if the crate library identity appears.
- Limit the slice to package/lib/import identity only.

## Inventory

- Cargo metadata before R5T reports 54 remaining `codex-*` workspace packages.
- Direct reverse dependencies: 0.
- Direct active refs: 3.
- Ref scope: root workspace metadata and `adapter-protocol/Cargo.toml`.

## OntoIndex

- `AdapterMessage`: LOW impact, 0 impacted nodes, no affected processes.
- `ProtocolParser`: ambiguous between same-crate struct and impl; direct inventory is used as the controlling scope.
- Repo path reported by the CLI fallback is `/opt/demodb/_workfolder/ontocode`.

## Guardrails

- Preserve protocol version strings.
- Preserve serde tags, renames, enum variants, and field shapes.
- Preserve NDJSON framing and parser limit behavior.
- Preserve conformance runner behavior and fixtures.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `adapter-protocol` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-adapter-protocol --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_adapter_protocol|codex-adapter-protocol`.
- `git diff --check`.
- OntoIndex CLI fallback `detect-changes --repo codex`.
