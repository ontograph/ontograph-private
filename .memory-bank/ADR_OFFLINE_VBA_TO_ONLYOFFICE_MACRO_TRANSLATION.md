# ADR: Offline VBA To ONLYOFFICE Macro Translation

## Status

Stage 0-3 completed narrowly; deferred follow-ons remain gated.

## Date

2026-06-24

## Context

The current Excel code-translation surface in `ontocode-rs/ext/excel` is intentionally narrow and already implemented:

- `excel.extract_vba_modules`
- `excel.extract_powerquery_queries`
- `excel.translate_vba_to_m_preview`
- `excel.translate_powerquery_to_sql_preview`
- `excel.analyze_vba_onlyoffice_migration`
- `excel.translate_vba_to_onlyoffice_js_preview`
- `excel.review_vba_onlyoffice_workbook`

The prior Excel code-translation ADR correctly rejected `excel.translate_vba_to_onlyoffice_javascript` because there was no concrete target runtime contract.

That specific blocker is weaker now. The checked-out ONLYOFFICE frontend provides a real spreadsheet macro target surface:

- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:462) seeds spreadsheet macros with `Api.GetActiveSheet()` and `Api.GetActiveWorkbook()`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1463) through [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1710) shows recorder-backed emitters for spreadsheet actions such as value/formula writes, formatting, sort, merge, filters, comments, hyperlinks, images, and shapes.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:225) shows the persisted macro payload shape with `macrosArray` and `current`.

At the same time, the usual repo constraints still apply:

- keep the owner in `ontocode-rs/ext/excel`
- keep outputs bounded and read-only
- do not introduce a generic `excel.translate` monolith
- do not build a second Excel workflow stack
- do not assume full VBA compatibility

This ADR consolidates the earlier solutions review into one robust proposal instead of keeping several equal-weight options open. The checked-out ONLYOFFICE files are evidence, not a durable dependency; Stage 0 pinned that evidence into a repo-local appendix before the model-visible ONLYOFFICE tools were added.

## Review And Challenge

### What is actually needed

The first useful product is not a full compiler and not a workbook migration bundle.

The useful product is a bounded offline migration assistant that:

1. parses a known subset of VBA
2. classifies what can and cannot map to ONLYOFFICE spreadsheet macros
3. emits preview JavaScript for the supported subset
4. returns deterministic blockers for the rest

That matches the existing `ext/excel` owner shape better than a large workflow tool.

### Why a pure pattern rewriter is not enough

A token or regex-based rewriter is attractive because it is cheap, but it fails exactly where the tool needs to be trustworthy:

- variable indirection
- nested member access
- simple control flow
- unsupported object model detection
- precise blocker reporting

A pattern rewriter is acceptable as a temporary bootstrap, but not as the long-term design authority.

### Why a workbook review bundle is the wrong first shape

A one-shot tool that inspects a workbook, extracts modules, translates them, and returns a migration bundle would recreate the broad workflow pattern already rejected in the Excel ADR loop:

- mixes extraction, analysis, translation, and reporting
- is harder to bound
- pushes `ext/excel` toward a second orchestration stack

The current owner works better as explicit small primitives.

### Why the target runtime can now be treated as real

The ONLYOFFICE macro recorder is enough to define a first target contract for spreadsheet macros:

- available root objects such as `Api.GetActiveSheet()` and `Api.GetActiveWorkbook()`
- concrete emitted action shapes for cell edits, formatting, sorting, filters, comments, and related spreadsheet operations
- stable JavaScript-oriented output style already used by ONLYOFFICE itself

This still does not prove semantic parity with VBA. It does prove that Ontocode can target a concrete offline macro API rather than inventing one.

## Decision

Adopt a three-layer offline architecture under `ontocode-rs/ext/excel`:

1. `VBA subset parser`
2. `neutral spreadsheet migration IR`
3. `ONLYOFFICE Api.* emitter`

