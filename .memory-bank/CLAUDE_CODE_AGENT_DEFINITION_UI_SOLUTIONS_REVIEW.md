# Claude Code Agent Definition UI Solutions Review

status: challenged-proposed
date: 2026-06-24
donor: `tmp/claude-code-main`
target: Ontocode Rust workspace
method: OntoIndex-grounded owner review plus donor source inspection

## Executive Decision

Ontocode should support Claude-style named agent definitions through the existing role/config path, not through a new donor-style agent registry.

The safe product shape is:

1. show existing roles/agent definitions in the current `/agent` and picker UX
2. add the smallest UI flow that writes a role file scaffold in the existing repo-local agent config location
3. reuse that role by passing `agent_type` during `spawn_agent`, so the saved prompt/instructions become the child agent's `developer_instructions`

Do not start by copying Claude's `loadAgentsDir.ts` precedence system, plugin agent loader, or full editor/runtime registry.
Do not start with a structured wizard, dual-scope writes, or hot-reload requirements.

## Problem To Solve

The user goal is narrower than "build Claude Code agents":

- create an agent definition in UI
- give it a stable name
- reuse it later in prompts and `/agent` workflows

Ontocode already has the runtime needed to run named sub-agents. The missing piece is a safe authoring and discovery layer.

## Challenge Verdict

Keep the central idea, but narrow the implementation.

The previous draft was right to reuse the existing role/config owner and reject a donor-style registry. It overreached by making a full wizard the recommended first implementation.

OntoIndex evidence points to a lazier and safer path:

- `load_agent_roles` already discovers standalone role files
- `apply_role_to_config` already makes named roles reusable during spawn
- external-agent import already writes role files instead of maintaining a second registry

That means the first useful UI creation flow does not need a structured field editor. It only needs:

- create a minimal `.codex/agents/<slug>.toml` scaffold
- open that file in the user's editor
- let the existing config/role path do the rest

This satisfies the user request with less new code and less new validation surface.

## Donor Findings

The donor splits this feature into three distinct concerns:

### 1. Registry and source precedence

Donor files:

- `tmp/claude-code-main/src/tools/AgentTool/loadAgentsDir.ts`
- `tmp/claude-code-main/src/tools/AgentTool/builtInAgents.ts`

What the donor does:

- loads built-in, plugin, user, project, policy, and flag agents
- merges them by precedence
- treats agent definitions as a first-class registry

### 2. Interactive management UI

Donor files:

- `tmp/claude-code-main/src/components/agents/AgentsMenu.tsx`
- `tmp/claude-code-main/src/components/agents/AgentEditor.tsx`
- `tmp/claude-code-main/src/components/agents/new-agent-creation/CreateAgentWizard.tsx`

What the donor does:

- lists agents by source
- creates agents through a wizard
- edits tools, model, and color
- treats restart/reload as part of the edit flow

### 3. Prompt reuse

The donor stores prompt/instruction text inside the agent definition and later reuses the definition by name when spawning or selecting an agent.

That reuse behavior is worth keeping. The registry architecture is not.

## OntoIndex Grounding

OntoIndex and local source review show Ontocode already has the right owners.

### Existing runtime owner

- `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs`
- `ontocode-rs/core/src/agent/control.rs`

Meaning:

- named sub-agents already run through `spawn_agent`
- role selection already exists as `agent_type`
- runtime/model validation already exists

### Existing role/config owner

- `ontocode-rs/core/src/config/agent_roles.rs`
- `ontocode-rs/core/src/agent/role.rs`

Meaning:

- roles already load from config layers and discovered `agents/` directories
- standalone repo-local `.codex/agents/*.toml` files already merge with precedence
- `apply_role_to_config` already turns a named role into effective `developer_instructions`, model, reasoning, and tier behavior

### Existing import proof

- `ontocode-rs/external-agent-migration/src/lib.rs`

Meaning:

