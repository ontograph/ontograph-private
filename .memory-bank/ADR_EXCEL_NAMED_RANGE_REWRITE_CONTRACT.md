# ADR: Excel Named-Range Rewrite Contract

## Status

Design-only. No dry-run engine, mutation path, Rust type, tool, fixture loader, or workbook writer is implemented by this ADR.

## Date

2026-06-27

## Context

Row `041` started from a reasonable user need, but the earlier solution space drifted into mutation semantics that belong to row `042` or a separate live/native companion owner.

Current code evidence keeps the first safe slice much narrower:

- `excel.inspect_sheet_formulas` already returns bounded worksheet formula inventory
- `InspectSheetFormulasResult` already includes workbook calculation flags, `defined_names`, `defined_names_sample`, and `has_external_links`
- `DefinedNameSummary` already includes `name`, `sheet_scope`, `local_sheet_id`, `hidden`, `target`, and `truncated`
- OntoIndex review still shows the current Excel owner as offline `ontocode-rs/ext/excel` inspection, not a live/native formula-mutation owner

That means discovery is already open. The missing piece is a deterministic review contract for proposed rewrites. Workbook mutation must remain optional and separately gated.

This ADR exists to define the contract for row `041A` only:

- offline
- read-only
- deterministic
- explicit mapping driven

## Decision

If row `041` is reopened for implementation, the first slice must be a read-only named-range rewrite dry-run inside the current offline `ontocode-rs/ext/excel` owner.

The contract is:

- input comes from existing offline workbook inspection plus an explicit user-authored mapping file
- output is a bounded dry-run review result
- no automatic name synthesis is allowed
- no workbook mutation is allowed in the first slice
- no apply path is approved by this ADR

Any mutation path remains optional `041B` work and requires a separate live/native owner decision outside offline `ext/excel`.

## Owner And Boundary Rules

Any future `041A` implementation must satisfy all of these rules:

- owner stays inside `ontocode-rs/ext/excel`
- source remains offline workbook inspection only
- reuse existing formula inventory and defined-name metadata instead of creating a new discovery stack
- no workbook save
- no named-range create, rename, or delete
- no formula writeback
- no live Excel, COM, `Formula2`, or host automation
- no dependency-graph extraction
- no SQL planning

The dry-run may normalize or tokenize formulas enough to review replacements deterministically, but it must never become a hidden writer.

## Request Contract

No dry-run request is valid without all of these:

- workbook path
- explicit user-authored mapping list
- explicit formula targets
- explicit statement that the request is dry-run only

The minimum mapping record is:

- `formula_targets`: explicit formula cells or formula regions to inspect
- `from_ref`: original cell or range reference pattern
- `to_name`: existing defined name
- `scope_expectation`: `workbook` or `sheet`
- `sheet_name`: required when `scope_expectation` is `sheet`
- `max_replacements_per_formula`: hard cap or exact count
- `reference_mode`: `absolute_only`, `relative_only`, or `exact_textual_match_only`
- `all_or_nothing`: whether any blocker cancels the whole formula candidate

This ADR does not approve automatic mapping inference from raw workbook content.

## Dry-Run Responsibilities

Any future `041A` dry-run must do only these things:

- load worksheet formula inventory for the targeted sheet or sheets
- resolve the candidate `to_name` against workbook scope and sheet scope
- identify exact formula token spans eligible for replacement
- classify blockers
- emit a per-formula diff preview

It must not:

- mutate formulas
- create or edit names
- save a workbook copy
- claim Excel-host fidelity for mutation semantics

## Dry-Run Result Contract

Any future dry-run result must return bounded structured review data per formula:

- formula cell reference
- original formula text
- proposed rewritten formula text
- matched mapping entries
- blocker reasons
- confidence
- truncation marker when any output is shortened

Workbook-level output must also preserve bounded evidence needed for review:

- selected sheet identity
- external-link presence
- workbook calculation flags
- bounded defined-name metadata relevant to the request
- warnings

## Scope Resolution Rules

Workbook and worksheet scope must be fail-closed.

Required rules:

- a workbook-scoped name must not be proposed when the mapping requires a sheet-scoped name
- a sheet-scoped name must not be proposed unless the target worksheet scope resolves unambiguously
- same-text names with conflicting workbook and sheet scope must be blockers unless the mapping resolves the ambiguity explicitly
- hidden or internal names must block unless the request explicitly allows them
- unresolved `localSheetId` to worksheet-name mapping must block

No dry-run proposal may claim a safe rewrite when workbook scope or sheet scope is unresolved.

## Blocker Taxonomy

The minimum blocker taxonomy for any future `041A` implementation is:

- `name-not-found`
- `scope-mismatch`
- `ambiguous-sheet-scope`
- `hidden-or-internal-name`
- `shared-formula-blocked`
- `array-or-dynamic-formula-blocked`
- `external-link-blocked`
- `string-literal-ambiguity`
- `structured-reference-blocked`
- `tokenization-failed`
- `replacement-count-mismatch`
- `formula-target-not-found`
- `sheet-name-missing`
- `sheet-name-mismatch`

Blocked formulas must return explicit, reviewable reasons. They must not degrade into guessed rewrites.

## Hard Safety Rules

Any future implementation must enforce all of these:

- no automatic creation of names
- no automatic deletion of old references after partial rewrite
- no rewrite proposal when workbook scope or sheet scope is unresolved
- no apply path inside offline `ext/excel`
- no mutation claim when workbook has external links unless a separate apply owner is explicitly approved for that workbook
- no safe-rewrite claim for shared formulas, array formulas, or dynamic-array formulas unless that exact class is explicitly reopened and proven
- no rewrite when the mapping crosses workbook or sheet scope incorrectly
- no success result when blockers exist for an `all_or_nothing` candidate

## Fixture Gate Before Code

Implementation remains closed until a fresh senior-review pass approves:

1. one real workbook that proves named-range rewrite is actually needed
2. one explicit user story for why direct references must become named ranges
3. one user-authored mapping file that satisfies this ADR
4. one fixture pack that covers the blocker taxonomy
5. concrete Rust-owned request and dry-run result types

The first fixture pack must prove at least:

- workbook-global name rewrite candidate
- sheet-local name rewrite candidate
- collision case with conflicting workbook and sheet scope
- formula containing string literals that look like references
- shared formula region
- array or dynamic-array formula
- hidden or internal name
- workbook with external links

## Optional Apply Boundary

This ADR does not approve apply.

If the user later proves that dry-run is not enough, any optional `041B` apply path must be reopened separately and must satisfy all of these:

- explicit dry-run approval set exists first
- a live/native mutation owner is approved outside offline `ext/excel`
- copy-write is the default save policy
- before/after workbook evidence is emitted
- row `042` semantics are handled by the live/native owner rather than backfilled into the offline stack

## Non-Goals

This ADR does not approve:

- formula writes
- named-range lifecycle management
- workbook save or save-as
- live Excel or `Formula2`
- SQL generation
- workbook graph extraction
- `.xls` support
- large-workbook XML budget policy changes

## Senior Challenge Outcome

The correct robust move is to stop at the dry-run contract.

The repo already has enough inspection surface to support explicit review inputs. It does not have a justified mutation owner. Row `041` therefore stays narrow:

- reuse current offline inspection
- require explicit mapping
- make dry-run the real target
- keep apply optional and separately gated

Anything broader would collapse row `041` into row `042` and build workbook mutation semantics in the wrong owner.
