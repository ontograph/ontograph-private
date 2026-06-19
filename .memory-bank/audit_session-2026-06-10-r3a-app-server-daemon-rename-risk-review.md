---
name: R3A App Server Daemon Rename Risk Review
description: Senior unblock decision for the first CLI/app crate rename slice
type: audit_session
date: 2026-06-10
status: approved
---

# R3A App Server Daemon Rename Risk Review

## Decision

Approve only `ontocode-app-server-daemon` -> `ontocode-app-server-daemon` as the next R3 implementation slice.

## Scope

- Rename package/lib/Bazel/import identity for `ontocode-rs/app-server-daemon`.
- Preserve all runtime behavior and public compatibility surfaces.
- Do not include `ontocode-app-server-test-client`, `codex-cli`, `ontocode-tui`, `ontocode-app-server`, `ontocode-app-server-client`, `ontocode-app-server-transport`, or protocol/generated crates in this slice.

## Evidence

- Reverse dependency inventory: `ontocode-app-server-daemon` is directly referenced only by `codex-cli` as a dev-dependency.
- OntoIndex impact: `LifecycleCommand` is LOW with zero upstream impacted nodes.
- OntoIndex impact: `BootstrapOptions` is LOW with two daemon-local impacted nodes.
- OntoIndex impact: `run_pid_update_loop` is LOW with one expected caller path through `ontocode-rs/cli/src/main.rs:cli_main`.
- OntoIndex impact: `PidBackend` is LOW with one test-only upstream caller, `update_loop_uses_hidden_app_server_subcommand`.

## Non-Goals

- Do not rename app-server wire methods.
- Do not rename generated protocol model names.
- Do not rename CLI command names or hidden subcommands.
- Do not change runtime socket behavior.
- Do not change update-loop behavior.
- Do not change remote-control behavior.
- Do not change managed install filenames.
- Do not change package-layout names.
- Do not change env/config semantics.
- Do not change telemetry or persisted state.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-daemon`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli remote_control_cmd`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale-reference search for `ontocode-app-server-daemon` and `codex_app_server_daemon`, excluding intentional compatibility strings if any.
- `git diff --check`
- OntoIndex `gn_verify_diff` or `ontoindex detect-changes` scoped to the R3A slice.

## Next Gate

After R3A is accepted, run a fresh senior review before approving any other R3 crate.
