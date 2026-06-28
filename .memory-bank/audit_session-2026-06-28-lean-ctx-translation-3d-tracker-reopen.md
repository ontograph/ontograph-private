# Lean-ctx Translation 3D Tracker Reopen

Date: 2026-06-28
Scope: `ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL_DETAILED_PROJECT_PLAN_TRACKING.md`

## Decision

Reopened `L3D-P1` after challenging the tracker against the accepted Phase 1
exit criteria and the current repo evidence.

## Why The Prior Closeout Was Too Strong

- The repo proves that the package files landed:
  - `plugins/ontocode-lean-ctx/.codex-plugin/plugin.json`
  - `plugins/ontocode-lean-ctx/.mcp.json`
  - `.agents/plugins/marketplace.json`
- The accepted plan still requires proof that the package:
  - loads through the existing plugin loader
  - fails closed on missing or mismatched daemon settings
- Current source evidence shows the existing loader and install path can support
  that proof, but the proof itself is not yet recorded in the repo.

## Reopened Task

- `L3D-P1` stays the active task.
- The next bounded move is to prove install/load through existing plugin flows
  and add the smallest possible fail-closed compatibility evidence.

## Non-Goals

- Do not open Phase 2 `/v1` client work yet.
- Do not add a second runtime, registry, or adapter-specific config family.
