# Excel Row 041 Evidence Pack Prep

## Date

2026-06-27

## Scope

Bounded follow-up after the row `041` contract closure.

## Decision

Prepared the local evidence-pack route instead of reopening implementation.

## What Was Proven

- current repo samples already cover several blocker classes
- `tmp/excel/samples/Заявка на мыло.xlsm` is the best local partial positive candidate because it has formulas, defined names, and no external links
- `tmp/excel/samples/Dynamic Dashboard Illustration V1.1.xlsm` is a strong external-link and complex-name blocker workbook
- `tmp/excel/samples/Automatically_Create_PowerPoint_From_Excel.xlsm` is a strong scope-collision and `#REF!` blocker workbook
- `tmp/excel/samples/Табель Макрос.xlsm` is a strong R1C1 holdout workbook

## What Was Not Proven

No current local sample yet proves the exact positive reopen tuple:

- direct worksheet reference in a real formula
- existing defined name resolving to the same target
- explicit user reason to rewrite that direct ref to that name

## Artifacts Added

- `EXCEL_ROW041_EVIDENCE_PACK_PREP.md`
- `EXCEL_ROW041_MAPPING_TEMPLATE.json`
- `EXCEL_ROW041_EXPECTED_DRY_RUN_TEMPLATE.json`

## Exact Next Reopen Move

Bring one real workbook and one concrete mapping entry.

If that cannot be supplied, the only honest fallback is one tiny custom workbook for the positive case plus the existing local blocker workbooks for negative coverage.
