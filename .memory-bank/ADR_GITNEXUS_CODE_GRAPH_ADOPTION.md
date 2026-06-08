# ADR: GitNexus Code-Graph Adoption And Core Decisions

## Status

Challenged - consolidate GitNexus and lean-ctx into one operational evidence backbone; third-party tooling stays behind local evidence/tooling boundaries, not core runtime.

## Date

2026-06-07

## Context

The local `../GitNexus` workspace contains graph-oriented functionality that is useful for Ontocode:

- a persisted runtime graph for spawned agent threads
- deterministic graph traversal through state-backed edges
- rollout trace bundles reduced into semantic runtime graphs
- interaction edges between agents, tools, code cells, terminal operations, inference, and compaction
- analytics facts for subagent/thread relationships
- agent identity primitives

Important finding: the core graph implementation files checked below are already byte-identical between local `../GitNexus` and current Ontocode. The immediate task is therefore not to copy code. The safe decision is to formalize the existing persisted `thread_spawn_edges` runtime graph as one input into a broader code-graph-memory backbone.

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

- Current npm metadata checked on 2026-06-08 shows `gitnexus@1.6.5` depends on `@ladybugdb/core` through the semver range `^0.16.1`, plus native/platform optional packages such as `@ladybugdb/core-linux-x64`, `@ladybugdb/core-darwin-arm64`, and `@ladybugdb/core-win32-x64`.
- The unscoped `ladybugdb` package name is not present in npm; the relevant package is `@ladybugdb/core`.
- `@ladybugdb/core` is a native in-process property graph database dependency with its own transitive dependencies and platform binaries. It is acceptable inside a hermetic local evidence binary, but not acceptable as a hidden Ontocode runtime, memory, state, app-server, or context dependency.
- Other GitNexus analyzer dependencies are acceptable only behind the same binary boundary: tree-sitter grammars, graphology packages, `onnxruntime-node`, `@huggingface/transformers`, Express, and the MCP SDK. Code-graph-memory must not make Ontocode crates, app-server, SDKs, or memory/context paths transitively depend on these packages.
- The earlier challenge rejected third-party dependency adoption too broadly. Corrected position: bundle all analyzer third-party dependencies into a local per-platform evidence binary, then let Ontocode consume only the bounded evidence artifact emitted by that binary.
- Lean-ctx is a development/workflow tool only. Ontocode must not vendor lean-ctx, depend on its CLI/runtime, copy its shell/read/search/session cache, or expose lean-ctx tools as model-visible product tools.
- Third-party tooling is consolidated into three allowed classes: binary-owned analyzer dependencies, repository-only scripts with no runtime dependency, and external developer tools used by agents outside Ontocode runtime.

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
| GitNexus analyzer and `@ladybugdb/core` | Produce static graph evidence. | Hermetic local evidence binary emitting versioned bounded artifacts. | Direct Rust/app-server/SDK dependency, direct `.gitnexus/lbug` parsing, in-process LadybugDB store. |
| GitNexus MCP/CLI | Development-time evidence collection and manager workflow support. | External tool invocation or evidence artifact import. | Production request-path dependency or persisted raw graph output. |
| Lean-ctx MCP/CLI | Agent development workflow for compressed reads, shell output, search, and session handling. | External agent workflow only. | Vendored runtime, product CLI dependency, copied cache/session/search/shell subsystem. |
| Repository-only scripts | Bootstrap reports, link checks, status counts, and task-card generation. | `scripts/` or memory-bank tooling with standard-library-first implementation. | Runtime crate dependency, app-server API, model-visible tool registration, automatic silent status mutation. |
| Operational evidence backbone | Durable local facts, gates, and readiness summaries derived from approved inputs. | Existing `StateRuntime`/state ownership, bounded records, redaction, retention, provenance. | Separate database root, raw logs/source/secrets, second memory store, third-party graph/search/runtime library. |
| Model-visible summaries | Optional bounded context after separate approval. | Existing `ContextualUserFragment` path with hard caps and memory-exclusion handling. | Side-channel context injection or unbounded tool output. |

Consolidation rules:

- One backbone schema owns both GitNexus code-graph evidence and lean-ctx-inspired operational evidence.
- GitNexus evidence is a `code_graph` evidence domain, not a separate storage engine.
- Lean-ctx-inspired task/gate/readiness records are `workflow`, `test`, `doc`, `redaction`, or `architecture` evidence domains, not a copied lean-ctx runtime.
- Every imported artifact must carry source tool name, tool version when known, schema version, provenance hash, created timestamp, redaction status, and max-size validation result.
- Normal Ontocode runtime must still work when GitNexus, lean-ctx, or the local evidence binary is absent.

## Source Evidence

Local GitNexus implementation:

- [agent-graph-store trait](/opt/demodb/_workfolder/GitNexus/codex-rs/agent-graph-store/src/store.rs:1)
- [agent-graph-store SQLite adapter](/opt/demodb/_workfolder/GitNexus/codex-rs/agent-graph-store/src/local.rs:1)
- [agent graph edge status](/opt/demodb/_workfolder/GitNexus/codex-rs/agent-graph-store/src/types.rs:1)
- [state graph model](/opt/demodb/_workfolder/GitNexus/codex-rs/state/src/model/graph.rs:1)
- [thread spawn edge migration](/opt/demodb/_workfolder/GitNexus/codex-rs/state/migrations/0021_thread_spawn_edges.sql:1)
- [state thread graph methods](/opt/demodb/_workfolder/GitNexus/codex-rs/state/src/runtime/threads.rs:78)
- [runtime subtree merge in thread manager](/opt/demodb/_workfolder/GitNexus/codex-rs/core/src/thread_manager.rs:508)
- [agent resume/close graph use](/opt/demodb/_workfolder/GitNexus/codex-rs/core/src/agent/control.rs:523)
- [rollout-trace design](/opt/demodb/_workfolder/GitNexus/codex-rs/rollout-trace/README.md:1)
- [ThreadTraceContext](/opt/demodb/_workfolder/GitNexus/codex-rs/rollout-trace/src/thread.rs:76)
- [RolloutTrace model](/opt/demodb/_workfolder/GitNexus/codex-rs/rollout-trace/src/model/mod.rs:54)
- [InteractionEdge model](/opt/demodb/_workfolder/GitNexus/codex-rs/rollout-trace/src/model/runtime.rs:305)
- [agent interaction reducer](/opt/demodb/_workfolder/GitNexus/codex-rs/rollout-trace/src/reducer/tool/agents.rs:16)
- [subagent analytics fact](/opt/demodb/_workfolder/GitNexus/codex-rs/analytics/src/facts.rs:347)
- [agent identity primitives](/opt/demodb/_workfolder/GitNexus/codex-rs/agent-identity/src/lib.rs:40)

Current Ontocode equivalents:

- [agent-graph-store trait](/opt/demodb/_workfolder/ontocode/codex-rs/agent-graph-store/src/store.rs:1)
- [agent-graph-store SQLite adapter](/opt/demodb/_workfolder/ontocode/codex-rs/agent-graph-store/src/local.rs:1)
- [agent graph edge status](/opt/demodb/_workfolder/ontocode/codex-rs/agent-graph-store/src/types.rs:1)
- [state graph model](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/model/graph.rs:1)
- [thread spawn edge migration](/opt/demodb/_workfolder/ontocode/codex-rs/state/migrations/0021_thread_spawn_edges.sql:1)
- [state thread graph methods](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime/threads.rs:78)
- [runtime subtree merge in thread manager](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/thread_manager.rs:508)
- [agent resume/close graph use](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/agent/control.rs:523)
- [rollout-trace design](/opt/demodb/_workfolder/ontocode/codex-rs/rollout-trace/README.md:1)
- [ThreadTraceContext](/opt/demodb/_workfolder/ontocode/codex-rs/rollout-trace/src/thread.rs:76)
- [RolloutTrace model](/opt/demodb/_workfolder/ontocode/codex-rs/rollout-trace/src/model/mod.rs:54)
- [InteractionEdge model](/opt/demodb/_workfolder/ontocode/codex-rs/rollout-trace/src/model/runtime.rs:305)
- [agent interaction reducer](/opt/demodb/_workfolder/ontocode/codex-rs/rollout-trace/src/reducer/tool/agents.rs:16)
- [subagent analytics fact](/opt/demodb/_workfolder/ontocode/codex-rs/analytics/src/facts.rs:347)
- [agent identity primitives](/opt/demodb/_workfolder/ontocode/codex-rs/agent-identity/src/lib.rs:40)

