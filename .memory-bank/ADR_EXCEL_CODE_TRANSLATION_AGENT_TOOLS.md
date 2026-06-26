# ADR: Excel Code Translation Agent Tools

## Context

This proposal is based on the donor review in [tmp/excel/in2sql_dotNet_addin/docs/CODE_TRANSLATION_TOOLS_REVIEW.md](/opt/demodb/_workfolder/ontocode/tmp/excel/in2sql_dotNet_addin/docs/CODE_TRANSLATION_TOOLS_REVIEW.md:1).

The donor proves two useful translation surfaces:

- `PowerQuery M -> SQL`
- `VBA -> M -> SQL`

The donor does **not** prove a real `VBA -> OnlyOffice JavaScript` translator.

In this repo, the current Excel tool owner is already `ontocode-rs/ext/excel`.

When this ADR was opened, the model-visible Excel surface was limited to workbook metadata/data tools:

- `excel.inspect_workbook`
- `excel.read_sheet_preview`
- `excel.export_sheet_to_csv`

The bounded manager loop for this ADR has since implemented four additional translation-related tools under the same owner:

- `excel.extract_vba_modules`
- `excel.extract_powerquery_queries`
- `excel.translate_vba_to_m_preview`
- `excel.translate_powerquery_to_sql_preview`

## Review of current tools

### Current Ontocode Excel tools

Current owner: `ontocode-rs/ext/excel`

What they do well:

- inspect workbook package structure and markers
- preview bounded sheet content
- export one sheet to CSV for downstream agents
- extract bounded VBA source and metadata from workbooks
- extract bounded PowerQuery source and metadata from workbooks
- translate pasted or extracted VBA into heuristic M previews
- translate pasted or extracted PowerQuery into heuristic SQL previews

What they still do not do:

- translate VBA to SQL
- return reviewed candidate bundles for workbook migration

This remains correct for the narrowed scope. The Excel tools stay small, bounded, read-only, and owner-local inside `ext/excel`.

### Current donor transpiler tools

Current donor owner: `DataManager/Features/Transpiler`

Useful donor surfaces:

- `MToSqlTranslator`
- `VbaToMTranslator`
- `TranslationService`
- workbook VBA readers
- tests proving the translation paths work

Donor surfaces that should **not** be copied as-is into Ontocode model-visible tools:

- one big `transpiler.translate` entry point with mode switching
- `saveArtifactAs`
- `exportZip`
- `createSqlQuery`
- UI/WebView workflow helpers

Those are app-local workflow actions, not good model-visible agent tools.

## Goal

Add code-translation agent tools for Excel-related source while keeping the existing `ext/excel` owner and bounded tool shape.

## Proposal A: Source-first translation primitives in `ext/excel`

### Summary

Add small, explicit translation tools that accept source text directly and return bounded translation results.

### Proposed tools

1. `excel.translate_powerquery_to_sql_preview`
2. `excel.translate_vba_to_m_preview`

### Example shapes

`excel.translate_powerquery_to_sql_preview`

- input:
  - `source_text`
  - optional `source_name`
- output:
  - `sql`
  - `warnings`
  - `unsupported_functions`
  - `success`

`excel.translate_vba_to_m_preview`

- input:
  - `source_text`
  - optional `source_name`
- output:
  - `m_queries`
  - `modified_vba`
  - `warnings`
  - `success`

### Why this is the recommended path

- reuses the existing `ext/excel` owner
- keeps workbook extraction separate from translation
- avoids COM and workbook-format complexity in the first slice
- avoids donor UI workflow baggage
- keeps outputs bounded and model-visible
- maps cleanly to the donor engine surfaces

### Required constraints

- hard caps on source length and output length
- no file writes
- no direct query creation side effects
- deterministic warnings for unsupported functions/patterns
- explicit preview wording so the model does not treat heuristic SQL as guaranteed-correct executable migration output

### Challenge to this proposal

When this ADR was opened, this was not enough for the workbook-first `ext/excel` owner.

At that point, the extension could report workbook markers like VBA and PowerQuery presence, but it did not yet return the underlying source text. That is why source-first translation tools alone mainly helped pasted source rather than workbook-driven use, and why Proposal B was promoted into the first implementation slice.

## Proposal B: Workbook-assisted extraction plus source-first translation

### Summary

Add extraction tools that surface bounded workbook code, then feed their output into the source-first translation tools from Proposal A.

### Proposed tools

1. `excel.extract_powerquery_queries`
2. `excel.extract_vba_modules`
3. keep Proposal A translation tools separate

### Why this is useful

- better user ergonomics for `.xlsm`/`.xlsb` workflows
- natural fit with current workbook inspection tools
- aligns with donor helpers such as `WorkbookVbaSourceReader`, `VbaProjectReader`, and `MQueryParser`

### Why this should move earlier

- when this ADR was opened, current `ext/excel` tools were workbook-first, not source-first
- users with `.xlsm` or `.xlsb` files needed extraction before translation was useful
- donor helpers already proved workbook-side source discovery was a separate concern

