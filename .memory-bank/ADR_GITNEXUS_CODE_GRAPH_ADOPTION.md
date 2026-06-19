# ADR: GitNexus Code-Graph Adoption And Core Decisions

## Status

Challenged - consolidate GitNexus and lean-ctx into one operational evidence backbone; third-party tooling stays behind local evidence/tooling boundaries, not core runtime.

## Date

2026-06-07

## Context

The local `../GitNexus` workspace currently checked in this environment is the TypeScript GitNexus analyzer/runtime package, not a Rust `ontocode-rs` donor tree. Its useful functionality for Ontocode is the bounded analyzer/report surface:

- `context` reports for symbol identity, incoming/outgoing relationships, process participation, and concepts
- `impact` reports for risk, affected processes, affected modules, direct counts, and bounded graph traversal summaries
- `detect_changes` reports for changed symbols, affected processes, risk level, and warnings
- audit lifecycle records with schema versioning, target-head freshness, graph index identity, verification evidence, dispatch, scope guards, tombstones, and projections
- analyzer storage behind `@ladybugdb/core`, tree-sitter grammars, graphology, ONNX/transformers, Express, and MCP SDK dependencies

Ontocode already contains runtime graph functionality that is useful for the same operational evidence backbone:

- a persisted runtime graph for spawned agent threads
- deterministic graph traversal through state-backed edges
- rollout trace bundles reduced into semantic runtime graphs
- interaction edges between agents, tools, code cells, terminal operations, inference, and compaction
- analytics facts for subagent/thread relationships
- agent identity primitives

Important finding after the 2026-06-16 source check: the ADR's earlier `../GitNexus/ontocode-rs/...` donor links are stale in this workspace. The immediate task is therefore not to copy GitNexus Rust code. The safe decision is to formalize Ontocode's existing persisted `thread_spawn_edges` runtime graph as one input into a broader code-graph-memory backbone, and to import GitNexus analyzer output only through bounded artifacts.

Code-graph-memory means durable, compact, provenance-rich graph facts that Ontocode can use across manager/subagent work: symbol owner evidence, impact risk, affected process labels, source links, runtime thread topology, and verification results. It does not mean storing raw source, importing a full GitNexus graph database, or running a second static symbol indexer inside Ontocode.

Static symbol/call graph analysis remains owned by external GitNexus tooling. Ontocode may consume bounded GitNexus reports and persist compact code-graph-memory records through existing state/memory owners, but must not create a second static code graph indexer.

This ADR also inlines the third-party dependency boundary from `ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md`. Lean-ctx-inspired workflow tools are not a separate runtime dependency family. They may produce repository-only reports or operational evidence records, but their read/search/shell/session mechanics remain external development workflow behavior.

## Challenge Findings

GitNexus review found two opposite risks:

- The first version was too optimistic about promoting graph APIs. `AgentGraphStore` is not the active runtime seam today. GitNexus context for `AgentGraphStore` shows only one implementation, `LocalAgentGraphStore`, and no execution-flow consumers. Current production graph behavior flows through `StateRuntime` methods and core agent/thread-manager call sites.
- The challenged version was too defensive for the new goal. It blocked GitNexus evidence bridges and model-visible graph context entirely, which prevents code-graph-memory from becoming a backbone capability.

Additional GitNexus challenge evidence:

- `ContextualUserFragment` upstream impact is `CRITICAL`: 29 direct implementers, 38 total impacted symbols, one affected process, and seven affected modules. A code-graph-memory context bridge must add a narrow fragment implementation, not mutate the shared trait or all context fragments.
- `mark_thread_memory_mode_polluted` upstream impact is `HIGH`: direct callers are MCP tool-call handling and stream-event external-context handling. Code-graph-memory ingestion must separate durable evidence import from model-visible external-context injection.
- `MemoryStore` as a state memory owner is low-risk as a concept, but existing `stage1_outputs` is rollout memory, not a generic graph fact table. Code-graph-memory needs its own state-backed schema rather than overloading rollout memory rows.

Third-party dependency challenge:

- Current checked source on 2026-06-16 shows `../GitNexus/gitnexus/package.json` at `gitnexus@1.6.2`, depending on `@ladybugdb/core` through the semver range `^0.16.1`, plus native/platform optional packages such as `@ladybugdb/core-linux-x64`, `@ladybugdb/core-darwin-arm64`, and `@ladybugdb/core-win32-x64`.
- The unscoped `ladybugdb` package name is not present in npm; the relevant package is `@ladybugdb/core`.
- `@ladybugdb/core` is a native in-process property graph database dependency with its own transitive dependencies and platform binaries. It is useful source evidence for the GitNexus analyzer design, but it is not acceptable inside Ontocode runtime, memory, state, app-server, context, SDK, or packaging.
- Other GitNexus analyzer dependencies are also rejected for Ontocode integration: tree-sitter grammars, graphology packages, `onnxruntime-node`, `@huggingface/transformers`, Express, and the MCP SDK. Code-graph-memory must translate GitNexus concepts into Rust-native Ontocode state/query/audit code instead of bundling those dependencies.
- The corrected Rust-only position: do not bundle analyzer third-party dependencies into an Ontocode-owned local binary. GitNexus may remain an external developer tool, but Ontocode adopts only the bounded evidence shapes and reimplements the durable backbone, gates, and summaries in Rust.
- Lean-ctx is a development/workflow tool only. Ontocode must not vendor lean-ctx, depend on its CLI/runtime, copy its shell/read/search/session cache, or expose lean-ctx tools as model-visible product tools.
- Third-party tooling is consolidated into two allowed classes: repository-only scripts with no runtime dependency, and external developer tools used by agents outside Ontocode runtime.

Corrected challenge result:

- Accept code-graph-memory as a first-class internal backbone layer made of bounded state-backed evidence records.
- Accept the Lean-CTX operational backbone concepts as non-graph domains inside the same operational evidence backbone: task cards, evidence records, gate results, and readiness summaries.
- Accept existing `thread_spawn_edges` plus `StateRuntime` graph methods as canonical runtime topology input to code-graph-memory.
- Accept GitNexus `context`, `impact`, and `detect_changes` outputs as bounded evidence inputs containing symbol IDs, file links, process labels, risk levels, source owners, and timestamps.
- Treat `AgentGraphStore` as a thin internal abstraction candidate, not the canonical core seam until real consumers exist.
- Allow an optional model-context bridge only through the existing bounded context-fragment architecture, with hard caps, explicit opt-in, and memory-exclusion handling.
- Defer app-server/TUI graph APIs and support-bundle graph summaries until separate owner-specific ADRs.
- Reject any embedded static code graph indexer in core, raw GitNexus graph DB import, in-process LadybugDB-backed runtime store, unified graph engine, analytics-backed graph state, or default model-visible graph dump.
- Reject any parallel lean-ctx runtime, tool registry, context store, memory store, or shell/read/search subsystem inside Ontocode.

## Consolidated Third-Party Dependency Boundary

This ADR is the canonical dependency consolidation record for GitNexus and lean-ctx derived work.

