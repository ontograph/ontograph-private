---
name: GitNexus Code-Graph Adoption Tracking
description: Dispatch and verification ledger for the operational evidence backbone project
type: tracking
date: 2026-06-16
status: active
---

# GitNexus Code-Graph Adoption Tracking

Authority:

- [ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md](ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md)
- [GITNEXUS_CODE_GRAPH_ADOPTION_PRE_JUNIOR_PROJECT_PLAN.md](GITNEXUS_CODE_GRAPH_ADOPTION_PRE_JUNIOR_PROJECT_PLAN.md)

Manager rules:

- Update this file before starting each stage.
- Use only `gpt-5.3-codex-spark` or `gpt-5.4-mini` for sub-agents.
- Run OntoIndex before each code stage and refresh OntoIndex after each accepted stage.
- Verify worker changes locally before accepting a stage.
- Do not dispatch `S10`; it is blocked pending a future ADR.

Current OntoIndex baseline:

- `gn_ensure_fresh` on 2026-06-16 reported index fresh at `73ba3040e201390b3b6b0bc05f7d8d33e9c215b6`.
- Dirty worktree already existed before implementation dispatch.

## Stage Ledger

| Order | Stage | Status | Assigned model | Worker | Verification | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | S0 baseline and impact | done | `gpt-5.4-mini` | `019ecfcc-1a26-7f31-8614-320a52eb17d8` | accepted | Next migration is `0036`; manager verified current tail is `0035_drop_memory_tables.sql`. `list_thread_spawn_descendants` is DANGEROUS/public API, so do not modify it in S1. |
| 1 | S1 state migration | done | `gpt-5.4-mini` | `019ecfce-9142-7980-887e-11394cc4d949` | accepted | Added `0036_operational_evidence_records.sql`; manager verified migration shape and `just test -p ontocode-state` passed 134 tests. |
| 2 | S2 model types | done | `gpt-5.4-mini` | `019ecfd1-729b-71c3-be45-e996abdeb64e` | accepted | Added model types, sibling enum tests, crate-root exports; manager changed `risk` to nullable to match SQL and removed temporary suppressions. `just test -p ontocode-state` passed 138 tests. |
| 3 | S3 state runtime methods | done | `gpt-5.4-mini` | `019ecfdd-0f63-7670-8959-4ae24b1f2999` | accepted | Added insert/upsert/query/prune with caps and tests; manager fixed test imports. `just test -p ontocode-state` passed 146 tests. |
| 4 | S4 secret redaction and dependency guards | done | `gpt-5.4-mini` | `019ecfeb-2246-78e1-847b-9c5f646a9c82` | accepted | Added obvious secret rejection before persistence plus manifest dependency guards for runtime/core/app-server/SDK surfaces. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 148 tests. |
| 5 | S5 runtime topology evidence | done | `gpt-5.4-mini` | `019ecff2-c226-7a30-b6bf-19c5b0b87168` | accepted | Added runtime-topology evidence ingestion from existing spawn edges. Manager fixed mixed-status descendant traversal. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 152 tests. |
| 6 | S6 internal query helpers | done | `gpt-5.4-mini` | `019ed001-5699-7031-a13a-d6c55ce43e31` | accepted | No code change. Existing bounded `OperationalEvidenceQuery` and tests already cover the required helper shapes. Worker ran fmt, `just test -p ontocode-state` with 152 tests passing, and path-scoped `git diff --check`. |
| 7 | S7 artifact importer | done | `gpt-5.4-mini` | `019ed005-86a4-7ac1-8051-e12c1055b88b` | accepted | Added explicit Rust-only state artifact importer. Manager strengthened valid-JSON raw source/diff/graph-dump rejection. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 158 tests. |
| 8 | S8 workflow evidence import | done | `gpt-5.4-mini` | `019ed016-e03f-77f1-95d9-163c8836df4b` | accepted | Extended the existing importer for bounded workflow record kinds without lean-ctx runtime. Manager corrected domain coverage for workflow/test/doc/redaction summaries. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 160 tests. |
| 9 | S9 planned-versus-done gates | done | `gpt-5.4-mini` | `019ed023-1da0-7590-810c-e23eb9f5f5b3` | accepted | Added state-owned planned-versus-done gate evaluator. Manager fixed no-code bypass policy. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 167 tests. |
| 10 | S10 context fragment review | blocked | none | none | none | Future ADR required. |

## Running Log

- 2026-06-16: Manager opened tracking and marked S0 in progress after OntoIndex freshness check.
- 2026-06-16: S0 accepted. Worker reported S1 can start; manager-side OntoIndex found `list_thread_spawn_descendants` is public/DANGEROUS, so topology stages must avoid changing that API unless separately approved.
- 2026-06-16: S1 marked in progress before dispatch.
- 2026-06-16: S1 accepted. Migration reviewed, `git diff --check` passed, and `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 134 tests.
- 2026-06-16: S2 marked in progress before dispatch.
- 2026-06-16: S2 accepted after manager cleanup. `risk` is nullable because the SQL column is nullable; enum tests live in a sibling test file. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 138 tests.
- 2026-06-16: S3 marked in progress before dispatch.
- 2026-06-16: S3 accepted after manager import fix. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 146 tests.
- 2026-06-16: S4 marked in progress before dispatch.
- 2026-06-16: S4 accepted after runtime secret rejection and manifest dependency guards landed. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 148 tests.
- 2026-06-16: S5 marked in progress before dispatch.
- 2026-06-16: S5 accepted after manager review. Runtime topology evidence now traverses mixed open/closed descendant paths without changing core/thread-manager live-edge behavior. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 152 tests.
- 2026-06-16: S6 marked in progress before dispatch.
- 2026-06-16: S6 accepted as a verified no-op. The generic bounded query API already supports task/thread/symbol/file/domain/gate/status/risk/target-head/freshness filters with capped summaries and provenance/source links. No wrappers added.
- 2026-06-16: S7 marked in progress before dispatch.
- 2026-06-16: S7 accepted after manager hardening. Valid JSON artifacts containing raw source, raw diff, or graph dump payloads are rejected before persistence. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 158 tests.
- 2026-06-16: S8 marked in progress before dispatch.
- 2026-06-16: S8 accepted after manager domain correction. Workflow record kinds are imported through the existing artifact contract, with task/gate/readiness as workflow evidence and test/doc/redaction summaries represented by their natural domains. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 160 tests.
- 2026-06-16: S9 marked in progress before dispatch.
- 2026-06-16: S9 accepted after manager policy hardening. The closure evaluator now always requires dispatch evidence when dispatched, requires no-code closure evidence for non-code closure, and does not allow the no-code flag to bypass code-edit gates. `CARGO_BUILD_JOBS=8 just test -p ontocode-state` passed 167 tests.
