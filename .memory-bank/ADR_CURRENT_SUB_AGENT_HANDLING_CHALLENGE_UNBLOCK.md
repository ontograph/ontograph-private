---
name: Current Sub-Agent Handling Challenge Unblock
description: Senior challenge closure for ADR_CURRENT_SUB_AGENT_HANDLING_SOLUTIONS wording versus implemented evidence
type: adr
date: 2026-06-21
status: accepted
---

# ADR: Current Sub-Agent Handling Challenge Unblock

Authority:
- `ADR_CURRENT_SUB_AGENT_HANDLING.md`
- `ADR_CURRENT_SUB_AGENT_HANDLING_SOLUTIONS.md`
- `ADR_CURRENT_SUB_AGENT_HANDLING_TRACKING.md`

## Decision

Close the challenge findings as documentation-scope corrections. Do not add new runtime grouping UI or TUI job dialogs under this ADR.

## Accepted Corrections

| Finding | Decision | Reason |
| --- | --- | --- |
| R1 claimed grouped activity UI | Narrow to status-bearing flat activity output through existing picker metadata and `list_agents`. | Current code proves `agent_role`, `model`, status, and last task message in a flat list. Grouping would be new UI behavior without fresh product evidence. |
| R5 name implied broad background-job UX | Narrow to existing job export/status output. | Current code proves assigned thread id and capped result JSON through existing agent-job handler output. New TUI/dialog UX would be speculative. |

## Stop Conditions

- Do not add grouped active/waiting/completed/failed/closed UI without a separate UI requirement and snapshot plan.
- Do not add job TUI/dialogs, public APIs, or new task tables without a separate ADR.
- Keep R3 and R6 blocked as before.
