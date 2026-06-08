# ADR: Native Heterogeneous Provider Engines

## Status

Proposed

## Date

2026-06-06

## Context

The current provider extensibility work introduced an internal descriptor seam for selecting provider runtime behavior.

Current code evidence:

- `ProviderEngine` currently has only `OpenAiResponses` and `AmazonBedrockResponses`: [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:7)
- `ProviderKind` maps those engines to the only current runtime implementations: [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:153)
- The remaining extensibility ADR explicitly defers public descriptors and external adapters until there is a real provider need: [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1)

This is enough for OpenAI-compatible providers and Bedrock, but it is not enough for native Claude, Gemini, and GitHub Copilot support. These providers are heterogeneous:

- Claude uses the Anthropic Messages API shape, Anthropic-specific headers, different tool/message translation rules, and different streaming events.
- Gemini uses the Google `generateContent` / `streamGenerateContent` API shape, Gemini content parts, safety settings, tool declarations, and `x-goog-api-key` or OAuth headers.
- GitHub Copilot uses Copilot-specific headers, GitHub-to-Copilot token exchange, and may route different models through different upstream API formats.

The `tmp/OmniRoute-main` codebase demonstrates one working architecture for this class of problem: a provider registry plus executor and translator layers.

OmniRoute should be treated as architectural evidence, not as an authoritative source for current model IDs, exact client-identifying headers, or provider policy. Any copied endpoint, header, model, or OAuth behavior must be independently verified against the target provider's public documentation or an approved internal compatibility decision before implementation.

## Gemini CLI Review Addendum

`ADR_GEMINI_CLI_TOOL_EXTENSIONS.md` originally proposed Gemini provider/auth points `1-40`. Those points are now delegated here because they extend the native heterogeneous provider plan rather than a Gemini CLI interop/import ADR.

Delegated original points:

- `1-20`: Gemini provider descriptor, `generateContent` contract, streaming, tool mapping, safety/generation config, model aliases, quota/error/retry/header/endpoint diagnostics.
- `21-40`: Gemini auth modes, API key, Vertex, OAuth, project ID, credential redaction, token refresh, enterprise auth, and auth readiness.

Architecture decision:

- API-key native Gemini support remains the first Gemini runtime milestone.
- Gemini OAuth, Vertex, and Gemini CLI managed-project auth remain separate follow-up slices.
- Provider work must extend the existing `ProviderEngine::GeminiGenerateContent`, provider descriptor, capability, and runtime-engine seams.
- Auth work must reuse existing login/RMCP credential boundaries and must not introduce a second credential broker.

Related source links:

- [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:7)
- [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:108)
- [client.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/client.rs:1584)
- [oauth.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:58)

## Crush Review Addendum

`ADR_CRUSH_TOOL_EXTENSIONS.md` originally proposed provider catalog/model/auth points `001-020` and auth/provider UX points `311-320`. Those points are delegated here only when they affect provider runtime descriptors, provider selection, capability metadata, model defaults, cost/capability diagnostics, or provider connection probes.

Delegated original points:

- `001-020`: provider catalog sync/diff/pin/rollback, default model selection, recent model tracking, model switching, flat-rate/cost/capability/reasoning/temperature diagnostics, extra headers/body, endpoint probes, and missing-credential diagnostics.
- `311-320`: API-key dialog/probe, missing-key onboarding, provider env suggestions, cloud credential detectors, subscription flags, and redacted auth fixtures when tied to provider runtime behavior.

Architecture decision:

- Extend `ProviderDescriptor`, `ProviderKind`, model-provider capabilities, and provider-owned diagnostics; do not add a Crush-style provider registry.
- Provider catalog update or model-switch behavior must remain internal until a separate config/schema ADR approves public keys.
- Auth-related provider UX must reuse existing login/provider auth boundaries and shared redaction; do not create a credential broker from Crush review items.
- Any model switching across an active session must preserve bounded context and be covered by session/resume compatibility tests.

Related source links:

- [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:66)
- [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:201)
- [spec_plan.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/tools/spec_plan.rs:190)

## OpenClaw Review Addendum

`ADR_OPENCLAW_TOOL_EXTENSIONS.md` originally proposed provider/model/media points `041-060`, `221-240`, and `321-340`. Those points are delegated here only when they extend provider descriptors, model capability metadata, model fallback behavior, provider-owned diagnostics, media capability metadata, or provider adapter planning.

Delegated original points:

