# Excel Offline Migration Artifacts Project Plan

## Status
In progress.

Landed:
- `EXCEL-MIGRATION-A1` `excel.generate_workbook_migration_manifest`
  - new owner-local module: `ontocode-rs/ext/excel/src/workbook_migration_manifest.rs`
  - wired through `ontocode-rs/ext/excel/src/lib.rs`
  - validated registration and relative-path tool execution in `ontocode-rs/ext/excel/src/tests.rs`
  - added dedicated fail-closed limit coverage in `ontocode-rs/ext/excel/src/workbook_migration_manifest_tests.rs`

Validation evidence for `A1`:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fmt`
- scoped `gn_verify_diff` passed with explicit changed files because repo-wide verification was noisy in a dirty worktree

OntoIndex note:
- the index did not resolve the newly added manifest symbol during follow-on planning, so direct source inspection is authoritative until the repo index is refreshed

This plan starts after the currently closed offline Excel slices:

- formula AST parsing
- formula SQL preview and readiness
- worksheet formula dependency scan
- SliderQuery package generation
- Power Query extraction and review bundle
- pivot report metadata
- workbook graph
- VBA source extraction

## Active Next Task
`EXCEL-MIGRATION-A3` `excel.inspect_vba_project_metadata`

## Goal
Add the next useful offline-only Excel tools by composing the existing `ontocode-rs/ext/excel` evidence instead of reopening live Excel, COM, ADO, DAX, or workbook-write scope.

The next real gap is not more raw extraction. The gap is review-ready migration artifacts built from the extraction and inspection surfaces that already exist.

## Donor Basis
- Primary donor: `tmp/excel/in2sql_dotNet_addin`
- Supporting evidence only:
  - `tmp/excel/vba-mcp-server`
  - `tmp/excel/excel-mcp-server`
  - `tmp/excel/mcp-server-excel`

Useful donor themes retained:
- deterministic workbook/package manifests
- formula-to-SQL lineage sidecars
- review-only cross-sheet SQL pipeline planning
- deeper offline VBA project metadata

Rejected donor themes for this plan:
- workbook create/write/update tools
- formatting/chart/pivot mutation
- DAX execution or table materialization
- formula application or recalculation suppression
- VBA write/delete/create tools

## Scope Rules
- Stay inside `ontocode-rs/ext/excel`
- Keep all work offline and read-only
- Reuse the existing workbook, formula, Power Query, pivot, graph, and VBA owners first
- Prefer one composition tool over several duplicate wrappers
- Do not add a second parser path
- Do not widen `inspect_workbook_with_display_path` unless a later review proves that is the smaller safe owner
- Do not reopen `excel-live`, COM, ADO, DAX, or workbook mutation

## Already Covered, Do Not Rebuild
- `read_data_validation_rules` is already covered by `excel.read_sheet_preview`
- formula syntax/readiness is already covered by:
  - `excel.inspect_sheet_formulas`
  - `excel.inspect_formula_sql_readiness`
- formula dependency scan already exists:
  - `excel.scan_sheet_formulas_dependency`
- SQL package generation already exists:
  - `excel.generate_slider_query_package`
- VBA source extraction already exists:
  - `excel.extract_vba_modules`

## Proposed Order
1. `EXCEL-MIGRATION-A1` `excel.generate_workbook_migration_manifest`
2. `EXCEL-MIGRATION-A2` formula-to-SQL lineage sidecar inside the manifest owner unless that shape becomes clearly worse
3. `EXCEL-MIGRATION-A3` `excel.inspect_vba_project_metadata`
4. `EXCEL-MIGRATION-A4` `excel.inspect_formula_cte_pipeline`

## Open Tasks

### `EXCEL-MIGRATION-A1` `excel.generate_workbook_migration_manifest`
- **Status**: completed
- **Shape**: new offline read-only packaging tool
- **Purpose**: compose the existing offline evidence into one deterministic review artifact for a workbook
- **Preferred owner**:
  - new owner-local module under `ontocode-rs/ext/excel/src/`
  - wire through `extension.rs` and `lib.rs`
- **Reuse inputs, do not duplicate reads when avoidable**:
  - `inspect_workbook`
  - `inspect_sheet_formulas`
  - `inspect_formula_sql_readiness`
  - `scan_sheet_formulas_dependency`
  - `extract_powerquery_queries`
  - `inspect_pivot_report_metadata`
  - `extract_vba_modules`
- **Minimum output**:
  - `manifest.json`
  - workbook summary
  - per-sheet formula summary
  - SQL readiness summary
  - dependency summary
  - Power Query summary
  - pivot summary
  - VBA summary
  - warnings and unsupported sections
- **Why first**:
  - smallest missing integration layer
  - highest user value without new semantics
  - composes existing tools instead of inventing more parsing
- **Donor basis**:
  - `tools/WorkbookArtifactExtractor/.../CanonicalBundleWriter`
  - `tools/WorkbookArtifactExtractor/.../ConnectionsExportWriter`
  - `docs/adr/0069-powerquery-export-performance-enrichment.md`
- **Acceptance**:
  - emits one deterministic folder or file set
  - does not claim import/apply capability
  - references unsupported areas explicitly instead of hiding them
  - does not widen workbook inspection output
- **Expected files**:
  - one new owner-local module
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/lib.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
  - `ontocode-rs/ext/excel/src/workbook_migration_manifest_tests.rs`

### `EXCEL-MIGRATION-A2` formula-to-SQL lineage sidecar
- **Status**: completed
- **Shape**:
  - prefer additive extension of `EXCEL-MIGRATION-A1`
  - only split into a standalone tool if the manifest becomes too large or too slow
- **Purpose**: emit a compact artifact linking source formula cells or regions to generated SQL preview objects and readiness families
- **Why second**:
  - the source evidence already exists
  - the missing piece is stable cross-artifact linkage, not more parsing
- **Donor basis**:
  - `docs/adr/0078-source-formula-lineage-for-generated-sql.md`
  - `docs/EXCEL_FORMULA_TO_SQL_MIGRATION_STRATEGY.md`
- **Minimum output**:
  - stable source id
  - sheet and cell or range
  - formula family
  - readiness state
  - SQL preview identity or text reference
  - blocker reasons when not ready
- **Acceptance**:
  - compact join artifact, not a second full formula dump
  - no fabricated lineage for unsupported formulas
  - no new SQL planner logic
- **Expected files**:
  - same owner as `A1` unless split is justified
  - tests covering ready and blocked formulas
  - fail-closed coverage for degraded composed sections and invalid output bundle paths

### `EXCEL-MIGRATION-A3` `excel.inspect_vba_project_metadata`
- **Status**: active
- **Shape**: new offline read-only inspection tool
- **Purpose**: expose project-level VBA metadata that `excel.extract_vba_modules` does not currently summarize
- **Bounded target fields**:
  - project presence
  - module inventory by type
  - project references when provable
  - forms or document-module presence
  - warnings for opaque or unsupported binary details
- **Why this instead of `vba_backup_modules`**:
  - backup is mostly duplication of the current source extractor
  - project metadata is the real missing review surface
- **Donor basis**:
  - `VbaProjectReader`
  - `VBA_LIBRARY_ANALYSIS.md`
- **Acceptance**:
  - reads only package-contained VBA project data
  - stays fail-closed on unparsed binary sections
  - does not add VBA write/edit surfaces
- **Expected files**:
  - one new owner-local module
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/lib.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`

