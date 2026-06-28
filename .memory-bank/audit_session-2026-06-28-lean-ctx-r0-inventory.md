# Lean-ctx Shutdown Inventory

Date: 2026-06-28
Task: `L3D-R0`
Status: done

## Evidence

- `plugins/ontocode-lean-ctx/.codex-plugin/plugin.json`
- `plugins/ontocode-lean-ctx/.mcp.json`
- `plugins/ontocode-lean-ctx/README.md`
- `.agents/plugins/marketplace.json`
- `ontocode-rs/core-plugins/src/manager_tests.rs`
- `LEAN-CTX.md`
- `GEMINI.md`
- `.memory-bank/reference_agent-rules.md`

OntoIndex status:

- repo index fresh at current HEAD
- dirty worktree, so graph results were used for routing only
- embeddings unavailable; lexical/direct source inspection used as the
  authoritative inventory source

## Classification

Remove:

- repo-local plugin package `plugins/ontocode-lean-ctx/`
- repo-local marketplace entry in `.agents/plugins/marketplace.json`
- plugin proof test in `ontocode-rs/core-plugins/src/manager_tests.rs`

Replace with existing owners:

- `LEAN-CTX.md` mandatory tool routing with OntoIndex plus native `rg` and
  accepted Native Context Tools guidance
- `GEMINI.md` lean-ctx preference block with current OntoIndex/native rules
- `.memory-bank/reference_agent-rules.md` lean-ctx shell/read/search directives
  with current OntoIndex/native rules

Keep as historical evidence only:

- lean-ctx ADRs and audit notes that already say lean-ctx must stay external,
  optional, or rejected as an Ontocode runtime dependency
- donor-reference documents that support historical architecture decisions

## Outcome

`L3D-R1` is the next valid task. No broader replacement slice is justified yet.
