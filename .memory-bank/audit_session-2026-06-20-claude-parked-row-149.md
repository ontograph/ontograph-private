# Claude Parked Row 149 Review

Date: 2026-06-20

## Decision

Row 149 stays parked.

## Source

- ADR row 149: `Existing / Conditional / NARROW / Session command should reuse current session state.`
- Donor row 149: `Add /hooks command. | hooks / TUI | Makes hook state manageable. | Hook list test.`

## Evidence

- Requested sub-agent model `gemini-3-flash` was unavailable in the current tool surface; `gpt-5.4-mini` was used as the available fallback and recorded in the tracker.
- Existing TUI command metadata already defines `/hooks` as the lifecycle-hook management command, and slash dispatch routes it to the hook browser output path.
- Existing TUI `hooks_rpc` fetches `hooks/list` for the current cwd and normalizes missing cwd entries.
- Existing app-server `hooks/list` resolves hooks from config layers and plugin hook sources, returning hook metadata, warnings, and errors.
- Existing hooks browser owns hook event counts, trust/review state, managed hooks, diagnostics, and hook trust writes.
- Existing hook live/history cells render active and completed hook runs.
- Existing `/debug-config` exposes managed-hook requirements, and existing app-server/TUI tests cover hook listing and diagnostics.
- No exactly-one existing-owner hooks/TUI hook-state listing gap was found for NARROW promotion.

## Outcome

No implementation dispatch. The donor `/hooks` command duplicates the current command, `hooks/list`, browser, trust, debug, and hook-run rendering owner chain. No Rust tests were run because no product code changed.
