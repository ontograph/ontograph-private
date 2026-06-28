# Excel Advanced Tools Phase 1 Closure

## Decision

Closed Phase 1 of `EXCEL_ADVANCED_TOOLS_PROJECT_PLAN.md`.

The requested `excel.read_data_validation_rules` capability is covered by the existing offline `excel.read_sheet_preview` owner through its `data_validations` output.

No separate tool surface was added.

## Implementation Shape

- extended focused fixture coverage for worksheet data validations
- kept parsing in `ontocode-rs/ext/excel/src/preview.rs`
- kept output in the existing `SheetDataValidationSummary`

## Guardrails

- list validations resolve inline lists and same-sheet ranges only
- non-list validations expose formulas and operators without trying to resolve list values
- unresolved list formulas remain explicit warnings
- no live Excel, COM, ADO, DAX, or mutation scope was opened

## Remaining Gates

- Phase 2 requires a fresh approved fixture pack proving exact DAG output and `SliderQuery` artifact layout
- Phase 3 requires an accepted live-owner ADR before any COM/ADO implementation
