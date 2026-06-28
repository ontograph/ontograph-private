# Excel MCP Row 040 SQL ADR Closure

Date: 2026-06-27

Scope:
- Close the design-only contract step for row `040` worksheet-formula-to-SQL generation and validation.

OntoIndex evidence used:
- `translate_powerquery_to_sql_preview` lives in `ontocode-rs/ext/excel/src/powerquery_translate.rs` and remains the current Power Query SQL preview owner, not a worksheet-formula planner.
- `ExcelInspectionTool.handle` still routes the offline workbook-inspection owner through `ontocode-rs/ext/excel/src/tool.rs` into `inspect_workbook_with_display_path`, which keeps the Excel extension boundary read-only and path-based.

Donor evidence used:
- donor planner tests prove some repeated same-row formulas can map to `SELECT` expressions, but only under explicit row-grain assumptions
- donor planner tests prove running totals need row identity and block when that identity is unavailable
- donor planner tests prove aligned `SUMIFS` aggregation must block on mismatched ranges
- donor planner and parser tests block external-link workbooks, hidden or filtered rows, merged-row-grain layouts, and unsupported AST nodes such as spill-capable `XLOOKUP(...)`

Decision:
- accept a design-only worksheet-formula-to-SQL contract
- keep any future reopen AST-backed, offline, fail-closed, and validated against cached Excel values or equivalent offline evidence
- do not reopen Power Query SQL preview, graph extraction, workbook mutation, live Excel, or evaluation-engine scope

Queue effect:
- row `040` is closed for the ADR-first reopen step
- legacy `.xls` feasibility becomes the next active design-only task
- no implementation-worker dispatch is opened
