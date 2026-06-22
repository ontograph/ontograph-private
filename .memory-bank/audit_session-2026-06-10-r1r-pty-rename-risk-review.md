# R1R PTY Rename Risk Review

Date: 2026-06-10

## Decision

Approve one exact identity-only slice:

- `codex-utils-pty` -> `ontocode-utils-pty`

## OntoIndex Impact

- `combine_output_receivers`: CRITICAL, 18 impacted nodes, 2 direct, 1 affected process, 6 modules.
- `SpawnedProcess`: CRITICAL, 29 impacted nodes, 4 direct, 3 affected processes, 12 modules.
- `spawn_from_driver`: HIGH, 10 impacted nodes, 5 direct, 2 affected processes, 4 modules.
- `TerminalSize`: LOW, 5 impacted nodes, 4 direct, 2 modules.

## Direct Dependents

- `ontocode-app-server`
- `codex-core`
- `ontocode-exec-server`
- `codex-rmcp-client`
- `codex-tools`
- `ontocode-windows-sandbox`

## Allowed Scope

- Cargo package rename.
- Rust library crate rename.
- Bazel crate-name metadata update.
- Workspace/dependent manifest updates.
- Rust import path updates.
- Lockfile updates.

## Non-Scope

- No PTY or pipe process behavior changes.
- No output receiver or broadcast semantics changes.
- No process-group, termination, detach, or parent-death behavior changes.
- No inherited-fd behavior changes.
- No terminal-size or ConPTY detection behavior changes.
- No app-server command-exec behavior changes.
- No exec-server protocol or local-process behavior changes.
- No RMCP stdio launch behavior changes.
- No Windows sandbox process adapter behavior changes.
- No public command, telemetry, env/config, protocol, persisted-state, or runtime layout changes.

## Required Verification

- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-pty`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-rmcp-client`
- `CARGO_BUILD_JOBS=8 just test -p codex-tools`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox`
- Focused process/session/command-exec/unified-exec/local-process/RMCP-stdio/tool-config/Windows-sandbox tests where available.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex-utils-pty|codex_utils_pty`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.
