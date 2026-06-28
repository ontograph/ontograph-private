# AgentGym Donor 2000 Useful Ideas Review

status: challenged-proposed
donor: `tmp/AgentGym`
target: Ontocode Rust workspace
date: 2026-06-27

## Current State

This file is donor challenge evidence, not the live implementation queue.

There is no implementation-ready task in this file. Reopen only the exact row whose evidence gate is newly satisfied.

## Scope

Review AgentGym donor behavior for benchmark env loops, structured action formats, batch evaluation harnesses, replay/visualization, and trajectory/eval artifacts.

This is no longer a 2000-idea queue. The earlier review was challenged against current Ontocode owners and reduced to only ideas that are new, extend existing core functionality, and do not introduce an environment runtime, benchmark stack, or second frontend.

## Evidence And Caveats

- Donor repo was verified with local CLI `ontoindex analyze ./tmp/AgentGym`, which produced `15,288 nodes | 21,361 edges | 368 clusters | 300 flows`.
- Donor review surfaces: [README.md](../tmp/AgentGym/README.md), [agentenv/agentenv/controller/env.py](../tmp/AgentGym/agentenv/agentenv/controller/env.py), [agentenv/agentenv/controller/task.py](../tmp/AgentGym/agentenv/agentenv/controller/task.py), [agentenv/agentenv/controller/utils.py](../tmp/AgentGym/agentenv/agentenv/controller/utils.py), [agentenv/agentenv/controller/agent.py](../tmp/AgentGym/agentenv/agentenv/controller/agent.py), [agentenv/agentenv/envs/webshop.py](../tmp/AgentGym/agentenv/agentenv/envs/webshop.py), [agentenv/agentenv/trainer/distributed_evaluator.py](../tmp/AgentGym/agentenv/agentenv/trainer/distributed_evaluator.py), [agentenv/utils/distributed_eval_task.py](../tmp/AgentGym/agentenv/utils/distributed_eval_task.py), [env-visualization/README.md](../tmp/AgentGym/env-visualization/README.md), [env-visualization/src/shared/services/baseClient.js](../tmp/AgentGym/env-visualization/src/shared/services/baseClient.js).
- Current Ontocode MCP index is fresh at commit `5edde24a78efe0f10bc710936dfa228427ab7fd1`, with a dirty worktree caveat.
- Current Ontocode owners reviewed with OntoIndex:
  - `run_agent_job_loop`, `SpawnAgentsOnCsvHandler`, and existing `agent_jobs` suite/spec coverage
  - `parse_mcp_tool`, `mcp_call_tool_result_output_schema`, `CallToolResult.as_function_call_output_payload`, and code-mode structured-output tests
  - TUI replay buffer, thread replay, and replay-mode tests under `tui/src/app/tests.rs` and `tui/src/chatwidget/tests/history_replay.rs`

## Current Ontocode Baseline

Ontocode already has the core shape that matters from this donor:

- `spawn_agents_on_csv` already provides the current batch-job owner, with persisted job state, CSV export, stop handling, and suite coverage for run/export, id dedupe, and stop semantics.
- Structured tool outputs already have existing owners in `mcp_tool.rs`, `protocol/src/models.rs`, and `code-mode/src/description.rs`, plus context and suite tests for structured output preservation and truncation.
- TUI transcript replay is already an existing owner, with replay buffer, thread replay, review-mode replay, and snapshot-preservation tests.
- Current architecture already keeps external runtimes outside core; the donor's env-server protocol and JS visualization client do not justify new in-core owners.

## Challenge Result

The previous draft kept several families that do not pass the stricter bar of "new and extended current core solutions":

- `ENV_PROTOCOL` is not current core. It is an external benchmark/env contract.
- `EXTERNAL_COMPANION` is not current core. It belongs outside the current Rust runtime.
- Most `TASK_HARNESS` rows were donor benchmark plumbing, not narrow extensions to the existing `agent_jobs` owner.
- One `REPLAY` row and much of the `ACTION_FORMAT` discussion were already covered by current replay and structured-output owners/tests.

After challenge, only the rows below remain worth keeping, and even those are not implementation-ready by default.

## Keep Only: New Existing-Core Extensions

