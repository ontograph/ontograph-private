# Claude Parked Row 108 Review

Date: 2026-06-20

## Decision

Row 108 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 108 says bridge auth needs ADR and compatibility tests first.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 108 proposes a bridge state enum under `app-server-protocol` with a serialization test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- `app-server-protocol` already has `RemoteControlConnectionStatus` with disabled, connecting, connected, and errored states.
- The status enum is already used by `remoteControl/status/read`, remote-control status notifications, app-server README text, generated JSON schema, and generated TypeScript.
- No fresh bug, regression, security, safety, product evidence, or missing compatibility-test owner was found for a separate bridge auth/state API.

## Outcome

No implementation dispatch. A new bridge state enum would duplicate existing remote-control state or create new bridge auth/state protocol surface without ADR-backed demand.