| Dependency source | Allowed role | Boundary | Rejected coupling |
|---|---|---|---|
| GitNexus analyzer and `@ladybugdb/core` | Source evidence and external developer analysis. | External tool only; Ontocode imports bounded JSON artifact shapes through Rust code when explicitly provided. | Direct Rust/app-server/SDK dependency, direct `.gitnexus/lbug` parsing, in-process LadybugDB store, bundled Node/LadybugDB evidence binary. |
| GitNexus MCP/CLI | Development-time evidence collection and manager workflow support. | External tool invocation by humans/agents, with explicit artifact handoff to Rust importers. | Production request-path dependency, automatic shell-out from core, or persisted raw graph output. |
| Lean-ctx MCP/CLI | Agent development workflow for compressed reads, shell output, search, and session handling. | External agent workflow only. | Vendored runtime, product CLI dependency, copied cache/session/search/shell subsystem. |
| Repository-only scripts | Bootstrap reports, link checks, status counts, and task-card generation. | `scripts/` or memory-bank tooling with standard-library-first implementation. | Runtime crate dependency, app-server API, model-visible tool registration, automatic silent status mutation. |
| Operational evidence backbone | Durable local facts, gates, and readiness summaries derived from approved inputs. | Existing `StateRuntime`/state ownership, bounded records, redaction, retention, provenance. | Separate database root, raw logs/source/secrets, second memory store, third-party graph/search/runtime library. |
| Model-visible summaries | Optional bounded context after separate approval. | Existing `ContextualUserFragment` path with hard caps and memory-exclusion handling. | Side-channel context injection or unbounded tool output. |

Consolidation rules:

- One backbone schema owns both GitNexus code-graph evidence and lean-ctx-inspired operational evidence.
- GitNexus evidence is a `code_graph` evidence domain, not a separate storage engine.
- Lean-ctx-inspired task/gate/readiness records are `workflow`, `test`, `doc`, `redaction`, or `architecture` evidence domains, not a copied lean-ctx runtime.
- Every imported artifact must carry source tool name, tool version when known, schema version, provenance hash, created timestamp, redaction status, and max-size validation result.
- Normal Ontocode runtime must still work when GitNexus, lean-ctx, or any external evidence artifact is absent.

## Source Evidence

OntoIndex implementation evidence reviewed:

- [OntoIndex README](/opt/demodb/_workfolder/OntoIndex/README.md:1) describes the local code graph, impact analysis, MCP, CLI, HTTP, and web surfaces that now correspond to this ADR's external analyzer boundary.
- [OntoIndex dependency manifest](/opt/demodb/_workfolder/OntoIndex/ontoindex/package.json:59), [IndexStore port](/opt/demodb/_workfolder/OntoIndex/ontoindex-shared/src/ports/index-store.ts:4), and [LadybugDB adapter](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/core/lbug/lbug-adapter.ts:7) show the analyzer-side dependency and storage boundary this ADR keeps out of Ontocode runtime.
- [analysis orchestrator](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/core/run-analyze.ts:720) and [runFullAnalysis](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/core/run-analyze.ts:744) are the source-side pipeline that builds graph artifacts and loads LadybugDB evidence.
- [MCP server](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/mcp/server.ts:1), [facade dispatch](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/mcp/facade/dispatch.ts:27), [super-tool definitions](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/mcp/super/tool-definitions.ts:46), and [super dispatch](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/mcp/super/dispatch.ts:95) implement the external tool/report surface that Ontocode may consume as bounded evidence.
- [context backend](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/mcp/local/backend-context.ts:178), [impact backend](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/mcp/local/backend-impact.ts:174), and [detect-changes backend](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/mcp/local/backend-detect-changes.ts:131) are the concrete report producers for the `context`, `impact`, and `detect_changes` evidence inputs accepted by this ADR.
- [audit event store](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/core/audit-lifecycle/audit-event-store.ts:132), [fresh-evidence verifier](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/core/audit-lifecycle/finding-verify.ts:77), [audit verify tool](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/mcp/super/audit-verify.ts:39), and [audit session tools](/opt/demodb/_workfolder/OntoIndex/ontoindex/src/mcp/super/audit-session-tools.ts:320) map to the ADR's bounded verification, dispatch, and tombstone lifecycle evidence.

Current checked GitNexus implementation:

- [GitNexus package manifest](/opt/demodb/_workfolder/GitNexus/gitnexus/package.json:3) shows the checked source version `1.6.2`, the CLI binary entrypoint, and analyzer dependencies including `@ladybugdb/core`, tree-sitter grammars, graphology, ONNX/transformers, Express, and MCP SDK.
- [GitNexus context backend](/opt/demodb/_workfolder/GitNexus/gitnexus/src/mcp/local/backend-context.ts:178) produces bounded symbol context with symbol identity, incoming/outgoing relationship groups, process participation, and concepts.
- [GitNexus impact backend](/opt/demodb/_workfolder/GitNexus/gitnexus/src/mcp/local/backend-impact.ts:156) produces target identity, direction, impact counts, risk, affected processes, affected modules, by-depth nodes, warnings, and partial-result markers.
- [GitNexus detect-changes backend](/opt/demodb/_workfolder/GitNexus/gitnexus/src/mcp/local/backend-detect-changes.ts:131) shells out to `git diff` with bounded timeout/buffer/file/hunk/symbol caps, then returns changed symbols, affected processes, risk level, and warnings.
- [GitNexus audit event store](/opt/demodb/_workfolder/GitNexus/gitnexus/src/core/audit-lifecycle/audit-event-store.ts:23) shows schema-versioned audit events and projections under `.gitnexus/audit`; Ontocode must not copy that storage layout.
- [GitNexus audit session schema](/opt/demodb/_workfolder/GitNexus/gitnexus/src/core/audit-lifecycle/audit-session.ts:38) shows useful fields to normalize: target repo/head, source hash, graph index id, verifier version, sidecar hash, changed files/symbols, stale warnings, finding status, verification evidence, tombstones, and bundles.

Current Ontocode runtime graph owners:

- [agent-graph-store trait](/opt/demodb/_workfolder/ontocode/ontocode-rs/agent-graph-store/src/store.rs:1)
- [agent-graph-store SQLite adapter](/opt/demodb/_workfolder/ontocode/ontocode-rs/agent-graph-store/src/local.rs:1)
- [agent graph edge status](/opt/demodb/_workfolder/ontocode/ontocode-rs/agent-graph-store/src/types.rs:1)
- [state graph model](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/model/graph.rs:1)
- [thread spawn edge migration](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/migrations/0021_thread_spawn_edges.sql:1)
- [state thread graph methods](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/runtime/threads.rs:78)
- [runtime subtree merge in thread manager](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/thread_manager.rs:508)
- [agent resume/close graph use](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/agent/control.rs:523)
- [rollout-trace design](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout-trace/README.md:1)
- [ThreadTraceContext](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout-trace/src/thread.rs:76)
- [RolloutTrace model](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout-trace/src/model/mod.rs:54)
- [InteractionEdge model](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout-trace/src/model/runtime.rs:305)
- [agent interaction reducer](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout-trace/src/reducer/tool/agents.rs:16)
- [subagent analytics fact](/opt/demodb/_workfolder/ontocode/ontocode-rs/analytics/src/facts.rs:347)
- [agent identity primitives](/opt/demodb/_workfolder/ontocode/ontocode-rs/agent-identity/src/lib.rs:40)

Current Ontocode memory/context backbone evidence:

- [lean-ctx project tool ADR](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1)
- [memory pipeline owner docs](/opt/demodb/_workfolder/ontocode/ontocode-rs/memories/README.md:1)
- [memory state migration](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/memory_migrations/0001_memories.sql:1)
- [MemoryStore owner](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/runtime/memories.rs:27)
- [Stage1Output model](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/model/memories.rs:1)
- [memory exclusion for contextual fragments](/opt/demodb/_workfolder/ontocode/ontocode-rs/memories/write/src/phase1.rs:457)
- [ContextualUserFragment trait](/opt/demodb/_workfolder/ontocode/ontocode-rs/context-fragments/src/fragment.rs:45)
- [contextual-user fragment classifier](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/context/contextual_user_message.rs:1)
- [rollout memory pollution bridge](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout/src/state_db.rs:457)

