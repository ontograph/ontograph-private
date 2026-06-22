# Claude Parked Row 110 Review

Date: 2026-06-20

## Decision

Row 110 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 110 says bridge protocol must prove app-server cannot serve it.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 110 proposes scheduling proactive bridge token refresh from JWT expiry under auth / app-server.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- Remote-control enrollment already refreshes server tokens before expiry with a skew window and has a focused unit test for that decision.
- Remote-control transport tests already cover refreshing persisted enrollment before connecting and refreshing after websocket unauthorized responses.
- Managed ChatGPT auth already supports proactive refresh using JWT expiration, and app-server auth tests cover proactive refresh failure and recovery.
- RMCP OAuth has separate token-needs-refresh skew handling.
- No bridge-specific protocol proof, fresh mid-session disconnect evidence, or missing scheduler-test owner was found.

## Outcome

No implementation dispatch. Adding a bridge token scheduler would duplicate existing auth/remote-control refresh owners or create new bridge protocol surface without ADR-backed demand.
