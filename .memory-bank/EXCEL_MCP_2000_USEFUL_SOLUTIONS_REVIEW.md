# Excel MCP 2000 Useful Solutions Review

Status: distinct Excel donor-solution review artifact, OntoIndex-grounded, no implementation dispatch.
Date: 2026-06-25
Donor sources:
- `tmp/excel/mcp-server-excel`
- `tmp/excel/excel-mcp-server`
- `tmp/excel/vba-mcp-server`
- `tmp/excel/mcp-google-sheets`
- `tmp/excel/spreadsheet-mcp`
- `tmp/excel/Excel-mcp-server`
- `tmp/excel/negokaz-excel-mcp-server`
- `tmp/excel/in2sql_dotNet_addin` (formula/model challenge evidence only; not part of the original seven-MCP donor set)
Current repo: Ontocode independent fork, not official OpenAI/Azure.

## Senior Challenge

This is not a 2000-task implementation queue. The donor repos overlap heavily: most rows are the same spreadsheet behaviors repeated across different runtimes.

The only useful solutions are the ones that:

1. extend the current `ontocode-rs/ext/excel` owner or
2. stay outside core as a clearly separate external MCP companion

Rejected by default:
- full mutable Excel runtime inside `ext/excel`
- generic spreadsheet abstraction over unrelated owners
- cloud Sheets mixed into the current offline workbook/VBA owner
- broad side-effect workflow wrappers when the current repo already has bounded primitives
- live Excel execution bundled into current source-first translation tools

## Review Challenge Findings

- The donor set is not seven independent architectures. It collapses into a small set of recurring families: runtime split, validation, pagination, visual verification, VBA code mobility, cloud tool filtering, and chart/data-model workflow sequencing.
- `mcp-server-excel` is the richest donor but also the easiest way to overbuild. Its best value is workflow constraints and verification rules, not wholesale adoption.
- `excel-mcp-server` and `negokaz-excel-mcp-server` contribute the strongest file-based patterns: validation, metadata, pagination, dual backends, and style/formula presentation.
- `vba-mcp-server` contributes the cleanest live VBA maintenance surface, but it solves a different problem than the current offline `extract_vba_modules` and translation preview tools.
- `spreadsheet-mcp` is the stronger Google Sheets workflow donor; `mcp-google-sheets` is the stronger token-budget/tool-filtering donor.
- Formula conversion is underrepresented in the first 36 rows. The strongest evidence is not from the seven MCP servers but from `in2sql_dotNet_addin`, which has typed formula AST, formula dependency, graph, SQL planning/emission, and validation components.
- The formula evidence also argues against over-dispatch: dynamic arrays, array constants, external workbook links, volatile functions, row-relative plans without stable row identity, and unvalidated generated SQL must become blockers, not best-effort conversions.

## OntoIndex Evidence Sampled

- `mcp-server-excel`
  - indexed at `2026-06-25 21:45 UTC`
  - `11,048 nodes | 29,210 edges | 414 clusters | 300 flows`
  - sampled via OntoIndex query/context around `PowerQueryHelpers`, `ExcelScreenshotTool`, chart and datamodel workflows
- `excel-mcp-server`
  - `471 nodes | 1,775 edges | 14 clusters | 39 flows`
  - sampled via OntoIndex query/context around `create_chart_in_sheet`, `validate_formula_in_cell_operation`, `read_excel_range_with_metadata`
- `vba-mcp-server`
  - `183 nodes | 238 edges | 3 clusters | 14 flows`
  - sampled via OntoIndex context on `vba_write_module`
- `mcp-google-sheets`
  - `343 nodes | 598 edges | 20 clusters | 5 flows`
  - sampled via OntoIndex context on `_parse_enabled_tools`, `get_sheet_data`, `batch_update`
- `spreadsheet-mcp`
  - `286 nodes | 575 edges | 16 clusters | 24 flows`
  - sampled via OntoIndex query around chart, sort, share, and sheet flows
- `Excel-mcp-server`
  - `499 nodes | 981 edges | 12 clusters | 43 flows`
  - sampled via OntoIndex indexing plus direct source review of the dual `workbook`/`path` contract
- `negokaz-excel-mcp-server`
  - `748 nodes | 1,386 edges | 42 clusters | 64 flows`
  - sampled via OntoIndex query around pagination, formula rendering, screen capture, and dual backends
- `in2sql_dotNet_addin`
  - indexed as `codex (/opt/demodb/_workfolder/ontocode/tmp/excel/in2sql_dotNet_addin)`
  - sampled via direct source and OntoIndex list evidence around formula AST parsing, formula dependency detection, migration graph building, Formula SQL planning/emission, validation, and XLSB formula sidecars
- current Ontocode owner remains `ontocode-rs/ext/excel`, with active bounded owners in:
  - `tool.rs`
  - `preview.rs`
  - `export.rs`
  - `vba_extract.rs`
  - `powerquery_extract.rs`
  - `vba_translate.rs`
  - `powerquery_translate.rs`
  - `vba_onlyoffice_analyze.rs`
  - `vba_onlyoffice_translate.rs`
  - `vba_onlyoffice_workbook_review.rs`

## Verdict Counts

- KEEP-CANDIDATE: 44

`KEEP-CANDIDATE` means retained for owner-local review or external companion use. It does not mean dispatchable implementation work without a fresh ADR or a proven current gap.

## Family Counts

- RUNTIME: 6
- VALIDATION: 6
- VBA: 6
- VISUAL: 6
- CLOUD: 6
- SAFETY: 6
- FORMULA_MODEL: 8

## Dispatch Rule

Do not dispatch rows mechanically.

Dispatch only when one of these is true:
- current `ext/excel` has a proven missing bounded read/analyze/translate capability
- an explicit ADR approves a separate external MCP companion for live Excel or cloud Sheets
- a real test artifact proves the current tool family misses a behavior that one donor already solves cleanly

Keep all implementation inside the current owner unless the point is explicitly to integrate an external MCP server as a separate companion.

## Bounded Manager Loop Closure - 2026-06-25

Manager decision: no implementation-worker dispatch from this artifact.

Senior-reviewer fallback and verification-worker review both challenged the open rows against the current `ext/excel` owner and found no row that satisfies the Dispatch Rule above. `KEEP-CANDIDATE` remains a future-review label only.

Rows `010-012` and `037` are the closest future read-only candidates, but they still require a concrete test artifact or ADR-bounded owner gap before implementation:
- `010-012`: validation and metadata visibility for preview/inspect flows; useful later, but not a failing current behavior.
- `037`: read-only formula inventory; closest future enhancement, but current `read_sheet_preview` only exposes formula text plus cell value, not R1C1, number format, calculation mode, defined names, or external-link sidecars.

Rows `038-044` stay blocked for the current manager loop:
- `038-040`: formula AST, formula-to-SQL, and validation planning require a separate formula-conversion ADR and fail-closed artifacts.
- `041`: named-range metadata can be inspected later, but automatic formula rewrite to named ranges remains rejected without user-authored mapping and workbook tests.
- `042`: live `Formula2` and array-formula write semantics belong to a future external/live Excel companion or a separate ADR, not current offline `ext/excel`.
- `043-044`: workbook graph work must prove real edge extraction before any graph claim or implementation dispatch.

