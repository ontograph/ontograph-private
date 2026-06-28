# Excel Offline Next Tools Project Plan

## Status
Open task queue with no dependency-ready implementation task. `EXCEL-OFFLINE-T0`, `EXCEL-OFFLINE-T2`, and `EXCEL-OFFLINE-T3` are completed. `EXCEL-OFFLINE-T1` stays open, but it is not dispatchable because it still needs explicit consumer proof against current workbook-marker coverage.

## Active Next Task
None. `EXCEL-OFFLINE-T1` is gated on new evidence; `EXCEL-OFFLINE-T4` remains deferred no-dispatch.

## Goal
Promote the strongest offline donor ideas from `tmp/excel` into the existing `ontocode-rs/ext/excel` owner without reopening postponed live COM/ADO work.

## Donor Basis
- Primary donor: `tmp/excel/in2sql_dotNet_addin`
- Secondary donor evidence only: `tmp/excel/mcp-server-excel`
- Rejected for this plan: generic workbook CRUD, formatting, charting, and live VBA write servers

## Scope Rules
- Stay inside `ontocode-rs/ext/excel`
- Reuse existing owners before adding new tools
- Keep all work read-only and offline
- Prefer additive metadata and artifact packaging over new orchestration layers
- Do not reopen `excel-live`, COM, ADO, DAX execution, chart mutation, or live VBA writes

## Phase 0: Bounded `.xlsb` Read Support In Existing Owners
**Shape**: no new tool
**Owner**: existing `read_sheet_preview` and `inspect_sheet_formulas`
**Why first**: donor evidence showed the best remaining offline gap was bounded offline `.xlsb` preview and formula reads without fabricating unsupported metadata
**Expected files**:
- `ontocode-rs/ext/excel/src/preview.rs`
- `ontocode-rs/ext/excel/src/formula_inspect.rs`
- `ontocode-rs/ext/excel/src/tests.rs`
**Exit gate**:
- `.xlsb` preview path remains fail-closed where unsupported
- formula/value reads come from the real offline reader path where available, and unsupported metadata stays explicit with warnings instead of fabricated parity claims

## Phase 1: `excel.inspect_pivot_report_metadata`
**Shape**: candidate new offline read-only tool only if existing workbook markers are proven insufficient
**Owner**: first challenge the need against the current `inspect_workbook` owner; only add one owner-local extractor module if a concrete consumer still needs metadata that current markers cannot provide
**Purpose**: extract PivotTable and PowerPivot report metadata from workbook packages, including cache links, source ranges, OLAP flags, stored MDX when present, and bounded warnings when not present
**Donor basis**: `ManagedPivotReportMetadataExtractor`
**Why later**: current `inspect_workbook` already exposes bounded pivot presence through workbook markers, so this stays behind the lower-blast Power Query follow-ons until a concrete offline consumer proves those markers are insufficient

## Phase 2: `excel.lint_powerquery_queries`
**Shape**: new offline read-only tool, or additive extension of `excel.extract_powerquery_queries` if the output surface stays simpler
**Owner**: existing Power Query extraction owner
**Purpose**: run bounded lint over extracted `.m` queries and emit a compact report with missing `let`/`in` structure and empty-query warnings
**Donor basis**: `MQueryLintPostWork`
**Why now**: cheapest next signal, lowest blast radius, and direct reuse of current extracted artifacts

## Phase 3: `excel.generate_powerquery_review_bundle`
**Shape**: new offline packaging tool
**Owner**: existing Power Query extraction owner
**Purpose**: package extracted queries, lineage summary, lint report, normalized copies, and a manifest into a review-ready folder
**Donor basis**: `LineageReportPostWork`, `MarkdownExportLogWriter`, `ArtifactPackageManifestPostWork`, `MQueryNormalizePostWork`
**Why next**: composes existing offline outputs into one consumable artifact without adding runtime dependencies

## Phase 4: Optional Follow-Up
**Tool**: `excel.inspect_workbook_connections_detailed`
**Purpose**: bounded connection/load-target metadata for Power Query and Data Model handoff
**Condition**: open only if Phase 1-3 land cleanly and a real consumer still needs deeper connection visibility

