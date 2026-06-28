# Excel Row 041 Synthetic Positive Fixture

## Date

2026-06-27

## Scope

Fallback continuation for row `041` after evidence-pack prep.

## Decision

Created one tiny synthetic positive workbook to prove the clean direct-ref-to-existing-name rewrite mechanics without reopening implementation.

## Artifact

`tmp/excel/generated/row041-positive-minimal.xlsx`

## Verified Facts

- `excel.inspect_workbook` reads the workbook as `.xlsx`
- no external links are present
- `excel.inspect_sheet_formulas` reports workbook-scoped name `SalesData=Data!$A$1:$A$3`
- `excel.inspect_sheet_formulas` reports formulas:
  - `Main!B2 = SUM(Data!$A$1:$A$3)`
  - `Main!B3 = AVERAGE(Data!$A$1:$A$3)`

## Companion Artifacts

- `EXCEL_ROW041_SYNTHETIC_POSITIVE_MAPPING.json`
- `EXCEL_ROW041_SYNTHETIC_EXPECTED_DRY_RUN.json`
- `EXCEL_ROW041_SYNTHETIC_POSITIVE_CASE.md`

## Status Outcome

This closes the missing mechanics proof.

Row `041` still does not have the preferred real-workbook user-demand proof, so production reopen remains gated unless the synthetic fixture is explicitly accepted as sufficient for the next prototype step.
