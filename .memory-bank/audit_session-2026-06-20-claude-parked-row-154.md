# Claude Parked Row 154 Review

Date: 2026-06-20

## Decision

Row 154 stays parked.

## Source

- ADR row 154: `Partial / Non-core / DEFER / Command localization is not core now.`
- Donor row 154: `Add remote-env command. | app-server / config | Makes remote execution setup explicit. | Config fixture.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing app-server protocol already exposes `environment/add` for named remote environments.
- Existing app-server thread/turn APIs already support sticky and turn-scoped environment selection.
- Existing remote-control APIs already carry environment ids.
- Existing command/process execution APIs already support environment overrides and disabled-local-environment errors.
- Existing exec-server and core tests cover remote environment routing and remote-backed tool execution.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
