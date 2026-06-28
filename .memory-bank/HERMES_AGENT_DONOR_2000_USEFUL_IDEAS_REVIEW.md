# Hermes Agent Donor 2000 Useful Ideas Review

status: challenged-narrowed
donor: `tmp/hermes-agent-main`
target: Ontocode Rust workspace
date: 2026-06-27

## Current State

This is a challenge pass over the fresh donor review against the current `main` checkout of Hermes Agent.

This is not a literal 2000-row backlog. Hermes is a large integrated agent product, so most apparent "ideas" are just consequences of its separate tool, plugin, provider, gateway, memory, scheduler, and app stacks.

After challenging the first-pass keep list against current Ontocode owners, current test coverage, and the older Hermes closure trail, only one family remains as new current-core extension work.

This file is donor evidence only. It does not open implementation work by itself.

## Scope

Review Hermes donor behavior for:

- built-in tools and tool registration
- subagent delegation and worker isolation
- background process and MCP runtimes
- skills, session search, memory, and provider layers
- plugin loading, gateway, cron, ACP, and desktop/web surfaces

The goal is to keep only ideas that extend current Ontocode owners. The goal is not to import a second agent framework, second gateway, second memory backend, or second provider/plugin runtime.

## Evidence And Caveats

- Donor repo was verified with local CLI `ontoindex analyze ./tmp/hermes-agent-main`, which produced `203,944 nodes | 388,008 edges | 4,888 clusters | 300 flows`.
- Donor OntoIndex CLI queries used in this review:
  - `ontoindex query -r hermes-agent -l 8 "tool registry mcp delegate subagent session search skills plugins providers cron gateway harness"`
  - `ontoindex query -r hermes-agent -l 8 "plugin runtime provider profile tool self registration process registry session search memory manager cron scheduler"`
  - `ontoindex context -r hermes-agent MemoryManager`
- Current Ontocode MCP/graph review used:
  - semantic search over `native tool planning agent_jobs skills catalog plugin manager app-server thread turns items list replay history subagent notifications`
- Historical Hermes closure evidence reviewed:
  - [ADR_HERMES_DONOR_CORE_EXTENSION_SOLUTIONS.md](ADR_HERMES_DONOR_CORE_EXTENSION_SOLUTIONS.md)
  - [HERMES_DONOR_CORE_EXTENSION_TRACKING.md](HERMES_DONOR_CORE_EXTENSION_TRACKING.md)
- Donor review surfaces:
  - [README.md](../tmp/hermes-agent-main/README.md)
  - [website/docs/developer-guide/architecture.md](../tmp/hermes-agent-main/website/docs/developer-guide/architecture.md)
  - [website/docs/developer-guide/tools-runtime.md](../tmp/hermes-agent-main/website/docs/developer-guide/tools-runtime.md)
  - [website/docs/developer-guide/provider-runtime.md](../tmp/hermes-agent-main/website/docs/developer-guide/provider-runtime.md)
  - [tools/registry.py](../tmp/hermes-agent-main/tools/registry.py)
  - [tools/delegate_tool.py](../tmp/hermes-agent-main/tools/delegate_tool.py)
  - [tools/process_registry.py](../tmp/hermes-agent-main/tools/process_registry.py)
  - [tools/mcp_tool.py](../tmp/hermes-agent-main/tools/mcp_tool.py)
  - [agent/memory_manager.py](../tmp/hermes-agent-main/agent/memory_manager.py)
  - [tools/session_search_tool.py](../tmp/hermes-agent-main/tools/session_search_tool.py)
  - [tools/skills_tool.py](../tmp/hermes-agent-main/tools/skills_tool.py)
  - [hermes_cli/plugins.py](../tmp/hermes-agent-main/hermes_cli/plugins.py)
  - [cron/scheduler.py](../tmp/hermes-agent-main/cron/scheduler.py)
  - [providers/base.py](../tmp/hermes-agent-main/providers/base.py)