- `041-060`: provider manifests, model catalog metadata, model allowlists, endpoint/request/response metadata, pricing, model ID normalization, fallback order, provider fixtures, and descriptor stubs.
- `221-240`: transcription, realtime voice, media understanding, image/video/music generation, and transcript-source metadata only as provider capability metadata; media runtimes and transcript ingestion remain lefties.
- `321-340`: provider runtime contracts, CLI backends, embedding/media providers, stream event fixtures, provider error/rate-limit/retry/context/tool-call/reasoning/cache/media input metadata, and staged adapter plans.

Architecture decision:

- OpenClaw provider plugins are evidence for descriptor-driven provider metadata, not a reason to add OpenClaw-style self-registering provider plugins to core.
- Native provider additions must extend `ProviderDescriptor`, `ProviderKind`, provider-owned capability metadata, and existing status surfaces.
- Executable provider adapters remain governed by `ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md`; this ADR accepts no plugin execution or gateway runtime.

Related source links:

- [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:7)
- [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:108)
- [client.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/client.rs:1584)

## Hermes Agent Review Addendum

`ADR_HERMES_AGENT_TOOL_EXTENSIONS.md` originally proposed provider/profile/model/runtime points `001-020`, provider auth classification points `021-040`, web-provider points `231-240`, multimodal capability points `255-256`, and diagnostics points `301-320`. Those points are delegated here only when they affect provider descriptors, runtime engines, capability metadata, model routing, catalog/probe behavior, provider-specific request quirks, or provider-owned status diagnostics.

Delegated original points:

- `001-020`: provider plugin manifests, provider profile mapping, aliases, API modes, fallback chains, model catalog probes, base-url guards, capabilities, auxiliary model routing, OpenAI-compatible endpoint reports, Anthropic/Bedrock/Copilot/Gemini/Nous provider reports, request headers/body quirks, and model catalog fetch strategies.
- `021-040`: auth type classification only when it changes provider runtime descriptors or provider-owned diagnostics; credential persistence remains in the provider-extensibility ADR.
- `231-240`: web-search provider capability mapping only if it extends existing provider/tool capability metadata rather than adding a product gateway.
- `255-256`: multimodal provider capability and fallback routing only as provider capability metadata; media runtime support is not delegated here.
- `301-320`: provider connectivity, usage/rate-limit/status diagnostics only when implemented through existing provider/status owners.

Architecture decision:

- Hermes provider plugins are architectural evidence for descriptor-driven provider metadata, not a license to add Python-style self-registering provider plugins to core.
- Native or external adapter support must extend `ProviderDescriptor`, `ProviderKind`, and the approved native/external adapter seams.
- Auth import, credential storage, plugin execution, managed tool gateways, and media providers remain outside this ADR unless a separate ADR accepts them.

Related source links:

- [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:7)
- [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:108)
- [card.rs](/opt/demodb/_workfolder/ontocode/codex-rs/tui/src/status/card.rs:161)

## GBrain Review Addendum

`ADR_GBRAIN_TOOL_EXTENSIONS.md` originally proposed provider/model/search/routing points `241-260` and diagnostics points `281-300`. Those points are delegated here only when they extend native provider descriptors, model capability metadata, provider-owned probes, rate-limit/status diagnostics, or cost/capability reporting.

Delegated original points:

- `241-260`: provider/model routing, account status, capability hints, and model metadata only when represented through native provider descriptors and provider-owned diagnostics.
- `281-300`: doctor/status categories only when they report native provider readiness, rate limits, account status, or model capability mismatches.

Architecture decision:

- GBrain does not justify a second provider registry or gateway in native provider support.
- Native provider additions must extend `ProviderDescriptor`, `ProviderKind`, provider-owned capability metadata, and existing status surfaces.
- Multimodal/media runtime behavior, remote brain services, and managed gateway execution remain outside this ADR unless separately approved.

## OpenCode Review Addendum

`ADR_OPENCODE_TOOL_EXTENSIONS.md` originally proposed provider/model/status points `008-009`, `141-160`, and `385`. Those points are delegated here only when they extend native provider descriptors, model capability metadata, provider-owned probes, model catalog behavior, provider-specific request quirks, rate-limit/status diagnostics, or cost/capability reporting.

Delegated original points:

- `008-009`: agent model/provider override and variant validation only when represented through provider descriptors and existing config/session owners.
- `141-160`: bundled-loader audit, header/SSE timeout diagnostics, model selector behavior, autoload readiness, public/free model fallback, env var detector, fuzzy model search, model status cache, model discovery, option partitioning, provider-specific endpoint helpers, provider transforms, error classification, cost visibility, variant reporting, default model resolver, OpenCode config import mapping, and duplicate registry detector.
- `385`: OpenCode provider compatibility matrix.

