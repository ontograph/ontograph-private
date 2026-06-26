# Offline VBA To ONLYOFFICE Macro Translation Tracking

## Scope

Bounded manager loop for [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md).

Current authority:

- Stage 0 target-contract capture is complete.
- Stage 1 analyzer is complete.
- Stage 2 preview translator is complete under the fail-closed Stage 2 contract in the ADR.
- The narrow Stage 3 workbook-assisted flow is complete as a follow-on-owned read-only wrapper; it does not broaden the canonical ADR into runtime validation, bundle generation, or generic translation routing.

## Source Evidence

- ONLYOFFICE source repo: `https://github.com/ONLYOFFICE/sdkjs.git`
- ONLYOFFICE source commit: `72b0421c0bbf9d01eed9cf14834ae47eb2df1b50`
- Local evidence path: `tmp/onlyoffice/sdkjs/common/macro-recorder.js`

## Tasks

| ID | Task | Role | Status | Notes |
| --- | --- | --- | --- | --- |
| OO-VBA-SR0 | Re-review ADR scope and Stage 0 acceptance criteria against current `ext/excel` owner and ONLYOFFICE evidence | senior-reviewer | completed | Fallback `gpt-5.4-mini` accepted Stage 0; initial `claude-sonnet-4-6` dispatch failed with 429. |
| OO-VBA-I0 | Implement Stage 0 target-contract capture artifact | implementation-worker | completed | Checked Stage 0 contract artifact added with pinned ONLYOFFICE commit, supported call catalog, wrapper shape, examples, deferred ops, bounds/redaction, and drift checks. |
| OO-VBA-V0 | Verify Stage 0 artifact, ADR alignment, and tracking state | verification-worker | completed | `gpt-5.4-mini` PASS. No findings; verified commit pin, recorder links, supported calls, examples, non-scope, bounds/redaction, and no Rust/tool-surface changes. |
| OO-VBA-I1 | Implement `excel.analyze_vba_onlyoffice_migration` | implementation-worker | completed | Analyzer-only tool added under `ontocode-rs/ext/excel`; scoped Excel tests pass. |
| OO-VBA-SR2 | Prepare fail-closed Stage 2 preview-emission contract | senior-reviewer | completed | Senior pass narrowed S2 to analyzer-gated emission only: empty output on unsupported operations, truncation, redaction, analyzer failure, or unknown operation mapping. |
| OO-VBA-I2 | Implement `excel.translate_vba_to_onlyoffice_js_preview` | implementation-worker | completed | Implemented as fail-closed analyzer-gated preview emitter under `ontocode-rs/ext/excel`; scoped tests pass. |
| OO-VBA-V2 | Verify Stage 2 fail-closed implementation | verification-worker | completed | `gpt-5.4-mini` PASS. Confirmed analyzer gate, empty emitted strings on unsafe paths, no workbook/runtime/generic translator scope, and required behavior tests. |
| OO-VBA-AUD1 | Reconcile canonical ADR state with landed Stage 3 workbook-review follow-on and current owner state | senior-reviewer | completed | Docs-only consistency repair accepted: the canonical ADR and this tracking file now state that narrow Stage 3 landed as a follow-on-owned read-only wrapper, while broader deferred slices remain gated. |
| OO-VBA-AUD2 | Challenge the Stage 0 first-slice call catalog against current positive translator coverage | senior-reviewer | completed | Challenge outcome: keep the Stage 0 call catalog because analyzer/emitter code already supports wrap/alignment operations, but open one focused coverage task before treating those calls as fully proven by tests. |
| OO-VBA-COV1 | Add positive end-to-end wrap/alignment coverage for the existing first-slice contract | implementation-worker | completed | Added one focused analyzer test and one focused translator test proving successful `WrapText`, numeric horizontal alignment, and quoted vertical alignment emission without changing tool behavior or public payloads. |

## Manager Log

