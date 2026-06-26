# Excel MCP N5 Proof Contract

Date: 2026-06-26

Scope:
- `EXCEL-MCP-N5`: workbook graph proof pack.

Decision:
- Advanced `N5` from vague proof-only status to a concrete proof contract.
- Did not open implementation.
- Kept graph extraction blocked until a tiny synthetic fixture and expected edge output are committed.

Minimum future contract:
- Nodes: workbook, worksheet, cell formula, defined name, table, Power Query.
- Edges: worksheet contains formula, formula references cell/range, defined name targets range/formula, table has range, Power Query references name/table.
- Evidence must cite source workbook parts or bounded decoded query lines.

Required fixture before code:
- Two worksheets.
- One formula referencing another sheet.
- One workbook-scope defined name.
- One sheet-scope defined name.
- One table.
- One Power Query reference to either the table or defined name.

Superseded clarification:
- Later senior review blocked fixture-first proof before an approved graph schema, Rust type, parser, and output contract exist.
- Treat the fixture list above as future ADR input only, not implementation approval.
- Current allowed next action is text design/ADR only.

Blocked expansions:
- No placeholder empty-edge graph output.
- No formula evaluation.
- No SQL generation.
- No formula rewrite to named ranges.
- No workbook mutation.
- No dependency recalculation engine in `ext/excel`.
