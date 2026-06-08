# ADR: Public Adapter SDK And Schema Migrations

## Status

Proposed

## Date

2026-06-08

## Context

`ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md` approved a private stdio JSON/NDJSON provider-adapter protocol and explicitly deferred public configuration, schema generation, app-server exposure, SDK packaging, and migration behavior to a later compatibility ADR.

The current codebase now has:

- `codex-rs/adapter-protocol` with protocol structs, transcript fixtures, and a conformance runner.
- Native provider engines for first-party heterogeneous providers.
- Config and app-server schema generation paths that must remain stable for public surfaces.
- SDKs in Python and TypeScript that expose generated or mirrored app-server/runtime contracts.

Public adapter support would turn an internal protocol into a user-visible extension surface. That requires a compatibility contract before implementation.

## Problem

Without a schema and migration ADR, adapter support risks:

- adding public config keys before stability, redaction, and migration rules are defined
- exposing app-server or SDK APIs that cannot be evolved safely
- allowing workspace-controlled adapter commands without enough trust and provenance metadata
- creating duplicate provider registries or bypassing existing model-provider ownership
- shipping protocol fixtures that do not match generated SDK types

## Decision Drivers

- Keep provider ownership in `codex-rs/model-provider` and avoid duplicate provider registries.
- Keep adapter execution opt-in and never auto-discovered from untrusted workspace files.
- Expose public config only with schema generation, compatibility tests, and migration rules.
- Keep app-server APIs in v2 and use existing schema/TypeScript generation workflows.
- Keep SDK contracts generated from one source of truth where possible.
- Preserve redaction guarantees for adapter commands, stderr, credentials, request payloads, and stream events.
- Bound all adapter-sourced content before it enters model context.

## Decision

Do not expose public adapter configuration or SDK APIs yet.

The next implementation stage must first define and verify a public compatibility surface with:

- a versioned adapter config shape
- schema migration behavior for config files and generated app-server schemas
- SDK generation or hand-maintained SDK mapping rules
- conformance fixtures that are stable enough for third-party adapter authors
- trust/provenance fields for adapter source, command path, and allowed capabilities
- diagnostics and support-bundle redaction requirements

Any runtime implementation must plug into the existing provider/model-provider owner rather than creating a second provider factory or registry.

## Public Config Requirements

Any public config must be schema-backed and include:

- adapter id
- command path and args with explicit trust/provenance metadata
- protocol version
- provider id
- declared capabilities
- timeout and stream cap overrides within hard-coded maxima
- credential reference policy, not raw credentials
- disabled reason or validation status

Public config must not include:

- raw tokens, cookies, authorization headers, or keychain paths
- workspace-autoloaded adapter commands
- shell-expanded command strings
- unbounded stderr, request, or stream capture settings

## Schema And Migration Requirements

Before implementation:

- update `ConfigToml` and generated config schema only after the public key shape is accepted
- run `just write-config-schema` for config schema changes
- update app-server v2 protocol only if adapter management becomes an API surface
- run `just write-app-server-schema` for app-server schema changes
- add compatibility tests for reading older config without adapter keys
- add migration tests for disabled/unknown adapter config entries
- document rollback behavior for invalid adapter configs

## SDK Requirements

SDK exposure is blocked until the public schema is accepted.

When accepted:

- Python and TypeScript SDK types must reflect the generated app-server/runtime contract.
- SDK examples must not execute real third-party adapters by default.
- SDK fixtures must use inert transcript adapters or generated conformance fixtures.
- SDK docs must explain trust, opt-in, redaction, and version compatibility.

## Conformance Requirements

The existing `codex-rs/adapter-protocol` fixtures are the seed, not the final public SDK contract.

Before public release:

- fixture coverage must include handshake, model list, normal stream, tool-call stream, provider error, cancellation, shutdown, oversize frame rejection, and protocol-version mismatch
- transcript fixtures must contain no secrets
- conformance runner must enforce request/event/stderr caps
- adapter authors must be able to validate without running the full Codex workspace

## GitNexus Evidence

Initial GitNexus query for adapter SDK/schema migration found these relevant owners:

- `codex-rs/adapter-protocol`
- `codex-rs/config/src/config_toml.rs::ConfigToml`
- app-server v2 schema generation paths
- SDK artifact generation in `sdk/python/scripts/update_sdk_artifacts.py`
- external-agent config import paths that must not become provider-adapter owners

## Next Implementation Tasks

1. Draft a concrete public adapter config schema proposal.
2. Map the schema proposal to `ConfigToml`, app-server v2, Python SDK, and TypeScript SDK surfaces.
3. Define compatibility tests and generated-schema commands required for any implementation PR.
4. Extend `adapter-protocol` conformance fixtures only after the public schema shape is accepted.

## Non-Goals

- Implement public adapter runtime execution in this ADR.
- Add public config keys before schema compatibility tests exist.
- Add credential import or credential persistence for adapters.
- Use MCP as the first model-provider adapter protocol.
- Execute workspace adapter code automatically.
