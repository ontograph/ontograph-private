# Audit Session: Excel ADR Drift Closure

## Date

2026-06-23

## Scope

Close the post-implementation ADR audit for [ADR_EXCEL_AGENT_TOOLS.md](ADR_EXCEL_AGENT_TOOLS.md) without widening the Excel feature surface.

The bounded repair scope was markdown truthfulness only:

- stale module-layout text
- overstated Stage 3 handoff verification claim
- metadata sections that claimed more than the current marker-only implementation

## What changed

- updated the crate/module layout to include `preview.rs` and `export.rs`
- changed Stage 3 wording from “verify explicit handoff into `spawn_agents_on_csv`” to “handoff-ready for explicit use”
- narrowed PowerQuery and VBA first-slice text to marker counts and part-path samples only
- narrowed comments and objects text to current marker inventory behavior

## Verification

- OntoIndex review/challenge against the current Excel extension code:
  - [ontocode-rs/ext/excel/src/backend.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/backend.rs:122)
  - [ontocode-rs/ext/excel/src/export.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/export.rs:28)
  - [ontocode-rs/ext/excel/src/tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/tests.rs:564)
  - [ontocode-rs/ext/excel/src/tests.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/ext/excel/src/tests.rs:905)
- `ontoindex analyze --skills --skip-agents-md`
- `gn_ensure_fresh` confirms the index is current at HEAD; dirty worktree remains expected
- `gn_verify_diff` matched the expected changed files, but changed-symbol output for markdown headings was treated as non-authoritative noise

## Decision

The ADR now matches the implemented Excel surface more accurately.

No code change was required for this audit closure. A real end-to-end handoff verification into `spawn_agents_on_csv` still requires a new cross-surface integration test if that stronger claim is desired later.