- Current Ontocode owners reviewed with MCP and direct source reads:
  - [ontocode-rs/core/src/tools/planning/native.rs](../ontocode-rs/core/src/tools/planning/native.rs)
  - [ontocode-rs/core/src/tools/handlers/agent_jobs.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs.rs)
  - [ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs](../ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs)
  - [ontocode-rs/core/tests/suite/agent_jobs.rs](../ontocode-rs/core/tests/suite/agent_jobs.rs)
  - [ontocode-rs/core/tests/suite/subagent_notifications.rs](../ontocode-rs/core/tests/suite/subagent_notifications.rs)
  - [ontocode-rs/ext/skills/src/catalog.rs](../ontocode-rs/ext/skills/src/catalog.rs)
  - [ontocode-rs/core-plugins/src/manager.rs](../ontocode-rs/core-plugins/src/manager.rs)
  - [ontocode-rs/app-server/README.md](../ontocode-rs/app-server/README.md)
  - [ontocode-rs/app-server/src/request_processors/thread_processor.rs](../ontocode-rs/app-server/src/request_processors/thread_processor.rs)
  - [ontocode-rs/tui/src/chatwidget/tests/history_replay.rs](../ontocode-rs/tui/src/chatwidget/tests/history_replay.rs)
- Current Ontocode worktree is dirty. This review maps donor ideas to current owners only; it does not rewrite tracking.

## Donor Tool And Harness Inventory

Hermes is not one donor feature. It is a large product made of several independent owner stacks:

- Self-registering tool runtime:
  [tools/registry.py](../tmp/hermes-agent-main/tools/registry.py) auto-discovers tool modules via AST and module-level `registry.register(...)` calls.
- Child-agent delegation runtime:
  [tools/delegate_tool.py](../tmp/hermes-agent-main/tools/delegate_tool.py) spawns isolated child agents, strips blocked tools, and manages child approval behavior separately from the parent.
- Background process subsystem:
  [tools/process_registry.py](../tmp/hermes-agent-main/tools/process_registry.py) keeps long-running process state, output buffers, watcher queues, kill/wait flows, and crash recovery.
- MCP runtime:
  [tools/mcp_tool.py](../tmp/hermes-agent-main/tools/mcp_tool.py) supports stdio, HTTP, and SSE transports, reconnect logic, dynamic discovery, and parallel-tool-call flags.
- Session-history recall layer:
  [tools/session_search_tool.py](../tmp/hermes-agent-main/tools/session_search_tool.py) exposes browse, search, read, and scroll behavior over session lineage and FTS-backed history.
- Skill progressive-disclosure layer:
  [tools/skills_tool.py](../tmp/hermes-agent-main/tools/skills_tool.py) separates skill metadata from full skill content and prerequisite/setup details.
- Memory orchestration layer:
  [agent/memory_manager.py](../tmp/hermes-agent-main/agent/memory_manager.py) is a single-manager integration point that owns provider fanout and permits only one external plugin provider at a time.
- Plugin runtime:
  [hermes_cli/plugins.py](../tmp/hermes-agent-main/hermes_cli/plugins.py) discovers bundled, user, project, and pip plugins and lets them register tools, hooks, and providers.
- Provider-profile runtime:
  [providers/base.py](../tmp/hermes-agent-main/providers/base.py) centralizes per-provider behavior in declarative `ProviderProfile` objects.
- Gateway and scheduler:
  Hermes has a large messaging gateway plus a first-class agent scheduler in [cron/scheduler.py](../tmp/hermes-agent-main/cron/scheduler.py).
- Extra surfaces:
  ACP, desktop, web, browser, media, gateway, and training/trajectory systems all exist in the donor repo.

Most donor "ideas" are artifacts of those separate systems. Ontocode should not copy that split.

## Current Ontocode Baseline

Ontocode already has the important current-core owners that donor ideas would need to extend:

