# Excel MCP U3 Closure

Date: 2026-06-26

Scope: bounded manager loop for `.memory-bank/EXCEL_MCP_2000_USEFUL_SOLUTIONS_REVIEW.md`.

Closed task:
- `EXCEL-MCP-U3`: added read-only `excel.inspect_sheet_formulas` under the existing `ontocode-rs/ext/excel` owner.

Implemented behavior:
- accepts workbook `path`, existing `SheetSelector`, and optional `max_formulas`
- defaults to 128 formulas and hard-caps at 512
- reports selected-sheet formula text, cached values, shared formula attributes, style index, number format id/code, workbook calculation flags, defined-name samples, external-link marker, truncation, and warnings
- preserves XML entity text in formulas and defined names
- records shared-formula follower cells without inventing a formula body

Rejected in this slice:
- formula evaluation
- formula AST parsing
- formula-to-SQL generation
- formula rewrites to named ranges
- workbook dependency graph extraction
- live Excel automation
- legacy `.xls` support

Verification:
- OntoIndex impact checks for `extension.rs:install`, `SheetSelector`, and `ExcelThreadState` returned `LOW` risk with no affected execution processes.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed: 57 tests.
- `CARGO_BUILD_JOBS=8 just fmt` passed.
- Senior-reviewer returned `PASS`.
- Verification-worker confirmed the scoped Excel tests and format pass, but returned `BLOCK` on global scope cleanliness because the repository has broad unrelated dirty state.
- Scoped OntoIndex `gn_verify_diff` with explicit U3 files and executed-test evidence returned `PASS`.

Residual caveat:
- The repository worktree remains broadly dirty with many unrelated changes, and `ontocode-rs/ext/excel/` is still reported as an untracked directory by `git status`; global diff verification must be read with that caveat.
