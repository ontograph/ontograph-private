# Claude Parked Row 195 Review

Date: 2026-06-20

## Decision

Row 195 stays parked.

## Source

- ADR row 195: `Existing | Non-core | DEFER | Test matrix planning should stay project-plan work.`
- Donor row 195: `Add plugin development guide. | plugins/README.md | Lowers extension authoring cost. | Link check.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Sub-agent `019ee51e-2709-7a20-90d8-e811a117cbca` recommended parked and made no edits.
- Existing plugin authoring guidance lives in `ontocode-rs/skills/src/assets/samples/plugin-creator/SKILL.md`, including scaffold, marketplace entry, optional folders, and validation commands.
- Existing local plugin update guidance lives in `ontocode-rs/skills/src/assets/samples/plugin-creator/references/installing-and-updating.md`, including cachebuster and reinstall flow.
- Existing plugin validation lives in `ontocode-rs/skills/src/assets/samples/plugin-creator/scripts/validate_plugin.py`.
- Runtime plugin ownership already has marketplace layout and discoverability tests in `ontocode-rs/core-plugins/src/marketplace_tests.rs`, `ontocode-rs/core/src/plugins/discoverable_tests.rs`, and `ontocode-rs/core/tests/suite/plugins.rs`.
- The only concrete local markdown link checker found is memory-bank scoped: `scripts/onto_memory_tools.py doc-link-check`. No repo plugin-doc link-check owner gap was proven.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
