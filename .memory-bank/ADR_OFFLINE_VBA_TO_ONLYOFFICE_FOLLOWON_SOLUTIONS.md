# Offline VBA To ONLYOFFICE Follow-On Solutions

Date: 2026-06-24

## Status

Mixed state.

- Option A is completed as a narrowed Stage 3 slice.
- Option B was re-reviewed and kept deferred with no implementation dispatch.
- Options C-F remain deferred proposals only.

## Context

The accepted current state is now the narrow Stage 0-3 path captured across [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md) and the follow-on tracking/closure notes:

- Stage 0 target contract is complete.
- Stage 1 analyzer is complete.
- Stage 2 fail-closed preview translator is complete.
- Stage 3 workbook-assisted flow is complete only as the narrow read-only `excel.review_vba_onlyoffice_workbook` tool.
- Static ONLYOFFICE preview validation was re-reviewed and remains deferred.

The current owner remains `ontocode-rs/ext/excel`. The current explicit tool surface now includes workbook inspection/export, VBA extraction, analyzer-only review, fail-closed ONLYOFFICE preview emission, the Stage 3 workbook-review wrapper, and source-first VBA-to-M preview under the same extension.

This document now serves as a current-state follow-on ledger: one proposal was accepted narrowly and landed, one was re-reviewed and deferred, and the rest remain deferred without implementation approval.

## Decision Frame

Any follow-on should be judged against these constraints first:

- stay inside `ontocode-rs/ext/excel`
- keep model-visible tools explicit unless a wrapper is clearly justified
- preserve the fail-closed analyzer-first contract
- avoid creating a second Excel orchestration stack
- avoid runtime/network dependencies unless they are already present and safely sandboxed

## Recommended Order

There is no further approved implementation order at this time.

Current state:

1. Stage 3 workbook-assisted flow is done.
2. Static ONLYOFFICE preview validation is deferred after no-dispatch review.
3. Neutral IR / module split is deferred.
4. Broad VBA parser dependency is deferred.
5. Generic `excel.translate` remains rejected as a public surface.
6. Runtime ONLYOFFICE validation is deferred.

## Option Set

### Option A: Narrow Stage 3 Workbook-Assisted Flow

Status: completed narrowly.

Landed explicit tool:

- `excel.review_vba_onlyoffice_workbook`

Inputs:

- local workbook path
- optional module filter

Behavior:

- reuse `excel.extract_vba_modules`
- run `excel.analyze_vba_onlyoffice_migration` per extracted module
- run `excel.translate_vba_to_onlyoffice_js_preview` only for analyzer-approved modules
- return a bounded per-module report with blockers, warnings, and preview macros

Operational note:

- top-level `success` on `excel.review_vba_onlyoffice_workbook` means the workbook review completed and at least one VBA module was reviewed
- it does not mean every reviewed module emitted a non-empty preview macro
- per-module `analysis`, `warnings`, `macro_value`, and `function_body` remain the authority for migration readiness

Approved narrowed contract:

- no public `emit_preview` mode switch
- exact module-name filtering only
- no full source re-exposure beyond the already bounded extractor-owned source slice
- no second orchestration stack or bundle flow

Why this is the best first follow-on:

- reuses existing owners instead of replacing them
- gives users workbook-level convenience without inventing a migration bundle
- preserves the analyzer-first gate and current fail-closed behavior

What it must not do:

- no workbook rewrite
- no generated package/artifact bundle
- no direct ONLYOFFICE macro injection
- no runtime execution

Recommended when:

- users repeatedly have multi-module `.xlsm` workbooks and current manual composition is too slow

### Option B: Static ONLYOFFICE Preview Validator

Status: reviewed and deferred. No implementation dispatch.

Proposed explicit tool:

- `excel.validate_onlyoffice_macro_preview`

Inputs:

- `macro_value`
- optional `function_body`

Behavior:

- verify IIFE wrapper shape
- verify only allowed `Api.*` calls appear
- reject `macrosArray` writes
- reject filesystem/network/process APIs
- enforce output caps and disallow obviously unsafe text

Why it is attractive:

- raises confidence without needing a runtime
- stays offline
- fits the current bounded-read-only philosophy

Why it should come before runtime validation:

- much cheaper to build and maintain
- no dependency on a local ONLYOFFICE runtime harness
- avoids turning validation into an integration-testing project

Recommended when:

- users want a second check before pasting preview JS into ONLYOFFICE

Why it stayed deferred after re-review:

- the current preview translator already fails closed on unsafe analyzer output
- the Stage 3 workbook-review wrapper already emits preview strings only for analyzer-approved modules
- a second public validator would mostly duplicate the existing gate without protecting a new sink

