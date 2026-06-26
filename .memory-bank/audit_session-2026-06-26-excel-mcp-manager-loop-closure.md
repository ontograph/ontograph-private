# Excel MCP Manager Loop Closure

Date: 2026-06-26
Scope: `.memory-bank/EXCEL_MCP_2000_USEFUL_SOLUTIONS_REVIEW.md`

## Closed

- `EXCEL-MCP-U1`: `excel.inspect_workbook` no longer fails on UTF-16 BOM `customXml/item*.xml` Power Query metadata. The fix stays in `ontocode-rs/ext/excel/src/backend.rs`, preserves XML read budgets, and adds `inspect_workbook_handles_utf16_power_query_custom_xml`.
- `EXCEL-MCP-U4`: `excel.read_sheet_preview` no longer drops XML entity operators from formulas such as `&amp;` and `&lt;&gt;`. The fix stays in `ontocode-rs/ext/excel/src/preview.rs` and adds `read_sheet_preview_preserves_formula_xml_entities`.

## Still Blocked

- `EXCEL-MCP-U2`: validation metadata proof is satisfied, but output shape is not approved.
- `EXCEL-MCP-U3`: formula inventory proof is satisfied, but output shape is not approved.
- Formula AST, SQL conversion, formula rewrite to named ranges, live Excel automation, workbook graph extraction, legacy `.xls`, and large-worksheet budget behavior remain separate ADR or contract decisions.

## Verification

- OntoIndex impact for `read_optional_named_entry`: `LOW`, direct callers are local workbook-inspection helpers.
- OntoIndex impact for `parse_sheet_preview`: `LOW`, direct production caller is `read_sheet_preview_with_display_path`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`: passed, 54 tests.
- `CARGO_BUILD_JOBS=8 just fmt`: passed after final code changes.
