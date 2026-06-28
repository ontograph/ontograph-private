# Excel Row 041 Synthetic Positive Case

## Status

Prepared. This is a mechanics-only positive fixture.

## Workbook

`tmp/excel/generated/row041-positive-minimal.xlsx`

## What It Proves

This fixture proves the exact row `041A` rewrite shape on a tiny clean workbook:

- workbook-scoped defined name already exists
- target formulas still use a direct worksheet reference
- the direct reference and defined name resolve to the same target
- no external links are present
- formulas are plain A1 formulas

Observed workbook facts from the existing Excel tools:

- defined name: `SalesData=Data!$A$1:$A$3`
- formula `Main!B2`: `SUM(Data!$A$1:$A$3)`
- formula `Main!B3`: `AVERAGE(Data!$A$1:$A$3)`

This yields one clean rewrite mapping:

- `Data!$A$1:$A$3` -> `SalesData`

Expected rewrites:

- `SUM(Data!$A$1:$A$3)` -> `SUM(SalesData)`
- `AVERAGE(Data!$A$1:$A$3)` -> `AVERAGE(SalesData)`

## Synthetic User Story

Replace repeated direct references to the same data range with an existing named range so summary formulas stay shorter and remain stable if the backing range is moved or reviewed by non-authors.

## Limits

This fixture proves mechanics only.

It does not replace the preferred reopen evidence:

- one real user workbook
- one real user reason for the rewrite
- one concrete mapping file authored from that workbook

Use this fixture to validate the dry-run contract shape, not to pretend production demand has already been proven.
