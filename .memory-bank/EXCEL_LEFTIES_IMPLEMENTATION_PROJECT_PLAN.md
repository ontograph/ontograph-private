# Excel Lefties Implementation Project Plan

## Status

In progress.

Phase 1A core parser slice, Phase 1B workbook-aware reference parsing, Phase 2 transient parse metadata, the Phase 3A AST-backed dependency-edge slice, Phase 3B defined-name edges, Phase 3C table edges, the Phase 3D-A Power Query lexical proof facts slice, the Phase 3D-B workbook-graph Power Query edge slice, and the Phase 4A scalar-first worksheet-formula planner/emitter are complete in `ontocode-rs/ext/excel`.

Phase 4B exact lookup SQL after reference proof is complete. Review-only exact lookup preview now covers exact `VLOOKUP(..., FALSE|0)` over a uniquely resolved defined-name range, exact `XLOOKUP`, and exact `INDEX(MATCH(...,0))` over proven aligned worksheet vectors.

Phase 4C optional aggregate SQL is complete for exact aligned `SUMIFS`, `COUNTIFS`, `AVERAGEIFS`, `MAXIFS`, and `MINIFS` preview shapes when target columns resolve exactly and all criteria/value ranges prove the same row grain.

There is no active post-Phase-4C dispatch in this loop. Reopen only for Phase 5 or Phase 6 with a separate accepted live-owner contract, or for a new offline SQL follow-up if a fresh approved fixture pack proves broader aggregate criteria semantics without weakening the fail-closed contract.

This plan converts the remaining Excel lefties into a bounded implementation order grounded in:

- current `ontocode-rs/ext/excel` owner surfaces
- donor evidence from `tmp/excel/in2sql_dotNet_addin`
- donor evidence from `tmp/excel/mcp-server-excel`

It is a project plan, not automatic approval for every phase. Each phase still needs normal implementation and verification.

## Goal

Close the remaining Excel lefties without collapsing offline parsing, graph extraction, SQL planning, and live Excel mutation into one oversized stack.

The remaining lefties are:

- no real worksheet-formula-to-SQL pipeline
- no live named-range rewrite/apply path
- no live `Formula2` / array-formula write support
- no workbook-complete dependency graph claim beyond the current bounded partial graph

## Current Baseline

The current repo already proves several bounded Excel surfaces inside offline `ext/excel`:

- workbook/package inspection
- sheet preview
- selected-sheet formula inventory
- review-only scalar worksheet-formula SQL expression preview for same-row arithmetic formulas
- review-only exact `VLOOKUP(..., FALSE|0)` worksheet-formula SQL preview when a defined-name lookup range is proven
- review-only aligned aggregate worksheet-formula SQL preview for exact `SUMIFS`, `COUNTIFS`, `AVERAGEIFS`, `MAXIFS`, and `MINIFS`
- partial workbook graph preview with bounded Power Query lineage edges from lexical proof
- named-range rewrite dry-run
- Power Query extract with lexical proof facts and heuristic SQL preview
- VBA extract and bounded translation/review helpers

This plan must extend those existing owners. It must not describe already-landed bounded surfaces as if they do not exist.

Two current starting points matter most for the remaining lefties:

- workbook graph is already implemented as an explicit Phase 1 partial graph:
  - workbook -> worksheet structure
  - selected worksheet -> formula membership
  - evidence-backed edges only
  - explicit partiality warnings
- row 041 is already implemented as a mapping-driven dry-run with positive and blocker coverage:
  - synthetic positive rewrite
  - external-link block
  - ambiguous-sheet-scope block
  - R1C1 block

## Owner Split

### Offline owner

`ontocode-rs/ext/excel`

This owner should continue to own:

- workbook/package inspection
- worksheet formula inventory
- graph extraction
- formula AST parsing
- transient formula parse metadata
- bounded SQL planning

### Live companion owner

Blocked future owner. Not accepted as an implementation owner in this repo yet.

