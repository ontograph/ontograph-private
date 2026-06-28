# Excel Offline Metadata Tools Project Plan

## Status

Current eligible scope complete. `EXCEL-META-C4` remains gated.

Active next task: none

This is the next offline follow-on plan after the currently landed Excel review and migration surfaces in `ontocode-rs/ext/excel`.

It is intentionally narrow:
- stay inside `ontocode-rs/ext/excel`
- keep all work offline and read-only
- reuse existing parsers and owners
- do not reopen live Excel, COM, DAX, ADO, or workbook-write scope

## Current Source Reality

Already present in `ontocode-rs/ext/excel`:
- workbook inspection
- sheet preview
- selected-sheet validation summaries in preview
- selected-sheet formula inventory
- formula AST parsing
- formula SQL preview
- formula SQL readiness
- formula CTE pipeline
- worksheet formula dependency scan
- SliderQuery package generation
- Power Query extraction and review bundle
- pivot report metadata
- workbook graph
- named-range rewrite dry-run
- VBA extraction and VBA project metadata
- workbook migration manifest

Current remaining gap from source and donor review:
- `excel.inspect_workbook` still only reports coarse connection presence
- table metadata is parsed for graph work, but not exposed as a direct read-only inventory
- sheet layout and review ergonomics are still thin
- a dedicated validation-rules tool is no longer the first missing slice because `excel.read_sheet_preview` already exposes bounded `data_validations`

OntoIndex grounding:
- `extension.rs` is current and shows the existing offline tool surface is already broad
- `powerquery_extract.rs` is current and already owns Power Query-centric connection parsing
- `workbook_graph.rs` was missing from the current OntoIndex file index, so direct source inspection is authoritative for table-owner reuse

## Goal

Add the next useful offline metadata tools without creating:
- a second workbook parser path
- a second connection parser stack
- a second table parser stack
- a live companion inside `ext/excel`

The shortest valid path is:
1. expose workbook connection inventory directly
2. expose workbook table inventory directly
3. expose bounded sheet layout metadata
4. only then consider a separate validation-rules tool if preview summaries prove insufficient

## Donor Basis

Primary donor:
- `tmp/excel/in2sql_dotNet_addin`

Supporting donors:
- `tmp/excel/mcp-server-excel`
- `tmp/excel/excel-mcp-server`
- `tmp/excel/negokaz-excel-mcp-server`

Retained donor themes:
- deterministic offline metadata exports
- workbook connection summaries
- table and load-target review metadata
- read-only layout and validation visibility

Rejected donor themes for this plan:
- live DAX execution
- Data Model mutation
- workbook writes
- recalc suppression
- COM VBA backup/export tools as offline owners
- screenshot, HTML rendering, or style-matrix readback as first-class offline owners

## Scope Rules

- Stay inside `ontocode-rs/ext/excel`
- Keep all work offline and read-only
- Reuse existing owners before adding a new top-level tool
- Do not grow already-large files just because helpers already live there
- Extract one small shared helper when it prevents duplicate parsing
- Keep unsupported workbook semantics explicit
- Fail closed instead of degrading to guessed behavior

## Proposed Order

1. `EXCEL-META-C1` `excel.inspect_workbook_connections`
2. `EXCEL-META-C2` `excel.inspect_workbook_tables`
3. `EXCEL-META-C3` `excel.inspect_sheet_layout_metadata`
4. `EXCEL-META-C4` `excel.inspect_sheet_validation_rules` only if the current preview-owned validation summaries prove insufficient

## Open Tasks

### `EXCEL-META-C1` `excel.inspect_workbook_connections`

- **Status**: complete
- **Purpose**: expose a workbook-level connection inventory that is not limited to the Power Query query list
- **Why now**:
  - `inspect_workbook` only reports `has_connections`
  - `extract_powerquery_queries` already parses connection metadata, but only through a Power Query-shaped output
  - donor evidence for offline connection summaries is strong
- **Preferred owner**:
  - a new owner-local module such as `ontocode-rs/ext/excel/src/workbook_connections.rs`
  - reuse connection parsing from `powerquery_extract.rs`
  - keep `backend.rs` limited to coarse workbook inspection
- **Bounded output**:
  - workbook connection count
  - bounded connection summaries
  - per-connection `id`, `name`, `connection_type`
  - `location`, `command_preview`, `command_type` when provable
  - query-name hint and load-target hint when provable
  - warnings for truncated command text, ambiguous load targets, and unsupported connection kinds
- **Acceptance**:
  - no connection testing
  - no secret extraction
  - no DAX or model execution
  - no mutation of workbook connections
  - no duplicate connection parser
