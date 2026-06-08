# ADR: GBrain Inspired Tool Extensions Review

## Status

Consolidated - historical source evidence; dispatch via `ADR_EXTERNAL_AGENT_INTEROP_DETECTORS_CONSOLIDATION.md`

## Date

2026-06-07

## Context

GBrain is a TypeScript knowledge-brain layer for agents. It combines hybrid retrieval, graph traversal, answer synthesis, gap analysis, scoped team memory, schema packs, MCP operations, ingestion/enrichment, citation discipline, minion jobs, guardrail seams, and eval gates.

This ADR originally stored 400 GBrain-inspired extension candidates. After GitNexus review, the broad catalog is not accepted as an implementation backlog. Most candidates duplicate current Ontocode owners or belong in prior ADRs. The only approved local direction is an inert GBrain interop detector/report that helps Ontocode understand whether a workspace already uses GBrain, without importing content, executing jobs, or changing runtime behavior.

## Consolidation Override

This ADR is no longer an independent dispatch plan. Its retained GBrain Stage 0 requirements are consolidated into [External-Agent Interop Detector Consolidation](ADR_EXTERNAL_AGENT_INTEROP_DETECTORS_CONSOLIDATION.md).

If this ADR conflicts with the consolidation ADR, the consolidation ADR wins.

Dispatch rule:

- Do not implement `onto_gbrain_interop_detector` as a separate detector stack unless it is a compatibility wrapper around the shared external-agent interop detector contract.
- Use the shared report envelope, redaction rules, source-specific GBrain requirements, and blocked-scope rules from the consolidation ADR.
- Keep this file as upstream/source evidence and historical disposition for the original GBrain review.

Reviewed upstream at commit `613da94093c248e6126f5d1eacc396a5833265c1`.

- Repository overview and README: <https://github.com/garrytan/gbrain>
- Retrieval architecture: <https://github.com/garrytan/gbrain/blob/613da94093c248e6126f5d1eacc396a5833265c1/docs/architecture/RETRIEVAL.md>
- Codex MCP setup: <https://github.com/garrytan/gbrain/blob/613da94093c248e6126f5d1eacc396a5833265c1/docs/mcp/CODEX.md>
- Downstream agent integration: <https://github.com/garrytan/gbrain/blob/613da94093c248e6126f5d1eacc396a5833265c1/docs/guides/agent-to-gbrain.md>
- Brain-agent loop: <https://github.com/garrytan/gbrain/blob/613da94093c248e6126f5d1eacc396a5833265c1/docs/guides/brain-agent-loop.md>
- Schema packs: <https://github.com/garrytan/gbrain/blob/613da94093c248e6126f5d1eacc396a5833265c1/docs/architecture/schema-packs.md>
- Minion shell jobs: <https://github.com/garrytan/gbrain/blob/613da94093c248e6126f5d1eacc396a5833265c1/docs/guides/minions-shell-jobs.md>
- Source attribution: <https://github.com/garrytan/gbrain/blob/613da94093c248e6126f5d1eacc396a5833265c1/docs/guides/source-attribution.md>
- Guardrail seams: <https://github.com/garrytan/gbrain/blob/613da94093c248e6126f5d1eacc396a5833265c1/docs/guardrails.md>
- Contract-first operations: <https://github.com/garrytan/gbrain/blob/613da94093c248e6126f5d1eacc396a5833265c1/src/core/operations.ts>
- Hybrid search implementation: <https://github.com/garrytan/gbrain/blob/613da94093c248e6126f5d1eacc396a5833265c1/src/core/search/hybrid.ts>

## GitNexus Challenge Evidence

GitNexus found existing Ontocode owners for the major GBrain surfaces:

- Memory/search/context behavior already has owners in the memories extension, memory writer, and bounded contextual fragment path: [tests.rs](/opt/demodb/_workfolder/ontocode/codex-rs/ext/memories/src/tests.rs:394), [phase1.rs](/opt/demodb/_workfolder/ontocode/codex-rs/memories/write/src/phase1.rs:148), [contextual_user_message_tests.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/context/contextual_user_message_tests.rs:33)
- Model-visible tool planning and extension tools already have owners: [spec_plan.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/tools/spec_plan.rs:160)
- MCP status, resource, and tool-call behavior already has owners: [mcp/mod.rs](/opt/demodb/_workfolder/ontocode/codex-rs/codex-mcp/src/mcp/mod.rs:317), [mcp_tool_call.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/mcp_tool_call.rs:107)
- External-agent detection/import already has owners: [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:260), [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/external_agent_config_processor.rs:391)
- Shell execution, policy, and sandbox behavior already have owners: [shell.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/tools/handlers/shell.rs:59), [exec_policy_tests.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/exec_policy_tests.rs:1800)
- Provider metadata, capabilities, and status diagnostics already have owners: [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:33), [card.rs](/opt/demodb/_workfolder/ontocode/codex-rs/tui/src/status/card.rs:161)
- Diagnostics and redaction testing already have owners: [output.rs](/opt/demodb/_workfolder/ontocode/codex-rs/cli/src/doctor/output.rs:1298), [context_snapshot.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/tests/common/context_snapshot.rs:503)

