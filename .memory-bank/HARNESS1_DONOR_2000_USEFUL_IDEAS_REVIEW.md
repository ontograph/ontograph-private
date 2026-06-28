# Harness-1 Donor 2000 Useful Ideas Review

status: challenged-closed-no-dispatch
donor: `tmp/harness-1`
target: Ontocode Rust workspace
date: 2026-06-27

## Current State

This file is donor challenge evidence, not the live implementation queue.

This is not a literal 2000-row backlog. Harness-1 is mostly a stateful search-agent, retrieval, training, and evaluation repo. After re-challenging the previously kept rows against the current Ontocode checkout with OntoIndex plus direct source reads, no donor row still qualifies as new current-core extension work.

There is no implementation-ready task in this file today. Reopen only with new owner-gap evidence.

## Scope

Review Harness-1 donor behavior for:

- responses/chat orchestration and tool-call continuity
- tool schemas, parallel tool use, and tool metadata
- trajectory, action, observation, and task-output serialization
- rerank and token-budget pruning behavior
- curated evidence, working-memory, and verification flow
- cookbook tool-use environment and smoke-test harness

The goal is to find useful ideas for current Ontocode owners, not to import a second retrieval harness, second replay model, or training stack.

## Evidence And Caveats

- Donor repo was verified with local CLI `ontoindex analyze ./tmp/harness-1`, which produced `12,033 nodes | 22,981 edges | 303 clusters | 300 flows`.
- Donor review surfaces:
  - [README.md](../tmp/harness-1/README.md)
  - [harness/agent.py](../tmp/harness-1/harness/agent.py)
  - [harness/tools.py](../tmp/harness-1/harness/tools.py)
  - [harness/trajectory.py](../tmp/harness-1/harness/trajectory.py)
  - [harness/tasks.py](../tmp/harness-1/harness/tasks.py)
  - [harness/rerank.py](../tmp/harness-1/harness/rerank.py)
  - [harness/ultra_core.py](../tmp/harness-1/harness/ultra_core.py)
  - [tests/smoke_cli.py](../tmp/harness-1/tests/smoke_cli.py)
  - [tinker_cookbook/tool_use/agent_tool_message_env.py](../tmp/harness-1/tinker-cookbook/tinker_cookbook/tool_use/agent_tool_message_env.py)
- Current Ontocode owners reviewed with OntoIndex and direct source reads:
  - [ontocode-rs/app-server/src/request_processors/thread_processor.rs](../ontocode-rs/app-server/src/request_processors/thread_processor.rs)
  - [ontocode-rs/app-server/tests/suite/v2/thread_read.rs](../ontocode-rs/app-server/tests/suite/v2/thread_read.rs)
  - [ontocode-rs/app-server/tests/suite/v2/client_metadata.rs](../ontocode-rs/app-server/tests/suite/v2/client_metadata.rs)
  - [ontocode-rs/thread-store/src/store.rs](../ontocode-rs/thread-store/src/store.rs)
  - [ontocode-rs/rollout-trace/src/reducer/conversation.rs](../ontocode-rs/rollout-trace/src/reducer/conversation.rs)
  - [ontocode-rs/core/src/tools/handlers/agent_jobs.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs.rs)
  - [ontocode-rs/core/src/tools/planning/native.rs](../ontocode-rs/core/src/tools/planning/native.rs)
  - [ontocode-rs/app-server/src/request_processors/catalog_processor.rs](../ontocode-rs/app-server/src/request_processors/catalog_processor.rs)
  - [ontocode-rs/core-plugins/src/manager.rs](../ontocode-rs/core-plugins/src/manager.rs)
- Current Ontocode worktree is dirty. This review maps donor ideas to current owners, but it does not reopen any existing tracking file by itself.

## Donor Tool And Harness Inventory

Harness-1 is not one feature. It is several overlapping systems:

- Search-agent orchestration and OpenAI Responses continuity in [harness/agent.py](../tmp/harness-1/harness/agent.py)
- Provider-agnostic tool schemas plus search/read/grep/parallel/prune tools in [harness/tools.py](../tmp/harness-1/harness/tools.py)
- Action, observation, tool-metadata, and trajectory hydration in [harness/trajectory.py](../tmp/harness-1/harness/trajectory.py) and [harness/tasks.py](../tmp/harness-1/harness/tasks.py)
- Token-budget reranking and latency logging in [harness/rerank.py](../tmp/harness-1/harness/rerank.py)
- Curated evidence, working-memory, fan-out search, verify, and prompt-budget logic in [harness/ultra_core.py](../tmp/harness-1/harness/ultra_core.py)
- RL cookbook message environment in [agent_tool_message_env.py](../tmp/harness-1/tinker-cookbook/tinker_cookbook/tool_use/agent_tool_message_env.py)
- Training, evaluation, model-export, and ablation runners from the repo root layout in [README.md](../tmp/harness-1/README.md)
- Cheap import/CLI smoke checks in [tests/smoke_cli.py](../tmp/harness-1/tests/smoke_cli.py)

Most of the donor's "2000 ideas" are consequences of being a search RL repo with a custom retrieval harness. Ontocode should not copy that shape.

## Current Ontocode Baseline

Ontocode already has the important current-core owners that most donor ideas would need to extend:

