# Excel MCP Row 038 ADR Closure

Date: 2026-06-26

Scope:
- Close the first active design task from the continued Excel manager loop: row `038` worksheet formula AST contract.

Decision:
- Added `ADR_EXCEL_WORKSHEET_FORMULA_AST_CONTRACT.md` as the approved design-only contract artifact.
- Kept implementation blocked.

Constraints preserved:
- owner remains current offline `ext/excel`
- first slice is `xlsx`/`xlsm` worksheet formulas only
- A1 formulas only
- read-only AST only
- no SQL, no graph, no mutation, no live Excel

Queue effect:
- row `039` is now valid as a design-only blocker taxonomy proof pack
- row `040` is now valid as a design-only worksheet-formula-to-SQL ADR, still constrained by row `038`
- no implementation-worker dispatch is opened
