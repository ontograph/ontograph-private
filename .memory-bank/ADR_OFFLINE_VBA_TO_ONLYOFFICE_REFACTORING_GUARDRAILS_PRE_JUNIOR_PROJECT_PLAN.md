---
name: Offline VBA To ONLYOFFICE Refactoring Guardrails Pre-Junior Project Plan
description: Junior-safe, evidence-first plan for testing whether any refactor under the ONLYOFFICE VBA guardrails is actually justified
type: project_plan
date: 2026-06-25
status: closed-no-dispatch
---

# Offline VBA To ONLYOFFICE Refactoring Guardrails Pre-Junior Project Plan

## Goal

Use [ADR_OFFLINE_VBA_TO_ONLYOFFICE_REFACTORING_GUARDRAILS.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_REFACTORING_GUARDRAILS.md) as a strict filter for future refactor pressure in `ontocode-rs/ext/excel`.

This is not a feature plan and not a permission slip to "clean up" the ONLYOFFICE VBA code.

A pre-junior may only:

- prove one candidate refactor is justified by current bounded evidence
- prove one candidate refactor is not justified and should stay closed
- prepare one senior-ready handoff packet for one concrete reopen class

No broad Rust rewrite, architecture reshape, runtime expansion, or public-surface change is allowed from this plan.

Current queue state:

- no dispatchable open tasks remain
- this plan stays dormant until one exact input packet is supplied under the entry criteria below

## Source Authority

Primary authority:

- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_REFACTORING_GUARDRAILS.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_REFACTORING_GUARDRAILS.md)

Required supporting authority:

- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md)
- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md)
- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md)

Current shipped-owner files:

- `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_workbook_review.rs`
- `ontocode-rs/ext/excel/src/tests.rs`

## Challenge Review

- This plan is a brake, not a refactor queue.
- One patch may examine one candidate only.
- If current evidence says "do nothing", that is a valid completion outcome.
- A pre-junior may not widen supported semantics, create a second translation owner, or relax fail-closed behavior.
- If OntoIndex coverage is partial for the newer `vba_onlyoffice_*` files, direct source reads remain the authority after the freshness check.
- If the candidate needs a new public tool, runtime harness, parser dependency, generic router, or workbook mutation flow, stop immediately. That is outside this plan.

## Allowed Work

- One bounded evidence pack for one candidate refactor.
- One docs-only drift repair that brings ADR wording back in line with current source.
- One focused test-only proof that preserves existing semantics and only strengthens current contract coverage.
- One owner-local helper extraction only after the exact `B1` pressure is proven.
- One syntax-only candidate packet only after the exact `C1` pressure is proven.

## Non-Goals

- Do not add `excel.validate_onlyoffice_macro_preview`.
- Do not add `excel.translate`.
- Do not add runtime ONLYOFFICE execution or replay validation.
- Do not add a broad VBA parser dependency.
- Do not split IR or emitter layers for neatness.
- Do not treat workbook review as a second migration pipeline.
- Do not change workbook-review `success` meaning.
- Do not widen bounds, caps, truncation, or redaction behavior casually.

## Dispatch Rule

Pre-junior dispatch limit:

- one candidate
- one owner area
- one reopen class
- one patch

Never dispatch a whole stage to one worker.

## Entry Criteria

Before any pre-junior starts work, the manager or senior reviewer must supply:

- one exact candidate
- one exact owner file
- one exact reopen class
- one short reason this is being checked now

If any of those are missing, the pre-junior must not infer them.

Required input packet:

```md
Candidate:
Owner file:
Reopen class:
Why this is being checked now:
Known blocked families ruled out:
```

## Deliverables

Every pre-junior patch must end with one of these deliverables only:

- one no-dispatch note with `not justified`
- one docs-only drift repair
- one test-only proof patch
- one senior handoff packet

Not an allowed deliverable:

- a broad cleanup branch
- a multi-owner refactor
- a reopen proposal disguised as helper extraction
- a semantics change paired with a "while here" note

## Work Packages

Use these as the only valid package shapes.

### Package `PJ-0`: Candidate Intake

Scope:

- verify the input packet is complete
- reject the task if the candidate is vague or spans more than one reopen class

Acceptance:

- candidate is singular
- owner file is singular
- reopen class is singular
- blocked-family confusion is ruled out

### Package `PJ-1`: Evidence Pack

Scope:

- produce Stage 1 packet only
- no code edits

Acceptance:

- evidence packet is complete
- verdict is explicit
- next step is either stop, docs-only, test-only, or senior handoff

### Package `PJ-2`: Docs Drift Repair

Scope:

- wording-only correction to ADR/tracking material
- no Rust edits

Acceptance:

- docs now describe shipped behavior exactly
- no broadened future claim appears

### Package `PJ-3`: Test Contract Proof

Scope:

- one focused test proving existing behavior
- no production behavior change

Acceptance:

- test proves an existing supported or fail-closed contract edge
- no new product claim was introduced

### Package `PJ-4`: Senior Handoff

Scope:

- stop before structural code movement
- package the argument and exact decision needed

Acceptance:

- handoff names the trigger
- handoff names the blocking ambiguity
- handoff requests one precise senior decision

## Stage 0: Preflight

Read:

- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_REFACTORING_GUARDRAILS.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_REFACTORING_GUARDRAILS.md)
- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md)
- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md)
- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md)

Checks:

1. Run OntoIndex freshness check for `codex`.
2. Record whether OntoIndex coverage for the relevant ONLYOFFICE file or symbol is usable or partial.
3. Confirm that the manager- or senior-supplied candidate matches exactly one accepted pressure class:
   - operation-growth pressure
   - duplicated emitter logic
   - large-file pressure
   - recorder-contract drift
   - one concrete redacted syntax-only gap in already supported semantics
4. Record the exact owner file and exact candidate from the input packet.

Acceptance:

- no code changed
- one manager- or senior-supplied candidate confirmed
- one manager- or senior-supplied owner confirmed
- freshness result recorded
- partial-coverage fallback recorded when needed
- no self-opened candidate inference occurred

Suggested commands:

```sh
cd /opt/demodb/_workfolder/ontocode
rg -n "vba_onlyoffice_|review_vba_onlyoffice_workbook|translate_vba_to_onlyoffice_js_preview|analyze_vba_onlyoffice_migration" ontocode-rs/ext/excel/src
```

Preferred OntoIndex checks:

```text
gn_ensure_fresh(repo="codex")
gn_explain_module(filePath="ontocode-rs/ext/excel/src/<owner>.rs")
```

If the file is not in index:

- record `OntoIndex coverage: partial`
- switch to direct source reads
- do not invent missing graph evidence

## Stage 1: Evidence Pack

Purpose:

Build the smallest possible bounded evidence pack for the candidate.

Required pack:

- one owner-symbol neighborhood check
- one cluster or process check when OntoIndex can supply it
- one bounded read-first file set
- one direct source confirmation
- one verdict:
  - `not justified`
  - `docs-only`
  - `test-only`
  - `B1 candidate`
  - `C1 candidate`
  - `E3 candidate`

Acceptance:

- pack stays under one short section
- evidence is bounded
- the verdict is explicit
- no production edit is proposed yet

Packet format:

```md
Candidate:
Owner:
OntoIndex freshness:
OntoIndex coverage:
Graph evidence:
Read-first files:
Direct source confirmation:
Verdict:
Why this is not broader scope:
```

Evidence quality rules:

- keep each field to one short paragraph or a few bullets
- cite only files actually read
- if graph evidence is unavailable, say so directly
- do not fill missing evidence with architectural opinion

## Stage 2: Non-Signal Rejection

Purpose:

Force the implementer to reject fake pressure before any code movement.

The candidate must be rejected here if the evidence is only:

- "the file is long"
- "the future architecture would look cleaner"
- "the workbook corpus is larger now"
- "the wrapper has several callsites"
- "the graph tool output is interesting"

Acceptance:

- if any non-signal is the main argument, stop with `not justified`
- if rejection is correct, produce a no-dispatch note and stop

Required rejection note:

```md
Result: not justified
Candidate:
Owner:
Main rejection reason:
Which non-signal was mistaken for pressure:
What future evidence would be required to reopen:
```

## Stage 3: Docs-Only Drift Path

Use only when:

- the candidate is purely wording drift between ADRs, tracking, and shipped behavior
- no product contract changes are needed

Allowed files:

- `.memory-bank/ADR_OFFLINE_VBA_TO_ONLYOFFICE_*.md`
- `.memory-bank/*TRACKING*.md`
- one new audit note if needed

Acceptance:

- wording matches current source
- no Rust files changed
- no new feature is implied by the docs

Checklist:

- confirm current source wording first
- remove stale future-tense statements presented as shipped behavior
- remove stale shipped claims presented as future work
- keep deferred-plan authority unchanged

## Stage 4: Test-Only Contract Proof

Use only when:

- the behavior is already implemented
- the gap is proof, not semantics

Allowed files:

- `ontocode-rs/ext/excel/src/tests.rs`

Not allowed:

- behavior widening
- new public payload fields
- emitter semantics changes

Acceptance:

- one focused positive or fail-closed test
- existing contract becomes better proven
- no product behavior drifts

Preferred proof targets:

