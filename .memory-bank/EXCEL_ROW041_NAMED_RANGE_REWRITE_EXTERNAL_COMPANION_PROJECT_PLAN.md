# Excel Row 041 Named-Range Rewrite External Companion Project Plan

## Status

Planning-only. This plan does not approve implementation by itself.

## Date

2026-06-27

## Scope

This plan covers the most robust path for row `041`, but in a challenged and reduced form:

- keep offline `ontocode-rs/ext/excel` read-only
- keep named-range discovery in the current owner
- make read-only dry-run planning the real `041` implementation target
- treat any workbook mutation as an optional `041B` follow-up that depends on a separate live/native owner decision

This is intentionally not the cheapest path. It is the path that best handles shared formulas, workbook scope, sheet scope, and Excel-owned rewrite semantics without pretending the current offline ZIP/OpenXML owner is a safe mutation engine.

## Why This Path

Current code evidence:

- `excel.inspect_sheet_formulas` already returns formula text, workbook calculation flags, external-link markers, and defined-name metadata
- `DefinedNameSummary` already carries `name`, `sheet_scope`, `local_sheet_id`, `hidden`, and `target`
- current `ext/excel` is still an offline ZIP/OpenXML inspection family, not a mutation owner

That means the missing capability is not discovery. The missing capability is deterministic rewrite review. Safe mutation is a separate question and must not be smuggled into row `041` by default.

## Challenge To The Other Solutions

### Solution 0: Keep It Closed

Still valid if there is no real workbook and no proven rewrite demand.

Weakness:

- does not help when the user actually needs repeatable named-range parameterization

### Solution 1: ADR-only rewrite contract

Good prerequisite, but not enough.

Weakness:

- still leaves the hard engineering question unresolved: who owns the write

### Solution 2: Read-only dry-run planner inside current `ext/excel`

Useful and now promoted.

Weakness:

- planner correctness still needs a deterministic mapping contract
- must not grow into a hidden writer

### Solution 3: Narrow token-aware writer inside current `ext/excel`

Cheaper than a companion, but not robust enough for the ugly cases.

Weakness:

- shared formulas
- sheet-scoped names
- workbook recalculation semantics
- host-owned formula normalization
- mutation rollback and workbook save behavior

### Solution 4: AST-backed rewriter inside current `ext/excel`

Technically cleaner than token rewrite, but still the wrong owner for mutation.

Weakness:

- solves parsing before it solves write ownership
- adds a lot of code to a surface that still cannot prove host-faithful write semantics

### Solution 5: External companion mutation path

This is the most robust option only if it is split:

- `041A`: offline inspection plus deterministic dry-run
- `041B`: optional apply through a separate live/native owner

Challenge:

- if used as the default row `041` plan, it collapses row `041` into row `042`
- OntoIndex does not show an existing live/native Excel mutation owner in this repo today

## Architecture Decision

Use a staged owner model.

### Stage `041A`: existing offline `ext/excel`

This is the primary row `041` implementation target.

Responsibilities:

- inspect formulas
- inspect defined names
- surface workbook and formula metadata
- support deterministic dry-run planning inputs

Non-responsibilities:

- mutate workbook formulas
- create, delete, or update named ranges
- save workbook
- evaluate post-write Excel semantics

### Stage `041B`: optional external native/live rewrite companion

This stage is not approved by row `041` alone. It exists only if row `042` or a separate companion ADR explicitly opens a live/native mutation owner.

Responsibilities:

- load the workbook in a host that owns formula semantics
- resolve workbook-scoped and sheet-scoped names
- apply approved rewrites
- save the workbook or write a copy
- emit before/after evidence

Non-responsibilities:

- broad workbook analysis
- formula-to-SQL
- graph extraction
- generic Excel automation platform work beyond the rewrite slice

## Required User Inputs

No dry-run is attempted without all of these:

- workbook artifact
- explicit user-authored mapping list
- target mode: dry-run

The mapping list must be explicit. No automatic name synthesis is allowed.

Minimum mapping record:

- `formula_targets`: explicit formula cells or formula regions to inspect
- `from_ref`: original cell or range reference pattern
- `to_name`: existing defined name
- `scope_expectation`: workbook or sheet
- `sheet_name`: required when scope is sheet
- `max_replacements_per_formula`: hard cap or exact count
- `reference_mode`: absolute only, relative only, or exact textual match only
- `all_or_nothing`: whether any blocker cancels the whole formula rewrite candidate

Additional apply-only inputs for optional `041B`:

- approved dry-run result set
- save mode: in-place or copy-to
- output path when copy-write is used

## Hard Safety Rules

- no automatic creation of names
- no automatic deletion of old references after partial rewrite
- no dry-run proposal may claim a safe rewrite when workbook scope or sheet scope is unresolved
- no mutation when workbook has external links unless explicitly approved in the request
- no mutation when formulas are shared, array, dynamic-array, or otherwise host-sensitive unless the companion proves exact behavior for that class
- no mutation when the target name is hidden or internal unless explicitly allowed
- no mutation when a mapping crosses workbook or sheet scope incorrectly
- no mutation when dry-run produced blockers

## Phased Plan

## Phase 0: Reopen Gate

Goal:

- prove row `041` deserves implementation at all

Inputs required:

- one real workbook
- one concrete user story for why references must become named ranges
- one mapping sheet authored by the user

Exit criteria:

- explicit acceptance that row `041A` may open as read-only dry-run work
- explicit decision whether `041B` apply is even needed after dry-run

Stop if:

