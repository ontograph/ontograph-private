---
name: Provider Auth Slash Command Menus ADR
description: UX and command contract for provider-aware /login, /auth, /logout, and /status menus
type: adr
date: 2026-06-17
status: proposed
---

# ADR: Provider Auth Slash Command Menus

## Status

Superseded for non-OpenAI model-provider OAuth menus.

Current authority, 2026-06-19:

- `/login`, `/logout`, and native account status remain OpenAI/Codex-first.
- Non-OpenAI model providers must not gain native OAuth login menus.
- External providers may surface only configuration/status for their external
  API endpoint; provider OAuth remains outside Ontocode.
- This ADR remains historical UX context, but no longer authorizes Gemini,
  Claude, Kimi, Antigravity, or other non-OpenAI native auth menu work.

No implementation is authorized by this ADR until a dispatch card names the exact
TUI and auth-owner symbols to change and runs OntoIndex impact first.

Review challenge 2026-06-17:

- Current `/logout` is global Codex/OpenAI account logout. Provider-scoped
  logout must not reuse it.
- Current `/status` has no bounded provider-auth status row owner. Add one
  before rendering provider auth details in the TUI.
- Current `/model` provider groups are model-picker data, not a general auth
  provider registry. Auth menus need their own small action-availability model
  backed by existing provider ids and auth owners.
- Current onboarding/app-server login is OpenAI-shaped. `/login gemini` must not
  imply browser OAuth until the Google OAuth client gate is resolved.

## Context

Ontocode now has provider-neutral auth plumbing:

- `ontocode-provider-auth` owns canonical provider OAuth credential contracts.
- `ontocode-login` owns auth persistence through the existing auth storage path.
- `model-provider` consumes provider auth through the existing provider auth path.
- `/model` already groups provider catalogs and can show disabled readiness text.

Gemini OAuth is partly available:

- normal `gemini` can use bearer auth when `GEMINI_API_KEY` is absent.
- user-supplied Google ADC or desktop OAuth JSON import is supported.
- `GEMINI_API_KEY` remains first precedence for `gemini`.
- bundled browser Gemini OAuth remains blocked until approved Google OAuth client
  metadata exists.
- `gemini-cli` Cloud Code Assist runtime remains blocked until product/API
  approval exists.

The current user problem is discoverability. Users should not need to remember
provider-specific auth commands. The slash command UX should present the same
provider picker for login, auth management, logout, and status.

Superseded authority, 2026-06-18:

- Codex/OpenAI remains the first-class default and must appear first in auth
  menus.
- Provider OAuth is additive and provider-scoped. Gemini, Claude, Copilot, Kimi,
  Antigravity, or future provider failures must not break Codex/OpenAI login,
  API-key use, startup, chat, or account status.
- `/logout` without a provider remains global Codex/OpenAI logout. Provider
  credential removal belongs to `/auth <provider> remove` unless a later
  compatibility-tested ADR makes `/logout <provider>` a strict alias.
- This authority is replaced by the 2026-06-19 revision above and
  [OpenAI-Only Provider Policy Cleanup](audit_session-2026-06-19-openai-only-provider-policy.md).

## Superseded Original Decision

Add provider-aware slash command menus for:

- `/login`
- `/auth`
- `/logout`
- `/status`

Each command opens a provider picker when no provider argument is supplied.
Direct provider arguments remain supported for scriptability.

Provider choices are sourced from existing canonical provider ids and a small
auth-menu action-availability view. That view may read existing model-provider
and provider-auth readiness data, but it is not a new provider registry and must
not reuse `/model` catalog groups as the source of truth.

Minimum provider set for the first implementation:

| Provider id | Display label | Initial actions |
| --- | --- | --- |
| `openai` | Codex / OpenAI | login, status, global logout, auth details |
| `gemini` | Gemini | import OAuth JSON, status, logout OAuth, auth details |
| `gemini-cli` | Gemini CLI | status only while runtime/login gates remain blocked |
| `claude` | Claude | status/auth details if an existing auth source exists |
| `copilot` | GitHub Copilot | status/auth details if an existing auth source exists |

The menu must show unsupported or blocked actions as disabled rows with one-line
reasons. It must not hide unavailable providers silently.

The Codex/OpenAI provider id is `openai`. Use `Codex / OpenAI` only as display
copy; do not invent `codex` as a new persisted id.

## Command Contract

### `/login`

`/login` opens the provider picker:

```text
Login

> Codex / OpenAI
  Gemini
  Gemini CLI
  Claude
  GitHub Copilot
```

Selecting a provider opens the smallest provider-specific login action menu.

