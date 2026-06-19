# ADR: External Provider Adapter Runtime

## Status

Proposed

## Date

2026-06-06

## Context

Native provider engines now cover the first built-in heterogeneous providers: Anthropic/Claude, Gemini, and GitHub Copilot. That is the right architecture for providers with stable APIs that Codex can support directly.

There remains a different problem: future providers may need support without waiting for a Codex Rust release, may have unstable request formats, or may require provider-specific auth and streaming logic that should not live in Codex core. The earlier native-engine ADR deferred this problem to a separate external adapter ADR after proving the internal runtime boundary.

This ADR defines the first external adapter direction. It does not change the public native engine configuration shape, does not add schema-backed provider engine fields, and does not implement adapters.

Any implementation configuration implied by this ADR is provisional and private. Public configuration, schema generation, app-server exposure, and migration behavior require a later schema ADR.

## Hermes Agent Review Addendum

`ADR_HERMES_AGENT_TOOL_EXTENSIONS.md` originally proposed provider plugin/runtime extensibility points `001-020`, external-process auth point `040`, plugin runtime points `321-340`, and provider/tool-gateway ideas across `221-240`. Those points are delegated here only when they require supporting future heterogeneous providers without a core Rust release.

Delegated original points:

- `001-020`: provider manifests/profiles and request quirks only when they cannot be represented as built-in descriptors or native engines.
- `040`: external-process auth only as part of a constrained provider adapter contract.
- `321-340`: plugin trust, quarantine, dependency scanning, version policy, manifest schema, and disabled-reason diagnostics when tied to adapter runtime safety.
- `221-240`: managed web/browser/tool-gateway ideas only if they become provider-adapter capability metadata; product gateway behavior remains lefties.

Architecture decision:

- Hermes-style drop-in Python provider plugins are not accepted as-is. Any adapter must use the approved stdio JSON-RPC/NDJSON transport, explicit user opt-in, bounded streams, redacted diagnostics, deterministic cancellation, and conformance tests.
- MCP must not be overloaded as the first model-provider adapter protocol.
- Public config/schema/app-server exposure still requires a later compatibility ADR.

Related source links:

- [provider.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/model-provider/src/provider.rs:108)
- [spec_plan.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:190)
- [plugin_namespace.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/utils/plugins/src/plugin_namespace.rs:81)

## GBrain Review Addendum

`ADR_GBRAIN_TOOL_EXTENSIONS.md` originally proposed AI gateway, model/tool routing, remote brain client, hosting, and proxy points across `241-260` and `351-360`. Those points are delegated here only if they become explicit external provider adapter capabilities under the approved adapter transport.

Delegated original points:

- `241-250`: gateway/routing ideas only when they describe provider-adapter capability metadata or request translation behavior.
- `251-260`: provider account/auth status only when it is adapter readiness metadata; credential persistence stays in provider/auth ADRs.
- `351-360`: remote brain thin-client/proxy ideas only if they become a constrained provider adapter, not a general remote memory service.

Architecture decision:

- GBrain-style gateway behavior is not accepted as a model-provider runtime by itself.
- Any accepted adapter must use the ADR-approved stdio JSON-RPC/NDJSON boundary, explicit opt-in, bounded streams, deterministic cancellation, redacted diagnostics, and conformance tests.
- MCP must not be used as the first model-provider adapter protocol, and remote memory/brain APIs remain lefties unless a separate service ADR is accepted.

## OpenCode Review Addendum

`ADR_OPENCODE_TOOL_EXTENSIONS.md` originally proposed plugin/provider-adapter points `101-120`, `150`, `154`, and `384`. Those points are delegated here only if they become explicit external provider adapter capabilities under the approved adapter transport.

Delegated original points:

- `101-120`: plugin origin preservation, local path resolution, spec dedupe, compatibility gates, pure mode, load-stage diagnostics, missing-entrypoint reports, dependency wait retry, dynamic-import cache warnings, built-in auth adapter audit, workspace adapter quarantine, plugin error event bridge, source classifier, deprecated package mapping, server/TUI split, option redaction, lifecycle tests, local-file trust prompt, manifest source-link checker, and lefties routing.
- `150`: custom model discovery only through the approved external adapter boundary.
- `154`: provider auth plugin quarantine only as adapter/import metadata; credential persistence stays in provider/auth ADRs.
- `384`: OpenCode plugin compatibility matrix.

