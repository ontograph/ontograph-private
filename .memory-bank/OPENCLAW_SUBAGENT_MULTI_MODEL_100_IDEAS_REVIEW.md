# OpenClaw Sub-Agent Multi-Model Ideas Challenge

status: challenged-narrowed
donor: `tmp/openclaw-main`
target: Ontocode Rust workspace
date: 2026-06-27

## Current State

This file is donor challenge evidence, not the live implementation queue.

- Current recommendation-layer authority: [ONTOCODE_CHAT_LOG_RECOMMENDATION_IMPLEMENTATION.md](ONTOCODE_CHAT_LOG_RECOMMENDATION_IMPLEMENTATION.md)
- Current shared loop-closeout authority: [ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md](ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md)
- Current sub-agent closeout authority for exact reopen-gate handling: [audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md](audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md)

There is no implementation-ready task in this file. Reopen only the exact row whose evidence gate is newly satisfied.

## Scope

Review OpenClaw donor behavior for sub-agent model planning, provider/runtime routing, context mode, depth limits, and workflow model selection when different child models are involved.

This is no longer a 100-item implementation queue. The original donor inventory was challenged against current Ontocode owners and reduced to only ideas that are new, extend existing core functionality, and do not introduce a parallel sub-agent runtime.

## Evidence And Caveats

- Donor repo was verified with local CLI `ontoindex analyze ./tmp/openclaw-main`, which reported `Already up to date`.
- Donor review surfaces: [src/agents/subagent-spawn-plan.ts](../tmp/openclaw-main/src/agents/subagent-spawn-plan.ts), [src/agents/model-selection.ts](../tmp/openclaw-main/src/agents/model-selection.ts), [src/agents/model-runtime-aliases.ts](../tmp/openclaw-main/src/agents/model-runtime-aliases.ts), [src/agents/model-picker-visibility.ts](../tmp/openclaw-main/src/agents/model-picker-visibility.ts), [src/agents/subagent-spawn.ts](../tmp/openclaw-main/src/agents/subagent-spawn.ts), [src/agents/subagent-depth.ts](../tmp/openclaw-main/src/agents/subagent-depth.ts), [src/plugin-sdk/provider-onboard.ts](../tmp/openclaw-main/src/plugin-sdk/provider-onboard.ts), [src/cron/isolated-agent/model-selection.ts](../tmp/openclaw-main/src/cron/isolated-agent/model-selection.ts), [src/cron/isolated-agent/run-config.ts](../tmp/openclaw-main/src/cron/isolated-agent/run-config.ts), [src/agents/openclaw-tools.subagents.sessions-spawn.model.test.ts](../tmp/openclaw-main/src/agents/openclaw-tools.subagents.sessions-spawn.model.test.ts), [src/agents/subagent-spawn.context.test.ts](../tmp/openclaw-main/src/agents/subagent-spawn.context.test.ts), [src/agents/subagent-spawn.depth-limits.test.ts](../tmp/openclaw-main/src/agents/subagent-spawn.depth-limits.test.ts), [docs/providers/models.md](../tmp/openclaw-main/docs/providers/models.md), [skills/coding-agent/SKILL.md](../tmp/openclaw-main/skills/coding-agent/SKILL.md).
- Current Ontocode MCP index is stale against `HEAD`, so OntoIndex was used for owner discovery only and direct source/ADR reads are the exact authority for current behavior.
- Current Ontocode sources reviewed: `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`, `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`, `ontocode-rs/core/src/tools/handlers/multi_agents_tests.rs`, `ontocode-rs/core/tests/suite/subagent_notifications.rs`.
- Current authority docs reviewed: [ADR_CUSTOM_SUBAGENT_MODELS_TRACKING.md](ADR_CUSTOM_SUBAGENT_MODELS_TRACKING.md), [QWEN_SUBAGENT_MULTI_MODEL_IDEAS_REVIEW.md](QWEN_SUBAGENT_MULTI_MODEL_IDEAS_REVIEW.md), [CLAUDE_CODE_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md](CLAUDE_CODE_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md).

## Current Ontocode Baseline

Ontocode already has the core shape that matters from the donor:

- `build_agent_spawn_config` carries the live turn config into the child spawn path rather than creating a second child-runtime config stack.
- `reject_full_fork_spawn_overrides` already blocks child `model` and `reasoning_effort` overrides on full-history forks.
- `apply_requested_spawn_agent_model_overrides` validates requested child model ids through the existing `ModelsManager` path.
- `resolve_requested_spawn_agent_model` already supports `inherit` and `fast` inside the current selector resolver.
- `spawn_agent_models_description` already gives bounded picker-visible guidance for valid child model selectors.
- Focused tests already cover unknown-model rejection, `inherit`, `fast`, full-history fork override rejection, service-tier validation of the effective child model, and role/provider override propagation.
- `CSM-9`, `QSM-K1`, and `QSM-K5` are already closed; this donor cannot reopen them as fresh work.

## Challenge Result

OpenClaw treats different-model sub-agents as one policy layer spread across several product owners: spawn planning, runtime aliases, provider onboarding, session-store lineage, cron/workboard routing, and coding-agent worker orchestration.

