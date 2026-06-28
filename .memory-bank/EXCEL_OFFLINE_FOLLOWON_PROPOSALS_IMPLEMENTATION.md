# Excel Offline Follow-On Proposals Implementation

## Status
`P1` is completed with code and scoped validation. The bounded `P3` extractor extension is now completed from session `019f08c4-9531-7ba1-9918-911b1bc33977`. Broader `P3-A` and `P3-B` are now also completed as additive routing extensions in the same owner.

Current reality:
- `EXCEL-OFFLINE-T0`, `EXCEL-OFFLINE-T2`, and `EXCEL-OFFLINE-T3` are closed.
- follow-on `P1` is now closed as an owner-local extension of `excel.generate_powerquery_review_bundle`
- `EXCEL-OFFLINE-T1` is still gated on explicit consumer proof.
- `EXCEL-OFFLINE-T4` is still deferred no-dispatch.
- `P3` is now closed only for the narrowed Power Query owner shape below; broader workbook-owner connection tooling remains gated
- one smallest-unblock evidence step is now closed: a sample-backed `P2`/`P3` matrix exists and tightens the remaining reopen gates
- session `019f08c4-9531-7ba1-9918-911b1bc33977` then supplied the missing consumer selection: open the Data Model load-target / backing connection-id question through option `1`
- fresh sample review under `tmp/excel/samples/` now proves two broader `P3` evidence-backed sub-slices exist:
  - worksheet-table load destinations
  - Data Model / pivot-consumer routing
- broader `P3-A` is now closed inside `excel.extract_powerquery_queries`
- broader `P3-B` is now closed inside `excel.extract_powerquery_queries` for bounded Data Model table routing and Data-Model-wide pivot consumers

## Goal
Capture the smallest real next implementation proposals from `tmp/excel` without reopening live Excel, COM, ADO, DAX, or workbook-write scope.

## Decision
Prefer the lowest-blast existing owner first:
1. extend the current Power Query bundle owner
2. only then consider workbook-owner additions
3. keep live companion donor ideas deferred

## Owner Fit
- `inspect_workbook_with_display_path` remains a high-risk owner because it fans into preview, formula inspection, CSV export, and workbook inspection surfaces
- `extract_powerquery_queries_from_workbook` remains a low-risk owner and is the safest place for the next offline follow-up

## Evidence
- local evidence is recorded in `EXCEL_OFFLINE_P2_P3_EVIDENCE_MATRIX.md`
- that evidence uses real workbook packages already present under `tmp/excel/`
- result: both `P2` and `P3` still have provable package-level metadata gaps
- `Unleashing-Power-Query-for-Data-Scientists-Reshape-Merge-Clean-at-Scale.xlsx` proves a bounded worksheet-load routing path exists from query connection to queryTable to table to worksheet range
- `Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm` proves a bounded Data Model / pivot-consumer routing path exists from query connection to model table to pivot cache / PivotTable surfaces
- `P3-A` no longer needs a reopen gate because the worksheet-load route now lands in extractor output
- `P3-B` no longer needs a reopen gate because bounded Data Model table routing and Data-Model-wide pivot consumers now land in extractor output

## Proposal P1: Normalized Power Query Copies In Review Bundle
**Status**: completed

**Shape**:
- no new top-level tool
- extend `excel.generate_powerquery_review_bundle`
- keep source `.m` files unchanged
- emit normalized copies into `normalized_m/`
- report normalization artifacts explicitly in `manifest.json`

**Why this first**:
- shortest real follow-up
- already has donor proof
- stays inside the existing low-blast Power Query owner
- fixes the one explicit honesty gap where the manifest currently reports normalization as `not_implemented`

**Owner**:
- `ontocode-rs/ext/excel/src/powerquery_review_bundle.rs`
- optionally a small owner-local helper module if needed