Architecture decision:

- OpenCode-style TypeScript plugin execution is not accepted as-is.
- Any accepted adapter must use the ADR-approved stdio JSON-RPC/NDJSON boundary, explicit opt-in, bounded streams, deterministic cancellation, redacted diagnostics, and conformance tests.
- Workspace plugin execution, npm install, dynamic import, and server/TUI plugin entrypoints remain lefties unless a separate plugin security ADR is accepted.

## CliRelay Review Addendum

`ADR_CLIRELAY_TOOL_EXTENSIONS.md` originally proposed gateway/adapter/protocol/stream points `014-016`, `161-200`, `261-280`, and SDK extensibility points `381-400`. Those points are delegated here only if they become explicit external provider adapter capabilities under the approved adapter transport.

Delegated original points:

- `014-016`: OpenAI-compatible, Vertex-compatible, and Amp upstream evidence only as possible adapter targets.
- `161-180`: request/response/stream format translation, token-count transforms, tool-call mapping, thinking field mapping, payload-rule review, schema normalization, provider-specific fixtures, error mapping, and conformance planning.
- `181-200`: SSE/WebSocket metadata, chunk redaction, first-token/completion/error metrics, tool events, bounded capture policy, and transport policy notes. WebSocket relay behavior remains blocked.
- `261-280`: proxy/TLS/cloaking metadata only when an adapter needs explicit network capability metadata. Existing network-proxy remains the owner for proxy policy.
- `381-400`: authenticator/access/usage/translator abstraction reviews, custom provider examples, startup registration mapping, excluded-model/OAuth/model-registry/path-routing/access/watcher/store/redaction test evidence, low-agent task split, and challenge checklist.

Architecture decision:

- CliRelay's Go proxy server is not accepted as an Ontocode adapter runtime.
- Any accepted adapter must use the ADR-approved stdio JSON-RPC/NDJSON boundary, explicit opt-in, bounded streams, deterministic cancellation, redacted diagnostics, and conformance tests.
- Localhost proxy daemons, WebSocket relays, API-key management, request body logging, remote persistence, and panel/update behavior remain lefties unless separate ADRs accept them.

## OpenClaw Review Addendum

`ADR_OPENCLAW_TOOL_EXTENSIONS.md` originally proposed provider plugin/runtime, gateway protocol, stream, and adapter-planning points `041-060`, `181-200`, and `321-340`. Those points are delegated here only if they become explicit external provider adapter capabilities under the approved adapter transport.

Delegated original points:

- `041-060`: provider endpoint/request/response shape metadata only when it cannot be represented as native provider descriptors.
- `181-200`: MCP/bundle metadata only as adapter-import evidence; MCP must not become the first model-provider adapter protocol.
- `321-340`: provider runtime contracts, CLI backends, stream events, error/rate-limit/retry mapping, native tool metadata, endpoint auth requirements, adapter fixtures, duplicate-runtime guards, and staged adapter plans.

Architecture decision:

- OpenClaw's gateway and plugin runtime are not accepted as an Ontocode adapter runtime.
- Any accepted adapter must use the ADR-approved stdio JSON-RPC/NDJSON boundary, explicit opt-in, bounded streams, deterministic cancellation, redacted diagnostics, and conformance tests.
- OpenClaw plugin execution, gateway WebSocket relay, MCP overload, channel transports, cron jobs, browser/canvas runtimes, and node pairing remain lefties unless separate ADRs accept them.

## Problem

Codex needs a path to support many future heterogeneous providers without core code changes while preserving:

- credential safety
- bounded request and stream sizes
- deterministic cancellation
- safe diagnostics and redaction
- stable version compatibility
- testable provider behavior

Flattening every provider into an OpenAI-compatible proxy loses provider-specific capabilities and pushes auth, tool-call, streaming, and error mismatches into opaque proxy behavior. Adding every provider as a built-in Rust engine also does not scale to experimental or fast-changing providers.

