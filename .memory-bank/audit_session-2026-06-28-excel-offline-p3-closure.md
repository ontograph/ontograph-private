# Excel Offline P3 Closure

## Decision
Closed the narrowed Power Query-owner `P3` follow-on.

## Scope
- Extended `excel.extract_powerquery_queries` inside `ontocode-rs/ext/excel/src/powerquery_extract.rs`.
- Updated focused Excel tests in `ontocode-rs/ext/excel/src/tests.rs`.
- Did not add a new tool, workbook-owner connection subsystem, live Excel path, COM, ADO, DAX, mutation, or pivot metadata extractor.

## Evidence
- Session `019f08c4-9531-7ba1-9918-911b1bc33977` reopened option `1`: Data Model load-target and backing connection-id visibility.
- OntoIndex impact for `extract_powerquery_queries_from_workbook` was `LOW`, with three direct upstream callers.

## Result
- Power Query extraction now returns bounded workbook connection summaries.
- Query rows now report backing workbook connection ids and load-target hints.
- Data Model hints are package-backed only.
- `commandType` is preserved as bounded connection metadata.

## Validation
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

`just fix` reported only the pre-existing `slider_query.rs` recursion warning outside this slice.

## Remaining Gates
- `P2` stays gated until a concrete offline consumer asks one of the sample-proven pivot-report questions.
- Broader `P3` stays gated until a consumer asks beyond the bounded Power Query extractor fields and the answer is still package-backed.
