# ADR: CliRelay Inspired Tool Extensions Review

## Status

Challenged - CliRelay Interop Stage 0 Only

## Date

2026-06-07

## Context

CliRelay is a Go-based proxy and management control plane for AI CLI subscriptions and compatible APIs. It combines multi-provider routing, OAuth/API-key channel management, per-key quotas, request logging, SQLite analytics, model registry sync, WebSocket relay, config watchers, TUI/panel operations, and pluggable persistence.

This ADR originally stored 400 CliRelay-inspired extension candidates. After GitNexus review, the broad catalog is not accepted as an implementation backlog. Most candidates duplicate current Ontocode owners or belong in prior ADRs. The only approved local direction is an inert CliRelay interop detector/report that helps Ontocode understand whether a workspace already uses CliRelay artifacts, without importing credentials, running a proxy, mutating provider selection, creating API keys, storing request logs, fetching remote panel assets, changing app-server APIs, or changing runtime behavior.

Reviewed upstream at commit `fff912271ac2ce47ce9ca44c951bc34506e0042a`.

## CliRelay Source Evidence

- Repository and README: <https://github.com/kittors/CliRelay/tree/fff912271ac2ce47ce9ca44c951bc34506e0042a>
- Feature overview, providers, persistence, and architecture: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/README.md>
- HTTP server, API routing, auth middleware, static panel hosting, and WebSocket attach points: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/api/server.go>
- Config schema, provider keys, payload rules, identity fingerprints, TLS, and migrations: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/config/config.go>
- Routing groups and path route normalization: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/config/routing_groups.go>
- Path routing middleware and channel group authorization: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/api/path_routing.go>
- OAuth/auth manager abstraction: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/sdk/auth/manager.go>
- Access manager abstraction: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/sdk/access/manager.go>
- Usage log, request content retention, dashboard series, quota snapshots, and latency queries: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/usage/usage_db.go>
- Model registry, quota suspension, provider lookup, and model availability: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/registry/model_registry.go>
- Runtime service model building, excluded models, aliases, and startup registration: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/sdk/cliproxy/service.go>
- Translator registry for request/response/stream/token-count transforms: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/sdk/translator/registry.go>
- WebSocket relay manager: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/wsrelay/manager.go>
- Config/auth watcher: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/watcher/watcher.go>
- File request logger and streaming logger: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/logging/request_logger.go>
- Management asset updater: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/managementasset/updater.go>
- Git, object-store, and PostgreSQL auth/config stores: <https://github.com/kittors/CliRelay/tree/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/store>
- Vision session registry: <https://github.com/kittors/CliRelay/blob/fff912271ac2ce47ce9ca44c951bc34506e0042a/internal/vision/registry.go>

## GitNexus Challenge Evidence

GitNexus found existing Ontocode owners for the major CliRelay surfaces:

- Provider descriptors, provider capabilities, and model-provider runtime ownership already exist: [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:33), [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:108)
- External-agent detection/import already has owners: [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:260), [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/external_agent_config_processor.rs:391)
- OAuth token persistence and credential-store mode resolution already have owners: [oauth.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:195), [manager.rs](/opt/demodb/_workfolder/ontocode/codex-rs/login/src/auth/manager.rs:732), [mod.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/config/mod.rs:254)
- API/session telemetry and WebSocket transport behavior already have owners: [session_telemetry.rs](/opt/demodb/_workfolder/ontocode/codex-rs/otel/src/events/session_telemetry.rs:510), [client.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/client.rs:1515)
- App-server API compatibility already has a v2 protocol owner and tests: [v2.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server-protocol/src/protocol/v2.rs:1), [analytics.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/tests/suite/v2/analytics.rs:33)
- Doctor, feedback, and redaction diagnostics already have owners: [output.rs](/opt/demodb/_workfolder/ontocode/codex-rs/cli/src/doctor/output.rs:1298), [feedback_doctor_report.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/feedback_doctor_report.rs:36), [lib.rs](/opt/demodb/_workfolder/ontocode/codex-rs/feedback/src/lib.rs:726)
- Network proxy and MITM policy are separate existing owners, not provider-runtime code: [network-proxy](/opt/demodb/_workfolder/ontocode/codex-rs/network-proxy/README.md:1), [mitm_tests.rs](/opt/demodb/_workfolder/ontocode/codex-rs/network-proxy/src/mitm_tests.rs:53)
- Model-visible tool planning already has an owner and must not be bypassed: [spec_plan.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/tools/spec_plan.rs:160)

## Decision

Accept only one Stage 0 task from this ADR:

| Label | Decision | Owner | Implementation boundary |
|---|---|---|---|
| `onto_clirelay_interop_detector` | Keep | external-agent config migration / diagnostics | Detect CliRelay workspace artifacts and produce a redacted dry-run report. No proxy execution, no provider mutation, no API-key CRUD, no credential import, no request-log import, no SQLite/Redis/Git/S3/Postgres access, no remote panel fetch, no app-server API exposure, no model-context injection, and no shell execution. |
| `onto_clirelay_interop_detector_tests` | Keep | external-agent config migration tests | Add fixture tests proving redaction, bounded output, no executable import, no credential leakage, no request/response body capture, no remote fetch, and stable classification of detected artifacts. |

