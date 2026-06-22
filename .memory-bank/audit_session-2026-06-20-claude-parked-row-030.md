name: Claude Parked Row 030 Review
desc: Row 030 stays parked because batch approval plan-mode UX still lacks fresh product demand or defect evidence
type: audit_session
date: 2026-06-20

# Claude Parked Row 030 Review

## Decision

Row 030 remains parked. No promotion packet.

## Evidence

- Parked ADR row 030 says multi-profile permission UX needs product demand first.
- Donor row 030 asks for batch approval plan mode for grouped changes in `core/src/session`.
- Duplicate gate found no Gemini or Oh My Pi reopen signal.
- OntoIndex reports `ontocode-rs/core/src/session/turn.rs` is the hot turn/session owner with `run_turn`, `build_prompt`, and `built_tools`; the file is 2252 lines.
- Worker review identified `core/src/session/handlers.rs:thread_settings_update` as live session-settings ownership reached from `update_thread_settings` and `user_input_or_turn_inner`.
- Current approval/permission/plan-mode tests cover nearby behavior; no single owner-local failing test gap was proven.

## Closure

DEFER promotion requires a real bug, regression, security/safety issue, or senior-approved product requirement. None was found, so grouped approval UX remains parked.
