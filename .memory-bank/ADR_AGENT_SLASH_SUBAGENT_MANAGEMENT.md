# ADR: Slash-Command Sub-Agent Management

## Status

Partially implemented

Narrow picker-owned definition authoring is implemented through valid blank scaffolds, proposal-to-definition scaffolds, optional role fields, repo-local definition copy, repo-local definition rename, and repo-local definition delete. Broader runtime dispatch/profile/job surfaces and richer donor parity remain blocked or deferred by the staged decisions below.

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
- rename live agent labels and, later, repo-local agent definitions
- set model names at agent-creation time and, later, in repo-local agent definitions
- delete live agent threads and, later, repo-local agent definitions
- review chat history and propose reusable agents for repeating tasks
- copy repo-local agent definitions and, later, save live agents as new definitions

OntoIndex review found that the repo already has most of the runtime substrate:

- `/agent` and `/subagents` exist in `ontocode-rs/tui/src/slash_command.rs`.
- Current `/agent` and `/subagents` dispatch only opens the agent picker in `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs`.
- Core multi-agent tools already expose `spawn_agent`, `send_message`, `followup_task`, `resume_agent`, `wait_agent`, `list_agents`, and `close_agent` from `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`.
- Spawn-time model, reasoning, and service-tier override plumbing already exists in `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`.
- Custom model behavior is governed by `ADR_CUSTOM_SUBAGENT_MODELS.md`.

The current gap is not a missing sub-agent runtime. The gap is a human-facing slash-command control surface.

Follow-up UX review against `./tmp/claude-code-main` found one donor idea that should be adopted directly at the product-decision level: agent-definition creation must produce a complete, loadable definition instead of a placeholder file. Claude Code's creation flow separates:

- a user prompt describing what the agent should do
- a generated identifier/name
- a required trigger description
- the full agent prompt/body
- optional model/tool/color/memory metadata

Ontocode's current role loader already requires a description, so any `/agent` creation path that omits `description` creates a broken file and later emits malformed-role warnings.

## Decision

Keep two separate tracks:

- live sub-agent management, which starts as a thin `/agent` slash-command wrapper over the existing multi-agent tool surface
- repo-local agent-definition authoring, which writes `.codex/agents/<slug>.toml` files through the existing role/config contract

Do not blur these tracks. `/agent create <name> --prompt ...` requests live runtime sub-agent work through the existing multi-agent tools; picker actions such as `Create from proposal` create reusable repo-local definitions.

The first implementation should make `/agent` and `/subagents` support inline arguments and translate those arguments into a bounded manager instruction submitted to the current session. The current model then uses the existing multi-agent tools.

Initial command shape:

```text
/agent create <name> [--model <model>] [--reasoning <effort>] [--prompt <prompt>]
/agent rename <agent> <name>
/agent delete <agent>
/agent send <agent> <message>
/agent wait [agent]
/agent list
/agent close <agent>
```

`/subagents` remains an alias for the same surface.

Bare `/agent` and bare `/subagents` keep the current behavior: open the agent picker.

For repo-local agent definitions, the accepted UI decision is to mirror the Claude Code donor idea one-for-one for the core creation concepts while writing Ontocode's existing TOML contract:

| Donor creation concept | Ontocode field / behavior |
| --- | --- |
| identifier/name | `name` plus `.codex/agents/<slug>.toml` |
| when-to-use / trigger description | required `description` |
| system prompt / agent body | `developer_instructions` |
| generate from user description | `/agent` `Create from proposal` |
| manual wizard field validation | reject or repair files missing required `description` |

Every `/agent` definition creation path must write a loadable file on first save. If the user supplies only one line, use that line as the initial `name`, `description`, and `developer_instructions` seed. The blank scaffold path may still exist, but it must also write a non-empty `description`.

This is not full donor parity. Donor-only or not-yet-owned metadata such as `tools`, `color`, memory settings, full model picker behavior, and plugin packaging stay deferred until the core loadable definition path is correct.

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

### Option C1: Template Creation From Proposal Prompt

Add `/agent` support for creating a repo-local agent definition from one freeform proposal prompt, then write a role file scaffold under `.codex/agents/<slug>.toml`.

