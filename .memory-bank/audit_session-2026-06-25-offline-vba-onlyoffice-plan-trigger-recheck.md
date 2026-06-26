# Offline VBA To ONLYOFFICE Plan Trigger Recheck

Date: 2026-06-25

## Scope

Bounded manager loop over `.memory-bank/ADR_OFFLINE_VBA_TO_ONLYOFFICE_DEFERRED_IMPLEMENTATION_PLAN.md` using OntoIndex plus direct `ext/excel` source reads.

## Roles

- manager: current session
- senior-reviewer: not dispatched; this pass stayed inside Slice 0 trigger review
- implementation-worker: not dispatched because no slice opened
- verification-worker: handled by manager locally because this was a docs-only trigger gate

## Evidence

- OntoIndex `gn_ensure_fresh` reported `codex` fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2` with `dirtyFileCount=249` and `scopeConfidence=medium`.
- OntoIndex could not explain the new ONLYOFFICE Excel files because they are still outside the current index view, so final gating used direct source reads.
- `ontocode-rs/ext/excel/src/vba_onlyoffice_analyze.rs` is large at 1086 lines, but that is analyzer pressure, not the emit-path growth trigger required for `B1`.
- `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs` remains a single bounded fail-closed emitter at 308 lines and does not yet justify an emit split.
- `ontocode-rs/ext/excel/src/vba_onlyoffice_workbook_review.rs` remains a wrapper over extract -> analyze -> translate, so it is not a second sink or independent emitter.
- `ontocode-rs/ext/excel/src/tests.rs` contains fail-closed and workbook-review coverage, but no newly isolated redacted syntax fixture proving a `C1` gap.

## Decision

No dispatchable tasks.

## Per-Slice Result

- `A2` internal validator: closed; no second preview sink or emitter exists
- `B1` emit module split: closed; current pressure is analyzer-side, not emitter-side
- `C1` targeted parser augmentation: closed; no redacted utility-style blocked syntax sample was presented
- `C2` private parser adapter: blocked; `C1` has not failed repeatedly and dependency gates remain unsatisfied
- `E3` snapshot contract drift checker: closed; no recorder-contract drift, new supported `Api.*` operation, or fixture mismatch was proven

## Follow-Up

The next valid manager action is still Slice 0 only.

If the queue is reopened, the first acceptable trigger should be one concrete redacted utility-style VBA snippet, preferably from `Табель Макрос.xlsm` or a similar non-event helper, that fails only because of a shallow syntax gap inside the current supported semantics.
