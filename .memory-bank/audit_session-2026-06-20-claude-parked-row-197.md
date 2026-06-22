# Claude Parked Row 197 Review

Date: 2026-06-20

## Decision

Row 197 stays parked.

## Source

- ADR row 197: `Partial | Non-core | NARROW | Extension docs should link to existing plugin/skill owners.`
- Donor row 197: `Add code-review plugin as extension. | core-plugins / skills | Keeps review logic outside core. | Plugin fixture.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Sub-agent `019ee523-7537-7b21-9a2d-008dfab5b1bf` recommended parked and made no edits.
- Existing review workflow guidance lives in `.codex/skills/code-review/SKILL.md` and companion `code-review-*` skills.
- `ontocode-rs/core/tests/suite/plugins.rs` already writes plugin-bundled skills and asserts developer-message plugin sections, plugin skill namespacing, and explicit plugin mention guidance.
- `ontocode-rs/core/src/plugins/render_tests.rs` already asserts plugin skill naming guidance in rendered plugin instructions.
- `ontocode-rs/core-skills/src/injection_tests.rs` keeps plugin skill namespaces in explicit skill mentions.
- `ontocode-rs/core-skills/src/manager_tests.rs` covers plugin-skill config filtering by namespaced skill name.
- `ontocode-rs/ext/skills/tests/skills_extension.rs` covers bundled skill inclusion through the skill provider query.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