Separate live/native Excel companion surface.

This owner should own:

- `Formula2` writes
- dynamic-array and array-formula write semantics
- named-range create/update/delete/apply
- calculation-mode coordination around live writes

Do not force live Excel mutation into offline `ext/excel`.

Do not treat this owner as implementation-ready until a separate ADR or concrete accepted owner contract exists. Until then, live work remains blocked future scope, not part of the main execution track.

## Donor Conclusions

### Donor worth reusing for offline work

`tmp/excel/in2sql_dotNet_addin`

Useful donor areas:

- typed Excel AST model and parser
- formula pattern normalization
- exact-lookup resolution
- bounded SQL emitter
- `.xlsb` formula decoding
- formula metadata sidecar shape
- Power Query lineage and graph concepts

### Donor worth reusing for live work

`tmp/excel/mcp-server-excel`

Useful donor areas:

- `Range.Formula2` read/write semantics
- named-range list/create/update/write/delete semantics
- calculation-mode suppression around bulk formula writes

### Donors not worth copying broadly

Most other Excel MCP donors are useful for simple range or workbook CRUD, but they do not materially unblock the remaining parser, SQL, graph, or live mutation gaps.

## Delivery Strategy

Build the approved remaining work in two active packages:

1. offline Power Query lineage proof plus graph emission
2. offline worksheet-formula SQL planning

Keep live companion mutation as a blocked third track that can open later only if a separate owner is accepted.

This keeps the smallest viable architecture:

- parser/graph/SQL stay offline
- live mutation does not enter the active implementation track until a real owner exists

## Phase 0: Contracts And Fixtures

### Goal

Freeze implementation boundaries before adding more code.

### Deliverables

- confirm the current ADRs remain authoritative:
  - `ADR_EXCEL_WORKSHEET_FORMULA_AST_CONTRACT.md`
  - `ADR_EXCEL_WORKSHEET_FORMULA_TO_SQL_CONTRACT.md`
  - `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md`
  - `ADR_EXCEL_WORKBOOK_GRAPH_IMPLEMENTATION_SEQUENCE.md`
  - `ADR_EXCEL_NAMED_RANGE_REWRITE_CONTRACT.md`
- add one new implementation tracker for these lefties if needed later
- collect bounded workbook fixtures for:
  - A1 formulas
  - sheet-qualified refs
  - defined names
  - structured table refs
  - exact lookup formulas
  - dynamic-array blockers

Optional later fixture track only:

- `.xlsb` formula proof cases

`.xlsb` must not be part of the baseline readiness gate for the first parser/graph/SQL slices because the current formula inspection owner supports only `.xlsx` and `.xlsm` in this stage.

### Exit criteria

- every later phase has at least one fixture that proves the intended behavior
- no phase relies on hand-authored fake graph edges or fake SQL outputs

## Phase 1: Worksheet Formula AST Parser

### Goal

Add a deterministic read-only parser inside offline `ext/excel`.

### Scope

#### Phase 1A: Core parser slice

Implement a typed AST with bounded diagnostics for:

- number, string, boolean, blank, and error literals
- unary and binary operators
- postfix percent
- function calls
- A1 references
- ranges
- sheet-qualified references

#### Phase 1B: Workbook-aware references

Only after Phase 1A is stable, add:

- defined-name references
- structured references

Explicitly represent unsupported constructs instead of guessing:

- dynamic arrays
- array constants
- external workbook references
- volatile-only constructs that are unsafe to lift

### Design rules

- parser must fail closed with diagnostics, not panic
- unsupported syntax becomes explicit unsupported nodes
- no SQL generation in this phase
- no live rewrite in this phase

### Donor guidance

Reuse shape and grammar ideas from:

- `tmp/excel/in2sql_dotNet_addin/.../Formula/Ast/ExcelFormulaAst.cs`
- `tmp/excel/in2sql_dotNet_addin/.../Formula/Ast/ExcelFormulaParser.cs`

