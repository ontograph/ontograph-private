# Offline VBA To ONLYOFFICE Deferred Implementation Plan

Date: 2026-06-25

Related ADR: `.memory-bank/ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_SOLUTIONS.md`

Authority note:

- this file is the sole trigger matrix and reopen authority for deferred ONLYOFFICE VBA work
- the detailed project plan is an execution appendix, not a second trigger authority

## Status

Conditional plan only. No implementation is approved until a listed reopen trigger is proven from current code, user samples, or runtime evidence.

The current best action remains no-dispatch because the ADR says the existing Stage 3 workbook-review flow satisfies none of the reopen triggers.

Current sample note:

- the local real-world add-in [EssBaseWF.xlam](/opt/YD/Downloads/Essbase.Danone/EssBaseWF.xlam) adds useful benchmark examples, but it does not by itself open any deferred slice
- most of its pressure is external `Declare ... Lib "HsAddin"` interop, workbook UI/state mutation, and domain-specific formula rewriting rather than a shallow syntax gap
- treat it as corpus evidence for fail-closed boundaries and future fixture selection, not automatic approval for parser, runtime, or router expansion
- the local `_w1` workbook set:
  - [Выдача спецодежды_без табельных.xlsm](/opt/YD/Temp/_w1/Выдача%20спецодежды_без%20табельных.xlsm)
  - [Заявка на мыло.xlsm](/opt/YD/Temp/_w1/Заявка%20на%20мыло.xlsm)
  - [Табель Макрос.xlsm](/opt/YD/Temp/_w1/Табель%20Макрос.xlsm)
- `_w1` is useful because it adds workbook events, sheet protection/visibility flows, and range/comment helpers without external DLL interop
- `_w1` also does not by itself satisfy any reopen trigger; most of its pressure is event semantics and workbook behavior, not a proven shallow parser gap
- workstation-local workbook paths in this plan are advisory trigger examples only; repo-local redacted extracts, tests, and tracking files remain the durable authority

## Goal

Provide a ready implementation sequence for the deferred ONLYOFFICE VBA macro work without reopening broad parser, runtime, router, or validation scope by accident.

## Non-Goals

- Do not add public `excel.translate`.
- Do not add a runtime macro executor.
- Do not add a parser dependency without the ADR dependency gates.
- Do not split IR or emitter modules only for neatness.
- Do not add a public validator while the only sink is the existing translator/reviewer flow.

## Preconditions

Before any worker dispatch:

1. Run OntoIndex freshness check for `codex`.
2. Read the ADR and this plan.
3. Identify exactly one satisfied trigger.
4. Record the trigger evidence in the tracking or audit note before editing.
5. Keep the write set inside `ontocode-rs/ext/excel` unless the trigger explicitly requires test fixture data.
6. For Slice 5 only, dependency metadata changes may also touch `ontocode-rs/Cargo.toml`, `ontocode-rs/Cargo.lock`, and `MODULE.bazel.lock`.

## Trigger Matrix

| Area | Default | Trigger Required | First Allowed Slice |
| --- | --- | --- | --- |
| Static validator | closed | second independent preview sink or emitter | `A2` internal helper |
| IR/module split | closed | concrete operation-growth, duplicated emission logic, or large-file pressure | `B1` emit module split |
| Parser work | closed | redacted in-scope samples blocked by shallow syntax gaps | `C1` targeted augmentation |
| Parser dependency | blocked | repeated `C1` failure plus all dependency gates | `C2` private adapter |
| Public `excel.translate` | rejected | persistent discovery evidence that explicit tools are insufficient | none until new ADR approval |
| Runtime validation | blocked | pinned recorder-contract drift evidence, new supported `Api.*` operation, or fixture mismatch | `E3` snapshot drift checker |

Interpretation for `EssBaseWF.xlam`:

- external add-in declarations are not `C1` evidence because semantics, not parsing, are the blocker
- workbook copy / comments / shapes / hyperlinks are not `A2` evidence unless they create a second preview emitter or duplicate preview-shape checks
- domain-specific `HsGetValue` formula splitting could become `C1` evidence only if a concrete supported source-first translation case is blocked by shallow syntax rather than by unsupported product scope

Interpretation for `_w1` workbooks:

- `Workbook_Open`, `Worksheet_Change`, and `Worksheet_SelectionChange` are not parser triggers by themselves; they are mostly unsupported execution context
- sheet protection / visibility automation is useful fixture evidence for fail-closed classification, not proof that runtime validation should reopen
- utility modules like `FindEndRowRange`, `FindEndColumnRange`, and `AddComment` could become `C1` evidence only if a supported narrow source-first translation path is blocked by syntax rather than by workbook/event semantics

