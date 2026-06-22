# R5BI Models Manager Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-models-manager` -> `ontocode-models-manager` and `codex_models_manager` -> `ontocode_models_manager`.
- Scope stayed identity-only: package metadata, library crate name, Bazel target/deps, imports, and lockfiles.
- Preserved model catalog loading, bundled `models.json`, cache TTL/etag behavior, remote model refresh, default model selection, model override precedence, collaboration mode preset contents, auth/backend behavior, app-server/TUI/CLI model listing behavior, config keys, wire/generated names, telemetry/product strings, persisted state, and the existing `models-manager` directory path.

## Verification

- Worker verification passed for `ontocode-models-manager`, compile-only core/app-server/TUI/CLI checks, fmt, Bazel lock update/check, stale-reference search, metadata count, diff check, and OntoIndex `detect-changes --repo codex`.
- Manager hygiene confirmed no `codex_models_manager` or `codex-models-manager` refs remain in `ontocode-rs`.
- Manager metadata check reports 12 remaining `codex-*` packages.
- Manager `git diff --check` is clean.

## Notes

- Huygens `019ebd12-dfaf-7b11-a05f-21a552ed79ee` completed the scoped patch and verification on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
