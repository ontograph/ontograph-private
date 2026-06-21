# ADR: Current Ontocode Sub-Agent Handling

- Status: accepted-current-state
- Date: 2026-06-21
- Scope: document how Ontocode currently exposes, spawns, configures, resumes, and tracks sub-agents

## Context

Ontocode already has a native sub-agent path. It should not grow a second task runtime, model router, permission engine, or agent registry.

OntoIndex was fresh at `d8ec11f538fb14941601332841ffd6dc1db734ac` for this review, with a dirty worktree. Evidence came from:

- `ontocode-rs/core/src/tools/spec_plan.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents_v2/*`
- `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`
- `ontocode-rs/core/src/thread_manager.rs`
- `ontocode-rs/core/src/agent/control.rs`
- `ontocode-rs/state/src/runtime/agent_jobs.rs`

## Decision

Treat the current sub-agent architecture as the owner for future sub-agent work.

Sub-agents are exposed as collaboration tools from `spec_plan.rs`, implemented by the multi-agent handlers, started through `AgentControl`/`ThreadManager`, and persisted or batch-tracked through the existing agent job state where applicable.

## Current Flow

1. Tool exposure is decided in `core/src/tools/spec_plan.rs`.
   - `add_collaboration_tools` registers multi-agent tools when collaboration features are enabled.
   - v1 tools are exposed under the `multi_agent_v1` namespace.
   - v2 tools are exposed as function tools through the multi-agent v2 handlers.
   - Tool exposure can be model-visible, deferred, or dispatch-only through the existing tool registry.

2. Tool schemas are built in `core/src/tools/handlers/multi_agents_spec.rs`.
   - `spawn_agent` accepts `message` or structured `items`.
   - Optional metadata includes `agent_type`, `model`, `reasoning_effort`, and `service_tier`.
   - Metadata can be hidden when the caller should not see agent type/model/reasoning knobs.
   - Available model guidance is generated from the active model catalog.

3. `spawn_agent` is handled in `core/src/tools/handlers/multi_agents/spawn.rs` and `multi_agents_v2/spawn.rs`.
   - Input is parsed with `parse_collab_input`.
   - The child depth is checked against `agent_max_depth`.
   - Spawn begin/end protocol events are emitted.
   - The child config starts from the parent turn's effective config.
   - Child runtime state is refreshed from the live turn before spawning.
   - The agent is started through `session.services.agent_control.spawn_agent_with_metadata`.

4. Runtime inheritance is centralized in `multi_agents_common.rs`.
   - Spawned agents inherit the current provider, model, reasoning defaults, developer instructions, compact prompt, approval policy, shell environment policy, sandbox executable, cwd, permission profile, and environment selections.
   - This avoids stale persisted config leaking into children.

5. Model and reasoning overrides are intentionally narrow.
   - If `model` is omitted, the child inherits the parent turn model.
   - If `model` is provided, it must exactly match an entry in `models_manager.list_models(RefreshStrategy::Offline)`.
   - If `reasoning_effort` is provided, it must be supported by the selected effective model.
   - Full-history forks reject `agent_type`, `model`, and `reasoning_effort` overrides because they inherit the parent agent identity and reasoning setup.

6. Service tier is validated against the effective child model.
   - Explicit `service_tier` requests are rejected if unsupported.
   - Otherwise the child uses the first supported tier from child config, explicit request, or parent tier.

7. Agent identity and lineage are encoded in `SessionSource::SubAgent`.
   - `thread_spawn_source` records parent thread id, depth, optional role, and optional task path.
   - Spawn results return the child thread id and nickname when available.

8. Existing agents are managed by collaboration tools.
   - `send_input`/`send_message` deliver follow-up input.
   - `followup_task` can trigger another turn for a target agent.
   - `wait_agent` waits for final status or mailbox updates.
   - `resume_agent` reopens a closed agent so it can receive messages and waits.
   - `close_agent` closes an agent through the same control path.

9. Agent jobs are the persisted batch/job layer.
   - `state/src/runtime/agent_jobs.rs` stores job rows and item rows in SQLite.
   - Jobs have pending/running/completed/error state transitions.
   - Items track assignment, attempts, result JSON, errors, completion, and reporting.
   - This is the right home for batch task state, not a new task table.

## Current Constraints

