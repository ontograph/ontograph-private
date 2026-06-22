# Claude Parked Row 150 Review

Date: 2026-06-20

## Decision

Row 150 stays parked.

## Source

- ADR row 150: `Existing / Conditional / NARROW / Resume command belongs in current rollout/session flow.`
- Donor row 150: `Add /skills command. | core-skills / TUI | Makes skill availability visible. | Skill list test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing TUI owners already define `/skills`, dispatch it to the skills menu, list skills, and manage skill toggles.
- Existing app-server and protocol owners already expose `skills/list`, skill config writes, and `skills/changed` notifications.
- Existing `core-skills` rendering already owns visible skill-list output, budget warnings, and related tests.
- Existing TUI help/tooltips already advertise `/skills` to the user.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