GitNexus challenge evidence captured for this ADR:

- `ContextualUserFragment` impact: `CRITICAL`, 29 direct implementers, 38 total impacted symbols.
- `mark_thread_memory_mode_polluted` impact: `HIGH`, direct callers in MCP tool-call and stream-event external-context paths.
- `MemoryStore` struct impact: `LOW`, but existing state memory schema is rollout-memory specific.
- Checked `../GitNexus/gitnexus` source reports `gitnexus@1.6.2`; its manifest depends on `@ladybugdb/core` `^0.16.1` and many analyzer/runtime packages. Those packages are rejected for Ontocode runtime/core/app-server/SDK dependencies and for Ontocode-owned binaries in this ADR.
- `@ladybugdb/core` npm metadata: latest checked version `0.17.1`, MIT license, native optional platform packages, repository `github.com/LadybugDB/ladybug`.
- Rust-only translation decision: third-party analyzer dependencies must not appear as direct dependencies of Ontocode Rust crates, app-server packages, SDKs, persisted state APIs, or Ontocode-owned helper binaries.

## Functionality Comparison

This table preserves the earlier Rust runtime comparison for Ontocode's existing graph behavior. The `../GitNexus/ontocode-rs/...` paths in the left column are historical evidence from an earlier workspace shape and are not valid implementation inputs in the current checked source. Current GitNexus implementation evidence is the TypeScript analyzer/report source listed above. For implementation, use the current Ontocode paths in the middle column and the GitNexus artifact contracts from `backend-context.ts`, `backend-impact.ts`, `backend-detect-changes.ts`, and audit lifecycle schemas.

| Functionality | Local `../GitNexus` implementation | Current Ontocode status | Core decision |
|---|---|---|---|
| Storage-neutral spawned-thread graph trait | `AgentGraphStore` defines upsert, close-status update, direct child listing, and descendant listing in [store.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/agent-graph-store/src/store.rs:12). | Same file exists in Ontocode at [store.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/agent-graph-store/src/store.rs:12), but GitNexus shows no execution-flow consumers beyond its local implementation. | Do not promote this as canonical yet. Keep as internal candidate; refactor away `#[async_trait]` only if a real consumer or cleanup task is accepted. |
| SQLite-backed graph adapter | `LocalAgentGraphStore` wraps `StateRuntime` in [local.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/agent-graph-store/src/local.rs:13). | Same adapter exists in Ontocode at [local.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/agent-graph-store/src/local.rs:13). | Keep adapter thin and inert. Do not add graph storage outside `codex-state`, and do not route production behavior through this trait without impact review. |
| Edge lifecycle status | `ThreadSpawnEdgeStatus` supports `open` and `closed` in [types.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/agent-graph-store/src/types.rs:5). | Same type exists in Ontocode at [types.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/agent-graph-store/src/types.rs:5). | Keep lifecycle enum narrow until concrete behavior needs more states. Do not infer runtime status from process liveness alone. |
| Persisted edge table | `thread_spawn_edges(parent_thread_id, child_thread_id primary key, status)` in [0021_thread_spawn_edges.sql](/opt/demodb/_workfolder/GitNexus/ontocode-rs/state/migrations/0021_thread_spawn_edges.sql:1). | Same migration exists in Ontocode at [0021_thread_spawn_edges.sql](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/migrations/0021_thread_spawn_edges.sql:1). | This table is the canonical durable parent-child topology, not transcript metadata. |
| State enum backing DB status | `DirectionalThreadSpawnEdgeStatus` uses snake-case string serialization through `strum` in [graph.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/state/src/model/graph.rs:8). | Same model exists in Ontocode at [graph.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/model/graph.rs:8). | Keep DB status and public graph-store status separate so storage can evolve without leaking DB details. |
| Upsert edge behavior | State runtime inserts or replaces child incoming edge in [threads.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/state/src/runtime/threads.rs:78). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/runtime/threads.rs:78). | Child thread has one persisted parent. Re-parenting is allowed only through explicit upsert behavior. |
| Close edge behavior | `set_thread_spawn_edge_status` updates missing child as successful no-op in [threads.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/state/src/runtime/threads.rs:105). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/runtime/threads.rs:105). | Close is graph lifecycle state, not deletion. Preserve closed edges for resume/history. |
| Direct child listing | `list_thread_spawn_children(_with_status)` returns direct children with stable ordering in [threads.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/state/src/runtime/threads.rs:118). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/runtime/threads.rs:118). | Use direct listing for list-agents and targeted child operations. |
| Descendant traversal | Recursive SQL lists descendants breadth-first by depth and thread id in [threads.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/state/src/runtime/threads.rs:137). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/runtime/threads.rs:137). | Use persisted descendant traversal for resume/close/history. Avoid ad hoc in-memory tree reconstruction when DB is available. |
| Path-based child lookup | Direct and descendant path lookup joins `threads.agent_path` in [threads.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/state/src/runtime/threads.rs:161). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/runtime/threads.rs:161). | Use canonical agent path lookups for v2 task-name targeting. Do not add a second name index. |
| Backfill from session source | State runtime extracts parent thread from `SessionSource` in [threads.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/state/src/runtime/threads.rs:315). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/runtime/threads.rs:315). | Keep backfill best-effort. Do not trust stale metadata over persisted edges when both exist. |
| Thread manager subtree merge | `list_agent_subtree_thread_ids` merges persisted descendants and live in-memory children in [thread_manager.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/core/src/thread_manager.rs:508). | Same behavior exists in Ontocode at [thread_manager.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/thread_manager.rs:508). | This is the correct bridge: persisted graph first, live graph second, deduped deterministically. |
| Resume open descendants | `resume_agent_from_rollout` walks open persisted children breadth-first in [control.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/core/src/agent/control.rs:523). | Same behavior exists in Ontocode at [control.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/agent/control.rs:523). | Resume should use edge data as source of truth for open descendants. |
| Close persisted edge | `close_agent` marks the edge closed before shutdown in [control.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/core/src/agent/control.rs:798). | Same behavior exists in Ontocode at [control.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/core/src/agent/control.rs:798). | Preserve close idempotency and stale-agent handling. |
| Trace bundle strategy | GitNexus rollout trace observes raw events first and reduces later, documented in [README.md](/opt/demodb/_workfolder/GitNexus/ontocode-rs/rollout-trace/README.md:11). | Same diagnostic architecture exists in Ontocode at [README.md](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout-trace/README.md:11). | Adopt offline reduction as the only approved runtime graph diagnostics path. Do not build reduced graphs on hot path. |
| Thread trace context | `ThreadTraceContext` is no-op capable and root/child aware in [thread.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/rollout-trace/src/thread.rs:76). | Same context exists in Ontocode at [thread.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout-trace/src/thread.rs:76). | Keep tracing opt-in and best-effort. Trace failures must never fail sessions. |
| Reduced rollout graph | `RolloutTrace` separates threads, turns, conversation, inference, code cells, tools, terminals, compactions, edges, and raw payload refs in [model/mod.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/rollout-trace/src/model/mod.rs:54). | Same model exists in Ontocode at [model/mod.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout-trace/src/model/mod.rs:54). | Keep diagnostic graph separate from product state and model-visible context. |
| Information-flow edges | `InteractionEdge` models runtime information flow in [runtime.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/rollout-trace/src/model/runtime.rs:305). | Same model exists in Ontocode at [runtime.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout-trace/src/model/runtime.rs:305). | Use edges only in trace/debug surfaces unless a separate app-server ADR approves a redacted view. |
| Multi-agent trace reducer | Agent reducer resolves spawn/send/close/result edges and fallbacks in [agents.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/rollout-trace/src/reducer/tool/agents.rs:16). | Same reducer exists in Ontocode at [agents.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout-trace/src/reducer/tool/agents.rs:16). | Reuse reducer for diagnostics. Do not duplicate graph inference in app-server or TUI. |
| Subagent analytics | `SubAgentThreadStartedInput` captures subagent thread start facts in [facts.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/analytics/src/facts.rs:347). | Same fact exists in Ontocode at [facts.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/analytics/src/facts.rs:347). | Analytics may aggregate graph facts, but must not become graph state. |
| Agent identity | `AgentIdentityKey` and `AgentIdentityJwtClaims` exist in [lib.rs](/opt/demodb/_workfolder/GitNexus/ontocode-rs/agent-identity/src/lib.rs:40). | Same identity crate exists in Ontocode at [lib.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/agent-identity/src/lib.rs:40). | Keep identity separate from topology. Do not use JWT identity as a graph primary key. |
| Static code graph | GitNexus tool/runtime supplies symbol context, impact, rename, and detect-changes externally. | Ontocode project rules already require GitNexus use, but no core static index exists. | Do not embed static code graph indexing into core. Add bounded report/import surfaces only if needed. |
| GitNexus third-party graph store | GitNexus currently uses `@ladybugdb/core` for its analyzer/index storage through the npm package. | Ontocode has no LadybugDB dependency and stores runtime/memory data through existing Rust/SQLite state ownership. | Do not use LadybugDB in Ontocode. Code-graph-memory stores normalized Rust records, not GitNexus/LadybugDB data files or query APIs. |
| GitNexus analyzer dependencies | GitNexus depends on tree-sitter grammars, graphology, ONNX/transformers, Express, and MCP SDK packages. | Ontocode must not inherit those Node/native dependencies for runtime, app-server, memory, context paths, or helper binaries. | Translate useful report concepts into Rust structs, validators, state rows, and query helpers. |
| Memory pipeline owner | No separate GitNexus copy is needed for Ontocode memory ownership. | Ontocode memory pipeline is documented in [README.md](/opt/demodb/_workfolder/ontocode/ontocode-rs/memories/README.md:1), with state-backed Stage 1 outputs and Phase 2 consolidation. | Code-graph-memory belongs beside state/memory ownership, not in `codex-core` orchestration or analytics. |
| Existing memory schema | Not a code graph fact store. | `stage1_outputs` stores rollout-derived `raw_memory`, `rollout_summary`, and selection metadata in [0001_memories.sql](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/memory_migrations/0001_memories.sql:1). | Do not overload rollout memory rows. Add a separate `code_graph_memory_records` schema if implementation starts. |
| Memory runtime owner | Not a static graph owner. | `MemoryStore` owns state-backed memory job and output operations in [memories.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/state/src/runtime/memories.rs:27). | Add code-graph-memory persistence through state/runtime ownership, keeping leases, pruning, and retention explicit. |
| Context fragment boundary | GitNexus evidence can be summarized for context, but not dumped. | `ContextualUserFragment` is the shared bounded injection trait in [fragment.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/context-fragments/src/fragment.rs:45). | If model-visible, code-graph-memory must be a new narrow fragment with hard caps; do not change the trait shape. |
| Memory exclusion for injected context | GitNexus evidence can be imported without being learned as rollout memory. | Phase 1 excludes AGENTS and skill contextual fragments in [phase1.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/memories/write/src/phase1.rs:457). | Code-graph-memory fragments need explicit classification so internal evidence is not accidentally converted into user memory. |
| External-context pollution bridge | GitNexus evidence import is not the same as model-visible external context. | `mark_thread_memory_mode_polluted` routes external-context pollution to memory state in [state_db.rs](/opt/demodb/_workfolder/ontocode/ontocode-rs/rollout/src/state_db.rs:457). | Only mark polluted when code-graph-memory is injected into the model turn, not when evidence is stored offline. |
| Code-graph-memory record | Proposed from GitNexus `context`, `impact`, `detect_changes`, and runtime graph summaries. | No current Ontocode schema exists. | Add compact records with source kind, symbol/file/process refs, risk, bounded summary, provenance hash, timestamps, and retention. |
| Code-graph-memory query | Proposed internal backbone read path. | No current Ontocode query API exists. | Return bounded summaries by task/thread/symbol. No public app-server API until a separate ADR approves wire shape and compatibility tests. |