| Id | Donor idea | Keep | Existing Ontocode owner | Challenge |
| --- | --- | --- | --- | --- |
| `AGENTGYM-K1` | Batch task execution should stay on one persisted job owner with explicit per-item completion, stop, and export semantics. | Deferred non-regression candidate only. Reopen only if a real `agent_jobs` gap appears in multi-item completion/stop/export behavior. | `core/src/tools/handlers/agent_jobs.rs`, `spawn_agents_on_csv.rs`, `state/src/runtime/agent_jobs.rs`, `core/tests/suite/agent_jobs.rs` | Do not import AgentGym task registries, env loops, or evaluator classes. If anything is missing, extend the current `agent_jobs` owner only. |
| `AGENTGYM-K2` | One schema source can drive multiple bounded action renderings such as text guidance, function-calling, and code-mode views. | Deferred UX/test candidate only if current tool descriptions prove materially ambiguous across existing owners. | `tools/src/mcp_tool.rs`, `protocol/src/models.rs`, `code-mode/src/description.rs`, existing code-mode/tool tests | Do not add a second action abstraction. Keep any future improvement as a rendering/description change over the current structured tool schema owners. |
| `AGENTGYM-K3` | Replay and inspection should remain read-only and separate from live execution control. | Deferred replay UX candidate only if current TUI replay is proven insufficient for bounded inspection. | `tui/src/app/resize_reflow.rs`, `tui/src/app/tests.rs`, `tui/src/chatwidget/tests/history_replay.rs`, `tui/src/app/session_lifecycle.rs` | Do not import the donor Vue frontend or a second control surface. Extend current replay/thread owners only. |
| `AGENTGYM-K4` | Donor trajectory/eval JSON shapes can be useful as bounded regression fixtures for export/report/replay tests. | Implementation-ready only as tests if a concrete uncovered regression path is found. | `core/tests/suite/agent_jobs.rs`, replay tests, export/reporting owners | Use the fixture shape only. Do not import datasets, training scope, or raw donor history into runtime behavior. |

## Covered: Not New Work

- `spawn_agents_on_csv` already covers the broad batch-job shape the donor evaluator points at, including output export and stop behavior.
- Structured tool output preservation, raw call-tool result handling, and bounded code-mode rendering already exist in current owners and tests.
- Thread replay, replay buffers, review-mode replay, and transcript-order replay are already covered in current TUI owners and tests.

## Blocked Or Deferred

- `AGENTGYM-K1` reopens only with a failing test or concrete missing-coverage proof in the existing `agent_jobs` run/stop/export path.
- `AGENTGYM-K2` reopens only with a concrete current-owner ambiguity in how one tool schema is rendered across existing text/structured/code-mode surfaces.
- `AGENTGYM-K3` reopens only with a concrete bounded replay-inspection need that current thread replay and review-mode owners do not serve.
- `AGENTGYM-K4` reopens only with a failing export/report/replay test that the donor JSON fixture shape can prove cleanly.

## Rejected: Wrong Owner Or Not Current Core

- Do not add the donor create/observe/step/reset env protocol as a new core runtime surface.
- Do not add env clients, per-environment task registries, or benchmark routing inside Ontocode core.
- Do not add `APIAgent`, vLLM cache/runtime logic, or a second provider/runtime wrapper.
- Do not add `DistributedEvaluator`, `BCTrainer`, `AgentEvolTrainer`, DPO, or any training/eval framework classes as current core work.
- Do not add the donor Vue visualization app, JS `BaseEnvClient`, or another replay/control frontend.
- Do not keep external-companion rows in this file. The user asked for current core only.
- Do not widen prompt packs, benchmark-local action grammars, or code-as-action parsing into a second tool/runtime abstraction.

## Recommended Reuse Shape

If this donor is revisited, the only valid implementation path is to extend existing core owners:

- batch job semantics stay in `agent_jobs`
- structured action rendering stays in current tool-schema and code-mode owners
- replay stays in current TUI replay/thread owners
- any donor-derived artifacts stay as bounded tests and fixtures

No implementation is currently ready from this file unless one of the exact reopen gates above is satisfied.

## Several Implementation Solutions

These are implementation options over the surviving rows only. They are ordered from smallest and safest to broadest. All of them must stay inside current owners.

### Solution 1: Fixture-First Regression Bundle

Scope:
- `AGENTGYM-K4` first
- then `AGENTGYM-K1` only if the fixtures expose a real current-owner gap

Current owner:
- `ontocode-rs/core/tests/suite/agent_jobs.rs`
- replay tests under `ontocode-rs/tui/src/app/tests.rs` and `ontocode-rs/tui/src/chatwidget/tests/history_replay.rs`