- imported external subagents already become local `.toml` role files
- the repo already has a sanctioned path from "external agent definition" to "local role file"
- UI-created definitions should target the same file format and home

### Existing UI/control owner

- `ontocode-rs/tui/src/bottom_pane/chat_composer.rs`
- `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs`
- `ontocode-rs/tui/src/app/agent_navigation.rs`

Meaning:

- `/agent` already exists
- the picker already exists
- agent definition authoring should extend this path, not add a second manager surface first

## Key Architectural Fact

Ontocode already has prompt reuse for named agents.

The reuse primitive is not "load an agent definition into a registry". The reuse primitive is:

- define a role file
- reference that role by `agent_type`
- let `apply_role_to_config` inject the stored instructions into the spawned child config

That is the core reason a donor-style registry copy would be redundant.

## Solution Options

## S0. Read-Only Agent Definition Discovery

Summary:

- surface existing built-in and configured roles in `/agent` and the picker
- allow selection by name
- no write path yet

What to build:

- picker section for available roles
- `/agent create <task> --agent-type <name>` or the current owner-local equivalent
- minimal role details view: name, description, configured model when already derivable from the current role file path

Where it lives:

- TUI picker and slash-command path
- existing role/config readers only

Pros:

- lowest risk
- no file-write UX yet
- immediately validates whether users actually use named roles

Cons:

- does not satisfy the full "create in UI" ask
- still requires manual file creation outside the UI

Verdict:

- good Stage 1
- insufficient as the final answer to the user request

## S1. Repo-Local Agent Definition Scaffold Writing `.codex/agents/*.toml`

Summary:

- add the smallest TUI creation flow to create a role file under `.codex/agents/`
- reuse the saved definition by passing `agent_type=<name>` during spawn

What to build:

- "Create agent definition" action in the existing `/agent` picker
- prompt only for the minimum inputs needed to create a valid role file:
  - `name`
  - optional one-line `description`
- write file to `.codex/agents/<slug>.toml`
- seed it with a minimal valid scaffold
- open the file in the user's editor for the real instructions
- require reopen/restart to pick up the new role unless a cheap existing refresh path is proven

Example file shape:

```toml
name = "researcher"
description = "Research role for focused codebase investigation"
developer_instructions = """
Fill in the instructions for this role.
"""
```

How prompt reuse works:

- user creates `researcher`
- edits the scaffolded file
- later uses `/agent create bug triage --agent-type researcher`
- runtime maps `agent_type=researcher`
- `apply_role_to_config` loads the stored instructions

Pros:

- satisfies the actual user request
- uses the existing role/config owner
- matches the current external-agent import format
- keeps definitions repo-local and reviewable
- avoids building field-by-field role serialization first

Cons:

- still needs safe write-path UX
- still needs duplicate-name handling
- requires an explicit refresh boundary in v1

Verdict:

- recommended primary solution
- best balance of usefulness and architecture safety

## S2. Repo-Local Structured Wizard

Summary:

- extend S1 later with structured optional fields instead of dropping users straight into the editor

What to build:

- optional fields after the scaffold path is proven:
  - `model`
  - `model_reasoning_effort`
  - `service_tier`
  - `nickname_candidates`
- serializer shared with the current role-file contract
- no new file format

Pros:

- better UX once real usage exists
- still reuses the existing config-layer owner

Cons:

- more validation surface
- easier to drift from the existing parser if built too early

Verdict:

- acceptable follow-up
- not first slice

## S3. Dual-Scope Creation: Repo-Local Or User-Local

Summary:

- same creation flow as S1 or S2, but with scope choice:
  - repo-local: `.codex/agents/`
  - user-local: `$CODEX_HOME/agents/`

What to build:

- one extra scope step
- source labeling in picker and role display
- explicit collision and precedence messaging

Pros:

- useful for personal reusable agents versus project-specific agents
- still reuses the existing config-layer owner