- Do not add a parallel sub-agent runtime.
- Do not bypass `AgentControl`, `ThreadManager`, or the multi-agent handlers.
- Do not add a separate model allowlist for sub-agents; use the model catalog.
- Do not loosen full-history fork inheritance.
- Do not add new public app-server or config surface for sub-agents without a separate ADR and compatibility tests.
- Keep model-visible context bounded; any future agent memory snapshot must use existing context-fragment caps.
- Do not add model-visible progress or memory fragments under this ADR.
- Do not add a new task, team, agent-definition, or source-precedence registry under this ADR.

## Extension Rules

Future sub-agent work should be one of these small owner-local changes:

- tool schema or visibility: `spec_plan.rs` and `multi_agents_spec.rs`
- spawn/resume/send/wait behavior: `multi_agents*` handlers
- runtime inheritance and override validation: `multi_agents_common.rs`
- thread lifecycle: `AgentControl` and `ThreadManager`
- persisted batch/task state: `state/src/runtime/agent_jobs.rs`
- TUI rendering only after runtime behavior is proven

## Donor-Inspired Requirements

These requirements come from `tmp/claude-code-main`, but must extend the current Ontocode owners above rather than copying Claude's runtime shape.

## Challenge Outcome

OntoIndex review keeps this ADR as a current-owner extension plan, not a donor-runtime port. The accepted implementation order is:

| Requirement | Decision | Challenge result |
| --- | --- | --- |
| R1 Agent Activity UI | Narrow | Start with a TUI read-only activity surface over existing collaboration events and `list_agents`. Mutating actions require the existing approval/confirmation path where applicable. Any app-server or public API exposure needs a separate ADR and compatibility tests. |
| R2 Stable Agent Visual Identity | Keep | TUI-local deterministic color is safe if it remains presentation-only, does not persist new protocol/state fields, and never becomes the only status signal. |
| R3 Agent Definition Visibility | Narrow | Show only role/config metadata already produced by the existing role/config path. Do not invent a Claude-style registry, source precedence model, or override resolver unless the current config stack already exposes that data. |
| R4 Bounded Agent Progress Summaries | Narrow | Limit to TUI/history summarization over existing events. Do not inject progress into model-visible context. If thread history reconstruction changes, use existing `ThreadHistoryBuilder`/app-server event handling and add schema/compatibility tests. |
| R5 Background Job UX | Keep narrow | Build only over `state/src/runtime/agent_jobs.rs` and `core/src/tools/handlers/agent_jobs.rs`. Do not add new task tables, public APIs, or alternate job state owners under this ADR. |
| R6 Optional Agent Memory Snapshot | Defer | Treat as out of scope until a separate memory/context ADR proves a failing workflow, redaction plan, storage owner, and hard context caps. |

Implementation must follow this order: R2 only if it falls out of R1 rendering, then R1 read-only, then R4 rendering caps, then R5 job status. R3 and R6 stay blocked unless their source-data and context-safety gaps are resolved in separate evidence-backed ADRs.

### R1. Agent Activity UI

Add an agent activity surface over existing collaboration events and `list_agents`.

Required behavior:

- Show agent nickname, role, model, status, and last task message.
- Group agents by active, waiting, completed, failed, and closed states.
- Start read-only; expose wait, send message, follow up, and close only through existing confirmed action paths.
- Reuse existing `CollabAgent*` events and TUI agent metadata.
- Add snapshot coverage for compact and expanded rendering.

Donor evidence:

- [AgentTool UI](../tmp/claude-code-main/src/tools/AgentTool/UI.tsx)
- [Agent display utilities](../tmp/claude-code-main/src/tools/AgentTool/agentDisplay.ts)
- [Agents command](../tmp/claude-code-main/src/commands/agents/agents.tsx)

Ontocode home:

- `ontocode-rs/tui/src/multi_agents*`
- `ontocode-rs/tui/src/chatwidget.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents_v2/list_agents.rs`

### R2. Stable Agent Visual Identity

Add deterministic, local-only visual identity for agent roles/types.

Required behavior:

- Assign a stable display color per agent role/type in the TUI.
- Do not persist color as protocol state unless another client needs it.
- Keep the default/general agent visually neutral.
- Never use color as the only status signal.

Donor evidence:

- [Agent color manager](../tmp/claude-code-main/src/tools/AgentTool/agentColorManager.ts)
- [AgentTool UI](../tmp/claude-code-main/src/tools/AgentTool/UI.tsx)

Ontocode home:

- `ontocode-rs/tui/src/multi_agents*`
- `ontocode-rs/tui/src/color.rs`

### R3. Agent Definition Visibility

Make configured sub-agent roles discoverable without adding a second registry.

Required behavior:

- Show role/type, effective model, inherited/configured state, and source only when the existing runtime/config path already exposes them.
- Group sources only by existing source data; do not add new source precedence just for display.
- Mark overridden definitions only when the current config owner already reports that relationship.
- Keep model display explicit: inherited vs configured.

Donor evidence:

- [Agent display utilities](../tmp/claude-code-main/src/tools/AgentTool/agentDisplay.ts)
- [Agent directory loader](../tmp/claude-code-main/src/tools/AgentTool/loadAgentsDir.ts)
- [Agents command](../tmp/claude-code-main/src/commands/agents/agents.tsx)

Ontocode home:

- existing role/config loading path
- `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`
- TUI command/dialog only after the runtime role source is proven

### R4. Bounded Agent Progress Summaries

Show sub-agent progress without flooding the transcript or model context.

Required behavior:

- Show only the most recent small number of progress items by default.
- Collapse repeated read/search/tool activity into counts.
- Keep detailed output available only through explicit expansion or existing job/item reads.
- Do not inject unbounded progress into model-visible context.
- Do not add any model-visible progress context under this ADR.

Donor evidence:

- [AgentTool UI progress handling](../tmp/claude-code-main/src/tools/AgentTool/UI.tsx)
- [Agent tool utilities](../tmp/claude-code-main/src/tools/AgentTool/agentToolUtils.ts)

Ontocode home:

- existing `CollabAgent*` protocol events
- `ontocode-rs/tui/src/history_cell*`
- `ontocode-rs/tui/src/multi_agents*`

### R5. Background Job UX Over Existing Agent Jobs

Expose batch/background state through `state/src/runtime/agent_jobs.rs`; do not add a new task database.

Required behavior:

- List jobs and job items with status, assigned thread, attempts, last error, and result availability.
- Read capped output/result details.
- Stop or cancel through the existing agent control/job transition path.
- Keep SQLite as the persisted job state owner.
- Do not add new task tables, job APIs, or alternate persistence owners under this ADR.

Donor evidence:

- [Task list tool](../tmp/claude-code-main/src/tools/TaskListTool/TaskListTool.ts)
- [Task get tool](../tmp/claude-code-main/src/tools/TaskGetTool/TaskGetTool.ts)
- [Task output tool](../tmp/claude-code-main/src/tools/TaskOutputTool/TaskOutputTool.tsx)
- [Task stop tool](../tmp/claude-code-main/src/tools/TaskStopTool/TaskStopTool.ts)
- [Tasks command](../tmp/claude-code-main/src/commands/tasks/tasks.tsx)

Ontocode home:

- `ontocode-rs/state/src/runtime/agent_jobs.rs`
- `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`
- TUI status/dialog layer

### R6. Optional Agent Memory Snapshot, Deferred

Rejected under this ADR. Reconsider only with a separate memory/context ADR after a concrete failing workflow proves it is needed.

Required behavior if accepted later:

- Store only bounded, redacted per-agent memory snapshots.
- Track snapshot timestamp and sync state.
- Require explicit initialize, replace, ignore, or mark-synced behavior.
- Use existing context fragment caps before anything becomes model-visible.

Donor evidence:

- [Agent memory snapshot](../tmp/claude-code-main/src/tools/AgentTool/agentMemorySnapshot.ts)
- [Agent memory](../tmp/claude-code-main/src/tools/AgentTool/agentMemory.ts)

Ontocode home:

- `ontocode-rs/core/src/context/*`
- `ontocode-rs/core/src/session/*`
- memory-bank/session context owners

## Rejected Donor Shapes

- No parallel Claude-style `AgentTool` runtime.
- No separate team abstraction before existing agent paths prove a gap.
- No remote-agent or browser/bridge behavior under this ADR.
- No worktree isolation under this ADR; that needs separate sandbox/worktree approval.
- No plugin marketplace or command UX as a prerequisite for sub-agent runtime improvements.
- No public app-server surface changes under this ADR.
- No model-visible progress summaries or agent memory snapshots under this ADR.
- No task/team/agent registry duplication under this ADR.

Skipped: donor-style team/task/plugin/bridge abstractions. Add them only when they extend these owners with a concrete failing test.