Architecture decision:

- OpenCode provider code is architectural evidence, not approval for a second provider registry or AI-SDK-style dynamic provider loader in core.
- Native provider additions must extend `ProviderDescriptor`, `ProviderKind`, provider-owned capabilities, and existing status surfaces.
- External/custom provider loading belongs in the external adapter ADR, not native provider support.

## CliRelay Review Addendum

`ADR_CLIRELAY_TOOL_EXTENSIONS.md` originally proposed provider/model/protocol/multimodal/status points `001-020`, `141-160`, `161-180`, `201-220`, and provider-facing diagnostics across `101-140`. Those points are delegated here only when they extend native provider descriptors, model capability metadata, provider-owned probes, model alias/exclusion behavior, provider-specific request quirks, rate-limit/status diagnostics, or cost/capability reporting.

Delegated original points:

- `001-020`: provider endpoint inventory, capability diff, auth-kind matrix, channel counts, aliases, redacted base URLs, header key names, excluded models, enabled state, proxy references, latency/failure surface mapping, OpenAI/Vertex/Amp/Kimi/Antigravity/iFlow/Qwen evidence.
- `101-140`: usage/latency/health metadata only when it becomes provider-owned diagnostics; request/response body retention, dashboards, and live monitoring stay blocked.
- `141-160`: model registry diff, owner/type mapping, alias diff, prefix policy, exclusions, quota-exceeded state, availability/count/overlap reports, pricing review, custom catalog disposition, image-generation capability, backfill evidence, registry hook mapping, fixtures, and deterministic snapshots.
- `161-180`: protocol format inventory, request/response/stream shape diffs, token/tool/thinking/error mapping, payload rule inventory, and transform conformance planning only after a native provider owner accepts the runtime behavior.
- `201-220`: multimodal capability inventory, image payload policy, MIME policy, provider matrix, fixtures, error taxonomy, and status-card notes as metadata only.

Architecture decision:

- CliRelay demonstrates gateway and registry patterns, but it does not justify a second provider registry, load balancer, model catalog, or OpenAI-compatible proxy inside Ontocode.
- Native provider additions must extend `ProviderDescriptor`, `ProviderKind`, provider-owned capability metadata, and existing status surfaces.
- Runtime request translation, stream parsing, model sync, quota routing, and media execution remain blocked unless native-engine slices accept them explicitly.

## Problem

Native provider support cannot be implemented safely by adding more conditional branches to OpenAI-compatible request code.

If Claude, Gemini, and Copilot are forced into the current OpenAI/Responses path, likely failures include:

- incorrect request bodies for non-OpenAI APIs
- incorrect streaming event parsing
- incomplete tool-call translation
- provider-specific auth and refresh logic leaking into generic code
- hard-coded model exceptions spread across runtime paths
- inability to add future heterogeneous providers without code churn

The provider abstraction must explicitly model runtime engines, not just model catalogs and base URLs.

## OmniRoute Examples

### Registry Shape

OmniRoute defines a registry entry with provider identity, wire format, executor, URL construction, auth metadata, headers, OAuth metadata, model metadata, and defaults: [providerRegistry.ts](/opt/demodb/_workfolder/ontocode/tmp/OmniRoute-main/open-sse/config/providerRegistry.ts:92)

Condensed example:

```ts
{
  id,
  format,
  executor,
  baseUrl,
  urlBuilder,
  authType,
  authHeader,
  headers,
  oauth,
  models,
  defaultContextLength,
}
```

Codex should not copy this TypeScript shape directly, but the separation is useful: provider metadata belongs in a registry/descriptor, while runtime behavior belongs in engine implementations.

### Claude Native Provider

OmniRoute models Claude as:

- `format: "claude"`
- `executor: "default"`
- Anthropic Messages endpoint
- OAuth/API-key style auth through `x-api-key`
- Anthropic version and beta headers
- Claude CLI-oriented header profile
- native Claude model metadata

Reference: [providerRegistry.ts](/opt/demodb/_workfolder/ontocode/tmp/OmniRoute-main/open-sse/config/providerRegistry.ts:621)

Implication for Codex:

- add a native `AnthropicMessages` engine
- translate internal request turns into Anthropic Messages requests
- translate Anthropic streaming events back into Codex events
- isolate Anthropic headers in a header-profile module
- support API key first; treat Claude OAuth import as separate from the native engine

### Gemini Native Provider

OmniRoute models Gemini API-key support as:

