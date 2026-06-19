# Claude OAuth Import Sample Contract

Date: `2026-06-04`

Source tracker: `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR_TRACKING.md`

## Purpose

This contract unblocks Stage 1 parser and test work without requiring real Claude secrets in the repository.

Live import validation still requires one redacted real Claude credential sample from a machine with an authenticated claude.ai MCP connector. Until that exists, implementation must stay fixture-driven and must not claim production compatibility with Claude credential storage.

## Senior Decision

Split the Stage 1 blocker into two tracks:

- `internal prep`: define accepted sample shape, redaction rules, parser behavior, mapping to `StoredOAuthTokens`, and test fixtures
- `live validation`: verify the parser against one real redacted Claude connector credential sample

The internal prep track is unblocked now.

The live validation track remains blocked until a real sanitized sample exists.

## Redaction Rules

Real credential samples must never include raw token values in the repo or in issue comments.

Required replacements:

- access tokens: `REDACTED_ACCESS_TOKEN`
- refresh tokens: `REDACTED_REFRESH_TOKEN`
- ID tokens: `REDACTED_ID_TOKEN`
- account identifiers: stable fake IDs such as `acct_redacted_1`
- workspace identifiers: stable fake IDs such as `workspace_redacted_1`
- emails: fake addresses such as `user@example.invalid`

Do not redact structural fields that are required for mapping:

- connector name
- connector URL
- OAuth client ID, if present
- scopes
- expiry field name and unit
- issuer or metadata URL, if present
- record nesting and collection shape

## Minimum Synthetic Fixture Shape

Use this shape for parser-shape tests until a real redacted sample is available:

```json
{
  "version": 1,
  "source": "claude-code",
  "credentials": [
    {
      "kind": "mcp_oauth",
      "connector_name": "slack",
      "server_url": "https://mcp.example.invalid/slack",
      "client_id": "claude-code-client-id",
      "access_token": "REDACTED_ACCESS_TOKEN",
      "refresh_token": "REDACTED_REFRESH_TOKEN",
      "scopes": ["channels:read", "chat:write"],
      "expires_at": 1893456000000,
      "expires_at_unit": "milliseconds_unix_epoch",
      "issuer": "https://auth.example.invalid",
      "auth_server_metadata_url": "https://auth.example.invalid/.well-known/oauth-authorization-server"
    }
  ]
}
```

This is not asserted to be Claude's real schema. It is a contract fixture for testing the internal mapping target and failure modes.

## Mapping Target

The first approved storage target is `StoredOAuthTokens` in `ontocode-rs/rmcp-client/src/oauth.rs`.

Required mapping:

| Sample field | `StoredOAuthTokens` field | Required |
| --- | --- | --- |
| `connector_name` | `server_name` | yes |
| `server_url` | `url` | yes |
| `client_id` | `client_id` | yes |
| `access_token` | `token_response.access_token` | yes |
| `refresh_token` | `token_response.refresh_token` | no, but required for refreshable import |
| `scopes` | `token_response.scopes` | no |
| `expires_at` | `expires_at` | no |

Unsupported or informational fields should not block parsing unless they change token ownership:

- `issuer`
- `auth_server_metadata_url`
- account metadata
- subscription metadata

## Parser Acceptance Rules

The parser should accept only records that can be keyed safely for Codex MCP OAuth storage.

Accept when:

- `kind` is `mcp_oauth`
- `connector_name` is non-empty
- `server_url` is non-empty and parses as a URL
- `client_id` is non-empty
- `access_token` is non-empty
- `expires_at`, if present, has a known unit
- `scopes`, if present, is an array of strings

Reject as non-importable when:

- the record is a global account grant instead of a connector-specific OAuth record
- `server_url` is missing
- `client_id` is missing
- token fields are absent or opaque
- expiry metadata exists but has an unknown unit
- multiple records map to the same `(connector_name, server_url)` key with conflicting token ownership

Rejection should return structured reasons rather than silently skipping records.

## Test Strategy

Stage 1 tests can proceed now with synthetic fixtures.

Required tests:

- valid synthetic fixture maps to `StoredOAuthTokens`
- missing `server_url` returns non-importable
- missing `client_id` returns non-importable
- opaque global grant returns non-importable
- unknown expiry unit returns non-importable
- redacted token placeholders are preserved only inside test values and are never logged
- duplicate connector records with conflicting URLs or client IDs are rejected

Tests must not write to the real user keyring or real home directory. Use temporary directories and file-mode storage only.

## Live Validation Gate

Before marking Stage 1 done, validate against a real redacted sample that preserves schema and non-secret structural values.

Accepted live evidence shape:

- one redacted JSON bundle with capture metadata and the sampled credential
  payload
- at least one `mcp_oauth` record
- preserved connector name, server URL, OAuth client ID, scopes, expiry
  fields, and issuer or auth-server metadata if present
- stable placeholder replacements for access tokens, refresh tokens, ID
  tokens, account IDs, workspace IDs, and emails
- validator output from `validates_redacted_live_sample_from_env`

The live sample must answer:

- whether Claude stores connector credentials as raw OAuth tokens or opaque grants
- where connector name and server URL are stored
- whether OAuth client ID is persisted
- how expiry is represented
- whether refresh token material is present
- whether main Claude login and MCP connector credentials are separate records

If the live sample does not contain enough fields to construct `StoredOAuthTokens`, Stage 1 should close with an explicit non-importable verdict and Stage 2 should design re-auth or externally mediated refresh UX instead of direct token import.

Stop conditions for the live validation gate:

- do not accept samples that omit the connector boundary
- do not accept samples that require raw tokens to prove structure
- do not accept a capture that would force a new broker, registry, or public
  API before the first live sample is understood
- do not accept samples that only prove the main Claude login store without a
  connector-specific record

## Next Engineering Task

Implement a fixture-driven parser module only after agreeing on the crate owner.

Recommended first owner:

- `ontocode-rs/external-agent-migration`

Reason:

- existing Claude config import logic already lives there
- parser tests can stay isolated from app-server and keyring behavior
- output can be an intermediate `ImportableMcpOAuthCredential` value before any storage write path is added

Do not modify `StoredOAuthTokens` for the first parser spike.

## Implementation Status

Implemented in `ontocode-rs/external-agent-migration/src/claude_oauth_import.rs`.

Current scope:

- parses the synthetic fixture shape into `ImportableMcpOAuthCredential`
- returns structured rejection reasons for non-importable records
- reports high-level import status as complete, partial, non-importable, or empty
- provides `OAuthBearerTokenParts` and `StoredOAuthTokens::from_bearer_token_parts` in `ontocode-rs/rmcp-client/src/oauth.rs` for future caller-level storage wiring
- provides an opt-in live-sample validator documented in `CLAUDE_OAUTH_LIVE_SAMPLE_RUNBOOK.md`
- provides `scripts/redact_claude_oauth_sample.py` to turn credential-like JSON into a sanitized sample and shape summary
- keeps storage wiring out of scope until a real redacted Claude credential sample validates the schema
