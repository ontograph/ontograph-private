# Claude Parked Row 146 Review

Date: 2026-06-20

## Decision

Row 146 stays parked.

## Source

- ADR row 146: `Partial / Conditional / NARROW / Tool-call command debugging should be dev-only.`
- Donor row 146: `Add /effort command mapped to reasoning effort. | config / model-provider | User can tune reasoning. | Turn config test.`

## Evidence

- Requested sub-agent model `gemini-3-flash` was unavailable in the current tool surface; `gpt-5.4-mini` was used as the available fallback and recorded in the tracker.
- Existing TUI command ownership already describes `/model` as choosing both model and reasoning effort, and dispatch opens the model popup.
- Existing reasoning popup code selects supported efforts, persists model/provider/effort together, and includes Plan-mode scope handling.
- Existing reasoning shortcuts adjust the active model effort without adding another command surface.
- Existing config owners persist `model_reasoning_effort`, and app-server v2 turn/thread settings already carry `effort` for current and future turns.
- Existing tests cover reasoning popup snapshots, popup persistence, shortcut behavior, config writes, thread start, thread settings update, and resume metadata fallback.
- No exactly-one existing-owner config/model-provider turn-config test or doc gap was found for NARROW promotion.

## Outcome

No implementation dispatch. `/effort` would duplicate the current `/model` plus reasoning-popup plus thread-settings owner chain. No Rust tests were run because no product code changed.
