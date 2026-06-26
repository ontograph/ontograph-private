# Audit Session: Agentic S10 Delete Definition Unblock

Date: 2026-06-24

## Scope

Senior unblock pass after `AGENTIC-S9` repo-local definition rename closure.

## OntoIndex Evidence

- `gn_ensure_fresh` reported the index current at `2e72a6d25e147f0619863e7721107b6f11a87fc2`, with dirty-worktree medium-confidence caveat.
- `impact(App.open_agent_picker)` remains `MEDIUM`, with direct impact limited to existing picker tests.
- Current code already has repo-local create, proposal, copy, and rename paths under the existing picker/prompt/app-event/scaffold owner.

## Decision

Promote `AGENTIC-S10`: repo-local definition delete.

Accepted boundary:

- expose delete only for role definitions loaded from repo-root `.codex/agents/*.toml`
- require one confirmation path before file removal
- remove only the targeted file
- keep reload semantics unchanged: reopen `/agent` or restart
- do not add archive/trash semantics, app-server API, runtime mutation, source-precedence UI, or a second registry

Still blocked:

- `AGENTIC-S3` deterministic direct dispatch, because no thin-wrapper reliability failure was shown
- `AGENTIC-S4` profile/config/model persistence, because source precedence and migration ownership are still not proven
- `AGENTIC-S5` extra job UX, because prior `SUBAGENT-R5` already closed the existing `agent_jobs` status/export gap
- donor-style generation preview, history mining, app-server APIs, runtime mutation, and cross-scope save/copy flows
