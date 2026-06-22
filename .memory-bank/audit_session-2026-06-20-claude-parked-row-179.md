# Claude Parked Row 179 Review

Date: 2026-06-20

## Decision

Row 179 stays parked.

## Source

- ADR row 179: `New / Non-core / DEFER / Session timeline UI is optional.`
- Donor row 179: `Add package manager auto-update wrapper. | install / TUI | Better update UX. | Platform-gated test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- TUI update actions already map npm, bun, brew, standalone Unix, and standalone Windows update commands with tests.
- CLI already exposes `codex update` and runs the detected update action for release builds.
- Update notice history cells and snapshots already render platform-specific update instructions.
- Update prompt/version probing and doctor update checks already own update discovery and health reporting.
- Standalone installer scripts already validate versions, detect platforms, update existing installs, and handle conflicting package-manager installs.
- No fresh install/update safety evidence or exactly-one owner-local platform-gated test/doc gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
