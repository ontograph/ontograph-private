# R2B-D Exec Server Rename Risk Review

Date: 2026-06-10

## Candidate

- `ontocode-exec-server` -> `ontocode-exec-server`
- Scope is identity-only package/lib/Bazel/import rename.

## OntoIndex Evidence

- `ExecServerClient.exec`: LOW impact in the current graph.
- `normalize_exec_server_url`: CRITICAL partial impact, 11 impacted nodes, 3 direct callers, 5 modules, including environment selection and app-server environment request processors.
- Direct reverse dependencies include app-server, app-server-client, apply-patch tests, arg0, CLI tests, core tests, core-api, core plugins, core skills, MCP, MCP server, RMCP client, TUI, core test support, and plugin utility paths.
- `codex-sandboxing::SandboxManager.transform` remains CRITICAL and is deferred until after this slice.

## Allowed Change

- Rename Cargo package `ontocode-exec-server` to `ontocode-exec-server`.
- Rename Rust library crate `codex_exec_server` to `ontocode_exec_server`.
- Update workspace dependency keys, dependent manifests, Rust imports/usages, Bazel crate names, protobuf build references if package-name-only, and lockfiles.

## Forbidden Change

- Do not rename public binaries, remote/local exec protocol names, protobuf package/message names, `codex.exec_server.*` wire identifiers, environment URL semantics, runtime path behavior, local process behavior, sandboxed file-system behavior, remote relay behavior, app-server environment APIs, telemetry, env/config semantics, persisted state, or generated schema/protocol surfaces.
- Do not include `codex-sandboxing` rename in this slice.

## Required Verification

- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-api`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-plugins`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-skills`
- `CARGO_BUILD_JOBS=8 just test -p codex-mcp`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-rmcp-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- Focused exec-server environment URL, local process, remote client, relay, sandboxed file-system, app-server environment, command exec, and RMCP/MCP stdio coverage where available.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale package/lib reference search for `ontocode-exec-server` and `codex_exec_server`; wire/protobuf compatibility strings may remain if intentional.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.
