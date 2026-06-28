# HarnessX Donor 2000 Useful Ideas Review

status: challenged-narrowed
donor: `tmp/HarnessX-main`
target: Ontocode Rust workspace
date: 2026-06-27

## Current State

This file is donor challenge evidence, not the live implementation queue.

This is not a literal 2000-row backlog. After challenging the donor's tools, harness, gateway, benchmarks, plugins, and meta-harness surfaces against current Ontocode owners, only a very small subset survives as possible current-core extension work.

The earlier [HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md](HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md) remains the authority for donor ideas that are specifically about child-model routing and sub-agent model policy.

There is no implementation-ready task in this file today. Reopen only an exact row when new owner-gap evidence exists.

## Scope

Review HarnessX donor behavior for:

- core harness composition and serialization
- built-in tools, MCP, skill loading, and spawn behavior
- plugins, skill indexes, and extension loading
- API, Lab UI, gateway, and external app surfaces
- replay, validation, meta-harness, and benchmark/evolution loops

The goal is to find useful ideas for current Ontocode owners, not to import a second agent framework, second frontend, benchmark runtime, or training stack.

## Evidence And Caveats

- Donor repo was verified with local CLI `ontoindex analyze ./tmp/HarnessX-main`, which produced `20,670 nodes | 39,332 edges | 596 clusters | 300 flows`.
- Donor OntoIndex query surfaces used in this review:
  - `tool registry built in tools mcp spawn subagent`
  - `harness run loop processors state trajectory replay meta harness`
  - `benchmarks gateway frontend extensions skills plugins evaluation`
- Donor review surfaces:
  - [README.md](../tmp/HarnessX-main/README.md)
  - [docs/architecture.md](../tmp/HarnessX-main/docs/architecture.md)
  - [harnessx/core/harness.py](../tmp/HarnessX-main/harnessx/core/harness.py)
  - [harnessx/tools/spawn_subagent.py](../tmp/HarnessX-main/harnessx/tools/spawn_subagent.py)
  - [harnessx/tools/mcp.py](../tmp/HarnessX-main/harnessx/tools/mcp.py)
  - [harnessx/tools/builtin/__init__.py](../tmp/HarnessX-main/harnessx/tools/builtin/__init__.py)
  - [harnessx/processors/tools/skill_loader.py](../tmp/HarnessX-main/harnessx/processors/tools/skill_loader.py)
  - [harnessx/workspace/skill_index.py](../tmp/HarnessX-main/harnessx/workspace/skill_index.py)
  - [harnessx/plugins/loader.py](../tmp/HarnessX-main/harnessx/plugins/loader.py)
  - [harnessx/api/app.py](../tmp/HarnessX-main/harnessx/api/app.py)
  - [harnessx/meta_harness/agent.py](../tmp/HarnessX-main/harnessx/meta_harness/agent.py)
  - [harnessx/meta_harness/replay.py](../tmp/HarnessX-main/harnessx/meta_harness/replay.py)
  - [harnessx/meta_harness/validate_workflow.py](../tmp/HarnessX-main/harnessx/meta_harness/validate_workflow.py)
  - [benchmarks/README.md](../tmp/HarnessX-main/benchmarks/README.md)
  - [tests/integration/test_full_flow.py](../tmp/HarnessX-main/tests/integration/test_full_flow.py)
  - [tests/integration/test_spawn_tool_usage.py](../tmp/HarnessX-main/tests/integration/test_spawn_tool_usage.py)
  - [tests/integration/test_spawn_sse_events.py](../tmp/HarnessX-main/tests/integration/test_spawn_sse_events.py)
- Current Ontocode owners reviewed with OntoIndex and direct source reads:
  - [ontocode-rs/core/src/tools/planning/native.rs](../ontocode-rs/core/src/tools/planning/native.rs)
  - [ontocode-rs/core/src/tools/handlers/agent_jobs.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs.rs)
  - [ontocode-rs/ext/skills/src/catalog.rs](../ontocode-rs/ext/skills/src/catalog.rs)
  - [ontocode-rs/core-plugins/src/manager.rs](../ontocode-rs/core-plugins/src/manager.rs)
  - [ontocode-rs/tui/src/bottom_pane/app_link_view.rs](../ontocode-rs/tui/src/bottom_pane/app_link_view.rs)
  - [ontocode-rs/app-server/README.md](../ontocode-rs/app-server/README.md)
