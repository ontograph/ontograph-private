# ADR: Remaining Provider Extensibility Implementation

## Status

Proposed

## Date

2026-06-06

## Context

`ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR.md` remains challenged because real Claude MCP OAuth credential evidence is still missing. The current codebase has made two useful but incomplete advances:

- Claude import now has an isolated parser/report/status boundary, redaction helper, and live-sample validation runbook.
- model-provider selection now has provider-owned capabilities and a private descriptor slice for built-in provider engines.

This leaves two different remaining goals:

- ship Claude MCP OAuth import safely once a real credential sample is available
- support many providers without spreading provider-specific branches through CLI, TUI, app-server, core, and auth code

These goals overlap, but they should not be forced into one large refactor.

## Problem

The current system is not yet ready for broad heterogeneous provider support without code changes.

The main blockers are:

- Claude credential shape, legality, and refresh semantics are unproven without a real redacted sample.
- imported Claude credentials are not wired into MCP OAuth persistence at runtime.
- `ProviderEngine` is private and supports only built-in `OpenAiResponses` and `AmazonBedrockResponses` engines.
- new wire protocols still require Rust implementation.
- there is no stable registry or plugin contract for external credential importers or runtime provider adapters.

## Decision Drivers

- avoid adding a broad credential broker before evidence proves it is needed
- preserve the existing MCP OAuth store as the first Claude import target
- keep provider-specific runtime behavior localized in `codex-rs/model-provider`
- support many OpenAI-compatible providers through config and descriptors
- provide a path for truly heterogeneous providers without making core auth code provider-specific
- keep each implementation slice independently testable and reviewable

## Crush Review Addendum

`ADR_CRUSH_TOOL_EXTENSIONS.md` originally proposed auth/import points `301-310`, external-state import point `340`, and test point `399`. Those points are delegated here only for future credential-import architecture. The Crush ADR itself may only run a redacted dry-run detector.

Delegated original points:

- `301-310`: Copilot/Hyper OAuth import, refresh, logout, token redaction, disk-store audit, provider refresh hooks, provider header setup, dry-run auth import, and redacted auth import reports.
- `340`: compatible external-agent state import.
- `399`: tests for Crush external-agent import.

Architecture decision:

- Treat Crush as another external-agent import source only after the Stage 0 dry-run report proves useful.
- Target existing credential stores and provider/MCP auth boundaries explicitly; do not create a full credential broker.
- Auth import remains blocked unless redacted real-world evidence proves the credential shape and legal/security constraints.
- Import behavior must define overwrite, provenance, deletion, validation, and redaction semantics before any persistence is enabled.

Related source links:

- [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:166)
- [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/external_agent_config_processor.rs:78)
- [oauth.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:58)

## OpenClaw Review Addendum

`ADR_OPENCLAW_TOOL_EXTENSIONS.md` originally proposed external-agent/config, auth, plugin manifest, MCP, and dry-run diagnostic points `021-080`, `161-200`, `241-260`, and `341-360`. Those points are delegated here only for future external-agent interop detection, redacted credential evidence, plugin/MCP metadata import planning, and migration diagnostics. The OpenClaw ADR itself may only run a redacted Stage 0 detector.

Delegated original points:

- `021-040`: OpenClaw config detection, schema/version hints, channel/model/agent/node/skill/plugin root metadata, redacted credential paths, unknown key reporting, import plan preview, and capped summary fragments.
- `061-080`: auth profile inventory, OAuth/API-key profile metadata, expiry/refresh field presence, provider scope metadata, fixture generation, sensitive-output guards, and no-write auth migration previews.
- `161-200`: skill/plugin/MCP manifest metadata, compatible bundle declarations, setup metadata, config contracts, MCP server metadata, tool-name collisions, transport classification, and no-runtime-load guards.
- `241-260` and `341-360`: redacted doctor sections, diagnostic fixtures, support-bundle leak tests, conformance tests, no-execution tests, and context-bound tests.