## Decision Drivers

- keep provider-specific protocol behavior out of generic Codex runtime paths
- avoid exposing secrets through command arguments, logs, stderr, or transcript fixtures
- make cancellation and process cleanup deterministic across platforms
- support language-agnostic adapters
- require explicit user opt-in before running adapter code
- keep adapter streams bounded and normalized before entering Codex context
- provide conformance tests so adapters can be validated independently
- avoid MITM, browser-cookie, and web-session credential paths in core support
- prevent accidental execution of untrusted adapter code through auto-discovery, PATH surprises, shell expansion, or workspace-controlled files

## Options Considered

### Option A: Stdio JSON-RPC Or NDJSON Adapter

Codex launches an adapter command and exchanges framed JSON messages over stdin/stdout. The adapter owns provider-specific auth, request translation, streaming response parsing, model listing, and provider error normalization.

Pros:

- no localhost port exposure
- process lifetime, stderr capture, cancellation, and exit status are familiar CLI concerns
- language-agnostic adapter implementation
- easy to sandbox, kill, version-gate, and test with transcript fixtures
- can support JSON-RPC request/response calls plus NDJSON-style stream events

Cons:

- stream framing, backpressure, partial writes, and stderr behavior must be specified precisely
- long-lived provider caches need adapter-managed files or explicit lifecycle methods
- large stream outputs need strict caps before entering Codex runtime

Verdict: approved first transport.

### Option B: MCP-Style Provider Adapter

Treat provider runtimes as MCP-like servers with model-listing and model-execution methods.

Pros:

- reuses some existing process and server lifecycle concepts
- useful if an adapter also exposes provider metadata, tools, or resources
- familiar to users already configuring local MCP servers

Cons:

- MCP tool-call semantics are not the same as model streaming semantics
- overloading MCP could blur the boundary between model execution and tool execution
- provider runtime security policy differs from tool-server policy

Verdict: do not overload MCP for the first adapter contract. A later ADR may define convergence points if the protocols remain clearly separated.

### Option C: Local HTTP Sidecar

Codex starts or connects to a localhost adapter service that exposes endpoints such as `/health`, `/models`, `/execute`, and `/cancel`.

Pros:

- easy to inspect with common HTTP tooling
- natural fit for SSE streaming
- good for long-running adapters with internal caches

Cons:

- adds localhost port exposure
- requires authentication between Codex and the sidecar
- creates stale-daemon and port-conflict failure modes
- harder to make lifecycle deterministic across platforms

Verdict: keep as a future transport after the stdio protocol is proven.

### Option D: WASI Plugin Runtime

Codex loads provider adapters as sandboxed WASM/WASI modules.

Pros:

- strong isolation story
- deterministic packaging and versioning
- explicit filesystem and network capability grants

Cons:

- high implementation cost
- provider SDKs, native TLS, OAuth, and streaming HTTP are harder inside WASI
- not needed to prove the external adapter contract

Verdict: defer until the adapter protocol and security model are proven.

### Option E: OpenAI-Compatible Proxy Contract

Require external adapters to expose a local OpenAI-compatible or Responses-compatible endpoint.

Pros:

- minimal Codex runtime changes
- useful for simple providers that already mimic OpenAI APIs
- compatible with existing proxy projects

Cons:

- poor fit for truly heterogeneous providers
- hides provider capability mismatches behind fake OpenAI semantics
- weak support for provider-specific auth refresh, tool-call streams, and model capability negotiation

Verdict: acceptable only as a compatibility bridge, not as the primary external adapter architecture.

## Decision

Use a stdio-first external provider adapter protocol.

Codex will launch an explicitly configured adapter process and communicate through framed JSON over stdin/stdout. Launching an adapter is an explicit trust decision: Codex must not discover, download, install, or execute adapters implicitly.

The adapter is responsible for provider-specific auth, request translation, streaming response translation, model listing, provider errors, and safe redaction before diagnostic text crosses the adapter boundary.

HTTP sidecars remain a possible future transport. MCP must not be overloaded with model execution semantics in the first adapter contract. WASI is deferred until the protocol is proven. OpenAI-compatible proxies are allowed only as a bridge for already-compatible providers.

