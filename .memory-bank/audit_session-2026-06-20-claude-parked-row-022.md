name: Claude Parked Row 022 Review
desc: Row 022 stays parked because permission and plan-mode behavior is already owned and no single explanation-field gap was proven
type: audit_session
date: 2026-06-20

# Claude Parked Row 022 Review

## Decision

Row 022 stays parked. No promotion packet was written.

## Evidence

- Parked ADR row 022 says: `Add only missing explanation fields; avoid new permission engine.`
- Donor source row 022 says: `Preserve pre-plan permission mode and restore on exit` in `core/src/session/turn_context.rs`.
- The sources do not describe one clean existing-owner gap.
- OntoIndex reports `ontocode-rs/core/src/session/turn_context.rs` public API includes `permission_profile`, `build_per_turn_config`, `make_turn_context`, and related turn construction helpers; the file is 907 lines.
- Current `TurnContext` carries both `permission_profile` and `collaboration_mode`.
- `run_turn` derives plan mode from `turn_context.collaboration_mode.mode == ModeKind::Plan`, rather than weakening permissions as a side effect.
- Existing searches found permission prompt/denial coverage in request-permissions, hooks, network approval, guardian, and TUI approval paths.
- Existing searches also found plan/collaboration-mode restoration tests in session and TUI paths.

## Closure

The row is not reducible to one missing explanation field or one plan enter/exit failing test. Promoting it would risk reopening permission-engine scope from a parked row.
