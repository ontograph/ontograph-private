# Claude Parked Row 144 Review

Date: 2026-06-20

## Decision

Row 144 stays parked.

## Source

- ADR row 144: `Partial / Non-core / DEFER / Release command is automation, not core.`
- Donor row 144: `Add /output-style via prompt fragments. | prompts / config | Customizes response style without core branches. | Fragment test.`

## Evidence

- Requested sub-agent model `gemini-3-flash` was unavailable in the current tool surface; `gpt-5.4-mini` was used as the available fallback and recorded in the tracker.
- Existing style-control owners already include TUI `/personality`, `Feature::Personality`, core `PersonalitySpecInstructions`, session personality update plumbing, prompt fragments, and personality integration/snapshot tests.
- No fresh bug, regression, security, safety, or product evidence was found for DEFER promotion.
- No concrete existing-owner prompt/config fragment test gap was found.

## Outcome

No implementation dispatch. `/output-style` would add a duplicate slash-command/style runtime surface rather than extend the current personality/style prompt-fragment owners. No Rust tests were run because no product code changed.
