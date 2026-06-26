---
name: Offline VBA To ONLYOFFICE Deferred Implementation Detailed Project Plan
description: Execution-grade conditional project plan for deferred offline VBA to ONLYOFFICE work after Stage 0-3 closure
type: project_plan
date: 2026-06-25
status: active
---

# Offline VBA To ONLYOFFICE Deferred Implementation Detailed Project Plan

## Goal

Provide a detailed, dispatch-ready project plan for the deferred offline VBA to ONLYOFFICE work without reopening blocked scope by accident.

This plan is execution-grade, but conditional:

- it defines exact phases, task cards, entry criteria, exit criteria, and verification
- it does not approve implementation by itself
- implementation may start only when one concrete trigger is proven under the source ADR and deferred plan
- it is a secondary execution appendix to the deferred implementation plan, not an independent trigger authority

## Source Authority

Primary authority:

- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md)

Supporting authority:

- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_SOLUTIONS.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_SOLUTIONS.md)
- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md)
- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION_TRACKING.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION_TRACKING.md)
- [ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS_TRACKING.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS_TRACKING.md)

Latest queue-state evidence:

- [audit_session-2026-06-25-offline-vba-onlyoffice-formulalocal-c1-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-formulalocal-c1-closure.md)
- [audit_session-2026-06-25-offline-vba-onlyoffice-ten-slot-review.md](audit_session-2026-06-25-offline-vba-onlyoffice-ten-slot-review.md)
- [audit_session-2026-06-25-offline-vba-onlyoffice-manager-loop-no-dispatch-2.md](audit_session-2026-06-25-offline-vba-onlyoffice-manager-loop-no-dispatch-2.md)

## Current State

Completed:

- Stage 0 target contract capture
- Stage 1 analyzer
- Stage 2 fail-closed preview translator
- narrow Stage 3 workbook review wrapper
- one narrow deferred `C1` slice:
  - `.FormulaLocal` accepted as a `.Formula` target variant
  - variable RHS remains fail-closed

Closed with no-dispatch:

- static validator reopen
- emit/module split reopen
- parser dependency reopen
- public `excel.translate`
- runtime validation reopen
- follow-on candidate queue beyond `.FormulaLocal`

Current queue state:

- no active implementation queue
- next valid action is still Slice 0 trigger review only

## Design Constraints

These are binding across all later slices:

- owner stays `ontocode-rs/ext/excel`
- preserve analyzer-first fail-closed behavior
- preserve public tool payloads unless a specific slice explicitly allows change
- do not add a public router
- do not add runtime macro execution
- do not add a parser dependency unless `C2` gates are all satisfied
- do not split IR or emit modules only for neatness
- do not reopen blocked slices because of task-count pressure alone

## Corpus And Trigger Sources

Primary real corpus:

- [EssBaseWF.xlam](/opt/YD/Downloads/Essbase.Danone/EssBaseWF.xlam)
- [Выдача спецодежды_без табельных.xlsm](/opt/YD/Temp/_w1/Выдача%20спецодежды_без%20табельных.xlsm)
- [Заявка на мыло.xlsm](/opt/YD/Temp/_w1/Заявка%20на%20мыло.xlsm)
- [Табель Макрос.xlsm](/opt/YD/Temp/_w1/Табель%20Макрос.xlsm)

Current interpretation:

- `EssBaseWF.xlam` is primarily fail-closed boundary evidence:
  - external `Declare ... Lib`
  - workbook mutation
  - shapes / comments / hyperlinks
  - domain-specific formula rewriting
- `_w1` is primarily event/workbook-semantics evidence:
  - `Workbook_Open`
  - `Worksheet_Change`
  - sheet protection / visibility choreography
- `Табель Макрос.xlsm` remains the best future `C1` trigger source because it contains utility-style helpers
- these workstation-local paths are advisory trigger examples only; durable reopen authority still comes from repo-local redacted extracts, tests, and tracking updates

Redaction rule before any trigger can be accepted:

- remove passwords and pass constants
- remove usernames and `Environ(...)` values
- remove business-specific workbook, sheet, and department names
- keep only the minimal syntax and control-flow shape required to prove the trigger

## Phase Map

### Phase 0: Trigger Intake

Purpose:

- decide whether any deferred work is actually open

Required checks:

