# ADR: Claude OAuth Import And Broad Provider Refactor

## Status

Challenged; superseded for model runtime by the 2026-06-19 OpenAI-only native
auth policy.

## Date

2026-06-04

## Context

Current authority, 2026-06-19:

- OpenAI/Codex is the only native OAuth-backed model provider.
- Claude model runtime must be configured as an external OpenAI-compatible API
  provider or sidecar.
- Ontocode core must not import, persist, refresh, or consume Claude OAuth
  credentials for model execution.
- Claude MCP OAuth, if ever needed, remains a separate MCP-domain question and
  must not become model-provider auth.

The previous 2026-06-18 authority below is superseded where it implied future
Claude model OAuth work inside Ontocode core.

The current codebase has two separate authentication domains:

- model-provider authentication centered on `ModelProvider`, `AuthManager`, and provider-specific auth resolution
- MCP OAuth authentication centered on `perform_oauth_login_return_url` and `mcpServer/oauth/login`

This separation works for the current first-party and custom-provider cases, but it is a weak foundation for direct credential import from external tools such as Claude Code.

The immediate product goal is to support importing OAuth-backed Claude credentials instead of only importing `.claude` MCP configuration and asking the user to authenticate again.

The broader architectural goal is to support more providers without expanding hard-coded branches or embedding provider-specific credential translation logic throughout the runtime.

Superseded authority, 2026-06-18:

- This ADR does not authorize a broad credential broker or second auth stack.
- Claude OAuth import/runtime remains blocked on sanitized real credential
  evidence and security review.
- Codex/OpenAI remains the first-class default and fallback.
- Future Claude OAuth work must be additive and provider-scoped; it must not
  replace, mutate, or make Codex/OpenAI depend on Claude auth.
- This authority is replaced by the 2026-06-19 revision above and
  [OpenAI-Only Provider Policy Cleanup](audit_session-2026-06-19-openai-only-provider-policy.md).

## Problem

If Claude credential import is added directly to the current design, the most likely result is:

- Claude-specific parsing inside generic auth paths
- more branching in provider selection and auth resolution
- duplicated logic across model-provider auth and MCP OAuth
- poor support for future external providers
- increased security and maintenance risk around foreign credential handling

The current `AuthManager` is optimized for one active top-level auth state plus optional external bearer refresh. It is not a natural multi-provider credential broker.

## Decision Drivers

- support direct credential translation from Claude without leaking Claude-specific assumptions into core auth code
- preserve the existing working MCP OAuth flow
- support future providers beyond Claude
- isolate foreign credential parsing for security review
- avoid turning `AuthManager` into a catch-all multi-provider subsystem
- enable incremental delivery

## Options Considered

### Option 1: Credential Broker Plus Adapter Registry

Introduce a new shared credential subsystem that stores multiple credentials by target subject, with adapter traits for:

- importing foreign credentials
- refreshing OAuth credentials
- applying request auth to outbound requests

Example subjects:

- `model_provider/openai`
- `mcp_server/slack`
- `external_connector/claude_ai/gmail`

Pros:

- clean separation between storage, import, refresh, and request wiring
- Claude import can be isolated in a single adapter
- extensible to additional providers
- avoids inflating `AuthManager`

Cons:

- moderate refactor across auth-related crates

### Option 2: Unified OAuth Account Model

Create one normalized internal OAuth credential shape used by both model-provider auth and MCP auth.

Example fields:

- issuer
- client_id
- subject or account_id
- scopes
- access_token
- refresh_token
- expires_at
- auth_server_metadata_url
- redirect_strategy

Pros:

- imported credentials become a translation problem rather than a storage redesign
- strongest long-term portability story
- improves consistency across OAuth-backed integrations

Cons:

- larger schema and migration impact
- broader rollout than needed for the first implementation

### Option 3: Shared Token Store, Separate Runtime Domains

Keep model-provider auth and MCP OAuth separate, but share only the credential persistence layer.

Pros:

- lowest-risk incremental change
- simpler to land quickly
- preserves current runtime architecture

Cons:

- selector logic stays duplicated
- does not fully solve provider growth or branching complexity

### Option 4: Capability-Driven Provider Selector

Replace hard-coded provider branching with a registry where each provider declares:

- supported wire APIs
- supported auth schemes
- whether it owns model catalog behavior
- whether it supports OAuth import
- whether it can refresh credentials

Pros:

- removes hard-coded provider branching
- supports broad provider growth
- combines well with a shared credential layer

Cons:

- does not by itself solve credential normalization or foreign import

### Option 5: Foreign Identity Bridge

Introduce a dedicated boundary layer for external credential sources. This layer reads foreign stores and emits only normalized internal records plus provenance metadata.

