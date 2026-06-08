# Claude OAuth Live Sample Runbook

Date: `2026-06-04`

Source tracker: `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR_TRACKING.md`

## Purpose

This runbook turns the remaining Stage 1 blocker into a repeatable handoff.

T2 is not blocked on engineering capacity anymore. It is blocked on one redacted real Claude MCP connector credential sample that preserves schema and non-secret structural fields.

## Collection Rules

Do not commit or paste raw credential values.

Replace secret values before moving the sample into the repo or issue tracker:

- access tokens: `REDACTED_ACCESS_TOKEN`
- refresh tokens: `REDACTED_REFRESH_TOKEN`
- ID tokens: `REDACTED_ID_TOKEN`
- account IDs: `acct_redacted_1`
- workspace IDs: `workspace_redacted_1`
- emails: `user@example.invalid`

Keep structural fields intact:

- record nesting
- connector name
- connector URL
- OAuth client ID
- scopes
- expiry field name and unit
- issuer or metadata URLs

## Where To Look

Check these locations on a machine where Claude has an authenticated MCP connector:

- `~/.claude/.credentials.json`
- `~/.claude.json`
- `~/.claude/settings.json`
- `~/.claude/managed-mcp.json`
- platform keychain or credential manager entries named for Claude, Anthropic, or MCP connectors

If credentials are stored only in an OS keychain, export only field names and redacted values.

## Validation Command

Redact a credential-like JSON export before validation:

```bash
python3 scripts/redact_claude_oauth_sample.py \
  /path/to/raw-claude-credential.json \
  --output /tmp/redacted-claude-oauth.json
```

Review `/tmp/redacted-claude-oauth.json` before sharing it. It must contain no raw tokens, cookies, emails, account IDs, or workspace IDs.

The helper intentionally preserves `client_id`, connector names, connector URLs, scopes, and expiry field names because those fields are required to prove the import mapping.

Then run:

```bash
CLAUDE_OAUTH_REDACTED_SAMPLE=/tmp/redacted-claude-oauth.json \
  just test -p codex-external-agent-migration \
  validates_redacted_live_sample_from_env \
  --run-ignored ignored-only \
  --no-capture
```

Expected successful output includes:

- `status=Complete` or `status=Partial`
- `importable_credentials` greater than `0`
- `refreshable_credentials` greater than `0` if refresh tokens are present

If status is `NonImportable`, Stage 1 should close with a non-importable verdict and the next implementation should use re-auth or externally mediated refresh instead of direct token import.

## Acceptance Criteria

The sample unblocks T2 only if it answers:

- whether connector credentials are raw OAuth tokens or opaque Claude grants
- where connector name and server URL are stored
- whether OAuth client ID is persisted
- how expiry is represented
- whether refresh token material is present
- whether main Claude login and MCP connector credentials are separate records

## Current Validator

The opt-in validator lives in `codex-rs/external-agent-migration/src/claude_oauth_import.rs` as:

- `validates_redacted_live_sample_from_env`

The validator uses the same parser as production import prep and redacts token fields from debug output.