### Constraints

- keep extraction read-only
- return bounded source text and metadata only
- do not auto-run translation as part of extraction
- keep PowerQuery and VBA extraction as separate tools

## Proposal C: Convenience VBA pipeline tool

### Summary

Add a one-step pipeline tool only if users repeatedly need `VBA -> M -> SQL` as one model-visible action.

### Proposed tool

1. `excel.translate_vba_to_sql_preview`

### Why this is not recommended first

- the donor only proves this as a composed pipeline, not as a distinct compiler surface
- it duplicates behavior that can already be expressed by `extract_vba_modules` + `translate_vba_to_m_preview` + `translate_powerquery_to_sql_preview`
- it should be justified by repeated usage, not assumed up front

## Proposal D: One-shot workbook translation review tool

### Summary

Add one tool that inspects a workbook, extracts candidate code, translates what it can, and returns a review bundle.

### Example

`excel.review_translation_candidates`

- input:
  - `path`
  - optional limits
- output:
  - `powerquery_candidates`
  - `vba_candidates`
  - translated SQL previews
  - blockers
  - warnings

### Why this is not the recommended first step

- too broad
- mixes inspection, extraction, translation, and migration review
- harder to bound
- too close to the donor app workflow shape
- likely to become a second Excel stack instead of extending the current one cleanly

This is acceptable only after smaller extraction and translation tools prove the right contracts.

## Proposal E: VBA to OnlyOffice JavaScript

### Summary

Do not plan this now as a normal Excel tool family extension.

### Reason

The donor review found no real translator for this path. The donor explicitly rejects an Office.js migration shape and uses VSTO + WebView2 instead.

### Decision

Reject for now unless a separate donor or target runtime contract appears that proves:

- the exact OnlyOffice JavaScript target surface
- the VBA subset worth translating
- the runtime APIs that translated code is allowed to call
- a validation strategy for behavior equivalence

Without that, this is speculation.

## Recommended path

The recommended path below was accepted and then executed through the bounded manager loop tracked in [ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS_TRACKING.md](ADR_EXCEL_CODE_TRANSLATION_AGENT_TOOLS_TRACKING.md).

Do not start with three translation tools.

Start with two small vertical slices that match the current workbook-first owner.

### Stage 1A: VBA slice

Implemented on 2026-06-24:

1. `excel.extract_vba_modules`
2. `excel.translate_vba_to_m_preview`

### Stage 1B: PowerQuery slice

Implemented on 2026-06-24:

1. `excel.extract_powerquery_queries`
2. `excel.translate_powerquery_to_sql_preview`

### Stage 2

Re-challenged on 2026-06-24 and closed deferred again. No new repeated-usage evidence or owner-local gap justified a dedicated convenience pipeline beyond the composed Stage 1A/1B primitives:

1. `excel.translate_vba_to_sql_preview`

This should be treated as a convenience pipeline over the earlier primitives, not as the primary first-stage contract.

### Stage 3

Re-challenged on 2026-06-24 and closed rejected again. No new donor/runtime contract or bounded-owner argument justified reviving the previously rejected workbook-review and broader workflow surfaces:

- `excel.review_translation_candidates`
- `excel.translate_vba_to_onlyoffice_javascript`
- `excel.translate` monolith with mixed modes and workflow side effects
- artifact-save, zip-export, and query-create tool surfaces

## Concrete design rules

- Keep everything under `ontocode-rs/ext/excel`
- Add new modules rather than growing one file into another monolith
- Do not keep growing `ontocode-rs/ext/excel/src/tool.rs` as the single home for all tool args, DTOs, specs, and handlers. New translation tool contracts should be split into owner-local modules.
- Keep translation tools read-only in early stages
- Do not add artifact export, ZIP creation, or query persistence as model-visible tools
- Do not add one generic `excel.translate` tool with a `mode` enum unless separate tools prove impossible
- Prefer explicit tool names over multi-mode dispatch
- Keep workbook extraction and code translation as separate surfaces
- Keep PowerQuery SQL generation conservative: unsupported `Table.SelectRows` predicates must fall back to warnings instead of guessed SQL
- Bound custom XML scanning and warning accumulation when probing workbook `DataMashup` payloads
- Preserve workbook PowerQuery presence markers even when `DataMashup` payload decoding fails

## Recommended tool set

Keep:

- `excel.inspect_workbook`
- `excel.read_sheet_preview`
- `excel.export_sheet_to_csv`

Implemented:

- `excel.extract_vba_modules`
- `excel.extract_powerquery_queries`
- `excel.translate_vba_to_m_preview`
- `excel.translate_powerquery_to_sql_preview`

Deferred:

- `excel.translate_vba_to_sql_preview`

Reject for now:

- `excel.review_translation_candidates`
- `excel.translate_vba_to_onlyoffice_javascript`
- `excel.translate` monolith with mixed modes and workflow side effects
- artifact-save, zip-export, and query-create tool surfaces
