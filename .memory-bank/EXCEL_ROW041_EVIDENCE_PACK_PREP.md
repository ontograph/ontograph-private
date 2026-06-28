# Excel Row 041 Evidence Pack Preparation

## Status

Preparation-only. This note does not reopen implementation by itself.

## Date

2026-06-27

## Purpose

This note turns the row `041` reopen contract into concrete local artifact choices.

The goal is not to reopen code prematurely. The goal is to separate:

- local workbooks that are good blocker fixtures now
- local workbooks that are only partial positive candidates
- the exact missing evidence that still blocks `041A`

## Current Row 041 Constraint

Row `041A` needs one real workbook that proves a direct worksheet reference should be rewritten to an existing defined name.

The key word is `existing`.

Local inspection confirms that the repo already has workbooks with:

- formulas
- defined names
- sheet-scoped and workbook-scoped names
- hidden names
- external-link cases
- `#REF!` cases
- R1C1 mode cases

But the current local samples still do not prove the full positive reopen case:

- one exact formula target
- one exact direct reference inside that formula
- one exact existing defined name that resolves to the same logical target
- one concrete user reason to rewrite that direct reference to the name

Until that exact tuple exists, row `041` stays evidence-blocked.

## Local Workbook Review

### Partial positive candidate

`tmp/excel/samples/Заявка на мыло.xlsm`

Why it is useful:

- formulas present in visible worksheets
- workbook has existing defined names
- no external links
- simple read-only shape compared with the larger dashboard workbooks

Observed evidence:

- formula examples include `СУП!C2`, `VLOOKUP(MONTH(E55),Справочник!$B$2:$C$13,2,FALSE)`, and `YEAR(E55)`
- workbook-level defined names include `Месяц=Справочник!$C$2:$C$13` and `Отдел=Справочник!$A$2:$A$27`
- formulas in the same workbook already use `Месяц` and `Отдел` as names in some cells

Why it is still insufficient:

- current local inspection did not prove one exact direct-ref formula that can be rewritten to one exact existing name without inventing a new name
- for example, `VLOOKUP(MONTH(E55),Справочник!$B$2:$C$13,2,FALSE)` does not match an existing defined name for the full lookup table
- `СУП!C2` also does not currently map to an existing defined name from the inspected workbook metadata

Verdict:

- best partial positive candidate
- still not enough to reopen `041A`

### External-link blocker fixture

`tmp/excel/samples/Dynamic Dashboard Illustration V1.1.xlsm`

Why it is useful:

- workbook has formulas and many defined names
- workbook has external links
- workbook contains complex name targets including `INDIRECT(...)`, `[1]...` external references, and `#REF!`

Observed evidence:

- `excel.inspect_workbook` reports `has_external_links: true`
- formulas include direct refs such as `Inputs!S9`
- formulas also include named-range usage such as `Sales_Per`, `Product`, `Period`, and `Position_Range`
- defined names include external targets and formula-based definitions

Verdict:

- excellent blocker workbook
- use for `external-link-blocked`, `string-literal-ambiguity`, and complex-name holdouts
- not a first positive reopen workbook

### Scope collision and broken-name blocker fixture

`tmp/excel/samples/Automatically_Create_PowerPoint_From_Excel.xlsm`

Why it is useful:

- same textual name appears with workbook scope and sheet scope
- several names target `#REF!`

Observed evidence:

- `_fnt1` appears both sheet-local and workbook-global
- `_fntref*` names include broken `#REF!` targets

Verdict:

- use for `scope-mismatch`, `ambiguous-sheet-scope`, and broken-target blocker cases
- not a first positive reopen workbook

### R1C1 and large named-range corpus blocker fixture

`tmp/excel/samples/Табель Макрос.xlsm`

Why it is useful:

- workbook has many defined names
- workbook calculation metadata includes `refMode=\"R1C1\"`

Verdict:

- use as an explicit out-of-scope or fail-closed workbook for the first slice
- helps prove that the first reopen should stay A1-only

## Recommended Evidence Pack

### Synthetic positive mechanics fixture

Prepared:

- workbook: `tmp/excel/generated/row041-positive-minimal.xlsx`
- mapping: `EXCEL_ROW041_SYNTHETIC_POSITIVE_MAPPING.json`
- expected dry-run: `EXCEL_ROW041_SYNTHETIC_EXPECTED_DRY_RUN.json`

What it proves:

- existing workbook-scoped name `SalesData=Data!$A$1:$A$3`
- direct-ref formulas `SUM(Data!$A$1:$A$3)` and `AVERAGE(Data!$A$1:$A$3)`
- exact clean rewrite pair exists without external links or scope ambiguity

What it does not prove:

- real user demand
- real production workbook pressure

### Positive slot

Use one of these:

- preferred: a real user workbook that already contains one exact direct-ref to existing-name rewrite candidate
- fallback: the prepared tiny custom workbook above to prove one clean positive mapping, plus one real workbook to justify the demand

Do not pretend the existing local samples already satisfy this requirement. They do not.

### Negative slots

Use these immediately:

- `Dynamic Dashboard Illustration V1.1.xlsm` for external-link and complex-name blockers
- `Automatically_Create_PowerPoint_From_Excel.xlsm` for scope collision and `#REF!` blockers
- `Табель Макрос.xlsm` for R1C1 fail-closed proof

## Required Artifact Set

To reopen `041A`, prepare all of these:

- one positive workbook
- one mapping JSON file
- one expected dry-run JSON file
- one short note explaining why the rewrite is needed

Template files are provided next to this note:

- `EXCEL_ROW041_MAPPING_TEMPLATE.json`
- `EXCEL_ROW041_EXPECTED_DRY_RUN_TEMPLATE.json`

They are schema placeholders only. They are not proof by themselves.

## Exact Missing Evidence

The smallest still-missing artifact is:

- one workbook plus one mapping entry where `from_ref` points at a direct worksheet reference used in a real formula and `to_name` points at an already existing defined name that resolves to the same target with unambiguous scope

Without that, row `041` remains blocked for the right reason.

## Recommendation

The laziest path that still works is:

1. keep the blocker workbooks from local samples
2. keep the prepared synthetic positive workbook for mechanics proof
3. ask for one real user workbook with one intended rewrite
4. reopen `041A` only after the mapping file is concrete and either real-workbook demand is present or the synthetic proof is explicitly accepted for prototype work

Do not open code first. The repo does not need a dry-run engine before the evidence tuple exists.
