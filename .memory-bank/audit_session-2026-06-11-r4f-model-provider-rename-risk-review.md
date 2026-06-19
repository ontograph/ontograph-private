# R4F Model Provider Rename Risk Review

Date: 2026-06-11

## Decision

- Approve one identity-only slice: `codex-model-provider` -> `ontocode-model-provider` and `codex_model_provider` -> `ontocode_model_provider`.
- Do not rename the existing `ontocode-rs/model-provider/` directory path.
- Do not change provider runtime, auth, model-list, catalog, capability, account-state, config, telemetry, environment, persisted-state, wire, or generated schema behavior.

## OntoIndex Evidence

- OntoIndex CLI repo: `codex`, path `/opt/demodb/_workfolder/ontocode`.
- Target: `Trait:ontocode-rs/model-provider/src/provider.rs:ModelProvider`.
- Risk: LOW.
- Impact: 1 impacted node, 1 direct node, 0 affected modules.
- The graph under-represents textual/import dependency sensitivity, so direct inventory controls verification scope.

## Direct Inventory

- Package refs: `codex-model-provider` appears in root workspace plus analytics, app-server transport/server, core, login, TUI, and related provider/runtime manifests.
- Crate refs: `codex_model_provider` appears in analytics, app-server transport/server, core, login, TUI, and provider diagnostics/model endpoint paths.
- Existing folder path `model-provider` remains in scope as a path only, not a public identity rename.

## Required Preservation

- Provider runtime engine selection and native provider routing.
- Auth-provider/header behavior and account-state behavior.
- Models endpoint behavior, model-list parsing, and provider capabilities.
- Bedrock, Gemini, Claude, Copilot, OpenAI-compatible, Ollama, and LM Studio provider behavior.
- Config loader/schema behavior, app-server provider capabilities, TUI status/provider display, analytics auth-header usage, telemetry/product strings, env/config semantics, wire/generated names, persisted state, and folder path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider-info --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-config --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-login --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-analytics --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core model_provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core remote_models --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server model_provider --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui status --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-ollama --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-lmstudio --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference scans for active `codex_model_provider` and `codex-model-provider` refs, with intentional path/package compatibility classified.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff` or CLI `detect-changes`.