Everything else from the original 400 candidates is removed from this ADR and handled by one of three outcomes:

- Delegated to an existing core owner and prior ADR.
- Blocked until a separate ADR proves runtime need and compatibility.
- Moved to [ADR_CLIRELAY_TOOL_EXTENSIONS_LEFTIES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_CLIRELAY_TOOL_EXTENSIONS_LEFTIES.md:1) because it is not a natural core extension.

## Approved Stage 0 Detector Scope

The detector may inspect only local file names, manifest/config metadata, config keys, env var names, counts, enabled/disabled flags, and redacted endpoint identities.

Allowed findings:

- CliRelay installation/config presence: `config.yaml`, `config.example.yaml`, `auths/`, `docker-compose.yml`, CliRelay binary names, or package/build metadata.
- Provider metadata: provider names, channel counts, enabled/disabled state, auth-kind names, model alias names, excluded-model names, and compatible-provider base URL host only.
- Routing metadata: group names, strategy names, route namespace names, member counts, and priority keys only.
- Auth metadata: provider auth kinds, OAuth flow names, expiry field presence, refresh capability presence, auth file counts, and redacted identity labels. No token, cookie, account secret, or credential value import.
- API-key metadata: key record counts, masked display shape, quota/rate-limit field names, group/model scope counts, and enabled/disabled flags. No key value import.
- Usage/log metadata: log table/file presence, aggregate field names, retention settings, and content-storage enabled/disabled state. No request or response body import.
- Model metadata: model IDs, aliases, provider owners, type/capability names, quota-exceeded markers, and pricing field presence only.
- Protocol metadata: declared request/response/stream formats and transform names only. No request transformation at runtime.
- Streaming metadata: SSE/WebSocket feature presence and metric field names only. No relay, bridge, reconnect, or stream capture.
- Multimodal metadata: image-capable model flags and size/config field names only. No image content, base64 data, or image cache.
- Persistence metadata: local, SQLite, Redis, Git, object-store, or PostgreSQL store-kind presence only. No connection or remote access.
- Management/deployment metadata: panel/update/TUI/compose feature presence only. No panel hosting, updater, installer, or sidecar behavior.
- Diagnostic matrix: compatible, delegated, blocked, lefties, and unknown counts.

Rejected Stage 0 behavior:

- No proxy process, API server, gateway, load balancer, routing engine, failover engine, quota engine, rate limiter, WebSocket relay, SSE relay, request translator, model registry, or management panel.
- No OAuth/browser/device/cookie login flow execution.
- No API-key CRUD, self-service lookup, public endpoint, app-server method, or schema change.
- No credential persistence, credential import, auth-store mutation, remote store sync, Git/S3/Postgres/Redis access, or request/response body storage.
- No dynamic model sync, OpenRouter fetch, update check, remote panel fetch, install script, Docker/Compose orchestration, or release sidecar.
- No model-visible tool exposure or model-context injection.

## Original Point Disposition

Each original point remains traceable through the following ranges:

| Original points | Disposition | Similar current solution / prior ADR | Challenge outcome |
|---|---|---|---|
| `001-020` provider gateway and descriptors | Delegated | [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:1), [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:33) | Keep only provider metadata detection here; provider runtime work belongs to native provider descriptors or external adapter ADRs. |
| `021-040` channel routing and failover | Blocked/delegated | [ADR_MODEL_PROVIDER_SELECTOR_REFACTOR.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_MODEL_PROVIDER_SELECTOR_REFACTOR.md:1), [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:108) | Runtime routing, load balancing, and failover are not accepted by this ADR. |
| `041-060` OAuth and auth inventory | Delegated | [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1), [manager.rs](/opt/demodb/_workfolder/ontocode/codex-rs/login/src/auth/manager.rs:732) | Keep auth-kind metadata only; credential import remains blocked pending evidence and auth owner acceptance. |
| `061-080` credential stores and import safety | Delegated/lefties | [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1), [mod.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/config/mod.rs:254) | Remote store ideas are not natural core; only store-kind detection is allowed. |
| `081-100` API keys, quotas, and rate limits | Blocked/lefties | [v2.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server-protocol/src/protocol/v2.rs:1) | API-key CRUD, quota enforcement, rate limits, and public lookup require a separate app-server/product ADR. |
| `101-120` usage logs and telemetry | Delegated/blocked | [session_telemetry.rs](/opt/demodb/_workfolder/ontocode/codex-rs/otel/src/events/session_telemetry.rs:510), [output.rs](/opt/demodb/_workfolder/ontocode/codex-rs/cli/src/doctor/output.rs:1298) | Metadata-only diagnostics are allowed; request/response body storage and SQLite log import are blocked. |
| `121-140` health and monitoring | Delegated/lefties | [feedback_doctor_report.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/feedback_doctor_report.rs:36), [card.rs](/opt/demodb/_workfolder/ontocode/codex-rs/tui/src/status/card.rs:161) | Static diagnostics may extend doctor/status owners; live dashboards and WebSocket monitors move to lefties. |
| `141-160` model registry and pricing | Delegated | [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:1), [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:33) | Model alias/exclusion metadata may inform provider ADRs; no second model registry or remote sync. |
| `161-180` protocol translation | Delegated/blocked | [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md:1), [client.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/client.rs:1673) | Runtime translators require native engine or adapter ADR acceptance; this ADR only records format metadata. |
| `181-200` SSE and WebSocket relay | Delegated/lefties | [client.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/client.rs:1515), [websocket_fallback.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/tests/suite/websocket_fallback.rs:27) | Stream metrics can inform telemetry; WebSocket relay/bridge behavior moves to lefties unless transport ADR accepts it. |
| `201-220` multimodal and image | Delegated/blocked | [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:1) | Capability metadata only; image cache, mutation, generation UI, and image content diagnostics are blocked. |
| `221-240` config loader, snapshots, and migration | Delegated | [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:260), [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1) | Redacted config detection fits external-agent migration; public config/schema changes require separate ADR. |
| `241-260` watcher and hot reload | Blocked/lefties | Current config/auth owners plus [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:260) | Watcher daemon and runtime hot reload are not accepted; only change-summary fixture ideas remain report-only. |
| `261-280` proxy, TLS, and request cloaking | Delegated/lefties | [network-proxy](/opt/demodb/_workfolder/ontocode/codex-rs/network-proxy/README.md:1), [mitm_tests.rs](/opt/demodb/_workfolder/ontocode/codex-rs/network-proxy/src/mitm_tests.rs:53) | Existing network-proxy owns proxy/TLS policy; this ADR may only detect redacted proxy/TLS config metadata. |
| `281-300` management API, TUI, and operator UX | Blocked/lefties | [v2.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server-protocol/src/protocol/v2.rs:1), TUI status owners | Management panel, API key tabs, visual config, and update UI are product/app-server scope and move to lefties. |
| `301-320` external-agent/config import | Keep/delegated | [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:260), [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/external_agent_config_processor.rs:391) | This is the natural home for the Stage 0 detector; full config/auth import remains blocked. |
| `321-340` persistence and backup | Blocked/lefties | Auth-store mode resolution [mod.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/config/mod.rs:254) | Remote persistence, Redis, SQLite usage DB, and backup/restore behavior are not accepted. |
| `341-360` diagnostics and redaction | Delegated | [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1), [output.rs](/opt/demodb/_workfolder/ontocode/codex-rs/cli/src/doctor/output.rs:1298) | Redaction templates and source-link checkers belong to diagnostics/project tooling; no new diagnostics framework. |
| `361-380` deployment, update, and packaging | Lefties | Release/packaging scope only | Docker/Compose, install scripts, updater sidecar, online update, panel assets, and pprof/debug servers are not core. |
| `381-400` SDK extensibility and tests | Delegated/lefties | [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1), [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md:1) | Test fixtures and challenge checklists may survive as project tooling; executor/translator/runtime abstractions require owning ADRs. |

## Cross-ADR Delegations

- Provider descriptors, model aliases, capability flags, model exclusions, multimodal capability metadata, provider status, and provider diagnostics are delegated to [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:1).
- Credential evidence, auth-kind classification, redacted dry-run imports, store-kind detection, deletion/overwrite/provenance semantics, and config migration are delegated to [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1).
- Runtime request/response translation, future custom providers, adapter conformance, bounded streams, cancellation, and external process boundaries are delegated to [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md:1).
- Source-link validators, duplicate detectors, redaction templates, task-card generation, and challenge checklists are delegated to [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1).
- Management panel, API key CRUD, public lookup, quota/rate enforcement, SQLite request body storage, online update, deployment sidecars, and remote persistence are moved to [ADR_CLIRELAY_TOOL_EXTENSIONS_LEFTIES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_CLIRELAY_TOOL_EXTENSIONS_LEFTIES.md:1).

## Implementation Task Card

Task: implement `onto_clirelay_interop_detector`.

Minimum implementation requirements:

- Add a detector under the existing external-agent config migration path rather than a new import service.
- Output a bounded report with deterministic ordering.
- Redact tokens, cookies, API keys, account IDs, auth indexes, proxy credentials, database URLs, object-store keys, raw request bodies, raw response bodies, and secret-bearing paths.
- Classify every discovered field as `compatible`, `delegated`, `blocked`, `lefties`, or `unknown`.
- Add tests using sanitized CliRelay fixtures for config, auth directory, provider channels, routing groups, API-key metadata, usage-log metadata, proxy/TLS metadata, and persistence-kind metadata.
- Do not add public config keys, app-server APIs, model-visible tools, provider runtime changes, or credential persistence in this slice.
