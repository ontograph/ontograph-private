---
name: Current Sub-Agent Handling Solutions
description: Senior unblock plan for accepted slices from ADR_CURRENT_SUB_AGENT_HANDLING
type: adr
date: 2026-06-21
status: accepted
---

# ADR: Current Sub-Agent Handling Solutions

Authority:
- `ADR_CURRENT_SUB_AGENT_HANDLING.md`
- `ADR_CURRENT_SUB_AGENT_HANDLING_TRACKING.md`

## Decision

Unblock only the owner-local slices already accepted by the current sub-agent ADR. Do not import Claude's runtime, registry, memory, task database, app-server API, or source-precedence model.

## Accepted Slices

| Slice | ADR req | Solution | Owner | Stop condition |
| --- | --- | --- | --- | --- |
| `SUBAGENT-R2` | R2 | Add deterministic, TUI-local agent role/type color if the existing picker/status rendering lacks it. Keep default/general neutral and status visible without color. | `ontocode-rs/tui/src/multi_agents.rs`, `ontocode-rs/tui/src/color.rs`, existing TUI snapshots/tests | Protocol fields, persisted color state, or color-only status. |
| `SUBAGENT-R1` | R1 | Add or verify a read-only status-bearing flat activity surface over existing picker metadata and `list_agents`: nickname, role, model, status, and last task message. Grouped active/waiting/completed/failed/closed UI remains out of scope until a concrete UI need is proven. Mutating actions stay out unless existing confirmed paths already own them. | `ontocode-rs/tui/src/multi_agents.rs`, `ontocode-rs/tui/src/app/session_lifecycle.rs`, `ontocode-rs/core/src/tools/handlers/multi_agents_v2/list_agents.rs` | New app-server API, new runtime, grouped activity UI without product evidence, or unconfirmed close/send/followup UI. |
| `SUBAGENT-R4` | R4 | Add bounded TUI/history progress rendering if existing `CollabAgent*` rendering is unbounded. Collapse repeated tool/read/search activity into capped display only. | Existing `CollabAgent*` event rendering and `ontocode-rs/tui/src/history_cell*` / `multi_agents.rs` | Model-visible progress context or transcript flooding. |
| `SUBAGENT-R5` | R5 | Add or verify job export/status output over existing `agent_jobs` state and handler output: status, assigned thread, attempts, last error, capped result/output availability. TUI/dialog job UX remains out of scope until a concrete UI workflow is proven. | `ontocode-rs/state/src/runtime/agent_jobs.rs`, `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`, existing TUI status/dialog layer only if later evidence requires it | New task tables, public APIs, alternate job state owners, or speculative TUI/dialog job UX. |

## Deferred Slices

| Slice | ADR req | Status | Reason |
| --- | --- | --- | --- |
| `SUBAGENT-R3` | R3 | blocked | Existing runtime/config source data must be proven first. Do not invent an agent-definition registry or source precedence layer. |
| `SUBAGENT-R6` | R6 | blocked | Requires a separate memory/context ADR with redaction, storage owner, and hard model-context caps. |

## Verification

Each active slice must:

- update `ADR_CURRENT_SUB_AGENT_HANDLING_TRACKING.md` before starting and after closure;
- run OntoIndex context/impact before editing production symbols;
- keep changes inside its listed owner;
- prefer proving existing behavior with tests over adding code;
- run scoped crate tests and `CARGO_BUILD_JOBS=8 just fmt` after Rust edits;
- refresh/check OntoIndex after closure.
