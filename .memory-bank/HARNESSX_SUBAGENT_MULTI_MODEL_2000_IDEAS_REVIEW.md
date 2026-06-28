# HarnessX Sub-Agent Multi-Model Ideas Challenge

status: challenged-narrowed
donor: `tmp/HarnessX`
target: Ontocode Rust workspace
date: 2026-06-27

## Current State

This file is donor challenge evidence, not the live implementation queue.

- Current recommendation-layer authority: [ONTOCODE_CHAT_LOG_RECOMMENDATION_IMPLEMENTATION.md](ONTOCODE_CHAT_LOG_RECOMMENDATION_IMPLEMENTATION.md)
- Current shared loop-closeout authority: [ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md](ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md)
- Current sub-agent closeout authority for exact reopen-gate handling: [audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md](audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md)

There is no implementation-ready task in this file. Reopen only the exact row whose evidence gate is newly satisfied.

## Scope

Review HarnessX donor behavior for child model overrides, role-keyed model configs, task-start routing, provider fallback chains, predeclared worker subagents, and child lineage when different models are involved.

This is no longer a 2000-idea queue. The earlier 60-seed review was challenged against current Ontocode owners and reduced to only ideas that are new, extend existing core functionality, and do not introduce a parallel model-routing or sub-agent runtime.

## Evidence And Caveats

- Donor repo was verified with local CLI `ontoindex analyze ./tmp/HarnessX`.
- Donor review surfaces: [harnessx/tools/spawn_subagent.py](../tmp/HarnessX/harnessx/tools/spawn_subagent.py), [harnessx/core/model_config.py](../tmp/HarnessX/harnessx/core/model_config.py), [harnessx/processors/multi_model/model_router.py](../tmp/HarnessX/harnessx/processors/multi_model/model_router.py), [harnessx/providers/group.py](../tmp/HarnessX/harnessx/providers/group.py), [harnessx/meta_harness/workers/trajectory_digester.py](../tmp/HarnessX/harnessx/meta_harness/workers/trajectory_digester.py), [harnessx/core/harness.py](../tmp/HarnessX/harnessx/core/harness.py), [harnessx/core/state.py](../tmp/HarnessX/harnessx/core/state.py), [harnessx/core/trajectory.py](../tmp/HarnessX/harnessx/core/trajectory.py), [harnessx/processors/control/compaction.py](../tmp/HarnessX/harnessx/processors/control/compaction.py), [tests/unit/test_spawn_subagent.py](../tmp/HarnessX/tests/unit/test_spawn_subagent.py), [tests/unit/test_model_router.py](../tmp/HarnessX/tests/unit/test_model_router.py), [tests/unit/test_provider_group.py](../tmp/HarnessX/tests/unit/test_provider_group.py), [tests/unit/test_reflect_worker_tool.py](../tmp/HarnessX/tests/unit/test_reflect_worker_tool.py), [tests/integration/test_spawn_tool_usage.py](../tmp/HarnessX/tests/integration/test_spawn_tool_usage.py), [tests/integration/test_spawn_tool_edge_cases.py](../tmp/HarnessX/tests/integration/test_spawn_tool_edge_cases.py), [tests/integration/test_spawn_sse_events.py](../tmp/HarnessX/tests/integration/test_spawn_sse_events.py).
- Current Ontocode MCP index is stale against `HEAD`, so OntoIndex was used for owner discovery only and direct source/ADR reads are the exact authority for current behavior.
- Current Ontocode sources reviewed: `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`, `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`, `ontocode-rs/core/tests/suite/subagent_notifications.rs`, `ontocode-rs/core/src/provider_route.rs`, `ontocode-rs/rollout-trace/src/reducer/tool/agents_tests.rs`.
- Current authority docs reviewed: [ADR_CUSTOM_SUBAGENT_MODELS_TRACKING.md](ADR_CUSTOM_SUBAGENT_MODELS_TRACKING.md), [QWEN_SUBAGENT_MULTI_MODEL_IDEAS_REVIEW.md](QWEN_SUBAGENT_MULTI_MODEL_IDEAS_REVIEW.md), [OPENCLAW_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md](OPENCLAW_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md).

## Current Ontocode Baseline

Ontocode already has the core shape that matters from the donor:

- `build_agent_spawn_config` carries the live turn config into the child spawn path instead of building a second child runtime stack.
- `apply_requested_spawn_agent_model_overrides` validates requested child model ids through the existing `ModelsManager` path.
- `resolve_requested_spawn_agent_model` already supports `inherit` and `fast` inside the current child selector resolver.
- `spawn_agent_role_provider_override_keeps_parent_settings_and_auth_unchanged` already covers role/provider override without mutating the parent auth/runtime contract.
- `spawn_agent_role_provider_override_routes_api_key_child_without_changing_parent_route` and the OAuth companion test already cover child-route changes without parent-route churn.
- Full-history fork rejection already blocks child model overrides in the current spawn path.
- `QSM-K5` is already closed for explicit requested/effective child-model outcome telemetry.
- `rollout-trace` already has existing spawn/result edge owners for child lineage, so the donor’s trace ideas do not start from zero here.

