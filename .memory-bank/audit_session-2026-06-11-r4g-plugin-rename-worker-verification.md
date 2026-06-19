# R4G Plugin Rename Worker Verification

Date: 2026-06-11

Scope:
- `codex-plugin` -> `ontocode-plugin`
- `codex_plugin` -> `ontocode_plugin`
- Identity-only Cargo package/lib/Bazel/import rename with `ontocode-rs/plugin` path preserved.

Preserved:
- Plugin ID parsing, validation, and display behavior.
- Plugin manifest schema and `.codex-plugin/plugin.json` compatibility path.
- Plugin install, discovery, import, remote, cache, and sharing behavior.
- Codex Apps/mention compatibility and MCP/plugin filtering behavior.
- Telemetry/product strings, env/config semantics, generated wire names, and persisted state.

Verification:
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
- `rg -n "\bcodex_plugin\b|\bcodex-plugin\b" ontocode-rs --glob "!target" || true`
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff` with explicit R4G changed-file set.

Results:
- All required verification passed.
- No active `codex_plugin` crate refs remain under `ontocode-rs`.
- Remaining `codex-plugin` refs are intentional compatibility references: `.codex-plugin/plugin.json` manifest paths, test/sample plugin IDs, and a TUI test temp directory stem.
