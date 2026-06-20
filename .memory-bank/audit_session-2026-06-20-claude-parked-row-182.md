# Claude Parked Row 182 Review

Date: 2026-06-20

## Decision

Row 182 stays parked.

## Source

- ADR row 182: `Existing | Non-core | DEFER | Docs generation exists as process, not core runtime.`
- Donor row 182: `Add strict/lax settings examples. | docs/examples | Helps users understand permission profiles. | Link check.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing docs already cover permission-profile examples: `ontocode-rs/app-server/README.md` documents `permissionProfile/list` and the `permissionProfile` versus `sandboxPolicy` boundary.
- Existing README examples cover concrete permission profiles in `ontocode-rs/network-proxy/README.md` and `ontocode-rs/linux-sandbox/README.md`.
- Existing link-check tooling is present in `scripts/onto_memory_tools.py doc-link-check`.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