Current Ontocode memory/context backbone evidence:

- [lean-ctx project tool ADR](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1)
- [memory pipeline owner docs](/opt/demodb/_workfolder/ontocode/codex-rs/memories/README.md:1)
- [memory state migration](/opt/demodb/_workfolder/ontocode/codex-rs/state/memory_migrations/0001_memories.sql:1)
- [MemoryStore owner](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime/memories.rs:27)
- [Stage1Output model](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/model/memories.rs:1)
- [memory exclusion for contextual fragments](/opt/demodb/_workfolder/ontocode/codex-rs/memories/write/src/phase1.rs:457)
- [ContextualUserFragment trait](/opt/demodb/_workfolder/ontocode/codex-rs/context-fragments/src/fragment.rs:45)
- [contextual-user fragment classifier](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/context/contextual_user_message.rs:1)
- [rollout memory pollution bridge](/opt/demodb/_workfolder/ontocode/codex-rs/rollout/src/state_db.rs:457)

GitNexus challenge evidence captured for this ADR:

- `ContextualUserFragment` impact: `CRITICAL`, 29 direct implementers, 38 total impacted symbols.
- `mark_thread_memory_mode_polluted` impact: `HIGH`, direct callers in MCP tool-call and stream-event external-context paths.
- `MemoryStore` struct impact: `LOW`, but existing state memory schema is rollout-memory specific.
- `gitnexus@1.6.5` npm metadata: depends on `@ladybugdb/core` `^0.16.1` and many analyzer/runtime packages; those packages are accepted only inside the local evidence binary, not as Ontocode runtime/core dependencies.
- `@ladybugdb/core` npm metadata: latest checked version `0.17.1`, MIT license, native optional platform packages, repository `github.com/LadybugDB/ladybug`.
- Local evidence binary decision: third-party analyzer dependencies may be bundled into a signed/checksummed local executable package. They must not appear as direct dependencies of Ontocode Rust crates, app-server packages, SDKs, or persisted state APIs.

## Functionality Comparison