Preferred future trigger sources:

- first `C1` candidates should come from small utility-style modules such as:
  - `Табель Макрос.xlsm` range/comment/string helpers
  - bounded non-event procedures from `EssBaseWF.xlam`
- avoid using these as first trigger sources unless product scope changes:
  - `Declare ... Lib "HsAddin"` interop from `EssBaseWF.xlam`
  - `Workbook_Open` / `Worksheet_Change` event bodies from `_w1`
  - sheet protection / visibility orchestration from `_w1`

Fixture intake rule:

- before any trigger review uses workbook-derived snippets, redact:
  - passwords and pass constants
  - usernames / `Environ(...)` reads
  - business-specific workbook, sheet, and department names
- preserve only the minimal syntax and control-flow shape needed to prove the trigger
- if redaction removes the very thing under analysis, the snippet is not ready for trigger use yet

## Implementation Slices

### Slice 0: Trigger Review

Purpose: decide whether any deferred task is genuinely open.

Steps:

1. Check current Excel tool registration in `ontocode-rs/ext/excel/src/extension.rs`.
2. Check ONLYOFFICE preview flow in `vba_onlyoffice_translate.rs`.
3. Check workbook composition flow in `vba_onlyoffice_workbook_review.rs`.
4. Check tests in `ontocode-rs/ext/excel/src/tests.rs` for an existing failing or missing case.
5. If no trigger is proven, stop and write a no-dispatch audit note.

Exit criteria:

- either one concrete slice below is opened, or no-dispatch is recorded

### Slice 1: `B1` Emit Module Split

Use only when operation growth creates concrete maintenance pressure.

Valid triggers:

- a new supported operation family is being added
- emission logic is duplicated across files
- `vba_onlyoffice_translate.rs` approaches the repo large-file threshold for high-touch modules

Scope:

- add `ontocode-rs/ext/excel/src/vba_onlyoffice_emit.rs`
- move line-emission and preview-assembly helpers only
- keep analyzer output and public tool payloads unchanged

Do not:

- add a new IR type
- change supported operation semantics
- change public result JSON

Tests:

- existing ONLYOFFICE translator tests must pass unchanged
- add one focused regression only if the move exposes a missed edge case

### Slice 2: `C1` Targeted Parser Augmentation

Use only when concrete redacted samples show shallow syntax gaps in current supported scope.

Scope:

- add the smallest parser support for one construct family
- keep unsupported semantics fail-closed
- preserve redaction and truncation behavior

Allowed first construct families:

- line continuations
- declaration variants
- selected control-flow forms already in product scope

Do not:

- broaden product claims
- add a parser dependency
- add control-flow execution semantics without a separate ADR
- accept executable VBA that cannot be mapped safely

Tests:

- one positive redacted sample for the new syntax
- one fail-closed sample for unsupported semantics adjacent to that syntax

### Slice 3: `A2` Internal Preview Validator Helper

Use only after a second internal preview sink or emitter duplicates preview-shape checks.

Scope:

- extract shared internal preview-shape validation
- keep public tools unchanged
- keep workbook review as a caller of the translator while it remains a wrapper

Do not:

- add `excel.validate_onlyoffice_macro_preview`
- validate arbitrary user-supplied JavaScript as a new public product surface

Tests:

- translator still emits known-good preview
- duplicated sink fails closed through the shared helper

### Slice 4: `E3` Snapshot Contract Drift Checker

Use only when there is concrete recorder-contract drift evidence and runtime execution is still too heavy.

Valid triggers:

- a supported `Api.*` operation is added or changed
- recorder evidence from pinned `sdkjs` fixtures no longer matches generated preview output
- the Stage 0 ONLYOFFICE contract artifact is deliberately refreshed

Scope:

- add recorder-derived fixture snapshots for generated preview output
- compare generated output against known-good `sdkjs` recorder shapes
- keep this as test-only drift detection

Do not:

- execute macros
- mutate workbooks
- add a public runtime validation tool

Tests:

- deterministic fixture comparison
- fixture docs identify the ONLYOFFICE donor commit or source path used

### Slice 5: `C2` Private Parser Adapter

Use only after `C1` repeatedly fails and all dependency gates are met.

Required gates:

- parser candidate named
- maintainer owner named
- license compatibility confirmed
- redacted sample corpus proves repeated `C1` failure
- output is bounded and fail-closed
- parser types do not leak into public tool payloads