## Non-Goals
- `excel.run_dax_evaluate`
- `excel.materialize_dax_to_table`
- `excel.write_cells_recalc_suppressed`
- direct chart, range-write, formatting, or workbook-create tools
- live VBA module writes or deletes

## Dispatch Order
1. Phase 0 bounded `.xlsb` read support in existing owners
2. Phase 2 Power Query lint
3. Phase 3 review bundle
4. Phase 1 pivot report metadata, only if a concrete offline consumer still needs more than current workbook markers
5. Phase 4 only with fresh demand

## Open Tasks

### `EXCEL-OFFLINE-T0` bounded `.xlsb` read support in existing owners
- **Status**: completed
- **Goal**: extend the existing preview and formula-inspection owners so `.xlsb` workbooks stop failing at the package gate when the offline reader can provide real sheet values or formula metadata, while keeping unsupported metadata fail-closed
- **Owners**:
  - `read_sheet_preview_with_display_path` in `ontocode-rs/ext/excel/src/preview.rs`
  - `inspect_sheet_formulas_with_display_path` in `ontocode-rs/ext/excel/src/formula_inspect.rs`
- **Expected files**:
  - `ontocode-rs/ext/excel/src/preview.rs`
  - `ontocode-rs/ext/excel/src/formula_inspect.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
- **Donor basis**:
  - `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Import/Managed/Xlsb/XlsbWorkbookReader.cs`
  - `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Import/Managed/Xlsb/XlsbFormulaReader.cs`
- **Acceptance**:
  - `.xlsb` preview no longer fails at the blanket Stage 2 package rejection when managed offline reads are possible
  - preview output stays fail-closed for fields the managed reader still cannot populate
  - formula inventory metadata comes from managed extraction, not guessed fallbacks
  - unsupported `.xlsb` subcases emit explicit warnings or bounded unsupported markers instead of silently pretending full parity
- **Validation**:
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- **Reopen gate if blocked**:
  - block only if the crate has no viable offline `.xlsb` reader path without reopening live Excel or adding a new tool surface
- **Closure**:
  - landed through the existing `read_sheet_preview` and `inspect_sheet_formulas` owners using a pure-Rust `.xlsb` reader path
  - real `.xlsb` fixture coverage now proves bounded preview reads and non-error formula inspection on binary workbooks
  - unsupported `.xlsb` metadata remains explicit: data validations, calculation settings, style metadata, shared-formula metadata, and richer defined-name fields still stay fail-closed with warnings or `None`

### `EXCEL-OFFLINE-T1` `excel.inspect_pivot_report_metadata`
- **Status**: open, pending `T2` and explicit consumer proof
- **Goal**: add a read-only offline pivot metadata surface only if current workbook markers cannot answer a concrete offline consumer question
- **Owner**: challenge the request against the existing `inspect_workbook` output first; only if that fails, add one owner-local extractor module under `ontocode-rs/ext/excel` and wire it through the existing Excel tool registry after the output shape is stable
- **Expected files**:
  - optionally `ontocode-rs/ext/excel/src/backend.rs` if the smallest correct fix is additive marker metadata under the current workbook owner
  - `ontocode-rs/ext/excel/src/tool.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
  - optionally one new owner-local extractor module under `ontocode-rs/ext/excel/src/`
- **Donor basis**:
  - `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/Import/Managed/ManagedPivotReportMetadataExtractor.cs`
- **Acceptance**:
  - the task closes no-dispatch if the existing `inspect_workbook` markers already answer the real consumer need
  - output includes bounded report identity, source/cache links, OLAP or Data Model flags, and stored MDX only when present in-package
  - missing cache, model, or MDX state yields warnings rather than fabricated fields
  - no live refresh, COM calls, or Data Model execution paths are introduced
- **Validation**:
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- **Reopen gate if blocked**:
  - reopen for implementation only after a concrete offline consumer cannot answer its pivot-report question from current `inspect_workbook` markers and the missing fields are still provable from workbook packages without live execution

