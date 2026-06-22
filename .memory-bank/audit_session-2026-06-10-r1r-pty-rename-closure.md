# R1R PTY Utility Rename Closure

Date: 2026-06-10

## Scope

- Renamed internal Cargo package/library crate identity from `codex-utils-pty` / `codex_utils_pty` to `ontocode-utils-pty` / `ontocode_utils_pty`.
- Updated root workspace dependency, direct dependent manifests, PTY crate manifest, Bazel `crate_name`, Rust import paths, README direct package references, `Cargo.lock`, and `MODULE.bazel.lock`.
- No PTY, pipe, process, output receiver, process-group, inherited-fd, terminal-size, ConPTY, app-server command-exec, exec-server, RMCP stdio, Windows sandbox, public command, telemetry, env/config, protocol, runtime layout, or persisted-state behavior changed.

## Verification

- PASS: `cargo metadata --format-version 1 --no-deps`.
- PASS: `CARGO_BUILD_JOBS=8 just fmt`.
- PASS: `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-pty` (16 passed).
- PASS: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server` (810 passed, 1 skipped).
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-core` (2648 passed, 14 skipped).
- PASS: `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server` (196 passed).
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-rmcp-client` (64 passed).
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-tools` (80 passed).
- PASS: `CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox` (10 passed).
- PASS: focused coverage through the package suites covered PTY/pipe/inherited-fd/process-group tests, app-server command/process exec, core unified-exec/write-stdin/shell snapshot, exec-server local-process/stdio lifecycle, RMCP stdio process-group cleanup, tool-config ConPTY gating, and Windows sandbox adapter compile/tests.
- PASS: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- PASS: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- PASS: stale-reference search for `codex-utils-pty|codex_utils_pty` under `ontocode-rs`, `ontocode-rs/Cargo.lock`, and `MODULE.bazel.lock`.
- PASS: `git diff --check`.
- PASS: scoped OntoIndex `gn_verify_diff`.

## Notes

- OntoIndex repo path was `/opt/demodb/_workfolder/ontocode`.
- OntoIndex pre-edit impact was CRITICAL for `combine_output_receivers` and `SpawnedProcess`, HIGH for `spawn_from_driver`, and LOW for `TerminalSize`.
- Implementation stayed identity-only.
- No blockers or PTY/process-related verification failures were observed.