- `format: "gemini"`
- Google Generative Language base URL
- URL builder for `models/{model}:generateContent`
- URL builder for `models/{model}:streamGenerateContent?alt=sse`
- auth through `x-goog-api-key`
- Gemini model metadata including tool and vision flags

Reference: [providerRegistry.ts](/opt/demodb/_workfolder/ontocode/tmp/OmniRoute-main/open-sse/config/providerRegistry.ts:703)

Implication for Codex:

- add a native `GeminiGenerateContent` engine
- map messages to Gemini `contents` and `parts`
- map tools to Gemini function declarations
- map generation controls to `generationConfig`
- parse both non-streaming and SSE stream outputs
- support API key first because it avoids the more complex Gemini CLI project bootstrap flow

### Gemini CLI Provider

OmniRoute also has a separate Gemini CLI executor:

- `format: "gemini-cli"`
- `executor: "gemini-cli"`
- Cloud Code PA internal base URL
- managed project loading and onboarding
- CLI-specific User-Agent and `X-Goog-Api-Client`
- OAuth bearer token headers

References:

- Registry entry: [providerRegistry.ts](/opt/demodb/_workfolder/ontocode/tmp/OmniRoute-main/open-sse/config/providerRegistry.ts:777)
- Header profile: [geminiCliHeaders.ts](/opt/demodb/_workfolder/ontocode/tmp/OmniRoute-main/open-sse/services/geminiCliHeaders.ts:1)

Implication for Codex:

- Gemini CLI is a different engine from Gemini API-key support.
- It should not be included in the first native Gemini milestone.
- It requires explicit acceptance criteria around OAuth, project discovery, token refresh, and policy.

### GitHub Copilot Provider

OmniRoute models GitHub Copilot as:

- `format: "openai"` for chat-compatible paths
- `executor: "github"`
- Copilot chat completions endpoint
- Copilot Responses endpoint for selected models
- OAuth auth
- Copilot-specific request headers
- GitHub access token to Copilot token refresh

References:

- Registry entry: [providerRegistry.ts](/opt/demodb/_workfolder/ontocode/tmp/OmniRoute-main/open-sse/config/providerRegistry.ts:995)
- Executor: [github.ts](/opt/demodb/_workfolder/ontocode/tmp/OmniRoute-main/open-sse/executors/github.ts:10)
- URL selection by target format: [github.ts](/opt/demodb/_workfolder/ontocode/tmp/OmniRoute-main/open-sse/executors/github.ts:27)
- Copilot token refresh: [github.ts](/opt/demodb/_workfolder/ontocode/tmp/OmniRoute-main/open-sse/executors/github.ts:158)

Implication for Codex:

- add a `GitHubCopilot` engine instead of treating Copilot as plain OpenAI-compatible
- keep Copilot headers in a dedicated header profile
- model the refresh chain explicitly: GitHub OAuth token -> Copilot internal token
- support Copilot API paths before considering Copilot Web or browser-session flows

### Executor Registry

OmniRoute routes specialized providers through executor classes and falls back to a default executor for simpler providers: [index.ts](/opt/demodb/_workfolder/ontocode/tmp/OmniRoute-main/open-sse/executors/index.ts:52)

Implication for Codex:

- `ProviderEngine` should select a runtime implementation.
- Each native engine should own URL construction, headers, request translation, response translation, and refresh hooks.
- Generic provider code should not know provider-specific protocol details.

## Decision Drivers

- support native Claude, Gemini, and GitHub Copilot without pretending they are OpenAI-compatible
- keep provider-specific protocol rules out of generic provider code
- preserve current OpenAI-compatible and Bedrock behavior
- allow future heterogeneous providers to add engines without refactoring selector logic each time
- keep public configuration stable until internal native engines prove the abstraction
- avoid importing risky web-session, MITM, or foreign credential behavior into core

## Options Considered

### Option 1: Add Provider-Specific Branches To Existing Provider Code

Add Claude, Gemini, and Copilot conditionals in the current provider implementation.

Pros:

- fastest short-term path
- fewer new files initially

Cons:

- repeats the exact branching problem the descriptor work is trying to avoid
- makes request and response translation hard to test in isolation
- grows high-touch provider code
- does not scale to additional heterogeneous providers

Verdict: rejected.

### Option 2: Add Internal Native Provider Engines

Extend `ProviderEngine` and `ProviderKind` with explicit native engines:

- `AnthropicMessages`
- `GeminiGenerateContent`
- `GitHubCopilot`

Each engine owns its own request translation, response translation, URL building, headers, auth application, model capability interpretation, and refresh hooks.

Pros:

- clear runtime boundary
- minimal public config impact
- easy to stage and test per provider
- aligns with the current descriptor seam
- keeps provider-specific code localized

Cons:

- requires new engine modules and integration tests
- still requires code changes for each new engine

Verdict: approved as the next implementation direction.

### Option 3: Internal Registry Plus Built-In Engine Descriptors

Keep the native engines from Option 2, but drive built-in provider metadata from an internal Rust registry inspired by OmniRoute.

Example internal descriptor fields:

```rust
struct BuiltInProviderDescriptor {
    id: &'static str,
    engine: ProviderEngine,
    auth_scheme: ProviderAuthScheme,
    base_url: &'static str,
    header_profile: HeaderProfile,
    model_catalog: BuiltInModelCatalog,
    capabilities: ProviderCapabilities,
}
```

Pros:

- separates metadata from runtime behavior
- avoids a large `match` statement as providers grow
- prepares for future public descriptors

Cons:

- more abstraction than required for the first native provider
- must avoid becoming a public plugin system prematurely

Verdict: approved after the first native engine lands, or sooner if the implementation starts duplicating metadata.

### Option 4: External Provider Adapter Runtime

Run providers through out-of-process adapters. Codex sends a normalized request and receives a normalized stream.

Pros:

- strongest path for adding many future providers without changing Codex core
- isolates experimental providers
- allows independent provider release cadence

Cons:

- security boundary and trust model are non-trivial
- streaming, cancellation, logging, and secret handling need a separate protocol
- not required to support Claude, Gemini, and Copilot API paths

Verdict: deferred to a separate ADR.

### Option 5: MITM Or Web-Session Provider Support

Adopt OmniRoute-style MITM or browser-session provider paths for Claude Code, Copilot Web, Gemini Web, and similar providers.

Pros:

- can reach services that do not expose stable public APIs
- useful for research and compatibility experiments

Cons:

- certificate, DNS, browser-token, and session-cookie risks
- fragile upstream behavior
- difficult to support safely in a general-purpose CLI
- likely to violate user expectations for credential boundaries

Verdict: rejected for core native provider support. Consider only as an explicitly opt-in external adapter experiment.

## Decision

Implement native heterogeneous provider support through internal provider engines.

Approved first-class engine targets:

- `AnthropicMessages`
- `GeminiGenerateContent`
- `GitHubCopilot`

Deferred targets:

- `GeminiCli`
- `ClaudeCodeCompatible`
- `CopilotWeb`
- general external adapter runtime
- public user-defined heterogeneous provider descriptors

The first implementation must be built-in-only and keep public configuration unchanged. Do not add public `WireApi` values such as `claude`, `gemini`, or `copilot` until at least two native engines prove the runtime boundary and the schema compatibility impact is reviewed.

## Runtime Boundary Decision

The hard boundary is not provider selection; it is request execution.

Current code evidence shows:

- `ModelProvider` mostly exposes provider metadata, auth, account state, capabilities, and model-manager creation: [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:88)
- `ProviderKind` currently selects between configured OpenAI-compatible and Bedrock providers: [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:153)
- the core request path is still Responses-oriented through `ModelClientSession::stream_responses_api`: [client.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/client.rs:1243)

Therefore native engines cannot be only metadata descriptors. The implementation must introduce a protocol-neutral runtime boundary that converts a normalized Codex turn into a normalized Codex event stream.

Required shape:

- provider selection remains descriptor-driven
- the runtime execution seam chooses a provider runtime before building an OpenAI Responses HTTP request
- OpenAI-compatible providers continue using the existing Responses path
- native runtimes own request translation, response translation, stream parsing, provider errors, and usage mapping
- generic session, guardian, multi-agent, and context code consume normalized Codex events and must not depend on provider-specific wire formats

Do not push Claude/Gemini/Copilot branches into `stream_responses_api`.

## Blast Radius

GitNexus impact analysis for `create_model_provider` reported **CRITICAL** risk:

- 19 direct callers
- 46 affected processes
- 11 affected modules
- affected areas include core session setup, guardian tests, multi-agent tests, remote model handling, CLI doctor/debug paths, provider tests, and config surfaces

Implementation must be staged narrowly:

- run impact analysis before editing each provider/runtime symbol
- preserve the existing `create_model_provider` call contract in the first stage
- add new runtime behavior behind provider descriptors rather than changing all callers
- run `gitnexus_detect_changes()` before finalizing each implementation batch
- warn explicitly before proceeding if impact remains HIGH or CRITICAL for the edited symbol

## Proposed Architecture

### Provider Runtime

