---
name: Kimi OAuth Import And Device Flow
description: Decision record for Kimi OAuth import and native device-flow sequencing from CLIProxyAPI donor evidence
type: adr
date: 2026-06-17
status: superseded_for_native_runtime
---

# ADR: Kimi OAuth Import And Device Flow

## 2026-06-19 Revision: External-Only Kimi Runtime

Native Kimi OAuth/device flow and model runtime are rejected for Ontocode core.
Kimi may be used only through a user-managed external OpenAI-compatible API
endpoint or sidecar.

Current policy:

- Ontocode core must not import, persist, refresh, or consume Kimi OAuth
  credentials for model execution.
- Kimi API keys or OAuth refresh belong outside Ontocode, in the external
  endpoint/sidecar configured by the user.
- No Kimi models should appear as native runtime options.
- GPT/Codex remains the native default and must keep working when Kimi
  credentials or sidecars fail.

The donor details below are retained only as historical sidecar evidence.

## Context

`tmp/CLIProxyAPI-main` has a real Kimi OAuth implementation, not just an import shape:

- `internal/auth/kimi/token.go` defines the saved Kimi token fields.
- `internal/auth/kimi/kimi.go` implements OAuth device flow and refresh.
- `sdk/auth/kimi.go` wraps Kimi login as a provider authenticator.
- `internal/runtime/executor/kimi_executor.go` injects Kimi headers and refreshes tokens for runtime calls.
- `internal/api/handlers/management/auth_files.go::RequestKimiToken` exposes Kimi login through CLIProxyAPI management UI.

Ontocode already has native owners for this work:

- Provider OAuth credentials persist through existing login/auth storage.
- `/login`, `/auth`, `/logout`, and `/status` already have provider-aware surfaces.
- Runtime execution belongs in existing model-provider/provider-runtime owners, not in external-agent migration.

Superseded authority, 2026-06-18:

- Codex/OpenAI remains the first-class default and fallback.
- Kimi import is additive and provider-scoped. It must not create a global auth
  mode, replace Codex/OpenAI, or mutate Codex/OpenAI credentials.
- Kimi runtime and device flow stay gated until approved client/runtime evidence
  and tests exist.
- This authority is replaced by the 2026-06-19 revision above and
  [OpenAI-Only Provider Policy Cleanup](audit_session-2026-06-19-openai-only-provider-policy.md).

OntoIndex evidence:

- Existing provider OAuth persistence is covered by `upsert_provider_oauth_credential`, `load_provider_oauth_credential_by_provider_id`, and `provider_oauth_credential_helpers_round_trip_through_auth_storage`.
- Existing provider auth UI routes through `ChatWidget.handle_auth_shell_command` and slash-command tests in `tui/src/chatwidget/tests/slash_commands.rs`.
- App-server external-agent import is not the right owner for first-party Kimi OAuth.

## Superseded Original Decision

Do not make CLIProxyAPI the built-in Kimi provider hub.

Use CLIProxyAPI as donor evidence for:

1. Kimi token import shape.
2. Kimi device-code OAuth flow shape.
3. Kimi refresh requirements.
4. Kimi runtime headers and model behavior, only after a separate runtime gate.

Import-only Kimi OAuth support is already landed. The next implementation slice is
device-flow login only, and only after the observed client id is explicitly
approved.

## Senior Challenge

The proposal is directionally useful but too broad in three places:

1. It treats Kimi device flow as an obvious second step. Narrow it: device flow is blocked until the observed Kimi client id is explicitly approved.
2. It treats `device_id` as import-ready metadata. Narrow it: import may parse `device_id`, but runtime use is blocked unless the existing provider OAuth credential type already has a bounded metadata home.
3. It risks adding Kimi model visibility too early. Narrow it: first slice may add only auth/status visibility, not `/model` entries or model catalogs.

CLIProxyAPI also has a mismatch worth keeping visible: `RequestKimiToken` registers `kimi` sessions directly, while `NormalizeOAuthProvider` does not accept `kimi`. Do not copy that management callback shape into Ontocode.

## Accepted Native Shape

Provider id:

```text
kimi
```

Initial command surface:

```text
/login kimi --import <path>
/auth kimi
/logout kimi
/status kimi
```

Import parser accepts:

```text
access_token
refresh_token
token_type
scope
expired
expires_at
device_id
type
```

Import rejects:

- files whose provider/type is not `kimi`
- files without `access_token`
- files without `refresh_token`
- files with malformed expiry
- files whose `device_id` is required for runtime but cannot be stored in the existing credential shape
- multi-account bundles where account selection is ambiguous
- files that require raw client secrets or proxy-specific runtime config

Persistence:

- Project imported tokens into the existing provider OAuth credential type.
- Preserve `device_id` only if the existing credential type already supports bounded provider metadata; do not add a new metadata store for the import slice.
- Save through existing Ontocode auth storage.
- Do not create a CLIProxyAPI token directory, Kimi-only secret store, or proxy account store.

Readiness:

- `missing_credentials`
- `ready`
- `expired`
- `refresh_failed`
- `runtime_unavailable`

All status and errors must redact access tokens, refresh tokens, raw import paths, keychain paths, and any authorization headers.

## Device Flow Gate

Kimi device flow may be added after import support is verified.

Required shape from CLIProxyAPI:

```text
client_id: donor-observed Kimi Code client id, pending approval
device authorization: https://auth.kimi.com/api/oauth/device_authorization
token endpoint: https://auth.kimi.com/api/oauth/token
API base: https://api.kimi.com/coding
```

Before implementation, approve the compatibility/legal risk of shipping the observed Kimi client id:

```text
17e5f671-d194-4dfb-9706-5516cb48c098
```

If this client id is not approved, device flow stays blocked and import-only remains the supported native path.

Do not add an app-server OAuth callback or background management session for Kimi in this ADR. The TUI can show the verification URL and poll in the login flow when device support is approved.

## Runtime Gate

Do not expose Kimi models in `/model` until native runtime works.

Runtime requires a separate ADR covering:

- Kimi request headers, including device id.
- Model id normalization, including `kimi-` prefix behavior.
- Refresh before expiry with dedupe.
- Tool-call and message normalization fixtures.
- Error and rate-limit behavior.
- Whether Kimi OAuth is first-party enough to ship as a built-in runtime rather than a user-owned custom provider.

## Donor Ideas To Keep

- Kimi token file field names.
- Device-code login UX: show verification URL and user code, optionally open browser.
- Refresh five minutes before expiry.
- Refresh dedupe by refresh token.
- Device id preservation for runtime headers, gated by credential metadata support.

## Donor Ideas To Reject

- Importing CLIProxyAPI scheduler, conductor, router, or proxy account store.
- Making Ontocode depend on a local CLIProxyAPI process for Kimi.
- Bundling a CLIProxyAPI provider preset as the native Kimi path.
- Exposing Kimi models before native runtime is tested.
- Adding app-server management OAuth sessions for Kimi before the TUI/device flow contract exists.
- Adding a custom credential metadata store just for Kimi.
- Storing raw donor JSON, raw credential paths, tokens, or authorization headers in memory-bank, logs, status, or diagnostics.

## Next Implementation Slice

Post-approval device-flow slice:

1. Reuse the existing auth/login/status owner for a login-only Kimi device-flow path.
2. Show the verification URL and user code in the existing TUI login flow.
3. Poll and surface the donor-observed device-flow errors: pending, slow-down,
   denied, expired, and timeout.
4. Keep persistence inside the existing provider OAuth credential shape.

Stop conditions for this slice:

- no app-server OAuth callback or background management session
- no new credential store, provider registry, or auth broker
- no `/model` entries, model catalogs, or runtime execution
- no raw client ids, tokens, headers, or import paths in status/log output

Minimal tests required before runtime enablement:

- device authorization request fixture proves the approved client id and donor
  headers are used
- polling tests cover pending, slow-down, denied, expired, and timeout
- redaction tests prove verification URLs, tokens, and import paths stay out of
  errors and status output
- status tests continue to show Kimi as `runtime_unavailable` after import and
  after any future device-flow login

## Verification Gates

- Import tests reject missing refresh material.
- Redaction tests prove secrets and raw paths do not appear in errors/status/debug output.
- Auth storage compatibility tests keep existing auth files loading.
- Status tests show Kimi as runtime unavailable after import.
- Runtime remains disabled until a native Kimi execution test exists.
- Device flow remains gated until the client id approval is recorded and the
  post-approval device-flow tests above pass.
- OntoIndex impact must be run before editing `AuthManager`, `AuthDotJson`, slash auth dispatch, model-provider auth handoff, or runtime request code.
