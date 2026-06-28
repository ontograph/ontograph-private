# Excel Offline T3 Closure

## Scope
- Closed `EXCEL-OFFLINE-T3` from `EXCEL_OFFLINE_NEXT_TOOLS_PROJECT_PLAN.md`
- Kept the change inside `ontocode-rs/ext/excel`
- Reused the existing Power Query extraction owner and added one owner-local packaging module

## What Landed
- Added `excel.generate_powerquery_review_bundle`
- Bundle generation now emits:
  - extracted query source files
  - `reports/lint-summary.json`
  - `reports/lineage-summary.json`
  - `manifest.json`
- Kept optional normalization honest:
  - manifest records `not_implemented`
  - no normalized query files are fabricated

## Validation
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

## Queue Update
- No dependency-ready open task remains in this plan
- `EXCEL-OFFLINE-T1` stays gated on concrete consumer proof against current workbook-marker coverage
- `EXCEL-OFFLINE-T4` stays deferred no-dispatch
