# Excel MCP Shape Gate Unblock

Date: 2026-06-26
Scope: `.memory-bank/EXCEL_MCP_2000_USEFUL_SOLUTIONS_REVIEW.md`

## Decision

Opened two implementation-ready read-only tasks:

- `EXCEL-MCP-U2`: add selected-sheet data-validation visibility to `excel.read_sheet_preview`.
- `EXCEL-MCP-U3`: add a bounded read-only `excel.inspect_sheet_formulas` tool under the existing Excel extension owner.

## Constraints

- `U2` is limited to selected-sheet validation summaries, inline list values, and simple same-sheet range-backed lists.
- `U3` is limited to formula text, cached values, formula attributes, style/number-format metadata, calculation properties, defined-name samples, and external-link markers.
- R1C1 synthesis, formula AST parsing, SQL generation, formula rewriting, workbook graph extraction, live Excel automation, and legacy `.xls` remain blocked.

## Evidence

- OntoIndex shows the existing Excel owner is `ontocode-rs/ext/excel`: `inspect_workbook_with_display_path`, `read_sheet_preview_with_display_path`, `parse_sheet_preview`, and the tool result structs in `tool.rs`.
- The prior samples review already satisfied proof for validation metadata and formula inventory gaps; this note approves the bounded output shape only.
