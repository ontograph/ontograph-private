# R5AL Test Binary Support Rename Risk Review

Date: 2026-06-12
Status: approved for identity-only dispatch
Model fallback: `gpt-5.4-mini` because the required Spark model is unavailable or usage-limited.

## Scope

- Rename Cargo package `codex-test-binary-support` to `ontocode-test-binary-support`.
- Rename Rust crate import `codex_test_binary_support` to `ontocode_test_binary_support`.
- Update workspace metadata, dependent test-only imports, and Bazel crate identity.
- Preserve the existing `test-binary-support` folder path.

## Direct Inventory

- Direct reverse dependencies: `ontocode-exec-server`, `ontocode-core`.
- Active refs are confined to:
  - `ontocode-rs/Cargo.toml`
  - `ontocode-rs/Cargo.lock`
  - `ontocode-rs/test-binary-support/Cargo.toml`
  - `ontocode-rs/test-binary-support/BUILD.bazel`
  - `ontocode-rs/exec-server/Cargo.toml`
  - `ontocode-rs/exec-server/tests/common/mod.rs`
  - `ontocode-rs/core/Cargo.toml`
  - `ontocode-rs/core/tests/suite/mod.rs`

## OntoIndex Impact

- `configure_test_binary_dispatch`: LOW, 0 impacted symbols, no affected processes.
- `TestBinaryDispatchMode`: LOW, 0 impacted symbols, no affected processes.
- `Struct:ontocode-rs/test-binary-support/lib.rs:TestBinaryDispatchGuard`: LOW, 1 direct module-level impacted symbol, no affected processes.
- `Impl:ontocode-rs/test-binary-support/lib.rs:TestBinaryDispatchGuard`: LOW, 0 impacted symbols, no affected processes.

## Guardrails

- Do not change dispatch semantics, ctor timing assumptions, tempdir lifetime, or arg0 alias installation behavior.
- Preserve `CODEX_HOME` environment compatibility and restoration behavior.
- Do not rename user-visible test binary names or runtime helper aliases.
- Do not touch unrelated residual `codex-*` packages.
- Run package/dependent checks, fmt, Bazel lock update/check, stale-reference classification, `git diff --check`, and OntoIndex diff detection before closure.