Do not port the whole donor stack. Reuse the grammar and node taxonomy only.

### Exit criteria

- AST parser works for the supported subset
- dynamic arrays and malformed formulas produce blockers/diagnostics
- focused Rust tests cover precedence, core references, and unsupported cases
- structured refs and defined-name scope rules are allowed to land as a follow-up after the core parser slice

## Phase 2: Formula Metadata Sidecar And Pattern Normalization

### Goal

Make parsed formulas usable for graph and SQL work without reparsing raw text everywhere.

### Scope

Start with the smallest thing that works:

- attach transient parse status, AST, and diagnostics to bounded formula records already produced by the formula inventory owner
- keep the representation internal to `ext/excel` first

Do not introduce a durable sidecar or exported record store unless at least two concrete consumers prove they need it.

If later needed, add a bounded internal record model with:

- worksheet name
- cell address
- formula text
- cached value
- parsed AST status
- warnings
- optional stable formula record id
- optional normalized pattern id

Pattern normalization is optional follow-on work. Add it only when graph or SQL shows repeated reparsing or duplicate-planning cost that simpler transient records do not solve.

### Donor guidance

Reuse ideas from:

- `WorksheetFormulaMetadataDocument.cs`
- `FormulaPatternNormalizer.cs`

### Exit criteria

- parsed formula records can be referenced by later graph and SQL phases
- no durable/exported sidecar exists without a proven second consumer
- stable ids and pattern ids are deferred unless they become necessary

## Phase 3: Workbook Graph Completion

### Goal

Extend the current Phase 1 workbook graph without replacing it.

This phase must explicitly extend the already-landed partial graph contract. It is not a rewrite and it is not a license to relabel current graph output as fake.

### Sub-phases

#### Phase 3A: Formula dependency edges

Use the AST to emit real edges for:

- formula references cell
- formula references range
- formula references worksheet
- formula references defined name

No guessed precedent edges from regex.

#### Phase 3B: Defined-name edges

Implement:

- `defined_name_targets_cell`
- `defined_name_targets_range`
- `defined_name_targets_formula_text`

Keep resolved range targets separate from opaque formula-text evidence.

#### Phase 3C: Table edges

Implement parser-backed table metadata first, then:

- `table_has_range`
- structured-reference graph links where the target table/column is proven

#### Phase 3D-A: Power Query lexical proof facts

Extend the current Power Query extractor first, not the high-blast formula inventory owner.

Implement only bounded proof facts from current offline evidence:

- query references query name when the M source contains an exact `shared` query reference
- query references workbook name or table when the M source contains an exact `Excel.CurrentWorkbook(){[Name="..."]}` hit
- query load metadata remains separate and explicitly non-lineage:
  - workbook connection name
  - `Location=...`
  - command preview text

Do not emit workbook graph edges in this sub-phase.

Do not treat `Location=SalesQuery` or `SELECT * FROM [SalesQuery]` as lineage by themselves. Those fields are useful hints, not proof.

Every proof fact must carry an evidence class and an exact bounded source excerpt or source location.

#### Phase 3D-B: Graph emission from proved Power Query facts

Only after 3D-A exists, extend the workbook graph with Power Query nodes and edges where the proof fact resolves to an existing graph target.

Allowed first edges:

- query references query
- query references defined name when the workbook name resolves exactly
- query references table when the workbook name resolves exactly to parser-backed table metadata

Required rules:

- unresolved names remain warnings or blockers, not edges
- load metadata alone must not create a lineage edge
- lexical evidence must remain explicitly lexical in edge evidence
- do not claim query lineage completeness for workbooks where only some queries resolved

### Donor guidance

Reuse ideas from:

- `PowerQueryLineageDetector.cs`
- `MigrationGraphBuilder.cs`
- `MQueryParser.cs`

Use them as evidence-class examples, not as approval to overclaim completeness.

