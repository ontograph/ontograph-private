# R5Q Stdio To UDS Rename Risk Review

Date: 2026-06-11

## Decision

- Approved next residual slice: `ontocode-stdio-to-uds` -> `ontocode-stdio-to-uds`.
- Approved crate import rename: `codex_stdio_to_uds` -> `ontocode_stdio_to_uds`.
- Scope is identity-only package/lib/Bazel/import rename.

## Inventory

- Cargo metadata reports 57 remaining `codex-*` workspace packages before this slice.
- Direct reverse dependency: `ontocode-cli`.
- Active refs: 16 refs across root workspace metadata, `ontocode-cli` dependency/import call sites, stdio-to-uds manifest/Bazel identity, README helper examples, binary usage text, and tests.

## OntoIndex

- `Function:ontocode-rs/stdio-to-uds/src/lib.rs:run`: LOW impact.
- Impact summary: 0 impacted nodes, 0 affected processes, 0 affected modules.
- Path-qualified `stdio-to-uds/src/main.rs::main` did not resolve; this is recorded as an OntoIndex symbol-resolution limitation, not a scope blocker.

## Guardrails

- Preserve stdio/UDS relay behavior.
- Preserve Unix socket transport behavior.
- Preserve CLI MCP proxy dispatch behavior.
- Preserve public `ontocode-stdio-to-uds` executable name.
- Preserve README/MCP command examples that reference `ontocode-stdio-to-uds`.
- Preserve tests that spawn the compatibility helper command unless a compatibility test is added first.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `stdio-to-uds` directory path.

## Verification Required

- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-stdio-to-uds --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Active-source stale-reference classification for `codex_stdio_to_uds|ontocode-stdio-to-uds`.
- `git diff --check`.
- OntoIndex CLI fallback `detect-changes --repo codex`.