- **Closure evidence**:
  - landed `ontocode-rs/ext/excel/src/workbook_connections.rs` as the direct inventory owner
  - reused the existing connection parser path from `powerquery_extract.rs`
  - added focused connection inventory tests and extension registration coverage
  - validation passed with `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`

### `EXCEL-META-C2` `excel.inspect_workbook_tables`

- **Status**: complete
- **Purpose**: expose a direct read-only inventory of workbook tables for review and migration planning
- **Why second**:
  - table metadata already exists inside graph parsing
  - current users should not need graph output just to inspect tables
  - this is a smaller follow-on than new graph work
- **Preferred owner**:
  - reuse table parsing already present in `workbook_graph.rs`
  - if reuse requires sharing internals, extract one small helper instead of re-parsing tables in a second place
  - do not add table parsing to `tool.rs`
- **Bounded output**:
  - table count
  - bounded table summaries
  - `name`, `alt_name`, `sheet_name`, `part_path`, `range_reference`
  - header-row and totals-row flags when provable
  - bounded column-name sample
  - warnings for unresolved or truncated table metadata
- **Acceptance**:
  - no graph edges
  - no structured-reference dependency extraction
  - no table mutation
  - no second table parser
- **Closure evidence**:
  - landed `ontocode-rs/ext/excel/src/workbook_tables.rs` as the shared table parser and direct inventory owner
  - rewired `workbook_graph.rs` to reuse the extracted table parser instead of keeping a duplicate table parse path
  - added focused table inventory coverage in `ontocode-rs/ext/excel/src/workbook_tables_tests.rs`
  - validation passed with `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - OntoIndex verify-diff remained noisy in the dirty worktree, so direct source inspection plus scoped Excel validation is the authoritative closeout evidence

### `EXCEL-META-C3` `excel.inspect_sheet_layout_metadata`

- **Status**: complete
- **Purpose**: add review-only sheet layout metadata that current preview and inspection outputs do not expose
- **Why third**:
  - useful for workbook review, but weaker than connection and table metadata
  - easy to overbuild into a style engine, so it must stay narrow
- **Preferred owner**:
  - a new owner-local module such as `ontocode-rs/ext/excel/src/sheet_layout_metadata.rs`
  - reuse lightweight XML-entry helpers from preview/backend owners
- **Bounded output**:
  - merged-range count and bounded sample
  - freeze-pane or split-pane summary when provable
  - auto-filter range when provable
  - print-area summary when provable
  - warnings for unsupported or truncated layout carriers
- **Acceptance**:
  - no per-cell style matrix
  - no screenshot or HTML rendering path
  - no formatting mutation
  - no generic workbook-design surface
- **Closure evidence**:
  - landed `ontocode-rs/ext/excel/src/sheet_layout_metadata.rs` as an owner-local offline layout metadata tool
  - reused existing `preview.rs` XML-entry helpers without editing the higher-blast-radius preview or backend owners
  - added focused layout metadata coverage in `ontocode-rs/ext/excel/src/sheet_layout_metadata_tests.rs`
  - validation passed with `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`

### `EXCEL-META-C4` `excel.inspect_sheet_validation_rules`

- **Status**: gated
- **Purpose**: add a sheet-level validation inventory only if the current preview-owned `data_validations` summary proves too small
- **Why gated**:
  - `excel.read_sheet_preview` already exposes bounded validation metadata
  - a separate tool is only justified if a real review workflow needs a rule-centric sheet inventory
- **Preferred owner**:
  - extend current preview-owned validation parsing first
  - extract a small shared helper only if preview and a new tool would otherwise duplicate rule parsing
- **Acceptable output if reopened**:
  - sheet-level validation-rule count
  - bounded rule summaries
  - same bounded rule fields already proven in preview, plus larger range coverage if needed
  - explicit unresolved-formula warnings
- **Reopen gate**:
  - current preview summaries must be shown insufficient by a real workbook or review workflow
  - do not open this just because donor servers have a separate command

## Non-Goals

- `excel.run_dax_evaluate`
- `excel.materialize_dax_to_table`
- `excel.write_cells_recalc_suppressed`
- `excel.vba_backup_modules`
- chart or formatting mutation
- full style readback matrices
- screenshot or HTML workbook rendering
- a second workbook graph path
- a second migration-manifest planner

## Validation

From `/opt/demodb/_workfolder/ontocode/ontocode-rs`:

```bash
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fmt
```

Use scoped `gn_verify_diff` with explicit changed files if repo-wide verification is noisy in the current dirty worktree.

## Exact No-Dispatch Gates

- live DAX, Data Model writes, recalc control, or COM workbook mutation open only under the separate live companion owner
- a standalone validation-rules tool opens only if `excel.read_sheet_preview.data_validations` is proven insufficient
- screenshot, HTML, or style-heavy workbook review surfaces open only if there is a concrete review gap that cannot be met by bounded metadata
