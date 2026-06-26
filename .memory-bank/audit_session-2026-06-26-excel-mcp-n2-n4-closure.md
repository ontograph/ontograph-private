# Excel MCP N2-N4 Closure

Date: 2026-06-26

Scope:
- `EXCEL-MCP-N2`: formula risk warnings.
- `EXCEL-MCP-N3`: defined-name inspection.
- `EXCEL-MCP-N4`: worksheet dimension metadata.

Decision:
- Closed `N2`, `N3`, and `N4` as existing-owner, read-only Excel extension work.
- Kept `N1` process-blocked until unrelated dirty worktree changes are isolated, committed, or explicitly excluded from global verification claims.
- Kept `N5` proof-only until a tiny expected-edge contract and fixture evidence exist.
- Kept `N6` ADR-only; no COM/live Excel dependency belongs in current offline `ext/excel`.

Implementation evidence:
- `ontocode-rs/ext/excel/src/formula_inspect.rs` adds lexical formula risk markers only.
- `ontocode-rs/ext/excel/src/formula_inspect.rs` and `tool.rs` expose bounded structured defined-name metadata while preserving `defined_names_sample`.
- `ontocode-rs/ext/excel/src/preview.rs` and `tool.rs` expose selected-sheet `<dimension ref="...">` metadata without full-sheet scans, paging, or write APIs.

Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed with 58 tests.
- `CARGO_BUILD_JOBS=8 just fmt` passed.
- Verification-worker requested for the slice; final manager decision remains scoped because the repository has broad unrelated dirty worktree state and `ontocode-rs/ext/excel/` is untracked in this checkout.

Rejected expansion:
- No formula AST parsing.
- No formula evaluation.
- No formula-to-SQL generation.
- No formula rewrite to named ranges.
- No workbook graph extraction.
- No live Excel companion implementation.
