# R5AH Memories Read Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-memories-read` -> `ontocode-memories-read`.
- Accepted `codex_memories_read` -> `ontocode_memories_read`.
- Scope was identity-only package/lib/Bazel/import rename.
- Manager takeover was required because the worker handle disappeared after applying the code changes.

## Preserved Surfaces

- Memory root path semantics.
- Memory citation parsing and rollout/thread id extraction.
- Hidden assistant markup handling through core stream events.
- Memory usage command classification and metric tags.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the `memories/read` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-read --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core stream_events_utils`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core memory_usage --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core memory_tool_makes_memories_root_readable_without_creating_or_widening_writes`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core workspace_write_includes_configured_writable_root_once_without_memories_root`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_memories_read|codex-memories-read`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5AH.
- Active old refs are clean in `ontocode-rs`.
- Cargo metadata reports 39 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5AH-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