The current implementation is an analyzer + direct fail-closed emitter, with the neutral IR and module split kept as a future-only design target.

The user-visible tool family should stay explicit and small:

- `excel.analyze_vba_onlyoffice_migration`
- `excel.translate_vba_to_onlyoffice_js_preview`
- `excel.review_vba_onlyoffice_workbook` as a thin read-only workbook wrapper over the same analyzer/emitter pair

The analysis tool is the safety gate. The translation tool is preview-only and only emits JavaScript for the subset that the analyzer has proven translatable.

Stage 0 was a hard gate. The target ONLYOFFICE macro contract was captured from a specific ONLYOFFICE commit and checked into the repo as bounded evidence before the analyzer and preview translator landed. A narrow Stage 3 follow-on later landed as `excel.review_vba_onlyoffice_workbook`, but only as a read-only composition tool; it did not reopen runtime execution, bundle generation, or generic translation routing.

## ONLYOFFICE Target Examples

Stage 0 turned the examples below into a checked appendix before implementation started. These examples are evidence from the current checkout, not an implicit dependency on `tmp/onlyoffice`.

### Macro payload wrapper

ONLYOFFICE records macro values as an IIFE string and stores that string in `macrosArray[].value`:

- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:243) builds the macro name and value.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:245) stores `guid`, `name`, `autostart`, and `value`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:253) persists the macro JSON through `asc_setMacros(...)`.

Target shape:

```javascript
(function()
{
    let worksheet = Api.GetActiveSheet();
    let workbook = Api.GetActiveWorkbook();
})();
```

The Ontocode preview tool should return this as `macro_value` and return the inner body separately as `function_body`.

### Spreadsheet roots

ONLYOFFICE's recorder seeds spreadsheet macros with active sheet and workbook roots:

- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:465) defines the spreadsheet default-variable path.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:467) emits `Api.GetActiveSheet()`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:468) emits `Api.GetActiveWorkbook()`.

Target snippet:

```javascript
let worksheet = Api.GetActiveSheet();
let workbook = Api.GetActiveWorkbook();
```

### Cell value and formula writes

ONLYOFFICE emits active-cell writes through `worksheet.GetActiveCell()`:

- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1533) formats string and numeric cell values.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1539) emits `SetValue(...)`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1541) formats string and numeric formulas.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1547) emits `SetFormulaArray(...)`.

VBA input example:

```vb
Sub FillCell()
    ActiveCell.Value = "Ready"
    ActiveCell.Formula = "=SUM(A1:A3)"
End Sub
```

Expected preview shape:

```javascript
(function()
{
    let worksheet = Api.GetActiveSheet();
    let workbook = Api.GetActiveWorkbook();
    worksheet.GetActiveCell().SetValue("Ready");
    worksheet.GetActiveCell().SetFormulaArray("=SUM(A1:A3)");
})();
```

### Formatting operations

ONLYOFFICE's recorder routes common formatting through `Api.GetSelection()`:

- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1465) emits `SetFontSize(...)`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1466) emits `SetFontName(...)`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1467) emits `SetBold(...)`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1468) emits `SetItalic(...)`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1523) emits `SetFontColor(...)`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1529) emits `SetBackgroundColor(...)`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1661) emits `SetNumberFormat(...)`.

VBA input example:

```vb
Sub FormatSelection()
    Selection.Font.Bold = True
    Selection.Font.Italic = False
    Selection.Font.Size = 12
    Selection.NumberFormat = "0.00"
End Sub
```

Expected preview shape:

```javascript
(function()
{
    let worksheet = Api.GetActiveSheet();
    let workbook = Api.GetActiveWorkbook();
    Api.GetSelection().SetBold(true);
    Api.GetSelection().SetItalic(false);
    Api.GetSelection().SetFontSize("12");
    Api.GetSelection().SetNumberFormat("0.00");
})();
```

### Deferred spreadsheet operations