1. run OntoIndex freshness check for `codex`
2. read the deferred implementation plan
3. try OntoIndex-backed owner/context lookup for the candidate file or symbol
4. if OntoIndex coverage is partial, stale in practice, or cannot resolve the candidate cleanly, fall back to direct source reads and record that fallback explicitly
5. identify exactly one candidate trigger
6. classify it as:
   - syntax-only
   - semantics-blocked
   - scope-blocked
   - duplicate of a previously closed candidate
7. record the decision before editing code

Exit:

- either one slice is opened
- or a no-dispatch note is written and the loop stops

### Phase 1: Narrow Existing-Owner Slices

Allowed slices:

- `B1` emit module split
- `C1` targeted parser augmentation
- `A2` internal preview validator helper
- `E3` snapshot contract drift checker

Blocked in this phase:

- `C2` parser adapter
- any public surface change
- any runtime execution change

### Phase 2: Escalation Review

Purpose:

- determine whether a blocked area may be promoted

Allowed only for:

- `C2` parser adapter after repeated `C1` failure

Required before promotion:

- proof that at least two or more concrete `C1` attempts are insufficient
- named parser candidate
- maintainer owner named
- license checked
- bounded output proof
- no parser types leaking into public payloads

### Phase 3: Closure And Queue Reset

Purpose:

- close the slice cleanly and reset the queue to no-dispatch until a new trigger exists

Required outputs:

- code verification
- audit note
- tracking update
- memory index update if a new durable document was added

## Active Work Packages

These are the only work packages this plan recognizes.

### WP-0 Trigger Review

Status:

- always available
- currently the only valid starting point

Purpose:

- check whether any deferred slice is truly open

Files to inspect:

- `ontocode-rs/ext/excel/src/extension.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_workbook_review.rs`
- `ontocode-rs/ext/excel/src/tests.rs`
- relevant redacted sample snippet

Acceptance:

- exactly one slice opened, or explicit no-dispatch result

### WP-1 `B1` Emit Module Split

Default:

- closed

Open only if:

- `vba_onlyoffice_translate.rs` gains real operation-family growth
- or emit logic is duplicated across files
- or the emitter file approaches large-file pressure

Allowed files:

- `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_emit.rs`
- `ontocode-rs/ext/excel/src/tests.rs`

Non-scope:

- new IR
- new public JSON
- new semantics

Acceptance:

- translator behavior unchanged
- focused regression only if needed

### WP-2 `C1` Targeted Parser Augmentation

Default:

- closed

Open only if:

- a concrete redacted sample shows a shallow syntax gap in already supported semantics

Current examples:

- completed:
  - `.FormulaLocal`
- still closed:
  - `.FormulaR1C1`
  - `.Value2`
  - `.NumberFormatLocal`
  - `.ColumnWidth`
- semantics-blocked:
  - `.Interior.ColorIndex`
  - `.Font.ColorIndex`
  - `.RowHeight`
  - dynamic formula concatenation
  - shape/control `.Text`
  - workbook `Visible` / `Protect`

Allowed files:

- `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs` only if the analyzer change produces a new existing-family emitted operation
- `ontocode-rs/ext/excel/src/tests.rs`

Acceptance:

- one positive redacted syntax test
- one adjacent fail-closed test
- no broadened claims

### WP-3 `A2` Internal Preview Validator Helper

Default:

- closed

Open only if:

- a second internal sink or emitter duplicates preview-shape checks

Current reason closed:

- workbook review still routes through the translator and is not a second emitter

Allowed files:

- `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_workbook_review.rs`
- `ontocode-rs/ext/excel/src/tests.rs`

Acceptance:

- no new public tool
- duplicated sink fails closed through shared helper

### WP-4 `E3` Snapshot Contract Drift Checker

Default:

- closed

Open only if:

- recorder-contract drift is proven
- or fixture mismatch appears between the pinned contract and generated output

Allowed files:

- `ontocode-rs/ext/excel/src/tests.rs`
- optional fixture docs under `.memory-bank/` if needed

Non-scope:

- adding a new supported `Api.*` operation
- runtime execution
- workbook mutation
- public validator tool

Acceptance:

- deterministic fixture comparison
- donor commit or source path recorded

### WP-5 `C2` Private Parser Adapter

Default:

- blocked

Open only if all gates are satisfied:

- repeated `C1` failure is documented
- parser candidate is named
- maintainer owner is named
- license compatibility is confirmed
- redacted sample corpus proves repeated failure
- parser output remains bounded and fail-closed
- parser types do not leak into public tool payloads

