# ADR: Custom Models For Sub-Agents

## Status

Accepted

## Date

2026-06-15

## Context

The checked-out code already has sub-agent model override plumbing:

- `SpawnAgentArgs` includes `model`, `reasoning_effort`, and `service_tier` in both multi-agent v1 and v2.
- `apply_requested_spawn_agent_model_overrides` resolves requested models through `ModelsManager`.
- `hide_spawn_agent_metadata` can intentionally remove `agent_type`, `model`, `reasoning_effort`, and `service_tier` from the exposed tool schema.
- full-history forked agents intentionally inherit parent agent type, model, and reasoning effort.

The active runtime tool schema exposed to this manager session does not expose `model`, `reasoning_effort`, or `service_tier`, so users cannot reliably force sub-agents to run on `gpt-5.4-mini` or another configured model from this session.

Custom model support must work through the existing model catalog. It must not create a second sub-agent-only model registry.

## Problem

Sub-agent model selection currently has a split contract:

- implementation code can accept model overrides
- generated/deployed tool schemas may hide those fields by configuration
- manager workflows need to dispatch workers on a requested model such as `gpt-5.4-mini`
- custom provider models must use the same catalog and validation rules as normal model selection

Without a clear contract, agents either overclaim model enforcement or fall back to "inherits parent model" without a user-visible reason.

## Decision

Expose custom sub-agent model selection through the existing `spawn_agent` tool fields by default. Spawn metadata can still be hidden by explicitly enabling `hide_spawn_agent_metadata`:

- `model`
- `reasoning_effort`
- `service_tier`

The resolver must use the existing `ModelsManager` catalog. This includes built-in models and any configured custom models already present in that catalog. First-class provider catalog integration is a dependency of the separate provider-catalog plan; this ADR must not require a new provider registry.

Default behavior remains unchanged: if `model` is omitted, the child inherits the parent model.

Hidden metadata behavior remains unchanged: when `hide_spawn_agent_metadata` is enabled, the schema must continue to omit `agent_type`, `model`, `reasoning_effort`, and `service_tier`. In that mode, callers cannot force a child model and must report that the runtime only supports inherited-model dispatch.

Manager dispatch prompts must state that limitation when the active runtime schema hides model fields. Do not claim a worker was pinned to a requested model unless the tool call actually exposes and accepts the model field.

Full-history fork behavior remains unchanged:

- multi-agent v1: `fork_context = true` rejects model overrides.
- multi-agent v2: `fork_turns = "all"` rejects model overrides.
- partial or no-fork spawns may use model overrides when the schema exposes them.

## Contract

When a caller passes `model = "gpt-5.4-mini"` or another configured custom model and the schema exposes model fields:

- `spawn_agent` validates the model against the offline `ModelsManager` catalog.
- if the exact model id exists, the child config uses that model.
- hidden catalog entries are allowed by exact id but are not shown in tool descriptions.
- if `reasoning_effort` is omitted, the child uses the selected model's default reasoning effort.
- if `reasoning_effort` is supplied, it must be supported by the selected model.
- if `service_tier` is supplied, it must be supported by the selected model.
- spawn events should record requested and effective model metadata when available.

Unknown model strings are rejected. Arbitrary pass-through model names are not accepted until they are represented in the model catalog.

Provider-qualified aliases such as `gemini/gemini-2.5-pro` are accepted only if they are canonical model ids in `ModelsManager`; this ADR does not add alias parsing.

Canonical provider-qualified model ids for Gemini, Claude, and other first-class providers belong in the provider-catalog ADR, not here.

## Implementation Notes

- Reuse `ontocode-rs/core/src/tools/handlers/multi_agents/spawn.rs`.
- Reuse `ontocode-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs`.
- Reuse `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`.
- Reuse `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs` for schema exposure and hidden-metadata behavior.
- Reuse `ontocode-rs/models-manager/` for catalog loading and custom model normalization.
- Do not promise model fields in the deployed schema when `hide_spawn_agent_metadata` is enabled.
- Include only picker-visible built-ins and custom models in the spawn-agent model description, bounded by the existing maximum.

## Non-Goals

- Do not create a second provider registry.
- Do not create a sub-agent-only model config file.
- Do not allow raw unknown model strings that bypass catalog validation.
- Do not allow model overrides for full-history forked agents.
- Do not expose hidden model ids in tool descriptions.
- Do not change provider execution ownership out of `model-provider`.
- Do not add provider-qualified alias parsing in this ADR.

## Verification

- Add or update schema exposure tests for `spawn_agent` showing `model`, `reasoning_effort`, and `service_tier` are present when `hide_spawn_agent_metadata = false`.
- Keep or add schema tests proving those fields are absent when `hide_spawn_agent_metadata = true`.
- Add a focused spawn-agent test where a configured custom model is accepted by exact id.
- Add a focused spawn-agent test where a hidden configured model is accepted by exact id but omitted from the description.
- Add a focused spawn-agent test where an unknown model is rejected with a bounded error.
- Add v1 and v2 full-history fork tests proving `model` override is rejected.
- Run:
  `CARGO_BUILD_JOBS=8 just test -p ontocode-core spawn_agent`

## Closed Questions

- Manager dispatch prompts must include a fallback statement when the active runtime schema hides model fields.
- Canonical provider-qualified ids for Gemini, Claude, and other providers are deferred to the provider-catalog ADR.
