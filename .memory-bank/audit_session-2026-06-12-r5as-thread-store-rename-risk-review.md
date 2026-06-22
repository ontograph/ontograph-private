# R5AS Thread Store Rename Risk Review

Date: 2026-06-12

## Decision

Dispatch R5AS as an identity-only residual package rename:

- `codex-thread-store` -> `ontocode-thread-store`
- `codex_thread_store` -> `ontocode_thread_store`

## OntoIndex Impact

- `ThreadStore`: LOW, 2 impacted implementations, no affected processes.
- `LocalThreadStore`: LOW, no affected processes.
- `InMemoryThreadStore`: LOW, no affected processes.
- `LiveThread`: LOW, no affected processes.
- `StoredThread`: HIGH, 19 impacted nodes, affected local search/read thread paths.
- `CreateThreadParams`: LOW, 26 impacted nodes, session creation and local writer tests.
- `ReadThreadParams`: HIGH, 49 impacted nodes, affected core turn, agent resume, hook, and local read paths.

## Scope

Allowed:

- Rename Cargo package metadata, Rust lib crate name, Bazel crate identity, dependency entries, README identity text, and Rust imports from old crate identity to Ontocode identity.
- Update lockfiles and generated Bazel lock metadata as required.

Forbidden:

- Do not change thread IDs, rollout paths, SQLite schema/data, archive semantics, resume semantics, list/search/sort behavior, metadata patch behavior, live writer lifecycle, in-memory store behavior, agent resume behavior, hook runtime reads, app-server thread APIs, env/config/wire/generated names, telemetry/product strings, persisted state, or folder path.
- Do not move the existing `ontocode-rs/thread-store` directory.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-thread-store --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- Focused app-server thread tests: `thread_read`, `thread_resume`, `thread_unarchive`, and `remote_thread_store`.
- Focused core thread/agent tests if available after the rename, especially `thread_rollback` and `resume_agent`.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_thread_store|codex-thread-store`
- Cargo metadata residual package count.
- `git diff --check`
- OntoIndex diff verification through `detect-changes --repo codex`

## Model Fallback

Dispatch uses `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.
