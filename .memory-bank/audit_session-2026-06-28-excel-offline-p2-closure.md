# Excel Offline P2 Closure

## Status
Closed.

## Scope
Implemented bounded offline pivot report metadata in `ext/excel`:
- new tool `excel.inspect_pivot_report_metadata`
- standalone owner `ontocode-rs/ext/excel/src/pivot_report_metadata.rs`
- no expansion of `inspect_workbook_with_display_path`

## OntoIndex Evidence
- `inspect_workbook_with_display_path`: `CRITICAL` upstream blast radius, 26 impacted, 5 direct callers
- `inspect_workbook`: `MEDIUM` upstream blast radius, 9 impacted, 9 direct test callers
- `ExcelExtension.tools`: `LOW`

Decision:
- keep pivot metadata in a standalone tool/module
- do not widen workbook inspection owners for this slice

## Package Proof
Sample workbook:
- `tmp/excel/samples/Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm`

Verified package-level route:
- `xl/workbook.xml` `pivotCache cacheId` plus `r:id`
- `xl/_rels/workbook.xml.rels` pivot cache definition targets
- worksheet rels to `xl/pivotTables/pivotTable*.xml`
- pivot table `cacheId` and `location ref`
- pivot cache `cacheSource connectionId`
- `xl/connections.xml` Data Model / OLAP markers

That proof was sufficient to continue past `P2-0`.

## Implemented Output
The tool now returns bounded:
- pivot table identity, worksheet, part path, location, cache id
- source type, source name/range
- connection id, name, type
- OLAP and Data Model flags
- stored MDX preview only when package command text looks like MDX
- role field samples
- cache field samples
- fail-closed warnings

## Validation
From `/opt/demodb/_workfolder/ontocode/ontocode-rs`:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - crate tests passed: 124 passed
  - repo-default bench tail continued under shared host contention after the crate test pass
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
  - completed; auto-fixed three local issues in `pivot_report_metadata.rs`
  - remaining warnings were pre-existing in `ext/excel/src/slider_query.rs`
- `CARGO_BUILD_JOBS=8 just fmt`
  - completed

## Remaining Gap
Nothing dependency-ready remains in this offline follow-on slice.

Exact reopen gates:
- surface field, measure, or MDX details only if package data proves they exist and the current tool omits them
- open graph/report consumers only when a concrete downstream offline user exists
- keep live PivotTable/DAX operations in the separate `excel-live` owner
