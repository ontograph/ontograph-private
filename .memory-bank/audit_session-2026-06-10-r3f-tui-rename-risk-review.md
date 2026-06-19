# R3F TUI Rename Risk Review

Date: 2026-06-10

## Candidate

- `ontocode-tui` -> `ontocode-tui`.
- Scope approved only for package/lib/Bazel/import identity changes.

## OntoIndex Evidence

- `tui/src/lib.rs::run_main`: HIGH, impacted through `run_interactive_tui`, `cli_main`, CLI main, and MCP server main.
- `tui/src/lib.rs::start_app_server`: HIGH, impacted through TUI app startup and CLI main.
- `tui/src/cli.rs::Cli`: LOW, 0 upstream impacted nodes.
- `tui/src/main.rs::main`: LOW, 0 upstream impacted nodes.
- OntoIndex repo path: `/opt/demodb/_workfolder/ontocode`.

## Direct Inventory

- Direct package/import refs include root workspace metadata, `codex-cli`, `codex-cloud-tasks`, TUI manifests/build metadata, TUI tests, and TUI standalone main.
- Many `ontocode-tui` strings are compatibility surfaces: telemetry/client names, originator allowlists, tool-discovery gating, analytics fixtures, snapshots, log file names, docs/comments, and command-history text.

## Guardrails

- Do not change TUI runtime behavior, app-server startup behavior, CLI behavior, MCP behavior, telemetry/client-name strings, originator allowlists, plugin/tool gating, snapshot names, log file names, public command names, env/config semantics, or persisted state.
- Do not rename `codex-cli` in this slice.
- Preserve the standalone `ontocode-tui` binary name unless a package-identity test proves changing it is required.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli config_overrides_from_interactive_preserves_global_options`
- `CARGO_BUILD_JOBS=8 just test -p codex-cloud-tasks --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale-reference search for `ontocode-tui|codex_tui`, excluding intentional telemetry/client-name/originator/snapshot/log/docs/comment/history/binary compatibility strings.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`

## Decision

- Approved as R3F, one slice only.
