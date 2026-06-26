# Excel Agent Tools Tracking

## Status

Stage 1 through Stage 3 closed; final bounded ADR drift repair loop closed

## Scope

Bounded manager loop for [ADR_EXCEL_AGENT_TOOLS.md](ADR_EXCEL_AGENT_TOOLS.md).

## Model routing

- senior-reviewer: requested `claude-sonnet-4-6`; unavailable in current sub-agent tool list, using `gpt-5.4-mini`
- implementation-worker: requested `gemini-3.5-flash-low`, fallback `gpt-5.3-codex-spark`, `gpt-5.4-mini`; only `gpt-5.4-mini` is available
- verification-worker: requested `gpt-5.4-mini`

## Tasks

| ID | Task | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| EXCEL-SR1 | Challenge ADR against current extension architecture and reject scope creep | senior-reviewer | completed | Stage 1 narrowed to bounded inventory/probe and one app-server install line. |
| EXCEL-I1 | Implement Stage 1: optional `ext/excel` crate with `excel.inspect_workbook` metadata inventory | implementation-worker | completed | Added bounded workbook inventory; no `spawn_agents_on_excel_sheet`; no full PowerQuery/VBA extraction. |
| EXCEL-V1 | Verify build/test/scope for Stage 1 | verification-worker | completed | `gpt-5.4-mini` verifier accepted; no P0/P1 blockers. |
| EXCEL-SR2 | Challenge post-closure findings and confirm minimal repair scope | senior-reviewer | completed | `gpt-5.4-mini` accepted input/read caps plus ADR drift cleanup; rejected Stage 2/3 expansion. |
| EXCEL-I2 | Repair Stage 1 release-readiness issues | implementation-worker | completed | Added package entry and XML entry caps; updated ADR wording to match accepted one-tool Stage 1. |
| EXCEL-V2 | Verify repaired Stage 1 scope and tests | verification-worker | completed | `gpt-5.4-mini` verifier accepted; no P0/P1 blockers or scope creep. |
| EXCEL-SR3 | Challenge Stage 1 release-ready claim after OntoIndex review | senior-reviewer | completed | Review found two hardening blockers: model-provided workbook path handling and total XML scan budget. Stage 2/3 remain rejected. |
| EXCEL-I3 | Repair Stage 1 hardening blockers only | implementation-worker | completed | Added local relative workbook path validation and cumulative XML scan/read budget. No preview/export/orchestration tools. |
| EXCEL-V3 | Verify hardening repair and close Stage 1 | verification-worker | completed | `gpt-5.4-mini` accepted; focused Excel extension tests and scoped OntoIndex diff verification passed. |
| EXCEL-SR4 | Challenge residual Stage 1 runtime-cwd risk and ADR drift after closure review | senior-reviewer | completed | Review kept the app-server integration unchanged, required owner-local cwd resolution in the Excel extension, and removed implicit Stage 4 orchestration from the ADR roadmap. |
| EXCEL-I4 | Repair residual Stage 1 runtime-cwd risk without widening core/tool payload surface | implementation-worker | completed | Excel extension now seeds thread-owned cwd from turn environments and resolves workbook paths against that cwd; tool output keeps the model-supplied relative path. |
| EXCEL-V4 | Verify cwd-aware path resolution and final Stage 1 closure | verification-worker | completed | Focused Excel extension tests pass with the new end-to-end tool-path regression; no app-server/runtime scope creep added. |
| EXCEL-SR5 | Challenge Stage 2 preview scope against current extension/backend owners | senior-reviewer | completed | Accepted one preview tool only: `.xlsx/.xlsm`, bounded rows/columns/text, shared-string resolution, optional formulas, no CSV export, no orchestration, no new app-server/core surface. |
| EXCEL-I5 | Implement bounded `excel.read_sheet_preview` | implementation-worker | completed | Reused the existing `ext/excel` owner, added `preview.rs`, kept `backend.rs` unchanged, and wired one new DirectModelOnly tool plus focused tests. |
| EXCEL-V5 | Verify Stage 2 preview scope, tests, and OntoIndex diff | verification-worker | completed | Focused Excel extension tests passed; fresh OntoIndex analyze plus scoped `gn_verify_diff` and `gn_test_gap` both passed. |
| EXCEL-SR6 | Challenge Stage 3 export scope against current extension/backend owners | senior-reviewer | completed | Accepted one explicit CSV export tool only; reuse the Excel extension; no agent orchestration wrapper; no new app-server/core surface. |
| EXCEL-I6 | Implement bounded `excel.export_sheet_to_csv` | implementation-worker | completed | Reused Stage 2 workbook/sheet selection patterns, added `export.rs`, and kept export logic owner-local instead of expanding `backend.rs`. |
| EXCEL-V6 | Verify Stage 3 export scope, tests, and OntoIndex diff | verification-worker | completed | Focused Excel extension tests passed; fresh OntoIndex analyze plus scoped `gn_verify_diff` and `gn_test_gap` both passed. |
| EXCEL-SR7 | Challenge ADR status/claims against implemented Excel surface | senior-reviewer | completed | Kept the scope to doc drift only: stale module layout, overstated Stage 3 handoff verification, and metadata sections that claimed more than current marker-only behavior. |
| EXCEL-I7 | Repair ADR wording to match current code and tests | implementation-worker | completed | Kept the three-tool surface unchanged and narrowed the markdown claims instead of widening code or test scope in this loop. |
| EXCEL-V7 | Verify repaired ADR wording against current code and OntoIndex evidence | verification-worker | completed | Refreshed OntoIndex after the markdown edits and verified the repaired claims against the current Excel code/tests; `gn_verify_diff` was file-clean but noisy for markdown symbol extraction. |
| EXCEL-SR8 | Challenge remaining ADR drift against current Excel argument shapes and workbook read path claims | senior-reviewer | completed | Scope stayed doc-only: repaired stale argument examples and narrowed workbook package method prose to current implementation evidence. |
| EXCEL-I8 | Repair remaining ADR drift without widening Excel code or tests | implementation-worker | completed | Updated the suggested request-shape example and workbook package method bullets only. |
| EXCEL-V8 | Verify final ADR wording against current code and refreshed OntoIndex evidence | verification-worker | completed | Refreshed OntoIndex, rechecked current Excel symbols, and accepted file-scoped markdown truthfulness verification despite noisy heading extraction in `gn_verify_diff`. |

