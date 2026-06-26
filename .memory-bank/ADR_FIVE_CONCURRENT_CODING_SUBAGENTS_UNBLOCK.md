---
name: Five Concurrent Coding Sub-Agents Unblock
description: Follow-up ADR for closing recursive spawn_agent leakage after senior review
type: adr
date: 2026-06-21
status: implemented
---

# ADR: Five Concurrent Coding Sub-Agents Unblock

## Context

Senior review of `ADR_FIVE_CONCURRENT_CODING_SUBAGENTS.md` found one remaining functional gap:

- `spawn_agent` is hidden only for code-mode `SubAgentSource::ThreadSpawn` sessions.
- Other code-mode sub-agent sources, such as `SubAgentSource::Other("agent_job:*")`, can still receive recursive collaboration tools because `collab_tools_enabled` returns true for all multi-agent v2 sessions.

The parent ADR requires ordinary coding sub-agents to not receive recursive agent tools.

## Decision

Broaden the existing owner-local tool-plan guard in `ontocode-rs/core/src/tools/spec_plan.rs`:

- hide `spawn_agent` for any code-mode `SessionSource::SubAgent(_)`;
- keep `spawn_agent` visible for root code-mode sessions;
- keep non-code sessions unchanged;
- do not add a new config key, scheduler, registry, database, or role system.

This closes the gap with the smallest existing-owner change.

## Required Tests

- Root code-mode still sees/registers `spawn_agent`.
- Code-mode `ThreadSpawn` sub-agent does not see/register `spawn_agent`.
- Code-mode `Other("agent_job:*")` sub-agent does not see/register `spawn_agent` while retaining ordinary non-recursive coordination tools such as `send_message` and `wait_agent`.

## Out Of Scope

- Existing unrelated `tool_search` changes in `spec_plan.rs` are not part of this ADR and must not be modified here.
- Runtime write-scope enforcement remains manager-owned per the parent ADR.