Allowed files:

- `ontocode-rs/ext/excel/src/*`
- `ontocode-rs/Cargo.toml`
- `ontocode-rs/Cargo.lock`
- `MODULE.bazel.lock`

Acceptance:

- all dependency gates passed
- lockfiles updated together
- no new public parser surface

## Detailed Task Cards

### Card T0: Queue Preflight

Owner:

- manager

Steps:

1. run OntoIndex freshness check
2. verify dirty-worktree caveat
3. read deferred plan and latest queue-state note
4. decide whether a real trigger exists

Done when:

- the manager can name one trigger or explicitly say there is none

### Card T1: Candidate Snippet Intake

Owner:

- implementation-worker or manager

Inputs:

- one workbook-derived snippet
- source workbook path
- redaction note

Steps:

1. extract the smallest relevant snippet
2. redact secrets and business data
3. state whether the blocker is syntax or semantics
4. reject the candidate if redaction destroys the evidence

Done when:

- the snippet is safe to cite and clearly classified

### Card T2: Syntax Trigger Proof

Owner:

- senior-reviewer or manager

Purpose:

- prove a candidate is shallow syntax only

Checklist:

- existing operation family already exists
- target root already fits current owner
- no new runtime behavior required
- no workbook/event semantics required
- no public surface expansion implied

Done when:

- candidate is promoted to `C1`
- or rejected as semantics-blocked or scope-blocked

### Card T3: Narrow `C1` Implementation

Owner:

- implementation-worker

Steps:

1. edit the smallest target classifier or parser branch
2. keep unsupported semantics fail-closed
3. add one positive and one negative regression
4. avoid unrelated cleanup

Done when:

- tests pass
- claims are still narrow

### Card T4: `B1` Split Evaluation

Owner:

- manager or senior-reviewer

Purpose:

- decide whether emitter growth justifies a split

Checklist:

- emitter file actually grew, not the analyzer
- new operation family is being added
- split reduces complexity now, not hypothetically

Done when:

- `B1` is opened
- or explicitly closed again

### Card T5: Drift Trigger Proof

Owner:

- manager or verification-worker

Purpose:

- decide whether `E3` is open

Checklist:

- recorder evidence changed
- or generated preview differs from pinned expectation
- or a new approved `Api.*` call family is being added

Done when:

- `E3` is opened
- or explicitly closed again

Note:

- if a new supported `Api.*` operation is deliberately added, that is not an `E3` trigger by itself
- treat that as a separate implementation slice under `C1` or another future approved owner-local code slice, then use `E3` afterward only for drift or fixture verification

### Card T6: Parser Dependency Promotion Review

Owner:

- senior-reviewer

Purpose:

- decide whether `C2` may be opened

Checklist:

- repeated `C1` failures exist
- repeated failures are documented
- dependency candidate and owner are named
- license review exists
- bounded output proof exists

Done when:

- `C2` is still blocked
- or narrowly promoted with explicit dependency scope

## Verification Matrix

### Docs-only queue review

Required:

- OntoIndex freshness check
- OntoIndex owner/context lookup when available
- direct source reads if dirty-worktree noise exists
- explicit note when direct source fallback was required because OntoIndex could not resolve the candidate cleanly

Not required:

- Rust tests

### `C1`, `B1`, `A2`, `E3` code changes

Run from `ontocode-rs/`:

```bash
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
```

Also:

- run OntoIndex diff verification if usable
- if dirty-worktree noise makes it unusable, record that explicitly

### `C2` dependency changes

Required:

```bash
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
just bazel-lock-update
just bazel-lock-check
```

And include together:

- `Cargo.toml`
- `Cargo.lock`
- `MODULE.bazel.lock`

## Stop Conditions

Stop immediately if any of these becomes true:

- no concrete trigger is proven
- the candidate requires workbook/event/runtime semantics
- the candidate implies a new public router or validator
- the candidate requires parser dependency promotion without `C2` gates
- OntoIndex freshness is fine but owner/symbol resolution is incomplete, and direct-source fallback still cannot justify the slice

## Dispatch Order

When a future trigger exists, use this order:

1. `WP-0` trigger review
2. `WP-2` narrow `C1` if syntax-only
3. `WP-1` `B1` only if emitter growth is proven
4. `WP-4` `E3` only if drift evidence is proven
5. `WP-3` `A2` only if a second sink appears
6. `WP-5` `C2` only after repeated `C1` failure

