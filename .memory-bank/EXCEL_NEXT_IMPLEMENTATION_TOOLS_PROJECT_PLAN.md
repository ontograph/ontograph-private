# Excel Next Implementation Tools Project Plan

## Status

In progress; `A1` and `A2` are complete, `A3` remains gated.

This file captures the next donor-backed Excel tool queue after the already landed offline surfaces in `ontocode-rs/ext/excel`.

It is intentionally narrow:
- reuse current owners
- stay offline and read-only
- do not reopen live Excel, COM, ADO, DAX, or workbook-write scope

## Current Source Reality

Already present in `ontocode-rs/ext/excel`:
- workbook inspection
- sheet preview
- sheet formula inventory
- formula AST parsing
- formula SQL preview
- formula SQL readiness
- worksheet formula dependency scan
- SliderQuery package generation
- Power Query extraction and review bundle
- pivot report metadata
- workbook graph
- named-range rewrite dry-run
- VBA source extraction
- workbook migration manifest

Current follow-on reality from source and worktree:
- `excel.inspect_vba_project_metadata` is the next smallest useful offline gap
- `excel.inspect_formula_cte_pipeline` is the next strongest formula follow-on after AST, dependency scan, SQL preview, and readiness
- live/write donor ideas remain separate-owner work, not `ext/excel` work

## Goal

Add the next useful Excel tools without creating a second parser path, a second workbook metadata owner, or a live companion inside offline `ext/excel`.

The shortest valid path is:
1. finish the missing offline VBA project metadata surface
2. add one review-only formula CTE pipeline surface
3. only then extend composed migration artifacts if a real gap remains

## Donor Basis

Primary donor:
- `tmp/excel/in2sql_dotNet_addin`

Supporting donors:
- `tmp/excel/vba-mcp-server`
- `tmp/excel/excel-mcp-server`
- `tmp/excel/mcp-server-excel`

Useful retained donor themes:
- COM-free VBA project inspection
- deterministic review artifacts
- review-only cross-sheet formula staging
- explicit blocker reporting instead of best-effort conversion

Rejected donor themes for this plan:
- DAX execution
- Data Model mutation
- workbook writes
- VBA write/create/delete
- formula apply/recalc flows
- COM-dependent backup/export tools as first-class offline owners

## Scope Rules

- Stay inside `ontocode-rs/ext/excel`
- Keep all work offline and read-only
- Extend existing owners before adding a new top-level tool
- Prefer composition over duplicate extractors
- Keep unsupported syntax and unsupported workbook semantics explicit
- Fail closed instead of degrading to guessed behavior

## Proposed Order

1. `EXCEL-NEXT-A1` `excel.inspect_vba_project_metadata`
2. `EXCEL-NEXT-A2` `excel.inspect_formula_cte_pipeline`
3. `EXCEL-NEXT-A3` manifest or bundle extension only if `A1` and `A2` still leave a real review gap

## Open Tasks

### `EXCEL-NEXT-A1` `excel.inspect_vba_project_metadata`

- **Status**: complete
- **Purpose**: expose project-level VBA metadata that `excel.extract_vba_modules` does not summarize
- **Preferred owner**:
  - `ontocode-rs/ext/excel/src/vba_project_metadata.rs`
  - reuse parsing helpers from `ontocode-rs/ext/excel/src/vba_extract.rs`
- **Bounded output**:
  - project presence
  - code page when provable
  - module counts by broad type
  - bounded module-name samples
  - provable reference-kind counts
  - warnings for opaque parser limits
- **Acceptance**:
  - no VBA write/edit surface
  - no fake per-reference detail strings
  - no fake form/document/class distinction beyond what the current parser proves
  - fail-closed warnings stay explicit
- **Why first**:
  - smallest real missing review surface
  - strong donor evidence
  - reuses existing VBA parse path
- **Landed evidence**:
  - added offline metadata tool and owner-local parser reuse
  - made bounded module-name samples explicit with truncation flags and warnings
  - corrected tool-specific path-validation errors for `excel.inspect_vba_project_metadata`
  - validated with `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - `gn_verify_diff` passed for `vba_extract.rs`, `vba_project_metadata.rs`, `vba_project_metadata_tests.rs`, and `tests.rs`

### `EXCEL-NEXT-A2` `excel.inspect_formula_cte_pipeline`

- **Status**: complete
- **Purpose**: turn the current worksheet formula DAG plus SQL readiness facts into review-only staged CTE candidates and blockers
- **Preferred owner**:
  - one owner-local module under `ontocode-rs/ext/excel/src/`
  - reuse `scan_sheet_formulas_dependency`
  - reuse `inspect_formula_sql_readiness`
  - reuse current formula inventory identities where possible
- **Bounded output**:
  - stage order
  - candidate CTE groups
  - blocked formulas
  - reasons for block
  - cycle summary
  - warnings when stage grouping is ambiguous
- **Acceptance**:
  - review-only output
  - no workbook-complete SQL claim
  - no execution/import path
  - explicit blockers for unsupported formulas, cycles, ambiguous ranges, and report-layout dependencies
- **Why second**:
  - the inputs already exist
  - this adds planning visibility, not another parser
  - it is the strongest remaining offline formula tool
- **Landed evidence**:
  - added a review-only CTE pipeline tool that composes existing readiness and dependency outputs instead of adding a second parser or planner
  - emits deterministic stage order, per-stage candidate CTE groups, blocked formulas, blocked dependency propagation, and cycle summaries
  - warns when a single stage mixes multiple readiness families and the grouping stays heuristic
  - validated with `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - `just fix -p ontocode-excel-extension` passed; remaining warning is unrelated pre-existing `clippy::only_used_in_recursion` in `slider_query.rs`
  - `CARGO_BUILD_JOBS=8 just fmt` passed

### `EXCEL-NEXT-A3` composed artifact extension only if needed

- **Status**: gated
- **Purpose**: extend `excel.generate_workbook_migration_manifest` or an existing bundle owner only if `A1` and `A2` still leave a concrete review gap
- **Preferred owner**:
  - existing manifest owner first
  - existing review-bundle owner second
- **Examples of acceptable follow-on shapes**:
  - add `vba_project_metadata` summary into the manifest
  - add `formula_cte_pipeline` summary into the manifest
  - emit one additional deterministic sidecar file from the existing owner
- **Reopen gate**:
  - only if a user or fixture-backed review workflow proves the current JSON outputs are still not enough
  - current stop condition for this loop: no dependency-ready task remains until that proof exists

## Non-Goals

- `excel.run_dax_evaluate`
- `excel.materialize_dax_to_table`
- `excel.write_cells_recalc_suppressed`
- `excel.apply_formula`
- `excel.validate_formula_syntax`
- `excel.vba_backup_modules` as a first-class offline tool
- chart/range/table/workbook mutation
- a new live companion inside `ext/excel`

## Validation

From `/opt/demodb/_workfolder/ontocode/ontocode-rs`:

```bash
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fmt
```

Use scoped `gn_verify_diff` with explicit changed files if repo-wide verification is noisy in the current dirty worktree.

## Exact No-Dispatch Gates

- Live DAX, Data Model materialization, or recalc-suppressed writes open only under a separate accepted live owner
- `vba_backup_modules` opens only if JSON inspection is insufficient and repo-owned filesystem export is explicitly required
- broader formula planning opens only if `excel.inspect_formula_cte_pipeline` proves too small for a real review workflow
