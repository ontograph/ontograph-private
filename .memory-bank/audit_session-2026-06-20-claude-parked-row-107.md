# Claude Parked Row 107 Review

Date: 2026-06-20

## Decision

Row 107 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 107 says browser/web bridge should wait for app-server demand.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 107 proposes a no-op bridge handle under `app-server` / TUI with a disabled bridge test.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- Existing TUI disabled-feature stubs and browser/login/open-url references are generic or unrelated to an app-server/TUI bridge handle.
- No concrete disabled bridge owner, bridge-handle surface, or disabled-bridge test gap was found.
- No fresh app-server demand, bug, regression, security, safety, or product evidence was found.

## Outcome

No implementation dispatch. Adding a no-op bridge handle would create bridge/app-server/TUI surface without accepted demand or an existing owner-local gap.