The recorder proves useful later targets, but these should stay deferred until the analyzer and IR are proven:

- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1640) emits merge and unmerge calls.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1650) emits sort calls.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1696) emits `SetAutoFilter()`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1702) emits `Api.GetRange(...).Select()`.

Do not include these in the first analyzer slice unless Stage 0 fixtures prove the exact operation contract and tests cover blocker behavior for unsupported variants.

## Implementation Shape

### Owner

Keep all logic in `ontocode-rs/ext/excel`.

Do not extend [ontocode-rs/ext/excel/src/vba_translate.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/vba_translate.rs:1) into a second monolith. OntoIndex evidence already shows that file is large enough.

### New modules and implementation shape

The current implementation is a unified analyzer + direct fail-closed emitter. Splicing this into a neutral IR or separate modules (like `vba_onlyoffice_ir.rs` and `vba_onlyoffice_emit.rs`) remains a future-only optimization.

### Parser

The parser is a hand-rolled bounded parser for procedure bodies, literals, simple assignments, and direct member chains. Do not add a broad VBA grammar or parser dependency without a follow-up ADR and dependency review.

### Tool contracts

`excel.analyze_vba_onlyoffice_migration`

- input:
  - `source_text`
  - optional `source_name`
- output:
  - `procedures`
  - `supported_operations`
  - `unsupported_operations`
  - `requires_manual_rewrite`
  - `warnings`
  - `success`

`excel.translate_vba_to_onlyoffice_js_preview`

- input:
  - `source_text`
  - optional `source_name`
- output:
  - `macro_value`
  - `function_body`
  - `procedure_summaries`
  - `unsupported_operations`
  - `redactions`
  - `warnings`
  - `success`

`macro_value` is the paste-ready ONLYOFFICE macro value: an IIFE shaped like `(function() { ... })();`, matching the persisted macro value observed in ONLYOFFICE's recorder. `function_body` is the bounded inner JavaScript body for review and diffing. The tool must not emit or write the surrounding `macrosArray` payload in the first slice.

### Stage 2 Completed Contract

Stage 2 was completed only as a fail-closed preview emitter over the completed Stage 1 analyzer result. It is not a second parser, not a workbook bundle, and not a runtime validator.

The translator must call the analyzer first and may emit JavaScript only when all of the following are true:

- analyzer `success` is `true`
- `requires_manual_rewrite` is `false`
- `unsupported_operations` is empty
- no warning contains truncation, redaction, missing-procedure, or missing-supported-operation language
- no supported operation contains a redacted value
- every supported operation name is in the explicit Stage 2 mapping table below
- every value expression is fully mapped to a supported target (unmapped value-expression fail-closed hardening)
- every unrecognized executable VBA statement is classified as unsupported rather than ignored

If any condition fails, the translator must return `success: false`, preserve analyzer `procedure_summaries`, `unsupported_operations`, `redactions`, and `warnings`, and return empty `macro_value` and `function_body`. This makes redaction, truncation, unsupported constructs, and analyzer uncertainty non-emitting states.

Stage 2 reuses the Stage 1 analyzer's parser/classifier. It must not introduce a broad VBA grammar dependency, workbook-assisted orchestration, ONLYOFFICE runtime execution, or generic `excel.translate` facade.

Allowed first Stage 2 operation mappings:

| Analyzer operation | ONLYOFFICE preview emission |
| --- | --- |
| `SetCellValue` | `worksheet.GetActiveCell().SetValue(...)` |
| `SetCellFormula` | `worksheet.GetActiveCell().SetFormulaArray(...)` |
| `SetFontBold` | `Api.GetSelection().SetBold(...)` |
| `SetFontItalic` | `Api.GetSelection().SetItalic(...)` |
| `SetFontName` | `Api.GetSelection().SetFontName(...)` |
| `SetFontSize` | `Api.GetSelection().SetFontSize(...)` |
| `SetTextColor` | `Api.GetSelection().SetFontColor(Api.CreateColorFromRGB(...))` |
| `SetFillColor` | `Api.GetSelection().SetBackgroundColor(Api.CreateColorFromRGB(...))` |
| `SetNumberFormat` | `Api.GetSelection().SetNumberFormat(...)` |
| `SetWrap` | `Api.GetSelection().SetWrap(...)` |
| `SetAlignHorizontal` | `Api.GetSelection().SetAlignHorizontal(...)` |
| `SetAlignVertical` | `Api.GetSelection().SetAlignVertical(...)` |

