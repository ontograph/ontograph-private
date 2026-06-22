# Claude Parked Row 165 Review

Date: 2026-06-20

## Decision

Row 165 stays parked.

## Source

- ADR row 165: `DEFER` / `New` / `Non-core` / `Rich media output is not current core need.`
- Donor row 165: `Add IDE auto-connect onboarding. | TUI / app-server | Smooth bridge setup. | Feature-gated snapshot.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing TUI onboarding orchestration and runner already own onboarding flow behavior.
- Existing `/ide` context helpers and tests already own IDE context support.
- Existing app-server remote-control tests cover pairing surfaces.
- Existing onboarding snapshot coverage already exercises adjacent onboarding UI behavior.
- No fresh bug, regression, security, safety, product evidence, or concrete feature-gated bridge onboarding snapshot gap was found.

## Outcome

No implementation dispatch. IDE auto-connect onboarding would add new bridge setup, auto-connect behavior, onboarding flow, app-server API/config, pairing, or rich-media/desktop surface. No Rust tests were run because no product code changed.
