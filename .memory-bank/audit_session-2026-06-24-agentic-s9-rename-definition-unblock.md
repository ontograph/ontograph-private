# Audit Session: Agentic S9 Rename Definition Unblock

Date: 2026-06-24

## Scope

Senior unblock pass over blocked `/agent` tasks after `Stage 4A0` valid scaffold repair.

## OntoIndex Evidence

- `gn_ensure_fresh` reported the index current at `2e72a6d25e147f0619863e7721107b6f11a87fc2`, with dirty-worktree medium-confidence caveat.
- `impact(App.open_agent_picker)` remains `MEDIUM`, with direct impact limited to existing picker tests.
- Current code already has repo-local role-file create/proposal/copy paths under `ontocode-rs/tui/src/app/session_lifecycle.rs`, `ontocode-rs/tui/src/chatwidget/interaction.rs`, `ontocode-rs/tui/src/app_event.rs`, and `ontocode-rs/tui/src/app/event_dispatch.rs`.

## Decision

Promote `AGENTIC-S9`: repo-local definition rename.

Accepted boundary:

- expose `Rename definition` only for loaded repo-root `.codex/agents/*.toml` definitions
- move the file to the new slug
- update only the internal `name = ...` field
- reject destination collisions before write
- keep reload semantics unchanged: reopen `/agent` or restart

Still blocked:

- `AGENTIC-S3` deterministic direct dispatch, because no thin-wrapper reliability failure was shown
- `AGENTIC-S4` profile/config registry work, because source precedence and migration ownership are still not proven
- `AGENTIC-S5` extra job UX, because prior `SUBAGENT-R5` already closed the existing `agent_jobs` status/export gap
- repo-local delete, model persistence, donor-style generated prompt preview, history mining, app-server APIs, and runtime mutation
