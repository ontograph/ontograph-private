# Offline VBA To ONLYOFFICE Stage 2 Closure

Date: 2026-06-24

## Scope

Manager closure for `OO-VBA-I2` from [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md).

## Result

Implemented `excel.translate_vba_to_onlyoffice_js_preview` under `ontocode-rs/ext/excel`.

The tool is fail-closed:

- calls `analyze_vba_onlyoffice_migration` first
- emits ONLYOFFICE JavaScript only when the analyzer result is fully safe
- returns `success: false` with empty `macro_value` and `function_body` for unsupported operations, analyzer warnings, truncation, redaction, unknown operation mappings, unmapped value expressions, and macro-size overflow
- keeps workbook bundle behavior, ONLYOFFICE runtime execution, generic `excel.translate`, and broad parser dependencies out of scope

## Verification

- Senior reviewer fallback `gpt-5.4-mini`: PASS.
- Implementation worker `gemini-3.5-flash-low`: completed; manager tightened an unsafe draft to the ADR fail-closed contract.
- Verification worker `gpt-5.4-mini`: PASS.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`: passed 39/39.
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`: passed.
- OntoIndex `gn_test_gap`: PASS.
- OntoIndex `gn_verify_diff`: failed globally because the repo has many unrelated dirty files; no S2-specific scope issue was identified.

## Residual Notes

The verification worker noted that a direct `ToolCall::handle` test would strengthen boundary coverage. This is optional because the ADR-required coverage is present through registration and direct behavior tests.

Stage 3 workbook-assisted flow remains outside ADR scope.