Next valid action is not implementation. The next valid action is to produce one of these unblock artifacts: a failing workbook fixture for validation/formula inventory, an ADR for formula AST/SQL conversion, or an ADR for a separate live/cloud Excel MCP companion.

## Senior Unblock Decision - 2026-06-26

Senior verdict: one implementation task is now unblocked, and two evidence-only tasks are opened. No formula-to-SQL, live Excel, cloud Sheets, or workbook-graph implementation is unblocked.

### EXCEL-MCP-U1 - implementation-ready

Fix `excel.inspect_workbook` so UTF-16 custom XML Power Query metadata does not make workbook inspection fail.

Evidence:
- `tmp/excel/in2sql_dotNet_addin/tools/WorkbookArtifactExtractor/examples/pq_test.xlsx`
- `tmp/excel/in2sql_dotNet_addin/tools/WorkbookArtifactExtractor/examples/pq_test.xlsm`
- both currently fail with `failed to read workbook entry customXml/item1.xml: stream did not contain valid UTF-8`
- `customXml/item1.xml` starts with UTF-16LE BOM and declares `encoding="utf-16"`
- current `backend.rs` reads optional XML entries through `read_to_string`, then `power_query_parts` scans custom XML for `DataMashup`

Implementation scope:
- stay inside `ontocode-rs/ext/excel/src/backend.rs`
- keep the existing package-budget limits
- decode UTF-8 and UTF-16 BOM XML, or degrade unreadable custom XML to a warning instead of failing the whole inspection
- add the smallest focused test using the donor workbook artifact or a tiny generated zip fixture

Do not add:
- a generic encoding framework
- formula inventory
- formula-to-SQL
- live Excel automation
- graph extraction

### EXCEL-MCP-U2 - evidence-only

Create a minimal workbook proof for rows `010-012` before any validation-metadata implementation.

Required proof:
- workbook fixture with inline list validation and range-backed list validation
- current `read_sheet_preview`/`inspect_workbook` output showing validation metadata is absent
- expected bounded output shape copied from donor evidence, not a new mutable validation API

Donor evidence:
- `tmp/excel/excel-mcp-server/src/excel_mcp/cell_validation.py`
- `tmp/excel/excel-mcp-server/src/excel_mcp/data.py`

Implementation remains blocked until this proof exists.

### EXCEL-MCP-U3 - evidence-only

Create a minimal workbook proof for row `037` formula inventory before any formula-sidecar implementation.

Required proof:
- workbook fixture with formulas, cached values, number format, calculation mode, and external-link or named-range risk where possible
- current `read_sheet_preview` output showing only formula text/value is exposed
- expected bounded read-only inventory shape, explicitly excluding SQL generation and formula rewriting

Donor evidence:
- `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Migration/Formula/WorksheetFormulaMetadataDocument.cs`
- `tmp/excel/in2sql_dotNet_addin/tools/WorkbookArtifactExtractor/WorkbookArtifactExtractor.OpenXml/Xlsb/FormulaSidecarModels.cs`

Implementation remains blocked until this proof exists.

Still blocked:
- rows `038-040`: require a separate formula AST / formula-to-SQL ADR and fail-closed validation contract
- row `041`: named-range inspection can be proposed later, but formula rewrite to named ranges remains rejected without explicit user mapping and workbook tests
- row `042`: live `Formula2` semantics belong to a separate live Excel companion ADR
- rows `043-044`: workbook graph work requires real edge extraction proof before any implementation claim

## New Samples Review - 2026-06-26

Sample folder reviewed: `tmp/excel/samples`.

New implementation-ready task:

### EXCEL-MCP-U4 - implementation-ready

Fix `excel.read_sheet_preview` formula text decoding so XML entities in `<f>` text are preserved as real Excel formula operators.

Evidence:
- `tmp/excel/samples/Dynamic Dashboard Illustration V1.1.xlsm`
- sheet `CH1`, cell `B3` raw formula XML is `VLOOKUP(C21,KPI_Name,2,FALSE)&amp;" for "&amp;C19&amp;" - "&amp;TEXT(C20,"mmm-yy")`
- expected formula text is `VLOOKUP(C21,KPI_Name,2,FALSE)&" for "&C19&" - "&TEXT(C20,"mmm-yy")`
- current `excel.read_sheet_preview` with `cell_content="values_and_formulas"` returns missing `&` operators
- sheet `CH1`, cell `C24` raw formula XML contains `&lt;&gt;`; current preview drops the `<>` comparison operator

Implementation scope:
- stay inside `ontocode-rs/ext/excel/src/preview.rs`
- unescape worksheet text only where XML text requires it, keeping existing preview bounds
- add one focused test using a tiny worksheet XML fixture with `&amp;` and `&lt;&gt;`

Do not add:
- formula AST parsing
- formula inventory sidecars
- SQL conversion
- formula rewriting

New proof status:
- `EXCEL-MCP-U2` proof is now satisfied by samples such as `Attendance-Sheet.xlsx`, `Inventory-Tracking-Sheet1.xlsx`, and `Customer-Invoice-and-Payment-Tracker.xlsx`: they contain list data validation, but current preview/inspect output does not expose validation metadata. This is still not implementation-ready until the bounded output shape is approved.
- `EXCEL-MCP-U3` proof is now satisfied by `Dynamic Dashboard Illustration V1.1.xlsm`: it has formulas, cached values, calculation metadata, defined names, and external-link risk. Current preview exposes only formula/value. This is still not implementation-ready until the read-only inventory shape is approved.

Additional blocked evidence:
- many `.xls` files are legacy OLE workbooks; current `excel.inspect_workbook` explicitly accepts only `.xlsx`, `.xlsm`, or `.xlsb`. Legacy `.xls` support remains out of scope without a separate ADR/dependency decision.
- `Выдача спецодежды_без табельных.xlsm` currently fails inspection because one worksheet exceeds the per-entry XML read budget. This is a bounded-large-workbook behavior decision, not an immediate implementation task.

## Manager Loop Closure - 2026-06-26

Closed implementation tasks:
- `EXCEL-MCP-U1`: implemented in `ontocode-rs/ext/excel/src/backend.rs` by reading optional XML entries as bounded bytes and decoding UTF-8 plus UTF-16 BOM XML. Added `inspect_workbook_handles_utf16_power_query_custom_xml`.
- `EXCEL-MCP-U4`: implemented in `ontocode-rs/ext/excel/src/preview.rs` by preserving `quick_xml` `GeneralRef` entity references while capturing worksheet formula/value/inline text. Added `read_sheet_preview_preserves_formula_xml_entities`.

Verification:
- OntoIndex impact checks for `read_optional_named_entry` and `parse_sheet_preview` both returned `LOW` risk with no affected execution processes.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed: 54 tests.
- `CARGO_BUILD_JOBS=8 just fmt` passed after the final code changes.

Still blocked:
- `EXCEL-MCP-U2`: proof is satisfied, but validation metadata output shape is not approved.
- `EXCEL-MCP-U3`: proof is satisfied, but formula inventory output shape is not approved.
- Rows `038-044`: formula AST, SQL conversion, formula rewrite to named ranges, live Excel, and workbook graph extraction still require separate ADR/output contracts.
- Legacy `.xls` and large-worksheet budget behavior remain separate decisions.

