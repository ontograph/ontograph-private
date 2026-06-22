# R5AS Thread Store Rename Worker Verification

Date: 2026-06-12

## Summary

Renamed `codex-thread-store` to `ontocode-thread-store` and `codex_thread_store`
to `ontocode_thread_store` as an identity-only change.

## Preserved Behavior

- Thread IDs, rollout paths, SQLite schema/data, archive semantics, resume semantics, list/search/sort behavior, metadata patch behavior, live writer lifecycle, in-memory store behavior, agent resume behavior, hook runtime reads, app-server thread APIs, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `thread-store` directory path.

## Verification

- Package tests, core/app-server compile checks, focused thread tests, focused core thread tests, `fmt`, Bazel lock update/check, active-source stale-reference search, cargo metadata residual count, `git diff --check`, and OntoIndex fallback verification passed.

## Model Fallback

- Used `gpt-5.4-mini` because `gpt-5.3-codex-spark` was unavailable or usage-limited.
