# ADR: Slash-Command Sub-Agent Management

## Status

Proposed

## Date

2026-06-15

## Context

Users need `/` commands that let agents manage sub-agents in a Claude Code-like workflow:

- create a sub-agent from a prompt
- choose or override the sub-agent model
- send follow-up work
- wait for sub-agent results
- list and switch active sub-agent threads
- close sub-agents when done

OntoIndex review found that the repo already has most of the runtime substrate:

- `/agent` and `/subagents` exist in `ontocode-rs/tui/src/slash_command.rs`.
- Current `/agent` and `/subagents` dispatch only opens the agent picker in `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs`.
- Core multi-agent tools already expose `spawn_agent`, `send_message`, `followup_task`, `resume_agent`, `wait_agent`, `list_agents`, and `close_agent` from `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`.
- Spawn-time model, reasoning, and service-tier override plumbing already exists in `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`.
- Custom model behavior is governed by `ADR_CUSTOM_SUBAGENT_MODELS.md`.

The current gap is not a missing sub-agent runtime. The gap is a human-facing slash-command control surface.

## Decision

Start with a thin `/agent` slash-command wrapper over the existing multi-agent tool surface.

The first implementation should make `/agent` and `/subagents` support inline arguments and translate those arguments into a bounded manager instruction submitted to the current session. The current model then uses the existing multi-agent tools.

Initial command shape:

```text
/agent create <name> [--model <model>] [--reasoning <effort>] [--prompt <prompt>]
/agent send <agent> <message>
/agent wait [agent]
/agent list
/agent close <agent>
```

`/subagents` remains an alias for the same surface.

Bare `/agent` and bare `/subagents` keep the current behavior: open the agent picker.

## Rationale

This is the smallest useful change:

- no new sub-agent runtime
- no new model registry
- no new MCP/tool registry
- no new persistence layer
- no app-server API commitment
- no duplicate validation path for model overrides

It reuses the existing multi-agent tools and preserves the accepted custom sub-agent model contract.

## Alternatives Considered

### Option A: Thin Slash Wrapper

Parse `/agent ...` in TUI and submit a manager instruction that uses existing multi-agent tools.

Pros:

- smallest implementation
- minimal blast radius
- reuses existing tool schema and validation
- easy to test with slash-command dispatch tests

Cons:

- less deterministic because the model still decides the exact tool call
- must clearly report when the runtime schema hides model override fields

### Option B: Deterministic TUI Dispatch

Parse `/agent create|send|wait|list|close` in TUI and dispatch directly through core multi-agent control paths.

Pros:

- deterministic command behavior
- better validation before dispatch
- less dependent on model compliance

Cons:

- requires more app event/core plumbing
- risks duplicating validation already owned by multi-agent handlers
- larger test surface

Use this only if Option A proves too unreliable.

### Option C: Persistent Agent Templates

Add Claude Code-like reusable agent definitions with name, prompt/instructions, model, and reasoning settings. Runtime `/agent create <template>` spawns from the template.

Pros:

- closest user model to named Claude Code agents
- reusable across sessions

Cons:

- config writes need ownership, validation, and migration rules
- should build on a working runtime command path

Defer until create/send/wait/list/close are useful.

### Option D: Full Agent Manager UI

Extend the existing agent picker into a management UI with create, prompt edit, model picker, send, wait, and close actions.

Pros:

- best TUI experience
- avoids fragile CLI-style parsing for interactive users

Cons:

- highest TUI churn
- requires snapshot coverage
- should wait until command semantics are stable

### Option E: App-Server API First

Add app-server v2 methods such as `agent/list`, `agent/create`, `agent/send`, and `agent/close`, then make TUI consume them.

Pros:

- supports TUI, desktop, SDK, and remote clients through one backend surface

Cons:

- public API/schema/docs/compatibility burden
- too large for the first slash-command slice

Do not start here unless non-TUI clients are an immediate requirement.

## Implementation Plan

### Stage 1: Inline `/agent` Wrapper

- Add `SlashCommand::Agent` and `SlashCommand::MultiAgents` to `supports_inline_args`.
- Keep bare command behavior as `AppEvent::OpenAgentPicker`.
- Add a small parser for the first token: `create`, `send`, `wait`, `list`, `close`.
- Convert valid inline commands into a bounded user message that instructs the current agent to use existing multi-agent tools.
- Emit usage errors for unknown subcommands.

### Stage 2: Model Override Guardrails

- Reuse `ADR_CUSTOM_SUBAGENT_MODELS.md`.
- If the active `spawn_agent` schema hides `model`, `reasoning_effort`, or `service_tier`, the generated manager instruction must say the child will inherit the parent model.
- Do not claim model pinning unless the tool schema exposes and accepts the model field.

### Stage 3: Deterministic Dispatch If Needed

Only if Stage 1 is too unreliable:

- add a narrow app event/core path for `create`, `send`, `wait`, `list`, and `close`
- route through existing multi-agent owners
- avoid copying model validation out of `multi_agents_common.rs`

### Stage 4: Templates And UI

Only after runtime commands work:

- add persistent agent templates using existing config/agent-role ownership
- extend the agent picker into a management UI
- add snapshot coverage for user-visible TUI changes

## Non-Goals

- Do not create a second sub-agent runtime.
- Do not create a sub-agent-only model registry.
- Do not bypass `ModelsManager` for model selection.
- Do not add provider-qualified alias parsing here.
- Do not add app-server public API surface in the first slice.
- Do not change MCP ownership or introduce a second MCP tool registry.
- Do not duplicate spawn validation logic outside the existing multi-agent owners.

## Verification

For Stage 1:

- Add slash-command tests proving bare `/agent` still opens the picker.
- Add slash-command tests for `/agent create`, `/agent send`, `/agent wait`, `/agent list`, and `/agent close`.
- Add an error test for an unknown `/agent` subcommand.
- Add or update TUI snapshots only if user-visible rendering changes.

For model override behavior:

- Reuse or extend `spawn_agent` tests from the custom sub-agent model ADR.
- Verify hidden metadata mode does not promise model override support.

Commands after Rust changes:

```text
cd ontocode-rs
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p ontocode-tui slash_commands
CARGO_BUILD_JOBS=8 just test -p ontocode-core spawn_agent
```

## Open Questions

- Should `/agent create` accept only explicit flags, or also free-form natural language?
- Should `/agent list` be deterministic from TUI state in Stage 1, or model-mediated through `list_agents`?
- Should persistent templates reuse existing agent role files directly, or introduce a thinner user-facing template command that writes those files later?