## Senior Unblock Decision - 2026-06-26 Shape Gates

Senior verdict: two more read-only tasks are now implementation-ready because the proof gate is satisfied and the output shape is bounded. This does not reopen formula-to-SQL, formula rewriting, live Excel, legacy `.xls`, or workbook graph extraction.

### EXCEL-MCP-U2 - implementation-ready

Add selected-sheet data-validation visibility to `excel.read_sheet_preview`.

Approved owner:
- `ontocode-rs/ext/excel/src/preview.rs`
- `ontocode-rs/ext/excel/src/tool.rs`
- focused tests in `ontocode-rs/ext/excel/src/tests.rs`

Approved output shape:
- add `data_validations: Vec<SheetDataValidationSummary>` to `ReadSheetPreviewResult`
- each summary contains `ranges_sample: Vec<String>`, `range_count: usize`, `validation_type: String`, `operator: Option<String>`, `allow_blank: Option<bool>`, `show_error_message: Option<bool>`, `formula1: Option<String>`, `formula2: Option<String>`, `resolved_values_sample: Vec<String>`, and `resolved_values_truncated: bool`
- cap summaries to 64 validations, range samples to 16 ranges, formulas to 256 chars, and resolved values to 128 strings
- resolve inline CSV list validations and simple same-sheet absolute ranges only; unresolved formulas remain as formula text plus a warning

Stop conditions:
- do not add validation writes
- do not evaluate arbitrary formulas
- do not resolve cross-workbook, external, volatile, or indirect list sources
- do not add a workbook-wide validation API until selected-sheet preview proves useful

Minimum test proof:
- tiny workbook with inline list validation
- tiny workbook with same-sheet range-backed list validation
- assertion that unsupported validation formulas preserve formula text and emit a warning instead of failing

### EXCEL-MCP-U3 - implementation-ready

Add a new bounded read-only `excel.inspect_sheet_formulas` tool under the existing Excel extension owner.

Approved owner:
- `ontocode-rs/ext/excel/src/tool.rs`
- new small implementation module under `ontocode-rs/ext/excel/src/`
- focused tests in `ontocode-rs/ext/excel/src/tests.rs`

Approved input shape:
- workbook `path`
- existing `SheetSelector`
- `max_formulas`, default 128 and hard cap 512

Approved output shape:
- `path`, `sheet`, `max_formulas_applied`, `formulas`, `truncated`, and `warnings`
- each formula summary contains `reference`, `formula`, `cached_value`, `formula_type: Option<String>`, `shared_index: Option<u32>`, `shared_range: Option<String>`, `style_index: Option<u32>`, `number_format_id: Option<u32>`, and `number_format_code: Option<String>`
- workbook context contains `calculation_mode: Option<String>`, `full_calc_on_load: Option<bool>`, `force_full_calc: Option<bool>`, `defined_names_sample: Vec<String>`, and `has_external_links: bool`
- cap formula text to 512 chars and defined-name samples to 64 names

Stop conditions:
- do not synthesize R1C1 formulas; OpenXML worksheet parts store A1 formulas, and invented R1C1 output would be false precision
- do not parse formulas into an AST
- do not generate SQL
- do not rewrite formulas to named ranges
- do not claim dependency graph support; external-link and defined-name context are metadata only
- do not support `.xls` in this slice

Minimum test proof:
- tiny workbook with scalar formulas, cached values, shared formula attributes, workbook calculation properties, defined names, and an external-link relationship marker
- tiny workbook with styles mapping a cell style to a custom number format
- assertion that formula collection truncates at the requested cap and emits a warning

## Manager Loop Closure - 2026-06-26 U2

Closed implementation task:
- `EXCEL-MCP-U2`: implemented selected-sheet data-validation visibility in `excel.read_sheet_preview`. The output stays read-only and bounded, includes explicit `resolved_values_source`, exposes dropdown/error-style flags, resolves only inline CSV lists and simple same-sheet ranges, and keeps unsupported formulas as formula text plus warnings.

Verification:
- OntoIndex impact checks for `read_sheet_preview_with_display_path`, `parse_sheet_preview`, and `ReadSheetPreviewResult` returned `LOW` risk with no affected execution processes.
- Senior-reviewer initially blocked the shape until total output budget and explicit source status were added; those block conditions were addressed before verification.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed: 55 tests.
- `CARGO_BUILD_JOBS=8 just fmt` passed after the code changes.

Still open:
- `EXCEL-MCP-U3`: bounded read-only `excel.inspect_sheet_formulas`.

Still blocked:
- Formula AST parsing, SQL generation, formula rewrites, workbook graph extraction, live Excel automation, legacy `.xls`, and large-worksheet budget behavior remain separate ADR or contract decisions.

## Manager Loop Closure - 2026-06-26 U3

Closed implementation task:
- `EXCEL-MCP-U3`: implemented bounded read-only `excel.inspect_sheet_formulas` under the existing Excel extension owner. The tool accepts workbook `path`, `SheetSelector`, and optional `max_formulas`; it returns selected-sheet formula inventory, cached values, shared-formula metadata, style number-format metadata, workbook calculation flags, defined-name samples, external-link marker, truncation state, and warnings.

Verification:
- OntoIndex impact checks for `extension.rs:install`, `SheetSelector`, and `ExcelThreadState` returned `LOW` risk with no affected execution processes.
- The implementation stays read-only: it opens workbook package parts and parses XML only; it does not evaluate formulas, generate SQL, rewrite formulas, extract workbook dependency graphs, automate live Excel, or support legacy `.xls`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed: 57 tests.
- `CARGO_BUILD_JOBS=8 just fmt` passed after the code changes.

Now closed in this manager-loop scope:
- `EXCEL-MCP-U1`: UTF-16 Power Query custom XML inspection hardening.
- `EXCEL-MCP-U2`: selected-sheet data-validation visibility.
- `EXCEL-MCP-U3`: selected-sheet formula inventory.
- `EXCEL-MCP-U4`: formula XML entity decoding in preview.

Still blocked:
- Formula AST parsing, SQL generation, formula rewrites to named ranges, workbook graph extraction, live Excel automation, legacy `.xls`, and large-worksheet budget behavior remain separate ADR or contract decisions.

## Senior Unblock Decision - 2026-06-26 Next Queue

User selected options `1-6`. Senior verdict: open the six as a bounded next queue, but do not treat them all as implementation tasks. The lazy path is to keep current `ext/excel` read-only/offline and only add metadata that directly improves already-landed tools.

### EXCEL-MCP-N1 - clean verification gate

Status: process-only, no dispatch.

Action:
- isolate, commit, or explicitly exclude unrelated dirty worktree changes before any global verification claim
- keep scoped `gn_verify_diff` acceptable only as interim evidence while the repo is broadly dirty

Stop condition:
- do not stash, revert, or rewrite unrelated user changes without explicit instruction

Manager closeout:
- scoped Excel verification is already recorded in this review artifact
- repository-wide clean verification remains a manager discipline item, not an implementation task
- no worker dispatch is justified unless a concrete missing verification gap is proven

