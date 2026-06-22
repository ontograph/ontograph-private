# Claude Parked Row 161 Review

Date: 2026-06-20

## Decision

Row 161 stays parked.

## Source

- ADR row 161: `Partial / Non-core / NARROW / TUI display polish is useful but not core extension.`
- Donor row 161: `Add plugin install errors as typed UI states. | core-plugins / TUI | Improves failure diagnosis. | Snapshot test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Core-plugins already defines typed install and remote sync error enums.
- TUI plugin install/detail failures already route through the existing plugin detail error popup state.
- The plugin detail error popup has snapshot coverage, and install success/refresh behavior is also tested.
- No single plugin install/error snapshot or typed-state gap was found without adding a new error taxonomy or UI flow.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
