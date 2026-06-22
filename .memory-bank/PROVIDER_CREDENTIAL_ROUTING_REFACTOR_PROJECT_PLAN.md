---
name: Provider Credential Routing Refactor Project Plan
description: Staged plan for provider-neutral credential routing and heterogeneous OAuth/account reuse across Codex, Claude, Gemini, and future providers
type: project_plan
date: 2026-06-13
status: completed_with_follow_on
---

# Provider Credential Routing Refactor Project Plan

## Purpose

Refactor the current provider/auth code so OAuth and adjacent credential types can be acquired, stored, refreshed, selected, and reused across heterogeneous providers without creating a second provider registry or a second token store.

This plan adopts the recommended execution order:

1. Option 3: Model Alias And Prefix Router First
2. Option 1: Unified Credential Registry
3. Option 4: Refresh Orchestrator Unification
4. Option 2: Credential Registry Plus Account Scheduler
5. Option 5: External Provider Adapter SDK

## Current State

The staged refactor defined in this file is complete.

- S1 landed as an internal alias/prefix/account-group router in `model-provider`.
- S2 landed as a normalized redacted credential-view layer rather than a second persistence authority.
- S3 landed as a shared refresh-orchestration contract with existing refresh owners kept in place.
- S4 landed as a private scheduler with deterministic selection, failover, and sticky-session behavior.
- S5 closed by senior review: the existing private `ModelProvider` auth seam is the correct internal adapter contract, and adding a second provider-auth trait family would duplicate the architecture.

Follow-on work still exists, but it is no longer this refactor line. The remaining gap is a provider-neutral secret-bearing OAuth credential model for heterogeneous native providers, especially Gemini OAuth and Copilot OAuth reuse.

Current follow-on authority, 2026-06-18:

- [Multi-Provider OAuth With First-Class Codex Project Plan](MULTI_PROVIDER_OAUTH_FIRST_CLASS_CODEX_PROJECT_PLAN.md)
  governs the remaining OAuth sequencing.
- This completed refactor remains the routing foundation, but it must not be
  interpreted as permission to replace Codex/OpenAI with a universal auth
  broker.
- Codex/OpenAI uses provider id `openai` and remains the first-class default and
  fallback.
- Provider OAuth credentials are additive and provider-scoped. Runtime provider
  choice stays explicit at config, thread, sub-agent, or future routing
  boundaries.

## Scope Boundaries

- Reuse existing owners:
  - `ontocode-rs/model-provider` for provider routing and selection
  - `ontocode-rs/login` for first-party auth persistence and refresh inputs
  - `ontocode-rs/rmcp-client` for MCP OAuth token storage and refresh behavior
  - `ontocode-rs/external-agent-migration` for foreign credential import
- Do not create:
  - a second provider factory
  - a second provider registry
  - a second credential persistence layer
  - provider-specific routing branches in app-server or CLI
- Treat OAuth import, OAuth acquisition, API-key import, and future session-token import as credential sources, not runtime routing owners.

## Challenge Summary

This plan is directionally correct, but it is easy to misexecute in four dangerous ways:

- S1 could drift into public config or app-server API work before the compatibility ADR gates are satisfied.
- S2 could turn into a new persistence authority instead of a normalized read model over existing stores.
- S3 could create a second refresh engine instead of converging existing refresh owners.
- S5 could expand into a public provider SDK before the internal seams are proven.

The remaining sections tighten those constraints.

## Non-Negotiable Interpretation Rules

- S1 is internal routing first; public config keys, app-server fields, and SDK types remain gated by `ADR_PUBLIC_ADAPTER_SDK_SCHEMA_MIGRATIONS.md`.
- S2 defines a normalized credential view for routing and diagnostics, not a new source-of-truth token store.
- S3 must converge refresh control and telemetry before it adds new refresh behavior.
- S4 must degrade cleanly when quota, health, or cooldown signals are missing.
- S5 starts as a private internal adapter contract; public SDK exposure remains separately gated.
- `gemini` must be treated as a heterogeneous provider that may use OAuth, API keys, or other session material depending on source; the architecture must not assume one auth mechanism for every provider.

