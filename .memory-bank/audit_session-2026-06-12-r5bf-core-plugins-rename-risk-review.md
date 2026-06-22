# R5BF Core Plugins Rename Risk Review

Date: 2026-06-12

## Decision

- Approve exactly one residual slice: `codex-core-plugins` -> `ontocode-core-plugins`.
- Approve crate import rename: `codex_core_plugins` -> `ontocode_core_plugins`.
- Scope is identity-only: package metadata, library crate name, Bazel crate name, Cargo lock, and dependent imports.

## OntoIndex Impact

- Exact `load_plugin_apps`: CRITICAL, 15 impacted nodes, 4 direct, 5 affected modules, 1 affected process.
- Exact `PluginInstallRequest`: CRITICAL, 41 impacted nodes, 11 direct, 8 affected modules, 2 affected processes.
- Exact `PluginsManager`: UNKNOWN graph risk, 0 impacted nodes.

## Direct Active References

- Root workspace dependency metadata.
- `core-plugins` manifest identity.
- App-server manifest, config, request processors, plugin processors, external-agent config, and plugin tests.
- Core plugin/discoverable imports and tests.
- CLI plugin command path through impact graph.
- Cargo lock entries.

## Guardrails

- Preserve plugin manifest parsing.
- Preserve plugin app, MCP, and skill loading.
- Preserve plugin telemetry metadata.
- Preserve marketplace add/remove/upgrade behavior.
- Preserve remote marketplace, catalog, and share behavior.
- Preserve plugin install, uninstall, read, and list behavior.
- Preserve config toggles.
- Preserve app-server plugin/config/external-agent processors.
- Preserve CLI plugin command behavior.
- Preserve core discoverable plugin behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `core-plugins` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-core-plugins --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`.
- Focused app-server plugin/config/external-agent plugin tests where available.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`.
- Focused core plugin/discoverable tests where available.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`.
- Focused CLI plugin command checks where available.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Scoped stale-reference search for `codex_core_plugins|codex-core-plugins`.
- Metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
