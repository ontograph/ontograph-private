# ADR: Excel Agent Tools As An Optional Extension Family

## Status

Accepted and implemented. Stage 1 through Stage 3 are closed as bounded inspection, preview, and CSV export tools with turn-cwd-aware path resolution.

## Date

2026-06-23

## Context

The donor review in [EXCEL_PARSING_TOOLS_REVIEW.md](/opt/demodb/_workfolder/ontocode/tmp/excel/in2sql_dotNet_addin/docs/EXCEL_PARSING_TOOLS_REVIEW.md:1) found a real Excel parsing stack with three distinct capabilities:

- workbook metadata inspection
- sheet materialization to CSV
- specialized extraction such as VBA and PowerQuery

The current proposal in [EXCEL_AGENT_TOOL_PROPOSALS.md](/opt/demodb/_workfolder/ontocode/tmp/excel/in2sql_dotNet_addin/docs/EXCEL_AGENT_TOOL_PROPOSALS.md:1) is directionally useful, but it leaves too much unresolved:

- it leaves the main ownership choice open between native `core` handlers and an extension family
- it proposes both `inspect_excel_workbook` and separate list-style tools even though inspection can already return sheets and names
- it introduces `spawn_agents_on_excel_sheet` before the lower-level read/export tools have proved their value

Current Ontocode architecture already has the seams needed for this feature:

- one core planner/router path in [spec_plan.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/spec_plan.rs:27)
- one runtime registry in [registry.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/registry.rs:329)
- extension-owned optional tool contribution through [ToolContributor](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/extension-api/src/contributors.rs:145)
- extension executor bridging in [extensions.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/planning/extensions.rs:18)
- existing row-wise batch reuse in [spawn_agents_on_csv.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/tools/handlers/agent_jobs/spawn_agents_on_csv.rs:14)

Existing optional tool families follow the same pattern:

- [web-search extension](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/web-search/src/extension.rs:22)
- [memories extension](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/memories/src/extension.rs:17)
- multi-tool namespace organization in [memories tools](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/memories/src/tools/mod.rs:17)

## Review And Challenge

### What stays

- Excel support should reuse the existing tool router and registry.
- Row-wise agent workflows should continue to reuse `spawn_agents_on_csv`.
- VBA and PowerQuery extraction should stay out of the first slice.

### What changes

1. The implementation path should not stay ambiguous between native and extension-owned.

The current codebase already treats optional, domain-specific tool families as extension-owned. Excel belongs in that category more than in `core`.

2. The v1 tool surface should be smaller than the current proposal.

`inspect_excel_workbook` can already include:

- sheets
- names
- workbook markers

So separate `list_excel_sheets` and `read_excel_defined_names` tools are redundant in the first release.

3. `spawn_agents_on_excel_sheet` should not be a v1 tool.

It couples extraction with job orchestration, hides the intermediate CSV artifact, and makes failures harder to inspect. Keep the existing explicit handoff:

1. inspect workbook
2. preview or export sheet
3. run `spawn_agents_on_csv`

4. Do not copy the donor's C# owner split into Rust.

The donor has a larger parser/import stack with Excel COM fallbacks and worker orchestration. Ontocode does not need that structure to deliver the first useful slice.

## Implementation Decision

## Solution A: Optional Rust-Native Extension

Create a new optional extension crate and implement workbook reading in Rust.

### Implemented Stage 1 Rust shape

- `ontocode-rs/ext/excel/Cargo.toml`
- `ontocode-rs/ext/excel/src/lib.rs`
- `ontocode-rs/ext/excel/src/extension.rs`
- `ontocode-rs/ext/excel/src/backend.rs`
- `ontocode-rs/ext/excel/src/tool.rs`
- `ontocode-rs/ext/excel/src/tests.rs`

### Implemented Stage 1 types

- `ExcelExtension`
- `ExcelInspectionTool`
- `InspectWorkbookArgs`
- `InspectWorkbookResult`

### Implemented Stage 2 Rust shape

- `ontocode-rs/ext/excel/src/preview.rs`
- `ExcelReadSheetPreviewTool`
- bounded worksheet XML and shared-string reads for `.xlsx` and `.xlsm`

### Implemented Stage 2 types

- `SheetSelector`
- `CellContentMode`
- `ReadSheetPreviewArgs`
- `ReadSheetPreviewResult`
- `SheetPreview`
- `SheetPreviewRow`
- `SheetPreviewCell`

