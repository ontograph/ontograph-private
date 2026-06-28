# Excel Formula SQL Readiness Closure

Date: 2026-06-28

## Scope Closed

Closed the bounded offline `ext/excel` follow-up:

- added standalone tool `excel.inspect_formula_sql_readiness`
- reused existing `inspect_sheet_formulas_with_display_path` and `SheetFormulaSummary.sql_preview`
- kept workbook inspection and parser owners unchanged

## Code Evidence

Changed code files:

- `ontocode-rs/ext/excel/src/formula_sql_readiness.rs`
- `ontocode-rs/ext/excel/src/formula_sql_readiness_tests.rs`
- `ontocode-rs/ext/excel/src/lib.rs`
- `ontocode-rs/ext/excel/src/extension.rs`
- `ontocode-rs/ext/excel/src/tests.rs`

Behavior closed in this slice:

- readiness counts for `scalar_row_local`, `exact_lookup`, `aligned_aggregate`, `blocked`, `malformed`, and `unsupported`
- bounded ready/blocked formula samples
- deterministic blocked-reason frequency summary
- explicit truncation warnings for bounded samples
- direct tool execution coverage for relative workbook paths

## Validation

From `ontocode-rs`:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fmt`

OntoIndex scope verification:

- `gn_verify_diff` PASS for
  - `ontocode-rs/ext/excel/src/formula_sql_readiness.rs`
  - `ontocode-rs/ext/excel/src/formula_sql_readiness_tests.rs`
  - `ontocode-rs/ext/excel/src/lib.rs`
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`

## Remaining Gaps

Still out of scope after closure:

- no new SQL planner families beyond current scalar, exact lookup, and aligned aggregate support
- no workbook-wide migration packaging
- no live Excel, COM, ADO, DAX, `Formula2`, or mutation work
- no widening of `inspect_workbook_with_display_path`

## Reopen Gate

Reopen only for one of:

- approved broader SQL families with real fixture proof
- workbook-wide migration packaging that composes readiness with other offline review artifacts
- a separately approved live-owner contract for apply or import work
