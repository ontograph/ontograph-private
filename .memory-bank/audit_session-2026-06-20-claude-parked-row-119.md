# Claude Parked Row 119 Review

Date: 2026-06-20

## Decision

Row 119 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 119 says remote workspace mapping is high-risk.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 119 proposes adding a bridge env-less transport abstraction under app-server with a transport trait test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `ontocode-rs/app-server-transport/src/transport/mod.rs` already owns app-server transports for stdio, Unix socket, websocket, and off modes.
- `ontocode-rs/app-server-transport/src/transport/remote_control/websocket.rs` and `ontocode-rs/app-server-transport/src/transport/remote_control/tests.rs` cover remote-control websocket runtime, reconnect, token refresh, buffering, and stdio client-name gating behavior.
- `ontocode-rs/exec-server/src/environment.rs` already owns explicit remote exec-server transport selection.
- App-server v2 already has experimental runtime workspace roots, but no accepted bridge remote workspace mapping contract was found.
- No bridge-specific env-less transport owner, failing transport trait test gap, or concrete remote workspace mapping proof was found.
- No fresh bug, regression, security, safety, or product evidence was found.

## Outcome

No implementation dispatch. Env-less bridge transport remains parked until remote workspace mapping and bridge transport ownership are proven.