### Implemented Stage 3 Rust shape

- `ontocode-rs/ext/excel/src/export.rs`
- `ExcelExportSheetToCsvTool`
- explicit CSV file export for direct handoff into `spawn_agents_on_csv`

### Implemented Stage 3 types

- `ExportSheetToCsvArgs`
- `ExportSheetToCsvResult`

### Future staged types

No additional staged tool types are approved in this ADR.

### Notes

- use the existing extension install/contributor pattern from web-search and memories
- use one `excel` namespace, like the memories tool family pattern
- do not introduce a backend trait on day one unless there are two real implementations

### Pros

- best fit for current architecture
- keeps `core` small
- no second runtime
- easiest to keep outputs bounded

### Cons

- likely needs a new workbook parser dependency
- advanced metadata fidelity may be lower than the donor stack at first

## Metadata Method Review

Solution A should treat `excel.inspect_workbook` as the workbook metadata owner. Do not add separate tools for each metadata category until one category proves it needs a separate bounded read surface.

### Workbook package methods

For `.xlsx` and `.xlsm`, the implemented Excel owner stays with direct ZIP/OpenXML reads, but the current behavior is narrower than the broader donor-inspired method list:

- open workbook as a ZIP package
- read `xl/workbook.xml`
- read `xl/_rels/workbook.xml.rels`
- read content type markers from `[Content_Types].xml`
- use package part names as marker inventory for tables, comments, drawings, embeddings, charts, pivots, and related carriers
- for `read_sheet_preview` and `export_sheet_to_csv`, read the selected worksheet XML and `xl/sharedStrings.xml` when needed

Per-sheet relationship reads such as `xl/worksheets/_rels/sheet1.xml.rels` and style/number-format reads from `xl/styles.xml` remain plausible later extensions, but they are not part of the currently implemented Excel surface.

This mirrors the donor's COM-free pattern in `OpenXmlPackageReader`, `XlsxOpenXmlWorkbookScanner`, and `OpenXmlFastPreflightService`.

For `.xlsb`, only commit to package-level detection and sheet inventory in the first Rust slice unless the selected Rust parser dependency already exposes BIFF12 worksheet/formula metadata safely. The donor's `.xlsb` support is substantial custom BIFF12 parsing; do not recreate that in the first slice.

### Donor evidence checked

- [WorkbookPackageProbe.cs](/opt/demodb/_workfolder/ontocode/tmp/excel/in2sql_dotNet_addin/tools/WorkbookArtifactExtractor/WorkbookArtifactExtractor.OpenXml/WorkbookPackageProbe.cs:14) proves the useful first slice is package probing: detect `.xlsx/.xlsm/.xlsb`, scan `customXml/item*.xml` for `DataMashup`, and read `xl/connections.xml` or `xl/connections.bin` without launching Excel.
- [DataMashupExportService.cs](/opt/demodb/_workfolder/ontocode/tmp/excel/in2sql_dotNet_addin/tools/WorkbookArtifactExtractor/WorkbookArtifactExtractor.OpenXml/DataMashupExportService.cs:100) proves full PowerQuery extraction is possible, but it requires base64 DataMashup decoding, embedded ZIP boundary detection, and part enumeration. That belongs after metadata detection.
- [VbaProjectReader.cs](/opt/demodb/_workfolder/ontocode/tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Import/Managed/Vba/VbaProjectReader.cs:60) proves VBA extraction requires reading `xl/vbaProject.bin`, parsing the compound file, reading `VBA/dir`, and decompressing module streams. First slice should only detect the carrier.
- [WorksheetDataExportService.cs](/opt/demodb/_workfolder/ontocode/tmp/excel/in2sql_dotNet_addin/tools/WorkbookArtifactExtractor/WorkbookArtifactExtractor.OpenXml/WorksheetDataExportService.cs:59) proves sheet CSV export and formula sidecars are separable from workbook probing. Keep formula content behind `read_sheet_preview` caps and keep CSV export as the explicit handoff to `spawn_agents_on_csv`.

### Metadata coverage table