Extend the engine selection concept so each engine maps a normalized Codex turn to a normalized Codex event stream.

The runtime must own:

- provider identity
- request URL construction
- request header construction
- auth application and refresh hooks
- request translation
- response translation
- stream event translation
- usage mapping
- provider error normalization
- retry and timeout compatibility
- cancellation propagation
- model capability mapping

Conceptual shape:

```rust
trait ProviderRuntime {
    fn execute(
        &self,
        turn: NormalizedProviderTurn,
    ) -> impl std::future::Future<Output = Result<NormalizedProviderEventStream>> + Send;
}
```

Do not expose this trait publicly until at least two native engines are implemented and the required API surface is proven.

### Header Profiles

Provider-specific headers should be isolated.

Examples:

- Anthropic version, beta, app, and client headers
- Gemini `x-goog-api-key` and optional Google API client headers
- GitHub Copilot editor, plugin, integration, API-version, and initiator headers

Generic provider code should receive already-built headers and should not know provider-specific header names.

### Request Translators

Each engine needs a translator from Codex's internal model request shape into the target API.

Minimum translator responsibilities:

- messages and system instructions
- tool declarations
- tool-call outputs
- reasoning/thinking controls
- response format or JSON schema behavior
- max output tokens and sampling parameters
- image or multimodal parts where supported

### Response Translators

Each engine needs deterministic translation back into Codex events.

Minimum response responsibilities:

- assistant text deltas
- tool-call deltas and completed tool calls
- reasoning/thinking deltas where supported
- usage accounting where available
- provider errors with safe redaction
- finish reasons

### Credential Refresh

Refresh must remain provider-specific.

Initial scope:

- Claude API key: no refresh.
- Gemini API key: no refresh.
- GitHub Copilot: refresh GitHub OAuth token if needed, then exchange for Copilot token.

Deferred scope:

- Claude OAuth import or Claude Code credentials.
- Gemini CLI OAuth and managed project discovery.
- Copilot Web session cookies or browser tokens.

### Credential Shape By Engine

| Engine | First credential shape | Refresh behavior | Deferred credential shape |
| --- | --- | --- | --- |
| `AnthropicMessages` | Anthropic API key from provider env/config auth | no refresh | Claude OAuth import, Claude Code credentials |
| `GeminiGenerateContent` | Google AI Studio API key through `x-goog-api-key` | no refresh | Google OAuth |
| `GitHubCopilot` | GitHub OAuth access token plus provider-specific Copilot token cache | refresh GitHub token if possible, then exchange for Copilot token | Copilot Web/browser session |

Credential records must include enough provenance to avoid applying a token to the wrong provider or account:

- provider id
- auth scheme
- account identity when known
- source of credential
- expiry
- refreshability
- redaction policy

### Compatibility Surfaces

The first implementation must not silently change these surfaces:

- `ModelProviderInfo` public config shape, including the single current `WireApi::Responses` value: [lib.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider-info/src/lib.rs:50)
- config schema generated from provider config types
- remote thread config serialization and deserialization: [remote.rs](/opt/demodb/_workfolder/ontocode/codex-rs/config/src/thread_config/remote.rs:145)
- app-server account-state behavior
- auth environment telemetry redaction and bucketing: [auth_env_telemetry.rs](/opt/demodb/_workfolder/ontocode/codex-rs/login/src/auth_env_telemetry.rs:31)
- model catalog merge behavior
- session resume behavior
- guardian and multi-agent turn execution

If native providers require public config fields, that must be a separate schema/migration change with compatibility tests.

## Implementation Plan

### Stage 1A: Built-In Descriptor Selection

Add only the minimum descriptor surface needed to select a built-in native engine.

Required outputs:

- `ProviderEngine::AnthropicMessages`
- descriptor tests proving OpenAI-compatible providers still select `OpenAiResponses`
- descriptor tests proving Bedrock still selects `AmazonBedrockResponses`
- descriptor tests proving Claude built-in metadata selects `AnthropicMessages`
- no public config schema change

### Stage 1B: Runtime Execution Seam

Add an internal protocol-neutral runtime seam before implementing the first native provider.

Required outputs:

- runtime-selected execution path that preserves the existing Responses path for OpenAI-compatible providers
- no provider-specific branches in `stream_responses_api`
- cancellation, timeout, retry, usage, and provider-error behavior documented for the seam
- test harness for normalized provider event streams
- request translation unit tests
- response/stream translation tests

### Stage 2: Claude API-Key Native Engine

Implement Anthropic Messages support with API-key auth.

Required behavior:

- build `POST /v1/messages`
- set Anthropic version headers
- translate Codex turns into Anthropic messages
- translate tools into Anthropic tool declarations
- translate Anthropic stream events into Codex events
- normalize Anthropic errors with secret redaction
- preserve existing OpenAI and Bedrock tests

Explicit exclusions:

- no Claude Code credential import
- no foreign credential store reading
- no MITM path

### Stage 3: Gemini API-Key Native Engine

Implement Google Generative Language support with API-key auth.

Required behavior:

- build `models/{model}:generateContent`
- build `models/{model}:streamGenerateContent?alt=sse`
- set `x-goog-api-key`
- translate messages to Gemini contents and parts
- translate tools to Gemini function declarations
- handle Gemini safety and generation config defaults
- parse Gemini SSE output
- normalize Gemini errors with secret redaction

Explicit exclusions:

- no Gemini CLI OAuth
- no Cloud Code PA project bootstrap
- no browser-session flow

### Stage 4: GitHub Copilot API Engine

Implement Copilot API support through GitHub OAuth and Copilot token exchange.

Required behavior:

- build Copilot chat completions requests
- optionally route selected models to Copilot Responses endpoint
- apply Copilot header profile
- exchange GitHub access token for Copilot token
- refresh GitHub token when refresh token is present
- redact all token values in logs and diagnostics
- test model-specific routing between Copilot chat-compatible and Responses-compatible paths if both are supported

Explicit exclusions:

- no Copilot WebSocket/browser-session executor
- no MITM target

### Stage 5: Internal Registry Hardening

If Stages 2 through 4 duplicate provider metadata, introduce an internal built-in provider registry.

Required behavior:

- engine selected from internal descriptor
- auth scheme selected from internal descriptor
- header profile selected from internal descriptor
- model catalog selected from internal descriptor
- tests prove existing configured providers still map to `OpenAiResponses`

### Stage 6: Reopen External Adapter ADR

Only after native engines are proven, revisit external adapters for providers that cannot be supported through stable API engines.

Follow-up ADR: [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](/opt/demodb/_workfolder/ontocode/ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md)

The external adapter ADR should compare these implementation options.

#### Option 6A: Stdio Adapter Protocol

Codex launches an adapter command and exchanges newline-delimited JSON or JSON-RPC over stdin/stdout.

Adapter responsibilities:

- provider-specific auth and refresh
- provider-specific request translation
- provider-specific streaming response translation
- model listing and capability reporting
- provider error normalization
- secret redaction before diagnostics cross the adapter boundary

Pros:

- best first transport because process lifetime, cancellation, and stderr capture are already familiar CLI concerns
- no localhost port exposure
- easy to sandbox, kill, version-gate, and test with fixture transcripts
- language-agnostic adapter implementation

Cons:

- requires a stable stream framing protocol
- backpressure, cancellation, and partial-output handling must be specified precisely
- long-lived caches require either adapter-managed files or explicit lifecycle methods

Verdict: recommended first N6 ADR direction.

#### Option 6B: MCP Provider Adapter

Treat provider runtimes as MCP-like servers with provider-specific methods for model listing and streaming.

Pros:

- reuses existing MCP process and tool-server operational patterns
- useful if an adapter also exposes tools, resources, or provider metadata
- can share some trust and lifecycle language with existing MCP support

Cons:

- MCP tool-call semantics are not the same as model streaming semantics
- risk of overloading MCP with provider-runtime behavior that should remain separate
- requires careful separation between model execution and tool execution capabilities

Verdict: possible later convergence, but not the first adapter contract.

#### Option 6C: Local HTTP Sidecar Adapter

Codex starts or connects to a localhost adapter service exposing endpoints such as `/health`, `/models`, and `/stream`.

Pros:

- easy to debug with common HTTP tooling
- suitable for long-running adapters with internal caches
- natural fit for SSE stream framing

Cons:

- adds localhost port exposure and stale-daemon cleanup risks
- requires authentication between Codex and the sidecar
- harder to make lifecycle deterministic across platforms

Verdict: keep as a future transport if stdio proves too limiting.

#### Option 6D: Sandboxed WASI Plugin Runtime

Codex loads provider adapters as WASM/WASI modules under a constrained runtime.

Pros:

- strongest isolation story among in-process-style adapters
- deterministic distribution and versioning
- easier to restrict filesystem and network capabilities explicitly

Cons:

- high implementation cost
- provider SDKs, native TLS, OAuth flows, and streaming HTTP are harder inside WASI
- not required to unblock heterogeneous providers after native engines

Verdict: defer until the adapter protocol is proven.

