# R3E App Server Rename Risk Review

Date: 2026-06-10

## Candidate

- `ontocode-app-server` -> `ontocode-app-server`.
- Scope approved only for package/lib/Bazel/import identity changes.

## OntoIndex Evidence

- `run_main`: LOW, 1 upstream impacted node.
- `run_main_with_transport_options`: HIGH, 4 impacted nodes through `cli_main`, CLI main, TUI main, and remote-control modules.
- `AppServerRuntimeOptions`: LOW, 2 upstream impacted nodes.
- OntoIndex repo path: `/opt/demodb/_workfolder/ontocode`.

## Direct Inventory

- Active package/lib refs include workspace metadata, app-server package metadata, app-server tests, and CLI imports.
- Intentional compatibility strings include JWT audience values `ontocode-app-server`.
- Protocol package refs such as `ontocode-app-server-protocol` are not in this scope.

## Guardrails

- Do not change `ontocode-app-server` or `ontocode-app-server-test-notify-capture` binary names in this slice.
- Do not change app-server wire methods, protocol/generated model names, JWT audience strings, socket paths, websocket/stdio/unix-socket behavior, remote-control behavior, config warning behavior, skills warning behavior, CLI/TUI behavior, telemetry semantics, env/config semantics, or persisted state.
- Do not rename `ontocode-app-server-protocol` in this slice.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli remote_control_cmd`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale active reference search for `ontocode-app-server|codex_app_server`, excluding intentional binaries, protocol/client/daemon/transport/test-client package names, docs/comments, and JWT audience strings.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`

## Decision

- Approved as R3E, one slice only.