## Closure Routing

Use one canonical closure path per slice family so future runs do not scatter status across unrelated notes.

- `WP-0` trigger review with no code change:
  - write one audit note under `.memory-bank/audit_session-YYYY-MM-DD-*.md`
  - if the queue state changes materially, update [ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md)
- `WP-1` `B1` and `WP-2` `C1` implementation closures:
  - update [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION_TRACKING.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION_TRACKING.md)
  - add one audit note for the specific closure
- `WP-3` `A2` and any later workbook-review-adjacent follow-on:
  - update [ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS_TRACKING.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS_TRACKING.md)
  - add one audit note for the specific closure
- `WP-4` `E3` and `WP-5` `C2` deferred-slice reopen or closure:
  - update [ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md)
  - add one audit note for the specific closure
- any new durable plan, ADR, or audit note:
  - add an index entry in [.memory-bank/MEMORY.md](MEMORY.md)

## Current Recommended Action

No active implementation dispatch.

The next valid action is still:

- perform Slice 0 trigger review against fresh local evidence
- stop if no trigger is proven

The most credible future reopen remains:

- one redacted utility-style VBA snippet, preferably from `Табель Макрос.xlsm` or a similar non-event helper, that proves a new shallow syntax gap inside already supported semantics

## Senior-Opened Readiness Tasks

These tasks are intentionally evidence-first. They do not reopen implementation scope by themselves, but they can create legitimate future Phase 0 triggers.

### SRT-1 Redacted Snippet Ledger

Status:

- closed 2026-06-25

Owner:

- senior-reviewer or manager

Purpose:

- build a compact ledger of redacted candidate snippets from the current corpus so future `C1` reviews do not need to re-extract the same workbook evidence

Inputs:

- `tmp/vba-samples/tabell.vba`
- `tmp/vba-samples/essbase.vba`
- `tmp/vba-samples/mylo.vba`

Expected output:

- one small `.memory-bank` note listing:
  - snippet label
  - workbook source
  - redaction status
  - syntax-only vs semantics-blocked classification
  - current result: open candidate, blocked candidate, or duplicate of closed candidate

Acceptance:

- no secrets remain
- no product scope claims are broadened
- every snippet is classified, not just collected

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-readiness-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-readiness-task-closure.md)
- result: current corpus is now ledgered and classified; no new reopen trigger was proven

### SRT-2 Supported-Versus-Corpus Matrix

Status:

- closed 2026-06-25

Owner:

- senior-reviewer

Purpose:

- compare the current analyzer/translator operation family against the real local corpus so the next reopen is based on an actual gap map rather than ad hoc examples

Inputs:

- current supported operation set in `vba_onlyoffice_analyze.rs` and `vba_onlyoffice_translate.rs`
- local corpus examples from `tabell.vba`, `essbase.vba`, and `mylo.vba`

Expected output:

- one bounded matrix with rows:
  - already supported
  - syntax-only gap
  - semantics-blocked
  - out of scope

Acceptance:

- each candidate family appears in exactly one bucket
- `E3`, `A2`, `B1`, and `C2` are not reopened by inference alone

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-readiness-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-readiness-task-closure.md)
- result: the matrix shows one already-supported family, no syntax-only reopen, and the rest still semantics-blocked or out of scope

### SRT-3 Fresh Corpus Intake Gate

Status:

- closed 2026-06-25

Owner:

- manager

Purpose:

- define the exact gate for accepting new local workbook samples into the deferred queue so future reopens are not based on raw file-count expansion

Required checklist:

- workbook path recorded
- why it is different from the existing four-sample set
- whether it adds utility-style helpers, recorder-shape cell formatting, or only more workbook/event semantics
- redaction feasibility confirmed
- first expected slice named before review begins

Acceptance:

- a future sample can be rejected quickly when it only repeats existing semantics-blocked evidence

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-readiness-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-readiness-task-closure.md)
- result: intake now requires a named first slice and a material-difference claim before review starts

### SRT-4 Recorder Drift Watch

Status:

- closed 2026-06-25

Owner:

- verification-worker or manager

Purpose:

- make `E3` cheaper to prove later by defining exactly what recorder-contract evidence would count as real drift

Expected output:

- one short note enumerating:
  - pinned ONLYOFFICE source path
  - supported `Api.*` calls currently relied on
  - which shape differences would count as drift
  - which differences are harmless formatting noise

Acceptance:

- this stays documentation or fixture-planning only
- no runtime execution or public validator scope is introduced

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-readiness-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-readiness-task-closure.md)
- result: recorder drift is now defined against the pinned `sdkjs/common/macro-recorder.js` contract and not against runtime execution

### SRT-5 OntoIndex Coverage Audit For `ext/excel`

Status:

- closed 2026-06-25

Owner:

- manager

Purpose:

- determine whether a future OntoIndex refresh would materially improve owner/symbol resolution for the new ONLYOFFICE Excel files, or whether direct-source fallback should remain the default

Expected output:

- one small note stating:
  - which `ext/excel` files are visible cleanly now
  - which are not
  - whether `ontoindex analyze` would be worth scheduling before the next real reopen

Acceptance:

- no refresh is run casually
- the outcome is a decision, not an automatic reindex

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-readiness-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-readiness-task-closure.md)
- result: current ONLYOFFICE Excel files still need direct-source fallback; no refresh was scheduled from this pass

## Senior-Opened Follow-Up Tasks

These tasks are still no-dispatch by default. They exist to make future trigger review cheaper and less repetitive.

### SFT-1 Shallow Syntax Family Presence Check

Status:

- closed 2026-06-25

Owner:

- senior-reviewer or manager

Purpose:

- determine whether the still-closed shallow syntax families actually occur in the current real corpus, instead of keeping them as abstract placeholders

Target families:

- `.Value2`
- `.FormulaR1C1`
- `.NumberFormatLocal`
- `.ColumnWidth`

Inputs:

- `EssBaseWF.xlam`
- `_w1` workbook set
- existing extracted VBA samples where available

Expected output:

- one short note saying, for each family:
  - present with a redacted snippet
  - absent from the current corpus
  - present only inside broader semantics-blocked logic

Acceptance:

- each family lands in exactly one bucket
- no `C1` reopen is inferred from mere presence alone

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-follow-up-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-follow-up-task-closure.md)
- result: `.Value2`, `.FormulaR1C1`, `.NumberFormatLocal`, and `.ColumnWidth` are absent from the current extracted corpus

### SFT-2 Utility-Style Fixture Pack From `Табель Макрос`

Status:

- closed 2026-06-25

Owner:

- senior-reviewer

Purpose:

- prepare a tiny redacted fixture pack from the strongest utility-style workbook so the next `C1` review can cite stable evidence without reopening the full workbook extraction loop

Inputs:

- [Табель Макрос.xlsm](/opt/YD/Temp/_w1/Табель%20Макрос.xlsm)
- `tmp/vba-samples/tabell.vba`

Expected output:

- two to five redacted snippets, each tagged with:
  - candidate family
  - syntax-only or semantics-blocked
  - duplicate or novel

Acceptance:

- no secrets or business labels remain
- snippets stay minimal and review-ready

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-follow-up-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-follow-up-task-closure.md)
- result: the fixture pack is useful for future review, but it adds no new syntax-only reopen

### SFT-3 Semantics-Blocked Boundary Ledger

Status:

- closed 2026-06-25

Owner:

- manager

Purpose:

- stop repeated reopen pressure on families that still require new semantics rather than shallow syntax handling

Seed families:

- `.Font.ColorIndex`
- `.Interior.ColorIndex`
- `.RowHeight`
- dynamic formula concatenation
- shape/text frame writes
- `Visible` and `Protect`

Expected output:

- one compact ledger with, for each family:
  - why current owners cannot absorb it as `C1`
  - what extra semantic model or runtime assumption would be required
  - which deferred slice it would point at, if any

Acceptance:

- blocked families are challenged, not just listed
- the result reduces future no-dispatch repetition

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-follow-up-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-follow-up-task-closure.md)
- result: the recurrent families remain blocked for semantic reasons, not for shallow parser reasons

### SFT-4 Direct-Source Owner Map For ONLYOFFICE Excel Flow

Status:

- closed 2026-06-25

Owner:

- manager

Purpose:

- offset partial OntoIndex coverage by documenting the minimal direct-source review path across `ext/excel` for future deferred-loop work

Inputs:

- `ontocode-rs/ext/excel/src/extension.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`
- `ontocode-rs/ext/excel/src/vba_onlyoffice_workbook_review.rs`
- `ontocode-rs/ext/excel/src/tests.rs`

Expected output:

- one short owner map naming:
  - entrypoint
  - analyzer owner
  - emitter owner
  - workbook composition owner
  - canonical regression area

Acceptance:

- future review can start from this map instead of rediscovering file ownership
- no architecture expansion is proposed

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-follow-up-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-follow-up-task-closure.md)
- result: the source review path across entrypoint, analyzer, emitter, workbook wrapper, and tests is now explicit

### SFT-5 Minimum `C1` Reopen Proof Pack

Status:

- closed 2026-06-25

Owner:

- senior-reviewer or manager

Purpose:

- define the smallest complete proof package required before any future narrow syntax reopen is accepted

Expected output:

- one checklist requiring:
  - redacted snippet
  - existing operation-family match
  - no new semantic owner
  - one positive test target
  - one adjacent fail-closed target
  - explicit non-scope statement

Acceptance:

- the checklist is short enough to use every time
- future `C1` review quality improves without adding process bloat

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-follow-up-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-follow-up-task-closure.md)
- result: future narrow reopen review now has one minimal, repeatable proof pack

## Senior-Opened Next Tasks

These tasks stay evidence-only. They exist because the last loop hit one practical gap: the strongest workbook evidence still lives outside workspace-local extraction scope.

### SNT-1 Workspace-Local Corpus Staging Gate

Status:

- closed 2026-06-25

Owner:

- manager

Purpose:

- define the minimum safe staging rule for bringing selected real workbook evidence into workspace-local scope so the existing Excel extraction tools can read it directly

Inputs:

- `/opt/YD/Temp/_w1/Табель Макрос.xlsm`
- `/opt/YD/Downloads/Essbase.Danone/EssBaseWF.xlam`
- current `tmp/vba-samples/*.vba` cache

Expected output:

- one short note stating:
  - what may be copied into workspace-local scope
  - what must be redacted before or after extraction
  - whether staged artifacts should be workbook files, extracted module text, or both

Acceptance:

- no raw secrets are preserved in workspace-local copies
- the staging rule is small enough to reuse without a new process layer

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-next-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-next-task-closure.md)
- result: `tmp/vba-samples/*.vba` is now the preferred review surface and no extra raw workbook copies are needed when extracted text already exists

### SNT-2 Real-Workbook Placeholder Recheck

Status:

- closed 2026-06-25

Owner:

- senior-reviewer or manager

Purpose:

- rerun the placeholder shallow-syntax family check against real workbook-derived extracts after `SNT-1` defines the staging path

Target families:

- `.Value2`
- `.FormulaR1C1`
- `.NumberFormatLocal`
- `.ColumnWidth`

Expected output:

- one short note saying, for each family:
  - confirmed absent in real workbook extracts
  - present with a redacted snippet
  - present only inside broader semantics-blocked logic

Acceptance:

- this supersedes the sample-cache-only result when better evidence becomes available
- no `C1` reopen is inferred from presence alone

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-next-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-next-task-closure.md)
- result: `.Value2`, `.FormulaR1C1`, `.NumberFormatLocal`, and `.ColumnWidth` remain absent in workspace-local workbook-derived extracts

### SNT-3 Tiny Trigger Scoreboard

Status:

- closed 2026-06-25

Owner:

- manager

Purpose:

- keep a one-screen scoreboard of the candidate families that still matter so future loops stop reopening already-defeated questions

Expected output:

- one compact list covering:
  - family
  - current status: supported, absent, semantics-blocked, or out of scope
  - last evidence source
  - next action if status changes

Acceptance:

- fits in one small note
- reduces future manager-loop repetition

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-next-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-next-task-closure.md)
- result: a one-screen candidate-family scoreboard now captures supported, absent, semantics-blocked, and out-of-scope statuses

## Senior-Opened Micro Tasks

These stay evidence-only. They exist to remove the last remaining formatting and intake ambiguity before the next real Slice 0 trigger appears.

### SMT-1 Provenance Header Normalization

Status:

- closed 2026-06-25

Owner:

- manager

Purpose:

- define the canonical header shape for staged `tmp/vba-samples/*.vba` extracts so future evidence notes cite workbook provenance consistently

Inputs:

- `tmp/vba-samples/tabell.vba`
- `tmp/vba-samples/essbase.vba`
- `tmp/vba-samples/mylo.vba`