| Category | First slice behavior | Method | Notes |
| --- | --- | --- | --- |
| Sheets | extract | `xl/workbook.xml` sheets plus workbook relationships | include name, index, visibility, relationship id, part path |
| Tables | detect | `xl/tables/*.xml` marker paths | detailed table definitions are later work |
| Defined names | defer | `xl/workbook.xml` `definedNames/definedName` | not parsed in Stage 1 |
| Connections | detect | `xl/connections.xml` or `.bin` marker paths | connection strings are not returned |
| PowerQuery | detect in v1, extract later | `customXml/item*.xml` DataMashup carrier, `Microsoft.Mashup` connection marker | authoritative M extraction is not first slice |
| VBA/macros | detect in v1, extract later | `xl/vbaProject.bin` presence | full module decompression needs compound file plus MS-OVBA support |
| Formulas `.xlsx/.xlsm` | detect | worksheet parts containing `<f` | formula text is not returned in Stage 1 |
| Formulas `.xlsb` | detect/defer | BIFF12 formula records | donor uses token disassembly; do not rebuild in first slice |
| Comments | detect | `xl/comments*.xml`, `xl/threadedComments/*.xml` | content extraction can be a later bounded option |
| Drawings/shapes | detect only | worksheet relationships to `xl/drawings/*.xml`, drawing rels | count drawings, charts, images, shapes; do not parse geometry in v1 |
| Embedded/OLE objects | detect only | `xl/embeddings/*`, `oleObjects`, drawing/worksheet rels | report presence and counts only |
| Charts/chartsheets | detect only | `xl/charts/*.xml`, `xl/chartsheets/*.xml` | report counts and related sheets when obvious |
| PivotTables/PowerPivot | detect | `xl/pivotTables/*`, `xl/pivotCache/*` | donor has managed pivot extractors; keep detailed extraction later |
| Hyperlinks | defer | worksheet `hyperlinks` plus relationship targets | not parsed in Stage 1 |
| Merged cells | defer | worksheet `mergeCells/mergeCell` | not parsed in Stage 1 |
| Data validation | defer | worksheet `dataValidations/dataValidation` | not parsed in Stage 1 |
| Conditional formatting | defer | worksheet `conditionalFormatting` | not parsed in Stage 1 |
| Protection | defer | workbook/sheet protection nodes | not parsed in Stage 1 |

### PowerQuery method

First slice:

- set `has_power_query` when any DataMashup carrier, Microsoft.Mashup connection, or custom XML formula marker is found
- include marker counts and carrier part-path samples only
- do not return full M text by default

Later bounded extraction:

- decode the custom XML DataMashup payload
- locate the embedded ZIP header
- enumerate `.m` parts
- emit query names, truncated M previews, and dependency edges

The donor's `DataMashupExportService` shows this is feasible, but it is larger than the first useful Rust extension slice.

### VBA method

First slice:

- set `has_vba_project` when `xl/vbaProject.bin` exists
- include marker counts and carrier part-path samples only

Later bounded extraction:

- parse the compound file
- read `VBA/dir`
- decompress module streams with MS-OVBA
- return module names and capped source previews

Do not put raw full VBA modules into model context by default.

### Formula method

For `.xlsx` and `.xlsm`:

- `read_sheet_preview` supports a content mode enum:

```rust
enum CellContentMode {
    Values,
    ValuesAndFormulas,
}
```

- when formulas are requested, read worksheet cell `<f>` formula text and cached `<v>` value
- cap rows, columns, formulas, and cell text

For `.xlsb`:

- report formula record presence only unless parser support is already available
- do not implement BIFF12 formula token disassembly in the first slice

### Comments and objects method

First slice currently surfaces comments and workbook objects only as marker inventory:

- counts
- part-path samples

Related worksheet mapping and bounded comment-text samples remain later work.

Do not extract embedded binary object payloads, images, or full drawing XML in v1.

## Decision

Adopt **Solution A** as the recommended implementation path.

### Why this is the recommended way

It best matches current architecture:

- optional tool families already live behind `ToolContributor`
- multi-tool namespaces already exist in extension-owned code
- Excel is an optional data-domain capability, not core orchestration
- the current CSV batch pipeline already solves the downstream job problem

### Target staged tool set

Stage 1 implements exactly one tool:

- `excel.inspect_workbook`

Stage 2/3 add:

- `excel.read_sheet_preview`
- `excel.export_sheet_to_csv`

### Why this is smaller and better

- `inspect_workbook` subsumes separate sheet/name listing tools
- `read_sheet_preview` gives bounded exploration before export
- `export_sheet_to_csv` gives a stable handoff into `spawn_agents_on_csv`

## Rust Translation Of The Recommended Path

### Current crate and module layout

- `ontocode-rs/ext/excel/src/lib.rs`
  - exports `install`
- `ontocode-rs/ext/excel/src/extension.rs`
  - extension registration
