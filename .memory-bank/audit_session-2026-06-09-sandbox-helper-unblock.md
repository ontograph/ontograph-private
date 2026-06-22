# Audit Session: Sandbox Helper Runtime Unblock

Date: 2026-06-09

## Scope

- Unblocked the R1B dependent verification blocker in `ONTOCODE_INTERNAL_CRATE_RENAME_TRACKING.md`.
- Targeted the stale Linux sandbox helper alias path after Ontocode hard-cutover work.

## Changes

- `arg0` now installs canonical `ontocode-linux-sandbox` and `ontocode-execve-wrapper` aliases while still accepting legacy `codex-linux-sandbox` arg0 dispatch.
- Core Linux sandbox spawning preserves a real helper alias path when present and falls back to `ontocode-linux-sandbox`.
- Sandboxing manager fs-helper requests now preserve current or legacy helper paths and fall back to `ontocode-linux-sandbox`.
- Stale apply-patch test expectation was updated to the canonical Ontocode helper alias.
- Empty local `/tmp/.codex` and `/tmp/.git` test contamination was removed from the environment.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p codex-arg0`
- `CARGO_BUILD_JOBS=8 just test -p codex-sandboxing`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server suite::v2::command_exec::command_exec_accepts_permission_profile suite::v2::command_exec::command_exec_without_process_id_keeps_buffered_compatibility -- --exact`
- `CARGO_BUILD_JOBS=8 just test -p codex-core suite::apply_patch_cli::apply_patch_cli_uses_ontocode_linux_sandbox_helper_alias -- --exact`
- `CARGO_BUILD_JOBS=8 just test -p codex-core suite::apply_patch_cli::apply_patch_cli_preserves_existing_hard_link_outside_workspace -- --exact`
- `CARGO_BUILD_JOBS=8 just test -p codex-core suite::unified_exec::unified_exec_network_denial_emits_failed_background_end_event -- --exact`

## Residuals

- `CARGO_BUILD_JOBS=8 just test -p codex-core` now runs 2648 tests with 2638 passed, 10 failed, and 14 skipped.
- Residual failures are not the sandbox-helper blocker: 9 are code-mode expectation/timeout failures and 1 is `shell_snapshot::tests::snapshot_shell_does_not_inherit_stdin`.
- Next decision: triage these residuals or explicitly accept reduced verification before dispatching the next exact internal crate rename slice.
