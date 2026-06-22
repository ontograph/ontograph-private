# Claude Parked Row 155 Review

Date: 2026-06-20

## Decision

Row 155 stays parked.

## Source

- ADR row 155: `Partial / Conditional / NARROW / Command permissions can extend existing approval metadata.`
- Donor row 155: `Add sandbox-toggle command. | permissions / TUI | Easier controlled permission mode changes. | Permission profile test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing TUI owners already define `/permissions` and open the permission profiles popup.
- Existing permission selection flow already updates permission profiles and approval policies through `SelectPermissionProfile`.
- Existing tests cover permission profile selection, full-access/default transitions, history messages, custom profiles, Windows permission prompts, and status rendering.
- Adding a sandbox-toggle command would introduce a duplicate command/permission surface rather than a single missing owner-local test or documentation gap.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