Architecture decision:

- Treat OpenClaw as another external-agent import source only after the Stage 0 dry-run report proves useful.
- Credential import remains blocked until credential shape, legal constraints, overwrite behavior, provenance, validation, deletion, and redaction semantics are defined.
- Plugin and MCP manifests must remain quarantined metadata unless the core plugin/MCP owners and adapter ADR separately approve runtime behavior.

Related source links:

- [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:260)
- [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/external_agent_config_processor.rs:391)
- [lib.rs](/opt/demodb/_workfolder/ontocode/codex-rs/external-agent-migration/src/lib.rs:1)
- [oauth.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:58)
- [loader.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core-plugins/src/loader.rs:555)

## Hermes Agent Review Addendum

`ADR_HERMES_AGENT_TOOL_EXTENSIONS.md` originally proposed auth/import points `021-040`, provider/plugin import points `001`, `008`, `032`, `041`, `049`, `060`, `321-340`, and external-agent import points `361-380`. Those points are delegated here only for future credential-import and external-agent import architecture. The Hermes ADR itself may only run a redacted Stage 0 detector.

Delegated original points:

- `021-040`: OAuth external token detection, device-code review, Copilot refresh-cycle audit, AWS credential-chain classification, credential precedence/conflict reports, expiry metadata, token host binding, permission checks, refresh preflights, retry comparison, and external-process auth sandbox rules.
- `001`, `008`, `032`, `041`, `049`, `060`, `321-340`: provider/plugin manifest detection, env requirement metadata, quarantine, trust states, plugin provenance, dependency scanning, disabled-reason reporting, and dry-run plugin import.
- `361-380`: Hermes config, SOUL/MEMORY, skills, command allowlists, messaging settings, API key env names, workspace instructions, profiles, plugins, MCP, providers, sessions, cron, gateway, trajectories, installed binary, dry-run command, and redacted import report tests.

Architecture decision:

- Hermes is another external-agent import source; start with inert detection and reports only.
- Credential import remains blocked until the credential shape, legal constraints, overwrite behavior, provenance, validation, deletion, and redaction semantics are defined.
- Plugin and provider manifests must be quarantined metadata unless the external adapter ADR separately approves runtime execution.

Related source links:

- [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:260)
- [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/external_agent_config_processor.rs:391)
- [lib.rs](/opt/demodb/_workfolder/ontocode/codex-rs/external-agent-migration/src/lib.rs:227)
- [oauth.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:58)

## GBrain Review Addendum

`ADR_GBRAIN_TOOL_EXTENSIONS.md` originally proposed MCP/auth/import points `111-130`, provider auth points `251-260`, and remote-client credential points `351-360`. Those points are delegated here only when they concern credential evidence, external-agent detection, OAuth/env classification, redacted import reports, or future import persistence decisions.

Delegated original points:

- `111-130`: GBrain MCP config detection, MCP auth readiness, operation inventory, redacted MCP diagnostics, and Codex MCP setup reports.
- `251-260`: provider/account/auth classification only when it affects provider auth evidence or credential import readiness.
- `351-360`: remote brain endpoint and credential bridge detection only as inert, redacted external-agent metadata.

Architecture decision:

- The GBrain ADR may detect GBrain config and MCP/auth metadata, but it must not persist credentials, call remote GBrain services, or create a credential broker.
- Any future credential import must reuse existing MCP OAuth/provider auth boundaries, define overwrite/provenance/deletion behavior, and require redacted real-world evidence before persistence.
- GBrain remote/server configuration remains report-only unless a later ADR accepts a provider adapter or external service boundary.

## OpenCode Review Addendum

`ADR_OPENCODE_TOOL_EXTENSIONS.md` originally proposed MCP/provider/auth import points `137`, `154`, `159`, and `161-180`. Those points are delegated here only when they concern external-agent detection, credential evidence, OAuth/env classification, auth plugin quarantine, redacted import reports, or future import persistence decisions.

