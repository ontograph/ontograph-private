# Claude Parked Row 115 Review

Date: 2026-06-20

## Decision

Row 115 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 115 says to reuse app-server v2 protocol, not a donor-specific bridge.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 115 proposes adding a bridge permission delegation path under app-server / core approvals with a permission response fixture.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- Existing app-server v2 already exposes `item/permissions/requestApproval` for client-mediated permission grants.
- `ontocode-rs/app-server-protocol/src/protocol/v2/permissions.rs` defines `PermissionsRequestApprovalResponse` and related permission-grant scope.
- `ontocode-rs/app-server/src/bespoke_event_handling.rs` converts client permission responses into `Op::RequestPermissionsResponse`.
- `ontocode-rs/app-server/tests/suite/v2/request_permissions.rs`, `ontocode-rs/app-server/tests/common/responses.rs`, `ontocode-rs/app-server-protocol/src/protocol/v2/tests.rs`, and `ontocode-rs/core/src/session/tests.rs` already cover request-permissions payloads, response scope, SSE fixtures, round trips, and core event resolution.
- No fresh bug, regression, security, safety, product evidence, or missing app-server v2 approvals fixture gap was found.

## Outcome

No implementation dispatch. A bridge permission delegation path would add donor-specific approval protocol instead of reusing the existing app-server v2 permission request owner.
