# R5AW Connectors Rename Risk Review

## Decision

Dispatch R5AW as an identity-only residual package rename:

- `codex-connectors` -> `ontocode-connectors`
- `codex_connectors` -> `ontocode_connectors`

## OntoIndex Impact

- `connector_display_label`: CRITICAL, 21 impacted nodes, TUI connector/mention UI and plugin/session paths.
- `connector_mention_slug`: CRITICAL, 16 impacted nodes, affected `run_turn` process through explicit app-id extraction.
- `cached_directory_connectors`: CRITICAL, 12 impacted nodes, core, ChatGPT, plugin/app-server, and session paths.

## Direct Inventory

- Root workspace metadata and connectors manifest/Bazel identity.
- Dependent manifests/imports in `chatgpt`, `core`, and `tui`.

## Guardrails

- Preserve connector directory cache keys, TTL, and disk cache format.
- Preserve connector filtering, merge semantics, metadata normalization, install URL generation, display labels, and mention slugs.
- Preserve plugin/app duplicate handling.
- Preserve TUI connector popup behavior.
- Preserve core session explicit app-id extraction and tool-suggest connector behavior.
- Preserve ChatGPT connector directory behavior.
- Preserve app-server plugin behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `connectors` directory path.

## Verification Required

- `CARGO_BUILD_JOBS=8 just test -p ontocode-connectors --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-chatgpt connectors`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core connectors`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core collect_explicit_app_ids_from_skill_items`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_connectors|codex-connectors`
- Cargo metadata residual count, expected 24 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`