| Functionality | Local `../GitNexus` implementation | Current Ontocode status | Core decision |
|---|---|---|---|
| Storage-neutral spawned-thread graph trait | `AgentGraphStore` defines upsert, close-status update, direct child listing, and descendant listing in [store.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/agent-graph-store/src/store.rs:12). | Same file exists in Ontocode at [store.rs](/opt/demodb/_workfolder/ontocode/codex-rs/agent-graph-store/src/store.rs:12), but GitNexus shows no execution-flow consumers beyond its local implementation. | Do not promote this as canonical yet. Keep as internal candidate; refactor away `#[async_trait]` only if a real consumer or cleanup task is accepted. |
| SQLite-backed graph adapter | `LocalAgentGraphStore` wraps `StateRuntime` in [local.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/agent-graph-store/src/local.rs:13). | Same adapter exists in Ontocode at [local.rs](/opt/demodb/_workfolder/ontocode/codex-rs/agent-graph-store/src/local.rs:13). | Keep adapter thin and inert. Do not add graph storage outside `codex-state`, and do not route production behavior through this trait without impact review. |
| Edge lifecycle status | `ThreadSpawnEdgeStatus` supports `open` and `closed` in [types.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/agent-graph-store/src/types.rs:5). | Same type exists in Ontocode at [types.rs](/opt/demodb/_workfolder/ontocode/codex-rs/agent-graph-store/src/types.rs:5). | Keep lifecycle enum narrow until concrete behavior needs more states. Do not infer runtime status from process liveness alone. |
| Persisted edge table | `thread_spawn_edges(parent_thread_id, child_thread_id primary key, status)` in [0021_thread_spawn_edges.sql](/opt/demodb/_workfolder/GitNexus/codex-rs/state/migrations/0021_thread_spawn_edges.sql:1). | Same migration exists in Ontocode at [0021_thread_spawn_edges.sql](/opt/demodb/_workfolder/ontocode/codex-rs/state/migrations/0021_thread_spawn_edges.sql:1). | This table is the canonical durable parent-child topology, not transcript metadata. |
| State enum backing DB status | `DirectionalThreadSpawnEdgeStatus` uses snake-case string serialization through `strum` in [graph.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/state/src/model/graph.rs:8). | Same model exists in Ontocode at [graph.rs](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/model/graph.rs:8). | Keep DB status and public graph-store status separate so storage can evolve without leaking DB details. |
| Upsert edge behavior | State runtime inserts or replaces child incoming edge in [threads.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/state/src/runtime/threads.rs:78). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime/threads.rs:78). | Child thread has one persisted parent. Re-parenting is allowed only through explicit upsert behavior. |
| Close edge behavior | `set_thread_spawn_edge_status` updates missing child as successful no-op in [threads.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/state/src/runtime/threads.rs:105). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime/threads.rs:105). | Close is graph lifecycle state, not deletion. Preserve closed edges for resume/history. |
| Direct child listing | `list_thread_spawn_children(_with_status)` returns direct children with stable ordering in [threads.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/state/src/runtime/threads.rs:118). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime/threads.rs:118). | Use direct listing for list-agents and targeted child operations. |
| Descendant traversal | Recursive SQL lists descendants breadth-first by depth and thread id in [threads.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/state/src/runtime/threads.rs:137). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime/threads.rs:137). | Use persisted descendant traversal for resume/close/history. Avoid ad hoc in-memory tree reconstruction when DB is available. |
| Path-based child lookup | Direct and descendant path lookup joins `threads.agent_path` in [threads.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/state/src/runtime/threads.rs:161). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime/threads.rs:161). | Use canonical agent path lookups for v2 task-name targeting. Do not add a second name index. |
| Backfill from session source | State runtime extracts parent thread from `SessionSource` in [threads.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/state/src/runtime/threads.rs:315). | Same behavior exists in Ontocode at [threads.rs](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime/threads.rs:315). | Keep backfill best-effort. Do not trust stale metadata over persisted edges when both exist. |
| Thread manager subtree merge | `list_agent_subtree_thread_ids` merges persisted descendants and live in-memory children in [thread_manager.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/core/src/thread_manager.rs:508). | Same behavior exists in Ontocode at [thread_manager.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/thread_manager.rs:508). | This is the correct bridge: persisted graph first, live graph second, deduped deterministically. |
| Resume open descendants | `resume_agent_from_rollout` walks open persisted children breadth-first in [control.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/core/src/agent/control.rs:523). | Same behavior exists in Ontocode at [control.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/agent/control.rs:523). | Resume should use edge data as source of truth for open descendants. |
| Close persisted edge | `close_agent` marks the edge closed before shutdown in [control.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/core/src/agent/control.rs:798). | Same behavior exists in Ontocode at [control.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/agent/control.rs:798). | Preserve close idempotency and stale-agent handling. |
| Trace bundle strategy | GitNexus rollout trace observes raw events first and reduces later, documented in [README.md](/opt/demodb/_workfolder/GitNexus/codex-rs/rollout-trace/README.md:11). | Same diagnostic architecture exists in Ontocode at [README.md](/opt/demodb/_workfolder/ontocode/codex-rs/rollout-trace/README.md:11). | Adopt offline reduction as the only approved runtime graph diagnostics path. Do not build reduced graphs on hot path. |
| Thread trace context | `ThreadTraceContext` is no-op capable and root/child aware in [thread.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/rollout-trace/src/thread.rs:76). | Same context exists in Ontocode at [thread.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rollout-trace/src/thread.rs:76). | Keep tracing opt-in and best-effort. Trace failures must never fail sessions. |
| Reduced rollout graph | `RolloutTrace` separates threads, turns, conversation, inference, code cells, tools, terminals, compactions, edges, and raw payload refs in [model/mod.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/rollout-trace/src/model/mod.rs:54). | Same model exists in Ontocode at [model/mod.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rollout-trace/src/model/mod.rs:54). | Keep diagnostic graph separate from product state and model-visible context. |
| Information-flow edges | `InteractionEdge` models runtime information flow in [runtime.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/rollout-trace/src/model/runtime.rs:305). | Same model exists in Ontocode at [runtime.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rollout-trace/src/model/runtime.rs:305). | Use edges only in trace/debug surfaces unless a separate app-server ADR approves a redacted view. |
| Multi-agent trace reducer | Agent reducer resolves spawn/send/close/result edges and fallbacks in [agents.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/rollout-trace/src/reducer/tool/agents.rs:16). | Same reducer exists in Ontocode at [agents.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rollout-trace/src/reducer/tool/agents.rs:16). | Reuse reducer for diagnostics. Do not duplicate graph inference in app-server or TUI. |
| Subagent analytics | `SubAgentThreadStartedInput` captures subagent thread start facts in [facts.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/analytics/src/facts.rs:347). | Same fact exists in Ontocode at [facts.rs](/opt/demodb/_workfolder/ontocode/codex-rs/analytics/src/facts.rs:347). | Analytics may aggregate graph facts, but must not become graph state. |
| Agent identity | `AgentIdentityKey` and `AgentIdentityJwtClaims` exist in [lib.rs](/opt/demodb/_workfolder/GitNexus/codex-rs/agent-identity/src/lib.rs:40). | Same identity crate exists in Ontocode at [lib.rs](/opt/demodb/_workfolder/ontocode/codex-rs/agent-identity/src/lib.rs:40). | Keep identity separate from topology. Do not use JWT identity as a graph primary key. |
| Static code graph | GitNexus tool/runtime supplies symbol context, impact, rename, and detect-changes externally. | Ontocode project rules already require GitNexus use, but no core static index exists. | Do not embed static code graph indexing into core. Add bounded report/import surfaces only if needed. |
| GitNexus third-party graph store | GitNexus currently uses `@ladybugdb/core` for its analyzer/index storage through the npm package. | Ontocode has no LadybugDB dependency and stores runtime/memory data through existing Rust/SQLite state ownership. | LadybugDB may live inside the local evidence binary only. Code-graph-memory stores normalized evidence records, not GitNexus/LadybugDB data files or query APIs. |
| GitNexus analyzer dependencies | GitNexus depends on tree-sitter grammars, graphology, ONNX/transformers, Express, and MCP SDK packages. | Ontocode must not inherit those Node/native dependencies for runtime, app-server, memory, or context paths. | Bundle analyzer dependencies into one local evidence binary. Ontocode consumes the binary's stable bounded JSON/text evidence contract and remains optional if the binary is absent. |
| Memory pipeline owner | No separate GitNexus copy is needed for Ontocode memory ownership. | Ontocode memory pipeline is documented in [README.md](/opt/demodb/_workfolder/ontocode/codex-rs/memories/README.md:1), with state-backed Stage 1 outputs and Phase 2 consolidation. | Code-graph-memory belongs beside state/memory ownership, not in `codex-core` orchestration or analytics. |
| Existing memory schema | Not a code graph fact store. | `stage1_outputs` stores rollout-derived `raw_memory`, `rollout_summary`, and selection metadata in [0001_memories.sql](/opt/demodb/_workfolder/ontocode/codex-rs/state/memory_migrations/0001_memories.sql:1). | Do not overload rollout memory rows. Add a separate `code_graph_memory_records` schema if implementation starts. |
| Memory runtime owner | Not a static graph owner. | `MemoryStore` owns state-backed memory job and output operations in [memories.rs](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime/memories.rs:27). | Add code-graph-memory persistence through state/runtime ownership, keeping leases, pruning, and retention explicit. |
| Context fragment boundary | GitNexus evidence can be summarized for context, but not dumped. | `ContextualUserFragment` is the shared bounded injection trait in [fragment.rs](/opt/demodb/_workfolder/ontocode/codex-rs/context-fragments/src/fragment.rs:45). | If model-visible, code-graph-memory must be a new narrow fragment with hard caps; do not change the trait shape. |
| Memory exclusion for injected context | GitNexus evidence can be imported without being learned as rollout memory. | Phase 1 excludes AGENTS and skill contextual fragments in [phase1.rs](/opt/demodb/_workfolder/ontocode/codex-rs/memories/write/src/phase1.rs:457). | Code-graph-memory fragments need explicit classification so internal evidence is not accidentally converted into user memory. |
| External-context pollution bridge | GitNexus evidence import is not the same as model-visible external context. | `mark_thread_memory_mode_polluted` routes external-context pollution to memory state in [state_db.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rollout/src/state_db.rs:457). | Only mark polluted when code-graph-memory is injected into the model turn, not when evidence is stored offline. |
| Code-graph-memory record | Proposed from GitNexus `context`, `impact`, `detect_changes`, and runtime graph summaries. | No current Ontocode schema exists. | Add compact records with source kind, symbol/file/process refs, risk, bounded summary, provenance hash, timestamps, and retention. |
| Code-graph-memory query | Proposed internal backbone read path. | No current Ontocode query API exists. | Return bounded summaries by task/thread/symbol. No public app-server API until a separate ADR approves wire shape and compatibility tests. |

