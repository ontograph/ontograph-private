# Offline VBA To OnlyOffice Macro Translation Solutions Review

## Scope

Propose several offline-only Ontocode tool designs for converting Excel VBA into ONLYOFFICE JavaScript macros without adding an LLM dependency, external service dependency, or a second Excel stack outside `ontocode-rs/ext/excel`.

This is a design review only. It does not reopen implementation automatically.

## Why this is reopened now

The last Excel code-translation ADR rejected `excel.translate_vba_to_onlyoffice_javascript` because there was no concrete target runtime contract.

That specific blocker is weaker now because the checked-out ONLYOFFICE frontend shows a real macro target surface:

- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:462) seeds spreadsheet macros with `Api.GetActiveSheet()` and `Api.GetActiveWorkbook()`.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1463) through [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1710) contain a large action-to-JavaScript emitter for spreadsheet actions such as font changes, borders, sort, merge, hyperlinks, filters, comments, images, and shapes.
- [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:225) shows persisted macro payload shape with `macrosArray` and `current`, stored through `asc_setMacros(...)`.

This still does not prove full semantic equivalence for VBA. It does prove that a bounded offline translator can target a specific macro runtime instead of inventing one.

## Current owner constraints

The current accepted Excel owner is still `ontocode-rs/ext/excel`.

Relevant current surfaces:

- [ontocode-rs/ext/excel/src/extension.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/extension.rs:17) already installs explicit small tools rather than a generic `excel.translate` dispatcher.
- [ontocode-rs/ext/excel/src/vba_extract.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/vba_extract.rs:145) already extracts bounded workbook VBA.
- [ontocode-rs/ext/excel/src/vba_translate.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/vba_translate.rs:199) already performs source-first heuristic translation for `VBA -> M`.
- OntoIndex module evidence shows [ontocode-rs/ext/excel/src/vba_translate.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/vba_translate.rs:1) is already 717 lines, so new macro-target work should go into new owner-local modules, not into the existing file.

## Non-goals

- No online model inference.
- No code execution against ONLYOFFICE during translation.
- No file writes into ONLYOFFICE projects or workbook mutation in the first slice.
- No generic `excel.translate` monolith.
- No one-shot workbook review bundle in the first slice.
- No promise of full VBA compatibility.

## Core design rule

The first useful product is not a compiler. It is a bounded migration assistant:

1. parse a known subset of VBA offline
2. map supported constructs to ONLYOFFICE `Api.*` macros
3. emit preview JavaScript plus explicit blockers for everything else

That is smaller, safer, and already matches how `ext/excel` works today.

## Solution 1: Token And Pattern Rewriter

### Summary

Add a narrow heuristic tool that rewrites common VBA snippets directly into ONLYOFFICE macro snippets using pattern rules and string templates.

### Proposed tool

- `excel.translate_vba_to_onlyoffice_js_preview`

### Likely shape

- input:
  - `source_text`
  - optional `source_name`
  - optional `target_kind` with allowed values like `spreadsheet`
- output:
  - `javascript`
  - `supported_patterns`
  - `unsupported_patterns`
  - `warnings`
  - `success`

### How it would work offline

- Reuse current source-size caps from `vba_translate.rs`.
- Detect a tiny subset with token or line-pattern matching:
  - active cell value writes
  - formula writes
  - font size/name/bold/italic
  - alignment
  - fill/text color
  - sort/filter
  - merge/unmerge
- Emit ONLYOFFICE snippets modeled on [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1463).

### Pros

- Smallest diff.
- No new parser dependency required.
- Gets to a usable preview quickly for repetitive recorder-like VBA.

### Cons

- Brittle.
- Hard to explain failures precisely.
- Will collapse when VBA control flow or variable indirection appears.

### Verdict

Acceptable only as a very small bootstrap slice. Good for proving demand, not a long-term owner shape.

## Solution 2: AST To Neutral Spreadsheet IR To ONLYOFFICE Emitter

### Summary

Parse a bounded subset of VBA into an AST, lower supported constructs into a tiny neutral IR, then emit ONLYOFFICE macro JavaScript from that IR using recorder-derived templates.

### Proposed tools

- `excel.analyze_vba_onlyoffice_migration`
- `excel.translate_vba_to_onlyoffice_js_preview`

### Proposed internal modules

- `vba_parse.rs`
- `vba_onlyoffice_ir.rs`
- `vba_onlyoffice_emit.rs`
- `vba_onlyoffice_tool.rs`

### IR examples

- `SelectRange("A1:B5")`
- `SetActiveCellValue(String|Number|Formula)`
- `SetFontBold(bool)`
- `SetFontName(String)`
- `SetFontSize(Number)`
- `SetFillColor(Rgb)`
- `SetNumberFormat(String)`
- `SetWrap(bool)`
- `SetSort(order)`
- `SetAutoFilter`
- `AddComment{text, author}`

### How it would work offline

- Parse only procedure bodies and a small statement subset.
- Support literals, simple variable assignment, and direct member-call chains.
- Reject unsupported constructs deterministically:
  - user forms
  - COM objects
  - file system access
  - workbook event hooks
  - external DLL calls
  - `On Error`
  - dynamic `CallByName`
  - late-bound objects
- Emit ONLYOFFICE JavaScript from the IR with templates grounded in recorder code.

### Pros

