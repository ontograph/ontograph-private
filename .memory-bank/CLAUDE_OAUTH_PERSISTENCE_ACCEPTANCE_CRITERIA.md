# Claude OAuth Persistence Acceptance Criteria

Date: `2026-06-06`

Source ADR: `ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md`

Related docs:

- `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR.md`
- `CLAUDE_OAUTH_EVIDENCE_GATE.md`
- `CLAUDE_OAUTH_IMPORT_SAMPLE_CONTRACT.md`
- `CLAUDE_OAUTH_LIVE_SAMPLE_RUNBOOK.md`

## Purpose

This document defines the Stage B acceptance contract for wiring imported Claude MCP OAuth credentials into Codex MCP OAuth persistence later.

It does not approve or implement runtime credential persistence. Runtime persistence remains blocked until:

- a real redacted Claude MCP OAuth sample validates the credential shape
- product/security approval allows reading the foreign credential source
- the implementation satisfies this contract

## Storage Target

The only approved first persistence target is the existing Codex MCP OAuth store.

Implementation must use the same public persistence boundary as normal MCP OAuth login:

- `StoredOAuthTokens`
- `OAuthBearerTokenParts`
- `save_oauth_tokens`
- `has_oauth_tokens`
- `delete_oauth_tokens`
- configured `OAuthCredentialsStoreMode`

The importer must not introduce a second Claude-specific credential store and must not add Claude-specific parsing or storage decisions to `AuthManager`.

## Identity Mapping

Each imported Claude MCP OAuth credential must map to exactly one Codex MCP OAuth credential identity.

Required identity fields:

| Imported field | Codex field | Requirement |
| --- | --- | --- |
| `connector_name` | `StoredOAuthTokens.server_name` | Required, non-empty after trimming |
| `server_url` | `StoredOAuthTokens.url` | Required, non-empty, URL-shaped, and exactly the URL used by the configured MCP server |
| `client_id` | `StoredOAuthTokens.client_id` | Required, non-empty |

Acceptance rules:

- The persistence key is the existing MCP OAuth key derived from `(server_name, url)`.
- `connector_name` must not be normalized beyond trimming unless the matching MCP server configuration uses the same normalized value.
- `server_url` must preserve the URL string used by Codex MCP server configuration; the importer must not silently rewrite hosts, paths, schemes, or trailing slashes.
- If the configured MCP server cannot be matched by both name and URL, the credential is `inactive` and must not be persisted.
- If Claude exposes account-level OAuth records without connector-specific name and URL, the record is `non_importable`.

## Duplicate Handling

Duplicate handling must be deterministic and conservative.

Acceptance rules:

- Multiple imported records with the same `(server_name, url)` are rejected as ambiguous unless they are byte-for-byte equivalent after secret redaction and structural normalization.
- Conflicting `client_id`, scope, expiry, account, issuer, or refresh metadata for the same `(server_name, url)` makes the duplicate set `non_importable`.
- A duplicate set must produce a structured rejection reason that contains only non-secret identity and provenance metadata.
- Non-conflicting records for different `(server_name, url)` pairs may be imported independently.

## Existing Credential And Overwrite Policy

Runtime import must never overwrite an existing Codex MCP OAuth credential implicitly.

Acceptance rules:

- Before writing, the importer must check existing credentials with `has_oauth_tokens(server_name, url, store_mode)`.
- If an existing credential is present and overwrite was not explicitly confirmed, the candidate becomes `requires_overwrite_confirmation`.
- Confirmation must be specific to the `(server_name, url)` identity being replaced.
- Bulk confirmation must list every target identity and require an explicit user action before writes begin.
- A rejected or cancelled overwrite must leave the existing credential untouched.
- An approved overwrite must use the same store mode and deletion semantics as normal MCP OAuth replacement.

## Refresh Classification

Every candidate must be classified before persistence.

Refresh classes:

