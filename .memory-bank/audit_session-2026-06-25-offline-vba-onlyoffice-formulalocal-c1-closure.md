# Offline VBA To ONLYOFFICE FormulaLocal C1 Closure

Date: 2026-06-25

## Scope

Bounded reopen of Slice `C1` from `.memory-bank/ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md`.

## Roles

- manager: current session
- senior-reviewer: manager-local narrow acceptance because the trigger stayed owner-local and code-sized
- implementation-worker: current session
- verification-worker: current session

## Trigger Evidence

- real workbook sample: `tmp/vba-samples/tabell.vba`
- source workbook: `tmp/vba-samples/Табель Макрос.xlsm`
- concrete snippet: `Cells(r, erc).FormulaLocal = er`
- interpretation: this is a real target-property syntax variant in the existing formula-assignment family
- non-trigger note: the sample still uses a variable RHS, so semantics remain fail-closed and were not broadened by this slice

## Change

- `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs`
  - classify `.FormulaLocal` as `SetCellFormula`, same as `.Formula`
- `ontocode-rs/ext/excel/src/tests.rs`
  - add positive coverage for literal `Cells(...).FormulaLocal = "=SUM(...)"` analysis
  - add fail-closed coverage for variable `Cells(r, erc).FormulaLocal = er`
  - add translator coverage for literal `.FormulaLocal`

## Non-Scope Kept Closed

- no public `excel.translate`
- no runtime validation
- no parser dependency
- no broad control-flow semantics
- no relaxation of literal-formula requirements

## Verification

- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- result: `50` tests run, `50` passed, `0` skipped

## Outcome

`C1` closed for this narrow variant only.

The deferred queue is closed again. The next valid reopen remains a fresh Slice 0 review with one new concrete trigger.
