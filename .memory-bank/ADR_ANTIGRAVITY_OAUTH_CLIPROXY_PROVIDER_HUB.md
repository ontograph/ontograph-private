---
name: Antigravity OAuth Import And CLIProxyAPI Provider Hub
description: Decision record for Antigravity OAuth import and limited CLIProxyAPI provider-hub use
type: adr
date: 2026-06-17
status: superseded_for_native_runtime
---

# ADR: Antigravity OAuth Import And CLIProxyAPI Provider Hub

## 2026-06-19 Revision: External-Only Antigravity

Native Antigravity model OAuth import/runtime is rejected. Antigravity may be
used only through a user-managed external OpenAI-compatible API endpoint or
sidecar.

Current policy:

- Ontocode core must not import, persist, refresh, or consume Antigravity OAuth
  credentials for model execution.
- CLIProxyAPI may be an external endpoint if the user runs it and configures it
  as a custom provider.
- CLIProxyAPI must not become a bundled provider hub, lifecycle dependency, or
  credential owner inside Ontocode.
- GPT/Codex remains the native default and must keep working when Antigravity
  credentials or sidecars fail.

The donor details below are retained only as historical sidecar/provider-hub
evidence.

## Context

`tmp/CLIProxyAPI-main` contains useful provider/account patterns for Gemini CLI and Antigravity:

- `internal/runtime/geminicli/state.go` keeps one shared OAuth credential with virtual per-project views.
- `internal/runtime/executor/antigravity_refresh_test.go` proves concurrent refresh deduplication for the same Antigravity refresh token.
- `internal/util/gemini_schema.go` has Gemini/Antigravity JSON-schema cleanup for tool-call compatibility.
- `internal/constant/constant.go` treats `antigravity`, `gemini`, and `gemini-cli` as distinct provider identifiers.
- `tmp/oh-my-pi-main/packages/ai/src/registry/oauth/google-antigravity.ts` adds loopback OAuth with fixed callback `http://localhost:<port>/oauth-callback`, fixed scopes, and Cloud Code Assist discovery via `POST /v1internal:loadCodeAssist` and onboarding fallback.

Ontocode already has the right native owners:

- Provider OAuth credentials persist through existing login/auth storage.
- Provider credential routing belongs in existing provider-auth/model-provider paths.
- `/login`, `/auth`, `/logout`, `/status`, and `/model` already have provider-aware surfaces.

Superseded authority, 2026-06-18:

- Codex/OpenAI remains the first-class default and fallback.
- Antigravity import is additive and provider-scoped. It must not create a
  global auth mode, replace Codex/OpenAI, or mutate Codex/OpenAI credentials.
- Antigravity runtime stays disabled until a runtime ADR and API proof exist.
- This authority is replaced by the 2026-06-19 revision above and
  [OpenAI-Only Provider Policy Cleanup](audit_session-2026-06-19-openai-only-provider-policy.md).

## Superseded Original Decision

Do not make CLIProxyAPI the built-in provider hub for Ontocode.

Use CLIProxyAPI in two narrow ways:

1. As a donor format for importing Antigravity OAuth credentials and project-discovery metadata.
2. As an optional external OpenAI-compatible provider endpoint for experiments.

Add native `antigravity` as the canonical provider id only when the import contract is ready. Keep Antigravity runtime and selectable models disabled until a real runtime adapter is approved and tested.

## Senior Challenge

The initial ADR was directionally right but too permissive in five places:

1. It implied `antigravity` provider identity can be added before the import owner is proven. Narrow it: provider identity can land only with import/status tests or as a disabled static row.
2. It listed shared virtual project credentials as a keeper without limiting scope. Narrow it: first slice stores one imported credential record with bounded metadata; shared/virtual project views wait until multi-project runtime demand exists.
3. It treated Cloud Code Assist discovery as runtime-ready. Narrow it: this slice captures discovery contract and result handling only; all discovery/network probing stays out of runtime.
4. It listed refresh dedupe as a keeper without a refresh adapter. Narrow it: no dedupe code until native refresh exists; the gate is one test proving concurrent refresh is deduped.
5. It showed CLIProxyAPI hub config as if product-supported. Narrow it: this is a user-managed custom provider example only. No bundled preset, health check, lifecycle management, or dependency on a local proxy.

OntoIndex evidence:

- Existing provider OAuth persistence is already covered by `AuthDotJson`, `upsert_provider_oauth_credential`, `load_provider_oauth_credential_by_provider_id`, and related storage tests.
- Existing runtime auth handoff is through `model-provider/src/auth.rs::auth_manager_for_provider`; do not add a second auth resolver.
- Existing slash auth flows already route through `ChatWidget.handle_auth_shell_command`; extend that surface only after parser/import tests exist.

## Accepted Native Shape

Provider id:

```text
antigravity
```

Initial command surface:

```text
/login antigravity --import <path>
/auth antigravity
/logout antigravity
/status antigravity
```

Import parser accepts only the minimal OAuth/account fields:

```text
access_token
refresh_token
token_type
expiry or expires_at
project_id (optional)
email or account id
scopes
```

