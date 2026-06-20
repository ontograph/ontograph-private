# Claude Parked Row 136 Review

Date: 2026-06-20

## Decision

Row 136 stays parked.

## Source

- ADR row 136: Partial / Non-core / NARROW / user command examples are docs/skill assets.
- Donor row 136: add frontmatter parser for markdown commands under `core-skills` / plugins with frontmatter tests.

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent tool surface; triage used the available `gpt-5.4-mini` fallback.
- `core-skills` already owns skill `SKILL.md` YAML frontmatter parsing through `parse_skill_file` / `extract_frontmatter`.
- `core-skills` loader tests already cover frontmatter-derived metadata and malformed frontmatter cases.
- `core-plugins` already owns plugin manifest parsing separately.
- No current owner for markdown prompt commands was found; adding one would create a new command/plugin runtime surface.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
