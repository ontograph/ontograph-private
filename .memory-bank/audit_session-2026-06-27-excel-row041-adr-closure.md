# Excel Row 041 ADR Closure

## Date

2026-06-27

## Scope

Bounded manager-loop continuation for `.memory-bank/EXCEL_ROW041_NAMED_RANGE_REWRITE_EXTERNAL_COMPANION_PROJECT_PLAN.md`.

## Decision

Closed the only active row `041` task by recording `ADR_EXCEL_NAMED_RANGE_REWRITE_CONTRACT.md`.

The accepted shape is:

- `041A` is read-only dry-run only
- offline `ontocode-rs/ext/excel` remains the owner
- explicit user-authored mapping is required
- any apply path stays optional `041B`
- any apply owner stays outside offline `ext/excel`

## OntoIndex And Code Evidence

- current Excel owner remains offline inspection in `ontocode-rs/ext/excel`
- `excel.inspect_sheet_formulas` already exposes formula inventory, workbook calculation flags, defined-name metadata, and external-link markers
- no live/native workbook mutation owner was accepted from current repo evidence

## No-Dispatch Result

No further implementation dispatch is valid today.

## Exact Reopen Gate

Reopen only when all of these exist:

- one real workbook that proves direct references should become named ranges
- one explicit user story for why dry-run review is needed
- one user-authored mapping file with `formula_targets`, `from_ref`, `to_name`, `scope_expectation`, `sheet_name` when needed, `max_replacements_per_formula`, `reference_mode`, and `all_or_nothing`

If apply is requested after dry-run, reopen separately with:

- explicit proof that dry-run alone is insufficient
- explicit approval for a live/native mutation owner outside offline `ext/excel`

## Follow-On Status

The next design or implementation step is blocked until the reopen gate is satisfied. No tracker rewrite beyond this closure is justified without new workbook evidence.
