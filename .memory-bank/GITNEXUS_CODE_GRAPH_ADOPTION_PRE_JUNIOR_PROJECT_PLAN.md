---
name: GitNexus Code-Graph Adoption Pre-Junior Project Plan
description: Junior-safe implementation plan for the operational evidence backbone from ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md
type: project_plan
date: 2026-06-16
status: done-with-blocked-s10
---

# GitNexus Code-Graph Adoption Pre-Junior Project Plan

Status note: implementation stages `S0` through `S9` are complete and accepted in
[GITNEXUS_CODE_GRAPH_ADOPTION_TRACKING.md](GITNEXUS_CODE_GRAPH_ADOPTION_TRACKING.md).
`S10` remains blocked until a separate ADR approves model-visible operational evidence.

## Goal

Implement the consolidated solution from [ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md](ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md): a small Rust state-backed `operational_evidence_records` ledger.

This is not a GitNexus port. Do not add a graph engine, app-server API, TUI API, prompt injection, Node runtime, LadybugDB dependency, lean-ctx clone, or bundled analyzer binary.

## Source ADR

Authoritative source: [ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md](ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md)

Binding order:

1. `G1` operational evidence state.
2. `G3` runtime topology ingestion.
3. `G4` bounded internal query helpers.
4. `G2` Rust artifact importer.
5. `G2b` workflow evidence importer.
6. `G5` optional context fragment only after a new approval.

## Pre-Junior Rules

- Change one stage at a time.
- Read the listed files before editing.
- Run OntoIndex impact before editing any Rust symbol.
- Do not touch `ontocode-rs/core` unless the stage explicitly says so.
- Prefer `ontocode-rs/state`; this feature is a state ledger first.
- No public API, schema, app-server, SDK, TUI, or model-context changes in stages `S0` through `S6`.
- No new third-party dependencies.
- No raw source, raw diffs, prompts, terminal output, logs, credentials, graph rows, or transcripts in persisted evidence.

## Non-Goals

- No GitNexus/LadybugDB runtime adoption.
- No `.gitnexus/lbug` parsing.
- No static code graph indexing in Ontocode.
- No `AgentGraphStore` promotion or trait refactor.
- No lean-ctx shell/read/search/cache/session clone.
- No app-server/TUI graph or audit endpoint.
- No default prompt/context injection.
- No new audit database.

## Stage S0: Baseline And Impact

Purpose: identify exact owners before editing.

Files to read:

- `ontocode-rs/state/src/runtime.rs`
- `ontocode-rs/state/src/lib.rs`
- `ontocode-rs/state/src/model/mod.rs`
- `ontocode-rs/state/src/runtime/threads.rs`
- `ontocode-rs/state/src/model/graph.rs`
- `ontocode-rs/state/migrations/`

Required OntoIndex checks before code edits:

```text
context: StateRuntime
impact: StateRuntime
context: list_thread_spawn_descendants
impact: list_thread_spawn_descendants
```

Done when:

- The worker has recorded impacted callers and risk.
- The next migration number is confirmed.
- No HIGH/CRITICAL warning is ignored.
- No code is changed in this stage unless it is only documentation/tracking.

## Stage S1: Add State Migration

Purpose: add the table and indexes only.

Files to change:

- `ontocode-rs/state/migrations/0036_operational_evidence_records.sql`

Required table:

- `operational_evidence_records`

Required fields:

- `id`
- `evidence_domain`
- `source_tool`
- `source_version`
- `schema_version`
- `source_ref`
- `repo`
- `task_key`
- `thread_id`
- `parent_thread_id`
- `child_thread_id`
- `symbol_uid`
- `symbol_name`
- `file_path`
- `process_label`
- `gate_name`
- `risk`
- `status`
- `summary`
- `source_links_json`
- `metadata_json`
- `provenance_hash`
- `redaction_status`
- `target_head`
- `graph_index_id`
- `plan_hash`
- `tracking_hash`
- `created_at`
- `expires_at`

Required indexes:

- `idx_operational_evidence_task_status`
- `idx_operational_evidence_thread`
- `idx_operational_evidence_symbol`
- `idx_operational_evidence_file`
- `idx_operational_evidence_domain_risk`
- `idx_operational_evidence_target_head`
- `idx_operational_evidence_expires_at`

Done when:

- Migration is additive.
- `summary`, `source_links_json`, and `metadata_json` have SQLite length checks from the ADR.
- `provenance_hash` is unique.
- No Rust code is changed in this stage.

Suggested verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Stage S2: Add Rust Model Types

Purpose: add data types without runtime behavior.

Files to change:

- `ontocode-rs/state/src/model/operational_evidence.rs`
- `ontocode-rs/state/src/model/mod.rs`

Required enums:

- `EvidenceDomain`: `CodeGraph`, `Workflow`, `Test`, `Doc`, `Redaction`, `Architecture`, `RuntimeTopology`
- `EvidenceStatus`: `Planned`, `Dispatched`, `Implemented`, `Verified`, `Stale`, `Blocked`, `Done`, `Rejected`
- `EvidenceRisk`: `None`, `Low`, `Medium`, `High`, `Critical`, `Unknown`
- `RedactionStatus`: `Clean`, `Redacted`, `Rejected`

Required structs:

- `OperationalEvidenceRecord`
- `NewOperationalEvidenceRecord`
- `OperationalEvidenceQuery`
- `OperationalEvidenceSummary`

Done when:

- Enums serialize to the snake-case DB strings from the ADR.
- Opaque literal call sites use argument comments if needed.
- No storage writes are implemented yet.

Suggested verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Stage S3: Add Insert, Upsert, Query, And Prune

Purpose: make the state ledger usable.

Files to change:

- `ontocode-rs/state/src/runtime/operational_evidence.rs`
- `ontocode-rs/state/src/runtime.rs`
- `ontocode-rs/state/src/lib.rs`

Required methods:

- `insert_operational_evidence(record)`
- `upsert_operational_evidence_by_provenance(record)`
- `query_operational_evidence(query)`
- `prune_operational_evidence(now)`

Required behavior:

- Query supports task key, thread id, symbol uid, file path, evidence domain, gate name, status, risk, target head, and freshness filters.
- Query enforces caps: default `limit` is 50, max `limit` is 200, default byte cap is 64 KiB, and max byte cap is 256 KiB.
- Query order is deterministic.
- Prune deletes only expired records.

Done when:

- Tests cover insert, duplicate provenance upsert, filters, stable ordering, byte cap, and expiry prune.
- Tests prove oversized summaries/metadata are rejected before persistence.
- No public API is added.

Suggested verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Stage S4: Add Redaction And Dependency Guards

Purpose: prevent the ledger from becoming a secret/log/source dump.

Files to inspect first:

- Existing redaction helpers in `ontocode-rs/`
- Existing state tests near `ontocode-rs/state/src/runtime/`

Required checks in this stage:

- Reject token-like content.
- Reject cookie-like content.
- Reject authorization headers.
- Reject keychain paths.
- Reject raw credential values.

Deferred to a later senior-reviewed slice:

- Raw source classification.
- Raw diff classification.
- Raw graph-row classification.
- Raw prompt classification.
- Raw terminal-output classification.
- Raw log classification.

Dependency guard:

- Add a test or script check that fails if runtime/core/app-server/SDK manifests gain `gitnexus`, `@ladybugdb/core`, tree-sitter, graphology, ONNX, transformers, Express, MCP SDK, or lean-ctx runtime dependencies.

Done when:

- Redaction tests fail on obvious secret samples.
- Dependency guard test passes.
- No new dependency is added.

Suggested verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Stage S5: Runtime Topology Evidence

Purpose: ingest existing spawned-agent topology as compact evidence.

Files to inspect first:

- `ontocode-rs/state/src/runtime/threads.rs`
- `ontocode-rs/state/src/model/graph.rs`

Files likely to change:

- `ontocode-rs/state/src/runtime/operational_evidence.rs`
- `ontocode-rs/state/src/runtime.rs`

Allowed stored fields:

- parent thread id
- child thread id
- edge status
- agent path when already available from existing state query
- source timestamp

Forbidden stored fields:

- transcripts
- prompts
- terminal output
- raw rollout payloads
- model context

Done when:

- Tests cover open descendants.
- Tests cover closed descendants.
- Ordering is deterministic.
- Evidence domain is `runtime_topology`.
- No core/thread-manager live-edge merge behavior is changed.

Suggested verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Stage S6: Internal Query Helpers

