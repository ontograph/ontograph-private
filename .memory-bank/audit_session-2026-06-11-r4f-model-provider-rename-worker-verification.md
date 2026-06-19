# R4F Model Provider Rename Worker Verification

Date: 2026-06-11

## Scope

- Implemented the identity-only rename `codex-model-provider` -> `ontocode-model-provider`.
- Implemented the identity-only crate import rename `codex_model_provider` -> `ontocode_model_provider`.
- Kept the existing `ontocode-rs/model-provider/` directory path unchanged.

## Preservation

- Preserved provider runtime selection, auth-provider/header behavior, models endpoint behavior, Bedrock/Gemini/Claude/Copilot/native-provider behavior, provider capabilities/account-state behavior, catalog/model-list behavior, telemetry/product strings, env/config semantics, wire/generated names, persisted state, and app-server/CLI/TUI/core behavior.

## Verification

- Passed `CARGO_BUILD_JOBS=8 just fmt`.
- Passed focused model-provider, provider-info, config, login, analytics, core model-provider compile, core remote_models, app-server model_provider, TUI status, Ollama, and LM Studio checks.
- Passed `CARGO_BUILD_JOBS=8 just bazel-lock-update` and `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Stale-reference searches found no remaining active `codex-model-provider` or `codex_model_provider` refs in `ontocode-rs`.
- `git diff --check` passed.
- Scoped OntoIndex verification passed.