| Class | Meaning | Persistence eligibility |
| --- | --- | --- |
| `locally_refreshable` | Access token, refresh token, client ID, expiry or refresh metadata are sufficient for Codex to refresh through the normal MCP OAuth path | Eligible after validation |
| `externally_refreshable` | Claude appears to own refresh through an opaque store, helper, or account grant that Codex cannot refresh directly | Not eligible for raw-token persistence |
| `access_token_only` | Access token exists but refresh token is absent | Inactive by default; eligible only for explicitly accepted short-lived/manual import UX |
| `non_importable` | Missing token ownership, identity, client ID, or required structural fields | Not eligible |

Acceptance rules:

- `locally_refreshable` requires enough material to build `StoredOAuthTokens` and to refresh after expiry without Claude.
- `externally_refreshable` must not be converted into a long-lived Codex credential unless a separate design defines the external refresh handoff.
- `access_token_only` must be reported separately from parser failure because it may support a later user-confirmed one-shot flow.
- Expired credentials without refresh material are `inactive` and must not be persisted.

## Validation And Inactive State

Imported credentials must not become active merely because parsing succeeded.

Required validation before activation:

- server name and URL match a configured MCP server
- scopes are present or explicitly unknown
- account metadata, if available, is shown to the user
- refresh class is known
- expiry unit and timestamp are valid when present
- user has consented to read and import from the foreign credential source

Inactive states:

| State | Meaning | Recoverability |
| --- | --- | --- |
| `requires_sample_validation` | Real Claude sample has not validated the schema | Recoverable when evidence gate closes |
| `requires_user_consent` | User has not approved reading/importing the foreign store | Recoverable by explicit consent |
| `requires_server_match` | No configured MCP server matches `(server_name, url)` | Recoverable by configuring or selecting the server |
| `requires_overwrite_confirmation` | Existing Codex credential would be replaced | Recoverable by explicit confirmation |
| `requires_keychain_unlock` | Source or target keychain is locked | Recoverable by unlocking/retrying |
| `requires_keychain_configuration` | Required keychain service is unavailable or missing | Recoverable by switching store mode or fixing OS keychain |
| `validation_failed` | Identity, scope, expiry, or refresh validation failed | Recoverable only if the input or config changes |

## Revocation And Delete Semantics

Imported credentials must be removable through the same user-facing semantics as normal MCP OAuth logout.

Acceptance rules:

- Delete must call the existing MCP OAuth deletion path for `(server_name, url)` and configured store mode.
- Delete must attempt to remove both keyring and fallback-file entries according to existing MCP OAuth behavior.
- Delete must report whether a local credential was removed.
- Delete must not claim remote OAuth provider revocation unless a future implementation calls a provider revocation endpoint and verifies success.
- If imported credentials include provenance, deletion must remove or tombstone local provenance without retaining secrets.
- Failed keyring deletion is recoverable and must identify the store class, not the secret value.

## Store Mode And Keyring Fallback

Persistence must respect the configured Codex MCP OAuth credential store mode.

Acceptance rules:

- `Auto` must prefer keyring and use the existing fallback-to-file behavior.
- `File` must write only through the existing fallback file path and permissions behavior.
- `Keyring` must fail as recoverable if keyring write is unavailable; it must not silently write to file.
- Locked keychain, missing keychain, denied access, unavailable DBus secret service, and fallback-file write failures must have distinct recoverable statuses where the platform exposes enough detail.
- Diagnostics may include store mode and store class but must not include token values, serialized credential JSON, or keyring payloads.

## Provenance

Every candidate and persisted result must carry non-secret provenance.

Minimum provenance fields:

- source application, for example `claude-code`
- source store class, for example file, macOS keychain, Windows Credential Manager, Linux Secret Service, or unknown
- source path or lookup descriptor when non-secret
- import timestamp
- connector name
- server URL
- account display metadata when available
- scope list when available
- refresh classification
- validation status

Acceptance rules:

- Provenance must not include access tokens, refresh tokens, ID tokens, authorization codes, or raw serialized secret payloads.
- Provenance must distinguish imported credentials from credentials obtained directly through Codex login.
- Provenance must be available in import reports before persistence and in diagnostics after persistence without exposing secrets.

## User Consent

Reading a foreign credential store and writing imported credentials both require explicit user consent.

Acceptance rules:

- The user must approve reading the Claude credential source before any source credential content is inspected.
- The user must approve writing each importable credential identity before persistence.
- Overwrite consent is separate from read consent.
- The consent prompt/report must show connector name, server URL, source application, source store class, account metadata when available, scopes when available, refresh class, and whether an existing Codex credential would be replaced.
- Consent must not show token values or raw serialized credential content.

## No-Secret Diagnostics

No diagnostics path may expose secrets.

Forbidden in logs, errors, debug output, snapshots, import reports, telemetry, and panic messages:

- access tokens
- refresh tokens
- ID tokens
- authorization codes
- keyring payloads
- raw source credential JSON containing secrets
- serialized `StoredOAuthTokens`

Allowed when non-secret:

- connector name
- server URL
- client ID only when product/security approves it as non-secret for the target provider
- scope names
- expiry timestamp
- source store class
- source path or key descriptor when it does not include secret values
- redacted token presence markers, for example `<redacted>` or `present`

Acceptance rules:

- `Debug` implementations for import structs must redact token-bearing fields.
- Error types must carry structured reason codes and safe identity metadata instead of raw credential values.
- Tests must assert that known token sentinel values do not appear in formatted reports, errors, logs, or snapshots.

## Test Inventory For Later Rust Work

Parser and classification tests:

- valid fixture maps connector name, server URL, client ID, scopes, expiry, and token presence into an import candidate
- missing connector name, server URL, client ID, or access token returns structured non-importable reasons
- account-level or opaque Claude grants are non-importable
- access-token-only records are classified as `access_token_only`
- records with refresh token material are classified as `locally_refreshable` only when identity and client ID are valid
- externally owned or opaque refresh records are classified as `externally_refreshable`
- expired access-token-only records are inactive

Identity and duplicate tests:

- candidate identity matches the existing MCP OAuth `(server_name, url)` key
- duplicate `(server_name, url)` records with conflicting client IDs are rejected
- duplicate `(server_name, url)` records with conflicting scopes, expiry, account, issuer, or refresh metadata are rejected
- different connector names or URLs do not collide
- URL rewriting is not performed silently

Persistence gate tests:

- existing credential produces `requires_overwrite_confirmation`
- cancelled overwrite leaves existing stored credential unchanged
- confirmed overwrite replaces only the selected `(server_name, url)` credential
- candidate with no matching configured MCP server remains inactive and is not persisted
- runtime persistence refuses to run while the live-sample evidence gate is unsatisfied

Store-mode tests:

- `Auto` writes to keyring when available and uses existing fallback behavior when allowed
- `File` writes only to the fallback file
- `Keyring` returns a recoverable keyring status when keyring write fails
- locked keychain and missing keychain map to distinct recoverable statuses where available
- fallback-file write failure is reported without serialized token values

Revocation/delete tests:

- delete removes the imported credential through existing MCP OAuth delete semantics
- delete attempts both keyring and fallback-file cleanup according to store mode
- delete reports local removal without claiming remote provider revocation
- failed keyring delete returns a recoverable no-secret error

Consent and diagnostics tests:

- read consent is required before foreign credential inspection
- write consent is required before persistence
- overwrite consent is required separately from read/write consent
- formatted import reports contain identity, scopes, provenance, and statuses but no tokens
- `Debug` output for candidates and reports redacts token-bearing fields
- logs, errors, and snapshots do not contain token sentinel values

## Exit Criteria

Stage B is complete when:

- this contract is reviewed and accepted
- later runtime persistence work is scoped to satisfy each acceptance section
- missing live Claude credential evidence remains tracked as a separate blocker
- no runtime credential persistence has been implemented as part of Stage B