Purpose: make manager/subagent code able to ask bounded questions without raw data.

Files likely to change:

- `ontocode-rs/state/src/runtime/operational_evidence.rs`
- `ontocode-rs/state/src/model/operational_evidence.rs`

Required helpers:

- Query by task.
- Query by thread.
- Query by symbol.
- Query by file.
- Query by evidence domain.
- Query by gate/status/risk.
- Query stale evidence by target head or freshness.

Done when:

- Helpers return capped summaries with provenance and source links.
- Callers can distinguish `code_graph`, `runtime_topology`, and `workflow`.
- No app-server, SDK, schema, or TUI surface is added.

Suggested verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Stage S7: Rust Artifact Contract And Fixture Importer

Purpose: import explicit bounded artifact files, not GitNexus runtime data.

Files to add only if needed:

- `ontocode-rs/state/src/runtime/operational_evidence_import.rs`
- fixture files under the existing state test fixture pattern

Required artifact fields:

- `schemaVersion`
- `sourceTool`
- `sourceVersion`
- `repo`
- `targetHead`
- `graphIndexId`
- `createdAt`
- `records`

Rules:

- Importer reads files only when explicitly called.
- Importer does not execute external commands.
- Importer does not parse `.gitnexus/lbug`.
- Unsupported schema versions fail closed.
- Missing provenance fails closed.

Done when:

- Low, high, and critical impact fixtures import.
- Raw source fixture is rejected.
- Raw diff fixture is rejected.
- Raw graph dump fixture is rejected.
- Missing external artifacts degrade to no evidence.

Suggested verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Stage S8: Workflow Evidence Import

Purpose: support bounded plan/task/test/readiness records without lean-ctx runtime.

Allowed record kinds:

- task card
- gate result
- doc-link report
- test summary
- redaction report
- readiness summary

Rules:

- Treat lean-ctx tool names and versions as provenance only.
- Do not invoke lean-ctx.
- Do not persist compressed shell/read/search output bodies.
- Do not persist cache/session data.

Done when:

- Fixture tests cover each allowed record kind.
- Oversized output is rejected.
- Secret-bearing output is rejected.
- No lean-ctx package, binary, MCP tool, or runtime cache is required.

Suggested verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Stage S9: Planned-Versus-Done Gates

Purpose: answer whether a task can be closed from compact evidence.

Required gate checks:

- plan evidence exists
- dispatch evidence exists when the task was dispatched
- impact evidence exists for code edits
- implementation evidence exists for code edits
- test evidence exists or an explicit no-code closure exists
- detect-changes evidence exists for code edits
- evidence target head and plan/tracking hashes are fresh

Done when:

- Tests cover accepted code task.
- Tests cover accepted no-code documentation task.
- Tests reject missing tests.
- Tests reject stale target head.
- Tests reject chat-only completion.

Suggested verification:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Stage S10: Optional Context Fragment Review

Status: blocked.

Do not implement this stage from this plan.

Only create a new ADR if a real manager/subagent prompt needs model-visible operational evidence after `S1` through `S9` are accepted.

Required future constraints:

- Use existing `ContextualUserFragment`.
- Do not change the trait shape.
- Add hard caps.
- Add redaction and truncation tests.
- Add memory-exclusion handling.
- Mark memory mode polluted only when evidence is injected into a model turn.

## Final Verification For Any Code Stage

Run after Rust changes:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

Run before manager acceptance:

```bash
cd /opt/demodb/_workfolder/ontocode
git diff --check
ontoindex analyze --skills --skip-agents-md
```

## Dispatch Order

| Order | Stage | Status | Owner level |
| --- | --- | --- | --- |
| 0 | S0 baseline and impact | done | senior or supervised junior |
| 1 | S1 state migration | done | pre-junior |
| 2 | S2 model types | done | pre-junior |
| 3 | S3 state runtime methods | done | junior |
| 4 | S4 secret redaction and dependency guards | done | senior-reviewed junior |
| 5 | S5 runtime topology evidence | done | junior |
| 6 | S6 internal query helpers | done | junior |
| 7 | S7 artifact importer | done | senior-reviewed junior |
| 8 | S8 workflow evidence import | done | junior |
| 9 | S9 planned-versus-done gates | done | senior-reviewed junior |
| 10 | S10 context fragment review | blocked | future ADR |
