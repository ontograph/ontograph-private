# R2D Windows Sandbox Rename Risk Review

Date: 2026-06-10

Scope:
- Approve only `ontocode-windows-sandbox` -> `ontocode-windows-sandbox` package/lib/Bazel/import identity rename.
- Preserve helper executable names, setup helper names, command-runner names, IPC protocol, sandbox setup behavior, token/ACL/WFP behavior, logging paths, app-server/TUI/CLI/core public behavior, env/config semantics, telemetry, runtime layout, and persisted state.

OntoIndex impact:
- `run_windows_sandbox_capture_with_filesystem_overrides`: LOW, 4 impacted nodes, direct callers in core exec and wrapper capture path.
- `current_log_file_path_for_codex_home`: LOW, 3 impacted nodes, direct app-server feedback and TUI feedback callers.
- `spawn_windows_sandbox_session_elevated_for_permission_profile` in `windows-sandbox-rs/src/unified_exec/mod.rs`: LOW.
- `spawn_windows_sandbox_session_legacy` in `windows-sandbox-rs/src/unified_exec/mod.rs`: LOW.
- `ResolvedWindowsSandboxPermissions`: LOW by explicit UID.
- `run_setup_refresh_with_extra_read_roots` in `windows-sandbox-rs/src/setup.rs`: LOW.
- `sandbox_setup_is_complete` in `windows-sandbox-rs/src/identity.rs`: LOW.
- Batch union: MEDIUM due package surface breadth.

Direct Cargo dependents:
- `ontocode-app-server`
- `codex-cli`
- `codex-core`
- `ontocode-tui`

Required verification:
- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- Focused Windows sandbox setup/unified-exec/logging/debug-sandbox tests where available.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale package/lib reference search for `ontocode-windows-sandbox` and `codex_windows_sandbox`, excluding intentional helper executable/runtime compatibility strings.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.

Decision:
- Approved as R2D, identity-only.
- Do not proceed to R2B runtime path/package-layout crates until R2D is accepted.
