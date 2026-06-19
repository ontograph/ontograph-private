# R4G Plugin Rename Closure

Date: 2026-06-11

Scope:
- Accepted identity-only rename `codex-plugin` -> `ontocode-plugin`.
- Accepted crate/lib import rename `codex_plugin` -> `ontocode_plugin`.
- Preserved existing `plugin` folder path, plugin ID parsing/validation/display behavior, plugin manifest schema/keys, plugin install/discovery/import behavior, Codex Apps/mention compatibility behavior, MCP/plugin filtering behavior, telemetry/product strings, env/config semantics, wire/generated names, and persisted state.

Manager verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-plugin --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core plugin --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server plugin --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `rg -n "\\bcodex_plugin\\b|\\bcodex-plugin\\b" ontocode-rs --glob "!target" || true`
- `git diff --check`
- OntoIndex `gn_verify_diff` scoped to the R4G changed file set passed.

Notes:
- Worker verification already covered fmt, core-plugins, core-skills, MCP plugin, TUI plugin, Bazel lock update/check, stale-reference classification, and scoped OntoIndex verification.
- No active `codex_plugin` crate refs remain.
- Remaining `codex-plugin` refs are intentional compatibility manifest paths, sample plugin IDs, and test temp stems.
- Remaining R4 crates require fresh one-slice senior risk review before dispatch.
