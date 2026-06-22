# R5AR Memories Write Rename Risk Review

Date: 2026-06-12

## Decision

Dispatch R5AR as an identity-only residual package rename:

- `codex-memories-write` -> `ontocode-memories-write`
- `codex_memories_write` -> `ontocode_memories_write`

## OntoIndex Impact

- `memory_root`: LOW, no affected processes.
- `clear_memory_roots_contents`: HIGH, 6 impacted nodes, affected CLI memory-clear and app-server memory-reset processes.
- `start_memories_startup_task`: LOW, 8 impacted nodes, affected app-server turn-start process.
- `rebuild_raw_memories_file_from_memories`: LOW, 3 impacted nodes, affected memories phase-2 write path.

## Scope

Allowed:

- Rename Cargo package metadata, Rust lib crate name, Bazel crate identity, dependency entries, and Rust imports from old crate identity to Ontocode identity.
- Update the memories README identity text if it refers to the package/lib name.
- Update lockfiles and generated Bazel lock metadata as required.

Forbidden:

- Do not change memory root paths, memory artifact filenames, extension pruning, raw memory rebuilding, rollout summary sync, startup task behavior, model/reasoning constants, leases/retry timing, token limits, workspace diff caps, or redaction behavior.
- Do not change CLI `debug clear-memories`, app-server memory reset, app-server turn start, persisted memory state, env/config/wire/generated names, telemetry/product strings, or folder path.
- Do not move the existing `ontocode-rs/memories/write` directory.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-write --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- Focused memory-clear/memory-reset/turn-start tests if available after the rename.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_memories_write|codex-memories-write`
- Cargo metadata residual package count.
- `git diff --check`
- OntoIndex diff verification through `detect-changes --repo codex`

## Model Fallback

Dispatch uses `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.
