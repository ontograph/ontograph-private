# R5K V8 POC Rename Risk Review

Date: 2026-06-11

## Candidate

- `codex-v8-poc` -> `ontocode-v8-poc`.
- `codex_v8_poc` -> `ontocode_v8_poc`.
- Identity-only package/lib/Bazel rename.

## Evidence

- Cargo metadata reports zero direct reverse dependencies.
- Direct text inventory has five active refs confined to root workspace metadata, `v8-poc/Cargo.toml`, and `v8-poc/BUILD.bazel`.
- OntoIndex CLI `impact --repo codex 'Function:ontocode-rs/v8-poc/src/lib.rs:bazel_target'` reports LOW risk with zero direct impacted nodes.

## Guardrails

- Preserve V8 proof-of-concept behavior.
- Preserve V8 sandbox feature semantics, embedded V8/version checks, and CRDTP tests.
- Preserve existing Bazel target labels and `v8-poc` directory path.
- Preserve env/config/wire/generated names, telemetry/product strings, and persisted state.

## Decision

R5K is approved for dispatch as a bounded residual package identity slice. No behavior edits or Bazel label rewrites are approved.
