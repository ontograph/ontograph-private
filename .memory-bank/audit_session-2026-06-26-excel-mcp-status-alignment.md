# Excel MCP Status Alignment

Date: 2026-06-26

Scope:
- Align the current-state summaries in `EXCEL_MCP_2000_USEFUL_SOLUTIONS_REVIEW.md` with already-landed `ext/excel` capabilities.

Findings:
- The "Already covered" summary still reflected the pre-`U2`/`U3`/`N2`/`N4` state.
- Row `041` still implied defined-name inspection was future work even though bounded defined-name metadata is already shipped.
- Broad "SQL conversion is blocked" wording was ambiguous because `excel.translate_powerquery_to_sql_preview` already exists for pasted M source.

Decision:
- Treat validation visibility, sheet dimension metadata, formula inventory, defined-name inspection, and lexical formula warnings as already landed in the current offline owner.
- Keep worksheet-formula-to-SQL, named-range rewrites, workbook graph extraction, and live Excel work blocked behind separate ADR or explicit user-demand gates.
- Distinguish blocked worksheet-formula-to-SQL proposals from the already-landed heuristic Power Query to SQL preview tool.

Verification:
- OntoIndex owner review confirmed the current offline owner remains `ext/excel` inspect/preview/translation modules, with no approved workbook graph or live Excel owner.
- Scoped doc-only verification follows separately.