## Target Outcome

Ontocode should be able to:

- resolve a requested model through provider-neutral alias and prefix rules
- select an eligible account/credential for `codex`, `claude`, `gemini`, and future providers
- refresh eligible credentials through one orchestration surface
- expose bounded, redacted status and diagnostics for provider/account health
- add future providers through adapter implementations instead of cross-cutting auth rewrites

## Post-Implementation Review

The `CLIProxyAPI` review confirmed that the missing piece is not more routing or more scheduling. The missing piece is one canonical internal OAuth credential object that all provider-specific login/import flows can project into without creating a new store.

What is already sufficient in Ontocode:

- redacted routing metadata in `ontocode_protocol::credential_routing`
- first-party OpenAI/Codex auth persistence and refresh in `ontocode-login`
- MCP OAuth token storage and refresh in `ontocode-rmcp-client`
- external Claude OAuth import in `ontocode-external-agent-migration`
- provider/runtime auth seam in `ontocode-model-provider`

What remains insufficient:

- Gemini OAuth is not yet a native first-class reusable auth owner.
- Copilot currently uses provider `env_key` input plus runtime token exchange, but does not persist a first-class reusable Copilot OAuth credential.
- There is no single provider-neutral secret-bearing OAuth record spanning Codex/OpenAI, Claude, Gemini, and Copilot.

Architecture rule after review:

- Reuse the existing routing view, refresh orchestrator, and provider auth seam.
- Do not add a second registry, second refresh engine, or second provider adapter stack.
- Add only a narrow internal secret-bearing OAuth credential model plus source adapters and runtime projections.

## Stage Order

| Stage | Option | Goal | Status |
| --- | --- | --- | --- |
| S1 | 3 | Add provider-neutral alias/prefix/account-group routing | completed |
| S2 | 1 | Normalize existing credentials into one internal registry model | completed |
| S3 | 4 | Unify refresh orchestration across credential owners | completed |
| S4 | 2 | Add multi-account scheduling and failover policy | completed |
| S5 | 5 | Add a provider-auth adapter SDK for future providers | completed_by_reuse_decision |

## S1: Model Alias And Prefix Router First

### Goal

Add a routing layer above current auth stores so model requests can be mapped to provider, alias, prefix, and account-group policy before credential resolution.

### Change Home

- Primary owner: `ontocode-rs/model-provider`
- Integration points:
  - provider selection entrypoints in app-server, CLI, and core request construction
  - current provider descriptors/capability surfaces

### Deliverables

- Provider-neutral routing config model for:
  - `model_alias`
  - `model_prefix`
  - `provider_preference`
  - `account_group`
  - `fallback_policy`
- Resolver that converts an external model request into a normalized routing decision
- Diagnostics showing:
  - requested model
  - resolved provider
  - alias/prefix rule used
  - blocked or ambiguous routing reason

### Challenge Notes

- S1 must not begin by adding user-visible config keys; start with internal resolver inputs wired from existing metadata and test fixtures.
- Prefix support must not become a second model catalog; it is only a routing hint on top of existing provider/model ownership.
- Ambiguous aliases must fail closed with diagnostics instead of silently picking the first provider.

### Required Tests

- Alias resolution integration tests
- Prefix collision tests
- Provider fallback tests
- Redacted diagnostics coverage

### Exit Criteria

- No provider-specific hardcoding added to CLI or app-server selectors
- Existing provider descriptor flow remains the single provider-routing owner

## S2: Unified Credential Registry

### Goal

Introduce one internal credential record shape that can represent existing Codex/OpenAI login state, MCP OAuth state, Claude import state, and future Gemini state without moving source-of-truth ownership away from current stores.

### Change Home

- Registry model near `ontocode-rs/model-provider` or a narrow sibling crate
- Source adapters in:
  - `ontocode-rs/login`
  - `ontocode-rs/rmcp-client`
  - `ontocode-rs/external-agent-migration`

### Deliverables

- Internal `StoredProviderCredential` model with fields such as:
  - `provider`
  - `account_id`
  - `credential_id`
  - `auth_kind`
  - `status`
  - `expires_at`
  - `scopes`
  - `model_prefixes`
  - `aliases`
  - `priority`
  - `group`
  - `provenance`
