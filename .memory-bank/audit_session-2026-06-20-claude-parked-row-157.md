# Claude Parked Row 157 Review

Date: 2026-06-20

## Decision

Row 157 stays parked.

## Source

- ADR row 157: `Existing / Conditional / NARROW / Shell command behavior must reuse existing shell/sandbox owner.`
- Donor row 157: `Add export session command. | thread-store / rollout-trace | Useful audit artifact. | Export fixture.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing TUI session commands cover archive/unarchive and rollout-trace diagnostics, but no session export command exists to harden.
- Existing app-server documentation and thread-store ownership cover archive/unarchive, append/update/live persistence, and rollout trace reduction, not a stable export API or fixture path.
- Adding export would introduce a new user-facing artifact/export surface and likely new format/API behavior rather than closing a single existing-owner shell/sandbox test gap.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