### `EXCEL-MIGRATION-A4` `excel.inspect_formula_cte_pipeline`
- **Status**: open, later
- **Shape**: review-only planning tool
- **Purpose**: turn the existing sheet formula DAG and current SQL-capable families into staged CTE candidates and blockers
- **Why last**:
  - highest semantic risk
  - depends on the earlier manifest and lineage evidence being stable first
- **Donor basis**:
  - `docs/EXCEL_FORMULA_TO_SQL_MIGRATION_STRATEGY.md`
  - `docs/adr/0075-excel-formula-sql-sliderquery-implementation.md`
- **Acceptance**:
  - outputs candidate stages only
  - no claim of full workbook translation
  - explicit blockers for unsupported formulas, circular references, report-layout dependencies, and ambiguous ranges
- **Expected files**:
  - likely one new owner-local module
  - may reuse `scan_sheet_formulas_dependency` and readiness outputs

## Non-Goals
- `excel.run_dax_evaluate`
- `excel.materialize_dax_to_table`
- `excel.write_cells_recalc_suppressed`
- `excel.apply_formula`
- `excel.validate_formula_syntax`
- `excel.vba_backup_modules` as a first-class next tool
- chart, range-write, table-write, or workbook-create operations

## Validation
From `/opt/demodb/_workfolder/ontocode/ontocode-rs`:

```bash
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fmt
```

Use scoped OntoIndex diff verification if repo-wide scope is noisy.

## Exact Reopen Gates For Deferred Donor Ideas
- open live DAX or worksheet-write tools only under the separate `excel-live` owner
- reopen `vba_backup_modules` only if a user needs repo-owned filesystem bundle export rather than JSON/source inspection
- reopen richer formula pipeline translation only after `A1` and `A2` prove the current evidence is still not enough
