# ADR: Offline VBA To ONLYOFFICE Refactoring Guardrails

## Status

Accepted current-state guardrails. No implementation approval by itself.

## Date

2026-06-25

## Context

The offline VBA to ONLYOFFICE work now has a narrow shipped path:

- Stage 0 target contract capture is complete
- Stage 1 analyzer is complete
- Stage 2 fail-closed preview translator is complete
- Stage 3 workbook review is complete only as the narrow read-only `excel.review_vba_onlyoffice_workbook` wrapper

The audit and challenge pass across the ADR set found that the current code is broadly aligned with the intended narrow product shape, but the documents can still be misread in ways that would justify unnecessary refactors:

- the canonical ADR describes a future three-layer `parser -> IR -> emitter` architecture
- the shipped implementation is still analyzer output flowing directly into a fail-closed emitter
- the workbook-review wrapper is a composition tool, not a second translation owner
- the deferred plan is the only reopen authority; the detailed plan is execution scaffolding only

OntoIndex was useful for freshness checks and general retrieval, but coverage for the newer `vba_onlyoffice_*` files remained partial during the audit. Final refactoring decisions therefore had to be grounded in direct source reads as the authority after the OntoIndex pass.

## Audit Conclusions

### What was proposed correctly

- keep the owner in `ontocode-rs/ext/excel`
- keep tool surfaces explicit
- keep the analyzer as the safety gate
- keep the translator preview-only and fail-closed
- keep workbook review as a thin read-only wrapper instead of a migration bundle

### What was proposed too aggressively

- treating the future IR split as if it were already part of the implemented architecture
- treating a public static validator as if it were a justified next step without a second sink
- treating workstation-local workbook samples as durable authority rather than advisory trigger evidence

### What was done correctly

- the translator refuses to emit when analysis is unsafe, unsupported, warning-bearing, redacted, oversized, or unmapped
- workbook review reuses extraction, analysis, and the same fail-closed translator rather than creating a parallel emitter
- Stage 0 wrap and alignment calls were kept in the contract and later proven by focused tests instead of being narrowed defensively

## Decision

Refactoring in this area must optimize for preserving the existing narrow owner and behavior, not for architectural neatness.

The binding current-state reading is:

1. the analyzer plus direct fail-closed emitter is the implemented architecture
2. the neutral IR and emit-module split are future refactor options, not present-tense design owners
3. workbook review is a wrapper over the analyzer/emitter pair, not a second translation pipeline
4. no refactor may create a second public translation, validation, routing, or runtime stack by accident
5. boundedness is part of the contract, not an implementation detail
6. warning-bearing or redacted analysis remains a non-emitting state, not a degraded-success state
7. workbook-review `success` remains review-completion state only, not migration-success state

## Allowed Refactors

These are acceptable only when justified by concrete local pressure that matches the deferred-plan trigger model:

- small owner-local cleanup inside `ontocode-rs/ext/excel`
- emit helper extraction only after real operation-growth or duplication pressure
- targeted parser augmentation only for one redacted syntax-only gap inside already supported semantics
- test-only recorder drift checks when the Stage 0 contract is deliberately refreshed or a supported `Api.*` shape changes
- docs cleanup that removes drift between ADRs, tracking, and shipped behavior

Small owner-local cleanup is not a loophole for speculative reshaping.

If a proposed cleanup:

- changes public payload meaning
- weakens fail-closed behavior
- widens bounds or caps
- introduces a second sink or routing path
- or moves toward parser, IR, validator, or runtime expansion without one concrete trigger

then it is not cleanup and must be challenged as a reopen proposal instead.

## Graph Evidence Rule

When OntoIndex coverage is available for the relevant owner files or symbols, refactoring proposals should be justified with a bounded graph-evidence pack before any structural change is accepted.

The preferred pack is:

- one owner-symbol neighborhood check
- one cluster or process check
- one bounded read-first file set
- one freshness note

The goal is not full-graph exploration. The goal is to answer one narrow question:

- is the proposed refactor responding to real owner pressure, or only to a cleaner-looking design?

When OntoIndex coverage is partial for the newer ONLYOFFICE files, direct source reads remain the authority, but the same bounded-evidence discipline still applies.

