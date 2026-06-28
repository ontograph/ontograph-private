# Excel Offline T2 Closure

## Scope
- Closed `EXCEL-OFFLINE-T2` from `EXCEL_OFFLINE_NEXT_TOOLS_PROJECT_PLAN.md`
- Kept the change inside the existing Power Query extraction owner in `ontocode-rs/ext/excel`
- Reused the current `excel.extract_powerquery_queries` output instead of adding a second tool

## What Landed
- Added bounded lexical lint findings to extracted Power Query queries
- Added aggregate `lint_finding_count` to the extraction result
- Kept lint fail-closed and source-first:
  - empty query body
  - missing shared-query definition fallback
  - missing `let`/`in` structure
- Added focused tests for bounded lint findings while keeping clean extraction and corrupted-payload coverage intact

## Validation
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

## Queue Update
- `EXCEL-OFFLINE-T3` is now the active next task
- `EXCEL-OFFLINE-T1` remains pending explicit consumer proof against current workbook-marker coverage