**Donor basis**:
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/PostWork/Normalization/MQueryNormalizePostWork.cs`

**Implementation slice**:
- normalize line endings to `\n`
- trim trailing whitespace per line
- trim trailing blank lines
- preserve the original extracted query files
- write normalized copies only when bundle generation runs
- record per-query normalized artifact paths in the manifest

**Expected files**:
- `ontocode-rs/ext/excel/src/powerquery_review_bundle.rs`
- `ontocode-rs/ext/excel/src/tests.rs`
- optionally one small new helper module under `ontocode-rs/ext/excel/src/`

**Acceptance**:
- bundle output includes `normalized_m/*.m`
- manifest no longer claims `not_implemented` for this bounded copy-only normalization mode
- missing normalization output is explicit and never silent
- no overwrite mode
- no semantic M rewriting

**Validation**:
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

**Closure**:
- `excel.generate_powerquery_review_bundle` now emits `normalized_m/*.m` copy artifacts
- `manifest.json` now reports per-query `normalized_source_path`
- normalization status is no longer `not_implemented`; it is an explicit copy-artifact mode
- source query files remain the extractor-owned originals; normalization only affects the copied artifacts
- scoped crate validation passed after the code landed

## Proposal P2: `excel.inspect_pivot_report_metadata`
**Status**: completed

Detailed plan:
- `EXCEL_PIVOT_REPORT_METADATA_IMPLEMENTATION_PLAN.md`

**Shape**:
- new offline read-only tool
- standalone owner-local implementation in `ontocode-rs/ext/excel/src/pivot_report_metadata.rs`

**Why standalone**:
- `inspect_workbook_with_display_path` remains high blast radius
- existing `inspect_workbook` pivot markers do not answer the sample-proven cache/source questions
- a small owner-local module keeps pivot metadata additive and fail-closed

**Owner**:
- preferred: `ontocode-rs/ext/excel/src/pivot_report_metadata.rs`
- wire through `ontocode-rs/ext/excel/src/extension.rs` only after parser proof is covered
- avoid expanding `inspect_workbook_with_display_path` unless fresh impact review proves that route is safer

**Donor basis**:
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Import/Managed/ManagedPivotReportMetadataExtractor.cs`

**Bounded target fields**:
- report identity
- cache id and cache links
- worksheet source range when present
- OLAP / Data Model flags
- stored MDX only when actually present in package data
- warnings when package data does not contain the requested detail

**Closure**:
- OntoIndex kept the work out of `inspect_workbook_with_display_path`
- the tool now reports bounded PivotTable identity, cache id, cache source, connection metadata, OLAP/Data Model flags, field samples, and fail-closed warnings
- sample-backed package proof confirmed the tool against `Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm`

**Exact reopen gate**:
- only reopen for package-proven field/measure/MDX details not currently surfaced
- do not reopen for live refresh, DAX execution, or workbook mutation in offline `ext/excel`

## Proposal P3: Detailed Workbook Connections In Power Query Extraction
**Status**: completed for the bounded extractor-extension slice

**Shape**:
- no new top-level tool
- extend `excel.extract_powerquery_queries` in the existing Power Query owner
- add bounded workbook connection metadata that answers which extracted queries load to Data Model and which connection ids back them
- do not introduce a second workbook connection subsystem

**Why now**:
- session `019f08c4-9531-7ba1-9918-911b1bc33977` reopened option `1` after the evidence matrix
- `extract_powerquery_queries_from_workbook` remains the low-blast owner for the first useful subset
- the current output already has `connection_name`, `location`, and `command_preview`; the missing fields are connection id, Data Model flag, command type, and load-target hints

**Owner**:
- `ontocode-rs/ext/excel/src/powerquery_extract.rs`
- `ontocode-rs/ext/excel/src/tests.rs`

**Donor basis**:
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/Connections/Application/PivotSourceService.cs`
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/Connections/Excel/ExcelPivotTableGateway.cs`
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/Connections/README.md`

**Implemented bounded fields**:
- workbook connection id
- connection name
- connection type
- command preview, bounded like the existing extractor output
- command type, bounded like the existing extractor output
- query-name hints when provable from `Location=...`, `Query - ...`, or Data Model `command=...`
- Data Model flag when the package exposes it
- per-query `load_target_hint` with explicit `unknown`, `workbook_connection`, or `data_model`
- per-query query-to-connection-id mapping when the existing matching rules can prove it
- warnings when inventory or connection-id lists exceed bounded output caps

**Acceptance**:
- `excel.extract_powerquery_queries` reports bounded connection metadata without changing workbook inspection output
- tests cover at least one Mashup query connection and one Data Model connection
- unsupported or missing package relationships produce warnings instead of inferred claims
- no live Excel, COM, ADO, DAX, mutation, or workbook-owner pivot expansion

**Closure**:
- `excel.extract_powerquery_queries` now emits a top-level bounded workbook connection inventory
- extracted queries now expose `workbook_connection_ids` and `load_target_hint`
- workbook connection summaries now preserve bounded `commandType` metadata
- the extractor stays fail-closed: unsupported or unprovable routing remains `unknown`
- scoped validation passed with:
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

**Remaining gap / exact broader reopen gate**:
- exact worksheet-table versus connection-only load destinations are now implemented in the extractor output as bounded worksheet load targets
- Data Model table routing and Data-Model-wide pivot consumers are now implemented in extractor output as bounded `data_model_load_targets`
- what remains outside `P3` is deeper PivotTable semantics:
  - exact table/query-specific PivotTable usage, measures, fields, MDX, and OLAP metadata are not inferred from Data-Model-wide pivot-cache links
- any deeper follow-up must stay fail-closed and should challenge workbook-owner scope before any new surface

**P3-A closure**:
- `excel.extract_powerquery_queries` now emits bounded `worksheet_load_targets` per extracted query
- routing is proven from existing package parts only:
  - workbook query connection id
  - `xl/queryTables/queryTable*.xml`
  - `xl/tables/table*.xml` plus table relationship parts
  - `xl/workbook.xml` `ExternalData_*` defined names
- the new output stays fail-closed:
  - no worksheet target is inferred without a provable connection id route
  - unresolved relationships simply omit the target instead of degrading into guessed table metadata
- scoped validation passed with:
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

**P3-B closure**:
- `excel.extract_powerquery_queries` now emits bounded `data_model_load_targets` per extracted query
- routing is proven from existing package parts only:
  - `xl/workbook.xml` `x15:modelTable` entries
  - `xl/connections.xml` query and Data Model connection ids
  - `xl/workbook.xml` pivot cache relationship ids
  - `xl/_rels/workbook.xml.rels`
  - `xl/pivotCache/pivotCacheDefinition*.xml`
  - `xl/pivotTables/pivotTable*.xml`
- pivot consumers are reported as Data-Model-wide consumers behind a Data Model connection id, not as unsupported claims that a PivotTable consumes only one query output
- the new output stays fail-closed:
  - no Data Model target is emitted without a query-to-model-table route
  - no pivot consumer is emitted without a pivot cache source connection id that matches a Data Model connection
  - unresolved package relationships simply omit the target or consumer instead of guessing
- scoped validation passed with:
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`

## Deferred Live Companion Ideas
These remain out of scope for offline `ext/excel` work:
- `excel.run_dax_evaluate`
- `excel.materialize_dax_to_table`
- `excel.write_cells_recalc_suppressed`
- `excel.vba_backup_modules`

Reason:
- donor evidence for these comes from live COM / Data Model / workbook-mutation servers, not the current offline owner

## Recommended Order
1. No dependency-ready offline follow-on task remains in this file
2. Keep deeper Power Query consumer attribution closed unless a package-level field proves a specific PivotTable consumes a specific Data Model table/query rather than the model as a whole
3. Route all DAX / mutation / live workbook asks to a future `excel-live` path instead of extending offline `ext/excel`

## Exact Next Step
No active next task remains in this follow-on file.

Exact reopen gates:
- `P2`: package-proven field/measure/MDX detail or a new bounded downstream offline consumer
- deeper Power Query to PivotTable attribution: a concrete offline consumer supplies package evidence that identifies table/query-specific pivot usage, not just Data-Model-wide pivot-cache consumption
