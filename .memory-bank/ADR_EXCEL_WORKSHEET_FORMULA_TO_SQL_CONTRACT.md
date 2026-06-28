# ADR: Excel Worksheet Formula To SQL Contract

## Status

Design-only. No planner, SQL emitter, tool, fixture set, or validation runner is implemented by this ADR.

## Date

2026-06-27

## Context

Current `ext/excel` owners are offline and read-only. OntoIndex confirms the existing SQL-related owner is `translate_powerquery_to_sql_preview` in `ontocode-rs/ext/excel/src/powerquery_translate.rs`, which parses Power Query M and emits a bounded heuristic SQL preview. That owner is not worksheet-formula planning and must not be stretched to claim worksheet-formula coverage.

Rows `038` and `039` define the minimum upstream dependencies for any worksheet-formula planner work:

- row `038`: an AST-backed worksheet formula contract
- row `039`: a fail-closed blocker taxonomy for array constants, spill-capable formulas, external references, volatile functions, and malformed formulas

Donor evidence from `tmp/excel/in2sql_dotNet_addin` shows why a narrow contract is required. The donor planner can map some repeated same-row formulas to `SELECT` expressions, some running totals to window expressions, and some aligned `SUMIFS` patterns to grouped aggregates. The same donor tests also block external-link workbooks, hidden or filtered row semantics, missing row identity, mismatched aggregate ranges, merged-header report regions, and unsupported AST nodes such as spill-capable `XLOOKUP(...)`.

This ADR exists to define the safe contract for any future worksheet-formula-to-SQL work without opening implementation.

## Decision

If worksheet-formula-to-SQL work is reopened, it must remain inside the current offline `ontocode-rs/ext/excel` owner and must be AST-backed from the start.

The contract is:

- input is worksheet-formula text and workbook metadata already available from offline workbook inspection
- planning is read-only and deterministic
- every emitted SQL plan must be traceable back to AST nodes and workbook metadata
- unsupported or ambiguous formulas must return explicit blockers, not guessed SQL
- validation must use cached Excel values or equivalent offline workbook evidence before any plan is treated as acceptable

This ADR approves only the contract. It does not approve a first implementation.

## Owner And Boundary Rules

Any future implementation must satisfy all of these boundaries:

- owner stays inside `ontocode-rs/ext/excel`
- source remains offline workbook inspection only
- no live Excel, COM, or `Formula2`
- no workbook mutation
- no dependency-graph extraction
- no formula evaluation engine
- no database execution sidecar
- no reuse of `translate_powerquery_to_sql_preview` as the worksheet-formula planner owner

The existing Power Query SQL preview tool may remain separate. It is a different source language, parser path, and trust model.

## Planning Model

Any future design must keep these stages explicit and fail-closed:

1. worksheet formula inventory and workbook metadata
2. AST parse under the row `038` contract
3. blocker classification under the row `039` taxonomy
4. relational-intent planning
5. SQL emission from structured plan nodes
6. cached-value or equivalent offline validation

No later stage may run when an earlier stage returned blockers.

## Supported Subset Tiers

The first implementation must not try to cover general Excel formulas. The supported subset must be tiered.

Tier A: only candidate first-slice formulas

- repeated same-row scalar formulas that map to a `SELECT` expression
- references must resolve to the same row grain inside one worksheet table or one provable repeated region
- header mapping must be explicit and deterministic

Tier B: future candidate only after separate reopen approval

- running totals or similar window-style formulas, but only when row identity is proven and stable

Tier C: future candidate only after separate reopen approval

- aligned `SUMIFS`-style grouped aggregates, but only when source and criteria ranges prove the same grain

Everything outside these tiers is blocked by default.

## Required Planning Metadata

Any future planner input must carry enough metadata to prove the mapping:

- worksheet name
- cell address
- original formula text
- normalized relative-pattern identity
- output column or target region identity
- referenced ranges or names
- cached value when present
- row identity availability for the containing region
- workbook flags that affect trust, including external-link presence and calculation-mode signals

If the planner cannot prove row grain, repeated-pattern identity, or header mapping from this metadata, it must block.

## Blocker Taxonomy For SQL Planning

The planner must fail closed for at least these categories:

- unsupported or partial AST parse
- malformed formulas
- array constants
- spill markers and spill-capable dynamic-array functions
- external workbook references or workbook-level external-link presence
- volatile functions
- hidden-row or filtered-row dependent semantics
- merged-range or merged-header report layouts that destroy stable row grain
- unresolved named ranges
- approximate, reverse, binary, or otherwise non-exact lookup modes
- mismatched aggregate source and criteria ranges
- circular references or iterative-calculation semantics
- macros, UDFs, pivot formulas, chart formulas, DAX, and non-worksheet engines

Blocked formulas must return bounded, reviewable blocker reasons. They must not degrade into best-effort SQL strings.

## Validation Contract

SQL emission alone is not enough.

Any future worksheet-formula SQL plan must be validated against offline workbook evidence before it is considered acceptable:

- compare emitted-plan results with cached Excel values for a bounded sample of formula cells, or
- compare with equivalent offline execution evidence explicitly approved in advance

Validation must block when:

- cached values are absent for the formulas under test
- workbook metadata indicates the cached values may be stale or non-trustworthy for the specific formula class
- blocker categories such as volatility, external links, hidden/filter semantics, or unsupported AST nodes are present
- the sampled SQL result diverges from cached workbook evidence

The first reopen must name the exact sample-size cap and mismatch-report shape up front.

## SQL Emission Safety Rules

Any future emitter must use structured plan nodes rather than interpolating formula text.

Required rules:

- quote identifiers deterministically
- escape literals deterministically
- preserve the source Excel caption separately from any normalized SQL identifier
- record identifier rewrites and header-normalization evidence
- never emit raw workbook formula text directly into executable SQL

## Fixture Gate Before Code

Implementation stays closed until a fresh senior-review pass approves:

1. the exact first-slice tier to open
2. Rust-owned planner and blocker output types
3. fixture strategy with real workbook artifacts, not donor-only text
4. cached-value validation rules and caps
5. blocker reporting shape for unsupported and ambiguous formulas

The first approved fixture pack must prove at least:

- same-row repeated scalar formulas
- row-grain failure when row identity is unavailable
- blocker behavior for hidden or filtered rows
- blocker behavior for external links
- blocker behavior for spill-capable formulas and unsupported AST nodes
- blocker behavior for mismatched aggregate ranges

Window and aggregate fixtures require separate approval unless they are the explicitly chosen first slice.

## Non-Goals

This ADR does not approve:

- Power Query SQL preview changes
- workbook graph extraction
- named-range rewrite
- live Excel or `Formula2`
- `.xls` support
- large-workbook XML budget policy changes
- formula evaluation or recalculation
- database execution or round-trip benchmarking

## Senior Challenge Outcome

The smallest professional move is to stop at the contract.

The donor evidence is useful because it defines where planning must block, not because it justifies broad SQL conversion. Any future reopen must start with a tiny AST-backed subset, explicit blocker reasons, and cached-value validation. Anything broader is graph theater or planner theater.
