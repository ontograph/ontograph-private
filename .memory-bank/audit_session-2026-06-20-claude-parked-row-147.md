# Claude Parked Row 147 Review

Date: 2026-06-20

## Decision

Row 147 stays parked.

## Source

- ADR row 147: `Partial / Conditional / NARROW / MCP command debugging extends existing manager diagnostics.`
- Donor row 147: `Add /model picker command. | models-manager / TUI | Makes provider model changes discoverable. | Picker snapshot.`

## Evidence

- Requested sub-agent model `gemini-3-flash` was unavailable in the current tool surface; `gpt-5.4-mini` was used as the available fallback and recorded in the tracker.
- Existing slash command metadata already exposes `/model` as the model and reasoning-effort entry point, and slash dispatch routes it to `open_model_popup`.
- Existing `ChatWidget` model popup owns the session-configured guard, all-model fallback, grouped provider picker, provider disabled rows, model rows, and reasoning-popup handoff.
- Existing provider model grouping tests cover OpenAI/Gemini/Gemini CLI/external provider catalog behavior.
- Existing TUI snapshot tests cover the base model picker, grouped provider picker, hidden-model filtering, unsupported-provider filtering, generic model-picker width, and reasoning popups.
- No exactly-one existing-owner models-manager/TUI picker snapshot gap was found for NARROW promotion.

## Outcome

No implementation dispatch. Adding `/model` picker work would duplicate the current command, picker, provider grouping, and snapshot owner chain. No Rust tests were run because no product code changed.