### EXCEL-MCP-N2 - formula risk warnings

Status: closed in manager loop on 2026-06-26.

Approved owner:
- `ontocode-rs/ext/excel/src/formula_inspect.rs`
- `ontocode-rs/ext/excel/src/tool.rs`
- focused tests in `ontocode-rs/ext/excel/src/tests.rs`

Approved shape:
- add bounded per-formula `warnings: Vec<String>` or equivalent risk markers to the existing `SheetFormulaSummary`
- detect only lexical high-risk markers from formula text: `INDIRECT`, `OFFSET`, volatile calculation functions, external workbook references, and dynamic-array/spill markers

Stop conditions:
- do not parse formula AST
- do not evaluate formulas
- do not generate SQL
- do not rewrite formulas

Closure evidence:
- implemented as bounded lexical risk markers on existing formula inventory output
- verified warning coverage for `INDIRECT`, `OFFSET`, volatile functions, external workbook/URL references, and dynamic-array/spill markers
- scoped test command passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`

### EXCEL-MCP-N3 - defined-name inspection

Status: closed in manager loop on 2026-06-26.

Approved owner:
- existing formula inventory path or a small sibling read-only module under `ontocode-rs/ext/excel/src/`
- `ontocode-rs/ext/excel/src/tool.rs`
- focused tests in `ontocode-rs/ext/excel/src/tests.rs`

Approved shape:
- expose bounded defined-name metadata: name, optional sheet scope, hidden flag when present, target text, truncation
- keep existing `defined_names_sample` compatibility until a later cleanup explicitly removes it

Stop conditions:
- do not rewrite formulas to named ranges
- do not resolve names through calculation
- do not claim dependency graph support

Closure evidence:
- implemented as bounded structured defined-name metadata on `excel.inspect_sheet_formulas`
- preserved `defined_names_sample` compatibility alongside the structured output
- scoped test command passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`

### EXCEL-MCP-N4 - worksheet dimension metadata

Status: closed in manager loop on 2026-06-26.

Approved owner:
- `ontocode-rs/ext/excel/src/preview.rs`
- `ontocode-rs/ext/excel/src/tool.rs`
- focused tests in `ontocode-rs/ext/excel/src/tests.rs`

Approved shape:
- add selected-sheet dimension metadata from the worksheet `<dimension ref="...">` marker when present
- optionally expose a conservative `preview_exceeds_dimension` style flag only when it can be derived without scanning the whole sheet

Stop conditions:
- do not implement large-worksheet paging
- do not infer dimensions by full-sheet scan
- do not add range write APIs

