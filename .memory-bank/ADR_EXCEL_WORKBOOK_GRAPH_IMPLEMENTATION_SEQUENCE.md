# ADR: Excel Workbook Graph Staged Implementation Sequence

## Status

Proposed. This ADR does not approve implementation by itself. It defines the minimum phased path if rows `043-044` are explicitly reopened after acceptance of `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md`.

## Date

2026-06-27

## Context

The existing workbook-graph contract ADR is intentionally design-only. It defines node types, edge families, proof gates, and stop conditions, but it does not approve a tool, Rust types, fixtures, or parser work.

That is still correct.

The missing piece is implementation order. Rows `043-044` should not reopen as one large "workbook dependency graph" task because that invites graph theater:

- empty-edge placeholder output
- guessed formula precedents
- fake Power Query lineage
- a broad graph surface without one proven edge family

The current offline `ontocode-rs/ext/excel` owner already has enough real source evidence to support a narrow staged path:

- workbook and worksheet structure from `backend.rs`
- sheet formula inventory from `formula_inspect.rs`
- defined-name metadata from `formula_inspect.rs`
- Power Query extraction from `powerquery_extract.rs`
- table presence markers from `backend.rs`

The safe conclusion is that rows `043-044` can only reopen as one staged goal with multiple bounded phases. Row `044` remains the rule for every phase: no graph claim is valid unless real edges are extracted from current workbook evidence.

## Decision

If rows `043-044` are reopened for implementation, they must be implemented as one staged read-only graph program in this order:

1. package structure plus per-sheet formula membership
2. defined-name target edges
3. table parser, then table range edges
4. Power Query lineage proof gate only

Do not start with formula precedent edges.

Do not start with a "full workbook migration graph."

Do not ship a graph surface that mixes implemented and placeholder edge families under one completeness claim.

## Goal

Build a bounded workbook-graph inspection surface that emits only proven nodes and edges with source evidence, warnings, blockers, and truncation markers.

This goal is read-only metadata only.

Not approved by this ADR:

- formula evaluation
- workbook recalculation
- SQL generation
- formula rewrite
- workbook mutation
- live Excel automation
- dependency guesses from regex alone

## Owner

The owner remains offline `ontocode-rs/ext/excel`.

Implementation should extend the current read-only Excel extraction stack, not introduce a second runtime, sidecar, or live companion for this slice.

## Phase 1: Package Structure And Per-Sheet Formula Membership

### Scope

Implement only real structural edges:

- `workbook -> worksheet`
- `worksheet -> cell_formula` via `worksheet_contains_formula`

### Evidence Sources

- `xl/workbook.xml`
- `xl/_rels/workbook.xml.rels`
- worksheet XML formula cells already surfaced by `excel.inspect_sheet_formulas`

### Why this phase goes first

- it uses current owners directly
- it proves real edges without formula parsing
- it is useful without pretending to be dependency analysis
- it matches the current owner shape better than a workbook-wide formula sweep

### Required output minimum

- workbook node
- worksheet nodes
- cell_formula nodes for emitted formula inventory items
- structural edges with per-edge evidence
- warnings and blockers when worksheet parts or formula cells are missing or truncated

### Current-owner constraint

Phase 1 must be described honestly as per-sheet formula membership unless a separate workbook-wide formula sweep is explicitly approved.

The current formula owner is selected-sheet only. Reusing it does not justify a workbook-complete formula graph claim.

### Non-goals

- no formula precedents
- no range-reference resolution
- no named-range lineage
- no table parsing
- no Power Query lineage
- no workbook-complete formula coverage claim from a selected-sheet extractor

## Phase 2: Defined-Name Evidence Edges

### Scope

Add only two explicitly distinct evidence-backed outcomes:

- `defined_name_targets_range`
- `defined_name_targets_formula_text`

### Evidence Sources

- `definedNames/definedName` text from `xl/workbook.xml`
- existing `sheet_scope`, `local_sheet_id`, and bounded name text from `formula_inspect.rs`

### Why this phase is second

- it still uses already extracted workbook evidence
- it adds meaningful lineage without formula parsing
- it is smaller and safer than table or Power Query reference extraction

### Required rules

- workbook-scoped and sheet-scoped names must remain distinct
- ambiguous sheet-scoped names must warn or block, not guess
- hidden or internal names must be marked explicitly
- resolved worksheet range targets and opaque formula-text targets must remain distinct edge kinds
- `defined_name_targets_formula_text` is bounded evidence only, not resolved lineage

### Non-goals

- no automatic resolution of arbitrary formula text inside a defined name
- no rewrite proposal
- no name synthesis

## Phase 3A: Table Parser

### Scope

Add the minimum table parser needed to resolve:

- owning worksheet relationship
- table part path
- table `ref`

