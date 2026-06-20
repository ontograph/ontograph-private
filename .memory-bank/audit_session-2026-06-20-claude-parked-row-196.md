# Claude Parked Row 196 Review

Date: 2026-06-20

## Decision

Row 196 stays parked.

## Source

- ADR row 196: `New | Non-core | DEFER | Marketplace publishing is speculative.`
- Donor row 196: `Add plugin-specific README template. | core-plugins | Standardizes plugin docs. | Template lint.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Sub-agent `019ee520-bd5b-7ce1-ae3c-6480a79c363f` recommended parked and made no edits.
- Existing plugin scaffold behavior in `ontocode-rs/skills/src/assets/samples/plugin-creator/scripts/create_basic_plugin.py` creates `plugin.json`, optional folders, `.mcp.json`, `.app.json`, and marketplace entries, but has no README runtime requirement.
- Existing plugin validation in `ontocode-rs/skills/src/assets/samples/plugin-creator/scripts/validate_plugin.py` checks the manifest contract and plugin assets, not README template policy.
- Existing `plugin-json-spec.md` documents the manifest and validation contract.
- Core plugin runtime manifest loading in `ontocode-rs/core-plugins/src/manifest.rs` covers manifest/interface/path fields.
- Remote bundle tests in `ontocode-rs/core-plugins/src/remote_bundle.rs` enforce the standard plugin root around `plugin.json`; README files are not part of that runtime gate.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