### Exit criteria

- graph contains real edge families beyond Phase 1 structure
- every emitted edge cites workbook evidence
- no placeholder completeness claims

## Phase 4: Real Worksheet-Formula-To-SQL Pipeline

### Goal

Add a bounded SQL planner/emitter driven by the AST, not by raw formula text.

### First supported subset

#### Phase 4A: Scalar-first planner/emitter

Start with one owner-local planner path dedicated to worksheet formulas. Do not stretch `translate_powerquery_to_sql_preview` into this job.

Implement only:

- same-row arithmetic projection
- simple scalar expressions
- reviewable blocked results for every unsupported formula class

Required metadata in the first shipped slice:

- worksheet and cell identity
- formula text
- AST-backed parse state
- bounded referenced-range metadata
- cached value presence for later validation

Pattern normalization is optional here. Reuse the donor idea only if it eliminates repeated planning work for copied-down formulas; otherwise skip it.

Do not promise lookup or aggregate SQL in the first emitted subset.

#### Phase 4B: Exact lookup SQL after reference proof

Only after table metadata, defined-name handling, and reference resolution are proven in current offline owners, add exact lookup patterns:

- `VLOOKUP(..., FALSE)`
- exact `XLOOKUP`
- `INDEX(MATCH(...,0))`

Exact means exact. Approximate, reverse, binary, spill-capable, or unresolved lookup shapes stay blocked.

Status: complete.

#### Phase 4C: Optional aggregates

Only after proven target-column resolution and an explicitly approved fixture pack:

- simple aggregate patterns when the target columns are proven
- grouped aggregate shapes only when the source and criteria ranges prove the same grain

Status: complete for exact aligned `SUMIFS`, `COUNTIFS`, `AVERAGEIFS`, `MAXIFS`, and `MINIFS` review-only preview shapes with equality criteria only. Operator-string criteria, wildcard criteria, mismatched ranges, unresolved targets, volatile functions, and external links remain blocked.

### Must stay blocked in first release

- approximate lookups
- volatile functions
- dynamic arrays
- external workbook refs
- unresolved structured refs
- unresolved defined names

### Donor guidance

Reuse concepts from:

- `FormulaSqlEmitter.cs`
- `FormulaLookupResolver.cs`
- `FormulaPatternNormalizer.cs`

### Output rules

- review-only or blocked is acceptable
- fabricated SQL is not acceptable
- executable SQL must come only from AST-backed plans

### Exit criteria

- planner classifies supported vs blocked formulas deterministically
- supported formulas emit bounded SQL
- unsupported formulas emit blockers, not fake SQL
- first shipped SQL slice may be scalar-only
- lookup SQL is blocked until table and name resolution are proven, not merely planned
- the first implementation stays review-only until cached-value validation rules are explicitly approved and proven

## Phase 5: Live Named-Range Rewrite Apply

### Goal

Promote row 041 from dry-run only to optional live apply, without mixing it with row 042.

Status: blocked future scope until a live owner is accepted.

### Scope

Keep two separate tools or phases:

- offline dry-run planner
- live apply tool

The live apply tool should:

- consume explicit mapping input
- validate scope and target workbook
- update formulas only when the dry-run proof is exact
- refuse ambiguous scope, external links, or unsupported reference modes

### Donor guidance

Reuse named-range CRUD semantics from:

- `mcp-server-excel` named-range commands

### Exit criteria

- dry-run remains primary
- apply is explicit and guarded
- no broad search-and-rewrite behavior
- no implementation dispatch under this phase without a separate accepted live-owner contract

## Phase 6: Live Formula2 And Array-Formula Writes

### Goal

Add the missing live formula mutation path.

Status: blocked future scope until a live owner is accepted.

### Scope

Support:

- `Formula2` get/set
- 2D formula array writes
- calculation-mode suppression during bulk writes
- explicit recalc step after bulk writes

### Separate support levels

#### Level 1

