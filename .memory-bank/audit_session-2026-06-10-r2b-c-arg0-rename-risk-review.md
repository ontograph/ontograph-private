# R2B-C Arg0 Rename Risk Review

Date: 2026-06-10

## Candidate

- `codex-arg0` -> `ontocode-arg0`
- Scope is identity-only package/lib/Bazel/import rename.

## OntoIndex Evidence

- `arg0_dispatch_or_else`: HIGH impact, 6 impacted direct callers across CLI, TUI, app-server, MCP server, exec, and thread-manager sample startup.
- `arg0_dispatch`: HIGH impact, 8 impacted nodes through test-binary dispatch and startup dispatch.
- `linux_sandbox_exe_path`: HIGH impact, 8 impacted nodes through arg0 guarded startup and Linux sandbox helper resolution.
- Direct reverse dependencies: app-server, app-server-client, CLI, core-api, MCP server, test-binary-support, TUI, core test support, and `ontocode-exec`.

## Allowed Change

- Rename Cargo package `codex-arg0` to `ontocode-arg0`.
- Rename Rust library crate `codex_arg0` to `ontocode_arg0`.
- Update workspace dependency keys, crate imports, Bazel crate names, and lockfiles.

## Forbidden Change

- Do not rename public binaries, `argv[0]` accepted helper aliases, runtime helper file names, `codex-package.json`, `codex-path`, `codex-resources`, package-layout behavior, startup dispatch semantics, dotenv filtering, shell escalation dispatch, apply-patch dispatch aliases, Linux sandbox helper dispatch, telemetry, env/config semantics, protocol/schema, or persisted state.

## Required Verification

- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-api`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-test-binary-support`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`
- Focused startup/arg0/helper alias tests covering dispatch, package path preservation, apply-patch alias, shell escalation, and Linux sandbox alias compatibility.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale package/lib reference search for `codex-arg0` and `codex_arg0`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.
