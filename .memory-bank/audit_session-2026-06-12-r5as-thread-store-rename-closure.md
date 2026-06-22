# R5AS Thread Store Rename Closure

## Scope

Accepted the identity-only residual crate rename:

- `codex-thread-store` -> `ontocode-thread-store`
- `codex_thread_store` -> `ontocode_thread_store`

## Preserved Behavior

- Thread IDs, rollout paths, SQLite schema/data, archive semantics, resume semantics, list/search/sort behavior, metadata patch behavior, live writer lifecycle, in-memory store behavior, agent resume behavior, hook runtime reads, app-server thread APIs, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `thread-store` directory path stayed unchanged.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-thread-store --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server thread_read`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server thread_resume`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server thread_unarchive`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server remote_thread_store`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core thread_rollback`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core resume_agent`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_thread_store|codex-thread-store`
- Cargo metadata residual count: 28 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Notes

- Active old thread-store identity refs are clean.
- OntoIndex reports the known broad high-risk dirty tree from the accumulated rename program.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