### Why this phase exists separately

- current code only detects table-part presence at marker level
- table XML and worksheet ownership are not yet parsed into reusable table metadata
- shipping `table_has_range` without this sub-phase would overstate current readiness

### Required rules

- no table graph node or edge from marker presence alone
- no guessed owning worksheet
- no silent fallback when table XML or table relationship resolution fails

### Exit criteria

- one bounded parser-backed table metadata result exists inside the graph owner
- parser output is sufficient to support evidence-backed `table_has_range`

## Phase 3B: Table Range Edges

### Scope

Add only `table_has_range`.

### Evidence Sources

- worksheet relationship parts
- `xl/tables/tableN.xml`
- table `ref`

### Why this phase is third

- it adds real structural lineage
- it is still concrete XML-backed extraction
- it does not require Power Query parsing or formula AST work
- it is allowed only after Phase 3A lands

### Required rules

- do not emit a table edge when the table relationship is missing
- do not emit a table edge when the table XML cannot be decoded
- warn or block when table parts are present at marker level but unresolved at parser level

### Non-goals

- no structured-reference formula interpretation
- no inferred table dependency from formula strings

## Phase 4: Power Query Lineage Proof Gate

### Scope

Do not open implementation by default.

Phase 4 is a separate proof gate that must decide one of these:

- explicit lexical-evidence-only lineage is acceptable for a narrow first slice
- a stronger parser or contract is required before any Power Query lineage edge is emitted

### Evidence Sources

- bounded decoded M source from DataMashup extraction
- workbook connection metadata from `xl/connections.xml`

### Why this phase is last

- it has the highest risk of fake certainty
- Power Query source can be missing, partial, or too broad
- text presence is not enough unless the emitted edge cites exact bounded evidence
- current query extraction is not yet a safe lineage extractor

### Required rules

- no graph edge from query-name presence alone
- no graph edge from workbook connection metadata alone unless the evidence is explicit
- missing or undecodable query source must warn or block
- unresolved references must remain warnings or blockers, not edges
- no implementation dispatch until the proof gate names the accepted evidence standard up front

### Non-goals

- no full M parser
- no semantic query plan
- no "lineage complete" claim

### Possible reopen outcome

If Phase 4 ever opens, it should reopen as a separate ADR or addendum that names the exact allowed evidence class for `power_query_references_name_or_table`.

## Cross-Phase Rule From Row 044

Row `044` is not a separate later feature. It is the invariant for every phase:

- no placeholder edges
- no empty-edge "complete graph" output
- no guessed dependency claims
- no tests that only assert hand-authored expected edges without parser-produced evidence

If a phase cannot extract one edge family from real workbook evidence, that phase must not ship.

## First Implementation Slice

The first valid implementation slice under this ADR is Phase 1 only.

That first slice should be described honestly as package structure plus per-sheet formula membership, not a complete dependency graph.

If a user asks for dependency analysis before later phases land, the correct answer is partial support with explicit limits.

## Output Contract Guidance

The future Rust output shape should stay small:

- bounded nodes
- bounded edges
- evidence per node and per edge
- warnings
- blockers
- truncation markers

One dedicated read-only graph tool is acceptable after explicit approval of concrete Rust-owned types.

Do not overload existing inspect outputs to imply graph completeness.

## Proof Gates Before Code

Before implementation opens under this ADR:

1. explicitly accept `ADR_EXCEL_WORKBOOK_GRAPH_CONTRACT.md`
2. explicitly accept this staged order
3. approve concrete Rust-owned output types
4. approve parser-backed fixture strategy for Phase 1 only
5. state explicitly whether Phase 1 is per-sheet only or includes a separately approved workbook-wide formula sweep

Before each later phase opens:

1. prove the prior phase is landed and named honestly
2. approve the next edge family explicitly
3. approve new fixture evidence for that edge family only
4. for Phase 4, approve the accepted Power Query evidence class before any edge output is allowed

## Stop Conditions

Stop and do not dispatch implementation when any of these are true:

- the ask is still just "workbook graph" without one named first edge family
- the proposed slice starts with formula precedents
- the proposed slice mixes real and placeholder edges
- the proposed slice claims dependency completeness from structural edges alone
- the proposed slice widens into SQL, formula rewrite, mutation, or live Excel work
- the proposed slice treats table marker presence as if table lineage were already parsed
- the proposed slice treats Power Query extraction as if lineage evidence were already approved

## Practical Reopen Recommendation

If implementation is ever approved, reopen only this sequence:

- Phase 1 now
- Phase 2 later if named-range lineage is still needed
- Phase 3A, then 3B later if table lineage is still needed
- Phase 4 only after a separate Power Query lineage proof decision

Everything else stays closed.
