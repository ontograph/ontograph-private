name: Claude Parked Row 016 Review
desc: Row 016 stays parked because no concrete Windows shell bug evidence promotes the deferred PowerShell capability idea
type: audit_session
date: 2026-06-20

# Claude Parked Row 016 Review

## Decision

Row 016 stays parked. No promotion packet was written.

## Evidence

- Parked ADR row 016 says: `Windows-specific command behavior needs concrete bug evidence.`
- Donor source row 016 says: `Add PowerShell tool exposure behind shell capability detection` in `exec-server` / `core/src/shell.rs`.
- No fresh bug, user-facing regression, security/safety issue, or senior-approved product requirement was found during triage.
- OntoIndex reports `ontocode-rs/core/src/shell.rs` public API as `name`, `derive_exec_args`, `shell_snapshot`, `empty_shell_snapshot_receiver`, `get_shell_by_model_provided_path`, `get_shell`, and `default_user_shell`; the file is 410 lines.
- Current shell code already models `PowerShell`, detects `pwsh` and `powershell`, uses PowerShell as the Windows default before falling back to `cmd.exe`, and derives PowerShell exec args with `-NoProfile` when login shell mode is disabled.
- OntoIndex impact for `get_powershell_shell` is HIGH across 3 modules through `get_shell`, session construction, and shell handlers.
- Existing `shell_tests.rs` has Windows-gated PowerShell coverage for default shell detection and `get_shell(ShellType::PowerShell, None)`.

## Closure

The row remains a deferred Windows-specific enhancement. Without concrete failure evidence, it is not dispatchable from this parked-row plan.
