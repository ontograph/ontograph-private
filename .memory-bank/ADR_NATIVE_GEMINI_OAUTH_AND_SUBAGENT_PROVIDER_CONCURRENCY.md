# ADR: Native Gemini OAuth Login And Sub-Agent Provider Concurrency

## Status

Superseded for model runtime by the 2026-06-19 OpenAI-only native auth policy.
Historical donor evidence remains useful for external sidecar design.

## 2026-06-19 Revision: No Native Gemini Model OAuth

Native Gemini browser/device OAuth and provider-aware sub-agent routing are no
longer accepted implementation goals for Ontocode core.

Current policy:

- OpenAI/Codex is the only native OAuth-backed model provider.
- Gemini model use must go through either the existing API-key/custom-provider
  path or a user-owned OpenAI-compatible external endpoint/sidecar.
- Any Gemini OAuth, Google account selection, Cloud Code Assist discovery,
  refresh, and Gemini protocol translation belongs inside that external
  endpoint or sidecar, not in Ontocode model runtime.
- GPT/Codex threads and sub-agents must never depend on Gemini credentials.

Rejected from this ADR:

- `/login gemini` as a native browser/device OAuth flow for model runtime
- native Gemini OAuth refresh in Ontocode core
- mixed GPT/Gemini native OAuth sub-agent routing
- Gemini model catalog visibility that implies native OAuth execution

The original donor notes below are retained only as historical evidence for
building or configuring an external sidecar.

## Context

Users need Gemini OAuth to work without manually importing fragile token-cache files. The current `/login gemini --import <path>` path accepts Google ADC/desktop OAuth-style JSON with `client_id`, `access_token`, and refresh data, but Gemini CLI cache files such as `~/.gemini/oauth_creds.json` may contain only token fields like `access_token`, `refresh_token`, `expiry_date`, `scope`, and `token_type`. Those token-cache files are not self-contained unless Ontocode also owns the matching OAuth client metadata and refresh behavior.

The donor project at `tmp/CLIProxyAPI-main` does not treat Gemini CLI token caches as complete credentials. It runs Gemini OAuth, uses Gemini CLI OAuth client metadata, stores token data under a `token` object, enriches storage with `client_id`, `client_secret`, `token_uri`, scopes, email, and project metadata, and refreshes access tokens through Google's token endpoint.

The Gemini CLI donor at `tmp/gemini-cli-main` is more useful than CLIProxyAPI for UX and boundary decisions:

- `packages/core/src/code_assist/oauth2.ts` defines the installed-app OAuth client id/secret and comments that the client secret is embedded because installed-app client secrets are not treated as real secrets.
- `packages/core/src/code_assist/oauth2.ts` first loads cached credentials, verifies them with `getAccessToken()` and `getTokenInfo()`, and only then starts interactive login.
- `packages/core/src/code_assist/oauth2.ts` supports two interactive paths: browser callback on loopback and a manual user-code path when browser launch is suppressed.
- The manual user-code path is not a standards OAuth device authorization flow. It opens a Google authorization URL with a fixed redirect URI (`https://codeassist.google.com/authcode`) and asks the user to paste the authorization code.
- `packages/core/src/code_assist/oauth-credential-storage.ts` stores OAuth credentials through hybrid/keychain-backed storage and can migrate the old `~/.gemini/oauth_creds.json` file.
- `packages/cli/src/ui/auth/AuthDialog.tsx` presents one clear auth picker: Sign in with Google, compute ADC when applicable, Gemini API key, or Vertex AI.
- `docs/get-started/authentication.mdx` documents that browser Google sign-in is the recommended local path, while headless mode should prefer API key or Vertex/ADC.

Users also need to work with GPT/Codex, Gemini, Claude, Copilot, and other providers at the same time in sub-agents. That means provider selection must be per task/agent, not a single global UI mode.

Superseded authority, 2026-06-18:

