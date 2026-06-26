# Offline VBA To ONLYOFFICE Fail-Closed Hardening

Date: 2026-06-24

## Scope

Bounded manager-loop follow-up from audit findings against [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md).

## Outcome

- Kept scope to the existing `ontocode-rs/ext/excel` analyzer and tests.
- Unknown executable VBA statements inside parsed procedures now become unsupported operations instead of being ignored.
- Bare alignment constants are no longer analyzer-approved unless the emitter can serialize them; numeric literals and quoted strings remain supported.
- Did not add workbook bundling, ONLYOFFICE runtime execution, generic `excel.translate`, broad parser dependency, or neutral IR/module split.

## Verification

- Senior-review fallback narrowed the work to the two audit findings only.
- `just fmt` passed from `ontocode-rs`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed 41/41.
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension` passed.
- OntoIndex freshness was checked; confidence remains medium because the worktree contains unrelated dirty files and the new Excel files are not yet indexed.
