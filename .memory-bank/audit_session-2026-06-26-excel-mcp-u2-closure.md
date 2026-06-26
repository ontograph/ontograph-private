# Excel MCP U2 Closure

Date: 2026-06-26
Scope: `EXCEL-MCP-U2` from `.memory-bank/EXCEL_MCP_2000_USEFUL_SOLUTIONS_REVIEW.md`

## Closed

- Added selected-sheet data-validation visibility to `excel.read_sheet_preview`.
- Output is bounded and read-only: validation summaries are capped, resolved validation values have a selected-sheet total budget, and `resolved_values_source` distinguishes inline lists, same-sheet ranges, unresolved formulas, and absence of values.
- Preserved stop conditions: no validation writes, no arbitrary formula evaluation, no cross-workbook/external/volatile/indirect resolution, and no workbook-wide validation API.

## Verification

- OntoIndex impact for `read_sheet_preview_with_display_path`, `parse_sheet_preview`, and `ReadSheetPreviewResult`: `LOW`.
- Senior-reviewer block was addressed by adding total output budget, explicit source status, and UX-visible dropdown/error-style fields.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`: passed, 55 tests.
- `CARGO_BUILD_JOBS=8 just fmt`: passed after final code changes.

## Remaining

- `EXCEL-MCP-U3`: bounded read-only `excel.inspect_sheet_formulas`.
- Formula AST parsing, SQL generation, formula rewrites, graph extraction, live Excel, legacy `.xls`, and large-worksheet budget behavior remain blocked.
