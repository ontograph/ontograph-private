# Excel MCP Blocked Rows Unblock Matrix

Date: 2026-06-26

Scope:
- Convert the remaining blocked Excel rows into explicit reopen contracts instead of generic "blocked" status.

Decision:
- Rows `038-040` reopen only through an ADR-first formula-analysis track: AST contract first, blocker taxonomy second, SQL planning/validation only after both exist.
- Row `041` reopens only as an explicit mutation contract with user-authored named-range mapping and workbook regression artifacts.
- Row `042` reopens only through a separate live Excel companion ADR outside offline `ext/excel`.
- Rows `043-044` reopen only after explicit acceptance of `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md` plus one concrete first edge family.
- Legacy `.xls` support reopens only through dependency/owner feasibility.
- Large-workbook XML budget work reopens only through one failing artifact plus a precise degrade-versus-fatal policy.

Manager result:
- No blocked row was promoted straight to implementation.
- Every blocked row now has one smallest valid next step recorded in the main review artifact.