Closure evidence:
- implemented selected-sheet `<dimension ref="...">` metadata in `excel.read_sheet_preview`
- kept the read path metadata-only with no full-sheet scan, paging, or write surface
- scoped test command passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`

### EXCEL-MCP-N5 - workbook graph proof pack

Status: design-contract-closed; code blocked.

Action:
- use `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md` as the only approved graph contract artifact
- keep code, fixtures, expected-edge data, and tool registration blocked until explicit user acceptance plus fresh senior review

Stop conditions:
- do not add graph output with empty or placeholder edges
- do not call formula presence a dependency graph
- do not build a calculation engine inside `ext/excel`

Closed design contract:
- `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md` defines the only approved node/edge/output-contract prose.
- It defines required source evidence, warning/blocker rules, and proof gates.
- It explicitly does not approve extraction, Rust graph types, fixtures, parser skeletons, JSON/TOML edge data, or tools.

Senior challenge:
- This is not implementation-ready. A workbook graph without real edge extraction is worse than no graph because downstream agents will treat it as proof.
- Do not open code until the graph architecture surface is explicitly accepted and a fresh senior review approves concrete Rust-owned output types and parser-backed fixture tests.
- Keep graph output read-only metadata. Do not add formula evaluation, SQL conversion, workbook mutation, or dependency recalculation.

Senior-reviewer result:
- `N5-A` fixture-first proof is blocked. A manually asserted expected-edge fixture would create a workbook graph concept without an approved Rust type, extractor, or output contract.
- Fixture-only graph tests are graph theater: they prove the zip entries exist, not that any edge extraction is real.
- No code, fixture, expected-edge JSON in Rust tests, graph type, parser skeleton, public tool, or private extractor is approved in this loop.

Allowed unblock options:
- Completed: `N5-ADR` is captured in `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md` as a design-only graph schema/output contract. No Rust.
- Alternative: keep `N5` closed until a future user explicitly asks for workbook graph extraction and accepts the new architecture surface.
- Later only after ADR approval: add a fixture test that imports real approved Rust graph types and asserts parser output. Do not create fixture-only expected-edge JSON before that type exists.
- Reject for now: mine real samples directly as the first graph proof. Real samples are too noisy; they should be regression/follow-up evidence after a typed contract and extractor exist.

### EXCEL-MCP-N6 - live Excel companion ADR

Status: ADR-only, no implementation dispatch.

Action:
- draft a separate live Excel companion ADR only if live charts/VBA/mutation remain desired
- keep it outside current offline `ext/excel` ownership unless a future architecture decision says otherwise

Stop conditions:
- do not add COM/live Excel dependencies to `ext/excel`
- do not mix live workbook state with path-based offline tool contracts
- do not add chart/write/mutation tools without screenshot or artifact verification gates

## Manager Loop Closure - 2026-06-26 N2-N4

Closed in this bounded loop:
- `EXCEL-MCP-N2`: formula risk warnings landed as lexical markers only.
- `EXCEL-MCP-N3`: defined-name inspection landed as bounded metadata only.
- `EXCEL-MCP-N4`: worksheet dimension metadata landed from `<dimension ref="...">` only.

Still gated:
- `EXCEL-MCP-N1`: process-only; no worker dispatch is justified unless a concrete missing verification gap is proven.
- `EXCEL-MCP-N5`: blocked by senior review; only text design/ADR is allowed before any code or fixture.
- `EXCEL-MCP-N6`: ADR-only; no live Excel dependencies belong in current offline `ext/excel`.

Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed before closure.
- `CARGO_BUILD_JOBS=8 just fmt` passed after code edits.
- Repository-wide verification remains gated by the broad dirty worktree; only scoped Excel evidence is claimed here.

## Current Unblock Decision - 2026-06-26

Issues fixed:
- Removed stale fixture-first guidance after senior review blocked `N5-A`.
- Closed `N5-ADR` as the design-only contract artifact.
- Kept `N5-CODE` blocked instead of treating ADR approval as implementation approval.

Several valid ways to unblock next:
- `N1-A` manager-only cleanup: isolate or explicitly exclude unrelated dirty worktree changes before claiming a future global verification pass. Do not stash or revert user work without instruction.
- `N5-ACCEPT`: explicitly accept the workbook graph architecture surface from `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md`, then run a fresh senior-review pass on concrete Rust output types and parser-backed fixture tests.
- `N5-HOLD`: leave workbook graph extraction closed; this is the cheapest valid option until there is real demand.
- `N6-A`: draft a live Excel companion ADR only if live charts/VBA/mutation is still desired; keep it outside offline `ext/excel`.

## Manager Loop Decision - 2026-06-26 N5-A Blocked

Senior-reviewer verdict:
- `N5-A` is blocked because it would introduce a workbook graph concept through a fixture without an approved graph schema, Rust type, parser, or output contract.
- Implementation-worker stopped and made no changes.
- Manager removed the local test experiment and did not dispatch any Rust implementation.

Closed/no-dispatch:
- No fixture-first test.
- No expected-edge JSON in `tests.rs`.
- No graph extractor, public tool, private parser, formula evaluation, SQL generation, formula rewrite, workbook mutation, or live Excel work.

## Manager Loop Decision - 2026-06-26 N5-ADR Closure

Senior-reviewer verdict:
- `N5-ADR` passed as text-only contract work with hard stop conditions.
- The ADR must not imply graph extraction exists, must not add Rust types, and must not add fixtures.

Closed in this loop:
- Added `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md` with graph node/edge prose, source-part mapping, warning/blocker rules, output-contract prose, proof gates, and stop conditions.

Still blocked after ADR:
- `N5-CODE` is not automatically opened. A future implementation requires explicit user acceptance of the graph architecture surface and a fresh senior-review pass.

## Manager Loop Decision - 2026-06-26 No Dispatch After Graph ADR

Senior-reviewer verdict:
- Pass on tracker hygiene.
- Block on all implementation dispatch paths.

No-dispatch decisions:
- `N1-A` is process-only; it is not a worker implementation task.
- `N5-ACCEPT` is blocked because a repeated manager-loop request is not explicit acceptance of the workbook graph architecture surface.
- `N5-HOLD` remains the default workbook graph status.
- `N6-A` is blocked because live Excel, COM, chart, VBA mutation, or external companion demand has not been explicitly stated.

Stop conditions:
- Do not dispatch graph code until the user explicitly accepts `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md` and names a concrete Rust output type or parser slice for fresh senior review.
- Do not draft live Excel companion ADR work until live Excel demand is explicit.
- Do not dispatch generic verification cleanup unless the tracker records a specific missing test or verification gap.

## Distinct Useful Solution Approaches

| # | Approach | Verdict | Donor Evidence | Current Owner / Home | Useful Action |
|---:|---|---|---|---|---|
| 001 | Keep offline workbook analysis separate from live Excel automation. | KEEP-CANDIDATE | `Excel-mcp-server/README.md`; `mcp-server-excel/README.md` | `ext/excel` vs external MCP companion | preserve current offline tools; add live mode only as separate integration |
| 002 | Model runtime explicitly as `workbook` vs `path` instead of one mixed tool contract. | KEEP-CANDIDATE | `Excel-mcp-server/README.md` | external companion config / future ADR | prefer explicit mode selection over hidden auto-behavior |
| 003 | Preserve embedded charts/images/shapes in closed-file edit mode. | KEEP-CANDIDATE | `Excel-mcp-server/README.md:10-10` | external companion or future export/edit slice | keep any file-based mutations non-destructive to embedded content |
| 004 | Use CLI and MCP as separate front doors over one capability core. | KEEP-CANDIDATE | `mcp-server-excel/README.md:111-118` | future external tooling, not current `ext/excel` | if companion exists, keep parity at one owner layer |
| 005 | Generate tool/command parity from core service definitions instead of duplicating schemas manually. | KEEP-CANDIDATE | `mcp-server-excel/specs/COMPILE-TIME-CONSISTENCY-SPEC.md` | external companion only | avoid parallel drift between CLI and MCP surfaces |
| 006 | Keep current `ext/excel` read/translate owner narrow; route broad mutable runtime elsewhere. | KEEP-CANDIDATE | donor set overall | `ontocode-rs/ext/excel` | reject pressure to turn `ext/excel` into a live Excel operator |
| 007 | Validate formulas before writing, not after failure. | KEEP-CANDIDATE | `excel-mcp-server/src/excel_mcp/validation.py` | future bounded formula inspection helper | add preflight checks before any formula-emitting write slice |
| 008 | Deny clearly unsafe formula functions early. | KEEP-CANDIDATE | `excel-mcp-server/src/excel_mcp/validation.py:186-194` | future write helper / analyzer warnings | treat network/indirect formula families as unsafe by default |
| 009 | Validate requested ranges against actual sheet dimensions and report overrun clearly. | KEEP-CANDIDATE | `excel-mcp-server/src/excel_mcp/validation.py:97-157` | `preview.rs`, future read/export hardening | expose `extends_beyond_data`-style metadata instead of failing opaquely |
| 010 | Return workbook/range metadata with read operations, not raw cells only. | KEEP-CANDIDATE | `excel-mcp-server/src/excel_mcp/data.py`; OntoIndex `read_excel_range_with_metadata` | `preview.rs`, `tool.rs` | expand bounded metadata before expanding mutability |
| 011 | Extract data validation metadata, including dropdown values and formulas. | KEEP-CANDIDATE | `excel-mcp-server/src/excel_mcp/cell_validation.py:9-179` | current `preview.rs` / future workbook inspection slice | add validation visibility before adding write workflows |
| 012 | Resolve list-validation ranges to actual allowed values when possible, degrade gracefully when not. | KEEP-CANDIDATE | `excel-mcp-server/src/excel_mcp/cell_validation.py:95-153` | future preview/inspect enhancement | prefer partial useful output over opaque “has validation” markers |
| 013 | Treat VBA-capable workbook extensions as an explicit trust boundary. | KEEP-CANDIDATE | `vba-mcp-server/server.py:75-82` | `vba_extract.rs`, `review_vba_onlyoffice_workbook.rs` | keep file-type gates strict and explicit |
| 014 | Reuse already-open workbooks rather than reopening blindly. | KEEP-CANDIDATE | `vba-mcp-server/server.py:64-72` | external live companion | avoid duplicate Excel instances and workbook conflicts |
| 015 | Auto-save only around explicit write operations. | KEEP-CANDIDATE | `vba-mcp-server/server.py:75-97`; `173-202` | external live companion | keep read paths side-effect free |
| 016 | Protect document modules from deletion; allow code clearing separately. | KEEP-CANDIDATE | `vba-mcp-server/server.py:230-247` | external live companion / future live VBA ADR | fail safe on `Sheet*` and `ThisWorkbook` owners |
| 017 | Export VBA modules to backup files plus a manifest for provenance. | KEEP-CANDIDATE | `vba-mcp-server/server.py:250-286` | external companion / future support tooling | prefer artifact export over direct destructive rewrites |
| 018 | Add workbook discovery as a separate read-only utility, not implicit repo scanning in write tools. | KEEP-CANDIDATE | `vba-mcp-server/server.py:289-309` | external support tooling | keep discovery bounded and explicit |
| 019 | Enforce screenshot-based visual verification after chart creation. | KEEP-CANDIDATE | `mcp-server-excel/skills/excel-mcp/references/screenshot.md:3-17` | external companion workflow; possible test discipline | treat visual verification as required, not optional fluff |
| 020 | Use explicit target ranges for charts to avoid overlap with data. | KEEP-CANDIDATE | `mcp-server-excel/skills/excel-mcp/references/screenshot.md:41-45`; `72-95` | external chart workflow / future artifact tests | never rely on default chart placement in automated flows |
| 021 | Verify complex formatting and Pivot layout visually, not only structurally. | KEEP-CANDIDATE | `mcp-server-excel/skills/excel-mcp/references/screenshot.md:47-59` | external companion or test artifacts | add screenshot checks where layout is the product |
| 022 | Separate Power Query, relationships, and DAX into ordered workflow stages. | KEEP-CANDIDATE | `mcp-server-excel/skills/excel-mcp/references/workflows.md:18-35` | documentation / external companion | encode sequencing rules instead of pretending one monolith solves it |
| 023 | Evaluate Power Query before persisting it into the workbook. | KEEP-CANDIDATE | `mcp-server-excel/skills/excel-mcp/references/workflows.md:38-45` | future Power Query companion; not current extractor | test M code first, then store, then refresh/load |
| 024 | Put computed columns in Power Query, not DAX, when Excel COM limits demand it. | KEEP-CANDIDATE | `mcp-server-excel/skills/excel-mcp/references/workflows.md:5-17` | documentation / external companion | keep architectural roles separated |
| 025 | Use named ranges / setup sheets as explicit workbook parameters for queries. | KEEP-CANDIDATE | `mcp-server-excel/skills/excel-mcp/references/workflows.md:46-52` | possible future workbook parameter inspection | treat workbook parameters as first-class artifacts |
| 026 | Verify Power Query / Data Model / chart steps with dedicated post-step checks. | KEEP-CANDIDATE | `mcp-server-excel/skills/excel-mcp/references/workflows.md:54-61` | external workflow discipline | sequence create -> verify -> continue |
| 027 | Filter tool exposure to reduce schema/token overhead when only a subset is needed. | KEEP-CANDIDATE | `mcp-google-sheets/src/mcp_google_sheets/server.py:48-75`; README tool-filtering section | external companion / general MCP ergonomics | expose opt-in subsets instead of defaulting to all tools |
| 028 | Let CLI args override env tool filters. | KEEP-CANDIDATE | OntoIndex context on `_parse_enabled_tools` tests | external companion | keep overrides explicit and local to invocation |
| 029 | Default to values-only reads and make richer grid metadata opt-in. | KEEP-CANDIDATE | `mcp-google-sheets/README.md:196-201` | future cloud companion / current bounded read philosophy | keep cheap/default reads small |
| 030 | Prefer batch update/read tools for multi-range operations. | KEEP-CANDIDATE | OntoIndex `batch_update`; `spreadsheet-mcp` and `mcp-google-sheets` README | external cloud companion | avoid chatty one-cell-at-a-time orchestration |
| 031 | Keep chart create/list/delete as separate tools, not one mega-chart method. | KEEP-CANDIDATE | `spreadsheet-mcp/src/spreadsheet_mcp/sheets_client.py:897-945` | external companion | smaller verbs are easier to verify and retry |
| 032 | Keep sharing/publication as explicit tools, separate from data operations. | KEEP-CANDIDATE | `spreadsheet-mcp/src/spreadsheet_mcp/sheets_client.py:1074-1115` | external cloud companion | isolate security-sensitive side effects |
| 033 | Use sort/find-replace/get-last-row as explicit data ops rather than folding them into generic sheet writes. | KEEP-CANDIDATE | `spreadsheet-mcp/src/spreadsheet_mcp/sheets_client.py:951-1068` | external companion | keep common spreadsheet mechanics composable |
| 034 | Abstract Excel backends behind one interface and prefer native/live backend first, file backend second. | KEEP-CANDIDATE | `negokaz-excel-mcp-server/internal/excel/excel.go:7-112` | external live/file companion | dual backend is valid if the abstraction is real and already needed |
| 035 | Make paging a strategy, not one hardcoded range splitter. | KEEP-CANDIDATE | `negokaz-excel-mcp-server/internal/excel/pagination.go:8-220` | future read companion / current preview philosophy | support fixed-size and print-area-aware paging |
| 036 | Render formulas/values/styles as structured human-readable tables for inspection. | KEEP-CANDIDATE | OntoIndex `CreateHTMLTableOfFormula`; `excel_screen_capture` / `excel_read_sheet` owners | future read companion / diagnostics | present spreadsheet state clearly before adding mutability |
| 037 | Extract formula text, R1C1 form, cached values, number format, calculation mode, and external-link flags as bounded sidecar metadata before any conversion. | KEEP-CANDIDATE | `in2sql_dotNet_addin/.../Formula/WorksheetFormulaMetadataDocument.cs`; `.../Xlsb/FormulaSidecarModels.cs`; current `preview.rs` formula capture | current `excel.inspect_sheet_formulas` plus possible later `xlsb`/R1C1 follow-up | bounded read-only formula inventory is now landed; remaining gaps are explicit non-go items until a separate ADR reopens them |
| 038 | Parse formulas into a typed AST before translation; regex evidence may classify but must not generate SQL. | KEEP-CANDIDATE | `in2sql_dotNet_addin/.../Formula/Ast/ExcelFormulaAst.cs`; `ExcelFormulaParser.cs` | future formula analyzer, not current `powerquery_translate.rs` | require AST-backed conversion if a future formula-to-SQL ADR exists |
| 039 | Treat array constants, dynamic arrays/spill, external workbook links, and volatile/indirect functions as blockers. | KEEP-CANDIDATE | `ExcelFormulaParser.cs`; `Tests/SqlEngine.Tests/Program.cs` dynamic-array and array-constant cases | future formula analyzer warnings | do not dispatch "convert to array formula" or dynamic-array SQL lifting as automatic work |
| 040 | Generate SQL only from typed relational intent plans, then validate against cached Excel values or block. | KEEP-CANDIDATE | `FormulaSqlClausePlanner.cs`; `FormulaSqlEmitter.cs`; `FormulaGeneratedNodeValidator.cs` | future external companion or separate ADR | no direct Excel-formula-string-to-SQL transpiler in current `ext/excel` |
| 041 | Preserve named ranges as workbook parameters/defined-name metadata; do not rewrite formulas to named ranges automatically. | KEEP-CANDIDATE | `mcp-server-excel/.../NamedRangeCommands.Operations.cs`; `FEATURES.md` named-range parameter automation | current defined-name inspection; bounded dry-run rewrite discipline only | defined-name inspection is landed; prototype dry-run rewrite is now landed and locally verified, while any promotion beyond the prototype still needs explicit user-authored mapping and real workbook evidence |
| 042 | Use modern `Formula2` semantics for live formula writes; do not default to legacy `Formula` or array-formula APIs. | KEEP-CANDIDATE | `mcp-server-excel/.../RangeCommands.Formulas.cs`; Formula2 regression tests; current ONLYOFFICE preview emits `SetFormulaArray` only inside the gated VBA translator | external live companion / future ONLYOFFICE translator challenge | preserve dynamic-array semantics; never "upgrade" scalar formulas to array formulas blindly |
| 043 | Model workbook migration as a graph of source workbook, sheets/tables, Power Query, formula regions, generated SQL, validation, and import actions. | KEEP-CANDIDATE | `MigrationGraphBuilder.cs`; `FormulaDependencyDetector.cs`; `PowerQueryLineageDetector.cs` | future graph export / external companion | start with bounded nodes and edges; no calculation engine in `ext/excel` |
| 044 | Reject placeholder graph claims until edge extraction is real. | KEEP-CANDIDATE | `NormalizedExportWriter.BuildDependencyGraph` currently returns nodes and empty edges with a placeholder comment | review gate for formula/model proposals | require tests proving precedents/dependents or query dependencies before calling a workbook graph complete |

## Proposed Use

### Already covered by current Ontocode `ext/excel`

- bounded workbook inspection
- bounded sheet preview
- sheet dimension metadata in preview
- bounded selected-sheet data-validation visibility in preview
- bounded sheet export
- bounded selected-sheet formula inventory with cached values, number formats, workbook calculation flags, defined names, and external-link markers
- lexical formula risk warnings on inspected formulas
- VBA module extraction
- Power Query extraction
- source-first VBA translation previews
- source-first Power Query translation previews
- heuristic Power Query to SQL preview translation from pasted M source
- fail-closed VBA to ONLYOFFICE analysis and preview emission

### Best extensions to consider later

1. Tighten validation visibility only if a real workbook proves unsupported validation classes matter.
2. Add stronger artifact-level verification discipline for chart/layout workflows.
3. If live Excel is ever approved, integrate it as a separate external MCP companion.
4. If cloud Sheets is ever approved, keep it as a separate provider path with tool filtering.
5. Extend formula coverage only where the current bounded inventory is still missing concrete metadata such as `xlsb` parity or proven R1C1 demand.
6. Treat worksheet-formula-to-SQL as a separate, fail-closed ADR: AST parse -> relational intent plan -> SQL emission -> execution/cached-value validation. Anything less is too risky. This does not reopen the existing heuristic Power Query to SQL preview tool.
7. Treat array formulas and dynamic arrays as unsupported/deferred until spill range semantics, row/column shape, and validation artifacts are proven.

### Formula And Calculation Graph Challenge Addendum

The current donor review was too weak on formula modeling. Rows 007-012 cover formula validation and data-validation formulas, and row 025 covers named ranges as workflow parameters, but neither is enough for formula conversion, formula-to-SQL, array formulas, or workbook calculation graph parsing.

The source-backed answer is narrow:

- Formula conversion should start as read-only formula inventory: cell address, formula text, cached value, formula dialect where available, workbook calculation metadata, and external-link risk.
- Worksheet-formula-to-SQL is not a general-purpose conversion feature. The only defensible shape is the `in2sql_dotNet_addin` pattern: typed AST, unsupported-node blockers, relational intent planning, structured SQL emission, and validation against cached values or an execution engine. This is separate from the already-landed source-first Power Query to SQL preview tool.
- Array formulas should not be introduced as an optimization or rewrite target. The donor code treats array constants and dynamic/spill functions as unsupported/deferred, which is the right default.
- Named-range conversion is not automatic refactoring. Named ranges can be inspected and used as explicit workbook parameters, but replacing formula references with named ranges requires user mapping, scope checks, collision checks, and workbook-level tests.
- Workbook graph modeling is useful only as metadata: sources, sheets, tables, formulas, Power Query, DAX/Data Model, generated SQL, validation, and import actions. It is not a mandate to build a calculation engine inside `ext/excel`.
- Negative evidence matters: `NormalizedExportWriter.BuildDependencyGraph` currently emits query nodes with empty edges and states it has no actual dependency analysis. That file should be treated as a warning against graph theater, not proof of a completed dependency graph.

### Explicit non-go items

- do not port `mcp-server-excel` wholesale into `ext/excel`
- do not merge Google Sheets into the current offline workbook owner
- do not replace source-first VBA analysis with live in-place mutation
- do not add mutable workbook APIs to current `ext/excel` without a new ADR and artifacts proving the need
- do not claim formula-to-SQL support without AST-backed SQL generation and validation evidence
- do not convert scalar formulas to array formulas or dynamic arrays without explicit workbook-shape proof
- do not market a workbook "calculation graph" when only formula presence or query names were extracted

## Manager Unblock Matrix - 2026-06-26

These rows are now unblocked only in the narrow manager sense that each has an explicit valid reopen path. None of them is automatically implementation-ready.

### Row 038 - worksheet formula AST parsing

Current state:
- design-contract-closed for the first reopen step; implementation still blocked

Smallest valid next step:
- completed: `ADR_EXCEL_WORKSHEET_FORMULA_AST_CONTRACT.md`

Required reopen contract:
- owner stays inside current offline `ext/excel`
- A1 formulas only in the first slice
- read-only AST, no SQL, no evaluation, no rewrite, no graph claims
- unsupported nodes must block cleanly and be preserved as explicit kinds

Implementation opens only after:
- the AST contract names concrete Rust types, hard size caps, and parser-backed fixtures

Closed design contract:
- `ADR_EXCEL_WORKSHEET_FORMULA_AST_CONTRACT.md` is now the approved row `038` contract artifact.
- It does not approve a parser, Rust type, fixture, tool, SQL planner, graph extractor, or mutation path.

### Row 039 - array/dynamic-array conversion semantics

Current state:
- design-proof-closed for the reopen step; implementation still blocked

Smallest valid next step:
- completed: blocker taxonomy proof pack recorded from donor parser and planner evidence

Required reopen contract:
- workbook artifacts showing spill markers, array constants, and shape-sensitive formulas
- explicit statement that the first slice remains fail-closed for conversion

Implementation opens only after:
- row `038` AST exists and the blocker taxonomy is proven on real workbook fixtures

Closed blocker proof:
- donor parser tests treat array constants as explicit unsupported nodes
- donor parser tests treat `FILTER(...)` and similar spill-capable formulas as unsupported/deferred rather than auto-converted
- donor parser tests treat external workbook references as unsupported
- donor planner/docs treat volatile functions such as `INDIRECT`, `OFFSET`, `RAND`, `NOW`, `TODAY`, `CELL`, and `INFO` as blocker categories
- malformed formulas remain blocker results, not silent fallback conversions

### Row 040 - worksheet-formula-to-SQL generation and validation

Current state:
- design contract closed for the first reopen step; implementation is still blocked and remains separate from the existing Power Query SQL preview tool

Smallest valid next step:
- completed: design-only worksheet-formula-to-SQL ADR recorded with supported-tier boundaries, fail-closed blocker taxonomy, and cached-value validation requirements

Required reopen contract:
- row `038` AST contract accepted first
- explicit first-slice subset named up front, with same-row scalar formulas as the only default candidate first slice
- validation path against cached Excel values or equivalent execution evidence

Implementation opens only after:
- a fresh senior-review pass approves the exact first slice, fixture pack, planner output shape, and validation caps on real workbook artifacts

### Row 041 - formula rewrites to named ranges

Current state:
- design contract is now recorded for the read-only dry-run shape; any apply path is still optional and dependent on a separate live/native owner decision
- local blocker workbooks are identified and a synthetic positive workbook proves the exact direct-ref-to-existing-name rewrite pair needed for the bounded prototype path
- prototype-only `041A` dry-run is now implemented in offline `ext/excel` as a read-only tool with exact textual replacement only, existing names only, and no apply path
- local verification passed for the package target, including positive, external-link, ambiguous-sheet-scope, and `R1C1` blocker coverage
- production reopen remains gated on real workbook evidence and explicit user demand

Smallest valid next step:
- hold the current prototype boundary and collect one real workbook plus one explicit user-authored mapping file if production promotion is still desired

Required reopen contract:
- one real workbook that proves direct references should become named ranges
- one explicit user story for why dry-run review is needed
- one user-authored mapping file with explicit formula targets and scope expectations
- no automatic name synthesis
- any apply owner stays outside offline `ext/excel`

Production promotion opens only after:
- the dry-run contract is backed by real workbook artifacts and mapping evidence, and any optional apply path is separately approved as a live/native owner decision

Prototype implementation is allowed now because:

- the synthetic fixture proves one clean direct-ref-to-existing-name mapping
- blocker workbooks are already identified for fail-closed coverage

### Row 042 - live `Formula2` / array-formula write semantics

Current state:
- blocked from current owner; now has a valid external-companion reopen path

Smallest valid next step:
- draft a separate live Excel companion ADR outside offline `ext/excel`

Required reopen contract:
- explicit live Excel / COM / mutation demand
- screenshot or workbook-artifact verification gates
- dynamic-array roundtrip proof using `Formula2` semantics

Implementation opens only after:
- the companion ADR is accepted and the owner boundary stays outside path-based offline tooling

### Rows 043-044 - workbook dependency graph extraction with real edges

Current state:
- design contract exists; code remains blocked but now has a single explicit reopen path
- staged implementation order is now proposed separately in `ADR_EXCEL_WORKBOOK_GRAPH_IMPLEMENTATION_SEQUENCE.md`: package structure plus per-sheet formula membership first, then defined-name evidence edges, then table parser plus table ranges, then a separate Power Query lineage proof gate

Smallest valid next step:
- explicitly accept `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md` and reopen one concrete edge family for fresh senior review

Required reopen contract:
- one edge family only in the first slice, specifically package structure plus per-sheet formula membership
- typed Rust output, parser-backed fixtures, and warning/blocker rules
- no empty-edge placeholders, no calculation engine, no evaluation

Implementation opens only after:
- the accepted edge family, output type, and fixture strategy are named up front

### Legacy `.xls` support

Current state:
- feasibility note closed for the first reopen step; implementation remains blocked

Smallest valid next step:
- completed: legacy-`.xls` feasibility note recorded with inspect-only scope, dependency options, and explicit non-approved paths

Required reopen contract:
- at least one real `.xls` fixture
- chosen parser/dependency or conversion strategy
- explicit statement whether support is inspect-only or broader

Implementation opens only after:
- the dependency and owner decision is accepted

### Large-workbook XML budget policy

Current state:
- policy note closed for the first reopen step; implementation remains blocked pending a real failing workbook artifact

Smallest valid next step:
- completed: large-workbook XML budget policy note recorded, separating required hard-stop reads from optional warning-candidate scans

Required reopen contract:
- preserve hard-stop behavior for required workbook parts
- distinguish optional-entry truncation from corrupted package reads
- keep explicit byte and entry caps

Implementation opens only after:
- a real failing workbook artifact proves an optional scan path, with explicit warning semantics and no downgrade of required reads

## Manager Dispatch Order - 2026-06-26 Continue

Senior-reviewer challenge:
- Do not pretend all reopened rows are equally ready. Most of them are still dependency-bound or demand-bound.
- Open the minimum number of upstream design tasks that can unblock other rows later.

Manager-approved active order:
1. Row `038-AST-ADR`: completed as design-only contract work.
2. Row `039` blocker taxonomy proof pack: completed as design-only proof work.
3. Row `040` worksheet-formula-to-SQL ADR: completed as design-only contract work.
4. Legacy `.xls` feasibility note: completed as design-only feasibility work.
5. Large-workbook XML budget policy note: completed as design-only policy work.

Still intentionally queued behind dependencies:
- Row `039`: completed as blocker taxonomy proof.
- Row `040`: completed as worksheet-formula-to-SQL ADR work, constrained by the row `038` AST contract and the row `039` blocker set.

Still intentionally demand-gated:
- Row `041`: do not open until a real workbook and explicit rewrite demand exist.
- Row `042`: do not open until live Excel / COM / mutation demand is explicit.
- Rows `043-044`: do not open until `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md` is explicitly accepted for one concrete first edge family.

No implementation-worker dispatch:
- No Rust implementation worker is justified yet from these rows.
- There is no active implementation-ready task in this queue after the current design notes.

## Manager Loop Decision - 2026-06-26 Row 038 ADR Closure

Senior-reviewer verdict:
- pass design-only contract work
- keep implementation blocked

Closed in this loop:
- added `ADR_EXCEL_WORKSHEET_FORMULA_AST_CONTRACT.md`

Effect on remaining queue:
- row `039` is now eligible for design-only blocker taxonomy work
- row `040` is now eligible for design-only SQL-planning ADR work, still constrained by row `038`
- no implementation-worker dispatch is opened by this closure

## Manager Loop Decision - 2026-06-26 Row 039 Blocker Proof Closure

Senior-reviewer verdict:
- pass design-only blocker taxonomy proof
- keep implementation blocked

Closed in this loop:
- recorded donor evidence that array constants, spill-capable dynamic-array formulas, external workbook references, volatile functions, and malformed formulas must remain fail-closed categories for any later formula planner

Effect on remaining queue:
- row `040` is now the next active formula-track design task
- no implementation-worker dispatch is opened by this closure

## Manager Loop Decision - 2026-06-27 Row 040 SQL ADR Closure

Senior-reviewer verdict:
- pass design-only contract work
- keep implementation blocked

Closed in this loop:
- added `ADR_EXCEL_WORKSHEET_FORMULA_TO_SQL_CONTRACT.md`

Effect on remaining queue:
- legacy `.xls` feasibility becomes the next active design-only task
- large-workbook XML budget policy remains queued behind legacy `.xls`
- rows `041-044` remain dependency-gated or demand-gated
- no implementation-worker dispatch is opened

## Manager Loop Decision - 2026-06-27 Legacy `.xls` Feasibility Closure

Senior-reviewer verdict:
- pass design-only feasibility work
- keep implementation blocked

Closed in this loop:
- added `EXCEL_LEGACY_XLS_FEASIBILITY_NOTE.md`

Effect on remaining queue:
- large-workbook XML budget policy becomes the next active design-only task
- rows `041-044` remain dependency-gated or demand-gated
- no implementation-worker dispatch is opened

## Manager Loop Decision - 2026-06-27 Large-Workbook XML Budget Policy Closure

Senior-reviewer verdict:
- pass design-only policy work
- keep implementation blocked

Closed in this loop:
- added `EXCEL_LARGE_WORKBOOK_XML_BUDGET_POLICY_NOTE.md`

Effect on remaining queue:
- no active design-only task remains in the current reopen order
- rows `041-044` remain dependency-gated or demand-gated
- no implementation-worker dispatch is opened