Delegated original points:

- `137`, `159`: OpenCode MCP/provider config detection and dry-run mapping only as redacted external-agent metadata.
- `154`: provider auth plugin detection only as quarantined metadata; plugin execution remains blocked.
- `161-180`: account status, browser callback failure, auth plugin import detection, env/config precedence, no-secret support bundles, refresh readiness, logout audit, Copilot/Codex/OpenAI plugin evidence, cloud provider classification, missing credential diagnostics, stale token warnings, redacted import reports, legal-risk markers, account UX audit, callback review, credential source matrix, secret-path redaction, config schema lint, and blocked-fixture tests.

Architecture decision:

- OpenCode auth/plugin/provider config may be detected, but must not persist credentials, execute auth plugins, fetch remote config, or create a credential broker.
- Any future credential import must reuse existing MCP OAuth/provider auth boundaries and define overwrite, provenance, validation, deletion, and redaction semantics before persistence.
- Executable OpenCode auth flows remain blocked until legal/security review and real-world redacted evidence prove the credential shape.

## CliRelay Review Addendum

`ADR_CLIRELAY_TOOL_EXTENSIONS.md` originally proposed auth/import/config points `041-080`, `221-240`, `301-320`, and store-kind/persistence points `321-340`. Those points are delegated here only when they concern external-agent detection, credential evidence, OAuth/API-key/env classification, redacted dry-run reports, store-kind metadata, or future import persistence decisions.

Delegated original points:

- `041-060`: provider auth inventory, auth metadata redaction, auth file shape classification, OAuth/device/browser/cookie flow evidence, refresh/expiry metadata, identity metadata, proxy references, auth-index stability, dry-run import, store-mode mapping, redaction fixtures, login-surface review, and deletion semantics.
- `061-080`: credential store kind inventory, secret/path redaction, remote store policy notes, sync blockers, rotation/provenance/rollback/delete/overwrite/audit requirements, sample contracts, fixture validation, live runbooks, keyring mapping, and doctor probes.
- `221-240`: config file/schema/env detector, redacted snapshots and diffs, legacy key detection, provider/routing/payload/TLS/cloak/identity metadata, and schema ADR stubs.
- `301-320`: CliRelay external-agent detection, config classification, dry-run reports, cc-switch/OpenCode-Go/Amp evidence, auth directory inventory, status matrix, source links, duplicate detection, task cards, tracking seeds, fixture packs, report schema, owner mapping, and lefties routing.
- `321-340`: persistence backend inventory and metadata-only policy notes. Runtime storage, remote fetch, backup/restore, and request body storage remain blocked.

Architecture decision:

- CliRelay is another external-agent import source; start with inert detection and reports only.
- Credential and config import remains blocked until credential shape, legal constraints, overwrite behavior, provenance, validation, deletion, and redaction semantics are defined.
- Remote stores, API-key CRUD, quota state, request logs, and full config import must not be persisted through this ADR.

## Options Considered

### Option 1: Minimal Claude MCP OAuth Import

Implement only the blocked Claude MCP import path after a real redacted credential sample is available.

Scope:

- validate the real redacted sample against the existing parser
- map `ImportableMcpOAuthCredential` into `StoredOAuthTokens`
- persist with the existing MCP OAuth store
- report `Complete`, `Partial`, `NonImportable`, and `Empty` outcomes
- add app-server or migration command integration
- add tests for save/load/status behavior

Pros:

- fastest safe path to user-visible Claude import
- avoids duplicate credential storage
- follows the challenged ADR counterproposal

Cons:

- does not solve general provider extensibility
- remains blocked until a real sample exists
- must define overwrite, identity, and security behavior before persistence is enabled

### Option 2: Descriptor-Driven Built-In Provider Config

Continue the current private provider descriptor work for built-in engines, without promoting it to public config yet.

