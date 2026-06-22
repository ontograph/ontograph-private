# ADR: Qwen Donor Blocked Rows Unblock

Date: 2026-06-20
Status: accepted-for-narrow-dispatch

## Context

`tmp/qwen-code-donor-dispatch-tracking.md` closed with 12 blocked Qwen donor rows:

- `QWN-005`, `QWN-007`, `QWN-009`
- `QWN-015`, `QWN-020`, `QWN-025`, `QWN-027`
- `QWN-036`, `QWN-107`, `QWN-127`, `QWN-164`, `QWN-165`

The previous closeout correctly rejected broad new owners: no duplicate tool metadata registry, approval policy engine, read-evidence database, hook API, shell task registry, transcript store, artifact classifier, or provider error taxonomy.

## Decision

Unblock only the narrow existing-owner slices below. Full public API, config, schema, persisted-state, model-context, HTTP-hook, or transcript surfaces remain blocked unless a later ADR approves them.

| Rows | Accepted narrow solution | Existing owner |
| --- | --- | --- |
| `QWN-005`, `QWN-007` | Add internal-only built-in tool classification if needed for tests/guardian/spec planning. Do not expose it in public `ToolSpec`. | `ontocode-rs/core/src/tools/spec_plan.rs` |
| `QWN-009` | Add disabled/source reason only to internal tool-search text or existing internal result fields. No public output schema change. | `ontocode-rs/tools/src/tool_search.rs`, dynamic tool search text |
| `QWN-015`, `QWN-020`, `QWN-027` | Allow only per-turn in-memory file-read evidence from explicit tool/shell reads. No durable proof cache. | existing tool-events / turn context paths |
| `QWN-025` | Add generated-file path heuristic only for apply-patch/guardian warning tests. No model-facing suggestion policy. | `guardian` / apply-patch adjacent code |
| `QWN-036` | Add a missing unified-exec lifecycle cleanup regression only if a real gap is found; otherwise close as covered. | `ontocode-rs/core/src/unified_exec` |
| `QWN-107` | Persist only an already-bounded final summary or last-N output if existing agent-job state can hold it without migration. No full transcript store. | `core/src/tools/handlers/agent_jobs.rs`, `state/src/runtime/agent_jobs.rs` |
| `QWN-127` | Do not add native HTTP hooks. Close or add coverage proving process hooks inherit network-proxy SSRF/private-IP controls. | `ontocode-rs/hooks`, `ontocode-rs/network-proxy` |
| `QWN-164` | Reuse existing bounded operational evidence artifacts only if compaction can reference already-imported bounded artifacts. No artifact classifier. | `state/src/runtime/operational_evidence_import.rs`, compaction owners |
| `QWN-165` | Extract/reuse a small context-window error classifier from existing provider/client behavior. No provider trait taxonomy. | `codex-api/src/sse/responses.rs`, compaction tests |

## Rejected For This Pass

- Persistent SQLite read-evidence tracking.
- Full sub-agent transcript storage.
- First-class HTTP hooks.
- New model-visible artifact selection.
- Public tool capability metadata.

SQLite in-memory is allowed only if a worker proves a plain per-turn Rust map is insufficient. Default implementation should be a small in-memory struct.

## Verification

Each dispatched slice must:

- Run OntoIndex context/impact before symbol edits.
- Update `tmp/qwen-code-donor-dispatch-tracking.md` before starting and after completion.
- Run scoped tests for changed crates.
- Run `CARGO_BUILD_JOBS=8 just fmt` after Rust edits.
- Refresh/check OntoIndex after completion.
