# Audit Session: Excel Stage 1 Cwd Closure

## Date

2026-06-23

## Scope

Close the remaining Stage 1 Excel ADR findings without widening core runtime, tool payloads, or app-server integration.

## Decision

Accepted as closed.

The remaining Stage 1 correctness gap was real: `excel.inspect_workbook` validated relative paths but still resolved them against the process cwd. The repair stayed owner-local inside `ext/excel`:

- reuse extension turn-input lifecycle to capture the primary turn cwd in Excel extension thread state
- resolve model-supplied relative workbook paths against that stored cwd inside `ExcelInspectionTool`
- preserve the user-facing relative path in `InspectWorkbookResult.path`
- keep app-server integration to the existing one install line
- keep Stage 2/3 and any orchestration wrapper out of scope

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `ontoindex analyze --skills --skip-agents-md`
- OntoIndex `gn_verify_diff` PASS for the Excel ADR/file set
- OntoIndex `gn_test_gap` PASS for the Excel extension file set

## Residuals

- Worktree remains dirty outside this slice, so closure is scoped to the Excel ADR files and `ext/excel` changes only.
- Embeddings remain absent in OntoIndex metadata; not required for this closure.
