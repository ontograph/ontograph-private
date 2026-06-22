# Claude Parked Row 133 Review

Date: 2026-06-20

## Decision

Row 133 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 133 says command discovery must not duplicate tools.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 133 proposes adding a command progress message field in the TUI command layer with a snapshot test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `ontocode-rs/tui/src/slash_command.rs` already owns command names, descriptions, inline-argument support, side-conversation availability, task availability, and visibility.
- `ontocode-rs/tui/src/chatwidget/slash_dispatch.rs` already owns per-command dispatch behavior and in-progress rejection messaging.
- Specific long or async command paths already add targeted progress/loading UI where needed, such as MCP inventory loading.
- No exactly-one current-owner failing progress-message metadata or snapshot gap was found.
- A generic command progress message field would add new command-progress/discovery surface instead of narrowing an existing test gap.

## Outcome

No implementation dispatch. Row 133 can reopen only with a concrete failing snapshot or metadata test in the existing TUI slash-command owner, without adding a new command registry, progress framework, API, or duplicated tool-discovery surface.