## Decision

Accept only one Stage 0 task from this ADR:

| Label | Decision | Owner | Implementation boundary |
|---|---|---|---|
| `onto_gbrain_interop_detector` | Keep | external-agent config migration / diagnostics | Detect GBrain workspace artifacts and produce a redacted dry-run report. No content import, no GBrain server calls, no job execution, no model-context injection, no credential persistence. |
| `onto_gbrain_interop_detector_tests` | Keep | external-agent config migration tests | Add fixture tests proving redaction, bounded output, no content import, no credential leakage, and stable classification of detected artifacts. |

Everything else from the original 400 candidates is removed from this ADR and handled by one of three outcomes:

- Delegated to an existing core owner and prior ADR.
- Blocked until a separate ADR proves runtime need and compatibility.
- Moved to [ADR_GBRAIN_TOOL_EXTENSIONS_LEFTIES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_GBRAIN_TOOL_EXTENSIONS_LEFTIES.md:1) because it is not a natural core extension.

## Approved Stage 0 Detector Scope

The detector may inspect only local file names, manifest metadata, config keys, counts, and redacted endpoint identities.

Allowed findings:

- GBrain installation/config presence: `~/.gbrain/`, workspace `gbrain.*`, package metadata, or documented config paths.
- MCP connection presence: server name, transport kind, endpoint host only, and operation count. No tokens, headers, raw URLs with credentials, cookies, or request bodies.
- Schema-pack presence: pack name, version, declared type count, and validation errors. No schema activation or memory model mutation.
- Skillpack presence: name, version, manifest path, command count, and routing rule count. No command execution.
- Source registry presence: source IDs, source kinds, enabled/disabled state, and file counts. No imported source content.
- Operation registry presence: operation IDs, local-only flags, MCP exposure flags, and schema names. No operation execution.
- Minion/job/scheduler presence: job IDs, schedule shape, disabled/enabled state, and command count. No shell command output or execution.
- Audit/eval/log presence: file counts, last-modified timestamps, and redacted category names.
- Storage/backend configuration kind: local, remote, sqlite, postgres, or unknown. No database URL, credential, or path containing secrets.
- Brain page/timeline/compiled-truth content shape: counts and section names only. No memory content import.

Rejected Stage 0 behavior:

- No hybrid search, RRF, reranking, graph traversal, embeddings, or query expansion.
- No new memory store, schema-pack runtime, graph store, source-ingestion pipeline, job queue, guardrail runtime, MCP manager, tool registry, provider gateway, or eval framework.
- No model-visible tool exposure.
- No app-server API exposure without a later compatibility ADR.
- No shell execution or background worker.
- No credential import or persistence.
- No unbounded context injection.

## Architecture Decision For New Work

If implemented, the detector should extend the existing external-agent detection/import boundary:

- Add GBrain as another inert external-agent detection source beside existing external-agent config detection.
- Keep report structs redacted and serializable for diagnostics, but do not expose public config or app-server APIs in the first slice.
- Reuse existing redaction helpers and import-report conventions.
- Keep implementation outside `codex-core` unless a later ADR proves it must affect runtime behavior.
- Add tests under the existing external-agent migration/config processor test patterns.
- Run GitNexus impact before editing any detection symbol, especially `ExternalAgentConfigService::detect_migrations` or app-server request processors.

## Original Point Disposition

Every original point is covered below. Contiguous ranges are used because the challenged candidates share the same owner and decision.

