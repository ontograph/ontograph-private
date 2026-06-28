# Excel Pivot Report Metadata Implementation Plan

## Status
Completed on 2026-06-28.

`P3-A` and `P3-B` are closed in `excel.extract_powerquery_queries`. `P2` is now also closed as a standalone offline metadata tool.

## Goal
Add a read-only offline tool:

`excel.inspect_pivot_report_metadata`

The tool answers the current reopen-gate questions:
- which PivotTable maps to which cache id
- which pivot cache is OLAP / Data Model backed
- which worksheet source range, source name, or connection id backs a PivotTable
- whether stored MDX exists in package data

## Non-Goals
- No live Excel, COM, ADO, DAX execution, or workbook mutation.
- No PivotTable creation, refresh, layout editing, slicer editing, chart creation, or formatting.
- No claim that a PivotTable consumes one specific Power Query unless the package proves it.
- No broad rewrite of `inspect_workbook_with_display_path`; OntoIndex marks that path high blast radius.
- No graph extraction changes in the first slice.

## Donor Evidence
Use ideas from:
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Import/Managed/ManagedPivotReportMetadataExtractor.cs`
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Import/Managed/PivotReportMetadata.cs`
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Migration/Detectors/Pivot/PivotCacheDetector.cs`
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Migration/Detectors/Pivot/PivotTableInventory.cs`
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Migration/Detectors/Pivot/PivotCacheInventory.cs`

Use sample proof from:
- `tmp/excel/samples/Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm`

Do not copy the donor stack. Reuse the DTO shape and package-reading route only.

## Owner Boundary
Preferred owner:
- new module `ontocode-rs/ext/excel/src/pivot_report_metadata.rs`
- wire through `ontocode-rs/ext/excel/src/extension.rs`
- tool constant/import coverage in `ontocode-rs/ext/excel/src/tests.rs`

Allowed shared helper extraction:
- only if `powerquery_extract.rs` relationship-target parsing would otherwise be duplicated in several places
- keep any helper private to `ext/excel`

Avoid:
- adding fields to `inspect_workbook` in the first implementation
- touching `inspect_workbook_with_display_path` unless fresh impact review proves the change is smaller than a standalone tool

## Output Shape
Top-level result:
- `mode`
- `path`
- `pivot_table_count`
- `pivot_cache_count`
- `pivot_tables`
- `pivot_caches`
- `warnings`

Pivot table record:
- `name`
- `worksheet_name`
- `part_path`
- `range_ref`
- `cache_id`
- `source_type`
- `source_name`
- `source_range`
- `connection_id`
- `connection_name`
- `connection_type`
- `olap`
- `data_model`
- `stored_mdx_preview`
- `stored_mdx_truncated`
- `row_fields_sample`
- `column_fields_sample`
- `data_fields_sample`
- `page_fields_sample`
- `warnings`

Pivot cache record:
- `cache_id`
- `source_type`
- `source_name`
- `source_range`
- `connection_id`
- `connection_name`
- `connection_type`
- `olap`
- `data_model`
- `cache_fields_sample`
- `warnings`

## Bounds
- Max PivotTables: 64.
- Max pivot caches: 64.
- Max fields per role sample: 32.
- Max cache fields sample: 64.
- Max stored MDX preview: 4096 chars.
- Max warnings: 32.
- Reuse existing workbook XML entry byte limits where possible.

If a bound is exceeded, truncate and emit a warning. Do not silently omit.

## Implementation Tasks

### P2-0: Proof And Owner Check
Status: completed.

Actions:
- Run OntoIndex impact on the target owner before edits.
- Confirm `inspect_workbook_with_display_path` remains high blast radius.
- Confirm a standalone tool module has lower write scope.
- Read donor extractor and sample package shape.

Exit:
- completed: OntoIndex confirmed `inspect_workbook_with_display_path` stays `CRITICAL`
- completed: the first implementation stayed a standalone offline tool, not an `inspect_workbook` expansion
- completed: sample package proof from `Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm` justified continuing

### P2-1: Parse Workbook Pivot Cache Routing
Status: completed.

Implement package readers for:
- `xl/workbook.xml` `pivotCache cacheId` plus `r:id`
- `xl/_rels/workbook.xml.rels` cache definition relationship targets
- `xl/pivotCache/pivotCacheDefinition*.xml` `cacheSource`
- `worksheetSource` sheet/name/ref
- external `connectionId`
- cache fields

Tests:
- worksheet-source cache
- external/Data Model cache
- missing relationship stays fail-closed with warning

### P2-2: Parse PivotTable Definitions
Status: completed.

Implement package readers for:
- worksheet relationship parts that point to `pivotTables/pivotTable*.xml`
- pivot table name
- worksheet name
- location ref
- cache id
- row/column/data/page fields when package data exposes them

Tests:
- PivotTable maps to cache id and worksheet location
- field samples are bounded
- malformed pivot table XML produces warning, not panic

### P2-3: Tool Wiring
Status: completed.

Add:
- `excel.inspect_pivot_report_metadata`
- path validation consistent with existing Excel path tools
- schema types with `JsonSchema`, `Serialize`, `Deserialize`
- installed-tool coverage

Tests:
- relative path resolves against turn cwd
- output is bounded and deterministic

### P2-4: Real-Sample Fixture Coverage
Status: completed.

Use synthetic fixtures for stable unit tests. Optionally add a small derived fixture only if it can be kept compact.

Do not commit a large donor workbook.

Required proof:
- completed: sample-backed package read against `Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm`
- completed: closure note records the covered cache-id, cache-source, worksheet-rel, pivot-table-location, and Data Model connection fields

## Validation
From `/opt/demodb/_workfolder/ontocode/ontocode-rs`:

```bash
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fmt
```

Then run scoped OntoIndex diff verification with changed files and executed test:

```text
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
```

## Stop Conditions
- Stop if package evidence cannot map PivotTable -> cache id -> source.
- Stop if implementing the tool requires live Excel or COM.
- Stop if the only remaining value is generic formatting/chart/table mutation.
- Stop if `inspect_workbook_with_display_path` becomes the only viable owner; re-review the blast radius first.

## Exact Reopen Gate After Closure
After this tool lands, reopen only for:
- field/measure/MDX details that are proven present in package data but not surfaced
- graph/report-candidate consumers that use this metadata
- live PivotTable/DAX operations under a separate accepted `excel-live` owner
