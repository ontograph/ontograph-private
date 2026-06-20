# Claude Parked Row 106 Review

Date: 2026-06-20

## Decision

Row 106 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 106 says remote bridge is platform work, not immediate core.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 106 proposes a bridge feature gate with import-safe stubs under `app-server` / bridge owners.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- Local bridge references are unrelated accepted owners such as zsh exec bridge, UDS/transport bridge, network proxy bridge, and generic TUI stub helpers.
- No concrete IDE/remote bridge owner or feature-off compile-test gap was found.
- No fresh bug, regression, security, safety, or product evidence was found.

## Outcome

No implementation dispatch. Adding bridge feature gates or stubs would create non-core platform surface without an accepted ADR/owner.
