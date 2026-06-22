# R2B-D Exec Server Rename Closure

Date: 2026-06-10

## Scope

- Accepted `ontocode-exec-server` -> `ontocode-exec-server` as an identity-only package/lib/Bazel/import rename.
- Preserved public binaries, remote/local exec protocol names, protobuf package/message names, `codex.exec_server.*` wire identifiers, environment URL semantics, runtime path behavior, local process behavior, sandboxed file-system behavior, remote relay behavior, app-server environment APIs, telemetry, env/config semantics, persisted state, and generated schema/protocol surfaces.

## Verification

- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-api` then `--no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-plugins`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-skills`
- `CARGO_BUILD_JOBS=8 just test -p codex-mcp`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-rmcp-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- Focused exec-server environment URL, local process, remote client, relay, sandboxed file-system, app-server environment, command exec, and RMCP/MCP stdio coverage.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-exec-server`
- Active stale package/lib reference search for `ontocode-exec-server` / `codex_exec_server`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.

## Notes

- Remaining `ontocode-exec-server` references are intentional runtime/test/logging compatibility strings.
- `codex.exec_server.*` wire/protobuf identifiers remain unchanged by design.
- Next R2B candidate is `codex-sandboxing` and requires fresh OntoIndex impact/risk review before dispatch.