The emitter must include the Stage 0 roots before emitted operations:

```javascript
let worksheet = Api.GetActiveSheet();
let workbook = Api.GetActiveWorkbook();
```

Implementation tests required for Stage 2:

- supported value/formula preview golden output
- supported formatting preview golden output, including RGB color conversion
- unsupported construct returns `success: false` and empty emitted strings
- analyzer truncation returns `success: false` and empty emitted strings
- analyzer redaction returns `success: false` and empty emitted strings
- unknown analyzer operation returns `success: false` and empty emitted strings
- unrecognized executable VBA statement returns `success: false` and empty emitted strings
- analyzer/emitter contract mismatch values, including unquoted alignment constants, return `success: false` and empty emitted strings
- tool registration test for `excel.translate_vba_to_onlyoffice_js_preview`

### Neutral IR

The IR should model spreadsheet intent, not VBA syntax and not ONLYOFFICE syntax.

Useful first IR operations:

- `SelectRange`
- `SetActiveCellValue`
- `SetActiveCellFormula`
- `SetFontBold`
- `SetFontItalic`
- `SetFontName`
- `SetFontSize`
- `SetFillColor`
- `SetTextColor`
- `SetNumberFormat`
- `SetWrap`
- `SetAlignHorizontal`
- `SetAlignVertical`
- `SetMerge`
- `SetSort`
- `SetAutoFilter`
- `AddComment`
- `AddHyperlink`

This keeps parsing, analysis, and JS emission loosely coupled.

## Supported First Slice

Start with recorder-like spreadsheet macros only.

Supported in the first analyzer slice:

- `Sub ... End Sub`
- string and numeric literals
- direct `Range(...)`, `Cells(...)`, `Selection`, and `ActiveCell` patterns when statically resolvable
- direct assignments to cell values and formulas
- direct formatting operations

Deferred until the analyzer and IR are proven:

- merge/unmerge
- sort/filter
- simple comments and hyperlinks

Unsupported in v1:

- workbook and worksheet events
- `Function` procedures with return semantics
- `On Error`
- COM automation
- external DLL calls
- user forms
- shell, file, or network I/O
- late-bound object access
- dynamic invocation patterns such as `CallByName`
- broad chart/pivot automation beyond trivial recorder-like cases

Unsupported constructs must become explicit blockers, not guessed output.

## Bounds And Redaction

The tools are model-visible and must treat VBA source as untrusted local code.

Required limits:

- source input cap
- output JavaScript cap
- procedure count cap
- warning and blocker count caps
- per-literal cap before a value can appear in output

Required redaction:

- connection strings
- passwords and token-looking literals
- authorization headers
- local keychain or credential-store paths
- full local filesystem paths when they appear inside VBA literals
- URLs with embedded credentials

Redacted values must be replaced consistently enough for review, but the tool must not preserve raw secrets in `macro_value`, `function_body`, warnings, blockers, or debug text.

## Validation Strategy

This feature is only acceptable offline if validation is stronger than the existing heuristic translators.

Required validation:

- parser tests for the supported VBA subset
- golden tests for analyzer classification
- golden tests for emitted ONLYOFFICE JavaScript
- negative tests proving unsupported constructs become blockers
- redaction tests proving secrets and credential paths do not appear in output
- cap tests for source, output, procedure, warning, blocker, and literal limits
- fixture drift checks against the observed ONLYOFFICE recorder surface