## Manager Notes

- 2026-06-23: OntoIndex freshness check passed for HEAD; worktree is dirty, so all edits must stay narrowly scoped.
- 2026-06-23: Start with Stage 1 only. Stage 2/3 remain pending until Stage 1 is verified.
- 2026-06-23: OntoIndex impact for `thread_extensions` is HIGH due app-server setup callers; implementation must keep the app-server edit to one extension install line and verify app-server thread extension behavior.
- 2026-06-23: Senior review accepted `ext/excel` ownership but rejected full Stage 1 metadata extraction as too broad. ADR updated to keep Stage 1 to package/sheet/marker inventory only.
- 2026-06-23: Implementation added `ontocode-rs/ext/excel` with exactly one model-visible tool, `excel.inspect_workbook`.
- 2026-06-23: Manager added explicit caps for workbook-controlled strings returned to the model.
- 2026-06-23: Validation passed: `CARGO_BUILD_JOBS=8 just fmt`; `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` (5 passed); `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server app_server_event_sink_forwards_thread_goal_updates`; `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`; `just bazel-lock-update`; `just bazel-lock-check`.
- 2026-06-23: Verification worker `gpt-5.4-mini` returned ACCEPT with no P0/P1 blockers. Follow-ups remain Stage 2/3: `excel.read_sheet_preview` and `excel.export_sheet_to_csv`.
- 2026-06-23: OntoIndex scoped `gn_verify_diff` passed for the Excel file set; unscoped verification is blocked by unrelated dirty worktree files.
- 2026-06-23: `ontoindex analyze --skills --skip-agents-md` refreshed the index successfully after closure: 80,026 nodes, 209,545 edges, 3,453 clusters, 300 flows.
- 2026-06-23: New bounded repair loop opened after senior challenge: add input/read caps for model-visible workbook inspection and clean stale ADR wording. No Stage 2/3 implementation in this loop.
- 2026-06-23: Senior reviewer `gpt-5.4-mini` accepted the repair scope as sufficient and minimal; caps must live in the shared archive/XML read path.
- 2026-06-23: Implementation added `MAX_PACKAGE_PART_COUNT` and `MAX_XML_ENTRY_BYTES` guards in `backend.rs`, plus focused regression tests. ADR now marks Stage 1 accepted/implemented and moves preview/export to future stages.
- 2026-06-23: Validation passed: `CARGO_BUILD_JOBS=8 just fmt`; `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` (7 passed); `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`.
- 2026-06-23: OntoIndex scoped `gn_verify_diff` passed for repair files.
- 2026-06-23: Verification worker `gpt-5.4-mini` accepted EXCEL-V2 with no P0/P1 blockers or scope creep.
- 2026-06-23: `ontoindex analyze --skills --skip-agents-md` refreshed the index successfully after repair: 80,055 nodes, 209,577 edges, 3,477 clusters, 300 flows.
- 2026-06-23: New bounded hardening loop opened from OntoIndex review. `gn_ensure_fresh` reports the index is current at HEAD but the worktree is dirty. Keep the loop to Stage 1 repairs only.
- 2026-06-23: Current extension API forwards model-visible tool calls without cwd/permission context, so EXCEL-I3 must add conservative local path validation in the Excel tool and document that full sandbox-policy integration is future extension-runtime work.
- 2026-06-23: Senior-reviewer sub-agent requested on `gpt-5.4-mini` errored with `Unsupported content type`; manager continued locally using OntoIndex evidence and did not retry the same model.
- 2026-06-23: Implementation added `workbook_path_from_model_arg` to reject absolute, parent-traversal, URL-like, non-workbook, and symlink-traversing model paths before `File::open`.
- 2026-06-23: Implementation added `XmlReadBudget` with `MAX_XML_SCAN_ENTRIES` and `MAX_XML_SCAN_BYTES` so formula and PowerQuery marker scans share a cumulative budget.
- 2026-06-23: Validation passed: `CARGO_BUILD_JOBS=8 just fmt`; `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` (10 passed).
- 2026-06-23: Validation passed: `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`; OntoIndex scoped `gn_verify_diff` passed for the Excel hardening files.
- 2026-06-23: Verification worker `gpt-5.4-mini` returned ACCEPT: hardening scope satisfied and no Stage 2/3 preview/export/orchestration implementation was added.
- 2026-06-23: Closure review found one remaining Stage 1 correctness gap: the tool validated relative paths but still resolved them against the process cwd instead of the turn cwd.
- 2026-06-23: Manager rejected any fix that widened `ToolCall`, changed app-server extension wiring beyond the existing install line, or added Stage 2/3 preview/export behavior.
- 2026-06-23: Implementation reused the existing extension turn-input lifecycle to store the primary turn cwd in Excel extension thread state and updated `ExcelInspectionTool` to resolve relative workbook paths against that stored cwd.
- 2026-06-23: Implementation kept the model-visible response path stable by reading the workbook from the resolved absolute path while preserving the model-supplied relative path in `InspectWorkbookResult.path`.
- 2026-06-23: ADR drift removed: the accepted roadmap stops at Stage 3, and any future `spawn_agents_on_excel_sheet` wrapper now requires separate usage evidence and a new ADR.
- 2026-06-23: Final validation passed: `CARGO_BUILD_JOBS=8 just fmt`; `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` (11 passed); `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`.
- 2026-06-23: `ontoindex analyze --skills --skip-agents-md` refreshed the index successfully after the Stage 1 closure repair: 80,077 nodes, 209,649 edges, 3,475 clusters, 300 flows.
- 2026-06-23: Scoped OntoIndex verification passed after the repair: `gn_verify_diff` PASS for the Excel ADR/file set and `gn_test_gap` PASS for the Excel extension file set.
- 2026-06-23: Stage 2 loop opened after Stage 1 closure. Senior-review target is the minimal bounded preview tool only: no export, no orchestration, no `.xlsb` sheet decoding, and no new app-server/core surface.
- 2026-06-23: Stage 2 implementation added `excel.read_sheet_preview` with one new `preview.rs` module, `SheetSelector` name/index selection, `CellContentMode`, shared-string and inline-string resolution, optional formula capture, and bounded worksheet/shared-string XML reads for `.xlsx/.xlsm` only.
- 2026-06-23: Validation passed before the fix pass: `CARGO_BUILD_JOBS=8 just fmt`; `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` (14 passed).
- 2026-06-23: `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension` completed with one `preview.rs` clippy autofix and no new scope.
- 2026-06-23: `ontoindex analyze --skills --skip-agents-md` refreshed the index successfully after Stage 2: 80,144 nodes, 209,837 edges, 3,468 clusters, 300 flows.
- 2026-06-23: Scoped OntoIndex verification passed for Stage 2: `gn_verify_diff` PASS and `gn_test_gap` PASS for `ontocode-rs/ext/excel/src/{extension,preview,tool,tests}.rs`.
- 2026-06-23: Stage 3 loop opened after Stage 2 closure. Senior-review target is one explicit CSV export tool only: no `spawn_agents_on_excel_sheet`, no background orchestration, no `.xlsb` sheet export, and no new app-server/core surface.
- 2026-06-23: Stage 3 implementation added `excel.export_sheet_to_csv` with one new `export.rs` module, relative CSV output-path handling, explicit CSV file writes for `.xlsx/.xlsm`, and direct handoff-friendly output for `spawn_agents_on_csv`.
- 2026-06-23: Validation passed before the fix pass: `CARGO_BUILD_JOBS=8 just fmt`; `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` (16 passed).
- 2026-06-23: `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension` completed with no additional code changes.
- 2026-06-23: `ontoindex analyze --skills --skip-agents-md` refreshed the index successfully after Stage 3: 80,242 nodes, 209,978 edges, 3,504 clusters, 300 flows.
- 2026-06-23: Scoped OntoIndex verification passed for Stage 3: `gn_verify_diff` PASS and `gn_test_gap` PASS for `ontocode-rs/ext/excel/src/{export,extension,lib,preview,tool,tests}.rs`.
- 2026-06-23: New bounded ADR drift repair loop opened from post-closure review. Keep the scope to markdown truthfulness only; do not widen the Excel implementation or add a new cross-surface integration claim without code/tests.
- 2026-06-23: ADR drift repair updated the module-layout section to include `preview.rs` and `export.rs`, changed Stage 3 wording from “verify explicit handoff” to “handoff-ready for explicit use”, and narrowed PowerQuery/VBA/comments/object text to the current marker-only behavior.
- 2026-06-23: `ontoindex analyze --skills --skip-agents-md` refreshed the index successfully after the ADR repair: 80,225 nodes, 209,992 edges, 3,479 clusters, 300 flows.
- 2026-06-23: Markdown-only verification note: `gn_verify_diff` matched the expected changed files, but OntoIndex symbol extraction is noisy on memory-bank headings; final acceptance used the file-scoped diff plus prior code-backed audit evidence rather than the changed-heading list.
- 2026-06-23: New final bounded ADR truthfulness loop opened after another OntoIndex-backed review found two low-severity drifts still left: stale `SheetSelector` / `cell_content` example shapes and workbook package bullets that still described broader OpenXML reads than the current owners clearly implement.
- 2026-06-23: Final ADR truthfulness repair updated the workbook package method section to distinguish current inspect/preview/export reads from later possible OpenXML expansions, and corrected the `SheetSelector` / `cell_content` example shapes to match `ontocode-rs/ext/excel/src/tool.rs`.
- 2026-06-23: `ontoindex analyze --skills --skip-agents-md` refreshed the index successfully after the final ADR repair: 80,236 nodes, 209,982 edges, 3,483 clusters, 300 flows.
- 2026-06-23: Verification closed EXCEL-V8 using direct OntoIndex symbol/context evidence plus file-scoped diff review; `gn_verify_diff` still over-reports changed markdown headings in the dirty worktree, so changed-symbol output was treated as noisy and non-blocking for this doc-only loop.