- Native tool exposure, collaboration gating, goal-tool gating, and `agent_jobs` gating are already centralized in [native.rs](../ontocode-rs/core/src/tools/planning/native.rs).
- Child-agent contracts, model metadata, and notification behavior already have dedicated coverage in [multi_agents_spec_tests.rs](../ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs) and [subagent_notifications.rs](../ontocode-rs/core/tests/suite/subagent_notifications.rs).
- Batch-worker orchestration and structured output export already have one persisted owner in [agent_jobs.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs.rs) and [agent_jobs.rs tests](../ontocode-rs/core/tests/suite/agent_jobs.rs).
- Skills already have a source-authority and merged-catalog owner in [catalog.rs](../ontocode-rs/ext/skills/src/catalog.rs), with app-server listing and change notifications in [app-server/README.md](../ontocode-rs/app-server/README.md).
- Plugin load/config/cache/reload behavior already has one owner in [manager.rs](../ontocode-rs/core-plugins/src/manager.rs), with listing/read/install surfaces documented in [app-server/README.md](../ontocode-rs/app-server/README.md).
- Stored thread history already has `thread/read`, `thread/turns/list`, and replay coverage in [app-server/README.md](../ontocode-rs/app-server/README.md) and [history_replay.rs](../ontocode-rs/tui/src/chatwidget/tests/history_replay.rs).
- The narrow read-only turn-item hydration gap is explicit today: [app-server/README.md](../ontocode-rs/app-server/README.md) documents `thread/turns/items/list`, while [thread_processor.rs](../ontocode-rs/app-server/src/request_processors/thread_processor.rs) currently returns unsupported-method.
- Process, command, and background-terminal control already exist as app-server surfaces in [app-server/README.md](../ontocode-rs/app-server/README.md), so Hermes `process_registry` does not justify a second process subsystem.
- Older Hermes donor follow-ups already closed the prior defensible regression slices for subagent spec fidelity, plugin connector invalidation, and long-running process cleanup in [ADR_HERMES_DONOR_CORE_EXTENSION_SOLUTIONS.md](ADR_HERMES_DONOR_CORE_EXTENSION_SOLUTIONS.md) and [HERMES_DONOR_CORE_EXTENSION_TRACKING.md](HERMES_DONOR_CORE_EXTENSION_TRACKING.md).

## Challenge Result

The first review still kept too much.

After challenge, four things became clear:

- `HERMES-K1`, `HERMES-K5`, and `HERMES-K6` are not new anymore. Their valid narrow forms were already accepted and closed in the older Hermes ADR/tracking trail.
- `HERMES-K2` is not a distinct new donor extension. Current `agent_jobs` already has export/finalization owners plus focused suite coverage around run/export, dedupe, wrong-thread rejection, and item finalization.
- `HERMES-K4` is still speculative. Current skills owners already expose list, cache invalidation, and change notifications; the remaining "ambiguity/prerequisite hint" idea is a UX preference, not a proven owner gap.
- `HERMES-K3` is the only family that is still both new and clearly mapped to an existing current-core owner.

## Keep Only: New Current-Core Extension

| Id | Donor idea | Keep | Existing Ontocode owner | Challenge |
| --- | --- | --- | --- | --- |
| `HERMES-K3` | Read-only history browsing, session lineage drill-down, and per-turn hydration are useful when they extend current app-server and replay owners. | Keep. This is the only donor family still open, still new, and still clearly mapped to an existing owner. `thread/turns/items/list` is the clearest current-owner gap. | [app-server thread history](../ontocode-rs/app-server/README.md), [thread_processor.rs](../ontocode-rs/app-server/src/request_processors/thread_processor.rs), [history_replay.rs](../ontocode-rs/tui/src/chatwidget/tests/history_replay.rs) | Do not import Hermes session-search, FTS, or memory recall stacks as a second history subsystem. |

## Covered: Already Present