## Accepted Scope After Challenge

| Scope | Decision | Reason |
|---|---|---|
| Operational evidence backbone | Accepted | Consolidates GitNexus graph evidence and lean-ctx-inspired workflow evidence into one bounded state-backed record model. |
| Code-graph-memory backbone | Accepted as a domain | Required to make graph evidence durable and reusable across manager/subagent workflows without re-querying or re-indexing every turn. |
| Operational evidence state schema | Accepted for implementation planning | Existing rollout memory schema is not a generic evidence store; use a separate state-backed table with retention and provenance. |
| GitNexus evidence ingestion | Accepted as bounded internal/project tooling | Stores compact `context`, `impact`, and `detect_changes` facts, not raw graph DB rows or raw source bodies. |
| Hermetic local evidence binary | Accepted | Bundles GitNexus/LadybugDB/parser/ML third-party dependencies behind one process boundary and breaks hard dependency links from Ontocode core. |
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

## Options

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

### Option 9 - Hermetic Local Code-Graph Evidence Binary

Package GitNexus analyzer dependencies into a local executable that emits bounded evidence artifacts for Ontocode to import.

Implementation:

- Build a local per-platform binary/package, for example `ontocode-codegraph-evidence`, that includes GitNexus analyzer code and all third-party analyzer dependencies.
- Pin and bundle `@ladybugdb/core`, platform LadybugDB packages, tree-sitter grammars, graphology packages, ONNX/transformers dependencies, Express/MCP SDK only inside that binary package.
- Expose a narrow CLI contract: input repo path and requested evidence type; output versioned bounded JSON evidence.
- Treat `.gitnexus/lbug` and all analyzer cache files as private binary-owned cache, never as Ontocode state.
- Require checksums, SBOM, license manifest, platform smoke tests, no-network default mode, and deterministic failure semantics.

