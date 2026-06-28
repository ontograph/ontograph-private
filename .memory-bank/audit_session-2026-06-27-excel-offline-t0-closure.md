# Excel Offline T0 Closure

## Scope
- Closed `EXCEL-OFFLINE-T0` from `EXCEL_OFFLINE_NEXT_TOOLS_PROJECT_PLAN.md`
- Kept the change inside `ontocode-rs/ext/excel`
- Reused existing `read_sheet_preview` and `inspect_sheet_formulas` owners

## What Landed
- Added a pure-Rust `.xlsb` reader path via `calamine` for:
  - `excel.read_sheet_preview`
  - `excel.inspect_sheet_formulas`
- Added a real embedded `.xlsb` fixture and bounded tests
- Kept unsupported `.xlsb` metadata explicit with warnings instead of claiming parity:
  - data validations
  - calculation settings
  - style metadata
  - shared-formula metadata

## Validation
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && just bazel-lock-update`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && just bazel-lock-check`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

## Queue Update
- Later plan review re-sequenced the queue so `EXCEL-OFFLINE-T2` became the active next task and `EXCEL-OFFLINE-T1` stayed gated on concrete consumer proof against current workbook-marker coverage
