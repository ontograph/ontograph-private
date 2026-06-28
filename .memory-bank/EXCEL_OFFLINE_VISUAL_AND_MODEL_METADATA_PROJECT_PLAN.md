# Excel Offline Visual And Model Metadata Project Plan

## Status

Proposed only. No task is active yet.

This file captures only genuinely new offline `ext/excel` follow-on proposals after the already landed Excel surfaces.

It explicitly excludes:
- anything already implemented in `ontocode-rs/ext/excel`
- anything already tracked in the current Excel offline review, metadata, migration, formula, or live-companion plans
- write, mutation, COM, DAX execution, ADO, screenshot, or workbook-refresh work

## Current Source Reality

Already present in `ontocode-rs/ext/excel`:
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

Already tracked elsewhere and not reopened here:
- `excel.inspect_workbook_comments_notes`
- `excel.inspect_sheet_validation_rules`
- all live Excel / COM / DAX / write flows

Current remaining donor-backed gaps not yet implemented or explicitly planned:
- direct chart inventory
- direct Data Model inventory
- direct conditional-format rule inventory
- direct slicer inventory
- direct sparkline inventory

## Goal

Add the next useful offline review metadata without:
- a second workbook parser stack
- a second Power Query / connection parser stack
- a second pivot or table parser stack
- any live companion inside offline `ext/excel`

The proposals below stay read-only and package-driven.

## Donor Basis

Primary donor evidence:
- `tmp/excel/in2sql_dotNet_addin`

Supporting donor evidence:
- `tmp/excel/mcp-server-excel`
- `tmp/excel/excel-mcp-server`
- `tmp/excel/negokaz-excel-mcp-server`

Retained donor themes:
- bounded visual-review metadata
- bounded semantic model metadata
- explicit unsupported states instead of guessed output

Rejected donor themes for this plan:
- chart creation
- conditional-format mutation
- slicer mutation
- workbook design editing
- DAX execution
- Data Model refresh or materialization

## Scope Rules

- Stay inside `ontocode-rs/ext/excel`
- Keep all work offline and read-only
- Reuse existing workbook, connection, pivot, and table owners first
- Prefer one small owner-local module per new surface
- Extract one shared helper only when it removes clear duplicate package parsing
- Keep unsupported workbook semantics explicit
- Fail closed instead of degrading to guessed metadata

## Proposed Order

1. `EXCEL-VISMODEL-D1` `excel.inspect_workbook_chart_metadata`
2. `EXCEL-VISMODEL-D2` `excel.inspect_data_model_metadata`
3. `EXCEL-VISMODEL-D3` `excel.inspect_sheet_conditional_formatting`
4. `EXCEL-VISMODEL-D4` `excel.inspect_workbook_slicers`
5. `EXCEL-VISMODEL-D5` `excel.inspect_sheet_sparkline_metadata`

## Open Tasks

### `EXCEL-VISMODEL-D1` `excel.inspect_workbook_chart_metadata`

- **Status**: open
- **Purpose**: expose a bounded offline chart inventory instead of only coarse `has_charts`
- **Why first**:
  - smallest new review gap
  - donor evidence is strong, but current repo has no direct chart inspection surface
  - can stay package-local and read-only
- **Preferred owner**:
  - new owner-local module such as `ontocode-rs/ext/excel/src/workbook_chart_metadata.rs`
  - reuse workbook and sheet part routing from existing inspection owners
  - do not widen `inspect_workbook_with_display_path`
- **Bounded output**:
  - chart count
  - bounded chart summaries
  - `sheet_name`, `chart_type`, `title`
  - anchor cell or drawing location when provable
  - source range hint when provable
  - warnings for unsupported chart carriers or unresolved drawing targets
- **Acceptance**:
  - no image rendering
  - no chart XML dump
  - no chart mutation
  - no guessed source ranges