Scope:

- keep `ProviderEngine` and `ProviderDescriptor` private for now
- describe engine, auth scheme, model-catalog behavior, probe behavior, and capabilities internally
- keep engine implementations built in
- support many OpenAI-compatible providers without new Rust branches
- add config/schema fields only after a real provider requires user-selectable engine configuration

Pros:

- improves provider growth with low runtime risk
- keeps provider behavior centralized in `codex-rs/model-provider`
- avoids plugin security complexity for the first extensibility step
- avoids premature public schema commitments

Cons:

- cannot support a brand-new protocol without Rust engine support
- schema changes require compatibility and migration review

### Option 3: Credential Import Adapter Registry

Introduce a small registry for foreign credential importers, without introducing a full credential broker.

Scope:

- define importer output as normalized internal records or explicit rejection reasons
- keep Claude as the first importer
- target existing stores first, especially MCP OAuth storage
- include provenance, redaction, deletion, and validation status

Pros:

- gives future imports a clean boundary
- avoids spreading Claude-specific parsing into generic auth code
- allows Cursor, Windsurf, or other imports to reuse the same interface

Cons:

- needs careful security review
- can become a broker if storage ownership is not kept explicit

### Option 4: Full Credential Broker

Build a shared credential subsystem for model-provider auth, MCP OAuth, imported credentials, refresh, provenance, and revocation.

Pros:

- strongest long-term architecture
- handles multiple credential lifecycles and domains
- avoids duplicated persistence semantics over time

Cons:

- too large before Claude evidence exists
- risks duplicating the existing MCP OAuth store
- high migration and security-review cost

### Option 5: External Provider Adapter Runtime

Allow providers to be implemented as external adapter processes with a stable request/response contract.

This option is intentionally not approved by this ADR. It is included only as a future alternative because it is the only path that can support truly heterogeneous providers without a Codex Rust release.

Scope:

- config points to an adapter command
- Codex sends normalized model requests to the adapter
- adapter owns provider-specific auth, refresh, request translation, streaming, and model listing

Pros:

- only option that can support truly heterogeneous providers without a Codex Rust release
- keeps experimental providers outside core runtime

Cons:

- difficult streaming, cancellation, error mapping, telemetry, and sandboxing model
- security boundary must be designed before production use
- worse debuggability than built-in engines
- requires a separate ADR before any implementation work

## Decision

Use a narrowed staged path:

- Approved now: continue Option 2 only as private built-in descriptor work; add no public provider-engine config yet.
- Blocked: Option 1 runtime wiring remains blocked until the real Claude credential evidence gate is satisfied.
- Deferred: Option 3 until a second foreign credential source appears or Claude import needs reusable importer lifecycle handling.
- Deferred: Option 4 until multiple credential domains prove the existing MCP OAuth store plus explicit importer adapters are insufficient.
- Separate ADR required: Option 5 external runtime adapters.

## Implementation Plan

### Stage A: Close Claude Evidence Gate

Required input:

- real redacted Claude MCP OAuth credential sample

Required outputs:

- validated sample shape
- refreshability verdict
- account, scope, and server identity metadata
- security/legal approval for import

Exit criteria:

- ignored live-sample validator passes against the redacted real sample
- ADR tracker can move `T2` from `blocked` to `in_progress`
- product/security owner approves reading and importing the foreign credential source

Status:

- blocked on external sample and approval

### Stage B: Define Claude Persistence Acceptance Criteria

This stage is the next actionable Claude-side work. It does not persist credentials yet.

Required decisions:

- identity key: imported credentials must map to the exact MCP server name and URL used by existing MCP OAuth storage
- duplicate handling: importer must reject ambiguous duplicate `(server_name, server_url)` records
- overwrite policy: importer must not overwrite an existing stored credential unless the user explicitly confirms replacement
- refresh policy: importer must classify credentials as locally refreshable, externally refreshable, access-token-only, or non-importable
- validation policy: imported credentials must remain inactive until server identity, scopes, and refresh behavior are validated or explicitly marked recoverable
- revocation policy: imported credentials must be deletable with the same user-facing semantics as normal MCP OAuth logout
- storage policy: imported tokens must use the configured MCP OAuth store mode and keyring/file fallback semantics
- diagnostics policy: debug output, errors, logs, test snapshots, and import reports must never contain token values

Security acceptance criteria:

- user consent is required before reading foreign credential stores
- provenance is recorded at least as source application, source path or store class, import timestamp, connector name, and server URL
- scope and account metadata are shown when available
- locked keychain, missing keychain, partial import, and parse failures have explicit recoverable statuses
- no provider-specific parsing is added to `AuthManager`

Tests:

- duplicate identity rejection
- explicit overwrite required
- no-secret debug/redaction assertions
- locked or unavailable store outcome
- access-token-only credential classification
- revocation/delete behavior plan

### Stage C: Wire Claude Import To Existing MCP OAuth Store

This stage is blocked until Stage A and Stage B are complete.

Implementation:

- convert `ImportableMcpOAuthCredential` to `OAuthBearerTokenParts`
- build `StoredOAuthTokens`
- save tokens with `save_oauth_tokens`
- verify `has_oauth_tokens` and MCP server status after import

Tests:

- parser-to-token conversion
- persistence save/load/delete
- partial import report
- non-importable outcome
- no-secret debug/redaction assertions

### Stage D: Continue Private Descriptor Hardening

Implementation:

- keep `ProviderEngine` private until a real provider requires user-selectable engine configuration
- keep descriptor behavior internal to `codex-rs/model-provider`
- add no schema-backed `engine`, auth policy, model catalog policy, or capability override fields yet
- preserve existing provider config compatibility

Tests:

- OpenAI-compatible provider remains default
- Azure probe policy remains descriptor-owned
- Bedrock stays non-OpenAI-auth and no `/models` probe
- custom OpenAI-compatible provider behavior stays descriptor-owned without new provider predicates

Public descriptor-config trigger:

- a real provider requires a user-selectable engine that cannot be derived from existing `ModelProviderInfo`
- the provider can still run on a built-in engine
- compatibility, config schema, and migration tests are included in the same change

### Stage E: Add Import Adapter Registry

Trigger:

- second foreign credential source appears, or Claude import needs reusable importer lifecycle behavior

Implementation:

- define importer trait in the external-agent migration boundary or a small dedicated crate
- importer output targets existing stores explicitly
- include provenance and validation status
- do not create a general broker in this stage

### Stage F: Evaluate External Runtime Adapters In A Separate ADR

Trigger:

- a provider cannot fit `OpenAiResponses` or `AmazonBedrockResponses`
- maintainers want provider support without Codex Rust release cadence

Required design work:

- streaming protocol
- cancellation
- auth handoff
- sandboxing
- telemetry and redaction
- version negotiation
- compatibility and deprecation policy

Exit criteria:

- a separate ADR is accepted before implementation starts

## Non-Goals

- no immediate full credential broker
- no public provider-engine config until a real selectable engine needs it
- no Claude credential import without real evidence
- no provider-specific parsing in `AuthManager`
- no external adapter runtime without a separate security review

## Readiness Verdict

The current codebase is ready for many configured providers that fit existing built-in engines, especially OpenAI-compatible providers.

The current codebase is not ready for plenty heterogeneous providers without code changes. That requires either additional built-in engines, a public descriptor/config contract, or an external adapter runtime.

## Follow-Up Tracking

Track implementation in the existing files:

- Claude import tasks: `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR_TRACKING.md`
- model-provider selector tasks: `ADR_MODEL_PROVIDER_SELECTOR_REFACTOR_TRACKING.md`

Create a new tracker only if the import adapter registry or external runtime adapter work becomes active.