Pros:

- keeps the same picker and role-file owner
- reuses the existing scaffold-write path
- gives immediate user value without reopening runtime role application

Cons:

- generated text quality can vary
- a structured wizard would add validation-drift risk too early
- should not grow into hot reload, editor-open plumbing, or a second registry

If templates are added, this is the first acceptable shape.

Donor-aligned refinement:

- treat this as the default creation path, not a secondary expert action
- prompt for "what should this agent do and when should it be used"
- write all required loadable fields immediately: `name`, `description`, and `developer_instructions`
- use a one-line prompt as the first seed for all three fields rather than creating a placeholder body
- keep richer donor fields (`tools`, `color`, memory settings, full model picker) out until the core loadable definition flow is stable

### Option C2: Rename Live Agent Label

Add `/agent` support for renaming only the visible name of a live agent thread.

Pros:

- smallest rename slice
- stays inside current thread metadata and picker ownership
- no config writes or file moves

Cons:

- does not rename the reusable definition behind the thread
- users must understand that live label and saved definition name are different things

This is the first acceptable rename shape.

### Option C3: Rename Repo-Local Agent Definition

Add `/agent` support for renaming a repo-local definition by updating both the file path and the `name = ...` field inside `.codex/agents/<slug>.toml`.

Pros:

- renames the reusable definition itself
- stays inside existing role-file ownership

Cons:

- needs collision handling and file-move semantics
- must not blur into live-thread rename, source precedence, or registry work

Defer until live-label rename is useful and the role-file authoring flow is already established.

### Option C4: Set Model For Live Agent Creation

Add `/agent` support for setting the model name only when creating a new live agent, using the existing `--model` create-time path.

Pros:

- smallest model-setup slice
- already aligned with current `spawn_agent` ownership
- no config writes

Cons:

- affects only new live agents
- must clearly fall back to parent-model inheritance when the active schema hides model fields

This is the first acceptable model-setup shape.

### Option C5: Set Model In Repo-Local Agent Definition

Add `/agent` support for setting `model = "..."` in repo-local `.codex/agents/<slug>.toml` definitions.

Pros:

- makes the chosen model reusable across sessions
- stays inside the existing role/config contract

Cons:

- needs file-write validation and reload semantics
- should not precede the scaffold-first definition flow

Defer until repo-local definition authoring is already useful.

### Option C6: Delete Live Agent Thread

Add `/agent` support for deleting only a live agent thread from the current session UI.

Pros:

- smallest delete slice
- stays inside current thread/picker ownership
- no config writes or file deletes

Cons:

- needs precise semantics: for the first slice, this should mean close and remove from current session visibility, not introduce a new archive system
- does not delete any reusable definition behind the thread

This is the first acceptable delete shape.

### Option C7: Delete Repo-Local Agent Definition

Add `/agent` support for deleting a repo-local `.codex/agents/<slug>.toml` definition file.

Pros:

- deletes the reusable definition itself
- stays inside existing role-file ownership

Cons:

- destructive file operation
- needs explicit confirmation and collision-with-current-use handling

Defer until repo-local definition management is already useful.

### Option C8: Create Agent From Current Chat

Add `/agent` support for creating a repo-local agent definition from the current chat thread.

Pros:

- smallest history-to-agent slice
- stays inside current thread context plus the existing scaffold writer
- no new history index or analytics owner

Cons:

- only sees one thread
- may miss repetition across sessions

This is the first acceptable history-based proposal shape.

### Option C9: Create Agent From Selected Recent Chats

Add `/agent` support for reviewing a few recent chats, selecting one or more, and generating a repo-local agent scaffold from those chosen threads.

Pros:

- better signal than one chat alone
- still user-guided
- avoids automatic clustering or background analytics

Cons:

- more picker UI
- requires explicit recent-thread selection flow

Defer until current-chat promotion is useful.

### Option C10: Copy Repo-Local Agent Definition

Add `/agent` support for copying one repo-local `.codex/agents/<slug>.toml` definition into a new definition file.

Pros:

- smallest useful copy slice
- stays inside existing role-file ownership
- no runtime mutation or cross-scope rules

Cons:

- needs collision checks
- should only update the file slug/path and internal `name = ...` field in the first slice