Import rejects:

- files without `refresh_token`, unless explicitly recorded as non-refreshable and runtime-disabled
- files whose provider cannot be identified as Antigravity
- files that require a bundled OAuth client secret
- files where project discovery is required but missing and the parser cannot store bounded discovery metadata
- multi-account bundles where account selection is ambiguous

Persistence:

- Project imported credentials into the existing provider OAuth credential type.
- Save through existing Ontocode auth storage.
- Do not create a CLIProxyAPI-shaped token directory, proxy account store, or Antigravity-only secret store.

Readiness:

- `missing_credentials`
- `ready`
- `expired`
- `refresh_failed`
- `runtime_unavailable`
- `project_discovery_required`

All status and errors must redact access tokens, refresh tokens, client secrets, raw import paths, and keychain paths.

## Cloud Code Assist Discovery Gate

Antigravity donor flows include project discovery after OAuth token exchange:

- `POST https://cloudcode-pa.googleapis.com/v1internal:loadCodeAssist`
- payload `metadata.ideType` = `ANTIGRAVITY`
- fallback to `POST https://daily-cloudcode-pa.googleapis.com/v1internal:onboardUser` when discovery is empty
- discovery metadata in donor shape includes:
  - `tier_id`
  - `metadata.ide_type = ANTIGRAVITY`

For this ADR slice, discovery is an acceptance-contract item, not a runtime feature:

1. Donor contract is captured in docs and redacted fixtures.
2. Imported credential projection stores bounded `project_id` when provided.
3. No runtime or model exposure occurs until a separate runtime ADR approves discovery-to-request wiring.

## CLIProxyAPI Hub Mode

CLIProxyAPI may be configured manually as a user-owned external OpenAI-compatible provider:

```toml
model_provider = "cliproxyapi"

[model_providers.cliproxyapi]
name = "CLIProxyAPI"
base_url = "http://127.0.0.1:8080/v1"
env_key = "CLIPROXYAPI_API_KEY"
wire_api = "chat"
```

This is dev-only integration. It must not become required for normal Ontocode provider support.

Do not add a bundled `cliproxyapi` preset until there is a separate compatibility ADR for custom provider presets and schema migration.

## Donor Ideas To Keep

- Loopback Google auth callback shape and scopes.
- shared credential plus virtual project/account entries, but only after multi-project runtime demand exists.
- Refresh dedupe for concurrent sub-agent use of the same refresh token, but only with native refresh implementation.
- Cloud Code Assist project discovery contract, including fallback provisioning shape.
- Provider-specific schema cleanup for Antigravity, but only after native runtime exists and a failing tool-schema fixture proves it is needed.
- Redacted account metadata in status/readiness.

## Donor Ideas To Reject

- Hardcoded OAuth client secrets.
- Importing CLIProxyAPI scheduler, conductor, router, or translator stack.
- Making Ontocode depend on a local proxy process for first-party provider behavior.
- Showing Antigravity models as selectable before runtime works.
- Storing raw donor JSON or raw credential file paths in memory-bank, logs, status, or diagnostics.

## Consequences

Users can bring existing Antigravity OAuth files into Ontocode without adopting CLIProxyAPI as infrastructure.

Sub-agents can later share a refreshed Antigravity account safely through native credential routing instead of each agent racing token refresh.

The first implementation slice stays small: import, redaction, readiness, discovery-contract fixtures, and disabled provider/menu state. Native Antigravity execution remains a separate gated decision.

## Minimal Implementation Order

1. Add a parser fixture with redacted fake Antigravity OAuth fields and project_id.
2. Add `antigravity` provider OAuth import projection through existing auth storage.
3. Add `/login antigravity --import <path>` and provider auth/status rows.
4. Add a discovery-contract test that records required `loadCodeAssist` metadata and `project_id` handling without enabling runtime calls.
5. Add disabled `/model` visibility only if it helps users understand why runtime is unavailable.
6. Stop. Runtime adapter requires a new ADR after endpoint/API compatibility is approved.

## Stop Conditions

- Do not enable any Antigravity runtime path, model catalog entry, or runtime selector until a separate runtime ADR accepts:
  - Cloud Code Assist request contract,
  - tool/request schema compatibility,
  - refresh/error retry behavior,
  - and failure behavior for model-provider mismatch.
- Do not add a provider broker/store/registry or custom auth stack beyond existing owners.
- Do not ship hardcoded client secrets or secret-bearing runtime network calls before approval.

## Verification Gates

- Import tests reject missing refresh material unless explicitly non-refreshable.
- Redaction tests prove secrets and raw paths do not appear in errors/status/debug output.
- Auth storage compatibility tests keep existing auth files loading.
- Discovery fixture/tests show Cloud Code Assist contract fields are preserved as metadata or rejected.
- Status tests show `runtime_unavailable` and `project_discovery_required` states before any Antigravity model/runtime visibility.
- Runtime remains disabled until a native Antigravity execution test exists.
- OntoIndex impact must be run before editing `AuthManager`, `AuthDotJson`, slash auth dispatch, or model-provider auth handoff.