Regular `Formula2` writes to cells and ranges.

#### Level 2

Dynamic-array aware writes and readback.

#### Level 3

Legacy array-formula semantics if needed by real fixtures.

### Donor guidance

Reuse semantics from:

- `mcp-server-excel` range formula commands

### Exit criteria

- live writes are available through a companion owner
- offline `ext/excel` remains read-only
- no implementation dispatch under this phase without a separate accepted live-owner contract

## Remaining Execution Order

1. Phase 4A scalar-first worksheet-formula planner/emitter
2. Phase 4B exact lookup SQL only after reference proof
3. Phase 4C optional aggregates only after separate fixture approval
4. Phase 5 live named-range apply only after separate live-owner approval
5. Phase 6 live `Formula2` and array-formula writes only after separate live-owner approval

## Donor-Backed Implementation Options After Current Closure

The current loop has no active implementation dispatch.

The donor review does not reopen broad new work by itself. It only sharpens which donor families are worth reusing if a later reopen gate is satisfied.

### Option A: Stay offline-only in `ext/excel`

Default option.

Use `tmp/excel/in2sql_dotNet_addin` only as a donor for:

- **SliderQuery mappings**: Reconstruct Excel reporting shapes to SQL using `SliderQuery` patterns:
  - **Calculated Columns**: Row-level formulas mapped directly to SQL calculated columns.
  - **Lookup Joins**: Exact-match lookup formulas mapped to SQL join clauses.
  - **Aggregation Query**: Conditional aggregates mapped to GROUP BY summary reports.
  - **Variables**: Single-cell values and named cells mapped to SQL variables.
  - **CTE Pipelines**: Cross-sheet formulas mapped to sequential Common Table Expressions.
- grammar and AST taxonomy
- bounded formula pattern normalization when proven necessary
- exact-lookup and optional aggregate planning ideas
- COM-free `.xlsb` parsing direction for later feasibility work
- Power Query lineage and graph evidence concepts

Why this stays the default:

- matches the existing owner split
- fits the current repo, which already has offline formula AST, workbook graph, named-range dry-run, and Power Query extraction owners
- keeps the shortest path inside one bounded read-only stack

This option now owns the landed Phase 4C aggregate slice:

- exact aligned `SUMIFS`
- exact aligned `COUNTIFS`
- exact aligned `AVERAGEIFS`
- exact aligned `MAXIFS`
- exact aligned `MINIFS`

Current rules:

- target columns must resolve exactly
- criteria ranges and value ranges must prove the same row grain
- only equality criteria are previewed
- filtered worksheet state, partial columns, ambiguous headers, wildcard criteria, operator-string criteria, volatile functions, and external links remain blockers

Do not reopen cross-sheet CTE or workbook-complete SQL claims from donor strategy notes unless a later approved fixture pack proves a new bounded owner-local slice.

### Option B: Separate live companion later

Future option only.

Use `tmp/excel/mcp-server-excel` only as a donor for:

- **DAX Query materialization**: Execute DAX `EVALUATE` queries against the Data Model through direct `ADOConnection.Execute` calls on the `ModelConnection`'s in-process provider (to bypass the COM `CUBEVALUE` worksheet function automation blocker).
  - Materialize DAX query results back into Excel tables (ListObjects with `xlSrcModel` type).
- **Range.Formula2 automation**: Handle dynamic-array spill range calculations safely via visible COM sessions.
- **Named-range lifecycle**: Control workbook scoped names programmatically, handling sheet-local vs global namespaces.
- **Recalc suppression**: Disable screen updating and workbook calculations during bulk writes to avoid Excel locks.

Why this must stay separate:

- the current plan keeps live mutation outside offline `ext/excel`
- row 041 apply and row 042-style live formula writes remain blocked until a live owner is accepted
- donor semantics are useful, but the donor runtime is not a drop-in owner for this repo

### Option C: Planner and executor split

Future option only.