- the workbook can be handled by using existing named ranges as parameters without rewriting formulas

## Phase 1: Rewrite Contract ADR

Deliverable:

- `ADR_EXCEL_NAMED_RANGE_REWRITE_CONTRACT.md`

Must define:

- request payload
- dry-run result shape
- blocker taxonomy
- workbook-scope versus sheet-scope rules
- exact mapping schema

Apply result shape and save policy belong to optional `041B`, not the minimum `041A` contract.

Key blocker reasons:

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

Exit criteria:

- contract accepted before any worker opens code

## Phase 2: Evidence And Fixture Pack

Goal:

- build the smallest workbook pack that proves the hard cases

Required fixtures:

- workbook-global name rewrite
- sheet-local name rewrite
- collision case where same name exists with conflicting scope
- formula containing string literals that look like references
- shared formula region
- array or dynamic-array formula
- hidden/internal name
- workbook with external links

Required artifact types:

- original workbook
- mapping file
- expected dry-run JSON
- expected per-formula review evidence

Exit criteria:

- every accepted mutation class has a fixture

## Phase 3: Read-Only Preparation Surface

Goal:

- reuse existing `ext/excel` outputs instead of building new discovery tools

Preferred path:

- use existing `excel.inspect_sheet_formulas`
- use existing defined-name summaries

Do not add:

- a generic rewrite engine to `ext/excel`
- formula mutation code to `ext/excel`
- a rewrite-request artifact exporter in the first slice

Exit criteria:

- existing inspect output plus user mapping is sufficient for the first dry-run slice

## Phase 4: Dry-Run Engine

Goal:

- prove every candidate rewrite before mutation

Dry-run responsibilities:

- parse or normalize the target formulas enough to review replacements deterministically
- identify exact token spans eligible for replacement
- resolve `to_name` against workbook and sheet scope
- classify blockers
- produce a per-formula diff preview

Dry-run result per formula:

- formula cell reference
- original formula
- proposed rewritten formula
- matched mapping entries
- blocker reasons
- confidence

Dry-run must never mutate the workbook.

Exit criteria:

- dry-run outputs stable structured review data for all fixtures

## Phase 5: Optional Apply Engine (`041B`, depends on row `042`)

Goal:

- perform only the rewrites that passed dry-run

Open this phase only if:

- dry-run output is not enough for the user goal
- a live/native mutation owner is explicitly approved
- row `042` companion boundary is accepted or a narrower companion ADR exists

Rules:

- apply only to formulas explicitly approved from dry-run output
- default to copy-write, not in-place
- preserve workbook if any approved mutation fails mid-run
- emit before/after formula evidence

Exit criteria:

- accepted formulas rewrite correctly in fixtures
- blocked formulas remain unchanged

## Phase 6: Verification

Required checks:

- dry-run outputs match expected fixture review data
- after-workbook formulas match approved rewritten formulas when `041B` is opened
- unrelated formulas are unchanged
- defined names remain intact
- workbook opens successfully in the chosen host when `041B` is opened
- no blocked formula was mutated

Nice-to-have, not first slice:

- host recalculation check
- screenshot evidence

## Phase 7: Minimal Tooling Surface

Do not build a large public API first.

Minimum useful `041A` command:

- `named_range_rewrite_dry_run`

Optional `041B` command only if apply opens:

- `named_range_rewrite_apply`

Minimum dry-run arguments:

- workbook path
- mapping path
- optional sheet filter

Optional apply arguments:

- approved dry-run selection
- output path for copy-write
- explicit in-place flag

That is enough. Skip generic name lifecycle tools, generic range mutation, and generic workbook automation in this slice.

## Work Breakdown

### Bundle A: contract and fixtures

- write ADR
- assemble workbook pack
- define blocker taxonomy

### Bundle B: dry-run

- mapping loader
- scope resolver
- token-match classifier
- dry-run result serializer

### Bundle C: optional apply

- approved-rewrite executor
- copy-write save path
- before/after evidence writer

### Bundle D: verification

- fixture assertions
- unchanged-formula checks
- blocker-preservation checks

## Test Plan

Tests must be mostly artifact-driven.

Required test families:

- workbook-global name success
- sheet-local name success
- scope mismatch blocked
- hidden/internal name blocked
- shared formula blocked
- array/dynamic formula blocked
- string-literal ambiguity blocked
- external-link workbook blocked

Apply-only test families for optional `041B`:

- copy-write output created and original unchanged
- approved rewrites applied and blocked rewrites preserved

Do not start with broad randomized parser tests. Start with workbook fixtures that exercise real rewrite decisions.

## Stop Conditions

Stop and close with no implementation if any of these become true:

- the user’s real need is solved by existing named ranges without formula rewrite
- deterministic dry-run evidence cannot be produced from existing inspection plus user mapping
- workbook fidelity requires a much broader Excel automation platform than row `041A` justifies
- the fixture pack shows shared formulas or dynamic arrays dominate the target workbook

## Recommended Dispatch Order

1. contract ADR
2. fixture pack
3. dry-run only
4. decide whether apply is actually needed
5. if needed, approve a mutation owner
6. apply engine
7. verification and closeout

Do not dispatch apply before dry-run outputs and blocker taxonomy are stable.

## Senior Verdict

`Solution 5` is the right robust path, but only in this reduced form:

- reuse existing offline inspection
- make dry-run the real row `041` target
- treat apply as optional `041B`
- require explicit mapping with explicit formula targets
- require dry-run before apply
- default to copy-write

Anything broader is just building an Excel platform by accident.
