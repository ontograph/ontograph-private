# Claude Parked Row 191 Review

Date: 2026-06-20

## Decision

Row 191 stays parked.

## Source

- ADR row 191: `Partial | Non-core | NARROW | Plugin packaging docs can improve extension workflow.`
- Donor row 191: `Add command test scripts for slash commands. | scripts/tests | Fast command regression checks. | test-commands equivalent.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `ontocode-rs/tui/src/bottom_pane/chat_composer/slash_input.rs` already has focused slash completion/parsing tests.
- `ontocode-rs/tui/src/chatwidget/tests/slash_commands.rs` already covers queued slash dispatch, command recall behavior, unavailable-command behavior, and many command-specific dispatch paths.
- `ontocode-rs/tui/src/chatwidget/tests/status_command_tests.rs` already covers `/status` behavior separately.
- Command popup rendering already has snapshot coverage under TUI bottom-pane snapshots.
- A separate `test-commands` script would add a new test surface rather than close one proven owner-local gap.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