- warning-bearing analysis emits no preview
- redacted analysis emits no preview
- workbook review keeps `success` as review-completion only
- a Stage 0 supported call remains positively covered
- an adjacent unsupported variant still fails closed

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
```

## Stage 5: `B1` Emit Helper Candidate

Use only when:

- operation-growth pressure is real
- or emitter logic is duplicated
- or large-file pressure is present together with one more real signal

Allowed files:

- `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`
- `ontocode-rs/ext/excel/src/tests.rs`

Not allowed:

- new IR
- public payload changes
- semantics changes

Pre-junior rule:

- do not apply the refactor unless the evidence pack proves one exact `B1` trigger
- if the trigger is arguable, stop and write a senior handoff instead
- do not create a new helper module or second file unless a senior decision explicitly authorizes that exact extraction target after the `B1` trigger is proven

Acceptance:

- behavior unchanged
- helper extraction only
- tests unchanged except for one focused regression if required

Mandatory rejection triggers:

- evidence relies only on file length
- extraction would introduce a new public or semi-public owner
- extraction starts to resemble IR separation
- helper name or module implies a second translation pipeline
- the proposed extraction target is a new file that has not been explicitly approved by senior review

## Stage 6: `C1` Syntax-Only Candidate

Use only when:

- there is one concrete redacted snippet
- the snippet stays inside already supported semantics
- the blocker is syntax, not runtime meaning

Allowed files:

- `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs` only if an existing operation family needs matching emitter support
- `ontocode-rs/ext/excel/src/tests.rs`

Not allowed:

- parser dependency
- new control-flow execution semantics
- workbook-state behavior
- event semantics
- shape/control flows

Acceptance:

- one positive redacted sample
- one adjacent fail-closed sample
- no broadened product claim

Mandatory source packet:

```md
Redacted snippet:
Already supported semantic family:
Why this is syntax-only:
Why this is not control flow or runtime meaning:
Adjacent fail-closed variant:
```

## Stage 7: `E3` Drift-Check Candidate

Use only when:

- the Stage 0 ONLYOFFICE contract is deliberately refreshed
- or one supported `Api.*` shape changes
- or a fixture mismatch is proven

Allowed files:

- test fixtures
- test-only drift checks
- stage-0 contract docs if refreshed deliberately

Not allowed:

- runtime execution
- workbook mutation
- public runtime validation tool

Acceptance:

- deterministic snapshot or fixture comparison
- source pin recorded
- drift proof stays test-only

Drift note must include:

- which recorder pin or contract source was used
- which supported `Api.*` shape moved
- whether the change is docs-only, fixture-only, or test-only

## Blocked Candidate Families

These stay blocked from this plan:

- public validator
- generic translate facade
- runtime execution or replay
- parser dependency
- workbook mutation or bundle flow
- workbook-review semantics changes
- IR split done only for cleanliness

If a candidate lands in one of these families, stop and write:

```md
Result: blocked
Blocked family:
Why it is blocked under the guardrails ADR:
What evidence would be needed for a senior reopen:
```

## Verification Rules

For any Rust edit:

1. keep the change inside the exact allowed file set
2. run `just fmt`
3. run `just test -p ontocode-excel-extension`
4. do not run broader workspace tests without senior approval

For docs-only work:

- run `git diff --check`

For any pre-junior closure:

- include the exact files read
- include the exact files changed
- state whether OntoIndex coverage was usable or partial
- state whether the outcome was `not justified`, `docs-only`, `test-only`, or `handoff`

## Senior Handoff Packet

When the pre-junior should stop:

- trigger exists but the refactor is still arguable
- OntoIndex coverage is too partial to support a structural conclusion
- the candidate starts to resemble a blocked family
- the change would alter public semantics

Handoff format:

```md
Candidate:
Owner:
Reopen class:
Evidence pack summary:
Why a pre-junior should stop here:
Exact senior decision needed:
```

Good senior questions:

- Is this really `B1`, or only a file-length complaint?
- Does this snippet stay inside already supported semantics, or is it actually a parser/runtime reopen?
- Would this extraction preserve single-owner behavior, or create a second pipeline by naming and structure?
- Is the claimed workbook-review pressure real, or should the wrapper remain untouched?

## Closure Checklist

- one candidate only
- one owner only
- one reopen class only
- no blocked family slipped in
- fail-closed behavior preserved
- bounded caps preserved
- workbook-review `success` meaning preserved

## Current Best Default

Default outcome from this plan is still:

- `not justified`

The pre-junior must prove why a refactor should happen. The refactor does not happen by default.

## Example Closures

### Good closure

```md
Result: test-only
Candidate: quoted vertical alignment support proof
Owner: ontocode-rs/ext/excel/src/tests.rs
Why allowed: behavior already ships; proof gap only
Files changed: ontocode-rs/ext/excel/src/tests.rs
OntoIndex coverage: partial
```

### Good closure

```md
Result: not justified
Candidate: split translator into IR and emitter modules
Owner: ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs
Main rejection reason: no concrete B1 trigger, only cleaner-looking architecture
OntoIndex coverage: partial
```

### Bad closure

```md
Result: helper extraction
Why: file felt large and future architecture would be easier later
```

Bad because:

- no concrete trigger class
- no bounded evidence pack
- no proof that single-owner pressure exists
- architecture preference was treated as evidence
