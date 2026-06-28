# Excel Broader P3 Sample Evidence

## Summary
Reviewed `tmp/excel/samples/` to find real workbook-package examples that can justify any broader `P3` reopen beyond the already-landed bounded extractor extension.

## Findings
- `tmp/excel/samples/Unleashing-Power-Query-for-Data-Scientists-Reshape-Merge-Clean-at-Scale.xlsx` proves a bounded offline worksheet-load routing chain exists:
  - query connection
  - `queryTable`
  - `tableType="queryTable"`
  - worksheet range via `ExternalData_*` defined names
- `tmp/excel/samples/Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm` proves a bounded Data Model / pivot-consumer routing chain exists:
  - query connections
  - `x15:modelTable`
  - Data Model connection
  - pivot cache definitions
  - PivotTable cache ids and locations

## Decision
- Keep `P2` closed.
- Keep broader `P3` split into two exact reopen gates:
  - `P3-A`: worksheet-table / worksheet-range load routing
  - `P3-B`: Data Model / pivot-consumer routing
- Do not dispatch either without an explicit bounded consumer question.

## Tracking impact
- `EXCEL_OFFLINE_P2_P3_EVIDENCE_MATRIX.md` now records both sample-backed proofs.
- `EXCEL_OFFLINE_FOLLOWON_PROPOSALS_IMPLEMENTATION.md` now carries the split `P3-A` / `P3-B` reopen gates.
