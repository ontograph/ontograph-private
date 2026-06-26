---
name: Agentic Proposals Promoted To Development
date: 2026-06-24
type: audit-session
status: accepted
---

# Agentic Proposals Promoted To Development

Authority:
- `ADR_AGENT_SLASH_SUBAGENT_MANAGEMENT.md`
- `CLAUDE_CODE_AGENTIC_ENGINE_SOLUTIONS_TRACKING.md`

## Decision

Explicit user demand is now sufficient to move three `/agent` proposals out of proposal-only ADR text and into the development queue:

- `AGENTIC-S6`: live-thread rename
- `AGENTIC-S7`: live-thread delete
- `AGENTIC-S8`: repo-local definition copy

## Why These Three

- they stay inside current accepted owners
- they do not require direct slash-runtime mutation, app-server APIs, or new registries
- rename/delete on live threads reuse the current picker/thread-metadata surface
- copy reuses the existing repo-local role-file writer path

## What Did Not Move

- `AGENTIC-S2` Stage 4A `Create from proposal` remains blocked behind `AGDEF-S2`
- `AGENTIC-S3/S4/S5` remain blocked by their earlier architecture gates
- broader Stage 4 items such as history-mining, structured field persistence, and profile/registry flows remain proposal-only
