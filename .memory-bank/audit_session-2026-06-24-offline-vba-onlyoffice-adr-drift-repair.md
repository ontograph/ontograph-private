# Offline VBA To ONLYOFFICE ADR Drift Repair

Date: 2026-06-24

## Scope

Docs-only manager-loop repair for [ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md](ADR_OFFLINE_VBA_TO_ONLYOFFICE_MACRO_TRANSLATION.md).

## Outcome

- Reconciled ADR wording with the current tracker authority: Stage 0 target contract, Stage 1 analyzer, and Stage 2 preview translator are complete.
- Kept Stage 3 workbook-assisted flow explicitly not approved.
- Preserved the accepted Stage 2 contract as fail-closed, analyzer-gated, preview-only, and limited to the explicit mapping table.
- Did not add code or broaden scope into workbook bundling, runtime ONLYOFFICE execution, generic `excel.translate`, broad parser dependency, or full VBA compatibility.

## Verification

- OntoIndex `gn_ensure_fresh` reported `codex` fresh at `2e72a6d25e147f0619863e7721107b6f11a87fc2`.
- Confidence remains medium because the worktree has unrelated dirty files.
- Docs-only change; no Rust tests were required for this repair.
