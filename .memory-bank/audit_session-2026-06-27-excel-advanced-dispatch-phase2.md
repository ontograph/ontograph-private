# Excel Advanced Dispatch Phase 2

## Decision

Opened Phase 2 of `EXCEL_ADVANCED_TOOLS_PROJECT_PLAN.md` as the active next task.

Phase 3 remains queued behind Phase 2.

## Scope

- `excel.scan_sheet_formulas_dependency`
- `excel.generate_slider_query_package`

## Guardrails

- keep the offline owner inside `ontocode-rs/ext/excel`
- preserve fail-closed handling for cycles and volatile formulas
- do not widen Phase 3 live COM/ADO execution until Phase 2 lands or is reprioritized

## Evidence

- `EXCEL_ADVANCED_PHASE2_FIXTURE_PROOF.md`
- `ADR_EXCEL_LIVE_COMPANION.md`
