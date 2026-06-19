---
name: R3C App Server Transport Rename Risk Review
description: Senior unblock decision for the third CLI/app crate rename slice
type: audit_session
date: 2026-06-10
status: approved
---

# R3C App Server Transport Rename Risk Review

## Decision

Approve only `ontocode-app-server-transport` -> `ontocode-app-server-transport` as the next R3 implementation slice.

## Scope

- Rename package/lib/Bazel/import identity for `ontocode-rs/app-server-transport`.
- Update direct dependent references required for compile/tests.
- Keep the slice separate from `ontocode-app-server`, `ontocode-app-server-client`, `codex-cli`, and `ontocode-tui`.

## Evidence

- Reverse dependency inventory: direct dependents are `ontocode-app-server` and `ontocode-app-server-daemon`; broader reach flows into app-server-client, CLI, TUI, and `ontocode-exec`.
- OntoIndex impact: `start_control_socket_acceptor` is CRITICAL through app-server and CLI/TUI runtime flows.
- OntoIndex impact: `start_websocket_acceptor`, `start_stdio_connection`, and `start_remote_control` are HIGH through app-server, CLI, and TUI runtime flows.
- OntoIndex impact: `app_server_control_socket_path` is CRITICAL through doctor, remote-control, TUI auto-connect, app-server-daemon, and CLI paths.
- OntoIndex impact: `policy_from_settings` is CRITICAL through app-server auth transport flows.
- OntoIndex impact: `RemoteControlHandle` is HIGH through remote-control and app-server runtime flows.

## Non-Goals

- Do not change socket path semantics.
- Do not change websocket, stdio, or unix-socket transport behavior.
- Do not change remote-control protocol or enrollment behavior.
- Do not change auth policy behavior or token validation.
- Do not change startup lock behavior.
- Do not change `AppServerTransport` parse behavior.
- Do not change CLI command behavior.
- Do not change telemetry semantics.
- Do not change env/config semantics.
- Do not change persisted state.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-transport`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-daemon`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli remote_control_cmd`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale-reference search for `ontocode-app-server-transport` and `codex_app_server_transport`, with intentional compatibility strings documented.
- `git diff --check`
- OntoIndex `gn_verify_diff` or `ontoindex detect-changes` scoped to the R3C slice.

## Next Gate

After R3C is accepted, run a fresh senior review before approving app-server, app-server-client, CLI, or TUI rename work.
