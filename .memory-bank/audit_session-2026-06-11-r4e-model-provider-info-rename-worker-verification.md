# R4E Model Provider Info Rename Worker Verification

Date: 2026-06-11

## Scope

- Implemented `codex-model-provider-info` -> `ontocode-model-provider-info`.
- Implemented `codex_model_provider_info` -> `ontocode_model_provider_info`.
- Kept `ontocode-rs/model-provider-info/` as the existing directory path.
- Preserved provider catalog, built-in provider IDs, wire API values, config behavior, app-server thread config behavior, TUI status behavior, exec/client behavior, telemetry/product strings, env/config semantics, wire/generated names, persisted state, and provider runtime behavior.

## Worker Verification

- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider-info --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-model-provider --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-config --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-login --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-core model_provider --no-tests=pass`: passed with zero selected tests and no compile failure.

## Manager Completion Required

- Manager completed the remaining verification matrix before acceptance.
