---
name: Agentic S7 Delete Closure
description: Closure note for the `/agent` live-thread delete slice with scoped verification and dirty-worktree caveat
type: audit-session
date: 2026-06-24
status: closed
---

# Agentic S7 Delete Closure

Authority:
- `CLAUDE_CODE_AGENTIC_ENGINE_SOLUTIONS_TRACKING.md`
- `ADR_AGENT_SLASH_SUBAGENT_MANAGEMENT.md`

Scope closed:
- `AGENTIC-S7` only: add picker-local live-thread delete inside `/agent`.

Implemented shape:
- Each non-primary thread in `/agent` now also gets a dedicated `Delete ...` row.
- Selecting that row removes the thread from the current TUI session only.
- Inactive thread delete drops local listener/channel/side-thread/picker state only.
- If the deleted thread is currently displayed, the TUI switches back to the main thread first and then discards the deleted thread's local state.
- No app-server archive/delete API, no role-file mutation, no new registry, and no persistent history semantics were added.

Touched files:
- `ontocode-rs/tui/src/app_event.rs`
- `ontocode-rs/tui/src/app/session_lifecycle.rs`
- `ontocode-rs/tui/src/app/event_dispatch.rs`
- `ontocode-rs/tui/src/app/side.rs`
- `ontocode-rs/tui/src/app/tests.rs`
- `ontocode-rs/tui/src/snapshots/ontocode_tui__app__tests__agent_picker_rename_row.snap`

Verification:
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-tui -- open_agent_picker_exposes_rename_action_for_existing_side_threads open_agent_picker_exposes_delete_action_for_existing_side_threads rename_agent_picker_thread_label_updates_visible_metadata_only delete_agent_picker_thread_removes_inactive_local_state open_agent_picker_rename_row_snapshot agent_picker_item_name_snapshot`
- Blocked outside slice: full `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-tui` still fails because the dirty worktree already contains broad unrelated TUI snapshot drift.

Residual next step:
- Move to `AGENTIC-S8` repo-local definition copy without widening into profile registries or app-server persistence.
