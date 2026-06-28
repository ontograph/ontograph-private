# Excel P3-B Closure

## Decision
Closed broader `P3-B` inside the existing offline Power Query extractor owner.

## Scope Landed
- `excel.extract_powerquery_queries` now emits bounded `data_model_load_targets` per extracted query.
- Data Model load routing is proven only from `xl/workbook.xml` `x15:modelTable` entries and workbook connections.
- Pivot consumers are reported only as Data-Model-wide consumers when a pivot cache source connection id matches a Data Model connection id.
- The extractor does not claim that a PivotTable consumes a specific query/table unless package evidence proves that narrower relationship.

## Files
- `ontocode-rs/ext/excel/src/powerquery_extract.rs`
- `ontocode-rs/ext/excel/src/tests.rs`
- `.memory-bank/EXCEL_OFFLINE_FOLLOWON_PROPOSALS_IMPLEMENTATION.md`
- `.memory-bank/EXCEL_OFFLINE_P2_P3_EVIDENCE_MATRIX.md`

## Validation
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

`just fix -p ontocode-excel-extension` still reports the pre-existing unrelated `slider_query.rs` `only_used_in_recursion` warning.

## Remaining Gate
No dependency-ready `P3` task remains. `P2` remains gated on a concrete offline consumer asking for detailed pivot-report metadata such as PivotTable cache/source details, OLAP/Data Model backing, source range/name, or stored MDX.
