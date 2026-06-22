# Audit: Lean-ctx Core Backbone Challenge

Date: 2026-06-08

## Event

Reviewed and challenged `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md` against the goal of moving useful lean-ctx-inspired workflow support into Ontocode's core backbone.

## Decision

- Accepted only a narrow Ontocode Operational Backbone contract for core: task cards, evidence records, gate results, and readiness summaries.
- Kept lean-ctx as an external workflow utility; no vendoring, runtime dependency, read/search/shell runtime, session cache, or tool-discovery copy is approved.
- Kept Stage 0 repository scripts as bootstrap/reporting helpers only.
- Required B0 data-contract stage before any Rust implementation, and separate ADR/stage cards for migrations, model-visible context, app-server APIs, or runtime behavior.

## Evidence

- GitNexus context confirmed existing `ContextualUserFragment` ownership for future bounded model-visible summaries.
- Source maps confirmed existing `StateRuntime`, `MemoryStore`, and local agent-graph state boundaries for durable records.
- GitNexus verify-diff was attempted and failed because the worktree already contains many unrelated tracked changes while these memory-bank files are untracked.

## Follow-Up

- Next valid core task: add a B0 task card defining data contracts and validation tests only.
- Next valid repository-script task: implement the three Stage 0 bootstrap commands if still needed.