- Persisted thread reads, turn pagination, and full turn-item pagination already exist in [thread_processor.rs](../ontocode-rs/app-server/src/request_processors/thread_processor.rs), [store.rs](../ontocode-rs/thread-store/src/store.rs), and [thread_read.rs](../ontocode-rs/app-server/tests/suite/v2/thread_read.rs).
- Responses API continuity via `previous_response_id` already exists in live request generation and replay reduction owners in [client_metadata.rs](../ontocode-rs/app-server/tests/suite/v2/client_metadata.rs) and [conversation.rs](../ontocode-rs/rollout-trace/src/reducer/conversation.rs).
- Batch worker export, recovery, resume, finalization, and worker prompt/report semantics already have one persisted owner in [agent_jobs.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs.rs), with exposure still gated in [native.rs](../ontocode-rs/core/src/tools/planning/native.rs) and focused suite/state coverage in [ontocode-rs/core/tests/suite/agent_jobs.rs](../ontocode-rs/core/tests/suite/agent_jobs.rs) and [ontocode-rs/state/src/runtime/agent_jobs.rs](../ontocode-rs/state/src/runtime/agent_jobs.rs).
- Skill and plugin listing already have cache/reload owners in [catalog_processor.rs](../ontocode-rs/app-server/src/request_processors/catalog_processor.rs) and [manager.rs](../ontocode-rs/core-plugins/src/manager.rs), so donor tool-loading architecture does not open a missing owner by itself.

## Challenge Result

Harness-1 spreads reusable behavior across several extra systems that Ontocode does not need:

- corpus-specific retrieval tools
- curated evidence and working-memory state
- reranker and retrieval-backend plumbing
- cookbook RL message environments
- SFT, RL, ablation, and export pipelines
- a search-task serialization model tied to the donor runtime

Most of the donor does not survive this challenge. After rechecking the two previously kept rows against current owners, neither remains both new and current-core.

## No Surviving New Current-Core Extensions

No kept row survives the stricter rule of "new and extended current core solutions only."

## Covered: Not New Work

- `thread/turns/list` and `thread/turns/items/list` already support persisted full item reads and pagination in [thread_processor.rs](../ontocode-rs/app-server/src/request_processors/thread_processor.rs) and [thread_read.rs](../ontocode-rs/app-server/tests/suite/v2/thread_read.rs).
- `previous_response_id` continuity already exists, and replay reduction explicitly reconstructs omitted prefixes from the prior request/response chain in [conversation.rs](../ontocode-rs/rollout-trace/src/reducer/conversation.rs).
- Request-level continuity proof already exists in [client_metadata.rs](../ontocode-rs/app-server/tests/suite/v2/client_metadata.rs).
- Current `agent_jobs` already export CSV snapshots, recover running items, resume rollout-backed workers, and fail workers that finish without `report_agent_job_result` in [agent_jobs.rs](../ontocode-rs/core/src/tools/handlers/agent_jobs.rs).
- Current `agent_jobs` suite already covers run/export, structured output, dedupe, stop behavior, and wrong-thread rejection in [ontocode-rs/core/tests/suite/agent_jobs.rs](../ontocode-rs/core/tests/suite/agent_jobs.rs).
- Current state-runtime tests already cover late-report rejection and final-summary preservation on resume in [ontocode-rs/state/src/runtime/agent_jobs.rs](../ontocode-rs/state/src/runtime/agent_jobs.rs).
- Current replay/history owners already have reducer and replay coverage in [conversation.rs](../ontocode-rs/rollout-trace/src/reducer/conversation.rs) and [ontocode-rs/tui/src/chatwidget/tests/history_replay.rs](../ontocode-rs/tui/src/chatwidget/tests/history_replay.rs).
- Current skill and plugin listing already own cached results plus force-reload behavior in [catalog_processor.rs](../ontocode-rs/app-server/src/request_processors/catalog_processor.rs) and [manager.rs](../ontocode-rs/core-plugins/src/manager.rs).

## Closed By Challenge

- `H1-K1` closed: not new enough. It reduces to regression proof around existing `agent_jobs` owners that already implement export, timeout reaping, resume recovery, finalization, wrong-thread rejection, stop behavior, and resume-summary preservation.
- `H1-K2` closed: not a current-core extension. It is fixture support only, which is useful only after a reproduced bug exists and does not justify keeping a donor row open by itself.

## Reopen Gates

These are not open tasks. They are the only exact reasons to revisit this donor note.

1. Reopen `H1-K1` only if a real `agent_jobs` regression appears that current suite/state tests do not already prove.
2. Reopen `H1-K2` only if a reproduced replay or thread-history bug cannot be expressed with current fixtures and a donor-shaped redacted fixture is strictly the smallest proof artifact.

There is no recommended open order because nothing remains open.

## Rejected: Wrong Owner Or Parallel Runtime

- Do not add donor `search_corpus`, `read_document`, `grep_corpus`, `prune_chunks`, `curate`, `verify`, or `fan_out_search` as current-core tool work from this review alone.
- Do not add donor Chroma-backed retrieval, reranker, or corpus plumbing as a second search stack.
- Do not add donor working-memory, curated-evidence, evidence-graph, or token-budget prompt assembler as a parallel context system.
- Do not add donor `Trajectory`, `SearchTaskOutput`, or chunk/document-id extraction helpers as a second persisted runtime model.
- Do not add donor cookbook `AgentToolMessageEnv`, reward grading, or RL episode logic as current Ontocode core work.
- Do not add donor SFT, RL, ablation, export, or dataset-generation pipelines as current Ontocode core work.
- Do not add donor provider-format conversion and search-harness orchestration as a second tool/runtime abstraction when current owners already exist.

## Recommended Reuse Shape

If this donor is revisited, the only valid implementation path is to extend current Ontocode owners:

- job-level recoverability stays in `agent_jobs` and state runtime owners
- replay and request continuity stay in `rollout-trace`
- persisted turn/item inspection stays in current `app-server` and `thread-store` owners
- donor artifacts stay as bounded fixtures and tests only

Anything larger than that is donor theater, not current-core extension.