- Current Ontocode worktree is dirty. This review maps donor ideas to current owners, but it does not reopen any existing tracking file by itself.

## Donor Tool And Harness Inventory

HarnessX is not one feature. It is several overlapping systems:

- Core harness builder and serializer:
  [harnessx/core/harness.py](../tmp/HarnessX-main/harnessx/core/harness.py),
  [harnessx/core/runloop.py](../tmp/HarnessX-main/harnessx/core/runloop.py),
  [harnessx/core/state.py](../tmp/HarnessX-main/harnessx/core/state.py),
  [harnessx/core/trajectory.py](../tmp/HarnessX-main/harnessx/core/trajectory.py)
- Built-in tools and registries:
  [harnessx/tools/builtin/__init__.py](../tmp/HarnessX-main/harnessx/tools/builtin/__init__.py),
  [harnessx/tools/base.py](../tmp/HarnessX-main/harnessx/tools/base.py),
  [harnessx/tools/inmemory.py](../tmp/HarnessX-main/harnessx/tools/inmemory.py),
  [harnessx/tools/mcp.py](../tmp/HarnessX-main/harnessx/tools/mcp.py),
  [harnessx/tools/spawn_subagent.py](../tmp/HarnessX-main/harnessx/tools/spawn_subagent.py)
- Processor pipeline:
  `context`, `memory`, `tools`, `control`, `evaluation`, `observability`, `multi_model`
- Plugin and skill runtime:
  [harnessx/plugins/loader.py](../tmp/HarnessX-main/harnessx/plugins/loader.py),
  [harnessx/plugins/registry.py](../tmp/HarnessX-main/harnessx/plugins/registry.py),
  [harnessx/workspace/skill_index.py](../tmp/HarnessX-main/harnessx/workspace/skill_index.py),
  [harnessx/processors/tools/skill_loader.py](../tmp/HarnessX-main/harnessx/processors/tools/skill_loader.py)
- API and Lab UI:
  [harnessx/api/app.py](../tmp/HarnessX-main/harnessx/api/app.py),
  `harnessx/api/routes/*`,
  `frontend/*`
- IM gateway and second console:
  `gateway/*`, `gateway/console/*`
- Meta-harness validation and replay:
  [harnessx/meta_harness/agent.py](../tmp/HarnessX-main/harnessx/meta_harness/agent.py),
  [harnessx/meta_harness/replay.py](../tmp/HarnessX-main/harnessx/meta_harness/replay.py),
  [harnessx/meta_harness/validate_workflow.py](../tmp/HarnessX-main/harnessx/meta_harness/validate_workflow.py)
- Benchmark and evolution loops:
  `benchmarks/*`, `recipe/*`, `rl/*`

Most of the donor's "2000 ideas" are just consequences of that architecture split. Ontocode should not copy the split.

## Current Ontocode Baseline

Ontocode already has the important current-core owners that most donor ideas would need to extend:

- Tool exposure is already feature-gated and runtime-scoped in [native.rs](../ontocode-rs/core/src/tools/planning/native.rs), including collaboration, goal, and agent-job gating.
- Batch worker orchestration already has one persisted owner in [agent_jobs.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs.rs).
- Skills already have source authority and merged catalog owners in [catalog.rs](../ontocode-rs/ext/skills/src/catalog.rs).
- Plugins already have a real marketplace/load/config owner in [manager.rs](../ontocode-rs/core-plugins/src/manager.rs).
- Rich interface, skill listing, plugin listing, thread lifecycle, and external-action APIs already exist in [app-server/README.md](../ontocode-rs/app-server/README.md).
- External browser/app handoff already has an existing TUI owner in [app_link_view.rs](../ontocode-rs/tui/src/bottom_pane/app_link_view.rs).
- Child-model routing and sub-agent model-policy donor ideas already have their own narrowed review in [HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md](HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md).

## Challenge Result

HarnessX spreads reusable behavior across several extra systems that Ontocode does not need:

- a second harness-composition framework
- a second built-in tool runtime
- a second plugin and skill runtime
- a second gateway/frontend stack
- a benchmark/evolution/training platform
- a meta-harness validator and replay stack

Most of the earlier broad keep list does not survive this challenge. The only valid reuse is the narrow part that is both new and extends current Ontocode owners without adding a parallel runtime.

## Keep Only: New Existing-Core Extensions

| Id | Donor idea | Keep | Existing Ontocode owner | Challenge |
| --- | --- | --- | --- | --- |
| `HARNESSX-K2` | Query-driven full skill content injection can be useful when static skill listings are too weak. | Deferred UX/test candidate only if current skill prompting proves materially ambiguous. | current skill catalog and prompt/context owners | Do not copy HarnessX `ProgressiveSkillLoader` as a second skill runtime. Reuse existing skill owners only. |
| `HARNESSX-K4` | Batch worker orchestration should remain one persisted owner with explicit export/report semantics. | Deferred non-regression candidate only. Reopen only if a real `agent_jobs` gap appears. | `ontocode-rs/core/src/tools/handlers/agent_jobs.rs` | Do not add a donor-style harness-runner/evaluator stack. Extend `agent_jobs` only. |
| `HARNESSX-K5` | Read-only replay and validation reports over recorded runs can be useful as bounded verification artifacts. | Deferred replay/report candidate only if current replay, review, or session-log surfaces are insufficient. | current TUI replay/session-log/review owners | Do not import the donor meta-harness replay system or validator workflow as a second review stack. |
| `HARNESSX-K8` | Donor benchmark and replay artifacts can be useful as bounded regression fixtures when they prove a current-owner gap. | Implementation-ready only as tests if one concrete Ontocode path needs the fixture shape. | current suite/integration/replay/report owners | Use fixture shape only. Do not import benchmark runners, recipes, or evaluation pipelines. |

## Covered: Not New Work

- Skill source authority and merged catalogs already exist in [catalog.rs](../ontocode-rs/ext/skills/src/catalog.rs).
- Current skill listing already has cache invalidation and reload behavior through `skills/list` `forceReload` plus `skills/changed` notifications in [app-server/README.md](../ontocode-rs/app-server/README.md).
- Plugin marketplace/config/load management already exists in [manager.rs](../ontocode-rs/core-plugins/src/manager.rs).
- Current plugin listing already has reload/cache owners through `plugins_for_config_with_force_reload`, cache clearing, and the documented `plugin/list` surface in [manager.rs](../ontocode-rs/core-plugins/src/manager.rs) and [app-server/README.md](../ontocode-rs/app-server/README.md).
- Tool exposure, collaboration gating, goal-tool gating, and agent-job gating already exist in [native.rs](../ontocode-rs/core/src/tools/planning/native.rs).
- Rich interface and listing APIs already exist in [app-server/README.md](../ontocode-rs/app-server/README.md), including `thread/start`, `skills/list`, and `plugin/list`.
- External app/auth handoff already has an existing TUI owner in [app_link_view.rs](../ontocode-rs/tui/src/bottom_pane/app_link_view.rs).
- Child spawn/model-policy donor ideas are already split into [HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md](HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md).

## Blocked Or Deferred

- `HARNESSX-K2` reopens only with a concrete current-skill prompt ambiguity that static catalog surfacing does not solve.
- `HARNESSX-K4` reopens only with a failing `agent_jobs` regression or missing export/report coverage proof.
- `HARNESSX-K5` reopens only with a concrete replay or validation-report gap in current owners.
- `HARNESSX-K8` reopens only when a donor fixture shape proves a real current-core regression path cleanly.

## Unblock Options

These are not open tasks yet. They are the smallest owner-local ways to earn reopen evidence without importing donor architecture.

### Recommended First Reopens

1. `HARNESSX-K4A` proof bundle in `agent_jobs`
   Scope: add focused regression coverage for auto-export, stale-worker failure, resume-from-rollout recovery, and finalization when a worker exits without `report_agent_job_result`.
   Open only if the current `agent_jobs` test surface does not already prove those paths.
