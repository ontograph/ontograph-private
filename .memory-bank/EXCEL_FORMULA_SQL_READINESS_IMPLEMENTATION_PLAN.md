# Excel Formula SQL Readiness Implementation Plan

## Status

Prepared and landed on 2026-06-28.

Code is landed for the bounded offline slice.

This document now records the completed implementation for the smallest valid offline SQL follow-up after Phase 4C in `EXCEL_LEFTIES_IMPLEMENTATION_PROJECT_PLAN.md`.

## Goal

Add a new read-only offline tool:

`excel.inspect_formula_sql_readiness`

The tool should answer one bounded question for a selected worksheet:

- which formulas are already eligible for the existing review-only SQL preview families
- which formulas are blocked
- the exact blocker reasons for blocked formulas

This is a readiness and triage tool, not a new SQL planner.

## Why This Slice

Current `ext/excel` already has:

- AST-backed parse metadata on `SheetFormulaSummary.parse`
- review-only SQL preview on `SheetFormulaSummary.sql_preview`
- bounded workbook graph proof
- bounded exact lookup and aligned aggregate preview logic

What is still missing is a compact migration-oriented answer that tells an agent or reviewer:

- how much of a worksheet is SQL-ready right now
- which formulas fit the current planner families
- where the real blockers are

That gives more value than opening more parser or live-workbook scope.

## Non-Goals

- No new SQL emission family.
- No new graph edges.
- No workbook-wide packaging in the first slice.
- No live Excel, COM, ADO, DAX, `Formula2`, or workbook mutation.
- No widening of `inspect_workbook_with_display_path`.
- No first-slice widening of `inspect_sheet_formulas` output unless a later implementation review proves that a standalone tool is strictly worse.

## Current Evidence

Current source already proves the core inputs:

- `ontocode-rs/ext/excel/src/formula_inspect.rs`
- `ontocode-rs/ext/excel/src/formula_sql.rs`
- `ontocode-rs/ext/excel/src/workbook_graph.rs`
- `ontocode-rs/ext/excel/src/tool.rs`

Relevant OntoIndex findings from this planning pass:

- `inspect_workbook_with_display_path` remains `CRITICAL` blast radius and is not an acceptable owner for this slice
- `inspect_sheet_formulas_with_display_path` is `HIGH` blast radius and should stay a source/composition dependency, not the primary place for new migration-specific output
- `ExcelExtension.tools` is `LOW` risk for additive tool registration

That points to a small standalone composition tool instead of expanding the existing workbook or formula inventory surfaces.

## Donor Basis

Use donor ideas only for classification and review workflow shape:

- `tmp/excel/in2sql_dotNet_addin/docs/EXCEL_FORMULA_TO_SQL_MIGRATION_STRATEGY.md`
- `tmp/excel/in2sql_dotNet_addin/docs/EXCEL_PARSING_TOOLS_REVIEW.md`

Do not copy donor runtime or broad migration infrastructure.

Reuse current repo owners first.

## Owner Boundary

Preferred owner:

- new module `ontocode-rs/ext/excel/src/formula_sql_readiness.rs`

Expected supporting files:

- `ontocode-rs/ext/excel/src/extension.rs`
- `ontocode-rs/ext/excel/src/lib.rs`
- `ontocode-rs/ext/excel/src/tests.rs`
- optionally `ontocode-rs/ext/excel/src/tool.rs` for shared schema types if that remains the smallest fit

Reuse, do not fork:

- `inspect_sheet_formulas_with_display_path` for bounded formula inventory
- `plan_formula_sql_preview` in `formula_sql.rs` for current SQL-family readiness

Avoid:

- adding more behavior to `inspect_workbook_with_display_path`
- introducing a second parser path
- reparsing formula text from scratch in the new tool

## Tool Shape

### Input

- `path`
- `sheet`
- optional `max_formulas`

Keep the selector shape aligned with `excel.inspect_sheet_formulas`.

### Top-Level Output

- `path`
- `sheet`
- `max_formulas_applied`
- `formula_count`
- `readiness_counts`
- `blocked_reason_counts`
- `ready_formulas`
- `blocked_formulas`
- `truncated`
- `warnings`

### Readiness Counts

Bounded counts for:

- `scalar_row_local`
- `exact_lookup`
- `aligned_aggregate`
- `blocked`
- `malformed`
- `unsupported`

### Ready Formula Item

- `reference`
- `formula`
- `family`
- `sql_expression`
- `parse_state`
- `warnings`