- **Expected files**:
  - `ontocode-rs/ext/excel/src/workbook_chart_metadata.rs`
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/lib.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`

### `EXCEL-VISMODEL-D2` `excel.inspect_data_model_metadata`

- **Status**: open
- **Purpose**: expose a bounded offline Data Model inventory without reopening live DAX or refresh work
- **Why second**:
  - highest-value semantic gap left in donor evidence
  - current source already exposes partial Data Model hints through Power Query and pivot metadata
  - the missing piece is a direct review tool, not execution
- **Preferred owner**:
  - new owner-local module such as `ontocode-rs/ext/excel/src/data_model_metadata.rs`
  - reuse `powerquery_extract.rs` Data Model load-target hints
  - reuse `pivot_report_metadata.rs` Data Model cache routing where useful
  - do not add DAX or model-write capability
- **Bounded output**:
  - Data Model presence
  - table count and bounded table summaries when provable
  - relationship count and bounded relationship summaries when provable
  - measure and calculated-column counts when provable
  - source connection ids or query-name hints when provable
  - warnings for opaque model carriers
- **Acceptance**:
  - no DAX execution
  - no refresh/materialization
  - no fabricated table or relationship details
  - unsupported model carriers remain warnings
- **Expected files**:
  - `ontocode-rs/ext/excel/src/data_model_metadata.rs`
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/lib.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`

### `EXCEL-VISMODEL-D3` `excel.inspect_sheet_conditional_formatting`

- **Status**: open
- **Purpose**: expose a rule-centric sheet review surface for conditional formatting
- **Why third**:
  - donor evidence is real
  - useful for review and migration, but less central than charts or Data Model metadata
  - should stay narrow to avoid turning into a style engine
- **Preferred owner**:
  - new owner-local module such as `ontocode-rs/ext/excel/src/sheet_conditional_formatting.rs`
  - reuse worksheet part reads from preview/layout owners
- **Bounded output**:
  - rule count
  - bounded rule summaries
  - target ranges
  - rule kind
  - formula or threshold preview when provable
  - priority/order when provable
  - warnings for unsupported rule families
- **Acceptance**:
  - no cell-by-cell style expansion
  - no formatting mutation
  - no visual simulation of rule results
  - unresolved formulas stay explicit
- **Expected files**:
  - `ontocode-rs/ext/excel/src/sheet_conditional_formatting.rs`
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/lib.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`

### `EXCEL-VISMODEL-D4` `excel.inspect_workbook_slicers`

- **Status**: open
- **Purpose**: expose a bounded offline slicer inventory for pivot/table review
- **Why fourth**:
  - real donor gap
  - depends on table and pivot surfaces already landed
  - narrower than reopening any visual authoring flow
- **Preferred owner**:
  - new owner-local module such as `ontocode-rs/ext/excel/src/workbook_slicers.rs`
  - reuse table and pivot metadata where bindings are already provable
- **Bounded output**:
  - slicer count
  - bounded slicer summaries
  - slicer name
  - source table or pivot binding when provable
  - field name when provable
  - destination sheet when provable
  - warnings for unsupported slicer carriers
- **Acceptance**:
  - no slicer mutation
  - no interactive filter state editing
  - no guessed bindings
- **Expected files**:
  - `ontocode-rs/ext/excel/src/workbook_slicers.rs`
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/lib.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`

### `EXCEL-VISMODEL-D5` `excel.inspect_sheet_sparkline_metadata`

- **Status**: open
- **Purpose**: expose a bounded offline sparkline inventory for sheet review
- **Why last**:
  - real but weaker review value than charts, Data Model metadata, conditional formatting, or slicers
  - should only open after the earlier higher-signal metadata gaps
- **Preferred owner**:
  - new owner-local module such as `ontocode-rs/ext/excel/src/sheet_sparkline_metadata.rs`
  - reuse worksheet XML routing from preview/layout owners
- **Bounded output**:
  - sparkline-group count
  - bounded group summaries
  - target ranges
  - source ranges
  - sparkline type
  - key group options when provable
  - warnings for unsupported carriers
- **Acceptance**:
  - no rendering
  - no per-cell visual output
  - no sparkline mutation
- **Expected files**:
  - `ontocode-rs/ext/excel/src/sheet_sparkline_metadata.rs`
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/lib.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`

## Non-Goals

- `excel.inspect_workbook_comments_notes`
- `excel.inspect_sheet_validation_rules`
- any live Excel / COM / ADO / DAX tool
- chart creation
- conditional-format authoring
- slicer editing
- sparkline editing
- screenshot or HTML rendering

## Validation

From `/opt/demodb/_workfolder/ontocode/ontocode-rs`:

```bash
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fmt
```

Use scoped `gn_verify_diff` with explicit changed files if repo-wide verification is noisy in the current dirty worktree.

## Exact No-Dispatch Gates

- do not reopen comments/notes or validation-rules here; those already have separate gates
- do not open DAX execution, Data Model materialization, or workbook writes in this plan
- do not accept any proposal that turns visual metadata into rendering or mutation