## Conceptual Protocol

This section defines protocol shape, not final Rust types.

### Handshake And Capabilities

Codex starts the adapter and sends a handshake message before any model execution:

```json
{
  "type": "handshake",
  "protocol_version": "provider-adapter.v1",
  "codex_version": "x.y.z",
  "requested_capabilities": ["model_list", "execute_stream", "cancel"]
}
```

The adapter responds with its supported protocol version, provider identity, supported methods, stream limits, credential requirements, and declared event types:

```json
{
  "type": "handshake_result",
  "protocol_version": "provider-adapter.v1",
  "adapter_name": "example-provider",
  "adapter_version": "1.0.0",
  "provider_id": "example",
  "capabilities": {
    "model_list": true,
    "execute_stream": true,
    "cancel": true,
    "tool_calls": true,
    "multimodal_input": false
  },
  "limits": {
    "max_request_bytes": 1048576,
    "max_event_bytes": 65536,
    "max_stderr_bytes": 65536
  }
}
```

Codex rejects adapters with unsupported versions, missing required capabilities, or unsafe declared limits.

Codex must complete protocol and capability negotiation before sending credential references or raw credential material. Provider identity returned by the adapter must match the configured provider id and the credential scope Codex is willing to expose.

Capability negotiation is an allowlist, not a feature discovery shortcut. Unknown capabilities, unknown event types, and requested credential classes are rejected unless the current protocol version explicitly defines them.

### Stream Framing

The v1 stdio transport uses UTF-8 line-delimited JSON frames: one compact JSON object per line on stdout. Pretty-printed JSON, binary payloads, control frames outside JSON, and free-form stdout text are invalid.

Required framing rules:

- stdout is protocol-only
- stderr is diagnostics-only and never parsed as protocol
- every frame must fit within the negotiated and Codex-enforced byte limit before JSON parsing
- every request-scoped frame must include `request_id`
- each request must emit exactly one terminal event: `completed`, `canceled`, or `provider_error`
- frames after a terminal event for the same request are ignored and counted as protocol violations
- malformed frames, oversized frames, invalid UTF-8, excessive nesting, or unknown unnegotiated event types fail the request
- Codex must enforce per-request event-count, byte-count, and idle-time caps before events enter model context

### Model List

Codex may request a model list and capability summary:

```json
{
  "type": "model_list",
  "request_id": "req-1"
}
```

The adapter returns bounded metadata only:

```json
{
  "type": "model_list_result",
  "request_id": "req-1",
  "models": [
    {
      "id": "provider-model",
      "display_name": "Provider Model",
      "capabilities": {
        "streaming": true,
        "tool_calls": true,
        "vision": false
      }
    }
  ]
}
```

Model metadata must not include secrets, cookies, account tokens, or unbounded provider descriptions.

### Execute Stream

Codex sends one normalized execution request:

```json
{
  "type": "execute_stream",
  "request_id": "turn-1",
  "model": "provider-model",
  "conversation": {
    "system": "bounded system instructions",
    "messages": [
      {
        "role": "user",
        "content": [
          {
            "type": "text",
            "text": "hello"
          }
        ]
      }
    ]
  },
  "tools": [
    {
      "name": "shell",
      "description": "bounded description",
      "input_schema": {}
    }
  ],
  "parameters": {
    "temperature": 0.2,
    "max_output_tokens": 1024
  },
  "credential_ref": "opaque-codex-managed-reference"
}
```

The adapter responds with a bounded event stream:

```json
{"type":"stream_started","request_id":"turn-1"}
{"type":"text_delta","request_id":"turn-1","delta":"hello"}
{"type":"tool_call_delta","request_id":"turn-1","call_id":"call-1","name":"shell","arguments_delta":"{}"}
{"type":"tool_call_done","request_id":"turn-1","call_id":"call-1"}
{"type":"usage","request_id":"turn-1","input_tokens":12,"output_tokens":4}
{"type":"completed","request_id":"turn-1","finish_reason":"stop"}
```

Required normalized event categories:

- stream lifecycle: started, completed, canceled
- assistant output: text delta, reasoning delta when supported
- tool calls: tool-call delta, tool-call completed
- usage accounting when available
- provider error with safe code, message, retryability, and redacted details

### Cancel

Codex may cancel an in-flight request:

```json
{
  "type": "cancel",
  "request_id": "turn-1"
}
```

The adapter must stop upstream work when possible and emit either `canceled` or a terminal provider error. If the adapter does not confirm cancellation within the configured timeout, Codex may terminate the process.

If cancellation happens before the first assistant-visible event, Codex fails the request without mutating conversation context. If cancellation happens mid-stream, Codex records the turn as interrupted or partial, never as completed.

### Shutdown

Codex sends shutdown before normal process exit:

```json
{
  "type": "shutdown",
  "reason": "session_closed"
}
```

The adapter should flush bounded diagnostics, close upstream connections, and exit cleanly. Codex may kill the process after the shutdown grace period.

## Security And Trust Model

External adapters execute code outside Codex's trust boundary. Running an adapter must require explicit opt-in configuration.

Required security rules:

- adapter commands must be configured explicitly by canonical path or allowlisted command identity
- adapter launch must not go through a shell unless a later security review explicitly approves that exact behavior
- adapter arguments must be static configuration values and must not contain secrets
- adapter working directory and inherited environment must be minimized and controlled by Codex
- Codex must not discover or execute arbitrary adapters from the working directory
- Codex must not auto-install, auto-update, or fetch adapter binaries as part of provider startup
- secrets must not be passed through command-line arguments
- secrets must not be passed through environment variables in the first implementation
- secrets must use an explicit credential channel or opaque credential reference after handshake succeeds
- adapters may request only credential classes declared by configuration and negotiated during handshake
- adapter stdout is protocol-only and must be size-bounded
- adapter stderr is diagnostic-only, size-capped per startup and per request, and redacted before display or logs
- adapter protocol messages must have maximum byte sizes and maximum nesting depth
- adapter output must never be injected into model context without normalization and caps
- Codex must redact token-like values, cookies, Authorization headers, API keys, and credential references
- adapters must not read browser cookies, perform MITM, install certificates, modify proxy settings, intercept browser sessions, or depend on web-session tokens for core support
- adapters that require cookie scraping, browser-profile reads, local TLS interception, or web-session replay are non-conformant for this protocol
- sandboxing policy must be explicit per adapter, including filesystem and network access expectations
- adapter provenance must be visible in diagnostics: command, version, provider id, and protocol version, without secrets

The first implementation should prefer passing credential references over raw credential values where possible. If raw tokens are unavoidable for a provider, they must be delivered over stdin protocol messages or a dedicated protected channel only after handshake, never through process arguments or environment variables unless a later security review approves the exact behavior.

## Cancellation, Timeout, Crash, And Restart Semantics

Cancellation:

- Codex sends `cancel` for cooperative cancellation.
- Adapters must make best effort to cancel upstream provider requests.
- Codex may terminate the adapter process when cancellation exceeds the configured grace period.
- Terminal cancellation events must be idempotent.

Timeouts:

- handshake timeout fails adapter startup
- model-list timeout fails provider discovery
- first-event timeout fails the request before context mutation
- idle stream timeout cancels the request
- shutdown timeout allows process termination

Crash handling:

- adapter exit during handshake is a startup failure
- adapter exit during model list is a discovery failure
- adapter exit during execute stream becomes a provider error with redacted bounded stderr
- partial stream output after a crash must not be replayed as a completed answer
- crash diagnostics must include exit status, adapter identity, and bounded stderr only

Restart:

- Codex may restart an adapter after startup failure only when policy allows retry
- in-flight requests are not automatically replayed unless the caller explicitly retries
- Codex must not automatically replay a request after any assistant-visible stream event has been emitted
- repeated crashes should trigger circuit-breaker behavior for the session
- restart limits must be bounded per adapter and per session

## Version Compatibility

The protocol must be versioned from the first implementation.

Compatibility rules:

- handshake must negotiate one exact protocol version
- additive optional capabilities are allowed after negotiation
- required capability changes require a new protocol version
- unknown event types are rejected unless explicitly negotiated as extensions
- adapters must declare their adapter version and provider id
- adapters must declare supported credential classes and provider account scope before credential handoff
- Codex diagnostics must report protocol mismatch without dumping raw protocol frames containing secrets

