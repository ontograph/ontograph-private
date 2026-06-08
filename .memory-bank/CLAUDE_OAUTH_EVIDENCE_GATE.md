# Claude OAuth Evidence Gate

Date: `2026-06-04`

Source tracker: `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR_TRACKING.md`

## Verdict

Blocked on current-machine evidence.

The current machine does not contain a live Claude credential artifact that can be inspected or imported, and the vendored Claude snapshot only provides changelog-level evidence rather than a schema-level credential format.

## Local Evidence

- `~/.claude` exists, but only contains rules and skills content.
- No local `.credentials.json`, `settings.json`, `managed-mcp.json`, or similar Claude runtime credential file was present on this machine.
- No `CLAUDE_*` or `ANTHROPIC_*` environment variables were present in the current session.
- No broader local Claude credential surface was found outside `~/.claude` on this host.
- This Linux host does not expose the macOS keychain tooling or Linux secret-store tools needed to inspect an OS-backed Claude credential store even if one existed.

Local paths checked:

- [`.claude`](/home/evrasyuk/.claude)
- [`lean-ctx.md`](/home/evrasyuk/.claude/rules/lean-ctx.md:1)
- [`SKILL.md`](/home/evrasyuk/.claude/skills/lean-ctx/SKILL.md:1)

## Claude Snapshot Evidence

The vendored Claude snapshot strongly suggests:

- Linux and Windows use `~/.claude/.credentials.json` for at least some auth state.
- macOS uses keychain-backed storage for some login and MCP OAuth state.
- OAuth refresh material exists for MCP connectors.
- `.credentials.json` includes at least `scopes` and `subscriptionType` in some form.

Relevant evidence:

- [CHANGELOG.md](/opt/demodb/_workfolder/ontocode/tmp/claude-code-main/CHANGELOG.md:160)
- [CHANGELOG.md](/opt/demodb/_workfolder/ontocode/tmp/claude-code-main/CHANGELOG.md:615)
- [CHANGELOG.md](/opt/demodb/_workfolder/ontocode/tmp/claude-code-main/CHANGELOG.md:738)
- [CHANGELOG.md](/opt/demodb/_workfolder/ontocode/tmp/claude-code-main/CHANGELOG.md:740)
- [CHANGELOG.md](/opt/demodb/_workfolder/ontocode/tmp/claude-code-main/CHANGELOG.md:1183)
- [CHANGELOG.md](/opt/demodb/_workfolder/ontocode/tmp/claude-code-main/CHANGELOG.md:1901)
- [CHANGELOG.md](/opt/demodb/_workfolder/ontocode/tmp/claude-code-main/CHANGELOG.md:1906)
- [CHANGELOG.md](/opt/demodb/_workfolder/ontocode/tmp/claude-code-main/CHANGELOG.md:1907)
- [CHANGELOG.md](/opt/demodb/_workfolder/ontocode/tmp/claude-code-main/CHANGELOG.md:2040)
- [CHANGELOG.md](/opt/demodb/_workfolder/ontocode/tmp/claude-code-main/CHANGELOG.md:3050)

## Codex MCP OAuth Store Fit

The existing Codex MCP OAuth store is raw-token based and already persists:

- server name
- server URL
- client ID
- access token
- refresh token
- scopes
- expiry metadata

Relevant code:

- [`StoredOAuthTokens`](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:56)
- [`save_oauth_tokens`](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:155)
- [`FallbackTokenEntry`](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:376)

This means direct import is only plausible if Claude stores connector credentials in a similarly raw and attributable form.

## Missing Evidence

The following are still unknown:

- exact `.credentials.json` schema
- whether claude.ai connector tokens are stored as per-server records or as a global account grant
- whether connector records include server name, URL, and client ID
- expiry field names and units
- whether macOS keychain entries contain raw token material or an app-specific envelope
- whether Claude main login credentials and connector credentials share one store or use separate stores

## Reopen Conditions

Stage 1 and Stage 2 should be reopened only after obtaining one sanitized real sample from a machine with:

1. an active Claude login
2. at least one authenticated claude.ai MCP connector

Preferred evidence:

- Linux or Windows: redacted `~/.claude/.credentials.json`
- macOS: redacted keychain export metadata plus the lookup keys used by Claude, without secret values
