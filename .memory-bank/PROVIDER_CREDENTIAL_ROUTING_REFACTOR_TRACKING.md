---
name: Provider Credential Routing Refactor Tracking
description: Dispatch and verification tracker for the staged provider-neutral credential routing refactor
type: tracking
date: 2026-06-13
status: active
---

# Provider Credential Routing Refactor Tracking

Authority: `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md`.

## Stage Status

| Stage | Slice | Status | Notes |
| --- | --- | --- | --- |
| S1 | S1-A internal alias/prefix routing core | done | Private `model-provider::route` core landed with exact-alias, longest-prefix, provider-hint mismatch, and fail-closed resolution tests |
| S1 | S1-B routing diagnostics and tests | done | Private route diagnostic snapshot landed with matched/no-match/blocked states and blocked-reason coverage |
| S2 | S2-A normalized credential view and source adapters | done | Shared redacted credential-view types landed in `ontocode-protocol`; additive source adapters landed in `login`, `rmcp-client`, and Claude import without changing persistence or refresh behavior |
| S2 | S2-B credential-summary diagnostics and redaction tests | done | Bounded `ProviderCredentialRoutingSummary` landed in `ontocode-protocol` with redaction coverage for optional identifiers and scope-count summaries |
| S3 | S3-A refresh adapter contract and orchestrator | done | New `ontocode-provider-auth` crate landed with async refresh-descriptor/refresh orchestration contracts; thin adapters landed in `login` and `rmcp-client` without moving refresh ownership |
| S3 | S3-B refresh diagnostics and backoff tests | done | Shared orchestrator now records bounded failure details, failure kind, consecutive-failure count, and transient retry suppression/backoff without moving refresh ownership out of existing auth owners |
| S4 | S4-A scheduler policies and deterministic selection tests | done | Private `model-provider::schedule` core landed with round-robin, priority, and failover policies plus deterministic trace coverage over normalized credential views and S3 refresh diagnostics |
| S4 | S4-B failover and sticky-session coverage | done | Sticky-session behavior landed in the private scheduler core with explicit reset boundaries and failover to the next eligible credential when the previous sticky selection becomes ineligible |
| S5 | S5-A internal provider-auth adapter contract extraction | done | Senior review collapsed this slice into the existing private `ModelProvider` auth seam (`auth_manager`, `auth`, `api_auth`, `account_state`, runtime engine selection); no second provider-auth trait family was added because it would duplicate the current owner |
| S5 | S5-B Codex/Claude/Gemini adapter conformance coverage | done | Existing `model-provider` tests already cover OpenAI/Codex, Anthropic/Claude, Gemini, Copilot, and Bedrock runtime/descriptor/auth behavior under the single private provider owner |
| F1 | F1-A internal canonical OAuth credential type | done | Additive `ontocode_provider_auth::ProviderOAuthCredential` landed as the canonical internal secret-bearing OAuth record with redacted debug behavior and projection to `ProviderCredentialRoutingView`; no source adapter rewiring landed in this slice |
| F1 | F1-B source adapters into canonical OAuth credential | done | `ExternalAuthTokens`, `StoredOAuthTokens`, and `ImportableMcpOAuthCredential` now expose additive conversion into `ProviderOAuthCredential`; persistence and refresh authority remain in existing owners |
| F1 | F1-C Copilot canonical-source and runtime-projection split | done | Copilot now projects GitHub OAuth/access input into canonical `ProviderOAuthCredential` source material while keeping the exchanged Copilot token runtime-only; both targeted core tests passed |
| F1 | F1-E canonical-to-routing redaction coverage | done | Additive `to_routing_summary()` projection landed in `ontocode-provider-auth` with coverage proving canonical OAuth credentials cannot leak secret-bearing or over-detailed fields into bounded routing diagnostics |
| F1 | F1-D Gemini OAuth source-adapter design and compatibility gap | done | Senior decision accepted: current Gemini support remains explicitly API-key-only until a real Gemini OAuth source owner exists; compatibility coverage now locks the native runtime boundary so no fake OAuth adapter is introduced prematurely |

## Current Dispatch

- Active slice: none
- Owner mode in this session: manager/senior direct execution
- Constraint: no callable sub-agent spawn surface is available in this chat, so delegated worker execution is simulated by sequential in-session delivery with verification after each slice
- Senior unblock: the completed S1-S5 line remains closed; the active work is now the follow-on canonical OAuth credential gap recorded in `PROVIDER_CREDENTIAL_ROUTING_REFACTOR_PROJECT_PLAN.md`

## Completed Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-model-provider`
- `git diff --check`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-protocol`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login external_auth_tokens_produce_redacted_provider_credential_routing_view`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client stored_oauth_tokens_produce_redacted_provider_credential_routing_view`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-migration importable_mcp_oauth_credential_produces_redacted_provider_routing_view`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-protocol`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth collect_descriptors_skips_absent_adapters`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth refresh_eligible_preserves_redacted_routing_records`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login auth_manager_refresh_adapter_reports_healthy_chatgpt_auth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login auth_manager_refresh_adapter_reports_non_refreshable_api_key_auth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client oauth_persistor_refresh_adapter_reports_healthy_tokens`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-rmcp-client`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-rmcp-client`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-external-agent-migration`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-protocol credential_routing`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-protocol`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core builds_canonical_copilot_source_oauth_credential_from_env_key`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core exchanges_github_token_for_copilot_token`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core native_runtime_requires_provider_env_key_api_key_auth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login auth_manager_refresh_adapter_reports_healthy_chatgpt_auth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client oauth_persistor_refresh_adapter_reports_healthy_tokens`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-model-provider`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-provider-auth`
- `ontoindex analyze`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client stored_oauth_tokens_convert_to_provider_oauth_credential`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-migration`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-rmcp-client`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-external-agent-migration`

## Verification Rules

- Update this file before starting the next slice.
- Run OntoIndex refresh after each completed slice.
- Keep S1 internal-only until public-surface gates are separately approved.
- Record blockers here before escalating them into plan or ADR changes.

## Blockers

- `ontocode-rmcp-client` full-crate verification retains one unrelated existing harness failure:
  - `streamable_http_remote_client_round_trips_through_exec_server`
  - cause: missing `codex` binary in the test environment
  - status: not caused by F1-B; the new conversion test passed
- `just fix -p ontocode-core` currently fails on an unrelated pre-existing Clippy warning in `core/src/mcp_tool_call_tests.rs`:
  - `clippy::unnecessary_literal_unwrap`
  - status: not caused by the Gemini compatibility slice
