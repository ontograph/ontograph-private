# Claude Parked Row 198 Review

Date: 2026-06-20

## Decision

Row 198 stays parked.

## Source

- ADR row 198: `New | Non-core | DEFER | External extension certification is too early.`
- Donor row 198: `Add feature-dev plugin workflow. | core-plugins / skills | Encodes common feature implementation steps. | Skill fixture.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Sub-agent `019ee526-807f-7ce0-8433-0cc847e0a7fe` recommended parked and made no edits.
- `ontocode-rs/skills/src/assets/samples/skill-creator/SKILL.md` already frames skills as reusable specialized workflows with scripts, references, assets, and progressive disclosure.
- `ontocode-rs/skills/src/assets/samples/skill-creator/scripts/init_skill.py` already provides a generic workflow/task/reference/capabilities skill template.
- `ontocode-rs/skills/src/assets/samples/plugin-creator/SKILL.md` already owns plugin scaffolding, optional `skills/`, validation, and update flow.
- `ontocode-rs/core-skills/src/loader_tests.rs`, `ontocode-rs/core-plugins/src/loader_tests.rs`, and `ontocode-rs/ext/skills/tests/skills_extension.rs` already cover skill roots, plugin fixtures, and bundled skill inclusion.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
