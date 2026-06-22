name: Claude Parked Row 017 Review
desc: Row 017 stays parked because shell-adapter and notebook-edit sources mismatch and no deferred evidence promotes notebook tooling
type: audit_session
date: 2026-06-20

# Claude Parked Row 017 Review

## Decision

Row 017 stays parked. No promotion packet was written.

## Evidence

- Parked ADR row 017 says: `OS shell adapter polish is not a donor priority without failures.`
- Donor source row 017 says: `Add notebook-edit capability as a separately gated tool` in `core/src/tools`.
- The sources do not describe one clean existing-owner gap.
- Oh My Pi explicitly blocks notebook execution, virtual URI schemes, DAP, browser control, and persistent language workers; notebook-as-text remains the only acceptable notebook direction there.
- No fresh bug, user-facing regression, security/safety issue, or senior-approved product requirement was found during triage.
- OntoIndex reports `ontocode-rs/core/src/tools/spec_plan.rs` public API as `build_tool_router`, `search_tool_enabled`, and `tool_suggest_enabled`; the file is 996 lines.
- OntoIndex notebook search found Python SDK notebook smoke tests and code-mode cell/runtime tests, but not an approved notebook-edit tool owner in `core/src/tools`.

## Closure

The row remains deferred. Promoting it would require reopening notebook tool/protocol scope or shell adapter work without concrete failure evidence.
