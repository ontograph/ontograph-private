# Lean-ctx Plugin Backend Runtime Proof

Date: 2026-06-28
Task: `L3D-F5`
Plan: `ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL_DETAILED_PROJECT_PLAN.md`

## Decision

Close `L3D-F5`.

The repo now owns a direct start/prove path for the maintained-fork backend
used by `plugins/ontocode-lean-ctx`, without teaching the plugin manager to
spawn background processes.

## Landed Change

- added `scripts/run_lean_ctx_plugin_backend.sh`
- added `scripts/smoke_lean_ctx_plugin_backend.sh`
- added `just` targets:
  - `lean-ctx-plugin-backend-build`
  - `lean-ctx-plugin-backend-run`
  - `lean-ctx-plugin-backend-start`
  - `lean-ctx-plugin-backend-status`
  - `lean-ctx-plugin-backend-stop`
  - `lean-ctx-plugin-backend-smoke`
- updated plugin guidance to use the repo-owned runtime path

## Runtime Contract

- backend source: `third_party/lean-ctx-fork`
- runtime binary: `third_party/lean-ctx-fork/rust/target/release/lean-ctx`
- host default: `127.0.0.1`
- port default: `7777`
- token env var: `LEANCTX_TOKEN`
- default local token fallback: `ontocode-lean-ctx-dev`
- enforced profile: `LEAN_CTX_TOOL_PROFILE=ontocode`

## Validation

- `just fmt`
- `just lean-ctx-plugin-backend-smoke`
- `git diff --check -- justfile scripts/run_lean_ctx_plugin_backend.sh scripts/smoke_lean_ctx_plugin_backend.sh plugins/ontocode-lean-ctx/README.md LEAN-CTX.md .memory-bank/ADR_LEAN_CTX_TRANSLATION_3D_PROPOSAL_DETAILED_PROJECT_PLAN_TRACKING.md .memory-bank/project_plan-current.md .memory-bank/audit_session-2026-06-28-lean-ctx-plugin-backend-runtime-proof.md .memory-bank/MEMORY.md`

## Note

The earlier broad `cargo test tool_profiles` filter remains non-blocking and
was not reopened in this slice. The exact profile-scoped test path already
proved the carried surface, while this slice proves the actual in-repo backend
runtime path.
