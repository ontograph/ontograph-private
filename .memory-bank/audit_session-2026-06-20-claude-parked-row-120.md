# Claude Parked Row 120 Review

Date: 2026-06-20

## Decision

Row 120 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 120 says bridge status UI is downstream of platform decision.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 120 proposes adding bridge teardown flush before close under app-server with a teardown test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- Existing stdio and remote-control writers carry `write_complete_tx` signals after successful writes.
- `ontocode-rs/app-server-transport/src/transport/remote_control/websocket.rs` buffers outbound envelopes, acknowledges them by backend ack, and has shutdown cancellation coverage.
- `ontocode-rs/app-server-transport/src/transport/remote_control/client_tracker.rs` waits for queue capacity before emitting connection-closed events.
- `ontocode-rs/app-server/tests/suite/v2/connection_handling_websocket_unix.rs` covers graceful websocket transport shutdown while a turn is running.
- No bridge-specific teardown flush owner, failing lost-final-message teardown test, or accepted bridge platform contract was found.
- No fresh bug, regression, security, safety, or product evidence was found.

## Outcome

No implementation dispatch. Bridge teardown flush remains parked until a concrete bridge platform decision or current-owner shutdown bug exists.
