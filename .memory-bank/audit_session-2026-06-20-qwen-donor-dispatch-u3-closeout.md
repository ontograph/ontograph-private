# Audit Session: U3 Closeout (QWN-036, QWN-127)

Date: 2026-06-20

## Summary

The Qwen donor blocked-row unblock ADR permitted narrow dispatch for QWN-036 (background shell cleanup) and QWN-127 (HTTP hook SSRF checks) only if there was a real gap in the existing `unified_exec` and `network-proxy` coverage. 

Verification proved both behaviors are already fully covered by the existing codebase. The task was closed without editing any code.

## Evidence

### QWN-036 (Unified Exec Lifecycle)
- **Gap analysis:** Existing `ontocode-rs/core/src/unified_exec/process_manager.rs` already defines `terminate_all_processes()` logic.
- **Hook-in:** `ontocode-rs/core/src/session/handlers.rs` correctly invokes `terminate_all_processes` in `shutdown_session_runtime()`.
- **Test coverage:** The file `ontocode-rs/core/src/unified_exec/mod_tests.rs` contains explicit regression coverage `completed_commands_do_not_persist_sessions` asserting that completed commands are purged from the map.

### QWN-127 (Hook SSRF Proxy Controls)
- **Gap analysis:** There is no distinct HTTP-hook runner in `ontocode-rs/hooks/src/engine`. All process-based hooks inherit the managed proxy controls when configured.
- **Test coverage:** `ontocode-rs/core/tests/suite/hooks.rs` checks that the hooks correctly inherit the `network_proxy` configuration inside danger-full-access sandboxes via `expect("expected runtime managed network proxy addresses")` and `test.session_configured.network_proxy`.

## Conclusion
Both donor requested slices are redundant. Tracking file `tmp/qwen-code-donor-dispatch-tracking.md` was updated to mark U3 as `blocked` (no code changes). `ontoindex detect-changes` confirms zero new modifications were made to `codex`.
