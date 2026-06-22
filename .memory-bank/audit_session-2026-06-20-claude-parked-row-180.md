# Claude Parked Row 180 Review

Date: 2026-06-20

## Decision

Row 180 stays parked.

## Source

- ADR row 180: `New / Non-core / DEFER / UI eval viewer belongs in tooling first.`
- Donor row 180: `Add native auto-updater abstraction. | install context | Separates install channels. | Config test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `install-context` already owns install-channel detection, package-layout inference, bundled resource discovery, and managed package layout behavior.
- TUI update actions already map install contexts to npm, bun, brew, standalone Unix, and standalone Windows update commands.
- `codex update` already invokes the detected update action in release builds.
- Doctor update checks already report update action, cached version state, latest-version freshness, and npm update-target safety.
- Install-context tests already cover standalone layouts, package layouts, npm/bun precedence, and Homebrew detection.
- No fresh update safety evidence or exactly-one install-context/config test gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
