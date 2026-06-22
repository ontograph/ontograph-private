---
name: Oh My Pi Row 86 Coverage Closeout
description: Closure evidence for bounded compaction failure event tests
type: audit_session
date: 2026-06-19
status: closed
---

# Oh My Pi Row 86 Coverage Closeout

Authority:
- `.memory-bank/ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md` row 86
- `.memory-bank/OH_MY_PI_DONOR_KEEP_REFACTOR_MAP_PRE_JUNIOR_PROJECT_PLAN.md`

Decision:
- Row 86 is covered without new runtime or test code.
- Existing local pre-turn compaction context-window failure coverage exercises the bounded failure event path.

Evidence:
- `ontocode-rs/core/tests/suite/compact.rs::snapshot_request_shape_pre_turn_compaction_context_window_exceeded` submits a second turn that triggers pre-turn compaction, waits for `EventMsg::Error`, then waits for `TurnComplete`, and snapshots the failed compaction request shape.
- Verification passed with:
  `TMPDIR="$(mktemp -d /var/tmp/ontocode-core.XXXXXX)" CARGO_BUILD_JOBS=8 just test -p ontocode-core snapshot_request_shape_pre_turn_compaction_context_window_exceeded`

Remote note:
- `remote_manual_compact_failure_emits_task_error_event` and `auto_remote_compact_failure_stops_agent_loop` are not closure evidence for this row in the current tree: both currently fall through to an unmatched `/v1/responses` request after compact failure and fail before proving the intended remote compact error event.
- The remote compaction request-emission/stop-after-failure issue remains separate from row 86 and must not be folded into later context rows casually.
