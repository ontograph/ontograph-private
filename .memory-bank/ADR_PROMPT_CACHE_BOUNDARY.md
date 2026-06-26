# ADR: Prompt Cache Boundary

## Status

Challenged - keep as an owner-local invariant only

## Date

2026-06-21

## Context

Donor prompt reviews found the same useful pattern in three places:

- Gemini CLI composes prompts from option-driven sections instead of a single static blob: [promptProvider.ts](../tmp/gemini-cli-main/packages/core/src/prompts/promptProvider.ts), [snippets.ts](../tmp/gemini-cli-main/packages/core/src/prompts/snippets.ts).
- OpenClaw separates stable prompt prefix from dynamic runtime context with an explicit cache boundary: [system-prompt.ts](../tmp/openclaw-main/src/agents/system-prompt.ts), [system-prompt-cache-boundary.ts](../tmp/openclaw-main/src/agents/system-prompt-cache-boundary.ts).
- Opencode models runtime system context as named sources that can be refreshed across turns: [system-context/index.ts](../tmp/opencode-main/packages/core/src/system-context/index.ts), [system-context/builtins.ts](../tmp/opencode-main/packages/core/src/system-context/builtins.ts), [session/runner/llm.ts](../tmp/opencode-main/packages/core/src/session/runner/llm.ts).

Ontocode currently has stable base prompt guidance in [default.md](../ontocode-rs/protocol/src/prompts/base_instructions/default.md). The useful donor idea is not a full prompt framework copy. It is the invariant that cacheable prompt content must remain byte-stable, while fast-changing facts must be appended after a clear boundary.

OntoIndex challenge evidence:

- `ContextualUserFragment` already owns typed injected context rendering and marker recognition in [fragment.rs](../ontocode-rs/context-fragments/src/fragment.rs).
- `InternalContextSource` already validates stable source labels for hidden runtime model context in [internal_model_context.rs](../ontocode-rs/core/src/context/internal_model_context.rs).
- Existing tests already cover internal model context detection, source validation, and dynamic compatibility in [contextual_user_message_tests.rs](../ontocode-rs/core/src/context/contextual_user_message_tests.rs).
- Existing websocket tests already exercise model request behavior when non-input request fields and base instructions change in [client_websockets.rs](../ontocode-rs/core/tests/suite/client_websockets.rs).

## Decision

If Ontocode introduces or extends provider/runtime prompt caching, the prompt assembly owner must render prompt content in this order:

1. Stable base instructions.
2. Stable project or user context that should benefit from provider prompt caching.
3. Explicit cache boundary marker.
4. Dynamic context: runtime mode, tools, MCP discovery state, sandbox state, current time, active topic, heartbeat/session facts, and other fast-changing inputs.

The marker should be an internal implementation detail. It must not create a user-facing prompt customization API by itself.

Dynamic sections that are refreshed across turns must use the existing contextual fragment/source identity model when possible. The preferred shape is a bounded `ContextualUserFragment` with a stable source identity, not a new prompt registry or cache-map layer.

Challenge result: this ADR is useful only as a narrow invariant for the existing prompt/context owners. It is not approval to build a parallel prompt engine, provider contribution framework, arbitrary prompt-transform hook system, or OpenClaw-style cache manager.

## Non-Goals

- Do not copy OpenClaw's full prompt renderer, hook override framework, stable-prefix cache map, or provider contribution system.
- Do not copy Opencode's Effect runtime, system-context registry, or generalized snapshot lifecycle.
- Do not add a cache boundary until there is an actual prompt assembly path that needs stable-prefix behavior.
- Do not move dynamic facts into the static markdown base prompt.
- Do not introduce a second source-label scheme; extend or reuse `InternalContextSource` and `ContextualUserFragment` unless current owners cannot express the case.
- Do not add arbitrary prompt mutation hooks.

## Implementation Sketch

Smallest acceptable implementation:

```text
stable base instructions
stable loaded context
<internal prompt cache boundary>
dynamic runtime/session/tool context
```

Suggested owner: the existing Ontocode prompt/context assembly path that already builds model instructions and contextual fragments. The owner should expose a tiny helper for splitting or appending after the boundary only if multiple call sites need it.

Refreshable dynamic sections should be represented as existing contextual fragments with stable source labels. Snapshot/update behavior should be tested at the owner that refreshes the fragment, not through a new global prompt-context registry.

## Acceptance Criteria

- A focused test or snapshot proves dynamic context appears after the boundary.
- A focused test or snapshot proves stable prefix text is byte-identical when only dynamic runtime facts change.
- A focused test proves refreshable dynamic sections retain stable source identity while their body updates.
- Source labels are validated through the existing source-label rules or a narrowly extended version of them.
- All injected model context remains bounded and covered by the existing contextual fragment architecture.
- Existing prompt/context owners remain the only assembly path; no parallel prompt engine is introduced.
- Provider-specific caching support is gated by the provider/runtime that can consume it.
- No new public config key, app-server API, SDK surface, or user-facing customization behavior is introduced without a separate ADR and compatibility tests.

## Prompt Rule Already Added

[default.md](../ontocode-rs/protocol/src/prompts/base_instructions/default.md) now instructs future prompt edits to preserve stable prompt text, keep volatile repo/session/tool/MCP context in dynamic sections, and place dynamic context after an explicit boundary when prompt caching exists.
