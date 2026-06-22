# Claude Parked Row 102 Review

Date: 2026-06-20

## Decision

Row 102 stays parked.

## Source

- ADR row: `.memory-bank/ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md` row 102 says to extend matcher tests only.
- Donor row: `.memory-bank/CLAUDE_CODE_DONOR_200_APPROACHES_REVIEW.md` row 102 proposes adding a task completed hook under `hooks`.

## Evidence

- The requested `gemini-3-flash` sub-agent model was unavailable in the current tool surface; the worker used `gpt-5.4-mini` fallback.
- The current `HookEventName` surface has no task-completed event.
- Existing matcher tests already cover supported matcher-bearing events and unsupported events.
- Stop-hook terminal payload and continuation behavior are already covered in the core hook suite.

## Outcome

No implementation dispatch. Task-completed hook support would require a new hook/protocol/job-lifecycle surface rather than a matcher-test-only extension.
