---
name: Agentic S6 Rename Closure
description: Closure note for the `/agent` live-thread rename slice with scoped verification and dirty-worktree caveat
type: audit-session
date: 2026-06-24
status: closed
---

# Agentic S6 Rename Closure

Authority:
- `CLAUDE_CODE_AGENTIC_ENGINE_SOLUTIONS_TRACKING.md`
- `ADR_AGENT_SLASH_SUBAGENT_MANAGEMENT.md`

Scope closed:
- `AGENTIC-S6` only: add picker-local live-thread rename inside `/agent`.

Implemented shape:
- `/agent` still lists live threads as selectable transcript targets.
- Each non-primary live thread now also gets a dedicated `Rename ...` row in the picker.
- Selecting that row opens a picker-owned prompt prefilled from the existing nickname.
- Submission updates only the cached visible label for the current TUI session and preserves cached role plus closed/open state.
- No app-server mutation, no `.codex/agents/*.toml` writes, no slash-dispatch expansion, and no new registry/state owner were added.

Touched files:
- `ontocode-rs/tui/src/app_event.rs`
- `ontocode-rs/tui/src/chatwidget/interaction.rs`
- `ontocode-rs/tui/src/app/session_lifecycle.rs`
- `ontocode-rs/tui/src/app/event_dispatch.rs`
- `ontocode-rs/tui/src/app/tests.rs`
- `ontocode-rs/tui/src/snapshots/ontocode_tui__app__tests__agent_picker_item_name.snap`
- `ontocode-rs/tui/src/snapshots/ontocode_tui__app__tests__agent_picker_rename_row.snap`

Verification:
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-tui -- open_agent_picker_exposes_rename_action_for_existing_side_threads rename_agent_picker_thread_label_updates_visible_metadata_only open_agent_picker_rename_row_snapshot agent_picker_item_name_snapshot`
- Blocked outside slice: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-tui` still fails because the dirty worktree already contains broad unrelated snapshot drift across many existing TUI suites.

Residual next step:
- Move to `AGENTIC-S7` live-thread delete without widening scope beyond picker-owned current-session visibility.