- Native tool exposure and collaboration gating already exist in [native.rs](../ontocode-rs/core/src/tools/planning/native.rs).
- Current subagent contracts already cover visible-model metadata, namespace behavior, role/model overrides, and notification routing in [multi_agents_spec_tests.rs](../ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs) and [subagent_notifications.rs](../ontocode-rs/core/tests/suite/subagent_notifications.rs).
- Current batch-worker flows already cover CSV run/export, output schema, dedupe, and stop behavior in [agent_jobs suite](../ontocode-rs/core/tests/suite/agent_jobs.rs).
- Current skill catalog/listing already has a catalog owner plus `skills/list`, `forceReload`, and `skills/changed` surfaces in [catalog.rs](../ontocode-rs/ext/skills/src/catalog.rs) and [app-server/README.md](../ontocode-rs/app-server/README.md).
- Current plugin discovery/listing already has marketplace, load, and reload owners in [manager.rs](../ontocode-rs/core-plugins/src/manager.rs) and [app-server/README.md](../ontocode-rs/app-server/README.md).
- Current stored history already has `thread/read`, `thread/turns/list`, and TUI replay coverage in [app-server/README.md](../ontocode-rs/app-server/README.md) and [history_replay.rs](../ontocode-rs/tui/src/chatwidget/tests/history_replay.rs).
- Current process and background execution already have app-server surfaces for command execution, host processes, and background terminal cleanup in [app-server/README.md](../ontocode-rs/app-server/README.md).
- Older Hermes slices already closed the defensible subagent-spec, plugin-cache, and process-cleanup regressions in [HERMES_DONOR_CORE_EXTENSION_TRACKING.md](HERMES_DONOR_CORE_EXTENSION_TRACKING.md).

## Closed By Challenge

- `HERMES-K1` closed: not new. The donor-valid subagent regression shape already landed through the older Hermes closure work.
- `HERMES-K2` closed: not distinct enough. Current `agent_jobs` already owns export/finalization behavior and has focused suite coverage; there is no fresh donor-specific owner gap yet.
- `HERMES-K4` closed: still speculative. Current skills owners already expose the real core surfaces, and the remaining donor value is just hint UX without proof.
- `HERMES-K5` closed: not new. The donor-valid plugin invalidation slice already landed through the older Hermes closure work.
- `HERMES-K6` closed: not new. The donor-valid long-running process cleanup slice already landed through the older Hermes closure work.

## Strongest Narrow Gap

The clearest donor-backed current-core gap is not a new Hermes subsystem. It is the already-shaped but still unsupported read-only per-turn item hydration path:

- [app-server/README.md](../ontocode-rs/app-server/README.md) documents `thread/turns/items/list`
- [thread_processor.rs](../ontocode-rs/app-server/src/request_processors/thread_processor.rs) currently returns unsupported-method

That makes `HERMES-K3` the only donor family here that has an obvious current-owner extension target without inventing a parallel architecture.

## Rejected: Parallel Runtime Or Wrong Owner

- Do not add Hermes self-registering tool discovery or AST import scans as a second tool runtime.
- Do not add Hermes provider-profile/plugin runtime as a second provider registry or auth-routing layer.
- Do not add Hermes plugin loader as a second plugin discovery, hook, or tool-registration system.
- Do not add Hermes memory-manager or user-modeling stack as a second memory architecture.
- Do not add Hermes session-search subsystem, FTS lineage store, or recalled-memory layer as a second history/search service.
- Do not add Hermes MCP runtime as a second MCP transport/discovery system.
- Do not add Hermes gateway, platform adapters, DM pairing, or mirrored messaging stack.
- Do not add Hermes cron scheduler as first-class current-core scope from this donor alone.
- Do not add Hermes ACP adapter, desktop app, web app, website, or dashboard surfaces.
- Do not add Hermes browser, media, voice, or cloud-tool gateway stacks as current-core follow-up from this review.
- Do not add Hermes batch trajectory, training, eval, or self-improving skill-generation systems.

## Recommended Reuse Shape

If this donor is revisited, the only valid path is to extend current Ontocode owners:

- replay/history hydration stays in current app-server and TUI replay owners

Any broader donor import would create parallel runtime architecture instead of extending the current core solution.

## Recommended First Slices

If this donor is reopened for implementation later, the smallest defensible slices are:

1. `HERMES-K3A` — implement the smallest current-owner version of `thread/turns/items/list` so one turn's full items can be fetched read-only without resuming the thread.
2. If `HERMES-K3A` lands, follow only with owner-local replay and read-surface coverage for the new item-hydration path.

Anything larger is donor theater, not a current-core extension.