The ONLYOFFICE recorder source should be reused as static evidence for emitter mappings and test fixtures, not as a runtime dependency.

## Staged Path

### Stage 0: Target contract capture

Record the observed ONLYOFFICE spreadsheet macro surface in repo-local fixtures or checked markdown/testdata.

Captured as [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION_STAGE0_TARGET_CONTRACT.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION_STAGE0_TARGET_CONTRACT.md).

Required Stage 0 outputs:

- ONLYOFFICE repository and commit id
- supported `Api.*` call catalog for the first slice
- expected macro output wrapper shape
- checked fixture or appendix containing the recorder-derived evidence
- example input/output pairs for the supported first-slice VBA subset
- drift check that fails when the fixture no longer matches the supported emitter catalog

Historical Stage 0 output: no new user-visible tool landed in this stage.

### Stage 1: Analyzer

Implemented `excel.analyze_vba_onlyoffice_migration`.

This stage proved:

- the supported VBA subset
- blocker classification
- the neutral IR boundary
- bounds and redaction behavior

### Stage 2: Preview translator

Implemented `excel.translate_vba_to_onlyoffice_js_preview`.

This stage emits bounded preview JavaScript only for analyzer-approved constructs.

Status: Completed.

### Stage 3: Workbook-assisted flow only if proven necessary

If repeated usage justifies it later, `excel.extract_vba_modules` can feed the analyzer and preview translator more directly in model workflows.

Do not approve a workbook review bundle in this ADR.

## Rejected Alternatives

### Generic `excel.translate` tool

Rejected.

It would mix unrelated translation paths and recreate the monolith pattern already rejected elsewhere in the Excel tool family.

### Pattern-only VBA rewriter as the primary design

Rejected as the main architecture.

It is too brittle to be the authoritative migration surface, though a tiny bootstrap pass could reuse some of its ideas internally.

### One-shot workbook review bundle

Rejected for the first slice.

It is too broad and too close to the donor workflow shape.

### Runtime execution or online validation against ONLYOFFICE

Rejected for this ADR.

The design target is offline analysis and preview generation only.

### Broad parser dependency in the first slice

Rejected for the first slice.

Start with a bounded local parser. A grammar dependency can be reconsidered only if the first parser becomes the limiting factor and the dependency review shows clear value.

## Consequences

### Positive

- gives Ontocode a real offline-only path for VBA-to-ONLYOFFICE migration assistance
- keeps the work inside the existing `ext/excel` owner
- reuses the current explicit tool-family style
- produces deterministic blockers instead of speculative output
- avoids building a second Excel workflow stack

### Negative

- implementation is materially larger than a heuristic translator
- only a subset of VBA will be supported
- a local bounded parser must be maintained unless a later ADR accepts a parser dependency
- workbook-level orchestration remains manual in the first slice
- Stage 0 adds up-front work before any user-visible tool can land

## Final Recommendation

The accepted Stage 0-2 implementation should continue to stay within this shape:

- owner: `ontocode-rs/ext/excel`
- architecture: `VBA subset parser -> neutral spreadsheet IR -> ONLYOFFICE Api.* emitter`
- tools:
  - Stage 1 completed: `excel.analyze_vba_onlyoffice_migration`
  - Stage 2 completed under the fail-closed Stage 2 Completed Contract: `excel.translate_vba_to_onlyoffice_js_preview`
- scope: offline, read-only, preview-only
- non-scope:
  - no generic translator
  - no one-shot workbook review bundle
  - no runtime execution
  - no full VBA compatibility claim

The current accepted implementation is Stage 0, Stage 1 (analyzer), and Stage 2 (preview translator). The preview translator does not expand beyond the fail-closed Stage 2 Completed Contract: analyzer success is the gate, redaction and truncation are non-emitting states, and the emitter is limited to the explicit mapping table.
