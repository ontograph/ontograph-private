# R5BF Core Plugins Rename Worker Verification

Date: 2026-06-12

## Verification

- Renamed `codex-core-plugins` -> `ontocode-core-plugins` and `codex_core_plugins` -> `ontocode_core_plugins`.
- Preserved plugin manifest parsing, plugin app/MCP/skill loading, plugin telemetry metadata, marketplace add/remove/upgrade behavior, remote marketplace/catalog/share behavior, plugin install/uninstall/read/list behavior, config toggles, app-server plugin/config/external-agent processors, CLI plugin command behavior, core discoverable plugin behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `core-plugins` directory path.
- Passed `CARGO_BUILD_JOBS=8 just test -p ontocode-core-plugins --no-tests=pass`, `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`, focused `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server plugin_`, `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`, focused `CARGO_BUILD_JOBS=8 just test -p ontocode-core plugin_install discoverable`, `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`, focused `CARGO_BUILD_JOBS=8 just test -p ontocode-cli plugin_ marketplace_`, `CARGO_BUILD_JOBS=8 just fmt`, `CARGO_BUILD_JOBS=8 just bazel-lock-update`, `CARGO_BUILD_JOBS=8 just bazel-lock-check`, stale-reference search, `cargo metadata --format-version 1 --no-deps` residual count, `git diff --check`, and OntoIndex `detect-changes --repo codex`.
- Cargo metadata reports 15 remaining `codex-*` packages.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