## Preferred Graph Techniques

Use existing graph-backed surfaces and patterns before proposing any new VBA-specific review helper:

- concept or owner discovery should follow the same bounded-report pattern used by exploration surfaces such as `gn_explore`
- symbol-neighborhood checks should follow the same caller/callee/co-change/cluster pattern used by `gn_find_related`
- corner-case ownership drift checks may use a bounded walk pattern like `gn_graph_walk`, with explicit caps
- process and cluster evidence should be used to challenge any claim that workbook review has become an independent pipeline owner

Do not introduce a special ONLYOFFICE/VBA graph tool unless the existing graph evidence surfaces first prove insufficient.

## Blocked Refactors

These remain blocked unless a fresh deferred-plan trigger is proven:

- public `excel.validate_onlyoffice_macro_preview`
- generic `excel.translate`
- runtime ONLYOFFICE execution or replay validation
- broad VBA parser dependency adoption
- neutral IR split performed only for neatness
- workbook-review expansion into bundle generation, mutation, or macro injection

## Refactoring Tests

Any future refactor in this area must preserve these facts:

- unsafe or partially understood VBA still fails closed
- warning-bearing analysis still produces no emitted macro preview
- redacted analysis still produces no emitted macro preview
- workbook review still routes through the same analyzer-first gate
- public payload meaning does not drift, especially workbook-review `success`
- Stage 0 contract calls that are claimed as supported remain positively covered by tests
- bounded caps remain intentionally enforced for source size, procedure count, operation count, warning count, and emitted macro size

## Bound Invariants

The current hard limits are part of the safety contract and must not be widened, removed, or bypassed casually during refactoring.

Until a separate review explicitly approves a change, preserve the existing bounded behavior around:

- source-size caps
- procedure-count caps
- operation-count caps
- warning-count caps
- emitted macro-size caps
- truncation and redaction behavior

A refactor may move where those checks live, but it must not weaken when they apply or what fail-closed outcome they force.

## Workbook Review Invariants

The workbook-review wrapper has specific meaning that refactors must preserve:

- preview generation still occurs only through the same analyzer-first path
- a reviewed module may still have empty `macro_value` and `function_body` even when workbook review succeeds
- top-level `success` still means the workbook review completed and at least one module was reviewed
- top-level `success` does not mean every module emitted a preview
- top-level `success` does not mean the workbook is migration-ready

Any refactor that wants to change these meanings is a product-contract change, not a structural cleanup.

## Graph-Backed Reopen Signals

Graph evidence is useful only if it distinguishes real structural pressure from aesthetics.

Acceptable graph-backed signals include:

- duplicated emitter ownership across more than one file or symbol neighborhood
- workbook-review logic gaining distinct cluster or process identity instead of remaining a thin wrapper over analyzer and emitter owners
- repeated co-change pressure showing the same refactor seam is touched together across owner files
- a new supported operation family causing the emitter owner to grow in a way that matches the existing deferred-plan `B1` trigger

Non-signals:

- a file merely being longer than preferred while graph ownership is still stable
- a wrapper having several callsites but no independent logic ownership
- a desire to make the architecture resemble the future parser/IR/emitter target before trigger evidence exists
- raw workbook corpus growth without a redacted syntax-only or drift-backed reopen trigger

## Trigger Rule

This ADR does not open work.

If a later refactor is proposed, the first question is not "would this structure be cleaner?"

The first question is:

- does current code now have concrete duplication, file-growth, drift, or syntax-gap pressure that the existing owner cannot absorb cleanly?

If the answer is no, do not refactor.

If the answer is yes, use the deferred implementation plan as the only reopen authority, classify the pressure explicitly, and keep the change inside the smallest existing owner that already owns the behavior.

Accepted pressure classes are:

- operation-growth pressure
- duplicated emitter logic
- large-file pressure
- recorder-contract drift
- one concrete redacted syntax-only gap in already supported semantics

Everything else should be treated as either semantics-blocked, scope-blocked, or not yet justified.

Large-file pressure alone is not enough when graph evidence still shows a stable single owner with no duplicated logic or second-pipeline drift. In that case, prefer no refactor.
