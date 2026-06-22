# R5BI Models Manager Rename Risk Review

Date: 2026-06-12

## Slice

- Rename `codex-models-manager` -> `ontocode-models-manager`.
- Rename Rust crate refs `codex_models_manager` -> `ontocode_models_manager`.
- Identity-only scope: package metadata, library crate name, Bazel target/deps, imports, and lockfiles.

## OntoIndex

- MCP `mcp__ontoindex` is still not wired to `/opt/demodb/_workfolder/ontocode`; it reports only repository `OntoIndex`.
- Local OntoIndex CLI status reports repository path `/opt/demodb/_workfolder/ontocode`, indexed/current commit `73ba304`, and up-to-date status.
- `ModelsManager`: LOW, 2 impacted nodes, 2 direct, no affected processes.
- `OpenAiModelsManager`: ambiguous/UNKNOWN.
- `builtin_collaboration_mode_presets`: ambiguous/UNKNOWN.
- `ModelPreset`: ambiguous/UNKNOWN.

## Guardrails

- Do not change model catalog loading, bundled `models.json`, cache TTL/etag behavior, remote model refresh, default model selection, model override precedence, collaboration mode preset contents, auth/backend behavior, app-server/TUI/CLI model listing behavior, config keys, wire/generated names, telemetry/product strings, persisted state, or the existing `models-manager` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-models-manager --no-tests=pass`
- Focused `ontocode-core` model catalog/manager checks.
- Compile-only or focused `ontocode-app-server` model/provider checks.
- Compile-only or focused `ontocode-tui` model picker/settings checks.
- Compile-only or focused `ontocode-cli` model command checks.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_models_manager|codex-models-manager`.
- Cargo metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