Cons:

- adds precedence UX without unblocking the core request
- needs clearer source and collision messaging
- should not precede repo-local proof

Verdict:

- defer
- not needed for first user-visible value

## S4. Full Donor-Style Agent Registry And Editor

Summary:

- copy the donor model: built-in/plugin/user/project/policy registry, precedence resolution, inline editor, and dedicated loader

Pros:

- closest functional parity with donor

Cons:

- duplicates the current role/config owner
- introduces a second definition system next to `agent_roles`
- creates new precedence and reload rules
- expands into plugins, policy, and tool allowlists before the base workflow is proven

Verdict:

- reject for first implementation
- only revisit if the existing role/config owner is proven structurally insufficient

## Recommended Plan

Bounded manager-loop outcome:

- promote `S0` to the only active slice
- keep `S1` and `S2` pending behind it
- reject `S3` and `S4` for the current owner set

### Stage A: Role Discovery UX

- expose current roles in `/agent` and picker
- display description and locked model/reasoning metadata
- support `/agent create <task> --agent-type <name>` or a thin alias that maps explicitly to `agent_type`

### Stage B: Create Agent Definition In UI

- add "Create agent definition" to the picker
- ask for only the minimum fields needed to create a valid role file
- write `.codex/agents/<slug>.toml`
- validate name collisions against loaded roles
- keep the file format identical to the current standalone role-file contract
- open the resulting file in the editor instead of building an in-TUI role editor first

### Stage C: Reuse In Prompts

- allow role insertion from picker into the compose flow
- support role-aware slash creation:
  - `/agent create review logging --agent-type reviewer`
  - `/agent create explore auth --agent-type researcher`
- optionally add a "copy spawn snippet" action that inserts a ready `/agent create ... --agent-type <name>` command

### Stage D: Edit And Source Visibility

- add read-only source display first
- editing can start as "open file in editor"
- only later consider field-level inline editing

## Professional Narrowing Rules

The implementation should not:

- add a second agent-definition registry
- copy donor `loadAgentsDir.ts` precedence semantics
- add config writes outside the current role-file homes
- add public app-server APIs in the first slice
- add model-visible agent-definition payloads to conversation context
- create a second prompt-template system separate from role files

## Main Technical Risks

### Reload behavior

Question:

- can TUI safely refresh loaded role definitions without a full restart?

Narrow answer:

- first ship with explicit "created successfully; reopen or restart to use immediately" behavior
- do not build dynamic config reload just for this slice unless an existing cheap refresh path is already present

### Validation drift

Question:

- can UI accidentally write files that the current parser rejects?

Required answer:

- use the existing role parser/validator shape
- centralize serialization in one owner-local helper

### Name and precedence confusion

Question:

- what happens when repo-local and user-local roles share a name?

Required answer:

- display the effective winner
- do not invent new precedence; follow the current config-layer rules already exercised by `config_tests.rs`

### Terminology drift

Question:

- should the UI introduce `role` as a new first-class runtime term?

Required answer:

- keep `agent_type` as the underlying owner term
- any `role` wording must be an explicit UI alias, not a second contract

## Test Expectations If Implemented

- config/role tests for new file-writing helpers
- TUI tests for picker action and slash reuse flow
- slash-command tests for `agent_type`-based reuse
- one integration test proving a scaffolded role is later reusable through `spawn_agent`

## Final Recommendation

The robust solution is not "Claude-style agent definitions". It is "UI-authored role files over the existing Ontocode sub-agent runtime."

That gives the user the behavior they want:

- name an agent
- save its instructions
- reuse it later in prompts and `/agent` flows

without paying for a second registry, second precedence system, or second prompt owner.

The first implementation should be even narrower than the earlier draft:

- discover existing roles
- scaffold a repo-local role file
- open it in the editor
- reuse it through `agent_type`

Anything larger should wait until that path proves real demand.