For `codex`:

```text
Codex / OpenAI login

> Browser login
  Paste API key
  Back
```

For `gemini`:

```text
Gemini login

> Import Google OAuth JSON
  Use GEMINI_API_KEY
  Browser login unavailable - approved Google OAuth client required
  Back
```

For `gemini-cli`:

```text
Gemini CLI login

> View status
  Browser login unavailable - approved Google OAuth client required
  Runtime unavailable - Cloud Code Assist approval required
  Back
```

Direct forms:

```text
/login codex
/login gemini
/login gemini --import /path/to/oauth.json
```

`/login gemini` without flags opens the Gemini login action menu. It must not
start browser OAuth while the Google OAuth client gate remains unresolved.

### `/auth`

`/auth` opens the provider picker and then an auth-management menu:

```text
Gemini auth

> Status
  Import Google OAuth JSON
  Remove OAuth credential
  Show precedence
  Back
```

Direct forms:

```text
/auth gemini
/auth gemini status
/auth gemini import /path/to/oauth.json
/auth gemini precedence
```

`/auth` is the power-user surface. `/login` should stay friendlier and only show
actions needed to get a provider working.

The first implementation may render the `/auth` menu without enabling mutation.
Import and removal actions become enabled only after their owner APIs exist.

### `/logout`

`/logout` currently means global Codex/OpenAI account logout. This ADR does not
change that behavior until provider-scoped credential deletion exists.

New behavior must use one of these safe forms:

- `/logout` keeps the existing global logout path.
- `/logout provider` is disabled with an explanation until provider-scoped
  credential deletion exists.
- `/auth <provider> remove` is the preferred future provider-scoped removal
  surface once the delete API lands.

Future provider-scoped removal opens the provider picker and then a confirmation
menu scoped to that provider.

For `gemini`:

```text
Remove Gemini OAuth credential?

Account: user@example.com
This will not change GEMINI_API_KEY.

> Remove OAuth credential
  Cancel
```

Direct forms:

```text
/logout codex
/logout gemini
```

Until provider-scoped deletion exists, direct provider forms must not call the
existing global app-server logout. They should show:

```text
Provider-scoped logout is not available yet.
Use /logout for global Codex/OpenAI logout, or /auth gemini remove after the
provider credential removal API lands.
```

If a future provider has multiple stored accounts, show an account picker before
the confirmation. Do not add multi-account management beyond selecting the
account to remove.

### `/status`

`/status` without arguments keeps the existing general status behavior. It may
include an Auth section only after a bounded provider-auth status row owner
exists.

```text
Auth

Codex / OpenAI   ready
Gemini           ready, API key active, OAuth fallback configured
Gemini CLI       unavailable, runtime blocked
Claude           not configured
GitHub Copilot   not configured
```

`/status auth` opens the provider picker.

Direct forms:

```text
/status auth
/status auth gemini
/status gemini
```

Provider-specific status must show auth precedence without secrets:

```text
Gemini auth

API key: configured
OAuth: user@example.com, ready
Active: API key

API key takes precedence. Remove GEMINI_API_KEY to use OAuth.
```

No token values, raw credential file paths, keychain paths, cookies, or
authorization headers may appear in status output.

Status data must come from a redacted `ProviderAuthStatusRow`-style owner before
the TUI renders it. The TUI must not inspect raw OAuth credentials directly.

## UX Rules

- Use one shared provider picker component for all four commands, but keep
  command-specific action lists.
- Keep provider labels human-readable, but persist canonical provider ids.
- Show disabled actions in place with short reasons.
- Keep the happy path under three steps:
  - `/login`
  - choose provider
  - choose action or provide file path
- Do not make users choose between `gemini` and `gemini-cli` without context:
  - `Gemini` means normal Generative Language API auth.
  - `Gemini CLI` means the separate Cloud Code Assist lane and remains gated.
- Never display raw secret material.
- Never silently change provider precedence.
- Never make `gemini-cli` selectable as a runtime provider from auth menus while
  the runtime gate remains blocked.
- Never route provider-scoped logout through the current global logout path.

## Architecture Rules

- Do not add a new provider registry.
- Do not add a new auth store.
- Do not add a Gemini-only auth manager.
- Do not copy donor OAuth client secrets or donor token-file layout.
- Reuse existing provider catalog/readiness data to populate provider choices.
- Reuse `ontocode-login` for persistence operations.
- Reuse `ontocode-provider-auth` for provider OAuth credential shape and
  redacted summaries.
- Reuse `model-provider` readiness/auth resolution instead of duplicating
  provider auth checks in the TUI.
