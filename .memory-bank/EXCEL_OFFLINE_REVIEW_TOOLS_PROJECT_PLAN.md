# Excel Offline Review Tools Project Plan

## Status

No eligible `active_next_task` remains. `EXCEL-REVIEW-T4` stays gated behind real review-gap evidence.

This plan starts after the currently landed offline Excel surfaces in `ontocode-rs/ext/excel`:
- workbook inspection
- sheet preview
- sheet formula inventory and AST
- formula SQL preview and readiness
- worksheet dependency scan
- SliderQuery package generation
- Power Query extraction and review bundle
- workbook connections, tables, and sheet layout metadata
- pivot report metadata
- workbook graph
- migration manifest
- VBA extraction and review metadata

It is intentionally narrow:
- stay inside `ontocode-rs/ext/excel`
- keep all work offline and read-only
- reuse existing parsers and owners
- do not reopen live Excel, COM, DAX, ADO, screenshots, or workbook writes

## Current Source Reality

Already true in current source:
- `extension.rs` shows the offline tool surface is already broad
- `inspect_workbook` reports coarse package markers, including `has_external_links` and `has_comments`
- `inspect_sheet_formulas` already emits bounded `defined_names` and `defined_names_sample`
- `read_sheet_preview` already emits bounded per-sheet dimensions and validation summaries

Current remaining review gaps after donor recheck:
- workbook-level defined-name inventory is still only available indirectly through formula inspection
- external links are only surfaced as coarse presence markers
- workbook-level used-range ergonomics are still thinner than donor review workflows
- comments/notes are only surfaced as coarse presence markers

OntoIndex grounding:
- `extension.rs` is current and confirms the registered owner surface
- retrieval for broad Excel-surface review was weak in this turn, so direct source inspection is authoritative for exact coverage

## Goal

Add the next smallest useful review-only tools without:
- a second workbook parser stack
- a second formula parser stack
- a second workbook graph path
- a live companion inside `ext/excel`

The shortest valid order is:
1. expose workbook defined names directly
2. expose bounded external-link inventory
3. expose workbook-level used-range summaries
4. only then consider comments/notes inventory if a real review workflow still needs it

## Donor Basis

Primary donor:
- `tmp/excel/in2sql_dotNet_addin`

Supporting donor evidence:
- `tmp/excel/mcp-server-excel`
- `tmp/excel/excel-mcp-server`
- `tmp/excel/negokaz-excel-mcp-server`

Retained donor themes:
- read-only named-range visibility
- bounded workbook review metadata
- used-range ergonomics
- explicit unsupported/unknown states instead of guessed behavior

Rejected donor themes for this plan:
- named-range create/update/delete
- workbook writes
- screenshot or HTML rendering
- DAX, Data Model execution, or refresh
- COM-backed VBA backup as an offline owner
- one giant artifact-inventory tool as the first follow-up

## Scope Rules

- Stay inside `ontocode-rs/ext/excel`
- Keep all work offline and read-only
- Reuse existing owners before adding a new top-level parser helper
- Keep `inspect_workbook_with_display_path` coarse unless direct additive extension is clearly safer than a new owner-local tool
- Extract one small shared helper only when it removes duplicate parsing
- Keep unsupported workbook semantics explicit
- Fail closed instead of degrading to guessed metadata

## Proposed Order

1. `EXCEL-REVIEW-T1` `excel.inspect_workbook_defined_names`
2. `EXCEL-REVIEW-T2` `excel.inspect_workbook_external_links`
3. `EXCEL-REVIEW-T3` `excel.inspect_workbook_used_ranges`
4. `EXCEL-REVIEW-T4` `excel.inspect_workbook_comments_notes` only if the first three still leave a real review gap

## Open Tasks

### `EXCEL-REVIEW-T1` `excel.inspect_workbook_defined_names`

- **Status**: complete
- **Active**: no
- **Purpose**: expose workbook-level defined-name inventory directly, without requiring sheet-formula inspection as the discovery surface
- **Why first**:
  - current source already has bounded `DefinedNameSummary`
  - the donor review still points to named-range visibility as a real review need
  - this is the smallest gap with the strongest reuse path
- **Preferred owner**:
  - a new owner-local module such as `ontocode-rs/ext/excel/src/workbook_defined_names.rs`
  - extract one small shared helper from `formula_inspect.rs` only if it cleanly removes duplicate defined-name parsing
  - do not bloat `tool.rs` with parsing logic
- **Bounded output**:
  - workbook defined-name count
  - bounded `defined_names` summaries reusing the existing shape where possible
  - `defined_names_sample`
  - explicit `inventory_truncated`
  - warnings for truncated inventories or unresolved numeric sheet scope
- **Landed shape**:
  - standalone owner-local tool in `ontocode-rs/ext/excel/src/workbook_defined_names.rs`
  - OpenXML `.xlsx` / `.xlsm` only in this stage
  - `.xlsb` defined-name inventory is explicit unsupported output, not degraded name-only metadata
  - malformed workbook XML and invalid `localSheetId` values fail closed
- **Acceptance**:
  - no named-range value materialization
  - no rewrite/apply surface
  - no silent hiding of unsupported targets
  - internal or hidden names remain explicit, not silently dropped
- **Expected files**:
  - `ontocode-rs/ext/excel/src/workbook_defined_names.rs`
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
  - optionally one small shared helper extraction from `ontocode-rs/ext/excel/src/formula_inspect.rs`
