# Offline VBA To ONLYOFFICE Audit Opened Tasks

Date: 2026-06-25
Status: opened-bounded-review-tasks

## Scope

- audit findings from the ADR review/challenge across:
  - `ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md`
  - `ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION_STAGE0_TARGET_CONTRACT.md`
  - `ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_SOLUTIONS.md`
  - `ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md`
  - `ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_DETAILED_PROJECT_PLAN.md`
  - `ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md`

## OntoIndex Evidence

- `gn_ensure_fresh` reported the `codex` index fresh at commit `2e72a6d25e147f0619863e7721107b6f11a87fc2`.
- Semantic search still found the older `ext/excel` surfaces and tests, but newer `vba_onlyoffice_*` files were only partially indexed.
- Because symbol coverage for the new ONLYOFFICE files was partial, final task-opening decisions used direct source reads after the OntoIndex freshness/search pass.

## Manager Decision

Open only these bounded review tasks:

1. `OO-VBA-AUD1`
   - reconcile Stage 3 status and current-state wording across the canonical ADR, macro-translation tracking, and follow-on docs
2. `OO-VBA-AUD2`
   - challenge the Stage 0 first-slice call catalog against the current positive translator coverage and decide whether to narrow the contract or add focused tests
3. `OO-VBA-AUD3`
   - clarify workbook-review success semantics and decide whether the finding is docs-only or needs a narrow product-behavior slice
4. `OO-VBA-AUD4`
   - challenge duplicated deferred planning docs and workstation-local sample authority; prefer docs simplification over new feature work

Keep closed:

- parser/runtime/router reopen proposals
- generic `excel.translate`
- static validator reopen
- new implementation slices without a separate trigger proof

## Role Notes

- Requested role ordering was noted, but no sub-agent dispatcher was available in this loop.
- Senior review therefore ran in the current session with OntoIndex freshness/search plus direct-source fallback where the newer Excel files were missing from the symbol index.
- No implementation-worker or verification-worker dispatch was opened because every reopened task is still a review/challenge task, not an approved code slice.
