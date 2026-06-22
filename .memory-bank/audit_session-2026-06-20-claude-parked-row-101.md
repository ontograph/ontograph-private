# Claude Parked Row 101 Review

Date: 2026-06-20

## Decision

Row 101 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 101 says to avoid a second hook registry.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 101 proposes adding a teammate idle hook under `hooks` / agent jobs.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- The existing hook registry and protocol hook event surface have no teammate-idle event.
- Existing matcher handling applies to the current hook events and does not expose an idle-teammate signal to test.
- The idle signal found in code is an extension lifecycle path, not a hooks registry event.

## Outcome

No implementation dispatch. A teammate idle hook would require new hook/protocol/config surface or a second lifecycle owner instead of hardening one existing matcher/event test gap.
