# R5BF Core Plugins Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-core-plugins` -> `ontocode-core-plugins`.
- Accepted `codex_core_plugins` -> `ontocode_core_plugins`.
- Scope remained package/lib/Bazel/import identity only.

## Risk

- OntoIndex exact impact was CRITICAL for `load_plugin_apps`: 15 impacted nodes, 4 direct, 5 modules, 1 affected process.
- OntoIndex exact impact was CRITICAL for `PluginInstallRequest`: 41 impacted nodes, 11 direct, 8 modules, 2 affected processes.
- OntoIndex graph risk for `PluginsManager` was UNKNOWN.
- Scope was accepted only because no plugin loading, install, remote, share, config, app-server, CLI, or discoverable plugin behavior changed.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-core-plugins --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server plugin_`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core plugin_install discoverable`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli plugin_ marketplace_`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Scoped stale-reference search for `codex_core_plugins|codex-core-plugins`
- Cargo metadata residual package count
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

## Result

- Active old crate refs are clean.
- `git diff --check` is clean.
- Cargo metadata reports 15 remaining `codex-*` packages.
- OntoIndex `detect-changes --repo codex` reports the known broad high-risk dirty tree after one transient FTS timeout retry.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