- 2026-06-24: Manager opened bounded loop. OntoIndex `gn_ensure_fresh` reported index fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2` with dirty worktree caveat. Stage 0 is the only dispatchable implementation task.
- 2026-06-24: Stage 0 contract capture completed in `.memory-bank/ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION_STAGE0_TARGET_CONTRACT.md`; memory-bank docs and index updated to point at the captured contract.
- 2026-06-24: Senior review fallback accepted Stage 0. No blocker findings; Stage 1 is pending for a future analyzer-only dispatch, and Stage 2 remains blocked behind analyzer proof.
- 2026-06-24: Verification worker accepted Stage 0 with no findings. Manager stops this bounded Stage 0 loop here; analyzer and preview translator work require a new implementation dispatch.
- 2026-06-24: Manager reopened the bounded loop for `OO-VBA-I1` only. `OO-VBA-I2` remains blocked; Stage 3 remains outside approved scope.
- 2026-06-24: `OO-VBA-I1` completed. Analyzer-only tool `excel.analyze_vba_onlyoffice_migration` was added with bounded supported/unsupported operation classification, redaction, caps, and focused Excel tests.
- 2026-06-24: Senior reviewer fallback accepted the analyzer-only implementation shape and flagged ADR final-recommendation wording that could be misread as authorizing Stage 2; ADR wording was tightened to keep the translator blocked.
- 2026-06-24: Validation: `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed 32/32; `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension` passed after final cleanup.
- 2026-06-24: Manager reopened the bounded loop for remaining open tasks. Primary senior reviewer `claude-sonnet-4-6` failed with 429; fallback `gpt-5.4-mini` reviewed `OO-VBA-I2` and directed no implementation dispatch. `OO-VBA-I2` remains blocked; Stage 3 remains outside scope.
- 2026-06-24: Senior unblock pass prepared a fail-closed Stage 2 emission contract in the ADR. `OO-VBA-I2` is now pending implementation, not blocked, but only as an analyzer-gated preview emitter with empty emitted strings on unsupported operations, truncation, redaction, analyzer failure, or unknown operation mapping.
- 2026-06-24: `OO-VBA-I2` completed. Added `excel.translate_vba_to_onlyoffice_js_preview` with analyzer-first emission, empty `macro_value` / `function_body` on unsafe states, and no workbook/runtime/generic translator expansion.
- 2026-06-24: Validation: `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed 39/39; `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension` passed. OntoIndex `gn_test_gap` PASS; `gn_verify_diff` remains globally noisy because the worktree contains unrelated dirty files.
- 2026-06-24: Docs-only ADR drift repair completed. ADR now reflects Stage 0-2 as completed, Stage 2 as a completed fail-closed contract, and Stage 3 as not approved; no code changes or tests were needed for this repair.
- 2026-06-24: Audit follow-up hardening completed. Analyzer now fails closed on unrecognized executable statements and rejects unquoted alignment constants that the emitter cannot serialize; scoped Excel tests passed 41/41 and scoped `just fix` passed.
- 2026-06-25: Narrow post-closure `C1` parser augmentation completed from real workbook evidence. `Табель Макрос.xlsm` exposed `.FormulaLocal` as a real target-property variant, so the analyzer now treats `.FormulaLocal` the same as `.Formula` for literal formula assignments while keeping variable RHS fail-closed. Scoped verification passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed 50/50.
- 2026-06-25: Bounded audit manager loop reopened only two tasks from the latest ADR challenge. OntoIndex freshness stayed current at `2e72a6d25e147f0619863e7721107b6f11a87fc2`, but `ext/excel` symbol coverage was partial, so the manager used OntoIndex for repo freshness and direct source reads for final evidence. Opened `OO-VBA-AUD1` for Stage 3 status/state reconciliation across ADRs and tracking, and `OO-VBA-AUD2` for the Stage 0 call-catalog versus positive-coverage challenge. No implementation dispatch was approved in this loop.
- 2026-06-25: Continued bounded manager loop closed both macro-tracking audit tasks. `OO-VBA-AUD1` resolved as docs-only consistency repair, and `OO-VBA-AUD2` resolved by keeping the existing Stage 0 contract while opening one narrow code follow-up, `OO-VBA-COV1`, for positive wrap/alignment coverage. OntoIndex freshness was rechecked before the decision; direct source reads remained the final authority because symbol coverage for the newer ONLYOFFICE files was still partial.
- 2026-06-25: `OO-VBA-COV1` completed. Added positive analyzer and translator coverage for `Selection.WrapText = True`, numeric `Selection.HorizontalAlignment`, and quoted `Selection.VerticalAlignment`. Validation passed: `CARGO_BUILD_JOBS=8 just fmt` and `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed 52/52; the scoped test command also completed the repo-default bench-smoke tail.
