# R5R LM Studio Rename Risk Review

Date: 2026-06-11

## Decision

- Approved next residual slice: `codex-lmstudio` -> `ontocode-lmstudio`.
- Approved crate import rename: `codex_lmstudio` -> `ontocode_lmstudio`.
- Scope is identity-only package/lib/Bazel/import rename.

## Inventory

- Cargo metadata reports 56 remaining `codex-*` workspace packages before this slice.
- Direct reverse dependency: `ontocode-utils-oss`.
- Active refs: 8 refs across root workspace metadata, the LM Studio manifest/Bazel identity, and OSS utility dependency/import/test usage.

## OntoIndex

- `Const:ontocode-rs/lmstudio/src/lib.rs:DEFAULT_OSS_MODEL`: LOW impact.
- `Function:ontocode-rs/lmstudio/src/lib.rs:ensure_oss_ready`: LOW impact.
- Both exact targets report 0 impacted nodes, 0 affected processes, and 0 affected modules.

## Guardrails

- Preserve LM Studio provider IDs.
- Preserve default model value.
- Preserve `lmstudio` command/process behavior.
- Preserve model loading/downloading readiness behavior.
- Preserve OSS provider selection behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `lmstudio` directory path.

## Verification Required

- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-lmstudio --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-oss --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Active-source stale-reference classification for `codex_lmstudio|codex-lmstudio`.
- `git diff --check`.
- OntoIndex CLI fallback `detect-changes --repo codex`.
