# Audit Session: Excel Stage 2 Preview Closure

## Date

2026-06-23

## Scope

Close Stage 2 of [ADR_EXCEL_AGENT_TOOLS.md](ADR_EXCEL_AGENT_TOOLS.md) with one bounded preview tool only:

- add `excel.read_sheet_preview`
- keep ownership inside `ontocode-rs/ext/excel`
- support `.xlsx` and `.xlsm` only
- keep rows, columns, cell text, worksheet XML, and shared strings bounded
- reject CSV export, orchestration, `.xlsb` sheet preview, and new app-server/core surface

## What changed

- added `ontocode-rs/ext/excel/src/preview.rs`
- added `ExcelReadSheetPreviewTool` in [ontocode-rs/ext/excel/src/tool.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/tool.rs:1)
- kept `backend.rs` as the workbook metadata owner and reused it for sheet selection
- reused the existing thread-owned cwd path resolution from Stage 1
- added focused regression coverage in [ontocode-rs/ext/excel/src/tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/tests.rs:1)

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `ontoindex analyze --skills --skip-agents-md`
- OntoIndex `gn_verify_diff` PASS for:
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/preview.rs`
  - `ontocode-rs/ext/excel/src/tool.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
- OntoIndex `gn_test_gap` PASS for the same file set

## Decision

Stage 2 is closed. The approved Excel surface now contains:

- `excel.inspect_workbook`
- `excel.read_sheet_preview`

Stage 3 `excel.export_sheet_to_csv` remains future work and requires a new bounded loop.