Ontocode should not copy that surface area. The only reusable ideas are the ones that extend the current `spawn_agent` model resolver, existing role/config precedence, and focused regression coverage. Everything else would create a second model router, second runtime alias system, second child-session store policy, or OpenClaw-specific workflow product.

## Keep Only: New Existing-Core Extensions

| Id | Donor idea | Keep | Existing Ontocode owner | Challenge |
| --- | --- | --- | --- | --- |
| `OPENCLAW-MM-K1` | Provider-qualified child selectors and normalization such as splitting `provider/model` refs before child spawn planning. | Same gate as `QSM-K2`. Reopen only after provider catalog ids are canonical and slash-bearing exact ids are proven safe. | existing `ModelsManager` catalog plus `multi_agents_common.rs` selector resolver | Do not add an OpenClaw-style provider parser or normalization policy inside `spawn_agent`. Provider parsing belongs to the current model-catalog owner. |
| `OPENCLAW-MM-K2` | Role/config-owned child default model policy separate from the main agent default. | Same gate as `QSM-K4`. Keep only if current role/config owners already own the source data and precedence. | existing role/config path plus `multi_agents_spec.rs` model guidance | Do not introduce `agents.defaults.subagents.model`, child fallback registries, or onboarding-written child defaults as a second config stack. |
| `OPENCLAW-MM-K3` | Focused regression tests proving provider/runtime inheritance still behaves correctly when child selection is `inherit`, `fast`, or role-driven. | Implementation-ready only as tests if a real uncovered path is found. | `multi_agents_tests.rs`, `core/tests/suite/subagent_notifications.rs` | The code already implements the behavior. Add no production routing code unless a failing test proves a gap. |
| `OPENCLAW-MM-K4` | Better invalid-selector help and bounded effective-model guidance when a requested child selector is ambiguous. | Deferred UX/test candidate only. | `resolve_requested_spawn_agent_model`, `spawn_agent_models_description` | Keep help bounded and derived from the existing visible model list. Do not dump provider catalogs, runtime aliases, or onboarding metadata into tool text. |
| `OPENCLAW-MM-K5` | Clarify configured-versus-effective child model precedence and persistence. | Deferred doc/test candidate only if the current precedence is materially ambiguous. | existing ADR/tracking docs plus current spawn tests | Do not add a second persisted child-model planner, session-patch policy, or recomputation layer just to mirror OpenClaw's `initialSessionPatch` behavior. |

## Covered: Not New Work

- OpenClaw's basic spawn-time model planning is already covered by the current `spawn_agent` resolver plus the accepted `inherit` and `fast` selectors.
- Current Ontocode already carries live config into the child spawn path and already tests effective child model/service-tier behavior.
- Full-history fork model and reasoning lock is already enforced by `reject_full_fork_spawn_overrides`.
- Explicit sub-agent completion telemetry is already closed under `QSM-K5`; this donor does not reopen it.
- Current role/provider override tests already prove that role-driven child settings can preserve parent runtime/auth settings when required.

## Blocked Or Deferred

- `OPENCLAW-MM-K1` reopens only when provider catalog ids are canonical enough to parse provider-qualified child selectors without breaking exact ids.
- `OPENCLAW-MM-K2` reopens only after the current role/config path proves it owns child default-model policy data and source precedence.
- `OPENCLAW-MM-K3` reopens only with a failing test or clear missing-coverage proof in the existing spawn/runtime inheritance path.
- `OPENCLAW-MM-K4` reopens only with a concrete UX failure where current selector errors are ambiguous despite the available-model list.
- `OPENCLAW-MM-K5` reopens only with concrete evidence that current configured-versus-effective child model precedence is unclear in existing owners.

## Rejected: Wrong Owner Or Parallel Runtime

- Do not add OpenClaw runtime alias execution routing, auth-profile-driven runtime remapping, or picker-visibility filtering as a second provider/runtime policy layer.
- Do not add provider onboarding alias writers, model alias indexes, or child-model preset merges as sub-agent runtime responsibilities.
- Do not add OpenClaw's dedicated child session registry, persisted spawn-depth lineage logic, role/control-scope flags, or inherited tool-state storage as a second sub-agent state system.
- Do not add OpenClaw cron isolated-agent routing, workboard execution-engine model routing, or coding-agent worker notification/runtime contracts as core Ontocode sub-agent work.
- Do not add thread-binding or channel-capability policy from OpenClaw unless a current Ontocode protocol owner proves a missing compatibility requirement.
- Do not import OpenClaw's context-engine bootstrap modes, lightweight context lane, or attachment/session-store orchestration as model-selection features.

## Recommended Reuse Shape

If this donor is revisited, the only valid implementation path is to extend the existing core owners:

- child model selector parsing stays in the current spawn-agent model resolver
- provider-qualified normalization stays in the existing model-catalog owner
- child default-model precedence stays in the current role/config owner
- effective-model help stays in the bounded `spawn_agent` model description
- validation uses focused multi-agent handler and suite tests

No implementation is currently ready from this file unless one of the exact reopen gates above is satisfied.
