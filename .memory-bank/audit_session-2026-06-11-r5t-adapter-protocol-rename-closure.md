# R5T Adapter Protocol Rename Closure

Date: 2026-06-11

## Scope

- Accepted the identity-only rename `codex-adapter-protocol` -> `ontocode-adapter-protocol`.
- Accepted the derived crate identity `codex_adapter_protocol` -> `ontocode_adapter_protocol`.
- Preserved protocol version strings, serde tags/renames, enum variants, field shapes, NDJSON framing, parser limits, conformance runner behavior, fixtures, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `adapter-protocol` directory path.

## Risk

- OntoIndex exact impact for `AdapterMessage`: LOW, 0 impacted nodes.
- `ProtocolParser` was ambiguous between same-crate struct and impl; direct inventory controlled the slice.
- No Rust protocol symbols were edited.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-adapter-protocol --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_adapter_protocol|codex-adapter-protocol`: clean.
- `git diff --check`: clean.
- OntoIndex CLI fallback `detect-changes --repo codex`: reported the known broad dirty-tree high-risk context, not a scoped adapter-protocol blocker.

## Result

- Cargo metadata reports 53 remaining `codex-*` workspace packages.
- Worker ran on `gpt-5.4-mini` with high reasoning after Spark usage-limit fallback.
