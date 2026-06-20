# Claude Parked Row 114 Review

Date: 2026-06-20

## Decision

Row 114 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 114 says bridge event streaming belongs in app-server if needed.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 114 proposes batching bridge outbound writes under app-server with a flush gate test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- Existing remote-control transport owners already cover bounded outbound buffering, ack handling, segmented ack advancement, queue capacity, and backpressure behavior.
- App-server transport already uses bounded outgoing queues and overload handling.
- No bridge-specific outbound batching owner or failing flush-gate test gap was found.
- No fresh bug, regression, security, safety, product evidence, or concrete app-server bridge protocol demand was found.

## Outcome

No implementation dispatch. Bridge outbound batching would add event-streaming and flush-gate behavior before the app-server bridge need is proven.