Deprecation policy:

- Codex may support multiple protocol versions during migration
- adapters using deprecated versions should receive structured warnings
- unsupported versions must fail during handshake, before any credential handoff

## Conformance Tests And Transcript Fixtures

The adapter protocol must include conformance tests before implementation is considered production-ready.

Required transcript fixtures:

- successful handshake and model list
- unsupported protocol version
- missing required capability
- successful text stream
- successful tool-call stream
- provider error with redacted details
- cancellation before first output
- cancellation mid-stream
- adapter crash during handshake
- adapter crash mid-stream
- stderr cap and redaction behavior
- free-form stdout rejection
- oversized protocol frame rejection
- credential handoff rejected before successful handshake
- mismatched provider id rejects credential handoff
- terminal event followed by additional frames
- repeated adapter crash triggers circuit breaker

Transcript fixtures must be safe to commit. They must not contain real tokens, cookies, account identifiers, or provider-private payloads.

## Implementation Stages

### Stage 1: Protocol ADR Acceptance

Finalize protocol scope, trust model, and acceptance criteria. No code changes are required in this stage.

### Stage 2: Internal Protocol Types And Fixture Runner

Add internal protocol message types, frame parser, size limits, and a transcript fixture runner. Do not expose user-facing config yet.

### Stage 3: Adapter Process Supervisor

Implement process launch, handshake, bounded stdout/stderr capture, cancellation, timeout, shutdown, and crash normalization.

### Stage 4: Runtime Integration Behind Private Gate

Route a private test provider through the adapter runtime. Keep existing native engine config unchanged.

### Stage 5: Conformance Test Kit

Provide fixture-based tests that adapter authors can run to validate protocol behavior.

### Stage 6: User-Facing Config ADR

Define any schema-backed adapter configuration in a separate ADR. This must cover command allowlists, credential references, sandbox policy, migration behavior, app-server/API compatibility, installation/update policy, and user consent language.

## Acceptance Criteria

- The ADR is accepted before any adapter runtime implementation starts.
- Stdio JSON framing, stream lifecycle, cancellation, and shutdown semantics are specified before code is written.
- Codex can reject unsupported protocol versions during handshake before credential handoff.
- Codex can reject mismatched provider identity and unapproved credential classes before credential handoff.
- Codex can bound request size, event size, stderr size, and stream lifetime.
- Secrets are passed through explicit credential channels or opaque references, not diagnostic text.
- Adapter stderr is capped and redacted before surfacing to users or logs.
- Adapter stdout is protocol-only; free-form stdout is rejected rather than logged or injected.
- Adapter crashes produce structured provider errors without marking partial streams complete.
- Cancellation does not leave orphaned in-flight adapter processes.
- Transcript fixtures cover success, tool call, provider error, cancellation, crash, redaction, and oversized-frame cases.
- Public native engine config remains unchanged until a later schema ADR approves adapter configuration.
- MITM, browser-cookie, and web-session adapters remain rejected for core support; any future exception requires a separate security ADR and must not be smuggled into this protocol.

## Non-Goals

- implementing the adapter runtime in this ADR
- adding public provider adapter config
- adding public schema-backed adapter configuration before a later schema ADR
- changing native Claude, Gemini, or Copilot engine config
- replacing built-in native engines
- replacing MCP
- adding a general credential broker
- supporting MITM, browser-cookie, or web-session provider adapters in core
- promising compatibility with arbitrary OpenAI-compatible proxies as the main architecture

## Consequences

Positive:

- future heterogeneous providers can be supported outside Codex core once the protocol exists
- provider-specific auth and wire translation stay isolated
- conformance fixtures make adapter behavior reviewable
- stdio keeps the first security surface smaller than a localhost daemon

Negative:

- a robust protocol and supervisor are required before adapters are safe
- adapter authors must implement normalized event semantics, not just proxy HTTP
- user-facing configuration remains blocked on a later schema ADR
- security review is mandatory before passing real credentials to adapters
