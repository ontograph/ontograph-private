# Offline VBA To ONLYOFFICE Follow-On Solutions Tracking

## Scope

Bounded manager loop for [ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md).

Current authority:

- Stage 0 target-contract capture is complete.
- Stage 1 analyzer is complete.
- Stage 2 fail-closed preview translator is complete.
- The only dispatchable follow-on is Option A as a narrow read-only workbook review tool under `ontocode-rs/ext/excel`.
- Static validator, generic `excel.translate`, broad parser dependency, neutral IR split, and runtime validation remain deferred.

## Tasks

| ID | Task | Role | Status | Notes |
| --- | --- | --- | --- | --- |
| OO-VBA-F3-SR1 | Challenge follow-on scope against current `ext/excel` owners and Stage 0-2 guardrails | senior-reviewer | completed | Fallback `gpt-5.4-mini` approved a thin workbook-review wrapper, rejected a public `emit_preview` mode switch, and required analyzer-gated preview output only for safe modules. |
| OO-VBA-F3-I1 | Implement `excel.review_vba_onlyoffice_workbook` as a read-only composition tool | implementation-worker | completed | Added the explicit workbook-review tool under `ontocode-rs/ext/excel`, reusing extraction, analyzer review, and analyzer-gated preview emission without widening scope. |
| OO-VBA-F3-V1 | Verify Stage 3 narrow workbook review slice | verification-worker | completed | Fallback `gpt-5.4-mini` PASS. Confirmed the slice stayed read-only and fail-closed, with no runtime validation, bundle generation, or generic router expansion. |
| OO-VBA-AUD3 | Clarify workbook-review success semantics and current-state wording after audit challenge | senior-reviewer | completed | Docs-only clarification accepted. Follow-on ADR wording now states that top-level workbook-review success means review completed, not that every module emitted a preview. No product-behavior change was approved. |
| OO-VBA-AUD4 | Challenge deferred-plan duplication and workstation-local sample authority in the ONLYOFFICE planning set | senior-reviewer | completed | Docs-only simplification accepted. The detailed plan remains a secondary execution appendix, while the deferred plan stays the sole trigger authority; workstation-local samples are treated as advisory trigger examples, not durable primary authority. |

## Manager Log

- 2026-06-24: Manager reopened the follow-on ADR as a bounded loop. OntoIndex `gn_ensure_fresh` reported the `codex` index fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2` with a dirty-worktree caveat (`dirtyFileCount=242`, `scopeConfidence=medium`).
- 2026-06-24: OntoIndex impact check confirmed low-risk owner edits for `Function:ontocode-rs/ext/excel/src/extension.rs:install` and low-risk upstream fan-in for `extract_vba_modules_from_workbook` limited to its tool handle and focused tests.
- 2026-06-24: Senior-reviewer fallback approved only one promotable follow-on: `excel.review_vba_onlyoffice_workbook` as a thin read-only wrapper with exact module-name filtering and analyzer-gated preview fields, while keeping runtime validation, generic routing, parser expansion, IR split, and workbook mutation deferred.
- 2026-06-24: `OO-VBA-F3-I1` completed. The Excel extension now exposes `excel.review_vba_onlyoffice_workbook`, a cwd-scoped workbook-review tool that returns bounded per-module analyzer results plus preview strings only for analyzer-approved modules.
- 2026-06-24: Validation: `just fmt` passed; `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension` passed 44/44; `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension` passed. OntoIndex `gn_verify_diff` PASS and `gn_test_gap` PASS on the intended file set.
- 2026-06-24: Verification-worker PASS. The narrow Stage 3 slice stayed read-only and fail-closed, with no workbook mutation, runtime validation, bundle generation, or generic `excel.translate` routing.
- 2026-06-24: Next follow-on recheck returned no-dispatch for Option B. Senior-reviewer kept `excel.validate_onlyoffice_macro_preview` deferred because the current preview translator already fails closed on unsafe analyzer output, and a second public validator would mostly duplicate the existing gate without protecting a new sink.
- 2026-06-25: Bounded audit manager loop reopened two follow-on review tasks only. `OO-VBA-AUD3` captures the workbook-review success-semantics challenge from the audit, and `OO-VBA-AUD4` captures the docs-only challenge to collapse duplicated deferred planning and demote workstation-local samples from primary authority where possible. No implementation dispatch was approved in this loop.
- 2026-06-25: Continued bounded manager loop closed both follow-on audit tasks as docs-only. Workbook-review success semantics are now spelled out in the follow-on ADR, and the planning set now treats the deferred implementation plan as the sole trigger authority while demoting the detailed plan to an execution appendix and workstation-local workbook paths to advisory evidence only.
