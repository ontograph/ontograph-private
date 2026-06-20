# Claude Parked Row 173 Review

Date: 2026-06-20

## Decision

Row 173 stays parked.

## Source

- ADR row 173: `New / Non-core / DEFER / Onboarding UI is not generated-code core.`
- Donor row 173: `Add cost threshold dialog. | TUI / token usage | Lets users stop expensive sessions. | Snapshot test.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `/status` already owns session configuration, token usage, rate limits, credits, and spend-control-related display.
- TUI tests already cover rate-limit warning thresholds, monthly/generic fallback warnings, lower-cost model switch prompt behavior, credits rendering, missing limit snapshots, and cached limit/credit behavior.
- App-server v2 account protocol already exposes rate-limit, credit, and spend-control snapshot types.
- Goal runtime tests already cover usage-limit behavior that stops idle continuation.
- No fresh spend-control safety/product evidence or exactly-one owner-local missing test/doc gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
