# Claude Parked Row 117 Review

Date: 2026-06-20

## Decision

Row 117 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 117 says browser extension ideas are not core.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 117 proposes adding a bridge status TUI indicator with snapshot coverage.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- Existing app-server remote-control status APIs already expose disabled, connecting, and related status values.
- `ontocode-rs/app-server/tests/suite/v2/remote_control.rs` covers status-read and enable status behavior.
- `ontocode-rs/app-server-daemon/src/remote_control_client.rs` handles connected and connecting status notifications.
- TUI `/status` already has remote app-server connection display plumbing in `ontocode-rs/tui/src/status/remote_connection.rs` and `ontocode-rs/tui/src/status/card.rs`.
- No bridge/browser-extension status protocol, TUI owner-local failing snapshot gap, or accepted product requirement was found.
- No fresh bug, regression, security, safety, or product evidence was found.

## Outcome

No implementation dispatch. A bridge status TUI indicator remains downstream of a concrete bridge/browser-extension platform decision.
