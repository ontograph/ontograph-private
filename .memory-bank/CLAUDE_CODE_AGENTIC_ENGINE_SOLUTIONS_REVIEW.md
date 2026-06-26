# Claude Code Agentic Engine Consolidated Solution

status: challenged-proposed
date: 2026-06-24
donor: `tmp/claude-code-main`
target: Ontocode Rust workspace
method: OntoIndex-grounded review plus donor source inspection

## Executive Decision

Implement a single Ontocode-native agentic engine by extending the existing multi-agent v2 runtime, TUI slash-command/picker surface, and persisted agent-job layer.

Do not import Claude Code's runtime architecture.

Do not create:

- a second sub-agent runtime
- a second agent registry
- a second task system
- a separate model-routing layer for agents
- a Claude-style `loadAgentsDir` clone as a new top-level owner

The right shape for Ontocode is one engine with three coordinated surfaces:

1. interactive agent control for `/agent` and `/subagents`
2. read-only and then bounded interactive TUI agent management
3. persisted batch/background execution through the existing agent-job owner

## Challenge Verdict

Keep the consolidated direction, but narrow the implementation order.

The draft is correct that Ontocode should not import Claude Code's runtime or create a second agent/task/model stack. The overreach is treating deterministic core dispatch as an early required phase. The already-proposed `/agent` ADR chose a thin slash wrapper first and reserved deterministic dispatch only if prompt-mediated tool use proves unreliable. That remains the safer professional path because it reuses the current multi-agent schemas, validation, model visibility rules, and test harness.

The first implementation target is therefore not a new "agentic engine" layer. It is a bounded human-facing command and UX layer over the engine Ontocode already has.

Challenge outcome:

- Keep: single runtime, single registry, single persisted job owner.
- Narrow: `/agent create|send|wait|list|close` starts as a thin TUI wrapper that submits bounded manager instructions to existing multi-agent tools.
- Defer: deterministic direct dispatch, reusable profiles, public app-server APIs, and persistent agent templates until the wrapper and picker prove the user workflow.
- Reject: any plan that adds Claude-style `loadAgentsDir` source precedence, a second task table, a new model router, or model-visible agent progress/memory payloads.

## OntoIndex Grounding

OntoIndex confirms the current owners already cover the core engine:

### Spawn tool contract

- `create_spawn_agent_tool_v2`
- file: `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`

This is the current schema owner for `spawn_agent`, including task name, model visibility, and metadata hiding.

### Agent runtime control and listing

- `AgentControl.list_agents`
- file: `ontocode-rs/core/src/agent/control.rs`

This is the current owner for enumerating live agents and agent metadata, and it is already called by the multi-agent v2 `list_agents` handler.

### TUI navigation and picker state

- `AgentNavigationState`
- file: `ontocode-rs/tui/src/app/agent_navigation.rs`
- related render owner: `ontocode-rs/tui/src/multi_agents.rs`

This is the current owner for stable spawn-order navigation and picker labels. It is the right place to extend Claude-style agent switching and bounded management UX.

### Slash command entry path

- `ChatComposer.try_dispatch_slash_command_with_args`
- file: `ontocode-rs/tui/src/bottom_pane/chat_composer.rs`
- `inline_command`
- file: `ontocode-rs/tui/src/bottom_pane/chat_composer/slash_input.rs`
- `ChatWidget.dispatch_command`
- file: `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs`

OntoIndex shows inline slash commands flow through the composer before `ChatWidget.dispatch_command`. Any `/agent` verb work must respect this entry path, not only `slash_command.rs` and `slash_dispatch.rs`.

### Batch/background execution

- `run_agent_job_loop`
- file: `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`
- `create_spawn_agents_on_csv_tool`
- file: `ontocode-rs/core/src/tools/handlers/agent_jobs_spec.rs`
- persisted state: `ontocode-rs/state/src/runtime/agent_jobs.rs`

This is already the current background worker engine. It should remain the only persisted task owner.

## Donor Findings That Matter

The donor mixes four different ideas under one "agentic engine" label:

1. sub-agent spawn/runtime control
2. named-agent and agent-menu UX
3. background task state and task output inspection
4. memory/speculation enhancements

Ontocode already has 1 and 3 in core form.

The real missing pieces are:

- a professional human-facing control surface for spawning and managing agents
- a clearer role/profile layer for reusable specialized agents
- a better UX bridge between interactive threads and persisted batch jobs

Memory/speculation should stay deferred. They are engine enhancers, not the engine.

## Consolidated Architecture

## Principle