## Accepted Scope After Challenge

| Scope | Decision | Reason |
|---|---|---|
| Operational evidence backbone | Accepted | Consolidates GitNexus graph evidence and lean-ctx-inspired workflow evidence into one bounded state-backed record model. |
| Code-graph-memory backbone | Accepted as a domain | Required to make graph evidence durable and reusable across manager/subagent workflows without re-querying or re-indexing every turn. |
| Operational evidence state schema | Accepted for implementation planning | Existing rollout memory schema is not a generic evidence store; use a separate state-backed table with retention and provenance. |
| GitNexus evidence ingestion | Accepted as bounded internal/project tooling | Stores compact `context`, `impact`, and `detect_changes` facts, not raw graph DB rows or raw source bodies. |
| Rust-native GitNexus concept translation | Accepted | Ontocode adopts the useful GitNexus evidence shapes as Rust structs, state records, queries, gates, and audit summaries. |
| Hermetic local evidence binary | Rejected | User direction is Rust-only; do not bundle GitNexus/LadybugDB/parser/ML/Node dependencies into an Ontocode-owned binary. |
| GitNexus/LadybugDB direct dependency adoption | Rejected | Ontocode must not link to GitNexus analyzer internals, LadybugDB native packages, or `.gitnexus/lbug` storage layout. |
| Stable evidence contract | Required | Importers must consume bounded, versioned evidence summaries so GitNexus can change analyzer/storage dependencies without breaking Ontocode. |
| Bounded code-graph context fragment | Accepted only behind hard gates | Uses existing `ContextualUserFragment` architecture with explicit opt-in, caps, redaction, and memory-exclusion handling. |
| Persisted spawned-agent topology | Accepted | Already implemented in `thread_spawn_edges` and consumed by resume/list/close behavior. |
| `StateRuntime` graph methods | Accepted | Active owner for graph persistence and traversal. |
| Core agent/thread-manager graph use | Accepted | Existing call sites already merge persisted and live state. |
| `AgentGraphStore` trait promotion | Deferred | No execution-flow consumers; trait still uses `#[async_trait]`. |
| Read-only internal graph summary helper | Deferred | Needs concrete caller and tests before adding abstraction. |
| App-server/TUI graph API | Blocked | Public API/schema/UI work requires separate ADR and compatibility tests. |
| Rollout-trace diagnostic summary | Deferred | Useful but sensitive; requires redaction and support-bundle ADR. |
| GitNexus static graph evidence bridge | Accepted as evidence import, not indexing | Belongs at the code-graph-memory record boundary; it must not import GitNexus graph storage or run indexing in core. |
| Static code graph index in core | Rejected | Duplicates GitNexus and risks unbounded context/data exposure. |
| Unified runtime/static/analytics/identity graph | Rejected | Mixes incompatible domains and ownership boundaries. |
| Lean-ctx runtime adoption | Rejected | Development-time compression/search/session behavior must remain outside Ontocode runtime and must not become a second context/memory/search substrate. |

## Consolidated Robust Solution

There is one implementation path for this ADR:

1. Add `operational_evidence_records` in the main Rust state DB.
2. Ingest runtime topology from existing `StateRuntime` and `thread_spawn_edges`.
3. Add bounded internal queries over those records.
4. Add explicit Rust artifact importers for external GitNexus/OntoIndex-style evidence.
5. Add planned-versus-done gates over the same records.
6. Add model-visible context only later, only as a bounded opt-in fragment.