| Original points | Disposition | Similar solution or owner | Action |
|---|---|---|---|
| `001-020` | Mostly delegated; Stage 0 keeps only dry-run loop/config detection. | Memories/context/skills already cover bounded memory fragments and skills. | Remove from this ADR except detector metadata. |
| `021-050` | Delegated or blocked. | Memory search already exists; retrieval diagnostics/evals belong to memory/search ADRs. | Do not add a GBrain retrieval stack. |
| `051-070` | Delegated or blocked. | GitNexus owns code graph; memories/search would own memory graph if later accepted. | Do not duplicate GitNexus or add graph runtime here. |
| `071-090` | Stage 0 keeps schema-pack detection only. Runtime schema-pack behavior is blocked. | External-agent detection can report manifests; memory schema changes need a separate ADR. | Remove runtime schema-pack points from this ADR. |
| `091-110` | Delegated. | Memories/provenance/redaction owners already cover source metadata and conflict safety. | No separate GBrain citation contract in this ADR. |
| `111-130` | Stage 0 keeps MCP config detection only. | MCP manager/tool-call/resource surfaces already exist. | Delegated to MCP owners; no GBrain MCP manager. |
| `131-150` | Delegated or blocked. | Existing tool specs and registry own model-visible tools. | No contract-first operation registry here. |
| `151-170` | Moved to lefties or delegated to shell policy if later approved. | Shell runtime/policy already owns execution safety. | No minion job queue or scheduler. |
| `171-190` | Delegated or moved to lefties. | Hooks, policy, redaction, and tests own guardrail behavior. | No new guardrail runtime. |
| `191-210` | Stage 0 keeps skillpack manifest detection only. | Skills extension and external-agent migration already own import/report seams. | No remote skillpack marketplace. |
| `211-240` | Stage 0 keeps source registry detection only. | External-agent migration owns inert detection; ingestion products are lefties. | No ingestion/enrichment pipeline. |
| `241-260` | Delegated or moved to lefties. | Provider descriptors/native/external adapter ADRs own provider capability behavior. | Provider-related points moved to provider ADRs. |
| `261-280` | Delegated or moved to lefties. | Existing test/diagnostic tooling and lean-ctx ADR own operational validation. | No GBrain eval framework. |
| `281-300` | Delegated or moved to lefties. | Doctor/status/support diagnostics already exist. | No admin dashboard or product UI. |
| `301-320` | Delegated or moved to lefties. | Memories/provenance may later own source confidence; domain-specific scoring is lefties. | No notability/trust score runtime here. |
| `321-330` | Delegated to GitNexus or moved to lefties. | GitNexus is the code intelligence owner. | Do not duplicate code graph retrieval. |
| `331-340` | Moved to lefties except optional shape-only detector metadata. | External content ingestion is not core. | No meeting/email/calendar/social/voice/media ingestion. |
| `341-350` | Delegated. | Security/redaction/diagnostic owners already exist. | Add only redaction requirements to detector tests. |
| `351-360` | Delegated or moved to lefties. | External-agent migration, MCP, provider adapter, and auth ADRs own these seams. | No remote brain server or credential bridge here. |
| `361-380` | Moved to lefties or blocked. | Storage/runtime owners would need a separate ADR. | No external DB/storage/tiering/runtime cache. |
| `381-400` | Delegated to lean-ctx project tooling ADR or lefties. | Memory-bank validators/readiness gates belong in project tooling. | Remove from this ADR; readiness gate is satisfied by this challenge. |

## Cross-ADR Delegations

The following prior ADRs now own the relevant surviving proposals:

- [ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS.md:1) owns GBrain-derived memory-bank validators, ADR challenge checklists, source-link verification, redaction gates, and readiness gates.
- [ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_PROVIDER_EXTENSIBILITY_REMAINING_IMPLEMENTATION.md:1) owns any future GBrain credential/import evidence, OAuth/env classification, and redacted import-report behavior.
- [ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_NATIVE_HETEROGENEOUS_PROVIDER_ENGINES.md:1) owns GBrain-inspired provider capability, model catalog, status, and cost/rate-limit diagnostics when they extend native provider descriptors.
- [ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md](/opt/demodb/_workfolder/ontocode/.memory-bank/ADR_EXTERNAL_PROVIDER_ADAPTER_RUNTIME.md:1) owns any future GBrain-style remote/provider gateway idea only if it becomes an explicit external adapter capability under the approved adapter transport.

## Implementation Task Card

Use this card if a low-capability agent implements the accepted slice:

- Task: implement `onto_gbrain_interop_detector`.
- Scope: redacted dry-run detection/report only.
- Before editing: run GitNexus context and impact for the target detection/import symbol.
- Files to inspect first: external-agent config detection, external-agent config processor tests, OAuth import redaction tests, and doctor diagnostic redaction tests.
- Acceptance tests: fixture with GBrain config, fixture with token-like values, fixture with scheduler/minion command, fixture with source registry, fixture with schema-pack manifest.
- Expected result: report contains names/counts/kinds only; report never contains raw tokens, authorization headers, cookies, database URLs, credential file contents, source content, shell output, or memory page content.
- Out of scope: importing GBrain memory, exposing GBrain as MCP, executing GBrain operations, running minions, adding model-visible tools, adding app-server APIs, or adding provider runtime behavior.

## Challenge Outcome

The current codebase is not missing a GBrain-like runtime layer. It already has distinct owners for memories, context, MCP, tools, shell, provider metadata, diagnostics, and external-agent migration. The useful GBrain extension is therefore interop awareness: detect and report existing GBrain workspace configuration safely so a future ADR can decide whether any deeper import is justified by real user evidence.