- Read adapters from current auth stores into the normalized model
- Redacted credential-summary diagnostics

### Challenge Notes

- `StoredProviderCredential` is too close to a persistence name; implementation should prefer a name that signals normalized view or routing record.
- This stage must not write tokens into a new shared file, keyring namespace, or database.
- Registry assembly should tolerate partial provider coverage so one missing adapter does not block all routing diagnostics.

### Required Tests

- Normalization tests for each current source
- Expired or incomplete credential-state tests
- Provenance and redaction assertions

### Exit Criteria

- Source stores remain authoritative
- No raw token values appear in registry diagnostics or snapshots

## S3: Refresh Orchestrator Unification

### Goal

Unify credential-refresh execution so refreshable provider credentials are managed through one orchestration path with provider-specific refresh adapters behind it.

### Change Home

- Primary owner: shared auth/runtime boundary between `ontocode-rs/login` and `ontocode-rs/rmcp-client`
- Provider refresh adapters plugged in per source owner

### Deliverables

- Refresh orchestration service with:
  - eligibility checks
  - refresh deadline policy
  - refresh backoff
  - last-refresh status
  - bounded failure diagnostics
- Adapter contract for refresh implementations
- Shared health/status surface for refresh state

### Challenge Notes

- Unification means orchestration unification first, not forced code motion of every refresh implementation into one crate.
- Existing proactive RMCP refresh logic should be reused as an implementation source, not duplicated.
- Refresh state must distinguish `non_refreshable`, `refreshable_but_unavailable`, `refresh_failed`, and `refresh_suppressed` so later scheduling can reason correctly.

### Required Tests

- Successful refresh path tests
- Non-refreshable credential tests
- Backoff and timeout tests
- Redaction tests for refresh failures

### Exit Criteria

- Existing refresh logic routes through one orchestration surface
- No duplicate refresh loops per provider family

## S4: Credential Registry Plus Account Scheduler

### Goal

Add multi-account selection policy once routing and normalized credential views already exist.

### Change Home

- Primary owner: `ontocode-rs/model-provider`
- Scheduler consumes normalized credential views from S2 and health from S3

### Deliverables

- Scheduling policies:
  - round-robin
  - weighted priority
  - failover
  - sticky session
  - group-local selection
- Account eligibility evaluation using:
  - provider match
  - alias/prefix match
  - credential health
  - quota or cooldown signals when available
- Scheduler diagnostics and decision trace

### Challenge Notes

- Round-robin should not become the mandatory default until provider/account health semantics are stable.
- Sticky-session behavior must define reset boundaries explicitly or it will conflict with failover.
- Cross-provider fallback must be opt-in; a request targeting one provider family must not silently jump to another family because aliases overlap.

### Required Tests

- Multi-account round-robin tests
- Sticky-session tests
- Failover after refresh or request failure
- Deterministic selection-order tests

### Exit Criteria

- Scheduler is policy-driven, not provider-branch-driven
- Selection decisions are explainable through diagnostics

## S5: External Provider Adapter SDK

### Goal

Define a narrow provider-auth adapter contract so future heterogeneous providers can be added without changing core routing and credential orchestration logic.

### Change Home

- Primary owner: existing provider runtime boundary around `ontocode-rs/model-provider`
- Public exposure gated by the accepted adapter/public schema ADR

### Deliverables

- Narrow trait family for:
  - acquire
  - import
  - refresh
  - resolve models
  - build runtime auth
- First implementations for current heterogeneous targets:
  - Codex/OpenAI
  - Claude
  - Gemini
- Compatibility notes for future external-provider onboarding

### Challenge Notes

- This stage should extract the minimal internal contract proven by S1-S4; it should not begin from a speculative universal provider SDK.
- If current native providers already expose the needed seams, prefer thin adapters over rewrapping the whole runtime.
- Public third-party adapter ambitions must not delay the internal heterogeneous-provider refactor.

### Required Tests

- Adapter conformance tests
- Runtime auth-build tests
- Capability mismatch tests
- Backward-compatibility coverage for existing providers

