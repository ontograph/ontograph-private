# Claude Parked Row 159 Review

Date: 2026-06-20

## Decision

Row 159 stays parked.

## Source

- ADR row 159: `Partial / Non-core / NARROW / Project command templates belong in memory-bank/skills.`
- Donor row 159: `Add history search dialog. | TUI / thread-store | Faster resume. | Snapshot + search test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- The resume picker is already always searchable and has search/error/search-line snapshot coverage.
- Thread-store already exposes search term/list behavior, and the TUI lookup path uses backend title search.
- No single resume-picker or thread-store search test gap was found.
- Adding a separate history search dialog would create new session-management UI surface.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
