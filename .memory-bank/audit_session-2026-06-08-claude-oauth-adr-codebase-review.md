# Claude OAuth ADR Codebase Review

Date: 2026-06-08

## Scope

Reviewed current codebase against `ADR_CLAUDE_OAUTH_PROVIDER_REFACTOR.md` using GitNexus and scoped tests.

## Verdict

The full ADR is not complete.

The ADR is explicitly `Challenged`, and the real Claude OAuth import path remains blocked on a real redacted Claude MCP credential sample. The implemented code matches the reduced preparation scope, not the full product goal.

## Findings

- Runtime Claude OAuth import is not wired.
- `parse_claude_oauth_import_sample` is only used by tests and the ignored live-sample validator, not by app-server, MCP runtime, or storage-writing code.
- `mcpServerStatus/list` authenticated-state validation, live MCP server call validation, and real refresh validation from Stage 1 are not implemented.
- `StoredOAuthTokens::from_bearer_token_parts` exists as a readiness helper, but no production call maps `ImportableMcpOAuthCredential` into `save_oauth_tokens`.
- No broad credential broker was introduced, which matches the revised ADR recommendation.
- GitNexus `detect-changes --scope all --repo codex` reported a critical dirty scope: 115 files, 667 symbols, 35 affected processes.

## Done

- Stage 0 evidence gate is documented/tracked as done.
- Claude OAuth parser/report/status boundary exists in `codex-rs/external-agent-migration/src/claude_oauth_import.rs`.
- Status outcomes exist: `Complete`, `Partial`, `NonImportable`, `Empty`, `ConsentRequired`, and `LockedKeychain`.
- Debug output redacts token-bearing fields.
- Synthetic fixture tests exist.
- Ignored live-sample validator exists behind `CLAUDE_OAUTH_REDACTED_SAMPLE`.
- Existing MCP OAuth storage helper exists in `codex-rs/rmcp-client/src/oauth.rs`.
- Provider-selector work was separated into another ADR.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p codex-external-agent-migration`: 35 passed, 1 ignored validator.
- `CARGO_BUILD_JOBS=8 just test -p codex-rmcp-client`: 63 passed.
- GitNexus index was up to date at commit `ad2012d`.

## Remaining Blocker

Stage 1 cannot close until one real redacted Claude MCP connector credential sample validates:

- actual credential shape
- importability into `StoredOAuthTokens`
- local or recoverable refresh behavior
- authenticated MCP server status
- successful MCP server call

## 2026-06-08 Addendum

Commit `e32502e` added runtime wiring from `ExternalAgentConfigService::import` to `import_mcp_oauth_credentials`, `parse_claude_oauth_import_sample`, `StoredOAuthTokens::from_bearer_token_parts`, and `save_oauth_tokens`.

The remaining blocker is now live validation only: no `CLAUDE_OAUTH_REDACTED_SAMPLE` was available in the environment, so the ignored live-sample validator and authenticated MCP server status/call checks still cannot close.
