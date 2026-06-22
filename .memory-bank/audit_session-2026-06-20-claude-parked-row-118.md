# Claude Parked Row 118 Review

Date: 2026-06-20

## Decision

Row 118 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 118 says IDE extension ideas are not core runtime.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 118 proposes adding a bridge diagnostics command under TUI / app-server with a command output fixture.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- Existing CLI/app-server remote-control command output already lives in `ontocode-rs/cli/src/remote_control_cmd.rs` and is routed from `ontocode-rs/cli/src/main.rs`.
- `ontocode-rs/app-server-daemon/src/lib.rs` covers remote-control output serialization and daemon readiness context.
- `ontocode-rs/app-server-transport/src/transport/remote_control/websocket.rs` includes HTTP error details for failed remote-control websocket connections.
- `ontocode-rs/app-server/tests/suite/v2/connection_handling_websocket.rs` covers `/readyz` and `/healthz` on the websocket listener.
- No bridge-specific diagnostics command owner, failing command-output fixture gap, or accepted IDE-extension product requirement was found.
- No fresh bug, regression, security, safety, or product evidence was found.

## Outcome

No implementation dispatch. A bridge diagnostics command remains parked as IDE-extension/platform scope until a concrete core owner gap exists.
