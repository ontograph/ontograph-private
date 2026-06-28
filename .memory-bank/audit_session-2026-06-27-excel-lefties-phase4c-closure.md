# Excel Lefties Phase 4C Closure

## Decision

Closed the bounded offline Phase 4C aggregate SQL slice inside `ontocode-rs/ext/excel`.

The landed review-only preview now covers exact aligned:

- `SUMIFS`
- `COUNTIFS`
- `AVERAGEIFS`
- `MAXIFS`
- `MINIFS`

## Implementation Shape

Kept all work inside the existing worksheet-formula SQL owner:

- `ontocode-rs/ext/excel/src/formula_sql.rs`
- `ontocode-rs/ext/excel/src/formula_sql_tests.rs`
- `ontocode-rs/ext/excel/src/tests.rs`

No new tool surface was added.

No live Excel, COM, `Formula2`, named-range apply, or graph scope was reopened.

## Guardrails

The landed slice stays fail-closed:

- target columns must resolve exactly
- criteria ranges and value ranges must prove the same row grain
- only equality criteria are previewed
- operator-string criteria remain blocked
- wildcard criteria remain blocked
- unresolved targets, volatile functions, external links, and mismatched ranges remain blocked

## Validation

- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
- `CARGO_BUILD_JOBS=8 just fmt`

`just test -p ontocode-excel-extension` passed before the final `fix` / `fmt` pass.

The post-test code change was a non-behavioral local argument-count cleanup in the new aggregate helper.

## Next State

There is no active post-Phase-4C implementation dispatch in this loop.

Exact reopen gates:

- Phase 5 / Phase 6 require a separate accepted live-owner contract
- any broader offline SQL follow-up requires a fresh approved fixture pack proving the new aggregate or criteria semantics without weakening the fail-closed contract