#### Option 6E: OpenAI-Compatible Proxy Contract

Require external adapters to expose a local OpenAI-compatible or Responses-compatible endpoint.

Pros:

- minimal Codex runtime change
- useful for simple providers already compatible with OpenAI-style APIs
- easy for existing proxy projects to integrate

Cons:

- poor fit for truly heterogeneous providers
- flattens provider capabilities into fake OpenAI semantics
- leaks tool-call, streaming, auth-refresh, and model-capability mismatches into proxy-specific behavior

Verdict: acceptable as a compatibility bridge, rejected as the primary external adapter architecture.

Required ADR topics for the recommended stdio-first design:

- adapter process protocol
- secret passing and redaction
- trust model
- cancellation and timeout semantics
- stream framing
- version compatibility
- installation and update policy
- capability negotiation
- model listing contract
- credential provenance and account scoping
- adapter sandboxing and filesystem/network access policy
- stderr/stdout logging rules
- adapter crash and restart behavior
- transcript fixture format for conformance tests

Minimum first ADR acceptance criteria:

- Codex can launch a configured adapter process without changing public native-engine config.
- Codex and the adapter negotiate protocol version and capabilities before model execution.
- Codex sends a bounded normalized request and receives a bounded normalized event stream.
- Cancellation terminates the in-flight adapter request and does not leave orphaned processes.
- Secrets are passed through explicit credential channels and never through diagnostic text.
- Adapter stderr is size-capped and redacted before surfacing to users or logs.
- The adapter protocol has golden transcript tests for success, tool call, provider error, cancellation, and crash cases.
- The ADR explicitly rejects MITM, browser-cookie, and web-session adapters for core support unless a later security review approves them.

## Acceptance Criteria

- Claude, Gemini, and Copilot can be selected as built-in providers without hard-coded request branches in generic provider code.
- Native providers do not add public `WireApi` values or config fields in the first implementation.
- Each native engine has isolated golden request translation fixtures.
- Each native engine has non-streaming response translation tests.
- Each native engine has streaming fixture tests.
- Each native engine has provider error redaction tests.
- Copilot has token refresh and Copilot-token exchange tests.
- Gemini has tool declaration and multimodal/content-part translation tests where supported.
- Claude has tool-use and tool-result translation tests.
- Each native engine has at least one integration-style test covering streamed assistant output through the normalized runtime seam.
- Provider-specific auth and refresh logic does not enter generic OpenAI-compatible provider code.
- Existing OpenAI-compatible provider behavior is unchanged.
- Existing Bedrock provider behavior is unchanged.
- Guardian and multi-agent tests that traverse provider creation remain green.
- Remote thread config tests remain green.
- Config schema is unchanged unless a separate schema ADR approves a change.
- `gitnexus_detect_changes()` shows only expected provider/runtime/doc impacts before finalization.
- No public descriptor/config expansion is introduced until a concrete built-in provider requires it.
- No MITM, browser-cookie, or foreign credential import path is added as part of this ADR.

## Risks

- Native translators may become large if all provider quirks are handled in one module.
- Copilot behavior may change upstream and break token exchange or header expectations.
- Gemini tool-call and multimodal semantics may require more normalization than Claude.
- Claude OAuth and Claude Code compatibility may be incorrectly conflated with Anthropic API support.

Mitigations:

- keep one module per provider engine
- keep translator tests close to each engine
- keep credential import in separate ADRs
- keep web/MITM support outside core
- stage providers one at a time
- treat OmniRoute headers and model IDs as examples requiring independent validation

## Non-Goals

- building a public plugin system in this ADR
- supporting arbitrary providers without code changes in the first implementation
- importing Claude Code credentials
- supporting Gemini CLI OAuth in the first Gemini milestone
- supporting Copilot Web or browser-session tokens
- replacing the existing MCP OAuth store
- changing app-server API surfaces

## Readiness Answer

The codebase is not yet ready for native Claude, Gemini, and Copilot as plenty heterogeneous providers.

It is ready for the next incremental step: add internal native engines behind the existing provider descriptor seam. That path supports real Claude, Gemini, and Copilot API behavior without committing prematurely to public provider plugins or risky web-session compatibility layers.

## Follow-Up Tracking

Create a tracking file only when implementation starts, with one task group per stage:

- Stage 1A: built-in descriptor selection
- Stage 1B: runtime execution seam
- Stage 2: Claude native engine
- Stage 3: Gemini native engine
- Stage 4: GitHub Copilot native engine
- Stage 5: internal registry hardening
- Stage 6: external adapter ADR