Expected output:

- one short note defining:
  - canonical `FILE:` line shape
  - whether module name should be recorded
  - whether extraction date should be recorded

Acceptance:

- future extracts can be checked quickly for provenance completeness
- no new tool or parser behavior is implied

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-micro-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-micro-task-closure.md)
- result: the required provenance field is the existing `FILE:` line and no extraction-date field is needed inside the extract

### SMT-2 Redaction Placeholder Lexicon

Status:

- closed 2026-06-25

Owner:

- senior-reviewer or manager

Purpose:

- standardize the placeholder tokens used when promoting workbook snippets into ADR or audit notes

Expected output:

- one compact lexicon for placeholders such as:
  - passwords
  - usernames
  - workbook names
  - sheet names
  - server or URL literals
  - business labels

Acceptance:

- future notes redact the same way every time
- the placeholders remain readable for technical review

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-micro-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-micro-task-closure.md)
- result: a compact placeholder lexicon now covers passwords, users, workbook or sheet names, hosts or URLs, and business labels

### SMT-3 Slice 0 Trigger Card Template

Status:

- closed 2026-06-25

Owner:

- manager

Purpose:

- define a one-card template for the next real trigger review so future manager loops stop reassembling the same decision scaffold

Expected output:

- one short template containing:
  - candidate family
  - redacted snippet
  - source provenance
  - existing operation-family match or no-match
  - status: supported, absent, semantics-blocked, or out of scope
  - reopen recommendation yes or no

Acceptance:

- the card fits in one screen
- it is enough to accept or reject a future trigger without extra process overhead

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-micro-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-micro-task-closure.md)
- result: the next Slice 0 review can use one short trigger-card template instead of rebuilding the decision scaffold

## Senior-Opened Gate Tasks

These stay evidence-only. They exist to make future "still blocked or now reopenable?" decisions crisp for the three recurring families that keep returning.

### SGT-1 Palette-Index Reopen Gate

Status:

- opened 2026-06-25
- closed 2026-06-25

Owner:

- senior-reviewer or manager

Purpose:

- define the exact evidence required before `.Font.ColorIndex` or `.Interior.ColorIndex` could ever move from semantics-blocked into a real reopen candidate

Expected output:

- one short gate note stating:
  - what deterministic palette mapping proof would be required
  - what recorder or target-contract evidence would be acceptable
  - what would still keep the family blocked

Acceptance:

- future loops can reject palette-index reopen requests quickly when the mapping proof is missing
- no color-mapping implementation is proposed

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-gate-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-gate-task-closure.md)
- result: palette-index requests remain semantics-blocked until deterministic palette-to-ONLYOFFICE mapping proof exists

### SGT-2 Row-Dimension Reopen Gate

Status:

- opened 2026-06-25
- closed 2026-06-25

Owner:

- manager

Purpose:

- define the exact evidence required before `.RowHeight` could ever move from semantics-blocked into a real reopen candidate

Expected output:

- one short gate note stating:
  - what ONLYOFFICE target-contract proof would be required
  - whether row dimension belongs in the current recorder subset or a later contract
  - what keeps the family blocked today

Acceptance:

- future loops can reject row-dimension reopen requests quickly when contract proof is missing
- no new emit family is proposed

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-gate-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-gate-task-closure.md)
- result: `.RowHeight` remains blocked until recorder-grounded row-dimension target proof exists

### SGT-3 Dynamic-Formula Reopen Gate

Status:

- opened 2026-06-25
- closed 2026-06-25

Owner:

- senior-reviewer or manager

Purpose:

- define the exact evidence required before dynamic formula concatenation could ever move from semantics-blocked into a real reopen candidate

Expected output:

- one short gate note stating:
  - what bounded expression-rewrite proof would be required
  - what classes of dynamic formula remain out of scope
  - what would distinguish a narrow accepted case from a broad formula engine

Acceptance:

- future loops can reject dynamic-formula reopen pressure quickly when the proof is absent
- no evaluator or formula IR work is proposed

Closure note:

- closed in [audit_session-2026-06-25-offline-vba-onlyoffice-gate-task-closure.md](audit_session-2026-06-25-offline-vba-onlyoffice-gate-task-closure.md)
- result: dynamic-formula concatenation remains blocked until a bounded expression-rewrite proof exists that does not imply a general formula engine
