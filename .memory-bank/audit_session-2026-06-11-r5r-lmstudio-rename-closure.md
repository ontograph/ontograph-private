# R5R LM Studio Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-lmstudio` -> `ontocode-lmstudio`.
- Accepted `codex_lmstudio` -> `ontocode_lmstudio`.
- Identity-only package/lib/Bazel/import rename.
- Preserved LM Studio provider IDs, default model value, `lmstudio` command/process behavior, model loading/downloading readiness behavior, OSS provider selection behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `lmstudio` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-lmstudio --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-oss --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active-source stale-reference search for `codex_lmstudio|codex-lmstudio`: clean.
- `git diff --check`: passed through lean-ctx wrapper.
- Cargo metadata reports 55 remaining `codex-*` workspace packages.
- OntoIndex CLI fallback `detect-changes --repo codex` still reports the known broad dirty-tree high-risk context, not a scoped R5R-only blocker.

## Notes

- Worker verification completed on fallback model `gpt-5.4-mini` after the `gpt-5.3-codex-spark` usage-limit fallback.
- Known unrelated warnings remain: duplicate Windows sandbox bin targets and the existing `ontocode-core` dead-code warning for token-usage breakdown fields.
- R6 cleanup remains blocked while residual `codex-*` package identities remain.
