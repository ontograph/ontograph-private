# Claude Code Sub-Agent Multi-Model Ideas Challenge

status: challenged-narrowed
donor: `tmp/claude-code-main`
target: Ontocode Rust workspace
date: 2026-06-27

## Current State

This file is donor challenge evidence, not the live implementation queue.

- Current recommendation-layer authority: [ONTOCODE_CHAT_LOG_RECOMMENDATION_IMPLEMENTATION.md](ONTOCODE_CHAT_LOG_RECOMMENDATION_IMPLEMENTATION.md)
- Current shared loop-closeout authority: [ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md](ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md)
- Current sub-agent closeout authority for exact reopen-gate handling: [audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md](audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md)

There is no implementation-ready task in this file. Reopen only the exact row whose evidence gate is newly satisfied.

## Scope

Review Claude Code donor behavior for sub-agents, teammates, forks, and specialized agents when different models or providers are involved.

This is no longer a 100-item implementation queue. The original broad inventory was challenged against current Ontocode owners and reduced to only ideas that are new, extend existing core functionality, and do not introduce a parallel sub-agent runtime.

## Evidence And Caveats

- Donor repo was indexed with `ontoindex analyze ./tmp/claude-code-main`.
- Donor review surfaces: [src/utils/model/agent.ts](../tmp/claude-code-main/src/utils/model/agent.ts), [src/tools/shared/spawnMultiAgent.ts](../tmp/claude-code-main/src/tools/shared/spawnMultiAgent.ts), [src/utils/swarm/spawnUtils.ts](../tmp/claude-code-main/src/utils/swarm/spawnUtils.ts), [src/tools/AgentTool/forkSubagent.ts](../tmp/claude-code-main/src/tools/AgentTool/forkSubagent.ts), [src/tools/AgentTool/AgentTool.tsx](../tmp/claude-code-main/src/tools/AgentTool/AgentTool.tsx), [src/tools/AgentTool/prompt.ts](../tmp/claude-code-main/src/tools/AgentTool/prompt.ts), [src/tools/AgentTool/built-in/exploreAgent.ts](../tmp/claude-code-main/src/tools/AgentTool/built-in/exploreAgent.ts), [src/tools/AgentTool/built-in/planAgent.ts](../tmp/claude-code-main/src/tools/AgentTool/built-in/planAgent.ts), [src/components/agents/ModelSelector.tsx](../tmp/claude-code-main/src/components/agents/ModelSelector.tsx), [src/utils/model/modelOptions.ts](../tmp/claude-code-main/src/utils/model/modelOptions.ts), [CHANGELOG.md](../tmp/claude-code-main/CHANGELOG.md).
- Current Ontocode MCP index is stale against `HEAD`, so OntoIndex was used for owner discovery only and direct source/ADR reads are the exact authority for current behavior.
- Current Ontocode sources reviewed: `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`, `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`, `ontocode-rs/core/src/tools/handlers/multi_agents_tests.rs`.
- Current authority docs reviewed: [ADR_CUSTOM_SUBAGENT_MODELS_TRACKING.md](ADR_CUSTOM_SUBAGENT_MODELS_TRACKING.md), [QWEN_SUBAGENT_MULTI_MODEL_IDEAS_REVIEW.md](QWEN_SUBAGENT_MULTI_MODEL_IDEAS_REVIEW.md), [audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md](audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md).

## Current Ontocode Baseline

Ontocode already has the core shape that matters from the donor:

- `build_agent_spawn_config` starts from the live turn config and carries model provider, model, reasoning, developer instructions, compact prompt, approval policy, sandbox, shell environment policy, and cwd.
- `reject_full_fork_spawn_overrides` blocks `agent_type`, `model`, and `reasoning_effort` on full-history forks.
- `apply_requested_spawn_agent_model_overrides` validates exact child model ids through the existing `ModelsManager` path.
- `resolve_requested_spawn_agent_model` already supports `inherit` and `fast`.
- `spawn_agent_models_description` exposes bounded picker-visible model guidance and says inherited parent model is preferred.
- Focused tests already cover fork override rejection, unknown model rejection, `inherit`, `fast`, and service-tier validation of the effective child model.
- `CSM-9`, `QSM-K1`, and `QSM-K5` are closed; `QSM-K2`, `QSM-K3`, and `QSM-K4` remain blocked with exact reopen gates.

## Challenge Result

Claude Code treats different-model sub-agents as policy over one agent runtime. That aligns with Ontocode's existing architecture. The useful donor lesson is not "add 100 features"; it is "keep model selection small, inherited by default, validated by the existing model catalog, and forbidden on full-history forks."

Most original rows are duplicate, already implemented, provider/product-specific, or require a second runtime owner. Keep only the candidates below.

## Keep Only: New Existing-Core Extensions

