# Excel MCP Continue Dispatch Order

Date: 2026-06-26

Scope:
- Continue the bounded manager loop after the blocked-row unblock matrix was added.

Senior-reviewer result:
- Reopened rows are not equally ready.
- Open only the minimum upstream design tasks that can later unblock dependent rows.

Manager decision:
- Active next task 1: row `038-AST-ADR`.
- Active next task 2: legacy `.xls` feasibility note.
- Active next task 3: large-workbook XML budget policy note.

Still queued behind dependencies:
- Row `039` waits on row `038`.
- Row `040` waits on row `038`.

Still demand-gated:
- Row `041` waits for explicit rewrite demand plus a real workbook.
- Row `042` waits for explicit live Excel / COM / mutation demand.
- Rows `043-044` wait for explicit acceptance of `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md` for one concrete edge family.

Dispatch result:
- No implementation-worker dispatch is valid yet.
- Next loop step is design/feasibility/policy work only.
