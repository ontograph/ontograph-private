# Excel Agent Tools Stage 1 Closure

Date: 2026-06-23

## Scope

Bounded manager loop for [ADR_EXCEL_AGENT_TOOLS.md](ADR_EXCEL_AGENT_TOOLS.md), limited to Stage 1 Solution A.

## Decision

Stage 1 is closed.

Implemented exactly one optional extension-owned model-visible tool:

- `excel.inspect_workbook`

The implementation lives in `ontocode-rs/ext/excel` and is installed through the existing app-server extension registry with one install line.

## Senior Challenge Outcome

The ADR was narrowed before implementation:

- keep `ext/excel` as owner
- keep workbook inspection to package, sheet, and marker inventory
- reject `spawn_agents_on_excel_sheet` for v1
- defer full PowerQuery M extraction, full VBA source extraction, embedded object payload extraction, sheet preview, and CSV export

## Verification

Passed:

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server app_server_event_sink_forwards_thread_goal_updates`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `just bazel-lock-update`
- `just bazel-lock-check`
- OntoIndex `gn_verify_diff` passed with scoped changed-file override

The unscoped OntoIndex diff verifier failed because the repository already had many unrelated dirty files. No unrelated dirty files were modified or reverted for this closure.

## Follow-Ups

Future stages remain:

- `excel.read_sheet_preview`
- `excel.export_sheet_to_csv`

Both need separate bounded manager-loop approval before implementation.
