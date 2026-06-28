# AgentBench Donor 2000 Useful Ideas Review

status: challenged-closed-no-dispatch
donor: `tmp/AgentBench-main`
target: Ontocode Rust workspace
date: 2026-06-27

## Current State

This file is donor challenge evidence, not a live implementation queue.

This is not a literal 2000-row backlog. After challenging the first pass against current Ontocode owners and current coverage, no surviving implementation row remains. No production implementation is open from this file today.

If this donor is revisited, reopen only an exact row whose evidence gate becomes true.

## Scope

Review AgentBench donor behavior for:

- task harness and session orchestration
- task-local function-calling loops
- sample status, output, and result schemas
- worker assignment and resume logic
- benchmark output aggregation and config layering
- environment and Docker controller surfaces

The goal is to find useful ideas for current Ontocode owners, not to import a second benchmark runtime, task controller, scheduler, agent server, or environment stack.

## Evidence And Caveats

- Donor repo was verified with local CLI `ontoindex analyze ./tmp/AgentBench-main`, which produced `1,666 nodes | 2,832 edges | 95 clusters | 89 flows`.
- Donor OntoIndex query used for this review:
  - `task session tool function calling worker controller assignment max flow replay output schema benchmark docker environment`
- Donor review surfaces:
  - [README.md](../tmp/AgentBench-main/README.md)
  - [docs/Introduction_en.md](../tmp/AgentBench-main/docs/Introduction_en.md)
  - [src/assigner.py](../tmp/AgentBench-main/src/assigner.py)
  - [src/client/agent.py](../tmp/AgentBench-main/src/client/agent.py)
  - [src/client/task.py](../tmp/AgentBench-main/src/client/task.py)
  - [src/configs.py](../tmp/AgentBench-main/src/configs.py)
  - [src/analysis.py](../tmp/AgentBench-main/src/analysis.py)
  - [src/typings/output.py](../tmp/AgentBench-main/src/typings/output.py)
  - [src/typings/status.py](../tmp/AgentBench-main/src/typings/status.py)
  - [src/server/tasks/dbbench/task.py](../tmp/AgentBench-main/src/server/tasks/dbbench/task.py)
  - [src/server/tasks/knowledgegraph/task.py](../tmp/AgentBench-main/src/server/tasks/knowledgegraph/task.py)
  - [src/server/tasks/webshop/task.py](../tmp/AgentBench-main/src/server/tasks/webshop/task.py)
- Current Ontocode index was rechecked with OntoIndex MCP and is fresh at commit `5edde24a78efe0f10bc710936dfa228427ab7fd1`, with a dirty worktree caveat.
- Current Ontocode owners reviewed with OntoIndex and direct source reads:
  - [ontocode-rs/core/src/tools/handlers/agent_jobs.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs.rs)
  - [ontocode-rs/core/tests/suite/agent_jobs.rs](../ontocode-rs/core/tests/suite/agent_jobs.rs)
  - [ontocode-rs/state/src/runtime/agent_jobs.rs](../ontocode-rs/state/src/runtime/agent_jobs.rs)
  - [ontocode-rs/tools/src/mcp_tool.rs](../ontocode-rs/tools/src/mcp_tool.rs)
  - [ontocode-rs/tui/src/chatwidget/tests/history_replay.rs](../ontocode-rs/tui/src/chatwidget/tests/history_replay.rs)

## Donor Reality Check

The donor `main` branch is smaller and narrower than the older docs imply.

- [README.md](../tmp/AgentBench-main/README.md) now centers the repository on the function-calling AgentBench FC path layered on top of `AgentRL`, with Dockerized task workers and benchmark setup.
- [docs/Introduction_en.md](../tmp/AgentBench-main/docs/Introduction_en.md) still documents the older three-part controller, worker, and agent-server architecture in detail.
- The actual reusable `main` code is mostly:
  - task adapters over `agentrl.worker.task.Task` and `Session`
  - per-task tool-call loops
  - typed status and output models
  - max-flow assignment in [src/assigner.py](../tmp/AgentBench-main/src/assigner.py)
  - config import/default/overwrite merging in [src/configs.py](../tmp/AgentBench-main/src/configs.py)
  - benchmark result aggregation in [src/analysis.py](../tmp/AgentBench-main/src/analysis.py)

That distinction matters. The donor is not a self-contained reusable current-core harness for Ontocode. Most of its value is challenge evidence against existing owners, not importable architecture.

## Donor Tool And Harness Inventory

AgentBench `main` currently spreads behavior across these donor-specific layers:

- task controller and worker HTTP loop described in [docs/Introduction_en.md](../tmp/AgentBench-main/docs/Introduction_en.md)
- thin agent and task client interfaces in [src/client/agent.py](../tmp/AgentBench-main/src/client/agent.py) and [src/client/task.py](../tmp/AgentBench-main/src/client/task.py)
- max-flow sample assignment and resume/output bookkeeping in [src/assigner.py](../tmp/AgentBench-main/src/assigner.py)
- explicit sample status and output schemas in [src/typings/status.py](../tmp/AgentBench-main/src/typings/status.py) and [src/typings/output.py](../tmp/AgentBench-main/src/typings/output.py)
- task-local function-calling control loops in:
  - [src/server/tasks/dbbench/task.py](../tmp/AgentBench-main/src/server/tasks/dbbench/task.py)
  - [src/server/tasks/knowledgegraph/task.py](../tmp/AgentBench-main/src/server/tasks/knowledgegraph/task.py)
  - [src/server/tasks/webshop/task.py](../tmp/AgentBench-main/src/server/tasks/webshop/task.py)