- `ontocode-rs/ext/excel/src/backend.rs`
  - concrete local workbook inspection implementation
- `ontocode-rs/ext/excel/src/preview.rs`
  - bounded worksheet preview reads for `.xlsx` and `.xlsm`
- `ontocode-rs/ext/excel/src/export.rs`
  - explicit CSV export implementation for `.xlsx` and `.xlsm`
- `ontocode-rs/ext/excel/src/tool.rs`
  - model-visible `excel.inspect_workbook`, `excel.read_sheet_preview`, and `excel.export_sheet_to_csv` tools
- `ontocode-rs/ext/excel/src/tests.rs`
  - bounded inspection, preview, export, and extension registration tests

### Namespace pattern

Follow the memories/web-search pattern:

- namespace: `excel`
- model-visible functions live under that namespace
- helper function similar to `memory_tool_name()` builds namespaced tool ids

### Suggested argument shapes

Prefer explicit structs and enums over vague option pairs.

Examples:

```rust
enum SheetSelector {
    Name { name: String },
    Index { index: usize },
}

struct InspectWorkbookArgs {
    path: String,
}

struct ReadSheetPreviewArgs {
    path: String,
    sheet: SheetSelector,
    max_rows: Option<usize>,
    cell_content: CellContentMode,
}

struct ExportSheetToCsvArgs {
    path: String,
    sheet: SheetSelector,
    output_csv_path: Option<String>,
}
```

### Output rules

Keep outputs bounded:

- workbook package entry count capped before entry enumeration
- XML entry reads capped before parsing
- preview rows capped
- long cell values truncated
- workbook metadata summarized rather than full-dumped
- no raw workbook binary, embedded object payloads, full drawing XML, or full VBA payloads in v1
- formula text and comment text returned only through bounded preview/sample fields

### Integration point

Install the extension where other optional tool families are installed, alongside:

- [ontocode_memories_extension::install](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/extensions.rs:32)
- [ontocode_web_search_extension::install](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/extensions.rs:33)
- [ontocode_image_generation_extension::install](/opt/demodb/_workfolder/ontocode/ontocode-rs/app-server/src/extensions.rs:34)

## Staged Delivery

### Stage 1

- add `ext/excel`
- add `excel.inspect_workbook`
- verify namespace wiring and extension registration
- keep host integration to one install line in `app-server/src/extensions.rs`
- accept only local relative workbook paths from the model-visible tool until the extension API exposes host cwd/permission context
- include bounded workbook inventory only:
  - package format and entry count
  - sheets from `xl/workbook.xml`
  - package marker counts and carrier paths for PowerQuery/DataMashup, VBA, comments, drawings/objects, charts, pivots, tables, connections, and formulas
- reject workbooks whose package entry count or XML metadata entries exceed fixed Stage 1 safety caps
- cap cumulative XML metadata reads across worksheet/custom XML scans; per-entry caps alone are not enough
- do not parse full table definitions, defined-name formulas, drawing XML, embedded object payloads, full PowerQuery M, or VBA source in Stage 1

### Stage 2

- extend `excel.inspect_workbook` only if Stage 1 users need richer bounded metadata, or add `excel.read_sheet_preview`
- add preview caps, optional formula mode for OpenXML workbooks, and fixture coverage only if preview is selected

### Stage 3

- add `excel.export_sheet_to_csv`
- verify that the exported CSV output is handoff-ready for explicit use with `spawn_agents_on_csv`

## Not In Scope For The First Slice

- `spawn_agents_on_excel_sheet`
- external workbook worker backend
- core-native Excel handlers under `core/src/tools/handlers`
- full VBA source extraction
- full PowerQuery M extraction
- connection extraction as a separate tool
- `.xlsb` formula token disassembly
- embedded object/image payload extraction
- full drawing/chart XML extraction
- a second job runner
- a second tool registry
- a copied donor worker/import architecture

Any future Excel-specific orchestration wrapper would require separate usage evidence and a new ADR; it is not part of this accepted roadmap.

## Consequences

If this ADR is followed:

- Excel support lands without expanding `core` unnecessarily
- current CSV job tooling stays the single owner for row-wise agent execution
- the first release stays small, reviewable, and architecture-aligned

If this ADR is ignored and Excel lands directly in `core` with wrapper-first orchestration:

- `core` grows with optional domain logic
- failures become harder to inspect
- neighbor-domain mixing increases without delivering more first-release value
