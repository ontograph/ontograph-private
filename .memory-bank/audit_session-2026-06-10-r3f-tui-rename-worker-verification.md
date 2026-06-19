# R3F TUI Rename Worker Verification

Date: 2026-06-10

## Scope

- Renamed Cargo package identity `ontocode-tui` -> `ontocode-tui`.
- Renamed TUI lib crate identity/imports `codex_tui` -> `ontocode_tui`.
- Updated direct workspace, CLI, cloud-tasks, TUI, Bazel, lockfile, script, and lint metadata references.

## Preserved

- Standalone `ontocode-tui` binary name.
- Telemetry/client-name strings, product client IDs, originator allowlists, tool/plugin gating strings.
- Existing `codex_tui__...` snapshot filenames via a test-only TUI `insta` shim.
- Log file name, command-history fixture text, public command names, env/config semantics, persisted state, app-server startup behavior, TUI runtime behavior, CLI behavior, and MCP behavior.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass` passed: 2772 passed, 4 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server` passed: 25 passed, 236 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli config_overrides_from_interactive_preserves_global_options` passed: 1 passed, 260 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-cloud-tasks --no-tests=pass` passed: 13 passed, 1 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server --no-tests=pass` passed: 14 passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-tui` passed.
- Active stale-reference search found only approved compatibility references.
- `git diff --check` passed.
- Scoped OntoIndex verification passed for the R3F changed files.
