# ADR: Multi-Provider OAuth Connection Routing

Date: 2026-06-18

## Status

Superseded for native provider OAuth. Current decision is OpenAI/Codex-native
only, with non-OpenAI providers connected through user-configured external
OpenAI-compatible API endpoints or sidecars.

## 2026-06-19 Revision: OpenAI-First Native Model Auth

The original route-based native OAuth design is rejected for model execution.
It explained the failure mode correctly: previous attempts made Gemini and GPT
exclusive because provider selection and credential lookup behaved like one
active global mode. The fix is not to make Ontocode a native OAuth broker for
every provider.

Accepted now:

- Native browser/device login UX belongs only to OpenAI/Codex unless a later
  ADR approves another first-party provider.
- GPT/Codex remains the first-class default route and must keep working when
  any external provider is missing, expired, or misconfigured.
- Gemini, Claude, Kimi, Antigravity, and future non-OpenAI providers are
  supported only as user-configured external OpenAI-compatible API providers.
- External endpoints or sidecars own non-OpenAI OAuth, API keys, refresh,
  account selection, provider catalogs, and protocol translation.
- Ontocode may consume already persisted provider OAuth credentials through the
  existing provider-auth store when a route explicitly selects that provider
  and optional credential/profile id. This is a rollback recovery of the
  resolver seam only; it does not re-enable native non-OpenAI login/import UX or
  add a second credential store.

Allowed external-provider shape:

```toml
[model_providers.gemini_sidecar]
name = "Gemini via sidecar"
base_url = "http://localhost:1234/v1"
env_key = "GEMINI_SIDECAR_API_KEY"
wire_api = "responses"
requires_openai_auth = false
```

Sidecars are allowed only as user-owned external API endpoints. Ontocode must
not manage their lifecycle, bundled client secrets, browser OAuth, refresh
loops, or provider-specific runtime protocols.

Rejected from this ADR:

- native Gemini/Claude/Kimi/Antigravity OAuth for model runtime
- mixed-provider native OAuth sub-agent routing
- adding a second credential broker, provider registry, or runtime stack

Acceptance criteria for the revised policy:

- Selecting a GPT/Codex model never routes to Gemini or any other non-OpenAI
  provider.
- Non-OpenAI models appear only under explicitly configured external providers.
- Missing or expired external-provider credentials do not affect OpenAI/Codex
  login, account status, or chat.
- Diagnostics redact external API keys and never store raw non-OpenAI OAuth
  tokens in memory-bank files.
- Provider OAuth request auth is provider/profile scoped and falls back to
  OpenAI/Codex auth only when no provider credential is selected or available.

## Context

Ontocode needs to run different model providers at the same time, including through sub-agents with different models and OAuth credentials. The current implementation still behaves like one thread has one active provider. This is why selecting Gemini can conflict with an OpenAI thread, and why prior fixes made Gemini or GPT work in exclusive mode instead of concurrently.

Current constraints:

- OpenAI/Codex must remain the first-class default and fallback route.
- Provider auth must support OAuth profiles and API-key providers.
- Sub-agents must be able to use different provider/model/auth combinations concurrently.
- Provider, auth, runtime, MCP, and model catalog work must extend existing owners rather than adding parallel registries or token stores.
- Raw tokens, cookies, authorization headers, keychain paths, and private user data must never appear in diagnostics or memory-bank files.

OntoIndex review notes:

- `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs` already has the narrow child-agent override seam: `apply_requested_spawn_agent_model_overrides` is called only by v1/v2 `spawn_agent` handlers, and OntoIndex reports LOW upstream impact.
- `ontocode-rs/model-provider/src/provider.rs:create_model_provider` is a broad provider factory used by model clients, tests, guardian, app-server account status, doctor, thread manager, and session turn context; OntoIndex reports CRITICAL upstream impact. Route work must not start by rewriting this factory.
- `ontocode-rs/model-provider/src/provider.rs:create_model_provider_with_id` already separates stable provider id from `ModelProviderInfo.name`; route work must preserve that distinction instead of treating display names as auth identities.
- `ontocode-rs/login/src/auth/manager.rs:provider_oauth_credential_for_auth` currently resolves by provider id only and returns the first matching credential. That is not enough for multi-profile routes.
- Existing TUI tests intentionally refuse ambiguous multi-credential removal for Gemini. Multi-profile routing must add explicit credential/profile selection before enabling mixed-profile execution.

