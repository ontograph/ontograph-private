# Excel Advanced Tools Project Plan

## Status
Phase 1 complete via the existing offline `excel.read_sheet_preview` owner. Phase 2 is complete in `ontocode-rs/ext/excel`; Phase 3 is postponed and not active.

## Goal
Implement the next tier of Excel tools identified during the donor review, strictly separating read-only offline parsing from live COM/ADO execution.

## Phase 1: Unblocked Offline Metadata
**Scope**: `excel.read_data_validation_rules`
**Owner**: `ontocode-rs/ext/excel`
**Details**: Pure XML parsing of `<dataValidation>` tags from worksheet parts.
**Status**: Complete as an additive extension of `excel.read_sheet_preview`; no separate tool surface was added. Fixture coverage now includes list validation and whole-number min/max validation rules.

## Phase 2: Offline DAG and SliderQuery Export
**Scope**: `excel.scan_sheet_formulas_dependency` and `excel.generate_slider_query_package`
**Owner**: `ontocode-rs/ext/excel`
**Details**: Resolves cell dependencies into a local DAG and transpiles sheets into a structured `.sql` artifact package.
**Gate**: Unblocked. Fixture pack contract established in `EXCEL_ADVANCED_PHASE2_FIXTURE_PROOF.md`. Status is now Complete after implementation and validation with `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`.

## Phase 3: Live Companion Execution
**Scope**: `excel.run_dax_evaluate`, `excel.materialize_dax_to_table`, `excel.vba_backup_modules`, `excel.write_cells_recalc_suppressed`
**Owner**: TBD (Pending ADR)
**Details**: Live interaction with the Excel COM API and the internal VertiPaq Data Model via ADO.
**Gate**: Postponed. Crate boundary and sidecar options are established in `ADR_EXCEL_LIVE_COMPANION.md`, but Phase 3 is not active until explicitly reprioritized.

## Execution Rules
- Phase 1 is closed; do not reopen a separate `excel.read_data_validation_rules` tool unless a new consumer proves `excel.read_sheet_preview.data_validations` is insufficient.
- Phase 2 is closed; Phase 3 is postponed until it is explicitly reprioritized.
