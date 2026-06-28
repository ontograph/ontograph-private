# Audit Session: Excel Lefties Phase 4B Closure

## Date

2026-06-27

## Scope

Close the active Phase 4B exact-lookup SQL slice inside `ontocode-rs/ext/excel` without widening owner scope.

## Decision

Phase 4B is complete.

The bounded review-only worksheet-formula SQL preview now covers:

- exact `VLOOKUP(..., FALSE|0)` over a uniquely resolved defined-name range
- exact `XLOOKUP` over proven aligned worksheet vectors
- exact `INDEX(MATCH(...,0))` over the same proven aligned worksheet vectors

Unsupported lookup modes remain fail-closed:

- approximate lookup modes
- reverse/binary `XLOOKUP` search modes
- unresolved or unproven lookup vectors
- workbook shapes with external links

## Evidence

- owner-local planner support lives in `ontocode-rs/ext/excel/src/formula_sql.rs`
- direct planner coverage lives in `ontocode-rs/ext/excel/src/formula_sql_tests.rs`
- end-to-end worksheet formula inventory coverage lives in `ontocode-rs/ext/excel/src/tests.rs`

## Validation

- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fmt`
- scoped `gn_verify_diff` on the touched Excel file set

## Reopen Gate

Do not auto-dispatch a Phase 4C aggregate slice. Reopen only if a fresh approved fixture pack proves target-column resolution and same-grain criteria/range alignment for optional aggregate planning.
