# Provider Canonical OAuth F1-A

Date: 2026-06-13

Scope:
- `F1-A` from `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md`
- Land the canonical internal secret-bearing OAuth credential type without creating a second persistence authority

Outcome:
- Added `ontocode_provider_auth::ProviderOAuthCredential` in `ontocode-rs/provider-auth/src/oauth_credential.rs`
- Kept the type internal-owner scoped to `ontocode-provider-auth`
- Added redacted `Debug` behavior
- Added projection to existing `ProviderCredentialRoutingView`
- Explicitly treated the new type as additive only; no adapter rewiring landed in this slice

Verification:
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-provider-auth`
- `git diff --check`
- `ontoindex analyze`

Next:
- `F1-B`: source adapters from existing OAuth owners into the canonical type