Donor review:

- [OpenClaw OAuth](../tmp/openclaw-main/docs/concepts/oauth.md) shows a useful single token-sink model and warns against copying OAuth refresh tokens between agents.
- [OpenClaw multi-agent model](../tmp/openclaw-main/docs/concepts/multi-agent.md) keeps each agent's workspace, state, auth profiles, and sessions separate.
- [OpenClaw agent runtimes](../tmp/openclaw-main/docs/concepts/agent-runtimes.md) separates provider, model, agent runtime, and channel.
- [OpenClaw runtime architecture](../tmp/openclaw-main/docs/agent-runtime-architecture.md) supports the provider/model/runtime/channel split without making sidecars own provider routing.
- [OpenClaw sidecar loader guard](../tmp/openclaw-main/scripts/check-runtime-sidecar-loaders.mjs) is the sidecar caution: runtime sidecar loaders must be explicit build/runtime entries.
- [OpenCode context contract](../tmp/opencode-main/CONTEXT.md) uses provider/model boundaries as context epochs and allows concurrent sessions with isolated state.
- [OpenCode config spec](../tmp/opencode-main/specs/v2/config.md) favors plural provider configuration and catalog defaults over a single global provider.
- [OpenCode session spec](../tmp/opencode-main/specs/v2/session.md) keeps session execution explicit and durable instead of hidden global provider mutation.
- [Oh My Pi architecture guidance](../tmp/oh-my-pi-main/AGENTS.md) reinforces separation between model catalog, provider client, and agent runtime.

## Decision

Introduce a route-based provider execution model.

The core unit for model execution is a provider route:

```text
ProviderRoute {
  provider_id,
  model,
  auth_kind,
  auth_profile_id,
  runtime_id
}
```

`provider_id` is a stable configured provider key, not a display name. `auth_kind` distinguishes OAuth, API key, external bearer, unauthenticated local providers, and future auth modes. `auth_profile_id` is the stored provider OAuth credential id and is required only for provider OAuth routes that are not unambiguous defaults.

Top-level `model` and `model_provider` remain only the default route. They must not be treated as an exclusive process-wide or thread-wide provider lock.

Every primary agent and sub-agent execution receives an explicit `ProviderRoute` after config and role resolution. If no route is supplied, Ontocode uses the default OpenAI/Codex route.

Provider OAuth resolution becomes provider/profile scoped:

```text
provider_id + auth_profile_id -> credential view
```

API-key provider resolution remains provider scoped through the existing provider configuration, environment key, or configured bearer token path. API-key routes must not require OAuth profile ids.

Token persistence and refresh authority stay in the existing login/auth-store owner. Model-provider and runtime code may request credentials, but must not parse unrelated provider token files or create a second token store. Request auth must not fall back to "first credential for provider" when a route names an auth profile.

Sub-agent dispatch must pass the resolved provider route into model-client/runtime construction. A parent thread can run on OpenAI/Codex while sub-agents run Gemini, Claude, Antigravity, Kimi, or future providers concurrently.

The first implementation slice should extend the existing sub-agent override path and auth lookup path. It should avoid a broad provider-factory rewrite.

## Rejected Alternatives

### Keep One Global Active Provider

Rejected. This is the current failure mode. It makes `/model` selection and thread state mutually exclusive with heterogeneous sub-agents.

### Add A Second Provider Registry

Rejected. Existing `model-provider`, login/auth, native runtime, and catalog owners already exist. A second registry would duplicate behavior and make OAuth bugs harder to trace.

### Make CLIProxyAPI Or Another Gateway The Core Runtime

Rejected for core architecture. It may remain a dev-only or compatibility bridge, but core Ontocode must keep native provider routing so OpenAI/Codex remains reliable by default.

### Start With A Full Sidecar Rewrite

Rejected for the first implementation slice. A sidecar may be useful later for cross-process credential refresh locking and external CLI credential discovery, but starting there adds lifecycle and IPC risk before the route model is proven.

