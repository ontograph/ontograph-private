# Claude Parked Row 184 Review

Date: 2026-06-20

## Decision

Row 184 stays parked.

## Source

- ADR row 184: `Existing | Non-core | DEFER | CI workflow polish should stay repo automation.`
- Donor row 184: `Add issue templates for model behavior vs bugs. | .github/ISSUE_TEMPLATE | Better triage. | Template lint.`

## Evidence

- Requested `gemini-3-flash` was unavailable in the current sub-agent model surface; used `gpt-5.4-mini` fallback.
- Existing issue templates already split app, IDE extension, CLI, other bug, feature request, and docs intake under `.github/ISSUE_TEMPLATE/`.
- `docs/contributing.md` already routes behavior changes and bugs through issue reports or existing issue upvotes.
- `.github/workflows/issue-labeler.yml` already includes a `model-behavior` label for undesirable LLM behavior.
- `.github/workflows/issue-deduplicator.yml` already handles duplicate issue triage.

## Outcome

No implementation dispatch. No Rust tests were run because no product code changed.
