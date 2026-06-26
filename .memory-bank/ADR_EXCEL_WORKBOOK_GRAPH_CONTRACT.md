# ADR: Excel Workbook Graph Contract

## Status

Design-only. No extraction is implemented; this document defines the contract that a future implementation must satisfy.

## Date

2026-06-26

## Context

The Excel MCP review kept workbook graph extraction blocked because the current Excel extension has no graph schema, no Rust graph type, no parser, and no output contract. A fixture-first proof was rejected as graph theater: it would assert expected edges without proving any real edge extraction.

Current `ext/excel` owners are read-only workbook inspection, preview, formula inventory, Power Query extraction, VBA extraction, and translation previews. This ADR does not add a tool, module, fixture, schema file, Rust type, or parser.

## Decision

If workbook graph extraction is ever reopened, it must start from this contract:

- Graph output is read-only metadata.
- Every node and edge must carry source evidence from a workbook part or bounded decoded source text.
- Missing or unsupported source evidence must produce warnings or blockers, not guessed edges.
- A future code slice requires a fresh senior-review pass and explicit user acceptance of the new graph architecture surface.

## Node Types

`workbook`

- Source: package path and workbook-level OpenXML metadata.
- Current related owner: `excel.inspect_workbook`.

`worksheet`

- Source: `xl/workbook.xml` sheet entries plus `xl/_rels/workbook.xml.rels`.
- Current related owner: workbook inspection and sheet selection.

`cell_formula`

- Source: selected worksheet XML formula cells, for example `xl/worksheets/sheetN.xml`.
- Current related owner: `excel.inspect_sheet_formulas`.

`defined_name`

- Source: `xl/workbook.xml` `definedNames/definedName`.
- Current related owner: `excel.inspect_sheet_formulas` defined-name metadata.

`table`

- Source: table relationship parts and `xl/tables/tableN.xml`.
- Current related owner: workbook marker inspection only; detailed table parsing is not implemented.

`power_query`

- Source: `xl/connections.xml`, `customXml/item*.xml`, DataMashup payloads, or bounded decoded Power Query source.
- Current related owner: `excel.extract_powerquery_queries`.

## Edge Types

`worksheet_contains_formula`

- From: `worksheet`.
- To: `cell_formula`.
- Required evidence: worksheet XML cell reference and formula element.

`formula_references_cell_or_range`

- From: `cell_formula`.
- To: worksheet cell or range reference.
- Required evidence: formula text from worksheet XML.
- Block when the reference cannot be resolved without formula parsing beyond the approved subset.

`defined_name_targets_range_or_formula`

- From: `defined_name`.
- To: worksheet cell/range reference or formula text.
- Required evidence: `definedName` text from `xl/workbook.xml`.

`table_has_range`

- From: `table`.
- To: worksheet range.
- Required evidence: table XML `ref` and table relationship from the owning worksheet.

`power_query_references_name_or_table`

- From: `power_query`.
- To: `defined_name` or `table`.
- Required evidence: bounded decoded Power Query source line or workbook connection metadata.

## Warning And Blocker Rules

Warnings are allowed only when the source is real but incomplete for graph confidence. Blockers are required when the graph would otherwise guess.

Must warn or block:

- dynamic arrays or spill markers
- external workbook links
- volatile functions
- `INDIRECT` and `OFFSET`
- unsupported structured references
- missing worksheet relationship parts
- missing table XML
- missing or undecodable Power Query source
- ambiguous sheet-scoped defined names

## Output Contract Shape

Future output should contain:

- a bounded list of nodes
- a bounded list of edges
- per-node source evidence
- per-edge source evidence
- warnings
- blockers
- truncation markers

This is intentionally prose only. Do not copy this section into Rust types or JSON fixtures without a new approved implementation task.

## Required Proof Gate Before Code

Before any Rust implementation opens:

1. Approve concrete Rust-owned output types in a fresh senior-review pass.
2. Add a synthetic fixture only after those types are approved.
3. The fixture must include two worksheets, one cross-sheet formula, one workbook-scope defined name, one sheet-scope defined name, one table, and one Power Query reference to a table or defined name.
4. The first implementation must assert parser-produced output, not manually constructed expected edges.

## Stop Conditions

- No Rust struct, enum, trait, module, or parser is approved by this ADR.
- No JSON, TOML, or workbook fixture is approved by this ADR.
- No `inspect_workbook_graph` or similar tool is approved by this ADR.
- No graph output may be shipped with placeholder or empty edges.
- No formula evaluation, SQL generation, formula rewrite, workbook mutation, dependency recalculation, or live Excel automation is approved.

## Senior Challenge Outcome

This ADR exists because `N5-A` fixture-first proof was blocked. A text contract is acceptable because it does not claim graph extraction exists. Any future code must prove real edge extraction from workbook sources and must be reviewed as a separate bounded task.