Everything else is rejected, deferred, or source evidence. Do not build a second graph engine, do not promote `AgentGraphStore`, do not add app-server/TUI APIs, do not bundle GitNexus/LadybugDB/Node tooling, and do not clone lean-ctx runtime behavior.

The robust shape is a small state-backed ledger, not a platform:

- Writer: validates and stores compact records with provenance, redaction status, target head, risk, status, source links, and expiry.
- Readers: return capped summaries by task, thread, symbol, file, evidence domain, gate, status, risk, and freshness.
- Importers: normalize external artifacts into the same record shape and fail closed on raw source, raw diffs, logs, prompts, terminal output, secrets, unsupported schema versions, or oversized payloads.
- Gates: answer whether a task has required plan, dispatch, impact, implementation, test, and detect-changes evidence before closure.
- Context bridge: remains absent until a real prompt needs it and G1/G3/G4 are already useful.

## Superseded Proposal Archive

The following options are retained as challenge history only. They are not parallel implementation choices; the accepted path is the consolidated solution above.

### Option 1 - Formalize Existing Runtime Agent Graph

Adopt the current `thread_spawn_edges` plus `AgentGraphStore` as the canonical runtime graph for spawned agents.

Implementation:

- Add this ADR as the decision record.
- Add a small follow-up task to document the owner boundary in `agent-graph-store` and `state`.
- Keep app-server/TUI consumers behind existing thread and multi-agent paths.

Pros:

- No risky code transfer because implementation is already present.
- Preserves current multi-agent resume/close/list behavior.
- Keeps static GitNexus code graph external.

Cons:

- Does not expose a new graph API yet.
- Leaves the `#[async_trait]` trait shape until a focused refactor.

### Option 2 - Refactor `AgentGraphStore` Before Expansion

Refactor the trait from `#[async_trait]` to RPITIT-style methods before making it more central.

Implementation:

- Change `AgentGraphStore` methods to return `impl Future<Output = AgentGraphStoreResult<_>> + Send`.
- Keep `LocalAgentGraphStore` behavior unchanged.
- Run `just test -p codex-agent-graph-store` and any affected core/state tests.

Pros:

- Aligns with project Rust trait rules.
- Low behavioral risk if scoped to this crate.
- Makes the seam safer before new consumers depend on it.

Cons:

- Touches a shared trait and requires GitNexus impact before editing.
- Adds little user-visible value by itself.

### Option 3 - Add Read-Only App-Server Graph View

Expose a redacted, bounded runtime graph view for clients that need to show agent trees.

Implementation:

- Add app-server v2 method only after separate API approval, for example `thread/graph/read`.
- Response includes thread IDs, parent IDs, edge status, agent path, nickname, and bounded status fields.
- No raw rollout payloads, transcripts, prompts, terminal output, credentials, or model context.

Pros:

- Useful for TUI/app-server visualizations and debugging.
- Reuses canonical graph state rather than reconstructing from rollouts.

Cons:

- Public API/schema work needs compatibility tests.
- Must define archive/resume/closed-edge behavior precisely.

### Option 4 - Use Rollout Trace As Diagnostic Graph Only

Extend support/doctor/debug flows to summarize rollout trace graph output.

Implementation:

- Keep `CODEX_ROLLOUT_TRACE_ROOT` opt-in.
- Add redacted reducer summaries and validation checks.
- Never upload or inject raw payloads into model context.

Pros:

- High debugging value.
- Preserves observe-first, reduce-later architecture.

Cons:

- Trace bundles are sensitive and require strict redaction tests.
- Not suitable as product state.

### Option 5 - Add GitNexus Static Graph Evidence Bridge

Do not embed GitNexus indexing. Instead, add a memory-bank/project-tooling convention for attaching GitNexus evidence to ADRs and task cards.

Implementation:

- Store bounded outputs: context target, impact risk, affected files/processes, detect-changes summary.
- Link to source owners.
- Keep raw graph details outside model context and source tree unless explicitly reviewed.

Pros:

- Leverages GitNexus without duplicating it.
- Matches current architecture reuse rules.

Cons:

- Not a runtime feature.
- Requires discipline in manager/subagent workflows.

### Option 6 - Operational Evidence Backbone

Make bounded GitNexus graph evidence, runtime graph summaries, and lean-ctx-inspired workflow evidence a durable Ontocode evidence layer.

Implementation:

- Add a state-owned `operational_evidence_records` table instead of overloading `stage1_outputs`.
- Store only compact facts: evidence domain, source kind, source ref, repo, task key, thread ID, symbol UID/name, file path, process label, gate name, risk level, bounded summary, source links, provenance hash, redaction status, created timestamp, and optional expiry.
- Add an internal writer that accepts GitNexus `context`, `impact`, `detect_changes`, runtime thread-graph summaries, repository script reports, and lean-ctx-inspired task/gate/readiness summaries after validation and redaction.
- Add bounded query helpers for manager/subagent workflows, keyed by task, thread, symbol, file, source owner, evidence domain, and risk.
- Add an optional `CodeGraphMemoryFragment` only after caps and exclusion rules are implemented.

Pros:

- Makes code-graph evidence reusable instead of ephemeral chat/task notes.
- Makes workflow gates and readiness evidence durable without copying lean-ctx runtime behavior.
- Keeps GitNexus as the static analysis owner while giving Ontocode durable graph memory.
- Reuses state/memory/context fragment architecture instead of building a parallel graph system.
- Gives subagents a safe backbone for impact/risk/source-owner facts.

Cons:

- Requires schema, retention, redaction, and import validation work.
- Needs careful boundaries so compact evidence does not become raw source storage.
- Model-visible use is high-touch because `ContextualUserFragment` has critical upstream impact.

### Option 7 - Unified Static + Runtime Graph In Core

Create one Ontocode-owned graph engine for code symbols, runtime threads, rollout traces, analytics, and identity.

Decision: reject.

Reasons:

- Duplicates GitNexus static code graph.
- Mixes product state, diagnostics, analytics, and identity.
- High migration/security risk.
- Likely creates unbounded context and support-bundle hazards.

### Option 8 - Vendor GitNexus/LadybugDB Into Ontocode

Add GitNexus analyzer storage and `@ladybugdb/core` as direct Ontocode dependencies so code-graph-memory can query the same graph database used by GitNexus.

Decision: reject.

Reasons:

- Introduces Node/native/platform dependency management into Rust runtime and memory paths.
- Couples Ontocode persistence to `.gitnexus/lbug` and GitNexus internal schema stability.
- Makes analyzer crashes or native package load failures affect core memory behavior.
- Recreates a static code graph runtime inside Ontocode, violating the owner boundary.
- Makes supply-chain review harder because GitNexus analyzer dependencies would become transitive product dependencies.

### Option 9 - Rust-Native GitNexus Concept Translation

Translate the useful GitNexus functionality into current Ontocode Rust solutions instead of bundling or invoking GitNexus runtime code.

Implementation:

- Define Rust structs for normalized `context`, `impact`, `detect_changes`, audit lifecycle, dispatch, scope-guard, and tombstone evidence.
- Store normalized facts in `operational_evidence_records` through `StateRuntime`.
- Add Rust validators for schema version, provenance, redaction, max-size limits, risk/status/domain enums, and stale target-head checks.
- Add Rust query helpers for task, thread, symbol, file, risk, gate, status, and evidence domain.
- Add Rust planned-versus-done and pre/post-edit gate helpers over the same records.
- Treat GitNexus TypeScript source as design evidence only. If a human/agent runs GitNexus externally, Ontocode may import the bounded JSON artifact through the Rust importer.
- Do not vendor, bundle, shell out to, or package GitNexus, LadybugDB, tree-sitter grammars, graphology, ONNX/transformers, Express, or MCP SDK in Ontocode.

