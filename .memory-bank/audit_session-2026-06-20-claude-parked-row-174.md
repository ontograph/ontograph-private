# Claude Parked Row 174 Review

Date: 2026-06-20

## Decision

Row 174 stays parked.

## Source

- ADR row 174: `Partial / Non-core / NARROW / Status/debug panes can use existing diagnostics.`
- Donor row 174: `Add rate-limit option command/UI. | TUI / model-provider | Makes retry/limit state clear. | Rate-limit fixture.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Backend-client tests already cover rate-limit payload mapping, additional limits, absent primary limits, and every `rate_limit_reached_type`.
- `/status` already renders rate-limit cards and has refresh/cached/unavailable/stale/credit snapshot coverage.
- Chatwidget tests already cover rate-limit warning thresholds, fallback labels, switch prompts, and rate-limit prompt behavior.
- SSE retry-after parsing and protocol usage-limit error formatting already have focused tests.
- `/debug-config` already has config-layer rendering tests.
- No exactly-one owner-local missing fixture, test, or doc gap was found.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