What to implement:
- add a tiny checked-in AgentGym-shaped fixture pack:
  - one `AgentEval`-style item list
  - one `AgentTraj`-style conversation trace
  - one expected exported-row shape for `spawn_agents_on_csv`
- add focused tests that reuse those shapes to validate:
  - export row stability
  - replay order stability
  - bounded result/report rendering

Why this is the best first move:
- smallest diff
- pure current-core work
- gives hard evidence for whether `AGENTGYM-K1` is real or already covered

What not to add:
- no benchmark loader
- no donor dataset import path
- no runtime env loop

Stop condition:
- once the fixture-driven tests either pass or expose one concrete current-owner failure

### Solution 2: Agent-Jobs Non-Regression Hardening

Scope:
- `AGENTGYM-K1`

Current owner:
- `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`
- `ontocode-rs/core/src/tools/handlers/agent_jobs/spawn_agents_on_csv.rs`
- `ontocode-rs/state/src/runtime/agent_jobs.rs`
- `ontocode-rs/core/tests/suite/agent_jobs.rs`

What to implement:
- start with tests around the existing job loop using AgentGym-like multi-step task shapes
- only if a test fails, make the smallest production fix inside current owners for:
  - per-item completion/failure accounting
  - stop-after-first or stop-after-error semantics
  - export shape stability after partial completion

Why this is safe:
- current repo already has the batch-job engine
- donor value here is mostly a challenge to the current regression envelope, not to the architecture

Tradeoff:
- likely produces tests only
- if it produces code, the code should be a small fix in `agent_jobs`, not a new evaluator abstraction

Stop condition:
- no new runner, evaluator, or task registry

### Solution 3: Schema-Driven Rendering Alignment

Scope:
- `AGENTGYM-K2`

Current owner:
- `ontocode-rs/tools/src/mcp_tool.rs`
- `ontocode-rs/code-mode/src/description.rs`
- `ontocode-rs/protocol/src/models.rs`
- existing context and code-mode suite tests

What to implement:
- strengthen the guarantee that one parsed tool schema drives all current renderings:
  - normal structured tool output
  - code-mode declaration/sample output
  - typed output rendering for `CallToolResult`
- likely first slice:
  - add tests that one tool schema with structured output renders consistent text and code-mode examples
- optional second slice only if tests show drift:
  - extract one owner-local helper so code-mode sample rendering and structured-output typing consume the same normalized schema facts

Why this is useful:
- this is the cleanest direct reuse of AgentGym's "same action, multiple bounded renderings" idea
- it improves existing core behavior without adding a second action model

What not to add:
- no new action abstraction
- no donor-style adapter hierarchy
- no benchmark-specific parser for code-as-action

Stop condition:
- keep the change inside schema/rendering owners; do not invent a new tool-description system

### Solution 4: Read-Only Replay Inspection Upgrade

Scope:
- `AGENTGYM-K3`

Current owner:
- `ontocode-rs/tui/src/app/resize_reflow.rs`
- `ontocode-rs/tui/src/app/session_lifecycle.rs`
- `ontocode-rs/tui/src/app/tests.rs`
- `ontocode-rs/tui/src/chatwidget/tests/history_replay.rs`

What to implement:
- keep replay read-only, but make bounded inspection easier inside the current TUI:
  - better replay banner or status summary
  - explicit turn-count or thread-source summary while replaying
  - tests proving replay metadata does not mutate live thread control state

Why this is acceptable:
- it extends a current owner
- it reuses the donor's replay/debug lesson without importing the donor frontend

Tradeoff:
- this is the least urgent option because current replay already exists
- should land only if there is a concrete review/inspection pain point

What not to add:
- no new frontend
- no JS client
- no live stepping controls inside replay mode

Stop condition:
- replay remains read-only and thread-local

## Recommended Implementation Order

If this donor is reopened for real work, the most defensible order is:

1. Solution 1
2. Solution 2 only if Solution 1 exposes a real `agent_jobs` gap
3. Solution 3 if current schema/rendering drift is proven
4. Solution 4 only after a concrete replay-inspection pain point is shown

## Recommended First Slice

The smallest slice that actually earns its keep is:

- add one tiny AgentGym-shaped fixture pack
- add one `agent_jobs` regression test
- add one replay-order or replay-summary test

That is enough to tell whether this donor produces real current-core work or just more review prose.
