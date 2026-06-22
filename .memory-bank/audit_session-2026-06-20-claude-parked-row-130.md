# Claude Parked Row 130 Review

Date: 2026-06-20

## Decision

Row 130 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 130 allows MCP resource caps/redaction only if added to the current manager.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 130 proposes adding an MCP server custom source-root environment variable under `codex-mcp` with an env validation test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- The parked ADR challenge says rows 122, 123, 128-130, and 145-147 must not become an MCP browser, source explorer, teaching server, command debugger, or second diagnostics surface.
- `McpConnectionManager` already owns resource listing and reading through `list_all_resources`, `list_resources`, `list_resource_templates`, and `read_resource`.
- Core MCP resource handlers serialize through `FunctionToolOutput`, which uses the shared function-output truncation path before model injection.
- App-server thread-resume redaction already redacts MCP tool-call result payloads and error messages.
- No exactly-one current-manager failing cap/redaction fixture was found, and the donor custom source-root env would create a new source-browsing/config surface.

## Outcome

No implementation dispatch. Row 130 can only reopen with a concrete failing current-manager cap/redaction test that does not add custom source roots, alternate repo roots, or new MCP env/config surface.
