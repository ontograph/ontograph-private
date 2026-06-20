# Claude Parked Row 160 Review

Date: 2026-06-20

## Decision

Row 160 stays parked.

## Source

- ADR row 160: `Partial / Non-core / NARROW / Command test fixtures useful, but not core alone.`
- Donor row 160: `Add global search dialog for commands/files. | TUI | Better discoverability. | Snapshot test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing command popup and file search popup are separate TUI owners.
- Command popup already has snapshot coverage.
- File search already has composer orchestration and popup synchronization.
- Generic searchable selection views and keymap picker search cover adjacent picker behavior.
- A combined global search dialog would add new navigation/UI surface rather than close one fixture gap.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