- Best balance of rigor and containment.
- Easier to test than regex rewriting.
- Lets Ontocode explain exactly why translation failed.
- Keeps target-specific logic isolated from VBA parsing.

### Cons

- More work than Solution 1.
- Needs a parser decision:
  - vendored grammar/parser
  - hand-rolled bounded parser
  - minimal dependency chosen only if clearly justified

### Verdict

This is the recommended direction if Ontocode ever implements this feature.

## Solution 3: Recorder-Corpus-Assisted Translator

### Summary

Use ONLYOFFICE's own macro recorder source as a static corpus to build and verify the emitter catalog offline.

This is not a separate user-facing tool family. It is support infrastructure for Solution 2.

### Proposed internal helpers

- `scripts/extract_onlyoffice_macro_surface.*`
- generated fixture file under `ext/excel/testdata/`

### How it would work offline

- Scan [tmp/onlyoffice/sdkjs/common/macro-recorder.js](/opt/demodb/_workfolder/ontocode/tmp/onlyoffice/sdkjs/common/macro-recorder.js:1) for `cellActions` and related emitters.
- Extract stable API templates and operation names into test fixtures or a checked-in mapping table.
- Use those fixtures to:
  - keep emitter code aligned with the observed ONLYOFFICE macro surface
  - drive snapshot tests for translated output
  - flag when upstream ONLYOFFICE changes macro API spellings

### Pros

- Reuses the target project's own emitter knowledge.
- Strengthens offline verification without running ONLYOFFICE.
- Reduces hand-maintained target mapping drift.

### Cons

- Adds build/test plumbing.
- Still does not solve VBA parsing by itself.

### Verdict

Strong companion to Solution 2. Weak as a standalone product.

## Solution 4: Workbook Migration Review Bundle

### Summary

Take extracted workbook modules, analyze them all, translate what is possible, and return a workbook-level migration report.

### Proposed tool

- `excel.review_onlyoffice_macro_candidates`

### Why this looks attractive

- Better UX for `.xlsm` migration projects.
- Lets the user point at a workbook and get a triaged answer.

### Why this should not be first

- Recreates the broad workflow bundle previously rejected in the Excel ADR loop.
- Mixes extraction, analysis, translation, and reporting.
- Harder to bound cleanly.

### Verdict

Keep rejected for the first implementation slice. Revisit only after Solution 2 proves stable contracts.

## Recommended tool set

If this is reopened for implementation, keep it small:

### User-visible first slice

- `excel.analyze_vba_onlyoffice_migration`
- `excel.translate_vba_to_onlyoffice_js_preview`

### Why two tools

- analysis and translation should stay separate, matching the current `ext/excel` style
- analysis can fail safely with blockers even when translation cannot proceed
- translation can accept pasted VBA or output from `excel.extract_vba_modules`

### Suggested outputs

`excel.analyze_vba_onlyoffice_migration`

- `procedures`
- `supported_operations`
- `unsupported_operations`
- `requires_manual_rewrite`
- `warnings`
- `success`

`excel.translate_vba_to_onlyoffice_js_preview`

- `javascript`
- `procedure_summaries`
- `unsupported_operations`
- `warnings`
- `success`

## Minimal supported VBA subset

Do less on purpose. Start with recorder-like spreadsheet macros only:

- `Sub ... End Sub`
- scalar local variables
- string and numeric literals
- direct `Range(...)`, `Cells(...)`, `Selection`, `ActiveCell` patterns when statically resolvable
- assignment to cell value/formula
- formatting calls
- sort/filter/merge
- comment insertion
- simple loops only if they can be unrolled or lowered safely

Explicitly unsupported in v1:

- workbook and worksheet events
- `Function` return semantics
- `On Error`
- COM automation
- user forms
- chart object models beyond trivial recorder-like cases
- pivot tables
- file/network I/O
- shell calls

## Testing and validation

Offline validation should be stronger than the current heuristic translators because the target is riskier.

Required checks:

- unit tests for parser subset
- snapshot tests for emitted ONLYOFFICE JavaScript
- golden tests using small VBA fixtures and expected `Api.*` output
- negative tests proving unsupported constructs become blockers, not guessed code
- fixture drift check against extracted ONLYOFFICE recorder patterns

## Recommended staged path

### Stage 0: Static target contract capture

- add a repo-local note or checked fixture describing the observed ONLYOFFICE spreadsheet macro surface
- no user-visible tool yet

### Stage 1: Analysis-only tool

- implement `excel.analyze_vba_onlyoffice_migration`
- parse and classify only
- no JS emission yet

### Stage 2: Preview translator

- implement `excel.translate_vba_to_onlyoffice_js_preview`
- support only the analysis-approved subset
- return preview text only

### Stage 3: Workbook-assisted batch mode only if demanded

- optionally revisit a workbook review surface
- only after repeated evidence shows the two-tool primitive path is too manual

## Recommendation

If Ontocode wants this capability, reopen it as a narrower proposal than the previously rejected `excel.translate_vba_to_onlyoffice_javascript`.

Recommended acceptance shape:

- keep the owner in `ontocode-rs/ext/excel`
- choose Solution 2 as the main design
- borrow Solution 3 as test/support infrastructure
- reject Solution 4 for the first slice
- use Solution 1 only if a tiny bootstrap pass is needed before the AST path

In short:

`VBA subset -> neutral spreadsheet IR -> ONLYOFFICE Api.* emitter` is the smallest serious offline design.
