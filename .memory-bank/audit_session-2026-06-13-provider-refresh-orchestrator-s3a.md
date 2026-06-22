# Provider Refresh Orchestrator S3-A

Date: 2026-06-13
Status: accepted

## Scope

- `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md` `S3-A`
- Shared refresh adapter contract and thin integration into existing auth owners

## Landed

- New internal crate: `ontocode-provider-auth`
- Async shared refresh contract:
  - `ProviderCredentialRefreshAdapter`
  - `ProviderCredentialRefreshDescriptor`
  - `ProviderCredentialRefreshOutcome`
  - `ProviderCredentialRefreshOrchestrator`
- Thin first-party adapter:
  - `AuthManagerRefreshAdapter`
- Thin RMCP adapter:
  - `OAuthPersistorRefreshAdapter`

## Architecture decision

- Refresh ownership stays in `ontocode-login` and `ontocode-rmcp-client`.
- `ontocode-provider-auth` owns only the shared orchestration contract.
- No second token store, refresh loop, or provider registry was introduced.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth collect_descriptors_skips_absent_adapters`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth refresh_eligible_preserves_redacted_routing_records`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login auth_manager_refresh_adapter_reports_healthy_chatgpt_auth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login auth_manager_refresh_adapter_reports_non_refreshable_api_key_auth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client oauth_persistor_refresh_adapter_reports_healthy_tokens`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-rmcp-client`
- `git diff --check`
- `ontoindex analyze`

## Next slice

- `S3-B` bounded refresh diagnostics and backoff coverage