Keep this repo as the planner and proof owner:

- **Dry-run rewrite plans**: Output JSON-based mapping plans showing the old range, new name target, and potential conflicts.
- **AST-backed SQL plans**: Emit structured `SliderQuery` connection, query, variable, and report definitions.
- **Explicit blockers and evidence**: Capture volatile formulas or ambiguous ranges cleanly.

Let a future live companion consume those plans for:

- **Connection provisioning**: Create a localized connection to the workbook's CSV/parquet export.
- **Query materialization**: Execute the queries and populate tables.
- **Named-range apply**: Call COM/VBA to safely overwrite Excel formulas without state corruption.

Why this is the cleanest future shape:

- reuses the current dry-run and inspection owners instead of re-deriving intent in a live tool
- keeps proof and execution separated
- avoids collapsing offline parsing and live COM semantics into one oversized surface

### Option D: Stay closed until new evidence

Current manager default.

Do not reopen implementation from donor review alone.

This option is correct when:

- there is no fresh aggregate fixture pack for Phase 4C
- there is no accepted live-owner contract for Phase 5 or Phase 6
- the remaining donor ideas are architecture notes rather than proven owner-local tasks

## Proposed Next Implementation Tools

Derived from the donor reviews of `in2sql_dotNet_addin` (offline SQL / SliderQuery mapping) and `mcp-server-excel` (live COM / ADO / DAX).

### Offline Tools (Option A)

- **excel.scan_sheet_formulas_dependency**: Resolves cell dependencies to form a local DAG of formulas within a worksheet. It is used to trace computation order and detect cycles before query planning.
- **excel.generate_slider_query_package**: Transpiles parsed sheets into a folder of `.sql` queries and variable files. This packages calculated columns, joins, and aggregates into a runnable `SliderQuery` artifact package.
- **excel.read_data_validation_rules**: Parses validation lists, ranges, and criteria (e.g. numeric min/max limits) directly from worksheet XML, surfacing input restrictions without live calculation.

### Live Companion Tools (Option B/C)

- **excel.run_dax_evaluate**: Queries the workbook's internal Data Model (VertiPaq) directly. It retrieves ADO connection provider strings (`MSOLAP.8`), issues DAX `EVALUATE` command text, and returns tabular JSON output.
- **excel.materialize_dax_to_table**: Creates or refreshes an Excel table (ListObject with `xlSrcModel` connection) populated by a DAX aggregation query.
- **excel.vba_backup_modules**: Connects via COM to list and extract VBA module files (`.bas`, `.cls`, `.frm`) into text backups, unblocking bulk code review and diffing.
- **excel.write_cells_recalc_suppressed**: Writes values/formulas to cell ranges with screen-updating and automatic calculation toggled off during execution, triggering a single calculation pass at the end to prevent locks.


## Explicit Non-Goals

- no generic all-formulas SQL transpiler
- no fake “complete dependency graph” output
- no direct donor stack port
- no live mutation inside offline `ext/excel`
- no broad regex-only lineage claims

## Acceptance Standard

Every phase must satisfy all of these:

- evidence-backed behavior
- bounded output
- fail-closed handling for unsupported cases
- focused fixture coverage
- owner-local implementation without parallel runtime sprawl unless the phase is explicitly live-companion work

## Short Recommendation

Proceed as follows:

1. keep `Option A` as the landed offline default
2. keep `Option D` for any proposed reopen that lacks fresh fixture evidence
3. keep `Option B` and `Option C` blocked until a separate live-owner contract exists

Exact current reopen gates:

1. Phase 5 and Phase 6 reopen only if a separate accepted live-owner contract exists for apply and `Formula2` work
2. Any broader offline SQL follow-up reopens only if a fresh approved fixture pack proves the new aggregate or criteria semantics without weakening the fail-closed contract

That is the shortest path that keeps the donor review useful, keeps the current loop closed honestly, and avoids reopening speculative runtime scope.
