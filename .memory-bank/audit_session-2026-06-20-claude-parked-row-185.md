# Claude Parked Row 185 Review

Date: 2026-06-20

## Decision

Row 185 stays parked for broad implementation, with one senior-approved docs-only fix applied.

## Source

- ADR row 185: `Existing | Non-core | DEFER | Packaging changes require separate release ADR.`
- Donor row 185: `Add funding/security docs separation. | docs | Cleaner repo governance. | Link check.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- `SECURITY.md` already exists and documents the security reporting path and security-boundary docs.
- `docs/open-source-fund.md` already exists as a separate funding page.
- `docs/contributing.md` already includes security and responsible AI reporting guidance.
- `scripts/onto_memory_tools.py doc-link-check` already covers memory-bank local markdown links.
- The root README docs index linked contributing, install, and open-source fund, but did not link the existing security policy.

## Outcome

Senior accepted the one concrete docs-index gap and added a README `Security` link to `./SECURITY.md`.

No Rust tests were run because no product code changed.
