# Excel MCP Next Unblock Queue

Date: 2026-06-26

Scope: user selected unblock options `1-6` after `EXCEL-MCP-U1` through `EXCEL-MCP-U4` closure.

Opened queue:
- `EXCEL-MCP-N1`: clean verification gate, process-blocked until unrelated dirty worktree state is isolated or explicitly accepted
- `EXCEL-MCP-N2`: formula risk warnings in `excel.inspect_sheet_formulas`, implementation-ready after fresh OntoIndex impact
- `EXCEL-MCP-N3`: bounded defined-name inspection, implementation-ready after fresh OntoIndex impact
- `EXCEL-MCP-N4`: worksheet dimension metadata in preview, implementation-ready after fresh OntoIndex impact
- `EXCEL-MCP-N5`: workbook graph proof pack only, no implementation until expected edges are specified and fixture-backed
- `EXCEL-MCP-N6`: live Excel companion ADR only, no implementation in current offline `ext/excel`

Senior constraint:
- keep `ontocode-rs/ext/excel` offline, read-first, and bounded
- do not reopen formula AST, SQL generation, named-range rewrites, workbook graph extraction, live Excel, legacy `.xls`, or large-worksheet paging as implementation work without a separate accepted contract

OntoIndex evidence:
- current Excel owners remain `preview.rs`, `formula_inspect.rs`, `tool.rs`, `extension.rs`, and focused `tests.rs`
- OntoIndex exploration found the relevant read paths under `read_sheet_preview_with_display_path`, `parse_sheet_preview`, and the Excel test fixture helpers