- Codex/OpenAI remains the first-class default and fallback.
- Gemini OAuth is additive and provider-scoped, not an app-wide exclusive mode.
- Stored provider OAuth credentials can coexist. Runtime still uses one provider
  per thread, turn, or sub-agent unless a future routing layer explicitly
  selects otherwise.
- This authority is replaced by the 2026-06-19 revision above and
  [OpenAI-Only Provider Policy Cleanup](audit_session-2026-06-19-openai-only-provider-policy.md).

## Superseded Original Decision

Implement native Gemini OAuth as the preferred `/login gemini` path, but do not claim standards device-flow support until the exact Google endpoint and client contract are verified.

Supported login methods:

- Browser callback flow: `/login gemini`
- Manual user-code fallback: `/login gemini --code`
- Device flow: `/login gemini --device`, only if a real OAuth device authorization endpoint is confirmed
- Existing import remains as fallback: `/login gemini --import <path>`

Gemini OAuth ownership belongs in the existing login/provider-auth boundary, not in a new provider stack. Persisted credentials must use the existing provider OAuth credential storage and routing summaries.

Sub-agents must carry an explicit provider/model/account selection through the existing role/config snapshot path when the user chooses one. The main session provider must not silently override sub-agent provider choices.

## Scope

In scope:

- Native Gemini browser OAuth login.
- Native Gemini manual user-code login when browser callback is unavailable.
- Native Gemini device authorization login only behind endpoint verification.
- Token refresh metadata persistence for Gemini OAuth.
- Redacted provider auth status for Gemini credentials.
- Provider-aware sub-agent role/config selection for concurrent GPT/Gemini/other work.
- Tests that secrets, authorization headers, cookies, keychain paths, and raw token values do not appear in UI/status output.

Out of scope for the first implementation:

- Gemini project onboarding and Cloud AI API enablement flows.
- Multi-account picker UI.
- Auto-import of every Gemini CLI cache variant.
- New provider registry, new credential store, or proxy-style runtime.
- Standards device-code login if Google does not support it for the chosen Gemini client.

## User Experience

`/login` opens the provider menu:

```text
Codex / OpenAI
Gemini
Claude
GitHub Copilot
```

`/login gemini` starts browser OAuth and shows:

```text
Opening Gemini login in your browser...
```

If a browser cannot be opened, the UI offers:

```text
Use /login gemini --code
```

`/login gemini --code` prints the authorization URL and asks the user to paste the authorization code. This mirrors Gemini CLI's `NO_BROWSER` path without inventing a new device-flow abstraction.

`/login gemini --device` may later show a short code and verification URL, then poll until success, expiry, or cancellation. It must stay hidden or return a clear unsupported message until a real device endpoint is confirmed.

`/status auth gemini` shows only redacted status:

```text
Gemini (gemini) - OAuth configured, expires <relative time>, account <redacted label>
```

`/auth gemini remove` removes the selected Gemini OAuth credential. Until account picker exists, it removes only when exactly one Gemini OAuth credential exists; if multiple exist, it refuses to guess.

Sub-agent creation should use the existing `agent_type` role config path first. A Gemini role can specify:

```text
model = "gemini-2.5-pro"
model_provider = "gemini"
```

Then the current spawn path can select it with `agent_type`. A Codex/OpenAI role can do the same with `model_provider = "openai"` and `model = "gpt-5.4-mini"`.

The TUI may later expose this through `/agent` or `/subagents`, but the first implementation should wire the existing role/config path rather than add a parallel management surface.

## Architecture

Use existing owners:

- `ontocode-rs/login`: Gemini OAuth flow, token exchange, refresh metadata, persistence adapter.
- `ontocode-rs/model-provider`: provider capability/routing consumption only.
- `ontocode-rs/tui`: slash command UX and redacted status messages only.
- Existing multi-agent/sub-agent launch path: role config, effective config snapshots, provider/model/account overrides.

Do not add:

- A second OAuth token store.
- A second provider registry.
- A proxy runtime.
- A separate Gemini CLI compatibility service.

## Challenge Review

The first ADR version overreached in three places:

- It treated device flow as required. Gemini CLI's donor code does not show a device authorization grant; it shows a manual authorization-code paste flow. Calling that `--device` would be misleading.
- It risked turning token import into a compatibility project. The lazy path is native login plus one clear cache-shape rejection message.
- It implied sub-agent provider concurrency might need new UI. It should first flow through existing sub-agent launch metadata and only add UI once the backend contract exists.

OntoIndex review added these constraints:

- `ontocode-rs/core/src/tools/handlers/multi_agents.rs` already applies role config during `spawn_agent`; `multi_agents_tests.rs::install_role_with_model_override` proves a role can set `model_provider = "ollama"` and a model override, then `spawn_agent` selects it via `agent_type`.
- Because provider override already exists through role config and config snapshots, this ADR must not introduce a new spawn-provider API as the first step. Add one only if the existing `agent_type` contract cannot express the UX after Gemini runtime credentials are wired.
- Existing provider OAuth credential helpers already exist around auth storage: `upsert_provider_oauth_credential`, `load_provider_oauth_credential_by_provider_id`, and `remove_provider_oauth_credential`. Native Gemini OAuth must use these helpers rather than adding another persistence layer.
- Existing import parsing allows a missing refresh token as a non-refreshable imported credential. That is acceptable for import compatibility, but native `/login gemini` should require refresh metadata or explicitly mark the credential non-refreshable and avoid claiming durable login.
- The current `/login gemini --import` path is a TUI command over external-agent migration parsing. Native OAuth should live in `ontocode-rs/login`; TUI should remain command/menu/status glue.

Accepted narrowing:

- `/login gemini` browser callback is the first implementation target.
- `/login gemini --code` is the headless/manual fallback target.
- `/login gemini --device` is documented as conditional, not promised.
- Sub-agent concurrency is a provider/model/account role/config contract, not a new scheduler, provider registry, or spawn API unless existing roles prove insufficient.

## Gemini CLI Donor Examples

Browser callback pattern from `packages/core/src/code_assist/oauth2.ts`:

```text
Generate auth URL with access_type=offline, cloud-platform/userinfo scopes,
listen on http://127.0.0.1:<port>/oauth2callback, validate state,
exchange code for tokens, then cache credentials.
```

Manual user-code fallback from `packages/core/src/code_assist/oauth2.ts`:

```text
When browser launch is suppressed and the session is interactive:
show an auth URL, ask "Enter the authorization code:", exchange that code
with PKCE verifier, then cache credentials.
```

Credential cache behavior from `packages/core/src/code_assist/oauth-credential-storage.ts`:

```text
Load current credentials from keychain/hybrid storage.
If absent, migrate old ~/.gemini/oauth_creds.json.
When saving new credentials, preserve an existing refresh token if Google
returns only a new access token.
```

Auth picker behavior from `packages/cli/src/ui/auth/AuthDialog.tsx`:

```text
Prefer Sign in with Google by default.
Show compute ADC only when environment enables it.
Keep Gemini API key and Vertex AI as separate auth choices.
Clear cached credentials before switching auth type.
```

Authentication guidance from `docs/get-started/authentication.mdx`:

```text
Local users should use Sign in with Google.
Headless users should prefer Gemini API key or Vertex/ADC.
Workspace accounts may require a Google Cloud project.
```

## Credential Shape

Persist a provider OAuth credential with:

- `provider_id = "gemini"`
- `source_kind = ExternalImport` or first-party Gemini OAuth equivalent
- `access_token`
- `refresh_token`
- `client_id`
- `token_endpoint = "https://oauth2.googleapis.com/token"`
- `scopes`
- `expires_at`
- optional redacted account label
- optional project id when known
- provenance such as `gemini-browser-oauth`, `gemini-user-code-oauth`, or `gemini-device-oauth`

Secrets must remain secret-bearing storage fields and must not be included in routing summaries, debug output, status lines, logs, or test failure messages.

## Device Flow

Device login, if implemented, must use the same credential persistence as browser login.

Required behavior:

- Start device authorization.
- Show verification URL and user code.
- Poll with the provider's interval.
- Stop on success, expiration, denial, or cancellation.
- Store the returned token data with refresh metadata.
- Show only redacted completion output.

If Gemini/Google does not provide a supported device endpoint for the chosen client, the command must fail clearly and suggest browser login.

Manual user-code login is separate from device login:

- It uses an authorization URL, PKCE verifier, and pasted authorization code.
- It does not poll a device endpoint.
- It should be exposed as `/login gemini --code`, not silently aliased to `--device`.

## Sub-Agent Provider Concurrency

The system must allow simultaneous active agents with different providers:

- Main session can use Codex/GPT.
- Sub-agent A can use Gemini.
- Sub-agent B can use Claude or another configured provider.

Each agent request resolves credentials using that agent's provider/model/account override first. Global model/provider config is only a default.

Provider choice must be recorded in agent metadata/status so the user can see which agent is using which provider without exposing credentials.

First implementation target:

- Define provider-specific agent roles, for example `gemini-reviewer` with `model_provider = "gemini"` and `model = "gemini-2.5-pro"`.
- Spawn those roles with the existing `agent_type` argument.
- Verify the child thread config snapshot keeps the selected provider/model while the parent remains on its own provider.
- Avoid adding `provider=` to `spawn_agent` until there is a concrete UX gap that role config cannot cover.

## Failure Modes

Bad import file:

```text
This looks like a Gemini CLI token cache, not a complete OAuth client file. Use /login gemini or /login gemini --code.
```

Missing refresh metadata:

```text
Gemini OAuth credentials are missing refresh metadata. Log in again with /login gemini.
```

Multiple Gemini credentials on remove:

```text
Multiple Gemini OAuth credentials exist. Account-specific removal is not implemented yet.
```

Device flow unsupported:

```text
Gemini device login is not available for this OAuth client. Use /login gemini.
```

Browser suppressed in an interactive terminal:

```text
Open this Gemini authorization URL, then paste the authorization code.
```

Browser suppressed in a non-interactive terminal:

```text
Gemini login needs an interactive terminal. Use /login gemini in a terminal, Gemini API key, or Vertex/ADC.
```

## Tests

Required focused tests:

- `/login gemini` starts browser OAuth path without printing secrets.
- `/login gemini --code` renders auth URL, accepts mocked code, and stores credential on mocked success.
- `/login gemini --device` either renders code/URL and stores credential on mocked success or returns the unsupported-device message, depending on verified endpoint support.
- Device flow expiry/denial produces a clear message.
- Manual user-code timeout and bad-code exchange produce clear messages.
- Imported Gemini CLI token-cache shape is rejected with the specific UX message.
- Gemini OAuth refresh metadata is persisted.
- Refresh-token preservation is tested when a token refresh returns only a new access token.
- `/status auth gemini` redacts access token, refresh token, client secret, authorization headers, and local key paths.
- `/auth gemini remove` removes a single Gemini credential and refuses multiple credentials.
- Two sub-agents launched with different `agent_type` role configs keep separate provider/model selections.

## Rollout

1. Keep `/login gemini --import <path>` as fallback.
2. Add browser OAuth behind `/login gemini`.
3. Add manual user-code fallback behind `/login gemini --code`.
4. Add `/login gemini --device` only after endpoint verification.
5. Wire sub-agent provider/model/account override through the existing role/config launch path.
6. Add account picker only after multiple stored Gemini credentials are common enough to justify UI.

## Consequences

Pros:

- Users can log in without hand-editing OAuth files.
- Gemini token refresh becomes owned by Ontocode, not by imported cache quirks.
- GPT, Gemini, Claude, and other providers can run concurrently in sub-agents.

Cons:

- Ontocode must own Gemini OAuth client metadata and refresh behavior.
- Device flow support depends on Google's supported OAuth endpoint behavior and is not guaranteed by Gemini CLI donor evidence.
- Multi-account UX remains intentionally minimal until needed.