This is the first acceptable copy shape.

### Option C11: Save Live Agent As New Definition

Add `/agent` support for taking a live agent and writing a new repo-local definition scaffold from it.

Pros:

- useful when a live agent setup worked well and should be reused
- still ends in the same role-file contract

Cons:

- source state can be incomplete or ambiguous compared with copying an existing definition
- should not overclaim that every runtime detail is preserved

Defer until definition-to-definition copy is already useful.

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
- Prefer create-time model selection over runtime mutation of already-running agents.

### Stage 3: Deterministic Dispatch If Needed

Only if Stage 1 is too unreliable:

- add a narrow app event/core path for `create`, `send`, `wait`, `list`, and `close`
- route through existing multi-agent owners
- avoid copying model validation out of `multi_agents_common.rs`

### Stage 4: Templates And UI

Only after runtime commands work:

- add persistent agent templates using existing config/agent-role ownership
- if creation-from-prompt is needed, start with a repo-local `Create from proposal` flow that:
  - asks for one freeform prompt describing what the agent should do and when to use it
  - generates or seeds only `name`, `description`, and `developer_instructions`
  - writes `.codex/agents/<slug>.toml`
  - guarantees the written file satisfies the current role loader's required fields
  - tells the user to edit the file and reopen `/agent` or restart
- prefer prompt-to-valid-definition over a structured wizard for the first slice
- extend the agent picker into a management UI only after the scaffold-first flow proves useful
- add snapshot coverage for user-visible TUI changes

### Stage 4A0: Valid Blank Scaffold And Repair

This is the first active definition-authoring blocker.

- update any remaining blank-definition path so it writes a required non-empty `description`
- treat existing repo-local files missing `description` as repair candidates, not valid roles
- optional first repair action: add `description = "<name>"` for malformed repo-local definitions under `.codex/agents`
- keep repair explicit and repo-local; do not silently mutate all malformed files during normal picker open
- do not relax the core role loader to allow missing descriptions

This keeps the donor rule intact: the creation UI must not produce files that the runtime rejects. A fallback description equal to the name is only a loadability repair, not a claim that the definition is semantically complete.

### Stage 4A: Prompt-To-Valid-Definition

If users need agent-definition authoring before a full template system:

- make `Create from proposal` the default creation action in the existing `/agent` picker
- prompt once for a freeform proposal such as role purpose, style, trigger conditions, and constraints
- generate a bounded TOML scaffold with only:
  - `name`
  - `description`
  - `developer_instructions`
- if only one non-empty line is provided, seed all three fields from that line so the file remains valid and immediately loadable
- do not generate yet:
  - `model`
  - `model_reasoning_effort`
  - `service_tier`
  - `nickname_candidates`
  - tool, sandbox, or approval metadata
- keep the success boundary the same: file write only, then manual edit and reload

This is the smallest acceptable authoring extension because it improves scaffold contents without introducing a second runtime or config-writing UX surface.

### Stage 4A4: Donor-Style Generate From Description

If users want closer Claude Code parity after valid scaffolds work:

- add a `Generate agent definition` action that accepts one user description
- produce only Ontocode's three core fields:
  - `name`
  - `description`
  - `developer_instructions`
- use the current model to derive a concise slug/name, a "Use this agent when..." style description, and a complete developer instruction body
- preview the generated TOML before writing
- keep the same repo-local `.codex/agents/<slug>.toml` owner

Do not add donor-only metadata (`color`, `tools`, memory config, plugin packaging) in this stage.

### Stage 4A2: Promote Current Chat To Agent

If users need reusable agents for repeating tasks seen in chat history:

- add `Create from this chat` in the existing `/agent` picker or current-thread actions
- use only the current thread as input
- summarize the repeated task pattern into a bounded scaffold with only:
  - `name`
  - `description`
  - `developer_instructions`
- write `.codex/agents/<slug>.toml`
- keep the same reload boundary: reopen `/agent` or restart

This is the recommended first history-based implementation because it reuses current thread context and the existing scaffold writer without adding a new history-analysis subsystem.

### Stage 4A3: Promote Selected Recent Chats

Only if current-chat promotion proves too narrow:

- add `Review recent chats for reusable task`
- let the user pick one or a few recent threads
- generate the same bounded scaffold from only those selected chats
- keep the output contract the same as prompt-to-scaffold

Do not start with automatic repeated-task detection.

### Stage 4A1: Create-Time Model Selection

If users need `/agent` model setup before full definition management:

- keep model setup on creation only
- allow `/agent create ... --model <model>` when the active runtime schema exposes `model`
- if the schema hides `model`, report clearly that the child will inherit the parent model
- do not add a separate runtime `set-model` command for existing agents

This is the recommended first model-setup implementation because it reuses existing spawn validation instead of inventing a second model-selection path.

### Stage 4B: Preview Before Write

If users need more trust before writing generated files:

- show the generated TOML preview
- allow confirm or cancel
- write only the exact previewed content on confirm

Do not make the preview a full editor in the same slice.

### Stage 4C: Structured Fields Only After Proven Need

Only if scaffold-first usage shows repeated manual edits for the same fields:

- add optional structured fields for:
  - `model`
  - `model_reasoning_effort`
  - `service_tier`
  - `nickname_candidates`
- keep the write scope inside the same `.codex/agents/<slug>.toml` path
- reuse the existing role/config contract instead of inventing a new file format

Do not start here. This is where validation drift and runtime-surface creep begin.

### Stage 4C1: Persist Model In Repo-Local Definitions

Only after scaffold-first definition authoring proves useful:

- allow repo-local definition creation or editing to persist `model = "..."` in `.codex/agents/<slug>.toml`
- keep validation tied to the existing role/config contract
- reuse the same reload boundary: reopen `/agent` or restart

Do not combine this with runtime retargeting of already-running agents.

### Stage 4D: Rename Live Agent Labels

If users need cleaner names for active agent threads:

- add `Rename agent` in the existing `/agent` picker for live threads
- prompt for a new display name
- update only the visible thread nickname/label
- do not modify any role/config file
- keep this separate from definition management

This is the recommended first rename implementation because it is only a thread-metadata UX change.

### Stage 4E: Rename Repo-Local Definitions

Implemented after the repo-local scaffold, proposal, optional-field, and copy paths proved the `.codex/agents` role-file owner.

- add `Rename definition` for repo-local `.codex/agents/<slug>.toml` entries
- update the filename slug
- update the internal `name = ...` field
- reject collisions before write
- keep the reload boundary the same: reopen `/agent` or restart

Do not combine this with live-thread rename in one ambiguous action. Keep `Rename agent` and `Rename definition` separate.

### Stage 4F: Delete Live Agent Threads

If users need to clean up active agents:

- add `Delete agent` in the existing `/agent` picker for live threads
- confirm once
- close and remove the thread from current session visibility
- do not modify any role/config file
- do not add archive or trash semantics in the first slice

This is the recommended first delete implementation because it is only session cleanup over the current thread owner.

### Stage 4G: Delete Repo-Local Definitions

Implemented after repo-local copy and rename proved the `.codex/agents` role-file owner. Keep the first slice file-local:

- add `Delete definition` for repo-local `.codex/agents/<slug>.toml` entries
- confirm before delete
- remove only the targeted file
- keep the reload boundary the same: reopen `/agent` or restart

Do not combine this with live-thread delete in one ambiguous action. Keep `Delete agent` and `Delete definition` separate.

### Stage 4H: Copy Repo-Local Definitions

If users need to duplicate saved definitions:

- add `Copy definition` for repo-local `.codex/agents/<slug>.toml` entries
- prompt for a new name
- duplicate the file to a new slug
- update only the internal `name = ...` field in the first slice
- reject collisions before write
- keep the reload boundary the same: reopen `/agent` or restart

This is the recommended first copy implementation because it is only role-file duplication over the existing owner.

### Stage 4I: Save Live Agents As Definitions

Only if users need to preserve successful live-agent setups:

- add `Save as definition` for live agents
- write a new `.codex/agents/<slug>.toml` scaffold from the live agent source
- keep the same bounded output contract used by other scaffold flows unless stronger fidelity is proven safe

Do not combine this with cross-scope copy or runtime mutation.

## Non-Goals

