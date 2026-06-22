---
name: Multi-Provider OAuth Donor Unblock Review
description: Senior review of tmp donor OAuth examples against blocked G1-G4 gates
type: audit_session
date: 2026-06-18
status: accepted
source_plan: MULTI_PROVIDER_OAUTH_FIRST_CLASS_CODEX_PROJECT_PLAN.md
---

# Multi-Provider OAuth Donor Unblock Review

Reviewed donor material under `./tmp` after G1-G4 were blocked.

## Donor Evidence

Useful donors:

- `tmp/CLIProxyAPI-main/sdk/auth/*.go` and `internal/auth/*`: complete example
  flows for Gemini CLI, Kimi device auth, Antigravity loopback OAuth, and
  Claude loopback OAuth.
- `tmp/oh-my-pi-main/packages/ai/src/registry/oauth/*.ts`: concise flow specs
  for Google Gemini CLI, Google Antigravity, Kimi device auth, and Anthropic
  OAuth/refresh.
- `tmp/OmniRoute-main/docs/security/PUBLIC_CREDS.md` and
  `open-sse/utils/publicCreds.ts`: public-client-id handling pattern for
  provider-distributed OAuth clients without storing scanner-matched literals.
- `tmp/openclaw-main/docs/auth-credential-semantics.md`: useful rule that
  OAuth profile material should not be copied by default; provider-owned flows
  may opt in only when refresh-token portability is known safe.

## Gate Updates

G1 Gemini browser/device OAuth remains blocked for built-in login. Donors show
Gemini CLI / Cloud Code Assist OAuth, not official Gemini API OAuth. Treat this
as `gemini-cli` donor/import/runtime evidence, not as proof to make official
`gemini` browser login native.

G2 Kimi/Antigravity is partially unblocked for planning:

- Kimi has enough donor evidence to write a narrow ADR stage for device-code
  auth: client id, device authorization endpoint, token endpoint, polling
  behavior, refresh behavior, and required device headers.
- Antigravity has enough donor evidence to write a narrow ADR stage for
  loopback Google OAuth plus Cloud Code Assist project discovery.
- Runtime enablement still requires separate Ontocode ADR acceptance and
  compatibility tests before changing selectable runtime providers.

G3 Claude is partially unblocked for planning:

- Donors show Anthropic OAuth PKCE, callback port/path, token endpoint, scopes,
  refresh behavior, and bootstrap identity lookup.
- Runtime enablement still requires a sanitized real credential sample or a
  live-validation artifact accepted into this repo. Fixture-only evidence is
  not enough.

G4 public API/schema/catalog migration remains blocked. Donors have large
  broker/catalog/plugin stacks, but copying those would create a parallel
  provider architecture. Keep public surface work behind
  `ADR_PUBLIC_ADAPTER_SDK_SCHEMA_MIGRATIONS` and its compatibility tests.

## Senior Decision

Do not copy donor auth brokers or runtime registries.

Next useful work is three small ADR-bound slices:

1. Kimi device-code OAuth implementation plan under existing provider-auth and
   login owners.
2. Antigravity loopback OAuth implementation plan under existing external-agent
   import/provider-auth owners.
3. Claude live-validation acceptance plan that defines the sanitized evidence
   artifact needed to move beyond fixture tests.

Codex/OpenAI remains the default and fallback. No donor path may change global
auth mode or make credentials exclusive.
