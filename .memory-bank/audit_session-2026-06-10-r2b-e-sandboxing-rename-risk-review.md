# R2B-E Sandboxing Rename Risk Review

Date: 2026-06-10

## Candidate

- `codex-sandboxing` -> `ontocode-sandboxing`
- Scope is identity-only package/lib/Bazel/import rename.

## OntoIndex Evidence

- `SandboxManager.transform`: CRITICAL impact, 21 impacted nodes, 8 direct callers, 2 affected runtime processes, including core shell/unified exec and app-server command exec paths.
- `get_platform_sandbox`: CRITICAL impact, 23 impacted nodes, 10 modules, including apply-patch safety, sandbox tags, sandbox selection, exec-server sandbox requests, and core session/tool paths.
- `create_seatbelt_command_args`: CRITICAL impact through Seatbelt sandbox command generation, CLI debug sandbox, core exec, exec-server file-system sandboxing, and sandbox transform paths.
- `create_linux_sandbox_command_args_for_permission_profile`: CRITICAL impact, 23 impacted nodes, 3 affected processes, including CLI debug sandbox and core shell/unified exec paths.
- Direct reverse dependencies: app-server, CLI tests, core, TUI, `ontocode-arg0`, `ontocode-exec-server`, and `ontocode-linux-sandbox`.

## Allowed Change

- Rename Cargo package `codex-sandboxing` to `ontocode-sandboxing`.
- Rename Rust library crate `codex_sandboxing` to `ontocode_sandboxing`.
- Update workspace dependency keys, dependent manifests, Rust imports/usages, Bazel crate names, and lockfiles.

## Forbidden Change

- Do not change sandbox policy semantics, permission-profile transforms, platform sandbox selection, Seatbelt argument generation, Landlock/Linux sandbox argument generation, bubblewrap lookup/warnings, helper argv0 values, network policy, managed MITM CA behavior, sandbox env vars, public command names, protocol/schema surfaces, telemetry, env/config semantics, runtime layout, or persisted state.
- Do not rename helper binaries or compatibility aliases.

## Required Verification

- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-sandboxing`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli debug_sandbox`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox`
- Focused sandbox manager, policy transforms, Seatbelt, Landlock/Linux sandbox, bwrap warning/lookup, app-server command exec, core shell/unified exec, apply-patch sandbox safety, CLI debug sandbox, and exec-server sandboxed file-system coverage.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale package/lib reference search for `codex-sandboxing` and `codex_sandboxing`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.
