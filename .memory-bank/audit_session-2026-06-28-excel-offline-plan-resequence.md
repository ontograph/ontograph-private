# Excel Offline Plan Resequence

## Scope
- Reviewed `EXCEL_OFFLINE_NEXT_TOOLS_PROJECT_PLAN.md` against current `ext/excel` source and OntoIndex blast radius
- Fixed overstated `.xlsb` wording
- Re-sequenced the next offline queue without reopening live Excel work

## Findings
- `EXCEL-OFFLINE-T0` landed bounded `.xlsb` read support, not full parity
- `inspect_workbook` already exposes bounded pivot presence markers, so `EXCEL-OFFLINE-T1` needed explicit consumer proof before becoming the default next dispatch
- `extract_powerquery_queries_from_workbook` is the lower-blast existing owner for the next offline slice

## Decision
- `EXCEL-OFFLINE-T2` is the active next task
- `EXCEL-OFFLINE-T1` stays open, but only as a gated follow-up after a concrete offline consumer proves current workbook markers are insufficient
- The queue stays offline-only and read-only inside `ontocode-rs/ext/excel`

## Queue Update
1. `EXCEL-OFFLINE-T0` remains closed as bounded `.xlsb` read support
2. `EXCEL-OFFLINE-T2` is now active
3. `EXCEL-OFFLINE-T3` remains pending `T2`
4. `EXCEL-OFFLINE-T1` is deferred behind the Power Query follow-ons unless fresh consumer evidence reopens it