Pros:

- Keeps the implementation in Rust and aligned with current Ontocode owners.
- Reuses existing SQLite state, `StateRuntime`, thread topology, memory-exclusion, and context-fragment boundaries.
- Avoids Node/native supply-chain and platform packaging risk.
- Lets Ontocode work without GitNexus installed.

Cons:

- Does not make Ontocode a static code graph analyzer.
- Requires explicit artifact handoff when external GitNexus evidence is desired.
- Some GitNexus report richness must be normalized or dropped to preserve bounds.

## Core Engine Integration Direction

"Integrate as core engine" means operational evidence becomes a core Ontocode capability. It does not mean GitNexus, LadybugDB, lean-ctx, or a second graph engine move into `ontocode-core`.

The supported integration shapes are:

- Minimal core backbone: add `operational_evidence_records` in state, with compact insert/query/prune helpers. This is the first implementation slice.
- Runtime graph evidence first: derive bounded topology evidence from existing `thread_spawn_edges` and `StateRuntime` graph methods.
- GitNexus artifact importer: ingest versioned `context`, `impact`, and `detect_changes` summaries from an external artifact, not from GitNexus storage internals.
- Internal query API: expose bounded internal helpers for manager/subagent workflows by task, thread, symbol, file, evidence domain, risk, and status.
- Gated context fragment: add a narrow `CodeGraphMemoryFragment` only after storage and query paths exist, with hard caps, redaction, opt-in, and memory-exclusion rules.
- Rust-native GitNexus concept translation: implement storage, validation, import, query, audit, and gates in Rust; external GitNexus output is optional design/evidence input only.

Rejected core-engine interpretations:

- Do not embed GitNexus or LadybugDB into Ontocode runtime crates.
- Do not parse `.gitnexus/lbug` from Ontocode.
- Do not add a static code graph indexer to core.
- Do not expose app-server, SDK, or TUI graph APIs from this ADR.
- Do not inject graph memory into prompts by default.
- Do not clone lean-ctx read/search/shell/cache/session behavior.

Forgotten / out of scope for this ADR:

- GitNexus runtime adoption: do not import GitNexus code, LadybugDB, Node tooling, MCP server code, tree-sitter graph pipeline, or analyzer storage.
- Hermetic evidence binary: do not build `ontocode-codegraph-evidence` or any bundled analyzer binary.
- Static code graph indexing in Ontocode: do not rebuild GitNexus in Rust, add a symbol graph DB, parse `.gitnexus/lbug`, or create a static analyzer pipeline.
- App-server/TUI graph APIs: keep public graph and audit APIs out of this ADR.
- Default prompt/context injection: no automatic `CodeGraphMemoryFragment`; keep model-visible evidence gated and future-only.
- `AgentGraphStore` promotion: do not make it central; current owner remains `StateRuntime` plus `thread_spawn_edges`.
- GitNexus audit-event-store clone: do not copy `.gitnexus/audit`; store compact Rust evidence records only.
- Raw evidence storage: do not store raw diffs, logs, source, prompts, graph rows, transcripts, terminal output, or full tool outputs.
- Lean-ctx behavior: do not clone shell/read/search/cache/session mechanics.
- Version-specific GitNexus coupling: do not depend on `gitnexus@1.6.2` behavior as a compatibility contract; the Rust artifact schema is the contract.

What remains in scope:

- Rust `operational_evidence_records`.
- Runtime topology ingestion from `StateRuntime`.
- Internal bounded query helpers.
- Rust artifact validators/importers.
- Planned-versus-done gates.
- Optional context fragment only after G1/G3/G4 prove useful.

Recommended implementation order:

1. Implement `G1` operational evidence state.
2. Implement `G3` runtime thread-topology ingestion.
3. Implement `G4` internal bounded query helpers.
4. Implement the `G2` GitNexus artifact importer.
5. Implement `G2b` workflow evidence import only as bounded records.
6. Implement `G5` only if a concrete manager/subagent prompt needs model-visible evidence.
7. Keep GitNexus runtime/binary packaging out of scope unless a future ADR reverses the Rust-only decision.

## Technical Implementation Contract

State ownership:

- Store operational evidence in the main state DB (`state_5.sqlite`), not in `memories_1.sqlite`.
- Add the first migration as `ontocode-rs/state/migrations/0036_operational_evidence_records.sql`.
- Do not add this table to `ontocode-rs/state/memory_migrations/`; `stage1_outputs` and memory jobs are rollout-memory specific, and the main state migration `0035_drop_memory_tables.sql` already removed old memory tables from the state DB.
- Add models in `ontocode-rs/state/src/model/operational_evidence.rs`.
- Add runtime methods in `ontocode-rs/state/src/runtime/operational_evidence.rs`.
- Export narrowly through `ontocode-rs/state/src/model/mod.rs`, `ontocode-rs/state/src/runtime.rs`, and `ontocode-rs/state/src/lib.rs`.
- Add methods on `StateRuntime`; do not introduce a new public trait for one implementation.

Initial SQLite shape:

```sql
CREATE TABLE operational_evidence_records (
    id TEXT PRIMARY KEY,
    evidence_domain TEXT NOT NULL,
    source_tool TEXT NOT NULL,
    source_version TEXT,
    schema_version INTEGER NOT NULL,
    source_ref TEXT,
    repo TEXT,
    task_key TEXT,
    thread_id TEXT,
    parent_thread_id TEXT,
    child_thread_id TEXT,
    symbol_uid TEXT,
    symbol_name TEXT,
    file_path TEXT,
    process_label TEXT,
    gate_name TEXT,
    risk TEXT,
    status TEXT NOT NULL,
    summary TEXT NOT NULL CHECK (length(summary) <= 8192),
    source_links_json TEXT NOT NULL DEFAULT '[]' CHECK (length(source_links_json) <= 16384),
    metadata_json TEXT NOT NULL DEFAULT '{}' CHECK (length(metadata_json) <= 16384),
    provenance_hash TEXT NOT NULL UNIQUE,
    redaction_status TEXT NOT NULL,
    target_head TEXT,
    graph_index_id TEXT,
    plan_hash TEXT,
    tracking_hash TEXT,
    created_at INTEGER NOT NULL,
    expires_at INTEGER
);
```

Required indexes:

- `idx_operational_evidence_task_status` on `(task_key, status, created_at DESC)`
- `idx_operational_evidence_thread` on `(thread_id, created_at DESC)`
- `idx_operational_evidence_symbol` on `(symbol_uid, created_at DESC)`
- `idx_operational_evidence_file` on `(file_path, created_at DESC)`
- `idx_operational_evidence_domain_risk` on `(evidence_domain, risk, created_at DESC)`
- `idx_operational_evidence_target_head` on `(target_head, created_at DESC)`
- `idx_operational_evidence_expires_at` on `(expires_at)`

Rust model enums:

- `EvidenceDomain`: `CodeGraph`, `Workflow`, `Test`, `Doc`, `Redaction`, `Architecture`, `RuntimeTopology`
- `EvidenceStatus`: `Planned`, `Dispatched`, `Implemented`, `Verified`, `Stale`, `Blocked`, `Done`, `Rejected`
- `EvidenceRisk`: `None`, `Low`, `Medium`, `High`, `Critical`, `Unknown`
- `RedactionStatus`: `Clean`, `Redacted`, `Rejected`

Initial `StateRuntime` methods:

- `insert_operational_evidence(record)`
- `upsert_operational_evidence_by_provenance(record)`
- `query_operational_evidence(query)`
- `prune_operational_evidence(now)`