- Browser Gemini OAuth remains disabled until the OAuth client gate is resolved.
- Cloud Code Assist remains disabled until the runtime endpoint gate is resolved.
- Provider-scoped credential removal requires a narrow delete owner/API before
  `/logout <provider>` or `/auth <provider> remove` can mutate storage.
- Provider-auth status rendering requires a bounded redacted status row owner
  before `/status` can show provider auth details.

## Proposed Implementation Slices

### S0: Read-Only Menu Contract And Snapshots

Scope:

- TUI slash command routing and popup/menu snapshot tests only.
- No auth storage writes.
- No provider runtime changes.
- No app-server API changes.
- No provider-scoped logout mutation.

Acceptance:

- `/login`, `/auth`, `/logout`, and `/status auth` open provider menus.
- Direct provider arguments route to the same provider-specific menu.
- Disabled Gemini browser login and `gemini-cli` runtime reasons render.
- `/logout gemini` is disabled and does not emit the existing global logout
  event.
- Snapshot coverage proves the provider menu shape.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just fmt`

### S1: Provider Auth Status Row Owner

Scope:

- Add the smallest bounded redacted provider auth status row owner.
- The owner may live in `ontocode-provider-auth` or `model-provider`, but the
  dispatch card must justify the final home with OntoIndex impact.
- TUI consumes only the redacted status rows.

Acceptance:

- Rows can represent `ready`, `not_configured`, `api_key_active`,
  `oauth_fallback_configured`, `blocked`, and `error_redacted`.
- Gemini status can express `GEMINI_API_KEY` precedence without token values or
  raw paths.
- `gemini-cli` status can express the runtime/login gates without suggesting it
  is executable.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just fmt`

### S2: Gemini OAuth Import Action

Scope:

- Wire the Gemini menu import action to the existing user-supplied Google OAuth
  JSON import path.
- Persist only through existing login/provider-auth owners.
- Do not add browser login.
- Do not add new app-server public API unless a separate compatibility ADR
  approves it.

Acceptance:

- `/login gemini --import <path>` imports a normal `gemini` OAuth credential.
- `/auth gemini import <path>` does the same.
- Errors are redacted.
- `GEMINI_API_KEY` precedence is shown after import when applicable.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-migration`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just fmt`

### S3: Provider Status Rows

Scope:

- Add bounded provider auth status rows to `/status` and provider-specific
  status views.
- Use existing redacted readiness/auth summary surfaces.

Acceptance:

- `/status` includes concise auth rows.
- `/status auth gemini` shows API-key precedence and OAuth readiness.
- No secret, token, raw path, keychain path, cookie, or authorization header is
  rendered.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 just fmt`

### S4: Provider Credential Removal API

Scope:

- Add a narrow provider OAuth credential delete owner/API.
- It must remove only the selected provider credential.
- It must not call the existing global app-server logout path.

Acceptance:

- Removing Gemini OAuth does not remove Codex/OpenAI auth.
- Removing Gemini OAuth does not mutate `GEMINI_API_KEY`.
- Multiple credentials, if present, require an explicit credential selection.
- Removal diagnostics are redacted.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-provider-auth`
- `CARGO_BUILD_JOBS=8 just fmt`

### S5: Logout Menus

Scope:

- Wire provider-scoped logout confirmation to the provider credential removal
  API from S4.
- Add account picker only if multiple existing credentials are present.

Acceptance:

- `/logout` opens provider picker.
- `/logout gemini` can remove Gemini OAuth without touching Codex/OpenAI auth or
  `GEMINI_API_KEY`.
- Confirmation text names only redacted account metadata.

Verification:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-login`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just fmt`

## Non-Goals

- No bundled Gemini browser OAuth in this ADR.
- No Cloud Code Assist runtime adapter in this ADR.
- No new public app-server API in this ADR.
- No multi-account scheduler UI in this ADR.
- No new config file format in this ADR.
- No migration of existing `/model` behavior beyond linking to auth readiness
  text where it already exists.

## Open Questions

- Whether `/status gemini` should be accepted as an alias for
  `/status auth gemini`; this ADR recommends accepting it because it is shorter.
- Whether Claude and Copilot should expose login actions immediately or only
  status/auth details until their existing flows are verified against this menu
  contract.

## Safety Checklist

- OntoIndex impact must run before editing each TUI command symbol or auth
  manager symbol.
- Any status or error string that includes provider auth data must have redaction
  tests.
- Any UI-visible change must include `insta` snapshot coverage.
- If `ConfigToml` or app-server API payloads change, stop and create a separate
  compatibility ADR first.
