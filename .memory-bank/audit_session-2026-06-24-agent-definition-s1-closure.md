---
name: Agent Definition S1 Closure
description: Closure note for the picker-owned repo-local agent definition scaffold flow
type: audit-session
date: 2026-06-24
status: accepted
---

# Agent Definition S1 Closure

Authority:
- `CLAUDE_CODE_AGENT_DEFINITION_UI_SOLUTIONS_REVIEW.md`
- `CLAUDE_CODE_AGENT_DEFINITION_UI_SOLUTIONS_TRACKING.md`

## Decision

`AGDEF-S1` is complete.

The implemented scope stays narrow:
- the existing `/agent` picker now includes `Create agent definition`
- selecting it opens a minimal name prompt
- submitting the prompt writes `.codex/agents/<slug>.toml` at the repo root when a git root exists, otherwise under the current cwd
- the scaffold contains only `name` and `developer_instructions`

The slice intentionally does not add:
- slash-dispatch changes
- arbitrary file-open editor plumbing
- hot reload
- runtime role mutation
- a structured field wizard
- dual-scope repo/user creation

## Verification

Exact checks passed:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui open_agent_picker_shows_configured_agent_roles_when_no_threads_exist open_agent_picker_allows_create_action_when_no_threads_exist create_agent_definition_scaffold_writes_repo_local_role_file`

Covered behaviors:
- the picker still renders configured roles and now shows the create action
- the create action is selectable even with no existing threads
- scaffold writes land in repo-local `.codex/agents/` rather than a nested cwd when a git root is present

## Residual Risk

- Reload remains explicit. The new role is not hot-loaded into the current session.
- OntoIndex `gn_verify_diff` stayed noisy because the worktree already had broad unrelated changes; local diff inspection was used to confirm the intended write set.

## Follow-up

- `AGDEF-S2` remains pending only if the scaffold-first flow proves insufficient and a structured optional-field wizard is still warranted.
