# R4E Model Provider Info Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-model-provider-info` -> `ontocode-model-provider-info`.
- Accepted `codex_model_provider_info` -> `ontocode_model_provider_info`.
- Scope stayed identity-only: Cargo package, Rust lib crate, Bazel crate/import wiring, dependent imports, and lockfiles.
- Preserved provider catalog semantics, built-in provider IDs, wire API values, model/provider merge and validation behavior, OSS/Ollama/LMStudio/Bedrock/OpenAI provider behavior, config schema/loader behavior, app-server thread config behavior, TUI status behavior, exec/client behavior, telemetry/product strings, env/config semantics, wire/generated names, persisted state, and the existing directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider-info --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-model-provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core model_provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server model_provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui status --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-ollama --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-lmstudio --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference scan found no active `codex_model_provider_info` or `codex-model-provider-info` refs in `ontocode-rs`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff` passed with explicit R4E changed files and executed tests.

## Result

- R4E is accepted.
- Remaining R4 support crates require a fresh one-slice senior risk review before dispatch.
