# R4F Model Provider Rename Closure

Date: 2026-06-11

Scope:
- Accepted identity-only rename `codex-model-provider` -> `ontocode-model-provider`.
- Accepted crate/lib import rename `codex_model_provider` -> `ontocode_model_provider`.
- Preserved existing `model-provider` folder path, provider runtime behavior, auth/header behavior, models endpoint behavior, native-provider behavior, capabilities/account-state behavior, catalog/model-list behavior, telemetry/product strings, env/config semantics, wire/generated names, and persisted state.

Manager verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core remote_models --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server model_provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n "\\bcodex_model_provider\\b|\\bcodex-model-provider\\b" ontocode-rs --glob "!target" || true`
- `git diff --check`
- OntoIndex `gn_verify_diff` scoped to the R4F changed file set passed.

Notes:
- Worker verification already covered fmt, provider-info, config, login, analytics, core model-provider compile, TUI status, Ollama, LM Studio, Bazel lock update/check, stale-reference search, and scoped OntoIndex verification.
- Remaining R4 crates require fresh one-slice senior risk review before dispatch.
