---
name: Agentic ADR Expanded Followups Loop
date: 2026-06-24
type: audit-session
status: closed
---

# Agentic ADR Expanded Followups Loop

Authority:
- `ADR_AGENT_SLASH_SUBAGENT_MANAGEMENT.md`
- `CLAUDE_CODE_AGENTIC_ENGINE_SOLUTIONS_TRACKING.md`
- `CLAUDE_CODE_AGENT_DEFINITION_UI_SOLUTIONS_TRACKING.md`

## Scope

- review only
- no implementation dispatch
- decide whether the expanded Stage 4 `/agent` proposals create a new active slice

## Outcome

- no new slice promoted
- `AGENTIC-S2` stays pending
- `AGENTIC-S3`, `AGENTIC-S4`, and `AGENTIC-S5` stay blocked

## Why

- `AGENTIC-S1` already delivered the approved thin `/agent` wrapper path
- the only plausible next slice is Stage 4A `Create from proposal`, but that path reuses the separate agent-definition owner and is still gated by `AGDEF-S2`
- rename, delete, copy, history review, and broader picker management remain proposal-only because they either mix live-thread and repo-local definition semantics or require destructive/history-selection flows not yet proven
- create-time model setup remains bounded by runtime schema visibility and should not become a separate active slice when the current schema hides model override fields

## Guardrails Reaffirmed

- do not widen into deterministic direct dispatch without a concrete thin-wrapper failure
- do not reopen app-server/public API, hot reload, new registries, or runtime mutation
- do not combine live-thread actions and repo-local definition actions into one ambiguous UX
- do not promote structured-field persistence or history-mining before scaffold-first demand exists
