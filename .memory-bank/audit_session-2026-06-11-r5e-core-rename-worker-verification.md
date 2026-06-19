# R5E Core Rename Worker Verification

Date: 2026-06-11

Scope:
- Implemented identity-only Cargo package/lib/Bazel/import rename `codex-core` -> `ontocode-core` and standalone crate import rename `codex_core` -> `ontocode_core`.
- Preserved the existing `ontocode-rs/core` directory path.
- Preserved session lifecycle, turn execution, model-client behavior, tool orchestration, shell/unified-exec/apply-patch behavior, guardian review behavior, config/env/wire/generated names, telemetry/product strings, persisted state, protocol names, and public command names.
- Did not rename `codex-core-plugins` or `codex-core-skills`.

OntoIndex:
- MCP repo wiring remained wrong for this checkout: the MCP facade reported available repo `OntoIndex` instead of `/opt/demodb/_workfolder/ontocode`.
- CLI status confirmed the indexed repo path `/opt/demodb/_workfolder/ontocode` was available and up to date before edits.
- CLI impact for `Struct:ontocode-rs/core/src/session/mod.rs:Codex` was CRITICAL: 41 impacted nodes, 21 direct, 3 affected execution flows, and 7 affected modules.
- CLI fallback `detect-changes --repo codex` was used for final verification.

Changed surfaces:
- `ontocode-rs/core/Cargo.toml`, `ontocode-rs/core/BUILD.bazel`, `ontocode-rs/core/README.md`, and core Rust imports now use `ontocode-core` / `ontocode_core`.
- Direct dependent manifests/imports were updated for app-server, app-server-client, app-server-transport, CLI, exec, MCP server, LM Studio, TUI, core-api, root workspace metadata, lockfiles, and internal crate-identity docs.
- `.config/nextest.toml` package filters were updated to `package(ontocode-core)`.

Verification:
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-core --no-tests=pass` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-cli --no-tests=pass` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server --no-tests=pass` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-exec --no-tests=pass` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server --no-tests=pass` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-update` passed.
- `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- `cargo metadata --format-version 1 --no-deps` confirmed package `ontocode-core` and lib target `ontocode_core`.
- `git diff --check` passed after memory-bank updates.
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex` completed via CLI fallback and reported high risk across the broad dirty tree: 200 changed files, 328 symbols, and 8 affected processes.

Stale reference classification:
- Remaining `codex_core` refs are only TUI historical fixture log strings, not active Rust imports.
- Remaining `codex-core` refs are intentional `codex-core-plugins` / `codex-core-skills` identities, compatibility path/workspace strings, telemetry/metrics labels, test temp stems, preserved command fixture text, protocol/docs/comments, and public/runtime compatibility names.

Manager follow-up:
- Manager review/acceptance remains before closing R5E and Stage 5.
- OntoIndex MCP repo wiring should be fixed; worker verification used CLI fallback because MCP verification could not target this repository.