### Blocked Formula Item

- `reference`
- `formula`
- `family_hint`
- `parse_state`
- `blocker_reasons`
- `warnings`

### Classification Rules

Classify only against families that current code already supports:

- `scalar_row_local`
- `exact_lookup`
- `aligned_aggregate`

Everything else stays blocked or unknown.

Do not invent a broader family taxonomy in the first slice.

## Design Rules

- Stay fail-closed.
- Reuse current `sql_preview` state and blocker reasons as authority where possible.
- Do not claim workbook-level migration readiness from one-sheet evidence.
- Do not claim a formula is ready if `sql_preview.state != review_only`.
- Do not silently collapse unsupported or malformed formulas into generic blocked counts without preserving exact reasons.
- Keep output bounded and review-friendly rather than dumping the full `InspectSheetFormulasResult`.

## Implementation Tasks

### R0: Preflight And Owner Check

Status: completed for planning.

Actions:

- confirm current owners and blast radius with OntoIndex
- confirm this should be a standalone additive tool
- confirm the new tool can compose existing formula inventory and SQL preview owners

### R1: Result Types And Classification Helper

Status: completed.

Implement:

- result DTOs
- readiness count DTO
- bounded blocker-frequency summary
- a small classification helper that maps existing formula preview evidence into:
  - `scalar_row_local`
  - `exact_lookup`
  - `aligned_aggregate`
  - `unknown`

The helper should prefer:

- current top-level AST function name when present
- existing `sql_preview.state`
- existing blocker reasons

### R2: Readiness Inspection Flow

Status: completed.

Implement the tool flow:

1. validate relative workbook path
2. call `inspect_sheet_formulas_with_display_path`
3. summarize formulas into readiness counts
4. collect bounded ready and blocked samples
5. collect bounded blocker-frequency counts
6. return warnings explicitly

Do not add a new workbook-reading path.

### R3: Tool Wiring

Status: completed.

Add:

- new tool registration in `extension.rs`
- module export in `lib.rs`
- installed-tool coverage in `tests.rs`

### R4: Focused Fixture Coverage

Status: completed.

Prefer current synthetic worksheet fixtures already used by:

- formula inspection tests
- formula SQL tests
- workbook graph tests

Required coverage:

- same-row scalar arithmetic reports `scalar_row_local`
- exact `VLOOKUP(..., FALSE|0)` reports `exact_lookup`
- exact `XLOOKUP` reports `exact_lookup`
- exact aligned `SUMIFS` reports `aligned_aggregate`
- unsupported dynamic-array or external-link formulas stay blocked with exact reasons
- malformed formulas count as malformed and do not appear ready

## Acceptance

- new tool stays offline and read-only
- output is bounded and deterministic
- no new SQL generation logic is introduced
- readiness families reflect only already-landed planner capabilities
- blocked formulas preserve exact blocker reasons from current parse/planner evidence
- the tool does not widen `inspect_workbook` or claim workbook-wide migration readiness

Accepted on 2026-06-28 with:

- standalone offline tool `excel.inspect_formula_sql_readiness`
- additive wiring through `extension.rs` and `lib.rs`
- focused fixture coverage for scalar, exact lookup, aligned aggregate, unsupported spill syntax, and malformed formulas
- direct tool-path coverage for relative path resolution against turn cwd

## Validation

From `/opt/demodb/_workfolder/ontocode/ontocode-rs`:

```bash
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension
CARGO_BUILD_JOBS=8 just fmt
```

Completed on 2026-06-28.

If repo-wide diff verification is noisy, use scoped OntoIndex verification with the changed files and executed test:

```text
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
```

## Stop Conditions

- Stop if the tool cannot classify readiness without duplicating or rewriting the current SQL planner logic.
- Stop if the only viable path requires widening `inspect_workbook_with_display_path`.
- Stop if current `SheetFormulaSummary.sql_preview` is too weak to distinguish ready formulas from blockers without first reopening broader SQL-planner work.
- Stop if the implementation starts drifting into workbook-wide migration bundling instead of one-sheet readiness inspection.

## Exact Reopen Gate After Closure

After this tool lands, reopen only for one of these:

- workbook-wide migration packaging that composes this readiness output with Power Query, pivot, and VBA review artifacts
- a fresh approved fixture pack proving broader SQL families beyond `scalar_row_local`, `exact_lookup`, and `aligned_aggregate`
- a separate accepted live-owner contract for apply/import work
