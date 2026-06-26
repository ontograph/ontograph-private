---
name: Five Concurrent Coding Sub-Agents
description: ADR for supporting five simultaneous coding sub-agents through the existing multi-agent v2 runtime
type: adr
date: 2026-06-21
status: implemented
last_review: 2026-06-21
review_outcome: implemented
---

# ADR: Five Concurrent Coding Sub-Agents

## Context

Manager dispatch tried to start five coding workers. Only two new workers spawned because older completed child agents still occupied active thread slots until explicitly closed.

The goal is five simultaneous direct coding sub-agents, not unlimited delegation, nested teams, background scheduling, or restart-surviving queues.

## Decision

Use the existing multi-agent v2 cap and close path. Do not add a scheduler, SQLite task table, worker-pool runtime, or default worktree isolation.

Stable implementation decision:

1. `features.multi_agent_v2.max_concurrent_threads_per_session` remains the capacity owner.
2. Configure the session cap to `6` for five coding children. `Config::effective_agent_max_threads` applies `saturating_sub(1)` to the session cap, producing `agent_max_threads = 5`. Root is registered in `agent_tree` but NOT counted in `total_count` — the subtraction handles this implicitly. Do not describe this as "root counts against the cap."
3. `Config::effective_agent_max_threads` remains the only conversion from session cap to child-agent cap.
4. Completed, failed, or cancelled children release slots only after the manager records the result and calls `close_agent`.
5. `list_agents`, `wait_agent`, and parent completion forwarding remain the status/result surface.
6. Write-scope conflict checks stay in manager tracking for phase one.
7. Child agents must not receive `spawn_agent` or recursive agent tools. `non_code_mode_only` (default `true`) restricts to `DirectModelOnly` but still includes the tool. A separate config gate or tool plan filter is required to REMOVE `spawn_agent` from coding sub-agent tool catalogs.

This is the smallest stable and performant path: O(active children) in-memory accounting already exists, result delivery already exists, and no new persistence or scheduling layer is needed.

## OntoIndex Review (2026-06-21)

### P0: Default cap permits only 3 children

[config/mod.rs:189](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/config/mod.rs:189): `DEFAULT_MULTI_AGENT_V2_MAX_CONCURRENT_THREADS_PER_SESSION = 4`. [config/mod.rs:1311-1312](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/config/mod.rs:1311): `effective_agent_max_threads` applies `saturating_sub(1)`. Result: out-of-box cap is 3 coding children, not 5.

**Implemented**: default raised to 6, with focused config coverage proving five child-agent slots.

### P0: Child agents retain recursive spawn_agent access

[spec_plan.rs:672](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:672): `non_code_mode_only = true` gives `ToolExposure::DirectModelOnly`, but the tool IS still exposed. [spec_plan.rs:290-298](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:290): `collab_tools_enabled` returns `true` for V2 unconditionally.

**Implemented**: coding thread-spawn sub-agents no longer receive `spawn_agent`; root and non-code sessions retain the existing tool path.

### P0: ADR references functions that don't exist

`maybe_notify_parent_of_terminal_turn` and `forward_child_completion_to_parent` do not exist in the current codebase (zero `rg` hits). Parent completion works through inter-agent messaging, not those symbols. The "Existing owners" section was incorrect and should refer to `ontocode-rs/protocol/src/protocol/mod.rs` for inter-agent communication.

### P1: Refusal message lacks slot-release guidance

[multi_agents_common.rs:112-119](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/multi_agents_common.rs:112): `collab_spawn_error` renders `AgentLimitReached` as `"collab spawn failed: Agent limit reached (max N threads)"`. No mention that root counts, or that closing agents frees slots.

**Implemented**: `AgentLimitReached` now tells the manager the configured child-agent cap was reached and that closing completed, failed, or cancelled agents frees slots.

### P2: No test for five-worker scenario

Tests use `max_concurrent_threads_per_session = 2`. No integration test exercises 5 concurrent children, the sixth refusal message, or `list_agents` after closing 5 agents.

**Implemented**: added focused coverage for five children, sixth refusal, close-slot release, and refusal-message text.

## Existing owners

- `ontocode-rs/core/src/config/mod.rs`: `MultiAgentV2Config.default`, `resolve_multi_agent_v2_config`, `Config::effective_agent_max_threads`.
- `ontocode-rs/core/src/tools/spec_plan.rs`: `collab_tools_enabled`, `non_code_mode_only` gating.
- `ontocode-rs/core/src/agent/control.rs`: `reserve_spawn_slot`, `close_agent`, `shutdown_live_agent`.
- `ontocode-rs/core/src/agent/registry.rs`: `AgentRegistry.release_spawned_thread`, `SpawnReservation`, `try_increment_spawned`.
- `ontocode-rs/core/src/tools/handlers/multi_agents_tests.rs`: close/list/wait/spawn tests, stale task-name close, `list_agents_omits_closed_agents` at line 1789.
- `ontocode-rs/core/src/agent/control_tests.rs`: slot release, forked context, parent completion.
- `ontocode-rs/core/src/tools/handlers/multi_agents_v2/close_agent.rs`: close handler.
- `ontocode-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs`: spawn handler with `collab_spawn_error` rendering.

## Consolidated Donor Evidence

