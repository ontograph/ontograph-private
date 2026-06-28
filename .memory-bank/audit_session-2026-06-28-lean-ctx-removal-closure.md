# Lean-ctx Removal Closure

Date: 2026-06-28
Tasks: `L3D-R1`, `L3D-R2`, `L3D-R3`
Status: complete

## Landed changes

- deleted repo-local plugin package `plugins/ontocode-lean-ctx/`
- deleted repo-local plugin marketplace file `.agents/plugins/marketplace.json`
- removed the proof-only lean-ctx install/load test from
  `ontocode-rs/core-plugins/src/manager_tests.rs`
- deleted `LEAN-CTX.md`
- removed the lean-ctx preference block from `GEMINI.md`
- rewrote `.memory-bank/reference_agent-rules.md` to use OntoIndex plus native
  shell and `rg`

## Validation

- `just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core-plugins`
- file-scoped `git diff --check`

OntoIndex note:

- repo index was fresh
- repo-wide `gn_verify_diff` remained noisy in this dirty worktree, so closure
  used file-scoped verification and recorded the limitation

## Closure basis

No surviving runtime dependency on external lean-ctx remains in the repo-local
plugin path or current agent guidance. No additional internal replacement slice
is justified because accepted owners already cover the surviving behavior.
