# Excel Offline Visual And Model Metadata Implementation Plan

## Status

Proposed. No task is active yet.

This plan implements all new offline-only follow-on proposals that are not already landed and not already tracked in the current Excel plans.

Companion proposal source:
- `EXCEL_OFFLINE_VISUAL_AND_MODEL_METADATA_PROJECT_PLAN.md`

## Goal

Add these read-only offline tools inside `ontocode-rs/ext/excel`:

1. `excel.inspect_workbook_chart_metadata`
2. `excel.inspect_data_model_metadata`
3. `excel.inspect_sheet_conditional_formatting`
4. `excel.inspect_workbook_slicers`
5. `excel.inspect_sheet_sparkline_metadata`

## Hard Boundaries

- Stay inside `ontocode-rs/ext/excel`
- Keep all work offline and read-only
- Reuse existing workbook/package readers before adding new parsing helpers
- Do not reopen live Excel, COM, ADO, DAX execution, workbook refresh, or writes
- Do not add rendering, screenshots, HTML workbook views, or style-matrix expansion
- Do not widen `inspect_workbook_with_display_path` unless fresh impact review proves it is safer than a standalone owner-local tool

## Current Source Baseline

Already landed and not part of this plan:
- workbook inspection
- sheet preview
- sheet formula inventory
- formula AST parsing
- formula SQL preview
- formula SQL readiness
- formula CTE pipeline
- worksheet formula dependency scan
- SliderQuery package generation
- Power Query extraction and review bundle
- pivot report metadata
- workbook connections
- workbook tables
- sheet layout metadata
- workbook graph
- named-range rewrite dry-run
- workbook migration manifest
- VBA extraction
- VBA project metadata
- workbook defined names
- workbook external links
- workbook used ranges

Already gated elsewhere and not reopened here:
- `excel.inspect_workbook_comments_notes`
- `excel.inspect_sheet_validation_rules`

## Donor Evidence

Primary donor:
- `tmp/excel/in2sql_dotNet_addin`

Supporting donors:
- `tmp/excel/mcp-server-excel`
- `tmp/excel/excel-mcp-server`
- `tmp/excel/negokaz-excel-mcp-server`

Retained donor value:
- bounded visual metadata
- bounded semantic model metadata
- explicit unsupported states

Rejected donor value:
- chart creation
- conditional-format editing
- slicer editing
- DAX execution
- Data Model refresh/materialization

## Implementation Order

1. `D1` charts first because it is the smallest new review gap.
2. `D2` Data Model second because it is the highest-value remaining semantic gap.
3. `D3` conditional formatting third because it is useful but should stay narrow.
4. `D4` slicers fourth because it depends on current table/pivot surfaces.
5. `D5` sparklines last because it is lower-value review metadata.

## Task Plan

### `D1` `excel.inspect_workbook_chart_metadata`

- **Status**: open
- **Preferred owner**:
  - `ontocode-rs/ext/excel/src/workbook_chart_metadata.rs`
- **Reuse first**:
  - workbook/sheet routing from existing inspection owners
  - drawing and relationship-part resolution patterns already used in pivot/table/external-link parsing
- **Minimum output**:
  - `chart_count`
  - bounded `charts`
  - per-chart `sheet_name`, `chart_type`, `title`
  - anchor cell or drawing location when provable
  - source-range hint when provable
  - `warnings`
- **Acceptance**:
  - no rendering
  - no chart XML dump
  - no mutation
  - no guessed source ranges
- **Expected files**:
  - `workbook_chart_metadata.rs`
  - `extension.rs`
  - `lib.rs`
  - `tests.rs`

### `D2` `excel.inspect_data_model_metadata`

- **Status**: open
- **Preferred owner**:
  - `ontocode-rs/ext/excel/src/data_model_metadata.rs`
- **Reuse first**:
  - Data Model load-target hints from `powerquery_extract.rs`
  - cache/Data Model routing from `pivot_report_metadata.rs`
  - workbook connection metadata where source identities are already proven
- **Minimum output**:
  - `data_model_present`
  - bounded `tables`
  - bounded `relationships`
  - measure count when provable
  - calculated-column count when provable
  - source connection ids or query-name hints when provable
  - `warnings`
- **Acceptance**:
  - no DAX execution
  - no refresh/materialization
  - no fabricated table or relationship detail
  - opaque carriers remain warnings
- **Expected files**:
  - `data_model_metadata.rs`
  - `extension.rs`
  - `lib.rs`
  - `tests.rs`

### `D3` `excel.inspect_sheet_conditional_formatting`

- **Status**: open
- **Preferred owner**:
  - `ontocode-rs/ext/excel/src/sheet_conditional_formatting.rs`
- **Reuse first**:
  - worksheet XML reads from preview/layout owners
  - existing bounded-text and warning patterns
- **Minimum output**:
  - `rule_count`
  - bounded `rules`
  - target ranges
  - rule kind
  - formula or threshold preview when provable
  - priority/order when provable
  - `warnings`
- **Acceptance**:
  - no cell-by-cell expansion
  - no formatting simulation
  - no mutation
  - unresolved formulas stay explicit
- **Expected files**:
  - `sheet_conditional_formatting.rs`
  - `extension.rs`
  - `lib.rs`
  - `tests.rs`

### `D4` `excel.inspect_workbook_slicers`

- **Status**: open
- **Preferred owner**:
  - `ontocode-rs/ext/excel/src/workbook_slicers.rs`
- **Reuse first**:
  - current table inventory
  - current pivot metadata
  - workbook relationship routing helpers where already proven
- **Minimum output**:
  - `slicer_count`
  - bounded `slicers`
  - slicer name
  - source table or pivot binding when provable
  - field name when provable
  - destination sheet when provable
  - `warnings`
- **Acceptance**:
  - no slicer editing
  - no interactive state output
  - no guessed bindings
- **Expected files**:
  - `workbook_slicers.rs`
  - `extension.rs`
  - `lib.rs`
  - `tests.rs`

### `D5` `excel.inspect_sheet_sparkline_metadata`

- **Status**: open
- **Preferred owner**:
  - `ontocode-rs/ext/excel/src/sheet_sparkline_metadata.rs`
- **Reuse first**:
  - worksheet XML routing from preview/layout owners
  - existing bounded metadata patterns
- **Minimum output**:
  - `sparkline_group_count`
  - bounded `sparkline_groups`
  - target ranges
  - source ranges
  - sparkline type
  - key group options when provable
  - `warnings`
- **Acceptance**:
  - no rendering
  - no per-cell visual output
  - no mutation
- **Expected files**:
  - `sheet_sparkline_metadata.rs`
  - `extension.rs`
  - `lib.rs`
  - `tests.rs`

## Shared Validation

For each task:
- run OntoIndex impact on the target owner before edits
- keep changes additive and owner-local
- add focused synthetic fixture coverage in `tests.rs` or dedicated sibling test files
- fail closed on unsupported carriers

From `/opt/demodb/_workfolder/ontocode/ontocode-rs`:

```bash
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fmt
```

Use scoped `gn_verify_diff` with explicit changed files if repo-wide verification is noisy.

## Stop Conditions

- stop if a task requires live Excel, COM, ADO, DAX execution, or mutation
- stop if the only remaining value is rendering or style expansion
- stop if the package cannot prove the bounded metadata without guessed behavior
- stop and re-review if a standalone module is no longer lower blast radius than extending an existing owner

## Exact No-Dispatch Gates

- do not reopen comments/notes or validation-rules in this plan
- do not add any chart, slicer, conditional-format, or sparkline write path
- do not add DAX execution or Data Model materialization here
