# R4G Plugin Rename Risk Review

Date: 2026-06-11

Candidate:
- `codex-plugin` -> `ontocode-plugin`
- `codex_plugin` -> `ontocode_plugin`

OntoIndex evidence:
- Tool: OntoIndex CLI against repo `codex`.
- Indexed repo path: `/opt/demodb/_workfolder/ontocode`.
- `Struct:ontocode-rs/plugin/src/plugin_id.rs:PluginId`: LOW impact, 0 upstream impacted nodes.

Direct inventory:
- `codex-plugin` package refs under `ontocode-rs`: 228.
- `codex_plugin` crate refs under `ontocode-rs`: 74.
- Scope is broad enough to require plugin/core/MCP/TUI/app-server dependent verification even though graph impact is LOW.

Approved scope:
- Identity-only Cargo package/lib/Bazel/import rename.
- Preserve existing `plugin` folder path.
- Preserve plugin ID parsing/validation/display behavior.
- Preserve plugin manifest schema/keys and plugin install/discovery/import behavior.
- Preserve Codex Apps/mention compatibility behavior and MCP/plugin filtering behavior.
- Preserve telemetry/product strings, env/config semantics, wire/generated names, and persisted state.

Rejected scope:
- No plugin runtime behavior changes.
- No manifest schema migration.
- No public command, config-key, persisted-state, or generated-wire rename.
- No broad find-and-replace outside active package/lib/Bazel/import references.

Required verification:
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-plugin --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-plugins --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-skills --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp plugin --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core plugin --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui plugin --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server plugin --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for active `codex_plugin` / `codex-plugin` refs under `ontocode-rs`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.