2. `HARNESSX-K5A` replay/history proof bundle
   Scope: add focused reducer and UI replay coverage for review-mode entry, hook prompts, MCP tool-call replay, dynamic-tool replay, and compaction replay in the current thread-history and TUI owners.
   Open only if the existing replay tests do not already prove those persisted-item shapes.

### Optional Direct Implementation Reopen

1. `HARNESSX-K5B` read-only turn-item hydration
   Scope: implement the smallest current-owner version of the documented-but-unsupported `thread/turns/items/list` path so persisted full items can be fetched for one turn without resuming the thread.
   Owner: current `app-server` and `app-server-protocol` history surfaces only.
   Do not add a donor replay engine, validation stack, or meta-harness workflow around it.

### Narrowed K2 Reopen Only

1. `HARNESSX-K2A` skill-ambiguity warning
   Scope: when explicit skill resolution intentionally returns none because a name is ambiguous or the structured path is invalid, surface a bounded warning or hint inside the existing skill-resolution UX path.
   This is the only defensible K2 reopen shape right now.
   Do not reopen K2 as broad "full skill content injection" or a second skill-loader runtime.

### Fixture Support Only

1. `HARNESSX-K8A` donor-fixture-backed regression support
   Scope: import only the smallest redacted donor fixture shape needed to prove a real K4 or K5 gap more cleanly than current fixtures.
   K8 should not open as a standalone benchmark, evaluation, or replay-project task.
   Use fixture shape only; no donor runner, leaderboard, or meta-harness code is allowed.

## Recommended Open Order

If this donor is reopened, the preferred order is:

1. `HARNESSX-K4A`
2. `HARNESSX-K5A`
3. `HARNESSX-K5B` only if the proof pass shows a real read-only hydration gap
4. `HARNESSX-K2A` only if a concrete ambiguity case is reproduced
5. `HARNESSX-K8A` only as fixture support for one of the rows above

## Rejected: Wrong Owner Or Parallel Runtime

- Do not add the donor 9-dimension harness architecture as a second configuration/runtime model.
- Do not add donor `HarnessBuilder`/`HarnessConfig` composition as a parallel owner to current Ontocode tool/session/config owners.
- Do not add donor built-in tool registries, `ToolRegistryConfig`, or `InMemoryToolRegistry` as a second core tool runtime.
- Do not add donor MCP client/runtime as a second MCP system under current Ontocode MCP owners.
- Do not add donor `pending_subagents`, async child completion message injection, or second child state protocol from `spawn_subagent.py`; that family already has its own narrowed sub-agent review.
- Do not add donor plugin directory loader, command frontmatter parser, or plugin capability scanner as a second plugin system.
- Do not add donor Lab frontend, Builder page, YAML import/export UI, or gateway console as a second frontend.
- Do not add donor IM gateway channels, channel processors, or prompt-template gateway stack into current core runtime.
- Do not add donor meta-harness validator, replay gate, worker evolution, journal-derived context, or reflect-worker tool as a second review/evolution system.
- Do not add donor benchmark adapters, leaderboard flows, RL bridge, or training recipes as current Ontocode core work.
- Do not add donor sandbox provider stack for Local/Docker/E2B as a second sandbox abstraction.
- Do not add donor workflow plugin engine or office-skill bundles as broad current-core architecture based on this review alone.

## Recommended Reuse Shape

If this donor is revisited, the only valid implementation path is to extend current Ontocode owners:

- tool exposure stays in native tool planning
- skill authority stays in the current skills catalog and listing APIs
- plugin capability/load work stays in core-plugins and app-server listing surfaces
- batch worker orchestration stays in `agent_jobs`
- replay and validation summaries stay in current replay/review/session-log owners
- donor artifacts stay as bounded fixtures and tests only

## Recommended First Slices

If this donor is reopened for real work, the smallest defensible slices are:

1. A focused skill-surface regression if one current skill catalog/prompt/read path is proven ambiguous.
2. A bounded replay/report regression using donor run-shape fixtures if current inspection output proves insufficient.
3. A bounded `agent_jobs` regression only if a concrete export/report path is still uncovered.
4. A donor-fixture-backed regression test only when the shape proves an existing owner gap more cleanly than current fixtures.

Anything larger than that is donor theater, not current-core extension.
