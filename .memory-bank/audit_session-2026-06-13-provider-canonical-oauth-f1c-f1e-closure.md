---
name: Provider Canonical OAuth F1-C F1-E Closure
description: Verification closure for Copilot canonical-source/runtime split and canonical-to-routing redaction coverage
type: audit
date: 2026-06-13
status: accepted
---

# Provider Canonical OAuth F1-C F1-E Closure

## Outcome

- `F1-C` accepted:
  - Copilot now projects GitHub OAuth/access source material into canonical `ProviderOAuthCredential`.
  - The exchanged Copilot token remains runtime-only and is not promoted to canonical persisted source material.
- `F1-E` accepted:
  - Canonical OAuth credentials now expose a direct `to_routing_summary()` helper.
  - Tests prove bounded redacted routing diagnostics do not expose access tokens, refresh tokens, raw identifiers, token endpoints, or raw scopes.

## Verification

- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core builds_canonical_copilot_source_oauth_credential_from_env_key`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core exchanges_github_token_for_copilot_token`

## Remaining Gap

- `F1-D` remains open:
  - Gemini support is still routing/runtime-shaped rather than a native first-class OAuth source owner.
  - A narrow ownership decision is required before implementing a Gemini canonical source adapter.