Pros:

- Breaks hard dependency links from Ontocode crates, app-server, SDKs, and state APIs to analyzer libraries.
- Preserves access to rich GitNexus/LadybugDB analysis without importing its storage engine into core.
- Gives operations one artifact to install, update, checksum, sandbox, and roll back.
- Lets Ontocode continue functioning without graph evidence when the binary is absent or fails.

Cons:

- Requires platform packaging and update discipline.
- Binary supply-chain review still matters because third-party code is bundled.
- Evidence schema compatibility must be tested across binary versions.

## Decision

Use Option 6 plus Option 9 and the accepted part of Option 1: make operational evidence a bounded backbone layer, treat code-graph-memory as one evidence domain, bundle third-party analyzer dependencies into a local evidence binary, and use existing `StateRuntime`/`thread_spawn_edges` behavior as the canonical persisted spawned-agent topology input.

Option 2 is no longer an automatic next step. Refactor `AgentGraphStore` only if a separate cleanup task proves it is worth touching a currently low-use trait.

Option 5 is accepted only as an ingestion convention into operational evidence records. It must not import the GitNexus graph DB or run static indexing in Ontocode core.

Consider Option 3 and Option 4 only after concrete UI/debugging requirements and a separate app-server/diagnostics ADR.

Do not implement Option 7 or Option 8.

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

- Add a state-owned `operational_evidence_records` table in a new memory/state migration.
- Define a model with `id`, `evidence_domain`, `source_tool`, `source_version`, `schema_version`, `source_ref`, `repo`, `task_key`, `thread_id`, `symbol_uid`, `symbol_name`, `file_path`, `process_label`, `gate_name`, `risk`, `status`, `summary`, `source_links`, `provenance_hash`, `redaction_status`, `created_at`, and optional `expires_at`.
- Supported initial domains are `code_graph`, `workflow`, `test`, `doc`, `redaction`, and `architecture`.
- Store compact summaries only; never store raw source bodies, raw prompts, credentials, terminal output, GitNexus graph rows, lean-ctx cache/session data, or full tool output.
- Add indexes for task/thread/symbol/file/domain/risk/status lookup.

Acceptance:

- State migration is backward-compatible.
- Tests cover insert, upsert by provenance hash, bounded summary size, expiry/pruning, domain filtering, and stable ordering.
- Redaction tests fail if token, cookie, authorization header, keychain path, or raw credential value appears in a record.
- Dependency tests fail if GitNexus analyzer packages, LadybugDB packages, or lean-ctx runtime packages are added to Ontocode runtime/core/app-server/SDK manifests.

### G2 - Hermetic Evidence Binary And Importer

Status: accepted for implementation planning.

Tasks:

- Define a local evidence binary contract before importing anything into state.
- Add a binary-owned package that bundles GitNexus analyzer code and all third-party analyzer dependencies.
- Add an internal/project-tooling writer that invokes the local binary or reads its artifact, then imports bounded GitNexus `context`, `impact`, and `detect_changes` summaries through a versioned evidence artifact.
- Normalize evidence into the state model without importing GitNexus graph storage.
- Preserve source links and risk summaries so later agents can reason from provenance.
- Reject oversized or unredacted payloads before persistence.
- Do not link to, load, or parse LadybugDB directly from Ontocode runtime code.
- Do not shell out to raw GitNexus internals from the core request path; if execution is needed, shell out only to the local evidence binary through a bounded timeout/sandbox wrapper.
- Do not parse `.gitnexus/lbug` from Ontocode.
- Treat GitNexus analyzer version, `@ladybugdb/core` version, and evidence schema version as provenance fields only.
- Keep binary installation/update/checksum/SBOM/license data outside the operational evidence record model, except for compact provenance fields.

Acceptance:

- Import fixtures cover low, high, and critical impact reports.
- Importer rejects raw code blocks and graph dumps.
- Importer is usable by manager/subagent workflows without changing production request paths.
- Importer fails closed on unsupported evidence schema versions or missing provenance.
- Missing or failing local evidence binary degrades to "no graph evidence" without breaking normal Ontocode runtime behavior.
- Dependency tests assert no `@ladybugdb/core`, `gitnexus`, tree-sitter, graphology, ONNX, transformers, Express, or MCP SDK packages are introduced into Ontocode runtime/core/app-server/SDK package manifests by this work; only the binary-owned packaging manifest may contain them.
- Binary packaging tests verify checksum, SBOM/license generation, no-network default mode, and platform smoke execution.

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
- GitNexus analyzer dependencies include native/platform packages and ML/parser packages; they are allowed only inside the local evidence binary, not Ontocode runtime crates or app-server processes.
- Lean-ctx dependencies and runtime behavior are allowed only as external development workflow tooling, not as Ontocode runtime, app-server, SDK, or persistence dependencies.
- LadybugDB storage files such as `.gitnexus/lbug` are implementation details and must never become an Ontocode persistence or compatibility contract.
- Semver ranges in the external analyzer package mean GitNexus storage behavior can change independently of Ontocode; the evidence artifact schema must be versioned and validated.
- The local evidence binary becomes a supply-chain artifact and requires checksum, SBOM, license, platform smoke, update, and rollback discipline.
- The binary and external workflow tools must fail closed and be optional for normal runtime behavior; operational evidence cannot make sessions depend on analyzer or lean-ctx availability.

## Final Recommendation

Adopt operational evidence as the Ontocode backbone layer, with code-graph-memory and lean-ctx-inspired workflow evidence as domains inside one bounded durable record model. The local evidence binary is the only approved place to include GitNexus/LadybugDB/tree-sitter/graphology/ONNX/transformers/MCP SDK third-party analyzer dependencies; lean-ctx remains external development workflow tooling and is not vendored or required by Ontocode runtime. Ontocode core consumes only versioned bounded evidence artifacts or repository-script summaries. The next engineering step is `G1`: add the unified state model and tests for compact GitNexus/runtime/workflow facts. After that, `G2` must define the local binary contract, package GitNexus third-party dependencies behind that boundary, import only versioned evidence artifacts, and fail closed on binary/schema/provenance drift. `G2b` may import lean-ctx-inspired workflow evidence only as bounded records, not as lean-ctx runtime behavior. Do not parse `.gitnexus/lbug`, do not add analyzer or lean-ctx packages to Ontocode runtime/core/app-server/SDK manifests, do not promote `AgentGraphStore`, and do not expose app-server/TUI graph APIs until separate ADRs approve those surfaces.