| Id | Donor idea | Keep | Existing Ontocode owner | Challenge |
| --- | --- | --- | --- | --- |
| `CLAUDE-MM-K1` | Bare family aliases such as `opus`, `sonnet`, and `haiku` preserve the parent's exact tier when the parent already matches that family. | Deferred candidate only. Add aliases only if the existing model catalog can express family/tier metadata without a new alias registry. | `multi_agents_common.rs` model resolver, existing `ModelsManager` catalog | Do not add Claude-specific hardcoding. If no catalog metadata exists, reject until model metadata is canonical. |
| `CLAUDE-MM-K2` | Provider-aware child model ids and fallback suggestions. | Same gate as `QSM-K2`. Provider-qualified parsing can reopen only after provider catalog ids are canonical and colon-bearing exact ids are proven safe. | existing model-provider and `ModelsManager` owners | Do not create a sub-agent provider parser. Do not copy Bedrock/Vertex region-prefix rules into spawn code. |
| `CLAUDE-MM-K3` | Role/config default child model policy such as cheap explorer or inherited planner. | Same gate as `QSM-K4`. Keep only if current role/config owners already expose the source data and precedence. | existing role/config path plus `multi_agents_spec.rs` guidance | Do not introduce a built-in specialist registry or source-precedence layer. |
| `CLAUDE-MM-K4` | Regression tests for provider/runtime inheritance when `inherit` or `fast` is selected. | Implementation-ready only as tests if a real uncovered path is found. | `multi_agents_tests.rs`, `core/tests/suite/subagent_notifications.rs` | The code already implements the behavior. Add no production code unless a failing test proves a gap. |
| `CLAUDE-MM-K5` | Better invalid selector help for `inherit`, `fast`, and picker-visible exact ids. | Deferred UX/test candidate. Error/help text may be improved only if it stays bounded and uses existing model descriptions. | `resolve_requested_spawn_agent_model`, `spawn_agent_models_description` | Current errors already list available models; do not widen prompt text or add unbounded catalog dumps. |
| `CLAUDE-MM-K6` | Cheap side-query fallback degrades to main model when the small/fast model is unavailable. | Keep only as a possible `fast` selector non-regression rule. | `SPAWN_AGENT_FAST_MODEL_PRIORITY`, existing resolver tests | Do not add a scheduler, task-class router, or specialist-agent runtime. |

## Covered: Not New Work

- Original rows `1-4`, `7-15`: exact model ids, `inherit`, `fast`, shared resolver ownership, and bounded model guidance already exist.
- Original rows `16-25`: parent model inheritance and full-history fork model lock already exist.
- Original rows `46-60`: Ontocode uses in-process config snapshots, not Claude's tmux/CLI child process, and current spawn config already carries the live runtime/provider policy.
- Original rows `61-70`: approval policy, permission profile, sandbox, shell environment policy, and cwd already travel through the current runtime override path.
- Original rows `71-85`: full-history fork override rejection is already enforced; cache-sharing guidance is a non-regression rule, not new scope.
- `QSM-K1`, `CSM-9`: `inherit` and `fast` are closed.
- `QSM-K5`: explicit sub-agent completion telemetry is closed in existing analytics/watcher owners.

## Blocked Or Deferred

- `CLAUDE-MM-K1` reopens only when the existing model catalog exposes trustworthy model family/tier metadata or an accepted ADR defines it. Without that, family aliases are Claude-specific hardcoding.
- `CLAUDE-MM-K2` reopens only when provider catalog ids are canonical enough to add provider-qualified parsing without breaking exact ids.
- `CLAUDE-MM-K3` reopens only after the existing role/config path proves it owns default child-model policy data and source precedence.
- `CLAUDE-MM-K4` reopens only with a failing test or missing coverage proof for existing `inherit`/`fast` provider/runtime propagation.
- `CLAUDE-MM-K5` reopens only with a concrete UX failure where current selector errors are ambiguous despite the available-model list.
- `CLAUDE-MM-K6` reopens only if current `fast` selector behavior is proven to fail a side-query fallback requirement in existing owners.

## Rejected: Wrong Owner Or Parallel Runtime

- Do not add a second model registry, second provider parser, second permission engine, second sub-agent scheduler, or specialist-agent runtime.
- Do not import Claude-specific Bedrock/Vertex region-prefix logic into `spawn_agent`.
- Do not add tmux, Chrome, plugin-dir, remote-memory, teammate-mode, or CLI env forwarding behavior as core Ontocode spawn requirements; those are donor runtime details, not current architecture gaps.
- Do not make omitted `subagent_type` implicitly fork unless an accepted ADR reopens fork semantics. Ontocode already has explicit full-history fork controls and rejection tests.
- Do not add parallel Opus/Sonnet/Haiku review orchestration as core runtime. At most, model selection remains a caller/tool policy over the existing `spawn_agent` field.
- Do not add prompt-cache placeholder protocols, fork output files, or "peek at running fork" behaviors without a protocol ADR and compatibility tests.

## Recommended Reuse Shape

If this donor is revisited, the only valid implementation path is to extend the existing core owners:

- model selector parsing stays in the current spawn-agent model resolver
- provider/catalog behavior stays in `ModelsManager` and model-provider owners
- role defaults stay in the existing role/config owners
- fork constraints stay in the existing fork/spawn validation path
- tests use existing multi-agent handler and suite harnesses

No implementation is currently ready from this file unless one of the exact reopen gates above is satisfied.
