# Offline VBA To ONLYOFFICE Follow-On ADR Repair

Date: 2026-06-25

## Scope

Docs-only repair of [ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md) so it matches the current implemented Stage 3 state and the later no-dispatch review of Option B.

## Decision

Accepted as docs drift repair only.

## What Changed

- marked Option A as completed rather than proposal-only
- replaced the stale “Stage 3 not approved” wording with the landed narrow `excel.review_vba_onlyoffice_workbook` state
- removed the rejected public `emit_preview` contract from Option A
- recorded that Option B was re-reviewed and kept deferred with no implementation dispatch
- replaced the stale recommended implementation order with a “no approved next task” current-state queue

## Evidence

- tracking authority: [ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS_TRACKING.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS_TRACKING.md)
- Stage 3 closure: [audit_session-2026-06-24-offline-vba-onlyoffice-followon-stage3-closure.md](audit_session-2026-06-24-offline-vba-onlyoffice-followon-stage3-closure.md)
- current Excel owner surface: `ontocode-rs/ext/excel/src/extension.rs`
- current workbook-review contract: `ontocode-rs/ext/excel/src/vba_onlyoffice_workbook_review.rs`
- current fail-closed preview translator: `ontocode-rs/ext/excel/src/vba_onlyoffice_translate.rs`

## Validation

- OntoIndex `gn_ensure_fresh`: fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2` with dirty-worktree caveat
- Docs-only change; no Rust/tool-surface changes were made