| Requirement | Donor evidence | Ontocode decision |
| --- | --- | --- |
| One numeric cap is enough for phase one. | `tmp/qwen-code/packages/core/src/agents/background-tasks.ts`, `tmp/gemini-cli-main/packages/core/src/agents/types.ts`, `tmp/openclaw-main/docs/tools/subagents.md` | Keep v2 session cap as the only runtime capacity control. |
| Parent-visible completion must be push-like. | `tmp/opencode-main/packages/opencode/src/tool/task.ts`, `tmp/openclaw-main/qa/scenarios/agents/subagent-completion-direct-fallback.md` | Use existing inter-agent messaging and `wait_agent`; record result before `close_agent`. |
| Child status must be compact and terminal states obvious. | `tmp/claude-code-main/src/tools/TaskListTool/TaskListTool.ts`, `tmp/qwen-code/packages/core/src/tools/task-list.ts`, `tmp/gemini-cli-main/packages/core/src/agents/types.ts`, `tmp/opencode-main/packages/ui/src/components/tool-count-summary.tsx` | Extend `list_agents`/job output only if current status is insufficient. |
| Child permissions must not bypass parent restrictions or recurse. | `tmp/opencode-main/packages/opencode/src/agent/subagent-permissions.ts`, `tmp/gemini-cli-main/packages/core/src/agents/local-executor.ts`, `tmp/openclaw-main/docs/tools/multi-agent-sandbox-tools.md` | Strip `spawn_agent` from coding sub-agent tool catalogs (P0 fix needed). |
| Dispatch prompts must declare scope. | `tmp/opencode-main/packages/opencode/src/tool/task.txt`, `tmp/openclaw-main/docs/concepts/parallel-specialist-lanes.md` | Manager must provide task id, write scope, tests, stop conditions, and production-edit permission. |
| Queues and worktrees deferred. | `tmp/opencode-main/packages/core/src/session/run-coordinator.ts`, `tmp/openclaw-main/src/plugin-sdk/keyed-async-queue.ts`, `tmp/gemini-cli-main/docs/cli/git-worktrees.md`, `tmp/opencode-main/packages/opencode/src/worktree/index.ts` | Defer keyed queue and opt-in worktrees. |
| Tool output needs bounds. | `tmp/opencode-main/packages/core/src/tool-output-store.ts`, `tmp/qwen-code/integration-tests/concurrent-runner/README.md` | Reuse existing output caps/redaction owners. |

## Rejected

- Full scheduler or worker pool.
- SQLite/in-memory custom slot table.
- Worktree-by-default isolation.
- Nested sub-agent fan-out.
- New task/list/output tools.
- Donor implementation architectures.

## Dispatch Contract

Every coding worker request must include:

- bundle/task id;
- declared write scope;
- expected tests;
- owner files/modules;
- stop conditions;
- whether production edits are allowed.

Before spawning:

1. List existing agents.
2. Record results for terminal agents.
3. Close terminal agents whose results are recorded.
4. Confirm the v2 session cap is at least `6`.
5. Refuse overlapping write scopes.
6. Spawn up to five direct coding children or record a blocked reason.

## Minimal Implementation Plan

1. Raise `DEFAULT_MULTI_AGENT_V2_MAX_CONCURRENT_THREADS_PER_SESSION` from 4 to 6. **Done.**
2. Strip `spawn_agent` from coding sub-agent tool catalogs. **Done.** (root and non-code sessions retain the tool.)
3. Verify status text or tool output explains that closing agents frees slots. **Done.**
4. Verify `close_agent` releases terminal child slots after result capture. (Existing test covers `list_agents` omits closed.)
5. Verify closed agents are omitted from `list_agents`. (Done in `multi_agent_v2_list_agents_omits_closed_agents`.)
6. Add integration test: 5 concurrent children, sixth refused. **Done.**
7. Verify child workers cannot call `spawn_agent`. **Done.**

## Acceptance Criteria

- Five disjoint coding workers can run at the same time under multi-agent v2.
- A sixth direct child is refused clearly (message includes how to free slots).
- Terminal children release slots after result capture and close.
- Parent-visible completion is not lost or double-counted.
- Manager tracking records write scopes before spawn.
- Status output is compact enough to manage five workers.
- Child agents CANNOT call `spawn_agent` or recursive agent tools.
- No new scheduler, task database, SQLite queue, recursive task runtime, or default worktree isolation.

## Phase-Two Gates

Add a keyed queue only if users repeatedly need "spawn later when a slot frees."

Add SQLite only if dispatch state must survive restart or be shared across manager sessions.

Add opt-in worktrees only if repeated write-scope conflicts happen despite manager discipline.

## Follow-Up Review

Senior review reopened one owner-local gap after initial implementation:

- Code-mode `SubAgentSource::Other(...)` sessions could still receive `spawn_agent`; only `ThreadSpawn` sub-agents were hidden.

Accepted follow-up:

- `ADR_FIVE_CONCURRENT_CODING_SUBAGENTS_UNBLOCK.md`

Follow-up implemented:

- All code-mode `SessionSource::SubAgent(_)` sessions now hide `spawn_agent`.
- Focused coverage proves root code-mode keeps `spawn_agent`, code-mode `ThreadSpawn` hides it, and code-mode `Other("agent_job:*")` hides it while retaining non-recursive coordination tools.