## Challenge Result

HarnessX spreads “sub-agent with different models” across five extra systems that Ontocode does not currently have:

- a role-keyed model registry
- a task-start model-router processor
- a provider fallback chain runtime
- a predeclared worker-subagent factory
- a second async-child state and SSE protocol surface

Ontocode should not copy that architecture. The only reusable parts are the ones that extend the current `spawn_agent` resolver, current role/config owners, and current regression coverage. Everything else would add a second model router, second provider failover runtime, second worker system, or second sub-agent state/trace contract.

## Keep Only: New Existing-Core Extensions

| Id | Donor idea | Keep | Existing Ontocode owner | Challenge |
| --- | --- | --- | --- | --- |
| `HARNESSX-MM-K1` | Role-keyed secondary model policy such as `small`/`summarize` can steer specific child work without replacing the main model contract. | Same gate as `QSM-K4`. Reopen only if the current role/config path proves it owns child default-model policy data and source precedence. | existing role/config path plus `multi_agents_spec.rs` model guidance | Do not add a HarnessX-style `main/small/summarize/judge` model registry. Keep any future policy on the current role/config owner only. |
| `HARNESSX-MM-K2` | If the child effectively inherits the parent model/provider, reuse the inherited child-runtime path instead of allocating a second model-routing stack. | Same gate as `QSM-K3`. Reopen only with evidence that the current spawn/runtime owner allocates extra provider/runtime state when the effective child target is unchanged. | `multi_agents_common.rs`, `multi_agents_v2/spawn.rs`, current provider-route owner | Do not import `ModelConfig`, sub-harness registries, or task-start route slots just to express “same child target as parent.” |
| `HARNESSX-MM-K3` | Focused regression tests for provider/role override, effective child route propagation, and inherited child model behavior. | Implementation-ready only as tests if a real uncovered path is found. | `multi_agents_tests.rs`, `core/tests/suite/subagent_notifications.rs`, `provider_route.rs` | The code already implements the behavior. Add no production routing code unless a failing test proves a gap. |
| `HARNESSX-MM-K4` | Bounded child-model diagnostics should stay explicit about requested versus effective child model when a child route changes. | Deferred UX/test candidate only if current output is materially ambiguous. | existing `spawn_agent` model guidance plus current sub-agent telemetry/trace owners | Do not add provider-fallback transcripts, route-slot dumps, or unbounded “attempted models” output without an accepted provider-runtime owner first. |

## Covered: Not New Work

- HarnessX child inherit/override on one shared spawn path is already covered by the current `spawn_agent` resolver plus the accepted `inherit` and `fast` selectors.
- Child route derivation after role/runtime overrides already exists in `provider_route.rs` and current suite coverage.
- Parent auth/runtime preservation under child role/provider overrides is already covered by existing sub-agent suite tests.
- Full-history fork model lock is already enforced in the current spawn validation path.
- Explicit requested/effective child-model outcome telemetry is already closed under `QSM-K5`.
- Child spawn/result lineage already has an existing `rollout-trace` owner and reducer tests, so the donor’s trace family is not a greenfield requirement here.

## Blocked Or Deferred

- `HARNESSX-MM-K1` reopens only after the current role/config path proves it owns child default-model policy data and precedence.
- `HARNESSX-MM-K2` reopens only with concrete evidence that the current inherited child path still allocates extra provider/runtime state when no effective child target changed.
- `HARNESSX-MM-K3` reopens only with a failing test or clear missing-coverage proof in the existing role/provider override and child-route propagation path.
- `HARNESSX-MM-K4` reopens only with a concrete ambiguity where current requested/effective child-model output is insufficient in existing owners.

## Rejected: Wrong Owner Or Parallel Runtime

- Do not add a HarnessX-style `ModelConfig` role registry, `ModelConfig.copy` runtime, or provider-spec serialization as a second sub-agent model system.
- Do not add `ModelRouterProcessor`, task-start route slots, router sub-harnesses, or classifier prompts as a second model-routing owner.
- Do not add `ProviderGroup`, retry/cooldown fallback chains, auth-error entry skipping, or `attempted_models` as a second provider failover runtime inside sub-agent handling.
- Do not add `spawn_reflect_worker`, worker kinds, worker-specific tool registries, or static worker prompt factories as core Ontocode sub-agent model work.
- Do not add HarnessX `pending_subagents`, async completion message injection, child-start SSE contracts, or nested child workspace rules as a second sub-agent state/protocol surface for this donor slice.
- Do not import HarnessX compaction sub-harnesses, secondary summarize roles, or nested child journals as justification for new sub-agent model architecture.

## Recommended Reuse Shape

If this donor is revisited, the only valid implementation path is to extend the current core owners:

- child model selector behavior stays in the current spawn-agent model resolver
- child route preservation stays in current role/provider override and provider-route owners
- default child-model policy stays in the current role/config owner
- requested/effective child-model diagnostics stay bounded inside current telemetry/trace owners
- validation uses focused multi-agent handler and suite tests

No implementation is currently ready from this file unless one of the exact reopen gates above is satisfied.
