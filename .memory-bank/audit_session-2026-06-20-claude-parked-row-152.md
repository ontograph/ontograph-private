# Claude Parked Row 152 Review

Date: 2026-06-20

## Decision

Row 152 stays parked.

## Source

- ADR row 152: `Partial / Non-core / NARROW / Plugin command polish belongs in plugin manager/UI.`
- Donor row 152: `Add reload-plugins command. | core-plugins | Faster plugin dev loop. | Cache clear test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing app-server/API documentation already exposes plugin and marketplace owners such as `plugin/list`, `plugin/installed`, `plugin/read`, `marketplace/add`, `marketplace/remove`, and `marketplace/upgrade`.
- Existing core plugin manager paths already clear caches after effective plugin changes, marketplace upgrades, curated repo sync, non-curated refresh, remote installed cache refresh, install, and uninstall.
- Existing TUI plugin popup behavior already refreshes list state and preserves row/tab selection across refreshes.
- Existing tests cover plugin cache/config cleanup, marketplace upgrade/no-op cases, installed count refresh, duplicate marketplace-tab preservation, and plugin list refresh behavior.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