Query options must include optional filters for task key, thread id, symbol uid, file path, evidence domain, gate name, status, risk, target head, and freshness. Query responses must enforce both max-record and max-byte caps.

GitNexus artifact shape:

```json
{
  "schemaVersion": 1,
  "sourceTool": "gitnexus",
  "sourceVersion": "1.6.2",
  "repo": "/path/to/repo",
  "targetHead": "git-sha",
  "graphIndexId": "opaque-index-id",
  "createdAt": 1234567890,
  "records": []
}
```

GitNexus normalization rules:

- `context` artifacts may store symbol uid/name/kind/file path, incoming/outgoing relationship counts, process labels, and concept names. Reject `include_content: true` outputs and any raw source body.
- `impact` artifacts may store target, direction, impacted count, risk, direct count, process count, module count, affected process labels, affected module names, partial marker, and warnings.
- `detect_changes` artifacts may store changed file count, changed symbols, affected process labels, risk level, scope, base ref, and warnings. Reject raw diffs.
- GitNexus audit lifecycle artifacts may store session id, target repo/head, graph index id, source hash, verifier version, status, finding ids, bundle ids, reason codes, and compact evidence metadata. Do not copy `.gitnexus/audit` event-store files into Ontocode state.
- Compute `provenance_hash` from source tool, source version, evidence kind, target head, task/symbol/file identity, and normalized payload.
- Reject artifacts with unsupported schema version, missing provenance, oversized summaries, oversized metadata, unredacted secrets, raw graph rows, raw logs, raw prompts, raw terminal output, or raw source.

## Code Development Integration

For day-to-day code development, this ADR should surface as developer evidence and manager gates, not as a new graph runtime.

Supported development integrations:

- Task evidence ledger: store impact reports, affected files, required tests, verification results, and closure status under a task key.
- Pre-edit impact gate: require bounded `context` or `impact` evidence before editing a symbol, then persist risk, callers, affected processes, and source links.
- Post-edit scope gate: persist `detect_changes` or diff-impact evidence after edits, including unexpected files, unexpected symbols, changed flows, and missing tests.
- Task-aware query helpers: allow internal manager/subagent code to query evidence by task, thread, symbol, file, risk, status, and evidence domain.
- Runtime agent topology evidence: derive compact parent/child ownership and open/closed status from `thread_spawn_edges` so dispatch recovery can answer who worked on what.
- Optional prompt fragment: inject only bounded task risk, affected files, required tests, and prior verification results after `G1`/`G4` exist and `G5` gates pass.

Rejected development shortcuts:

- Do not let subagents scrape raw GitNexus, OntoIndex, or lean-ctx output into prompts.
- Do not close a task from chat-only claims when required impact or verification evidence is missing.
- Do not add app-server/TUI graph APIs just to support manager workflows.
- Do not use operational evidence as a raw log store, transcript store, terminal-output archive, or source-code cache.

Development rollout follows the consolidated G-plan: land `G1`, then `G3`, `G4`, `G2`, `G2b`, and only then consider `G5`.

## Planned-Versus-Done Audit Integration

For auditing what was planned and what was actually done, operational evidence should act as a compact audit ledger. It must compare stated plan items, dispatch records, implementation evidence, test evidence, and closure decisions without becoming a raw transcript or build-log archive.

Supported audit integrations:

- Plan-item ledger: import each approved ADR/project-plan task as a bounded evidence record with task key, owner, expected files/symbols, required tests, status, and source plan link.
- Dispatch evidence: record manager/subagent assignment, parent thread, child thread, model request when known, start time, claimed scope, and expected verification gates.
- Completion evidence: store the worker-reported changed files, changed symbols, tests run, verification status, residual risks, and source links.
- Plan drift audit: compare planned scope against actual diff evidence and flag unexpected files, missing expected files, missing tests, unclosed blockers, and duplicate or stale task claims.
- Evidence freshness audit: mark evidence stale when the target HEAD, plan file, tracking file, or affected symbol changes after verification.
- Closure gate: allow `done` only when required impact, implementation, test, and detect-changes evidence exists for the task key, or when the plan explicitly records a no-code/documentation-only closure.
- Rollup summaries: generate bounded counts for planned, in-progress, blocked, verified, stale, and done tasks by ADR/project plan.

Rejected audit shortcuts:

- Do not infer completion from chat text alone.
- Do not store raw build logs, full diffs, transcripts, prompts, credentials, or terminal output as audit evidence.
- Do not mutate tracking files silently from imported evidence; manager-facing updates must be explicit.
- Do not create a second audit database when `operational_evidence_records` can hold compact records.
- Do not expose public app-server/TUI audit APIs from this ADR.

Audit rollout uses the same G-plan. Plan, dispatch, completion, freshness, closure, and rollup records are all `operational_evidence_records`, not a separate audit store.

## Decision

Adopt the consolidated robust solution: make operational evidence a bounded Rust state-backed ledger, treat code-graph-memory as one evidence domain, translate external GitNexus/OntoIndex-style report concepts into Rust records/importers/queries/gates, and use existing `StateRuntime`/`thread_spawn_edges` behavior as the canonical persisted spawned-agent topology input.

Option 2 is no longer an automatic next step. Refactor `AgentGraphStore` only if a separate cleanup task proves it is worth touching a currently low-use trait.

Option 5 is accepted only as an ingestion convention into operational evidence records. It must not import the GitNexus graph DB or run static indexing in Ontocode core.

Consider Option 3 and Option 4 only after concrete UI/debugging requirements and a separate app-server/diagnostics ADR.

Do not implement Option 7, Option 8, or any bundled GitNexus/LadybugDB/Node evidence binary.

## Implementation Plan

### G0 - ADR And Backbone Ownership

Status: accepted by this ADR.

Tasks:

- Keep this ADR as the graph adoption decision record.
- Keep this ADR as the canonical third-party dependency consolidation record for GitNexus and lean-ctx derived work.
- Declare operational evidence as an internal backbone layer, not a public graph API or workflow API.
- Treat code-graph-memory as the `code_graph` domain inside operational evidence.
- Keep static symbol/call graph analysis externally owned by GitNexus.
- Keep lean-ctx read/search/shell/session behavior externally owned by lean-ctx.
- Confirm `thread_spawn_edges` remains the canonical durable spawned-agent topology.

Acceptance:

- No runtime code changes required.
- No duplicate graph store, indexer, or app-server API.
- No duplicate lean-ctx runtime, cache, session store, shell wrapper, or tool registry.
- Memory-bank index points to this ADR as the third-party dependency consolidation and evidence-backbone record.

### G1 - Operational Evidence State Model

Status: accepted for implementation planning.

Tasks:

- Add a state-owned `operational_evidence_records` table in the main state DB migration stream as `ontocode-rs/state/migrations/0036_operational_evidence_records.sql`; do not add it to `memory_migrations`.
- Define the model and indexes exactly as specified in the Technical Implementation Contract.
- Supported initial domains are `code_graph`, `workflow`, `test`, `doc`, `redaction`, `architecture`, and `runtime_topology`.
- Store compact summaries only; never store raw source bodies, raw prompts, credentials, terminal output, GitNexus graph rows, lean-ctx cache/session data, or full tool output.
- Add the task/thread/symbol/file/domain/risk/status indexes from the Technical Implementation Contract.

Acceptance:

- State migration is backward-compatible.
- Tests cover insert, upsert by provenance hash, bounded summary size, expiry/pruning, domain filtering, and stable ordering.
- Redaction tests fail if token, cookie, authorization header, keychain path, or raw credential value appears in a record.
- Dependency tests fail if GitNexus analyzer packages, LadybugDB packages, or lean-ctx runtime packages are added to Ontocode runtime/core/app-server/SDK manifests.

