# ADR: Qwen Donor Remaining Blockers Solutions

Date: 2026-06-20
Status: accepted-for-minimal-dispatch

## Context

`tmp/qwen-code-donor-dispatch-tracking.md` is closed, but ten Qwen rows remain
blocked or rejected after the first ADR pass:

- `QWN-005`, `QWN-007`, `QWN-009`
- `QWN-015`, `QWN-020`, `QWN-025`, `QWN-027`
- `QWN-107`, `QWN-164`, `QWN-165`

`QWN-036` and `QWN-127` are already covered and must not be redispatched unless
new failing evidence appears.

## Decision

Reopen only the ten remaining rows below. Workers must start from existing
owners, add the smallest failing/passing regression, and stop if the required
fix crosses the boundary listed here.

| Rows | Accepted solution | Owner boundary |
| --- | --- | --- |
| `QWN-005`, `QWN-007`, `QWN-009` | Add internal-only reason text for disabled/hidden tools and search classification in existing spec-plan/tool-search paths. | No public schema, no new registry, no app-server/TUI API. |
| `QWN-015`, `QWN-020`, `QWN-025`, `QWN-027` | Add per-turn in-memory read evidence and generated-file warning heuristics in existing edit/apply-patch/guardian owners. | No SQLite, no durable evidence table, no protocol change. |
| `QWN-107` | Store only the existing bounded `final_summary` field. A migration is allowed only to repair the already-referenced column. | No transcript store, no raw output persistence. |
| `QWN-164` | Add a bounded operational-evidence context fragment only if it follows `core/context` fragment rules and hard caps. | No raw blob ingestion, no artifact classifier outside operational evidence. |
| `QWN-165` | Add a small provider context-window classifier at the existing provider/client boundary and reuse it from compaction. | No new provider trait taxonomy, no protocol-wide error rewrite. |

## Rejected

- SQLite in-memory tracking for read evidence. A per-turn Rust map/set is enough.
- Full transcript storage for sub-agents.
- Public tool metadata/search APIs.
- Native HTTP hooks.
- Raw artifact injection into model context.

## Dispatch

Use these disjoint implementation slices:

| Task | Rows | Write scope |
| --- | --- | --- |
| `R1` | `QWN-005`, `QWN-007`, `QWN-009` | `ontocode-rs/core/src/tools/spec_plan.rs`, `ontocode-rs/tools/src/tool_search.rs`, focused tests only. |
| `R2` | `QWN-015`, `QWN-020`, `QWN-025`, `QWN-027` | Existing edit/apply-patch/guardian modules and tests only. |
| `R3` | `QWN-107` | `ontocode-rs/state/migrations`, existing agent-job runtime/tests. |
| `R4` | `QWN-164` | `ontocode-rs/core/src/context`, operational-evidence query/import callers, focused context tests. |
| `R5` | `QWN-165` | Provider/client context-window matching and compaction tests only. |

## Verification

- Update `tmp/qwen-code-donor-dispatch-tracking.md` before each dispatch.
- Run OntoIndex impact before editing Rust symbols.
- Run scoped tests for changed crates.
- Run `CARGO_BUILD_JOBS=8 just fmt` after Rust edits.
- Refresh/check OntoIndex after each accepted task.
