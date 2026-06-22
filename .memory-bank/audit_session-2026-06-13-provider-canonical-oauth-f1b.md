# Provider Canonical OAuth F1-B

Date: 2026-06-13

Scope:
- `F1-B` from `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md`
- Add source adapters from existing OAuth owners into the canonical internal OAuth credential type

Outcome:
- Added `to_provider_oauth_credential()` to `ExternalAuthTokens`
- Added `to_provider_oauth_credential()` to `StoredOAuthTokens`
- Added `to_provider_oauth_credential()` to `ImportableMcpOAuthCredential`
- Added coverage in `ontocode-login`, `ontocode-rmcp-client`, and `ontocode-external-agent-migration`
- Kept all persistence and refresh ownership unchanged

Verification:
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client stored_oauth_tokens_convert_to_provider_oauth_credential`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-migration`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-rmcp-client`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-external-agent-migration`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `git diff --check`
- `ontoindex analyze`

Residual verification note:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client` still has one unrelated existing environment/harness failure:
  - `streamable_http_remote_client_round_trips_through_exec_server`
  - reason: test could not locate binary `codex`
  - the new canonical OAuth conversion test passed

Next:
- `F1-C`: treat Copilot as canonical GitHub OAuth/access source plus runtime-only Copilot exchange token projection
