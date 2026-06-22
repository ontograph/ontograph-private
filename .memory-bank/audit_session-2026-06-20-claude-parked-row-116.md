# Claude Parked Row 116 Review

Date: 2026-06-20

## Decision

Row 116 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 116 says cross-process bridge tests need a concrete protocol.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 116 proposes adding bridge attachments ingestion under app-server / protocol with an attachment parsing test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- Existing app-server v2 `turn/start` input already handles typed user inputs such as text, remote images, local images, skills, and mentions.
- `ontocode-rs/app-server-protocol/src/protocol/v2/turn.rs`, `ontocode-rs/app-server/src/request_processors/turn_processor.rs`, and `ontocode-rs/app-server/tests/suite/v2/turn_start.rs` cover current typed input mapping and local image forwarding.
- Feedback upload has log attachment handling, but that owner is feedback-report packaging, not bridge protocol ingestion.
- No bridge-specific attachment protocol, parser owner, failing attachment parsing test gap, or concrete cross-process bridge protocol was found.
- No fresh bug, regression, security, safety, or product evidence was found.

## Outcome

No implementation dispatch. Bridge attachments ingestion remains parked until a concrete bridge protocol or app-server v2 owner gap exists.
