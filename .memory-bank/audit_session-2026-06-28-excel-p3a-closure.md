# Excel P3-A Closure

## Summary
Closed the first broader offline Power Query follow-up by extending `excel.extract_powerquery_queries` with bounded worksheet-load routing.

## Implemented
- Added per-query `worksheet_load_targets` output in `ontocode-rs/ext/excel/src/powerquery_extract.rs`
- Routed targets from existing package evidence only:
  - workbook connection ids from `xl/connections.xml`
  - `xl/queryTables/queryTable*.xml`
  - `xl/tables/table*.xml` and table relationship parts
  - `xl/workbook.xml` `ExternalData_*` defined names
- Added focused fixture coverage in `ontocode-rs/ext/excel/src/tests.rs`

## Boundaries kept
- No new tool surface
- No workbook-owner pivot expansion
- No live Excel, COM, ADO, or mutation work
- Unprovable routes remain omitted rather than guessed

## Validation
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

## Next gate
- `P2` stays gated on an explicit pivot-report consumer question
- remaining broader `P3-B` stays gated on an explicit Data Model / pivot-consumer routing question
