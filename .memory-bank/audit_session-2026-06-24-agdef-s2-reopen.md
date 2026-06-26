---
name: AGDEF S2 Reopen
description: Senior decision to deliberately reopen the structured agent-definition field flow as the next slice
type: audit-session
date: 2026-06-24
status: recorded
---

# AGDEF S2 Reopen

## Decision

- `AGDEF-S2` is reopened deliberately as the next active slice.
- `AGENTIC-S2` is reclassified as the queued dependent follow-up behind `AGDEF-S2`.

## Why

- OntoIndex is still fresh, but no new code evidence naturally unblocks the remaining blocked items.
- The user explicitly requested reopening blocked work.
- The smallest defensible reopen is still the agent-definition field flow, not direct dispatch, profile/config writes, or broader `agent_jobs` UX.

## Accepted Boundary

- Keep writes inside the existing picker -> prompt -> `.codex/agents/<slug>.toml` path.
- Limit scope to optional fields that replace repeated hand-edits:
  - `model`
  - `model_reasoning_effort`
  - `service_tier`
  - `nickname_candidates`

## Explicit Non-Scope

- No slash-dispatch rewrites
- No runtime role application changes
- No app-server API
- No hot reload
- No registry or source-precedence work
- No arbitrary file-open editor plumbing