### G2 - Rust Evidence Artifact Importer

Status: accepted for implementation planning after G1, G3, and G4.

Tasks:

- Define the versioned bounded JSON artifact contract before importing anything into state.
- Implement a Rust importer that reads explicit artifact files matching the Technical Implementation Contract.
- Add Rust normalization code for bounded GitNexus `context`, `impact`, `detect_changes`, and audit lifecycle summaries.
- Normalize evidence into the state model without importing GitNexus graph storage.
- Preserve source links and risk summaries so later agents can reason from provenance.
- Reject oversized or unredacted payloads before persistence.
- Do not link to, load, or parse LadybugDB directly from Ontocode runtime code.
- Do not shell out to raw GitNexus internals or any GitNexus-owned binary from the core request path.
- Do not parse `.gitnexus/lbug` from Ontocode.
- Treat GitNexus analyzer version, `@ladybugdb/core` version, and evidence schema version as provenance fields only.
- Keep external-tool installation/update/checksum/SBOM/license data outside the operational evidence record model, except for compact provenance fields when an artifact supplies them.

Acceptance:

- Import fixtures cover low, high, and critical impact reports.
- Importer rejects raw code blocks and graph dumps.
- Importer is usable by manager/subagent workflows without changing production request paths.
- Importer fails closed on unsupported evidence schema versions or missing provenance.
- Missing external GitNexus artifacts degrade to "no graph evidence" without breaking normal Ontocode runtime behavior.
- Dependency tests assert no `@ladybugdb/core`, `gitnexus`, tree-sitter, graphology, ONNX, transformers, Express, or MCP SDK packages are introduced into Ontocode runtime/core/app-server/SDK package manifests by this work.
- Tests assert the importer does not execute external commands.

### G2b - Lean-ctx-Inspired Workflow Evidence Import

Status: accepted for implementation planning after G1.

Tasks:

- Import only bounded outputs from repository scripts or external lean-ctx-assisted workflows.
- Normalize task cards, gate results, doc-link reports, test summaries, redaction reports, and readiness summaries into `operational_evidence_records`.
- Treat lean-ctx tool names and versions as provenance fields only.
- Do not invoke lean-ctx from the production request path.
- Do not persist lean-ctx cache/session data or compressed shell/read/search output bodies.

Acceptance:

- Fixture tests cover task-card, gate-result, doc-link, redaction, and readiness records.
- Oversized output and secret-bearing output are rejected before persistence.
- Missing lean-ctx degrades to "no workflow evidence" without affecting normal Ontocode behavior.
- No lean-ctx package, binary, MCP tool, or runtime cache is required by Ontocode core/app-server/SDK manifests.

### G3 - Runtime Graph Memory Ingestion

Status: accepted for implementation planning.

Tasks:

- Add bounded summaries of runtime thread topology from `StateRuntime` and existing core merge paths.
- Include parent/child thread IDs, edge status, agent path, and source timestamp only.
- Keep rollout-trace reduced graphs diagnostic-only unless a diagnostics ADR approves summary import.

Acceptance:

- Tests cover open/closed descendants, stale source metadata, duplicate live/persisted edges, and deterministic ordering.
- No transcript, prompt, terminal output, or raw rollout payload is stored.

### G4 - Backbone Query API

Status: accepted as internal-only.

Tasks:

- Add internal read helpers for task, thread, symbol, file, owner, evidence domain, gate, status, and risk.
- Return bounded summaries with source links and provenance, not raw evidence.
- Keep app-server, SDK, schema, and TUI exposure blocked until a separate API ADR.

Acceptance:

- Query output has deterministic ordering and explicit max-item/max-byte caps.
- No public API surface is created.
- Callers can distinguish GitNexus static evidence, runtime topology evidence, and lean-ctx-inspired workflow evidence.

### G5 - Optional Context Fragment Bridge

Status: gated.

Tasks:

- Add a narrow `CodeGraphMemoryFragment` only if a concrete manager/subagent prompt needs model-visible graph memory.
- Implement it through existing `ContextualUserFragment` without changing the trait.
- Add memory-exclusion handling so injected operational evidence does not become rollout user memory.
- Mark memory mode polluted only when operational evidence is injected into the model turn.

Acceptance:

- Fragment has hard caps below the project context limits.
- Tests cover classification, redaction, truncation, and no accidental memory learning.
- GitNexus impact is rerun before editing any context-fragment or memory-exclusion symbol.

### G6 - Deferred Graph API And Diagnostics

Status: blocked pending separate ADRs.

Tasks:

- Keep app-server/TUI graph APIs blocked until wire shape, docs, schema, compatibility tests, and UI snapshots are approved.
- Keep rollout-trace diagnostic summaries blocked until redaction/support-bundle rules are approved.
- Keep `AgentGraphStore` trait cleanup deferred until a real consumer or cleanup task exists.

Acceptance:

- No public graph API, SDK behavior, schema, dashboard, wizard, support bundle, or export path is added by this ADR.
- No `AgentGraphStore` promotion happens as part of code-graph-memory backbone work.

## Risks And Constraints

- `AgentGraphStore` currently uses `#[async_trait]`, which violates the preferred trait shape for new or promoted traits.
- App-server graph exposure is public API work and requires separate schema/docs/tests.
- Rollout traces contain sensitive raw evidence and must never be model-visible by default.
- Static GitNexus code graph data must stay external; Ontocode stores only compact evidence records.
- Operational evidence state must not become an analytics store, credential store, raw source store, raw tool-output store, or default model/context injection path.
- Operational evidence context injection has critical blast radius because it touches the shared contextual-fragment path.
- GitNexus analyzer dependencies include native/platform packages and ML/parser packages; they are not allowed in Ontocode runtime crates, app-server processes, SDKs, or Ontocode-owned binaries for this ADR.
- Lean-ctx dependencies and runtime behavior are allowed only as external development workflow tooling, not as Ontocode runtime, app-server, SDK, or persistence dependencies.
- LadybugDB storage files such as `.gitnexus/lbug` are implementation details and must never become an Ontocode persistence or compatibility contract.
- Semver ranges in the external analyzer package mean GitNexus storage behavior can change independently of Ontocode; the evidence artifact schema must be versioned and validated.
- External workflow tools must fail closed and be optional for normal runtime behavior; operational evidence cannot make sessions depend on GitNexus, analyzer, or lean-ctx availability.

## Final Recommendation

Adopt operational evidence as the Ontocode backbone layer, with code-graph-memory and lean-ctx-inspired workflow evidence as domains inside one bounded durable record model. GitNexus is source evidence and an optional external developer tool, not an Ontocode runtime, binary, dependency, or storage engine. Ontocode implements the adopted functionality in Rust: state records, validators, importers, query helpers, runtime topology summaries, planned-versus-done audits, and optional bounded context fragments. The next engineering step is `G1`: add the unified state model in the main state DB and tests for compact GitNexus/runtime/workflow facts. Then implement `G3` runtime topology ingestion and `G4` bounded internal queries. After the state/query layer exists, `G2` must define the Rust artifact importer, import only versioned evidence artifacts, and fail closed on schema/provenance/redaction drift. `G2b` may import lean-ctx-inspired workflow evidence only as bounded records, not as lean-ctx runtime behavior. Do not parse `.gitnexus/lbug`, do not add analyzer or lean-ctx packages to Ontocode runtime/core/app-server/SDK manifests, do not bundle GitNexus/LadybugDB/Node tooling into an Ontocode-owned binary, do not promote `AgentGraphStore`, and do not expose app-server/TUI graph APIs until separate ADRs approve those surfaces.