Example provenance:

- source application
- source account
- import timestamp
- refreshability
- validation state

Pros:

- strong isolation for risky foreign parsing
- easier security review and revocation flows
- avoids spreading Claude-specific parsing into core crates

Cons:

- still requires a storage and selection strategy behind it

## Candidate Decision Under Challenge

The original proposed direction was:

- Option 1 as the primary refactor shape
- Option 4 for provider selection
- Option 5 for Claude-specific import isolation

This direction is now challenged. It should not be implemented until the evidence gates in the counterproposal are satisfied.

One part remains valid regardless of the final design: do not extend `AuthManager` into the universal broker.

Instead:

- keep `AuthManager` focused on the current user-session and provider-scoped auth flows it already owns
- add a separate credential broker for imported and provider-targeted credentials
- add a provider and auth adapter registry for selection
- add a Claude bridge that translates foreign credentials into internal broker records

## Challenge Summary

The decision above is directionally clean, but it is not yet justified by the evidence currently available in the repo.

The main weakness is that it selects a broad new credential broker and provider selector before proving that:

- Claude credentials can be legally and technically imported
- the existing MCP OAuth token store cannot be reused
- the existing `ExternalAuth` pattern cannot cover the first viable import path
- model-provider selection must change to support Claude MCP credential import

Until those points are proven, the ADR should be treated as a candidate architecture, not an implementation mandate.

## Challenges

### 1. Claude Credential Import Is Not Yet Proven Feasible

The `tmp/claude-code-main` checkout does not contain the Claude Code runtime credential implementation. It contains plugins, docs, and changelog entries. That means the ADR currently assumes details that are not established:

- where Claude stores OAuth credentials
- whether those credentials are raw provider OAuth tokens or Claude-managed session artifacts
- whether refresh tokens are present
- whether token refresh requires Claude-specific client credentials, redirect URIs, or backend mediation
- whether importing those credentials is allowed by product, legal, and security policy

This is a gating issue. If Claude stores only opaque claude.ai connector grants, there may be no direct token translation path into MCP OAuth storage.

### 2. A New Broker May Duplicate Existing MCP OAuth Storage

The repo already has MCP OAuth storage in `ontocode-rs/rmcp-client/src/oauth.rs`.

That implementation already includes:

- `StoredOAuthTokens`
- keyring storage
- file fallback storage
- delete support
- token expiry tracking
- `OAuthPersistor` for persisting refreshed credentials

Before creating a new broker, the ADR must explain why this store cannot be generalized or wrapped for imported MCP tokens. The current decision risks creating a second credential persistence layer with overlapping responsibility.

### 3. `AuthManager` Is Not Universal, But It Already Has Extension Points

The ADR correctly warns against turning `AuthManager` into a universal credential broker. However, it underplays the existing `ExternalAuth` extension point.

App-server already uses an `ExternalAuthRefreshBridge` to delegate refresh to a client-facing flow. That pattern may be enough for some imported or externally managed credentials, especially if Claude credentials cannot be persisted as raw OAuth tokens.

The ADR should distinguish:

- credentials that can be stored and refreshed locally
- credentials that must be refreshed by an external source
- credentials that are only usable through a foreign runtime or backend

### 4. Provider Selector Refactor May Be Unrelated To The First Claude Goal

The immediate goal is Claude OAuth import for MCP connectors. The ADR also proposes refactoring model-provider selection.

That may be valuable, but it is not clearly required for the first Claude import milestone. Combining these efforts increases scope and makes the first implementation harder to review.

The model-provider selector work should be a separate ADR or a later phase unless the first implementation proves it is required.

### 5. Credential Subject Identity Is Underspecified

The ADR examples use subject strings such as:

- `model_provider/openai`
- `mcp_server/slack`
- `external_connector/claude_ai/gmail`

This is too loose for credential correctness. Existing MCP OAuth storage keys tokens by server name and URL. OAuth identity may also need:

- issuer
- resource
- client_id
- scopes
- account id
- environment or workspace id
- source application provenance

If subject identity is not precise, imported credentials could be applied to the wrong connector or account.

### 6. Token Refresh Semantics Are A Major Missing Constraint

Importing an access token is not enough. The architecture must define what happens when it expires.

Critical questions:

- Can the token be refreshed by this app?
- Does refresh require Claude's OAuth client id?
- Does refresh require a Claude-managed session cookie or account token?
- Are refresh scopes identical to MCP scopes?
- What happens if refresh succeeds in Claude but fails here?

If refresh cannot be performed locally, storing imported tokens may create a short-lived and confusing user experience.

### 7. Security Requirements Are Too Thin

The ADR says foreign parsing should be isolated, but it does not define the minimum security bar.

