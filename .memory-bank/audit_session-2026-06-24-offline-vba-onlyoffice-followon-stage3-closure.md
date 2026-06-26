# Offline VBA To ONLYOFFICE Follow-On Stage 3 Closure

Date: 2026-06-24

## Scope

Close the only approved follow-on from [ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_FOLLOWON_SOLUTIONS.md): a narrow read-only workbook-assisted review tool under `ontocode-rs/ext/excel`.

## Decision

Accepted.

The implemented slice is `excel.review_vba_onlyoffice_workbook` only.

It stays inside the existing Excel extension owner and composes the existing VBA extraction, ONLYOFFICE analyzer, and fail-closed preview translator surfaces without introducing a new orchestration stack.

## What Landed

- New direct-model tool: `excel.review_vba_onlyoffice_workbook`
- Input contract: cwd-scoped local workbook `path` plus optional exact `module_names`
- Output contract: bounded per-module analyzer review, module warnings, and preview `macro_value` / `function_body` only for analyzer-approved modules
- Guardrails: no workbook mutation, no generated bundle/artifact, no runtime validation, no generic `excel.translate` router

## Validation

- OntoIndex `gn_ensure_fresh`: fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2` with dirty-worktree caveat
- OntoIndex impact: LOW for `Function:ontocode-rs/ext/excel/src/extension.rs:install`
- OntoIndex impact: LOW for `Function:ontocode-rs/ext/excel/src/vba_extract.rs:extract_vba_modules_from_workbook`
- `just fmt`: PASS
- `CARGO_BUILD_JOBS=8 just test -p ontocode-excel-extension`: PASS (44/44)
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-excel-extension`: PASS
- OntoIndex `gn_verify_diff`: PASS on the intended file set
- OntoIndex `gn_test_gap`: PASS on the intended file set
- Verification-worker fallback `gpt-5.4-mini`: PASS

## Residual Scope

Still deferred:

- static ONLYOFFICE preview validator
- internal neutral IR split
- broad VBA parser dependency
- public generic `excel.translate`
- runtime ONLYOFFICE validation