Build a single agentic system around existing Ontocode owners:

- multi-agent v2 is the interactive runtime
- TUI `/agent` plus picker is the human control surface
- `agent_jobs` is the persisted batch/background surface

Everything else plugs into one of those three owners.

## Target User Experience

The final user-visible behavior should be:

- `/agent create <name> [--role <role>] [--model <model>] [--reasoning <effort>] <prompt>`
- `/agent send <agent> <message>`
- `/agent wait [agent]`
- `/agent list`
- `/agent close <agent>`
- `/subagents` as an alias
- bare `/agent` still opens the picker
- the picker shows live agent state, stable labels, role/model summaries, and compact recent activity
- long-running fan-out or CSV/batch work appears through the existing job layer rather than an ad hoc thread list

## Solution

### Layer 1: Agent Control Surface

Extend `/agent` and `/subagents` into the primary Claude Code-like control surface.

Implementation home:

- `ontocode-rs/tui/src/slash_command.rs`
- `ontocode-rs/tui/src/bottom_pane/chat_composer.rs`
- `ontocode-rs/tui/src/bottom_pane/chat_composer/slash_input.rs`
- `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs`

Required behavior:

- support inline management verbs: `create`, `send`, `wait`, `list`, `close`
- preserve existing bare-command picker behavior
- expose model/reasoning flags only when the runtime schema already supports them
- report inheritance clearly when metadata fields are hidden by runtime policy
- convert valid commands into bounded manager instructions that use existing multi-agent tools

Design rule:

The control surface must be thin. It may parse commands and route them to the current session, but it must not become a parallel runtime or duplicate multi-agent validation.

### Layer 2: Conditional Deterministic Core Dispatch

Use the current multi-agent runtime as the final execution owner for interactive management commands.

Implementation home:

- `ontocode-rs/core/src/tools/handlers/multi_agents_v2/*`
- `ontocode-rs/core/src/agent/control.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`

Required behavior:

- `create` routes to current spawn behavior
- `send` routes to current message/follow-up behavior
- `wait` routes to current wait behavior
- `list` routes to `AgentControl.list_agents`
- `close` routes to current close behavior

Decision:

Do not make deterministic dispatch a required Phase 2 deliverable. Treat it as a follow-up only if the thin slash wrapper cannot provide reliable command semantics.

Reason:

- professional CLI semantics are easier to test and explain
- model compliance should not be the correctness boundary for core agent management
- all critical validation already exists in current owners and should remain there
- direct dispatch needs more app event/core plumbing and can accidentally duplicate validation owned by the multi-agent handlers

Gate:

Before implementing deterministic dispatch, capture at least one concrete failure mode from the thin wrapper and write the owner map for the app event/core path it needs. Otherwise this is extra architecture, not demonstrated value.

### Layer 3: Read-Only Then Bounded Interactive Agent UX

Build the Claude-style agent experience in the TUI over existing events and list data.

Implementation home:

- `ontocode-rs/tui/src/multi_agents.rs`
- `ontocode-rs/tui/src/app/agent_navigation.rs`
- `ontocode-rs/tui/src/chatwidget/tool_lifecycle.rs`
- existing `list_agents` output path

Required behavior:

- stable visual labels per agent
- role and effective model visibility
- compact progress summaries
- last task message and terminal status visibility
- read-only default

Allowed later:

- bounded send/wait/close actions from the picker

Not allowed:

- protocol-level color metadata
- unbounded transcript progress injection
- a second TUI-only source of truth for agent state

### Layer 4: Reusable Agent Profiles

Add named specialized agents only after the control surface and read-only picker are stable.

Implementation home:

- existing role/config path
- `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`
- possibly skills/plugins if packaging fits better than core config

Required behavior:

- users can choose a known role/profile instead of always free-form prompting
- the system shows effective inherited/configured model clearly
- profile visibility reuses current config/role ownership
- no config writes or public schema surface without a separate ADR

Not allowed:

- a new global agent-definition registry
- Claude donor `loadAgentsDir.ts` semantics copied into a separate owner
- source precedence rules invented solely for agent definitions

Decision:

Profiles are a layer on top of the runtime, not a replacement for it. The first acceptable version may be read-only role/profile discovery if existing config data already exposes the source.

### Layer 5: Batch and Background Workflows

Use `agent_jobs` as the only persisted background/batch execution owner.

Implementation home:

- `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`
- `ontocode-rs/core/src/tools/handlers/agent_jobs_spec.rs`
- `ontocode-rs/state/src/runtime/agent_jobs.rs`

Required behavior:

- unify interactive agent and job UX enough that users understand when they are managing a thread versus a job
- expose job status, item progress, final summaries, and capped output/result access
- keep fan-out and CSV work here instead of pushing it into the interactive picker

Decision:

Claude's `Task.ts` / `TaskListTool` / `TaskOutputTool` idea maps to Ontocode `agent_jobs`, not to a new runtime.

## Phased Plan

## Phase 1: Thin Professional `/agent` Wrapper

Deliver:

- inline `/agent create|send|wait|list|close`
- `/subagents` alias
- bare `/agent` still opens picker
- explicit usage and inheritance messaging
- bounded manager instruction generation over existing multi-agent tools

Success condition:

- users can manage agents without knowing raw tool names

Non-goal:

- direct core dispatch

## Phase 2: Read-Only Agent Activity UX

Deliver:

- compact activity list
- stable labels and role/model visibility
- bounded progress summaries sourced from existing events and `list_agents`

Success condition:

- the picker becomes the operational dashboard for live agent threads without adding a second state owner

## Phase 3: Deterministic Dispatch Path If Proven Necessary

Deliver:

- core-backed dispatch for management verbs only after wrapper failure evidence exists
- direct routing through current multi-agent owners, not copied validation
- explicit test coverage for every command verb

Success condition:

- command semantics are stable without introducing duplicate runtime or validation logic

## Phase 4: Reusable Profiles

Deliver:

- read-only role/profile discovery first
- effective model/tool visibility
- spawn-by-profile support only if existing role/config ownership can express it

Success condition:

- repeated agent workflows become discoverable and reusable

## Phase 5: Job/Thread Unification

Deliver:

- clearer boundary between live agent threads and persisted jobs
- batch/background status surfaces over `agent_jobs`

Success condition:

- users can move between single-agent delegation and fan-out work without learning a second mental model

## Deferred

Keep deferred until separately justified:

- memory snapshots for agents
- speculative read-only background suggestions
- remote/worktree/monitor task-type expansion as the primary plan
- public app-server API for agent management
- donor bridge/remote orchestration ideas

## Stop Conditions

Stop and reject any proposal that requires:

- a new sub-agent runtime
- a new persisted task database
- a duplicate model validation path
- a new top-level agent-definition registry
- model-visible unbounded progress or memory payloads
- app-server/public API changes before TUI/core semantics are stable
- direct deterministic dispatch before the thin wrapper has proven unreliable
- persistent profile/template writes before source ownership, migration, and compatibility tests are specified

## Why This Is The Right Solution

This consolidates the earlier proposal set into one professional architecture:

- it preserves Ontocode's current runtime owners
- it adopts the donor's best UX ideas
- it keeps interactive and persisted execution clearly separated
- it gives a direct path from today's picker-only `/agent` to a real Claude Code-like agent workflow
- it avoids importing Claude Code's internal architecture where Ontocode already has a working owner

## Acceptance Standard

The solution is complete when:

- `/agent` is a real management surface, not just a picker shortcut
- multi-agent v2 remains the only interactive runtime
- `agent_jobs` remains the only persisted batch owner
- users can discover, manage, and monitor agents with bounded UX
- reusable profiles exist without introducing a parallel registry, or remain explicitly deferred with a source-owner gap
- no second engine was created to achieve the Claude-style experience

## Review Findings To Carry Forward

### P1: Deterministic dispatch is currently premature

The current graph shows `AgentControl.list_agents` is called by the v2 list handler, while slash commands enter through `ChatComposer.try_dispatch_slash_command_with_args`, `inline_command`, and `ChatWidget.dispatch_command`. Jumping directly to deterministic dispatch would require new app event/core plumbing and risks duplicating handler validation. Keep it conditional.

### P1: The implementation-home list was incomplete

Any `/agent` verb implementation must include the composer inline-argument path, not only `slash_command.rs` and `slash_dispatch.rs`. Otherwise tests may pass for direct dispatch helpers while real typed slash commands behave differently.

### P2: Profiles are not part of the first engine slice

Reusable profiles are useful, but the current accepted sub-agent handling ADR blocks new agent-definition registries and source-precedence rules. Start with visibility over existing role/config data. Writes, discovery precedence, and spawn-by-profile need separate proof.

### P2: Job/thread "unification" must stay UX-only

`run_agent_job_loop` already owns persisted batch work and calls `AgentControl.spawn_agent_with_metadata` plus state runtime job/item transitions. Unification should mean consistent status and navigation language, not merging interactive threads into the job table or adding a new task abstraction.
