# Claude Parked Row 131 Review

Date: 2026-06-20

## Decision

Row 131 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 131 says slash command metadata should extend the existing command system.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 131 proposes adding command types `prompt`, `local`, and `local-UI` in the TUI command layer with a command registry enum test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `ontocode-rs/tui/src/slash_command.rs` already owns slash-command metadata and behavior through descriptions, command strings, inline-argument support, side-conversation availability, task availability, and visibility.
- `ontocode-rs/tui/src/bottom_pane/slash_commands.rs` consumes that metadata for command filtering, service-tier command insertion, lookup, and availability checks.
- `ontocode-rs/tui/src/bottom_pane/command_popup.rs` and `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs` already route popup and dispatch behavior through the existing command system.
- No exactly-one current-owner failing test or docs gap was found for a `prompt/local/local-UI` command-type enum.
- Adding that taxonomy would introduce a new command classification dimension rather than narrow existing metadata coverage.

## Outcome

No implementation dispatch. Row 131 can reopen only with a concrete failing test in the existing TUI slash-command metadata owner that does not add a new command architecture, API, or runtime surface.
