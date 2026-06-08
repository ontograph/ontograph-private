---
name: Ontocode Pending Tasks
description: Living backlog summary derived from the current project tracking file
type: backlog
date: 2026-06-07
status: active
---

# Ontocode Pending Tasks

Authority: `CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md`.

## Active Tasks

### Public Adapter SDK And Schema Migrations ADR

Status: `in_progress`.

Goal:
- Define the compatibility contract before public adapter config, schema generation, app-server exposure, or SDK APIs are implemented.

Reuse anchors:
- `ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md`
- `codex-rs/adapter-protocol`
- `ConfigToml`
- app-server v2 schema generation
- Python/TypeScript SDK artifact generation

Next actions:
- Review the proposed ADR and split accepted implementation tasks into schema, app-server, SDK, and conformance tracks.

## Blocked Tasks

### Claude OAuth Live Validation

Status: `blocked`.

Reason:
- No `CLAUDE_OAUTH_REDACTED_SAMPLE` path is available in the environment.

Needed:
- One real redacted Claude MCP connector credential sample that preserves non-secret schema fields.

## Done

### External-Agent Import Internals

IDs: `213-215, 217, 218, 220`.

Completed slices:
- User consent gate for foreign credential reads.
- Import dry-run mode for safe preview.
- Provenance metadata for imported configs.
- Locked-keychain and unavailable-store diagnostic statuses.
- Redacted import reports verified with snapshots.

### Session/Context Bounded Diagnostics

IDs: `1, 87, 89, 100, 104, 109-111, 117, 119-121, 124, 130, 132, 140, 181, 185`.

Completed slices:
- Bounded `DiagnosticFragment` with 1000-token hard cap.
- Granular usage attribution (model vs tool tokens) in `history.rs`.
- Multi-agent safety inheritance logic verified via `Session::update_settings` and context creation.
- Improved hook error handling and deterministic action semantics.

### External Adapter Protocol Safety

IDs: `16-30`.

Completed slices:
- v1 Adapter Protocol crate with NDJSON framing.
- Timeout categorization (handshake, list, stream, idle, shutdown).
- Crash semantics and circuit breaker status.
- Credential gate state (handoff after handshake).
- Protocol transcript fixtures and conformance runner.

### Hook And Shell Permission Safety

IDs: `47-51, 53-56, 58-63, 161, 166-168, 172, 174, 175`.

Completed slices:
- Thread-local regex caching for hook matcher.
- HookAction semantics (Warn, Block, SystemMessage).
- Loop protection for Stop hooks (max 10 iterations).
- Robust PowerShell discovery with safety timeouts.
- Cross-platform PowerShell safety identification.

### MCP Reliability And Auth Hardening

IDs: `141-146, 149-151, 155-157`.

Completed slices:
- Internal pagination loops for tool/resource/template discovery.
- Improved config parsing diagnostics in `codex-core`.
- Image sanitization for non-vision models in `execute_mcp_tool_call`.
- Proactive OAuth refresh with 60s safety timeout in `RmcpClient`.

### OAuth/Auth-Store Validation And Redacted Diagnostics

IDs: `5, 6, 7, 11, 160`.

Completed slices:
- Centralized URL/query redaction in `codex-login`.
- OAuth token structural validation in `codex-rmcp-client`.
- Redaction-aware Claude OAuth import in `codex-external-agent-migration`.
- Scope-less retry fallback in `codex-cli`.

### Provider Provenance/Status/Capability Diagnostics

IDs: `2, 3, 4, 8, 9, 12, 13, 14`.

Completed slices:
- Provider descriptor diagnostic snapshot.
- Retry/timeout/5xx/429 telemetry coverage.
- Doctor WebSocket provider diagnostics.
- Scoped verification recorded in tracking.
