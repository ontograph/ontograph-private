# ADR: Separate Model Provider Selector Refactor

## Status

Proposed

## Date

2026-06-04

## Context

Model-provider runtime selection is currently hard-coded in `create_model_provider` in `ontocode-rs/model-provider/src/provider.rs`.

The current shape already has more than one provider class:

- the default OpenAI-compatible configured-provider path
- the Amazon Bedrock path with custom auth, custom runtime base URL handling, custom capabilities, and custom model-catalog behavior

This means the system is not selecting between simple provider configs. It is already selecting between distinct runtime provider implementations.

At the same time, provider predicates and auth assumptions are visible outside `ontocode-rs/model-provider`, including in `core`, `cli`, and `app-server`.

This architecture concern is separate from Claude MCP OAuth credential import. Mixing them into one decision record would conflate:

- foreign credential import and token ownership
- model-provider runtime construction and provider-class branching

## Problem

The current factory seam is small in code size but large in blast radius.

Provider selection is still expressed as direct branching on provider metadata rather than as an explicit selector concept. That makes it harder to:

- add another non-default provider class
- reason about provider-specific auth behavior
- keep provider construction logic localized
- prevent provider predicates from spreading further across the codebase

## Decision

Treat model-provider runtime selection as a separate architecture concern from Claude OAuth import.

The first stage of this refactor should introduce an internal selector for provider-class resolution inside `ontocode-rs/model-provider`, while preserving current runtime behavior.

The selector refactor is non-blocking for Claude MCP OAuth import and should not be made a prerequisite for that work.

## Recommended Scope

- Introduce an internal selector result such as `ProviderKind` or an equivalent resolver-owned classification.
- Route `create_model_provider` through that selector instead of branching directly on `provider_info.is_amazon_bedrock()`.
- Keep provider-specific construction and auth wiring tied to the selected provider class.
- Preserve the existing external behavior of `ModelProvider`, `ConfiguredModelProvider`, and `AmazonBedrockModelProvider` in the first implementation slice.

## Non-Goals

- no credential-broker design work
- no Claude OAuth import design
- no rewrite of app-server or TUI authentication UX
- no requirement to immediately remove every provider predicate outside `ontocode-rs/model-provider`

## Rationale

This refactor deserves a separate ADR because:

- it is a general platform architecture decision, not Claude-specific work
- `create_model_provider` is a high-impact factory seam
- the design pressure comes from provider-class growth, not from one import workflow
- keeping it separate reduces scope and review complexity for the Claude MCP import path

## Migration Triggers

This ADR should move from design-only to implementation when one or more of these become true:

- a second non-default hosted provider requires a custom runtime implementation
- another provider needs custom `models_manager`, `api_auth`, or `account_state`
- more provider predicates appear outside `ontocode-rs/model-provider`
- provider auth resolution needs more than the current configured-provider path plus Bedrock special casing

## Minimum Implementation Slices

1. ADR and invariants only.
2. Introduce the internal selector result type with no behavior change.
3. Route `create_model_provider` through the selector.
4. Move configured-provider and Bedrock construction behind selector-owned constructors or equivalent helpers.
5. Audit external provider predicates and classify them as:
   - leave as-is
   - migrate later
   - must migrate now

## Current Coupling Points

- `create_model_provider` is the primary selection seam and has broad upstream impact.
- `AuthManager::external_bearer_only(...)` is a secondary auth-coupling point for provider command auth.
- `requires_openai_auth` currently influences account UX and auth gating in multiple layers.
- provider metadata predicates remain visible outside `ontocode-rs/model-provider`, so selector introduction alone does not complete provider encapsulation.
- Amazon Bedrock already proves the codebase has more than one runtime provider class.

## Consequences

### Positive

- provider-class selection becomes explicit
- future custom providers get a clearer integration path
- Claude MCP OAuth import stays decoupled from model-provider factory work
- the first implementation can stay behavior-preserving

### Negative

- this adds another design artifact to maintain
- the first implementation slice may not visibly reduce all external provider branching
- follow-up classification work is required to decide which external predicates should remain

## Follow-Up

- keep this ADR separate from `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR.md`
- use GitNexus impact analysis before touching `create_model_provider` or related provider-selection symbols
- record any selector-driven implementation stages in the dedicated tracking file for that work when implementation begins