- Do not create a second sub-agent runtime.
- Do not create a sub-agent-only model registry.
- Do not bypass `ModelsManager` for model selection.
- Do not add provider-qualified alias parsing here.
- Do not add app-server public API surface in the first slice.
- Do not change MCP ownership or introduce a second MCP tool registry.
- Do not duplicate spawn validation logic outside the existing multi-agent owners.
- Do not add hot reload, runtime role mutation, source-precedence UX, or arbitrary file-open editor plumbing to the first proposal-prompt authoring slice.
- Do not introduce alias-only rename persistence or a second naming registry for the first rename slice.
- Do not add a first-slice `/agent set-model <agent> ...` path that mutates already-running agents.
- Do not introduce a second model registry or bypass existing spawn/model validation.
- Do not add trash/archive semantics or a second deletion registry for the first delete slice.
- Do not add a first-slice automatic history-mining, clustering, or repeated-task analytics subsystem for history-based agent proposals.
- Do not add cross-scope copy semantics, archive/version history, or a second copy registry in the first copy slice.

## Verification

For Stage 1:

- Add slash-command tests proving bare `/agent` still opens the picker.
- Add slash-command tests for `/agent create`, `/agent send`, `/agent wait`, `/agent list`, and `/agent close`.
- Add an error test for an unknown `/agent` subcommand.
- Add or update TUI snapshots only if user-visible rendering changes.

For model override behavior:

- Reuse or extend `spawn_agent` tests from the custom sub-agent model ADR.
- Verify hidden metadata mode does not promise model override support.
- Add `/agent create ... --model ...` coverage only for create-time behavior; do not imply runtime retargeting support.

For repo-local definition authoring:

- add scaffold-writer tests proving blank creation writes `name`, required `description`, and non-blank `developer_instructions`
- add proposal-writer tests proving one-line input seeds `name`, `description`, and `developer_instructions`
- add malformed-file repair coverage only if a repair action is implemented; do not weaken role-loader validation
- add copy tests proving copied definitions stay repo-local and keep required fields
- add snapshots only if picker rendering changes

Commands after Rust changes:

```text
cd ontocode-rs
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p ontocode-tui slash_commands
CARGO_BUILD_JOBS=8 just test -p ontocode-core spawn_agent
```

For rename support:

- add picker or slash-command tests proving live-agent rename only changes visible thread metadata
- definition rename is covered by picker selection, file move/update, and collision tests for `.codex/agents/<slug>.toml`
- add snapshots only if the picker rendering changes

For delete support:

- add picker or slash-command tests proving live-agent delete only affects current session thread visibility
- definition delete is covered by picker selection, explicit confirmation, and file-removal tests for `.codex/agents/<slug>.toml`
- add snapshots only if the picker rendering changes

For history-based proposal support:

- add tests proving `Create from this chat` uses only the current thread as source input
- verify generated output remains bounded to `name`, `description`, and `developer_instructions`
- if recent-chat selection is later added, add tests proving only selected threads are included

For copy support:

- add tests proving `Copy definition` duplicates only the targeted `.codex/agents/<slug>.toml` file
- verify the first slice updates only the new slug/path and internal `name = ...` field
- if `Save as definition` is later added, add tests proving the written output stays bounded to the accepted scaffold contract

## Open Questions

- Should `/agent create` accept only explicit flags, or also free-form natural language?
- Should `/agent list` be deterministic from TUI state in Stage 1, or model-mediated through `list_agents`?
- Should persistent templates reuse existing agent role files directly, or introduce a thinner user-facing template command that writes those files later?
- If proposal-prompt authoring is added, should the first version write immediately after generation or require a read-only preview confirmation?
- Should `/agent rename` target only live agent labels at first, with definition rename remaining picker-only until role-file management grows?
- Should create-time model setup remain slash-flag-only at first, or also appear as a picker action before broader definition management exists?
- Should `/agent delete` target only live session threads at first, with definition delete remaining picker-only until role-file management grows?
- Should history-based proposal creation start as `Create from this chat` only, or should selected recent chats be available in the first user-visible slice?
- Should copy start only as repo-local definition duplication, with `Save as definition` for live agents remaining a later picker action?
