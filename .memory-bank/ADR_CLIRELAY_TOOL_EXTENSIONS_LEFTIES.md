# Lefties: CliRelay Tool Extensions

## Status

Moved out of core ADR

## Date

2026-06-07

## Context

These CliRelay-inspired ideas were removed from [ADR_CLIRELAY_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_CLIRELAY_TOOL_EXTENSIONS.md:1) because they duplicate existing Ontocode owners, need a separate ADR, or do not naturally extend the core codebase.

## Moved Items

| Original points | Reason moved |
|---|---|
| `021-040` runtime channel routing, load balancing, failover, route namespaces, and path authorization | Duplicates provider selector/runtime routing concerns and needs a separate provider/app-server ADR. |
| `061-080` Git, object-store, PostgreSQL, Redis, and remote credential/config stores | Remote secret persistence is not natural core and is blocked without security/product ADRs. |
| `081-100` API-key CRUD, quotas, rate limits, self-service lookup, and public key dashboards | This is management-plane/product scope, not core provider extensibility. |
| `106`, `113`, `119` request/response body retention, per-key dashboards, and SQLite usage DB ownership | Full traffic capture and local DB ownership are privacy/storage features requiring separate ADRs. |
| `121-140` live health dashboards, WebSocket monitoring, CPU/memory/network monitor, DB-size monitor, and health-score routing | Current doctor/status/telemetry owners may expose static summaries; live control-plane monitoring is not core. |
| `148`, `152`, `160` runtime model suspend/resume, OpenRouter model sync, and dynamic model-library behavior | Duplicates provider/model catalog ownership and remote fetch remains blocked. |
| `170`, `178`, `180` payload mutation rules, raw debug passthrough, and generic protocol gateway behavior | Provider request mutation and gateway behavior require native-engine or adapter ADRs. |
| `187-190`, `200` stream backpressure, reconnect, WebSocket bridge, and relay/server behavior | Existing transport owns stream behavior; generic relay architecture is not accepted here. |
| `206`, `208`, `218`, `220` image cache, image mutation pipelines, model-context image diagnostics, and generation UI | Media runtime/product scope and not part of CliRelay interop detection. |
| `232-235`, `238` auto-update config, remote panel config, Redis config, pprof config, and management password config | Release/product/debug-server concerns outside this ADR. |
| `241-260` daemonized config/auth watcher and runtime hot reload | Runtime mutation needs config/auth owner approval; detector may only produce static reports. |
| `264`, `277-280` outbound proxy runtime, proxy health checks, proxy rotation, and proxy/TLS control-plane management | Existing network-proxy owns proxy policy; control-plane proxy management is out of scope. |
| `281-300` management API, web panel, visual config, auth/key/log TUI tabs, online update UI, and operator dashboards | App-server/product/TUI scope requiring separate ADR and compatibility tests. |
| `309`, `320` full config import and proxy-control-plane imports | Stage 0 may dry-run only; full import requires compatibility and security ADRs. |
| `321-340` persistence backends, backup/restore, request body storage, remote fetch, and export storage | Storage/backups are not accepted without storage/security ADRs. |
| `361-380` Docker Compose, containers, install scripts, updater sidecar, online update, release assets, frontend repo sync, pprof, and deployment smoke tests | Packaging/release/deployment scope, not core runtime architecture. |
| `389`, `397` executor binding and store implementation tests as runtime work | Runtime executors and stores need owning ADRs; tests can only be reused as evidence. |

## Re-entry Criteria

Any moved item can return only with:

- a concrete user problem not solved by current provider, auth, app-server, telemetry, diagnostics, network-proxy, external-agent, GitNexus, or memory-bank owners
- a narrow ADR naming the exact owner and compatibility surface
- redaction and privacy tests for any credential, API key, proxy URL, request log, response body, database URL, model ID, provider endpoint, or diagnostic output
- no public config, app-server API, SDK, schema, dashboard, or persistence change without compatibility tests
- bounded context rules for anything model-visible
- explicit opt-in, sandbox, cancellation, and conformance rules for anything executable or remote
- GitNexus context and impact analysis before implementation
