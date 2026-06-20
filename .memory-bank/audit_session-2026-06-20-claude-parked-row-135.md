# Claude Parked Row 135 Review

Date: 2026-06-20

## Decision

Row 135 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 135 says command docs can be generated from metadata.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 135 proposes adding a moved-to-plugin command wrapper in `core-plugins` with a legacy command test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `ontocode-rs/tui/src/slash_command.rs` already owns slash-command metadata through command names, descriptions, inline-argument support, side-conversation availability, task availability, and visibility.
- `ontocode-rs/tui/src/bottom_pane/command_popup.rs` already renders user-visible command rows from that metadata.
- Existing popup tests cover metadata-to-description behavior for service-tier commands.
- No exactly-one current-owner generated-docs metadata or docs test gap was found.
- Moving commands into plugin wrappers would add a new command/plugin runtime surface and is outside the accepted narrow docs-from-metadata path.

## Outcome

No implementation dispatch. Row 135 can reopen only with a concrete failing generated-command-docs test in the existing slash-command metadata owner, without adding plugin wrappers or moving core command behavior.