### Option C: Internal Neutral IR Split

Split current analyzer/emitter internals into:

- `vba_onlyoffice_ir.rs`
- `vba_onlyoffice_emit.rs`

Keep tool contracts unchanged.

Behavior:

- analyzer lowers supported statements into a small internal IR
- translator emits from IR rather than from analyzer operation summaries directly

Why it is useful:

- makes future operation expansion easier
- reduces analyzer/emitter contract drift
- creates a cleaner point for static validation and test fixtures

Why it should remain internal:

- public tool JSON does not need to change
- a new user-visible abstraction adds no immediate product value

Recommended when:

- more than a few new operations are being added and direct operation-summary emission becomes brittle

### Option D: Broad VBA Parser Dependency

Replace or supplement the hand-rolled parser with a real VBA grammar/parser dependency.

Possible triggers:

- repeated unsupported syntax cases that are still in-scope product needs
- too many parser-specific bugs in line continuations, declarations, or control flow
- IR work is blocked by parser ambiguity rather than product-scope limits

Why this should stay deferred:

- larger maintenance surface
- dependency and license review required
- easy to over-expand syntax support before the target operation surface is proven

Acceptable narrow form:

- parser used only to improve bounded source understanding
- no expansion of product claims by default
- still fail closed on unsupported semantics

Recommended when:

- there are at least several concrete user samples that cannot be handled safely with the current parser and are still inside approved feature scope

### Option E: Generic `excel.translate` Facade

Expose one top-level tool that routes between:

- VBA -> M preview
- Power Query -> SQL preview
- VBA -> ONLYOFFICE JS preview

Why it is tempting:

- simpler discovery story
- one entry point for “translate this Excel-related source”

Why it is risky:

- recreates the monolith shape already rejected in the ADR
- mixes unrelated translation products with different trust and validation models
- makes future permissions and tool help less explicit

Least-bad form if forced:

- internal router only
- public explicit tools remain primary
- facade is opt-in and returns which explicit tool path it chose

Recommended when:

- only if tool-surface discoverability is proven to be a major user problem and explicit-tool routing is already stable internally

### Option F: Runtime ONLYOFFICE Validation

Execute generated preview macros against a local ONLYOFFICE environment or recorder-compatible runtime.

Why it has the highest confidence ceiling:

- validates more than shape
- can catch emitter/runtime mismatches that static checks miss

Why it is the worst immediate choice:

- biggest maintenance burden
- environment-sensitive
- hard to keep offline and deterministic in CI
- easy to accidentally turn into a second runtime/integration stack

Acceptable narrow form:

- local-only, explicitly experimental
- separate from normal translation tools
- sandboxed
- no network
- no workbook mutation persisted by default

Recommended when:

- a stable repo-local ONLYOFFICE validation harness already exists and static validation still leaves material user risk

## Comparison

| Area | Smallest useful solution | Value | Risk | Recommendation |
| --- | --- | --- | --- | --- |
| Stage 3 workbook-assisted flow | Explicit workbook review/composition tool | High | Medium | Completed |
| Static ONLYOFFICE preview validation | Explicit preview validator | Medium | Medium | Deferred after no-dispatch review |
| Runtime ONLYOFFICE validation | Experimental sandboxed local runner | Medium | High | Defer |
| Generic `excel.translate` | Internal router only | Low | High | Keep rejected publicly |
| Broad VBA parser dependency | Bounded parser upgrade behind ADR | Medium | High | Defer until samples justify |
| Neutral IR/module split | Internal IR + emitter split | Medium | Medium | Do before parser if growth demands it |

## Current Queue

There is no currently approved next implementation task from this ADR.

If this ADR is reopened later, the follow-on candidates should be reconsidered in this order:

1. Recheck whether static preview validation protects a new sink or merely duplicates the current fail-closed translator gate.
2. Recheck whether emitter growth justifies an internal IR/module split.
3. Recheck broad parser work only against several concrete in-scope user samples.

## Recommendation

Current recommendation:

- treat Option A as closed and implemented
- keep Option B deferred unless a new sink or trust boundary appears that the current fail-closed translator does not already protect
- keep the remaining options as deferred notes, not as an active implementation queue

Keep these closed for now:

- public generic `excel.translate`
- runtime ONLYOFFICE validation
- broad VBA parser dependency

Keep this internal-only if reopened:

- neutral IR / module split

## Explicit Non-Recommendations

Do not do any of these as the next step:

- one-shot workbook migration bundle
- runtime execution plus workbook rewrite in one tool
- generic translation facade as the main public surface
- parser dependency plus product-scope expansion in the same change
- large IR refactor before there is concrete pressure from new supported operations
