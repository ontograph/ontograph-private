# Offline VBA To ONLYOFFICE S2 Unblock

Date: 2026-06-24

## Scope

Senior unblock pass for `OO-VBA-I2` in [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md).

## Evidence

- OntoIndex `gn_ensure_fresh` reported repo `codex` fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2`, with dirty-worktree degraded confidence.
- OntoIndex exploration confirmed the existing Excel translation surface is under `ontocode-rs/ext/excel`, with existing VBA translation owner paths in `vba_translate.rs`.
- The new Stage 1 analyzer file is not indexed yet because it is uncommitted; local source and tests were used for the analyzer operation vocabulary and guardrails.
- Stage 1 tests already cover supported analyzer output, unsupported blockers, redaction, and procedure/operation caps.

## Decision

`OO-VBA-I2` is unblocked for implementation only under the fail-closed Stage 2 contract now recorded in the ADR.

The implementation worker may build `excel.translate_vba_to_onlyoffice_js_preview` only as an analyzer-gated preview emitter:

- call Stage 1 analyzer first
- emit JavaScript only when analyzer `success` is true and there are no unsupported operations, manual-rewrite requirements, truncation warnings, redaction warnings, missing-operation warnings, redacted values, or unknown operation mappings
- return `success: false` with empty `macro_value` and `function_body` for every unsafe state
- preserve analyzer summaries, unsupported operations, redactions, and warnings in the result
- keep the implementation under `ontocode-rs/ext/excel`

## Explicit Non-Scope

- no workbook review bundle
- no runtime execution against ONLYOFFICE
- no generic `excel.translate` facade
- no broad VBA parser dependency
- no operation mappings outside the ADR Stage 2 table

## Required Verification For Implementation

- supported value/formula preview golden output
- supported formatting preview golden output, including RGB color conversion
- unsupported construct returns `success: false` and empty emitted strings
- analyzer truncation returns `success: false` and empty emitted strings
- analyzer redaction returns `success: false` and empty emitted strings
- unknown analyzer operation returns `success: false` and empty emitted strings
- tool registration test for `excel.translate_vba_to_onlyoffice_js_preview`

## Status

`OO-VBA-I2` moved from blocked to pending implementation. Stage 3 remains outside ADR scope.
