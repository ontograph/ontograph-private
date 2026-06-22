# Claude Parked Row 123 Review

Date: 2026-06-20

## Decision

Row 123 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 123 says MCP source browsing must not bypass OntoIndex/security policy.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 123 proposes adding a list-commands MCP endpoint under app-server / commands with a command registry test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `ontocode-rs/tui/src/slash_command.rs` owns the slash command enum and descriptions.
- `ontocode-rs/tui/src/bottom_pane/slash_commands.rs` owns command filtering, service-tier insertion, lookup, and feature/side-conversation gating, with local tests.
- `ontocode-rs/tui/src/chatwidget/tests/slash_commands.rs` covers command dispatch through app-server-backed flows such as `/mcp`.
- `ontocode-rs/codex-mcp/src/connection_manager.rs` already owns MCP tool listing through `McpConnectionManager::list_all_tools`.
- No fresh bug, regression, security, safety, product evidence, or concrete existing app-server command-registry test gap was found.
- Adding command listing over MCP would create a new command/source-browsing API surface and risks bypassing OntoIndex/security policy.

## Outcome

No implementation dispatch. Command discoverability remains owned by TUI slash-command surfaces unless a concrete app-server command-registry requirement is accepted.
