# Excel Samples Review

Date: 2026-06-26
Scope: `tmp/excel/samples`

## Findings

Implementation-ready:

- `EXCEL-MCP-U1` remains valid and is stronger: `Building-Advanced-Excel-Dashboards-Power-Query-Power-Pivot-and-VBA.xlsm`, `Unleashing-Power-Query-for-Data-Scientists-Reshape-Merge-Clean-at-Scale.xlsx`, `Книга1_(version_1).xlsb`, and `Тепловая карта 4 кв 2024.xlsb` all contain UTF-16 `customXml/item*.xml` parts that current `excel.inspect_workbook` cannot read.
- `EXCEL-MCP-U4` is newly opened: `excel.read_sheet_preview` with `cell_content="values_and_formulas"` drops XML-escaped formula operators from `Dynamic Dashboard Illustration V1.1.xlsm`. Raw formulas contain `&amp;` and `&lt;&gt;`; preview output loses `&` and `<>`.

Proof satisfied but still shape-gated:

- `EXCEL-MCP-U2`: validation metadata proof exists in `Attendance-Sheet.xlsx`, `Inventory-Tracking-Sheet1.xlsx`, and `Customer-Invoice-and-Payment-Tracker.xlsx`.
- `EXCEL-MCP-U3`: formula inventory proof exists in `Dynamic Dashboard Illustration V1.1.xlsm`.

Still blocked:

- Legacy `.xls` support: many samples are OLE `.xls`; current tool contract only accepts `.xlsx`, `.xlsm`, and `.xlsb`.
- Large worksheet budget behavior: `Выдача спецодежды_без табельных.xlsm` exceeds a per-entry XML inspection budget. Treat as a separate bounded-large-workbook behavior decision.

## Verification

- Ran package-level scan over `tmp/excel/samples`.
- Reproduced current `excel.inspect_workbook` failures for UTF-16 custom XML workbooks.
- Reproduced current `excel.read_sheet_preview` formula operator loss with `values_and_formulas`.
- Confirmed current preview already exposes formula text and cached value for simpler formulas, but not validation metadata, number format, calculation mode, defined-name context, or external-link risk per cell.
