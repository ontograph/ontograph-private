# Lean-ctx Shutdown Pivot

Date: 2026-06-28
Scope: `ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL.md` and related plan/tracking files

## Decision

Replanned the lean-ctx project around dependency removal because the external
lean-ctx project is expected to shut down.

## What Changed

- The old plugin-only adapter ADR is now historical and superseded.
- The detailed plan now targets dependency inventory, runtime removal, and
  reuse of existing internal owners.
- Tracking is reopened with `L3D-R0` as the active next task.

## Replacement Direction

- `ontocode-core-plugins`: cleanup and removal of the repo-local plugin path
- accepted Native Context Tools owners: internal shell/read work only where a
  real post-removal need remains
- OntoIndex: semantic/code search
- bounded evidence/state owners: workflow/task/readiness facts

## Guardrail

Do not keep external lean-ctx as a required runtime dependency, and do not
respond to the shutdown by cloning lean-ctx runtime architecture inside
Ontocode.
