# R4E Model Provider Info Rename Risk Review

Date: 2026-06-11

## Decision

- Approve one identity-only slice: `codex-model-provider-info` -> `ontocode-model-provider-info` and `codex_model_provider_info` -> `ontocode_model_provider_info`.
- Do not rename the existing `ontocode-rs/model-provider-info/` directory path.
- Do not change provider catalog, config, wire, status, client, telemetry, environment, persisted-state, or generated schema behavior.

## OntoIndex Evidence

- OntoIndex CLI repo: `codex`, path `/opt/demodb/_workfolder/ontocode`.
- Target: `Struct:ontocode-rs/model-provider-info/src/lib.rs:ModelProviderInfo`.
- Risk: CRITICAL.
- Impact: 49 impacted nodes, 29 direct nodes, 10 affected modules.
- Directly visible impact includes `model-provider-info` tests, `model-provider`, `config`, `core`, `app-server`, `login`, `ollama`, and TUI status paths.

## Direct Inventory

- Package refs: `codex-model-provider-info` appears in root workspace plus app-server, config, core-api, core, exec, image/web-search extensions, lmstudio, login, model-provider, ollama, and TUI manifests.
- Crate refs: `codex_model_provider_info` appears broadly in provider catalog/config/client/status tests and runtime imports.
- Existing folder path `model-provider-info` remains in scope as a path only, not a public identity rename.

## Required Preservation

- Built-in provider IDs and provider catalog semantics.
- Wire API enum/string values and generated API/schema names.
- Model provider merge, validation, auth, and runtime-engine selection behavior.
- OSS/Ollama/LMStudio/Bedrock/OpenAI provider behavior.
- Config loader/schema behavior and app-server thread config behavior.
- TUI status display behavior, exec/client behavior, telemetry/product strings, env/config semantics, persisted state, and folder path.

## Required Verification

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
- Stale-reference scans for active `codex_model_provider_info` and `codex-model-provider-info` refs, with intentional folder/path compatibility classified.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff` or CLI `detect-changes`.
