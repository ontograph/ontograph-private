# Offline VBA To ONLYOFFICE Readiness Task Closure

Date: 2026-06-25

## Scope

Close the five senior-opened readiness tasks in [ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_DETAILED_PROJECT_PLAN.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_DETAILED_PROJECT_PLAN.md) using current local corpus evidence plus OntoIndex visibility checks.

## Roles

- manager: current session
- senior-reviewer: handled by manager locally because this is a bounded no-dispatch readiness pass
- implementation-worker: not dispatched
- verification-worker: handled by manager locally because this is documentation and queue-state verification only

## Evidence

- OntoIndex `gn_ensure_fresh` reports `codex` fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2`.
- Dirty-worktree caveat remains active with `dirtyFileCount=258` and `scopeConfidence=medium`.
- Current analyzer operation family is read from `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs`.
- Current preview emitter operation family is read from `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`.
- Local corpus inputs reviewed:
  - `tmp/vba-samples/tabell.vba`
  - `tmp/vba-samples/essbase.vba`
  - `tmp/vba-samples/mylo.vba`
- Stage 0 recorder contract remains pinned in [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION_STAGE0_TARGET_CONTRACT.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION_STAGE0_TARGET_CONTRACT.md) to `https://github.com/ONLYOFFICE/sdkjs.git` commit `72b0421c0bbf9d01eed9cf14834ae47eb2df1b50`.

## SRT-1 Redacted Snippet Ledger

| Snippet label | Workbook source | Redaction status | Classification | Current result |
| --- | --- | --- | --- | --- |
| `formula-local-variable-rhs` | `tabell.vba` | safe after no secret rewrite | semantics-blocked | blocked candidate |
| `font-colorindex-write` | `tabell.vba` | safe | semantics-blocked | blocked candidate |
| `fill-colorindex-write` | `tabell.vba` | safe | semantics-blocked | blocked candidate |
| `comment-visible-false` | `tabell.vba` | safe | out of scope | blocked candidate |
| `selection-numberformat-literal` | `essbase.vba` | safe | already supported | duplicate of shipped capability |
| `shape-textframe-write` | `essbase.vba` | safe | out of scope | blocked candidate |
| `dynamic-formula-concat` | `essbase.vba` | safe | semantics-blocked | blocked candidate |
| `row-height-literal` | `essbase.vba` | safe | semantics-blocked | blocked candidate |
| `sheet-visible-hidden` | `mylo.vba` | safe | out of scope | blocked candidate |
| `workbook-protect-password` | `mylo.vba` | requires password redaction | out of scope | blocked candidate |

Decision:

- `SRT-1` is closed.
- The current corpus does not produce a second justified reopen beyond the already closed `.FormulaLocal` target-variant work.

## SRT-2 Supported-Versus-Corpus Matrix

Current supported operation family:

- `SetCellValue`
- `SetCellFormula`
- `SetFontBold`
- `SetFontItalic`
- `SetFontName`
- `SetFontSize`
- `SetTextColor`
- `SetFillColor`
- `SetNumberFormat`
- `SetWrap`
- `SetAlignHorizontal`
- `SetAlignVertical`

Matrix:

| Bucket | Corpus families |
| --- | --- |
| already supported | literal `Selection.NumberFormat = "#,##0.00"`; literal `.Formula` and `.FormulaLocal` shapes already covered in shipped tests |
| syntax-only gap | none proven from the current three-file corpus |
| semantics-blocked | variable RHS `.FormulaLocal = er`; dynamic formula concatenation; `.Font.ColorIndex`; `.Interior.ColorIndex`; `.Rows(...).RowHeight = 15` |
| out of scope | comments visibility; shapes/text boxes; sheet visibility; workbook or worksheet protection; workbook or event lifecycle logic |

Decision:

- `SRT-2` is closed.
- `E3`, `A2`, `B1`, and `C2` remain closed because the corpus still shows either semantics-blocked or out-of-scope families rather than a clean owner-local reopen.

## SRT-3 Fresh Corpus Intake Gate

Accepted intake gate:

- record the exact workbook or extracted module path
- state what is materially new versus the existing `tabell` / `essbase` / `mylo` set
- classify the sample as recorder-shape formatting, utility-style helper logic, or workbook/event semantics before review starts
- confirm redaction feasibility before any reopen discussion
- name the first expected slice up front, for example `C1 literal formula variant` or `E3 recorder drift proof`
- reject the sample immediately when it only adds more workbook lifecycle, protection, visibility, shape, or event semantics already classified out of scope

Decision:

- `SRT-3` is closed.
- Fresh sample count alone is not a trigger.

## SRT-4 Recorder Drift Watch

Pinned recorder evidence:

- source repo: `https://github.com/ONLYOFFICE/sdkjs.git`
- source commit: `72b0421c0bbf9d01eed9cf14834ae47eb2df1b50`
- source path: `tmp/onlyoffice/sdkjs/common/macro-recorder.js`

Currently relied-on recorder-grounded calls:

- `Api.GetActiveSheet()`
- `Api.GetActiveWorkbook()`
- `Api.GetSelection()`
- `worksheet.GetActiveCell()`
- `Api.CreateColorFromRGB(...)`
- `SetBold`
- `SetItalic`
- `SetFontSize`
- `SetFontName`
- `SetFontColor`
- `SetBackgroundColor`
- `SetNumberFormat`
- `SetWrap`
- `SetAlignHorizontal`
- `SetAlignVertical`
- `SetValue`
- `SetFormulaArray`

Counts as real drift:

- IIFE wrapper shape changes
- `macrosArray` or `current` persistence shape changes
- any relied-on call above disappears, renames, or changes ownership/root shape
- formula emission no longer maps through the active-cell recorder path
- a currently deferred family becomes recorder-standard and materially simpler to support

Harmless formatting noise:

- whitespace, indentation, or quote-style differences
- local variable naming differences that preserve the same call graph shape
- statement ordering changes inside the recorder that do not change emitted macro semantics

Decision:

- `SRT-4` is closed.
- `E3` remains documentation or fixture-planning only; no runtime validation is opened.

## SRT-5 OntoIndex Coverage Audit For `ext/excel`

Current OntoIndex visibility:

- visible cleanly:
  - `ontocode-rs/ext/excel/src/extension.rs`
  - `ontocode-rs/ext/excel/src/vba_translate.rs`
- not visible cleanly and reported as `file not in index`:
  - `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs`
  - `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`
  - `ontocode-rs/ext/excel/src/vba_onlyoffice_workbook_review.rs`

Decision:

- `SRT-5` is closed.
- A casual `ontoindex analyze` is not justified from this readiness pass alone.
- Until a real implementation reopen needs richer symbol ownership on the ONLYOFFICE Excel files, direct-source fallback should remain the default.

## Final Queue Decision

No implementation-worker dispatch is justified from these readiness closures.

What changed:

- the queue is better instrumented for future reopens
- the current corpus is classified and partially deduplicated
- the recorder drift contract is cheaper to re-check later

What did not change:

- no deferred implementation slice reopened
- no public `excel.translate` surface opened
- no runtime ONLYOFFICE validation opened
- no broad VBA parser dependency opened
- no OntoIndex reindex was scheduled
