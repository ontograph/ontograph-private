# R5V Memories Extension Rename Risk Review

Date: 2026-06-11

## Decision

- Dispatch `codex-memories-extension` -> `ontocode-memories-extension`.
- Dispatch `codex_memories_extension` -> `ontocode_memories_extension`.
- Limit the slice to package/lib/Bazel/import identity only.

## Inventory

- Cargo metadata before R5V reports 52 remaining `codex-*` workspace packages.
- Direct reverse dependencies: 1, `ontocode-app-server`.
- Direct active refs: 6.
- Ref scope: root workspace metadata, app-server dependency/import wiring, and memories extension manifest/Bazel identity.

## OntoIndex

- `Function:ontocode-rs/ext/memories/src/extension.rs:install`: LOW impact, 0 impacted nodes, no affected processes.
- Repo path reported by the CLI fallback is `/opt/demodb/_workfolder/ontocode`.

## Guardrails

- Preserve memory tool namespace and tool names.
- Preserve add/list/read/search behavior.
- Preserve local memories backend behavior.
- Preserve prompt/template content and metrics behavior.
- Preserve app-server extension registration behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `ext/memories` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_memories_extension|codex-memories-extension`.
- `git diff --check`.
- OntoIndex CLI fallback `detect-changes --repo codex`.
