# Excel MCP Manager Unblock Pass

Date: 2026-06-26

Scope:
- Act as manager on the remaining tasks from `EXCEL_MCP_2000_USEFUL_SOLUTIONS_REVIEW.md` after status alignment.

Current owner check:
- OntoIndex evidence still shows the active owner is the offline `ext/excel` inspect/preview/export/translation surface.
- No approved workbook-graph extractor owner exists.
- No approved live Excel, COM, or external companion owner exists in the current extension.

Manager decision:
- Do not dispatch any implementation worker from the current remaining queue.
- Keep `N5-HOLD` as the default because it is the cheapest valid path and avoids graph theater.
- Keep `N6-A` blocked unless live Excel, chart mutation, VBA mutation, or an external companion is explicitly requested.
- Treat `N1-A` as optional manager hygiene only; it is not a product task unless someone needs a repo-wide clean verification claim.

Valid unblock paths, in order:
1. `N5-HOLD`: leave workbook graph extraction closed.
2. `N1-A`: if a clean global verification claim is needed, isolate or explicitly exclude unrelated worktree changes and rerun repo-level verification.
3. `N5-ACCEPT`: if workbook graph extraction is actually wanted, explicitly accept `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md` and reopen one concrete Rust slice for fresh senior review.
4. `N6-A`: if live Excel or mutation workflows are actually wanted, open a separate live Excel companion ADR outside current offline `ext/excel`.

Not valid unblocks:
- Repeating the manager loop without new acceptance or new failing artifacts.
- Opening fixture-only graph proof work.
- Recasting existing Power Query SQL preview as approval for worksheet-formula SQL generation.