## Sidecar Position

A sidecar is allowed only as a later coordination layer, not as a replacement provider architecture.

Allowed sidecar responsibilities:

- credential refresh locking
- token-sink access mediation
- redacted provider auth status
- external CLI credential discovery
- short-lived credential/session handle issuance

Not allowed:

- owning the model catalog
- owning provider selection policy
- duplicating native runtime request construction
- bypassing existing redaction and diagnostics contracts

If added, sidecar loaders and IPC contracts must be explicit build/runtime entries, not hidden dynamic imports or implicit file probes.

## Implementation Plan

### Stage 0: Route Contract

- Add the internal `ProviderRoute` shape.
- Keep existing config keys as the default route source.
- Derive a route from the already-resolved child config and role config.
- Require stable provider ids; do not use provider display names as credential identities.
- Represent API-key providers as explicit routes with no OAuth profile requirement.
- Add tests proving OpenAI/Codex remains the fallback when no route is supplied.

### Stage 1: Auth Profile Resolution

- Resolve credentials by `provider_id + auth_profile_id`.
- Add request-auth lookup that accepts an explicit credential id.
- Preserve existing API-key precedence for providers with configured `env_key` or explicit bearer token.
- Keep provider-id-only lookup only for status/default compatibility where ambiguity is acceptable.
- Fail closed with an ambiguous-profile error when a provider route needs OAuth and multiple credentials exist without an explicit `auth_profile_id`.
- Add stable redacted auth status reasons: `ok`, `missing_credential`, `expired`, `refresh_failed`, `excluded_by_policy`, `no_model`.
- Keep refresh and persistence in login/auth-store.

### Stage 2: Model Client Route Injection

- Build model clients from a `ProviderRoute`.
- Remove assumptions that one thread provider owns all model calls.
- Prefer passing route context into existing model-client construction over changing `create_model_provider` globally.
- Keep provider-specific capability filtering close to existing native provider code.

### Stage 3: Sub-Agent Concurrency

- Extend sub-agent execution to carry explicit routes.
- Allow parent and sub-agents to use different providers concurrently.
- Treat provider/model route changes as context-epoch boundaries.

### Stage 4: UX And API Surface

- Update `/model` and provider picker behavior so global selection changes only the default route.
- Add per-sub-agent model/provider/profile selection where the existing sub-agent config surface already owns model overrides.
- Show redacted provider/profile status only.

### Stage 5: Optional Sidecar

- Add a sidecar only if in-process refresh locking and credential discovery prove insufficient.
- Keep sidecar scope limited to auth coordination and redacted status.
- Require a failing concurrency or cross-process refresh test before sidecar work starts.

## Consequences

Positive:

- OpenAI/Codex remains the default route and recovery path.
- Gemini/OAuth bugs are scoped to the Gemini route, not the whole thread.
- Sub-agents can run different providers at the same time.
- Existing owners stay intact.

Negative:

- Model-client construction needs route plumbing.
- Tests must cover mixed-provider execution, not just provider-specific auth.
- Any later sidecar must be tightly scoped to avoid becoming a second runtime stack.
- Explicit profile ids add UX work for account selection and removal.

## Acceptance Criteria

- A thread using OpenAI/Codex can spawn a Gemini sub-agent without changing the parent provider.
- Two sub-agents can run with different provider routes in the same parent turn.
- A sub-agent can select a non-default provider OAuth credential by explicit `auth_profile_id`.
- A sub-agent can run an API-key provider route without any OAuth profile.
- Multiple credentials for the same provider never resolve by accident; ambiguous routes fail with a redacted error.
- Provider routing uses stable provider ids even when provider display names differ.
- Missing or expired Gemini OAuth does not break the OpenAI/Codex default route.
- Provider OAuth status is redacted and scoped by provider/profile.
- No new provider registry, token store, or runtime stack is introduced.
- Sidecar work, if any, is optional and does not own provider routing.

## Challenge Result

Accepted direction, narrowed implementation.

The ADR is valid only if implementation starts at the existing low-impact sub-agent override and provider-auth seams. A broad rewrite of `create_model_provider`, a sidecar-first design, or provider-name-based auth identity would recreate the same exclusivity bug in a larger shape.