### `EXCEL-OFFLINE-T2` `excel.lint_powerquery_queries`
- **Status**: completed
- **Goal**: provide a bounded lint pass over already extracted Power Query `.m` text
- **Owner**: extended the existing Power Query extraction owner in `ontocode-rs/ext/excel/src/powerquery_extract.rs`; no separate tool surface was needed
- **Expected files**:
  - `ontocode-rs/ext/excel/src/powerquery_extract.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
  - optionally `ontocode-rs/ext/excel/src/tool.rs` if a dedicated tool surface is required
- **Donor basis**:
  - `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/PostWork/Lint/MQueryLintPostWork.cs`
- **Acceptance**:
  - lint reports at least malformed or missing `let`/`in` structure, empty queries, and other lexical red flags that can be proven without executing M
  - the lint pass works only from offline extracted text and never claims semantic correctness it cannot prove
  - if a separate tool is added, it reuses existing extraction logic instead of duplicating package reads
- **Validation**:
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- **Reopen gate if blocked**:
  - block only if the extracted text surface is too lossy to support even lexical lint without false confidence
- **Closure**:
  - landed as an additive extension of `excel.extract_powerquery_queries`, not a second Power Query tool
  - extraction results now include bounded per-query lint findings and aggregate `lint_finding_count`
  - the lint slice stays lexical and fail-closed: empty query body, missing shared-query definition, and missing `let`/`in` structure are reported without claiming semantic correctness
  - focused tests cover clean extraction, lexical-reference extraction, corrupted payload handling, and bounded lint findings

### `EXCEL-OFFLINE-T3` `excel.generate_powerquery_review_bundle`
- **Status**: completed
- **Goal**: package existing offline Power Query artifacts into one review-ready folder
- **Owner**: composed around the existing Power Query extraction owner with one owner-local packaging module and no parallel extraction stack
- **Expected files**:
  - `ontocode-rs/ext/excel/src/powerquery_extract.rs`
  - `ontocode-rs/ext/excel/src/tool.rs`
  - `ontocode-rs/ext/excel/src/tests.rs`
  - optionally one new owner-local packaging module under `ontocode-rs/ext/excel/src/`
- **Donor basis**:
  - `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/PostWork/LineageReport/LineageReportPostWork.cs`
  - `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/PostWork/ArtifactPackageManifestPostWork.cs`
  - `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Application/PostWork/MQueryNormalizePostWork.cs`
  - `tmp/excel/in2sql_dotNet_addin/DataManager/Features/PowerQuery/Infrastructure/Logging/MarkdownExportLogWriter.cs`
- **Acceptance**:
  - emits a deterministic folder with extracted queries, normalized copies if normalization is implemented, lint summary, lineage summary, and a manifest
  - bundle generation remains offline and read-only
  - missing optional artifacts are represented explicitly in the manifest instead of being silently dropped
- **Validation**:
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`
  - `cd /opt/demodb/_workfolder/ontocode/ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- **Reopen gate if blocked**:
  - block only if the existing offline extraction outputs are too incomplete to make the bundle materially better than the raw extractor output
- **Closure**:
  - landed as `excel.generate_powerquery_review_bundle`
  - emits a deterministic folder with extracted `.m` query files, `reports/lint-summary.json`, `reports/lineage-summary.json`, and `manifest.json`
  - normalization remains explicit and honest: the manifest reports `not_implemented` instead of fabricating normalized query copies
  - focused tests cover direct bundle generation and turn-cwd-relative tool execution

### `EXCEL-OFFLINE-T4` `excel.inspect_workbook_connections_detailed`
- **Status**: deferred, no-dispatch
- **Goal**: expose deeper workbook connection and load-target metadata only if a real offline consumer still needs it after `T1` to `T3`
- **Owner**: extend current workbook and Power Query package readers, not a new connection subsystem
- **Acceptance**:
  - open only with fresh consumer evidence that Phase 1 to 3 outputs are insufficient
  - stay bounded to offline connection metadata already present in the workbook package
- **Exact reopen gate**:
  - reopen only after `EXCEL-OFFLINE-T1`, `EXCEL-OFFLINE-T2`, and `EXCEL-OFFLINE-T3` land and a concrete downstream consumer still cannot answer its connection-routing question from those outputs

## Reopen Gate For Live Donors
If a future request needs DAX execution, calculation-mode control, or live workbook mutation, route it to the postponed `excel-live` path rather than extending this plan.
