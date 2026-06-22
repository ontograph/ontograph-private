# Claude Parked Row 199 Review

Date: 2026-06-20

## Decision

Row 199 stays parked.

## Source

- ADR row 199: `New | Non-core | DEFER | Extension analytics/privacy surface needs product ADR.`
- Donor row 199: `Add frontend-design plugin workflow. | core-plugins / skills | Keeps UI guidance opt-in. | Skill fixture.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Sub-agent `019ee529-2703-7743-87c9-509a5da56a19` recommended parked and made no edits.
- Existing frontend guidance lives in model instructions such as `ontocode-rs/core/templates/model_instructions/gpt-5.2-codex_instructions_template.md`.
- Existing TUI style and snapshot rules live in `AGENTS.md` and `ontocode-rs/tui/styles.md`.
- `skill-creator` and `plugin-creator` already own workflow scaffolding, optional resources, and plugin skill folders.
- `agents/openai.yaml` samples already cover UI-facing skill metadata for system skills.
- `ontocode-rs/skills/src/lib.rs` embeds the system skill samples, with tests proving sample traversal.
- `ontocode-rs/core-skills/src/loader_tests.rs` covers hermetic skill loading and metadata parsing.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
