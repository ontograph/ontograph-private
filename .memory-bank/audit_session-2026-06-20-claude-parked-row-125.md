# Claude Parked Row 125 Review

Date: 2026-06-20

## Decision

Row 125 stays rejected.

## Source

- ADR row 125: `Existing | Non-core | REJECT | Symbol search should use OntoIndex/search path.`
- Donor row 125: `Add get-command-source MCP endpoint gated to dev mode. | codex-mcp | Helps command review. | Dev-only permission test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Sub-agent `019ee536-52a8-7563-96bf-ef36f786bd79` recommended rejected and made no edits.
- `ontocode-rs/tui/src/slash_command.rs` owns `SlashCommand` metadata and built-in command enumeration.
- `ontocode-rs/tui/src/bottom_pane/slash_commands.rs` and `ontocode-rs/tui/src/bottom_pane/command_popup.rs` consume slash-command metadata for popup/discovery behavior.
- `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs` owns typed slash-command dispatch.
- `.memory-bank/ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md` records OntoIndex/GitNexus-style code graph tooling as the code-exploration owner.
- The parked ADR already rejects row 125 because symbol search should use OntoIndex/search rather than a cloned MCP source surface.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