### Exit Criteria

- New providers plug into existing routing/registry/orchestrator layers
- No provider adds a bespoke parallel auth stack

## Dependencies And Gating

- `ADR_PUBLIC_ADAPTER_SDK_SCHEMA_MIGRATIONS.md` remains the public-surface compatibility gate for config, app-server, and SDK exposure.
- `Claude OAuth Live Validation` remains a separate evidence gate for live imported Claude credential coverage.
- Any public config, schema, SDK, or app-server changes require compatibility tests before implementation.

## Follow-On Gap

This gap is adjacent to the completed refactor, but should be tracked as a separate implementation line.

### Goal

Add one provider-neutral secret-bearing OAuth credential model that can be populated from existing and future provider auth flows, then projected into current runtime owners.

### Why It Is Still Needed

- `ProviderCredentialRoutingView` is intentionally redacted-only.
- `StoredOAuthTokens` is MCP-shaped rather than provider-neutral.
- `ExternalAuthTokens` is bearer-token-shaped rather than refreshable OAuth-shaped.
- Copilot requires a GitHub OAuth/access token plus a runtime exchange token, but only the first token should be canonical.

### Recommended Shape

- internal `ProviderOAuthCredential` or equivalent name, with fields for:
  - `provider`
  - `account_id`
  - `credential_id`
  - `access_token`
  - `refresh_token`
  - `client_id`
  - `token_endpoint`
  - `scopes`
  - `expires_at`
  - `source_kind`
  - `provenance`

### Provider Mapping Rules

- Codex/OpenAI: project current first-party login/auth records into the canonical OAuth shape when applicable.
- Claude: project imported and future native OAuth credentials into the canonical shape, then into MCP/runtime-specific storage where needed.
- Gemini: add a future Google OAuth source adapter instead of inventing Gemini-specific parallel routing/auth logic.
- Copilot: treat the GitHub OAuth/access credential as canonical and the exchanged Copilot token as a runtime projection only.

### Explicit Non-Goals

- no second token database
- no replacement of `ontocode-login` or `ontocode-rmcp-client` as persistence owners
- no direct runtime use of redacted routing views as secret-bearing auth objects
- no provider-specific branching in CLI or app-server to compensate for missing canonical OAuth modeling

## Stage-Specific Blockers

| Stage | Hard blocker | Reason |
| --- | --- | --- |
| S1 | public-surface drift | Internal routing must be proven before schema or API exposure |
| S2 | second-store drift | Normalized credential view must not become persistence authority |
| S3 | duplicate refresh loops | Existing refresh owners must be converged, not shadowed |
| S4 | undefined health semantics | Scheduling needs stable eligibility and refresh-state meaning |
| S5 | premature public SDK scope | Internal contract must be proven before extension packaging |

## Implementation Guidance

- Start with private/internal wiring first.
- Keep all new diagnostics redacted and bounded.
- Prefer integration tests over unit-only coverage for routing and refresh behavior.
- Run impact analysis before changing selection or auth-owner entrypoints.
- Refresh OntoIndex after each implementation slice so routing and auth graph changes stay queryable.
- Prefer read models and decision traces before adding write paths or automation.
- If a stage exposes more than one new concept at once, split it again before dispatch.

## Recommended Dispatch Units

1. F1-A: internal canonical OAuth credential type and ownership decision
2. F1-B: source adapters for Codex/OpenAI and Claude into the canonical shape
3. F1-C: Copilot canonical-source plus runtime-projection split
4. F1-D: Gemini OAuth source-adapter design and compatibility tests
5. F1-E: redacted diagnostics proving canonical-to-routing projection remains bounded and secret-safe

## Success Criteria

- A model request can be resolved, routed, credential-selected, refreshed if needed, and explained through diagnostics without provider-specific branching spread across the stack.
- Existing auth owners remain intact and reusable.
- Adding one more heterogeneous provider does not require another architecture rewrite.
- Secret-bearing OAuth credentials for Codex/OpenAI, Claude, Gemini, and Copilot can be represented through one internal canonical model without creating a second persistence authority.
