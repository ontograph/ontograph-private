# Audit Session: Excel Stage 3 Export Closure

## Date

2026-06-23

## Scope

Close Stage 3 of [ADR_EXCEL_AGENT_TOOLS.md](ADR_EXCEL_AGENT_TOOLS.md) with one explicit CSV export tool only:

- add `excel.export_sheet_to_csv`
- keep ownership inside `ontocode-rs/ext/excel`
- support `.xlsx` and `.xlsm` only
- write a local CSV file for explicit handoff into `spawn_agents_on_csv`
- reject orchestration wrappers, `.xlsb` export, and new app-server/core surface

## What changed

- added `ontocode-rs/ext/excel/src/export.rs`
- added `ExcelExportSheetToCsvTool` in [ontocode-rs/ext/excel/src/tool.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/tool.rs:1)
- reused the existing Stage 1/2 workbook inspection, sheet selection, and turn-cwd path handling
- kept `backend.rs` unchanged and left orchestration in the existing core CSV job owner
- added focused export coverage in [ontocode-rs/ext/excel/src/tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/tests.rs:1)

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `ontoindex analyze --skills --skip-agents-md`
- OntoIndex `gn_verify_diff` PASS for:
  - `ontocode-rs/ext/excel/src/export.rs`
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/lib.rs`
  - `ontocode-rs/ext/excel/src/preview.rs`
  - `ontocode-rs/ext/excel/src/tool.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
- OntoIndex `gn_test_gap` PASS for the same file set

## Decision

The ADR loop is closed. The approved Excel surface now contains:

- `excel.inspect_workbook`
- `excel.read_sheet_preview`
- `excel.export_sheet_to_csv`

Any future Excel-specific orchestration wrapper still requires separate usage evidence and a new ADR.