Any direct credential import design needs explicit requirements for:

- user consent before reading foreign credential stores
- visible provenance for imported credentials
- never logging secrets
- redaction in diagnostics
- revocation and deletion
- account boundary validation
- token scope validation
- storage encryption or keyring policy
- handling locked keychains and partial import failures

Without these requirements, the design is incomplete.

### 8. The Recommended First Step Is Too Broad

The current staging starts with:

1. Introduce the credential broker interfaces and storage model.
2. Introduce registry-based selector interfaces.

That front-loads abstraction before proving the foreign credential path.

A better first step is a narrow evidence-gathering implementation that answers whether Claude credentials can be converted into the existing MCP OAuth token model.

## Counterproposal

Use a staged validation path before committing to the broker and selector refactor.

### Stage 0: Evidence Gate

Determine the actual Claude credential shape and policy constraints.

Required outputs:

- credential locations and formats
- whether credentials are raw OAuth tokens or opaque Claude connector grants
- refresh mechanism
- account and scope metadata
- legal/security constraints for import
- one redacted live sample bundle that preserves the connector boundary and can
  be validated without exposing secret values

No broad refactor should start before this is complete.

### Stage 1: Minimal Import Spike

Attempt to map one Claude MCP connector credential into the existing `StoredOAuthTokens` shape.

Success criteria:

- imported token can be stored with the current MCP OAuth store
- `mcpServerStatus/list` reports authenticated state
- the MCP server can be called successfully
- refresh either works locally or fails with a clear recoverable state

The only approved live-evidence input for this stage is a redacted bundle that
contains:

- source metadata for the machine and Claude build
- a connector-scoped credential record with preserved `connector_name`,
  `server_url`, `client_id`, `scopes`, and expiry fields when present
- deterministic redactions for tokens, account IDs, workspace IDs, and emails
- validator output showing `Complete` or `Partial` with at least one
  importable credential

If the live sample instead proves only an opaque global grant, that is a valid
decision record but not a basis for enabling runtime import.

If this succeeds, the first production implementation should prefer extending the existing MCP OAuth store instead of creating a new broker.

### Stage 2: Foreign Identity Bridge

Add a Claude-specific import boundary only after the token shape is understood.

This bridge should output internal MCP OAuth records or an explicit non-importable status. It should not invent a new credential substrate unless Stage 1 proves the existing one is insufficient.

### Stage 3: Generalization Decision

Decision: do not introduce a new credential broker for the current Claude MCP OAuth import effort.

Current repo evidence does not justify it:

- T1 did not establish a schema-level Claude credential format
- T2 and T3 are blocked pending one sanitized real Claude credential sample
- the existing MCP OAuth store already supports the first import target shape
- externally mediated refresh already has an extension seam

Reopen this decision only if T2 or T3 prove one of these:

- imported credentials cannot be represented as `StoredOAuthTokens`
- imported credentials must target non-MCP domains in the first shipped path
- local and external refresh need a shared lifecycle the current store cannot support
- provenance, revocation, or validation cannot be layered onto the current MCP store cleanly

### Stage 4: Provider Selector ADR

Move capability-driven model-provider selection into a separate ADR.

That work may still be useful, but it should not be a prerequisite for Claude MCP OAuth import unless implementation evidence says otherwise.

## Revised Recommendation

- keep `AuthManager` focused on user-session auth
- treat the existing MCP OAuth store as the only approved first storage target
- add a Claude-specific bridge only after a real credential sample exists
- prefer re-auth or externally mediated refresh when direct token import is not viable
- keep provider-selector work in `ADR_MODEL_PROVIDER_SELECTOR_REFACTOR.md`

## Rationale

This combination provides the best balance between:

- immediate Claude import requirements
- clean separation of responsibilities
- future provider support
- security containment
- incremental delivery

It avoids the most likely failure mode: implementing Claude import through scattered special cases in `AuthManager`, `ModelProvider`, and MCP runtime code.

## Deferred Generalization

Credential-broker, adapter-registry, and capability-selector work are not approved by this ADR.

They may be reconsidered only after T2 or T3 produce concrete evidence that the existing MCP OAuth store and external-refresh seams are insufficient.

## Non-Goals

- fully unify all authentication flows into a single monolithic auth manager
- rewrite the existing MCP OAuth flow before the broker exists
- implement direct Claude credential import in this ADR

## Open Questions

- What is the stable minimum credential shape needed across all OAuth-backed integrations?
- Should broker records be stored in the existing auth storage backend, a new backend, or both?
- Which crate should own the broker API to minimize cross-crate coupling?
- How should imported credentials be revoked, invalidated, or re-imported?
- What validation level is required before imported foreign credentials become active?
