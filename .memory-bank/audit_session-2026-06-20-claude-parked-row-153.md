# Claude Parked Row 153 Review

Date: 2026-06-20

## Decision

Row 153 stays parked.

## Source

- ADR row 153: `New / Non-core / DEFER / Command packs are speculative.`
- Donor row 153: `Add terminal setup command. | TUI / install context | Improves shell integration. | Platform-gated snapshot.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- The donor row provides only a proposed terminal setup command and platform-gated snapshot, not fresh product or bug evidence.
- Existing terminal-related owners are diagnostics, terminal startup probing, keyboard handling, terminal-title setup, runtime TUI initialization, and Windows sandbox setup.
- No concrete existing-owner install/TUI failing test or documentation gap was found.
- Promoting this would require new installer, shell-integration, platform setup, or command-pack surface.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