Do not:

- add a parser sidecar first
- add dependency types to API schemas
- broaden supported VBA semantics without tests

### Blocked Slices

These are not implementation tasks under this plan:

- `A1` public validator: blocked until a new user-visible sink exists
- `B2` internal IR: blocked until `B1` is insufficient
- `C3` parser sidecar: effectively rejected unless dependency isolation becomes a hard constraint
- `D2` internal router: blocked until current code has duplicate selection logic
- `D3` public `excel.translate`: rejected without new ADR approval
- `E1` runtime tool: blocked until a stable repo-local harness exists
- `E2` runtime replay tests: maintainer-only and blocked until a stable local harness exists

## Worker Dispatch Template

Use this shape for any future implementation worker:

```text
Task: implement <slice id> from ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md.

Trigger evidence:
- <file/sample/test proving trigger>

Scope:
- allowed files: <exact files>
- non-scope: public router, runtime executor, parser dependency except for Slice 5 after all C2 gates are satisfied, unrelated Excel tools

Requirements:
- preserve analyzer-first fail-closed behavior
- preserve public tool payloads unless this slice explicitly allows otherwise
- add the smallest focused tests for the trigger
- do not revert unrelated dirty-worktree changes

Verification:
- cd ontocode-rs
- CARGO_BUILD_JOBS=8 just fmt
- CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension
```

## Verification Rules

For docs-only trigger review:

- use OntoIndex freshness plus direct source reads when the worktree is dirty
- no Rust tests required

For code changes in `ontocode-rs/ext/excel`:

- run `CARGO_BUILD_JOBS=8 just fmt`
- run `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`
- run OntoIndex diff verification or record why dirty-worktree noise prevents a clean result

For dependency changes:

- allowed only for Slice 5 after all C2 gates are satisfied
- run `just bazel-lock-update`
- run `just bazel-lock-check`
- include `Cargo.lock`, `Cargo.toml`, and `MODULE.bazel.lock` updates together

## Current Queue

No active implementation queue.

Next valid action is Slice 0 trigger review only. If Slice 0 finds no trigger, stop.

Latest recheck:

- 2026-06-25 manager recheck kept the queue closed again
- OntoIndex stayed fresh at commit `2e72a6d25e147f0619863e7721107b6f11a87fc2`, but the worktree remained dirty and the new `ext/excel` ONLYOFFICE files were still unindexed/untracked, so direct source reads were used for the final gate
- `A2` remains closed because `vba_onlyoffice_workbook_review.rs` still routes through the existing translator and does not create a second preview sink or emitter
- `B1` remains closed because the growth pressure is in `vba_onlyoffice_analyze.rs`, while the `B1` trigger is specifically an emit-path split and `vba_onlyoffice_translate.rs` remains small enough to leave inline
- `E3` remains closed because no recorder-contract drift, no new supported `Api.*` operation, and no fixture mismatch were proven
- the only credible next reopen is still `C1`, but only after a redacted utility-style workbook snippet proves a shallow syntax gap inside currently supported semantics

Latest implementation closure:

- 2026-06-25 reopened `C1` for one narrow syntax variant only
- trigger evidence came from `tmp/vba-samples/tabell.vba`, which contains `Cells(r, erc).FormulaLocal = er` inside the real `AddComment` helper from `Табель Макрос.xlsm`
- the accepted slice was narrower than the workbook sample itself: add `.FormulaLocal` as the same target family as `.Formula`, while keeping variable RHS fail-closed
- this keeps the local workbook sample blocked on semantics, but removes the target-property syntax gap for literal formula assignments in the same family
- scoped verification passed: `CARGO_BUILD_JOBS=8 just fmt` and `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` with `50` tests passing
- queue is closed again after this slice; next valid reopen is still a fresh Slice 0 review

Latest ten-slot review:

- 2026-06-25 manager opened ten candidate review slots and closed them against current triggers instead of inventing ten implementation tasks
- accepted:
  - `.FormulaLocal` target variant under `C1`, already implemented above
- closed as no-dispatch because no local corpus trigger was proven:
  - `.FormulaR1C1`
  - `.Value2`
  - `.NumberFormatLocal`
  - `.ColumnWidth`
- closed as semantics-blocked rather than syntax-blocked:
  - `.Interior.ColorIndex`
  - `.Font.ColorIndex`
  - `.RowHeight`
  - dynamic formula concatenation such as `"=" & vName & "!A1"`
  - shape/control `.Text` assignment
  - workbook `Visible` / `Protect` choreography