- **Reopen gate if blocked**:
  - block only if the existing defined-name parse path cannot be reused or safely extracted without reopening a broader formula owner rewrite

### `EXCEL-REVIEW-T2` `excel.inspect_workbook_external_links`

- **Status**: complete
- **Active**: no
- **Purpose**: expose a bounded external-link inventory instead of only `has_external_links`
- **Why second**:
  - current workbook review and formula SQL paths already treat external links as blockers
  - users still cannot see what was detected without opening raw package data
- **Preferred owner**:
  - a new owner-local module such as `ontocode-rs/ext/excel/src/workbook_external_links.rs`
  - reuse `inspect_workbook` package-part scanning and workbook relationship helpers where possible
  - keep `backend.rs` limited to coarse markers unless one tiny shared helper is enough
- **Bounded output**:
  - external-link count
  - bounded per-part summaries
  - workbook relationship ids and targets when provable
  - bounded nested relationship targets when provable
  - warnings for unsupported or unreadable external-link details
- **Landed shape**:
  - standalone owner-local tool in `ontocode-rs/ext/excel/src/workbook_external_links.rs`
  - OpenXML `.xlsx` / `.xlsm` only in this stage
  - `.xlsb` external-link inventory is explicit unsupported output, not guessed package metadata
  - unsupported detail kinds such as `dde_link` remain explicit warnings
- **Acceptance**:
  - no network access
  - no attempt to dereference targets
  - no guessed lineage into formulas or graph output
  - unsupported link kinds remain warnings, not fabricated metadata
- **Expected files**:
  - `ontocode-rs/ext/excel/src/workbook_external_links.rs`
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
  - optionally a tiny shared helper in `ontocode-rs/ext/excel/src/backend.rs`
- **Reopen gate if blocked**:
  - block only if package parts prove presence but not bounded inventory fields beyond what current markers already expose

### `EXCEL-REVIEW-T3` `excel.inspect_workbook_used_ranges`

- **Status**: complete
- **Active**: no
- **Purpose**: add workbook-level used-range summaries for sheet triage and export planning
- **Why third**:
  - donor evidence is real
  - current `read_sheet_preview` has dimensions, but only one sheet at a time
  - this is useful, but weaker than defined-name and external-link visibility
- **Preferred owner**:
  - a new owner-local module such as `ontocode-rs/ext/excel/src/workbook_used_ranges.rs`
  - reuse current worksheet-dimension parsing from preview for OpenXML sheets
  - use the existing `.xlsb` reader only if it can provide the same bounded dimension data cheaply; otherwise emit explicit `.xlsb` warnings
- **Bounded output**:
  - per-sheet used-range summaries
  - sheet name
  - part path when known
  - range reference when provable
  - bounded warnings for unknown or unsupported sheets
- **Landed shape**:
  - standalone owner-local tool in `ontocode-rs/ext/excel/src/workbook_used_ranges.rs`
  - OpenXML sheets reuse `preview.rs::parse_sheet_dimension`
  - `.xlsb` sheets reuse the existing range-dimension helper instead of faking parity
  - unreadable sheets remain warning-only per-sheet summaries, not fabricated ranges
- **Acceptance**:
  - no full-sheet reads into model output
  - no inferred header typing
  - no cell-level preview duplication
  - no fake parity for `.xlsb` if the dimension path is weaker
- **Expected files**:
  - `ontocode-rs/ext/excel/src/workbook_used_ranges.rs`
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
  - optionally a small helper extraction from `ontocode-rs/ext/excel/src/preview.rs`
- **Reopen gate if blocked**:
  - block only if workbook-level aggregation would duplicate too much preview logic without a safe shared helper

### `EXCEL-REVIEW-T4` `excel.inspect_workbook_comments_notes`

- **Status**: gated
- **Purpose**: expose sheet-level comment or threaded-comment inventory only if the first three tasks still leave a real review gap
- **Why gated**:
  - current source already exposes `has_comments`
  - comments are less central to migration and formula review than defined names, external links, or used ranges
- **Preferred owner**:
  - extend current package-part scanning first
  - add a new tool only if coarse marker output remains too small for a real review workflow
- **Acceptable output if reopened**:
  - sheet-level comment-part summaries
  - classic vs threaded comment counts when provable
  - warnings for unsupported or unresolved ownership details
- **Reopen gate**:
  - current coarse comment markers must be shown insufficient by a real workbook or review workflow
  - do not open this just because a donor server has richer note/comment APIs

## Deferred / Rejected

- `excel.inspect_sheet_validation_rules`
  - keep gated behind proof that current preview-owned `data_validations` are insufficient
- `excel.inspect_workbook_artifact_inventory`
  - defer unless users need one composed umbrella view more than the existing direct tools
- any live/write donor tool
  - keep outside offline `ext/excel`

## Validation

From `/opt/demodb/_workfolder/ontocode/ontocode-rs`:

```bash
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fmt
```

Use scoped `gn_verify_diff` with explicit changed files if repo-wide verification is noisy in the current dirty worktree.

## Exact No-Dispatch Gates

- do not reopen named-range mutation, workbook writes, live Excel, COM, DAX, or screenshots in this plan
- do not add a second formula or workbook parser stack just to expose review metadata
- do not open `comments_notes` until the first three tasks land or fail with evidence
