# R5AR Memories Write Rename Closure

Date: 2026-06-12

## Closure

Accepted R5AR: `codex-memories-write` -> `ontocode-memories-write` and `codex_memories_write` -> `ontocode_memories_write`.

The rename stayed identity-only across Cargo package metadata, Rust lib crate name, Bazel crate identity, workspace dependencies, README identity text, and direct imports.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-write --no-tests=pass`: passed, 30 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli debug_clear_memories`: passed, 2 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server memory_reset_clears_memory_files_and_rows_preserves_threads`: passed, 1 test.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server turn_start_additional_context_flows_to_model_input`: passed, 1 test.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`: passed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`: passed.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active-source stale-reference search for `codex_memories_write|codex-memories-write`: clean.
- Cargo metadata residual `codex-*` package count: 29.
- `git diff --check`: clean.
- OntoIndex `detect-changes --repo codex`: high risk from the known broad dirty tree, not a new memories-write-specific blocker.

## Preserved Surfaces

- Memory root paths, artifact filenames, extension pruning, raw memory rebuilding, rollout summary sync, startup task behavior, model/reasoning constants, leases/retry timing, token limits, workspace diff caps, and redaction behavior.
- CLI `debug clear-memories`, app-server memory reset, app-server turn start, persisted memory state, env/config/wire/generated names, telemetry/product strings, and `ontocode-rs/memories/write` directory path.

## Model Fallback

Worker and manager recorded fallback use of `gpt-5.4-mini` because `gpt-5.3-codex-spark` was unavailable or usage-limited.