- benchmark aggregation and config composition in [src/analysis.py](../tmp/AgentBench-main/src/analysis.py) and [src/configs.py](../tmp/AgentBench-main/src/configs.py)
- environment-controller and Docker/manual runtime delegation through `agentrl.worker.environment`

Most of those layers are benchmark-specific. Ontocode should not copy them into core.

## Current Ontocode Baseline

Ontocode already has the current-core owners that matter for the few reusable lessons in this donor:

- `run_agent_job_loop` already owns persisted batch worker orchestration, recovery, item finalization, export, timeout handling, and cancellation flow in [agent_jobs.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs.rs).
- `StateRuntime` already owns persisted agent-job and item state in [state/src/runtime/agent_jobs.rs](../ontocode-rs/state/src/runtime/agent_jobs.rs).
- `core/tests/suite/agent_jobs.rs` already covers run/export, id dedupe, stop behavior, and wrong-thread rejection.
- `parse_mcp_tool` and `mcp_call_tool_result_output_schema` already own normalized structured tool output shaping in [mcp_tool.rs](../ontocode-rs/tools/src/mcp_tool.rs).
- TUI replay and history proof already exist in current replay tests, including the `history_replay` module.

## Challenge Result

Most apparent AgentBench ideas fail the current-core filter because they are really one of these:

- benchmark controller and worker runtime
- second scheduler based on donor max-flow assignment
- task-local tool protocol as a second tool runtime
- environment and Docker orchestration
- benchmark scoring and leaderboard output
- legacy architecture documentation that does not match the real `main` code

The only defensible reuse is future proof and fixture evidence over current Ontocode owners. That is not enough to keep an open donor row by itself.

## No Surviving New Current-Core Extensions

After challenge, none of the prior kept rows survives as a distinct current-core extension:

- `AGENTBENCH-K1` does not stay open. Current `agent_jobs` already owns persisted batch execution, export, item finalization, and stop behavior, and current suite coverage already exercises run/export, output schema, id dedupe, stop, and wrong-thread rejection. Without a failing proof around resume or stale-item recovery, this is only a future regression gate, not a surviving donor row.
- `AGENTBENCH-K2` does not stay open. Donor status and result enums are fixture shapes only. They do not justify a new row unless a current result or schema owner actually fails.
- `AGENTBENCH-K3` does not stay open. Donor histories and result objects are only support fixtures for replay/export proof. They are not a distinct extension family.

The valid donor lesson is narrower than a kept row:

- if a current owner fails, donor shapes may be useful as bounded regression fixtures
- until that happens, there is nothing to dispatch

## Covered: Not New Work

- Current `agent_jobs` already owns the persisted batch-runner shape the donor assigner points toward.
- Structured tool output shaping already exists in current MCP tool schema owners.
- Current replay/history owners already provide read-only transcript verification without donor task controllers.
- Ontocode already has stronger current-core persistence boundaries than the donor's filesystem-based output directories.

## Closed By Challenge

No implementation is ready from this file today.

The only valid reopen gates are:

- current `agent_jobs` proof shows a real missing path in resume, stale-item recovery, or export finalization
- current structured output or reporting owners fail on a concrete status/result shape that a donor fixture proves better than existing fixtures
- current replay/export/history tests fail on a concrete persisted trajectory or result shape that donor samples can express cleanly

If none of those gates is newly satisfied, the correct next decision remains no-dispatch.

## Rejected: Wrong Owner Or Parallel Runtime

- Do not add the donor task controller and task worker HTTP architecture to Ontocode core.
- Do not add donor `AgentClient` and `TaskClient` as a second agent/tool/session protocol.
- Do not add donor max-flow assignment as a second scheduler beside current `agent_jobs`.
- Do not add donor function-calling task loops as a second tool runtime or second task execution framework.
- Do not add donor Docker, Redis, Freebase, WebShop, ALFWorld, DB, KG, or OS environment stacks into current core runtime.
- Do not add donor benchmark aggregation, model score reporting, or leaderboard logic as current-core work.
- Do not treat the legacy docs architecture as an implementation target when the current `main` branch is narrower.
- Do not import donor config import/default/overwrite layering as a parallel config owner for Ontocode.

## Recommended Reuse Shape

If this donor is revisited, the only valid implementation path is:

- keep batch semantics inside current `agent_jobs`
- keep structured output shaping inside current MCP tool and result owners
- keep replay and history proof inside current TUI replay owners
- use donor sample/status/history/result shapes only as bounded regression fixtures

No production runtime, scheduler, benchmark stack, or environment harness should be opened from this donor review.
