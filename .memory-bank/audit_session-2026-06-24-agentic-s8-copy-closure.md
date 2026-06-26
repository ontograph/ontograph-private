---
name: Agentic S8 Copy Closure
description: Closure note for the repo-local agent-definition copy slice in `/agent`
type: audit-session
date: 2026-06-24
status: recorded
---

# Agentic S8 Copy Closure

## Scope

- Authority: `CLAUDE_CODE_AGENTIC_ENGINE_SOLUTIONS_TRACKING.md`
- Slice: `AGENTIC-S8`
- Accepted boundary: reuse the existing picker, prompt, app-event, and scaffold-writer path only

## Implementation

- `/agent` now exposes copy rows for repo-root standalone role definitions already loaded from `.codex/agents/*.toml`.
- Selecting copy opens a prompt seeded with the current role name.
- The copy path writes a new repo-local `.codex/agents/<slug>.toml` file.
- The copied file preserves the source content and updates only:
  - destination slug/path
  - internal `name = "..."` field

## Non-Scope Preserved

- No direct dispatch
- No app-server API
- No role registry or source-precedence UX
- No hot reload
- No runtime role mutation
- No history-mining or proposal generation

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui -- open_agent_picker_exposes_copy_action_for_repo_local_role_definition copy_agent_definition_scaffold_duplicates_repo_local_role_file open_agent_picker_copy_role_row_snapshot`

## Residual Risk

- Broader `ontocode-tui` snapshot drift in the dirty worktree remains outside this slice and was not absorbed into `AGENTIC-S8`.
