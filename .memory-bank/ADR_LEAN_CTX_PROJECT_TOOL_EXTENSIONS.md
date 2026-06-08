# ADR: Lean-ctx Project Tool Extensions

## Status

Challenged - inlined into GitNexus dependency consolidation; Stage 0 repository tools remain bootstrap only

## Date

2026-06-07

## Context

The project already relies on lean-ctx for compressed shell output, file reads, searches, context maps, and agent-oriented workflows. The current project plan also depends on repeated manager loops, GitNexus impact checks, ADR tracking, provider/OAuth/MCP diagnostics, and memory-bank updates.

This ADR stores a catalog of candidate lean-ctx-style tools that could extend the current codebase without duplicating existing architecture. The catalog is intentionally broad. Each item is a proposal candidate, not an approved implementation task.

Primary references reviewed:

- lean-ctx repository: `https://github.com/yvgude/lean-ctx`
- lean-ctx tools documentation: `https://leanctx.com/docs/tools/`
- lean-ctx feature catalog: `https://github.com/yvgude/lean-ctx/blob/main/LEANCTX_FEATURE_CATALOG.md`
- Current project memory bank: `.memory-bank/MEMORY.md`
- Current project plan: `.memory-bank/project_plan-current.md`
- Current agent rules: `.memory-bank/reference_agent-rules.md`

## Problem

The project has many repeated operational tasks:

- reading and summarizing ADRs before implementation
- updating tracking files before dispatch
- checking GitNexus context and impact before editing code
- enforcing provider/auth/MCP architecture reuse rules
- validating redaction and credential safety
- running scoped tests and summarizing failures
- preserving memory-bank state after manager loops
- reducing context bloat during long-running agent sessions

These tasks are currently handled by human discipline and ad hoc agent prompts. That is error-prone, especially when multiple sub-agents work in parallel.

## Decision

Use this ADR as a frozen proposal catalog plus the lean-ctx-specific input to the consolidated Ontocode operational evidence backbone. It does not approve implementation of the full catalog. The canonical third-party dependency policy and unified state model now live in `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md`.

Prioritize future task cards that automate existing rules and reduce risk:

- memory-bank and tracking-file hygiene
- GitNexus impact/detect-change gates
- provider/OAuth/MCP reuse checks
- scoped test planning and failure summarization
- bounded context generation for external agents
- manager-loop dispatch, verification, and redo tracking

Do not implement tools that duplicate core runtime architecture. New tools must orchestrate, verify, summarize, or enforce policy around existing owners. Lean-ctx remains an external agent/workflow utility; this ADR does not approve vendoring lean-ctx, adding a lean-ctx runtime dependency, or creating a second context/memory/search substrate inside Ontocode.

The accepted core-backbone scope is not lean-ctx itself. It is the workflow evidence domain inside the unified operational evidence backbone: task cards, gate results, repository-script summaries, redaction reports, test summaries, doc-link reports, and readiness summaries.

## Dependency Consolidation Override

`ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md` is the canonical consolidation record for third-party dependencies from both GitNexus and lean-ctx derived work. If this ADR conflicts with that ADR, the GitNexus ADR wins.

Consolidated rules:

- Lean-ctx must remain an external development workflow tool, not an Ontocode runtime, app-server, SDK, state, memory, context, or model-visible dependency.
- Lean-ctx-inspired records must be stored through the unified `operational_evidence_records` contract from the GitNexus ADR, not through a separate Lean-CTX schema.
- GitNexus-derived evidence is the `code_graph` domain; lean-ctx-inspired evidence is `workflow`, `test`, `doc`, `redaction`, or `architecture` domain.
- Stage 0 repository scripts must use standard-library-first implementation and must not add third-party runtime dependencies.
- Missing GitNexus, lean-ctx, or the local evidence binary must degrade to missing evidence, not broken Ontocode runtime behavior.

## Implementation Constraints

- Reuse existing project architecture and GitNexus analysis before touching code.
- Keep tools additive and operational unless a separate ADR approves runtime behavior changes.
- Security-sensitive tools must reuse shared redaction behavior and include leak tests.
- Context-producing tools must enforce hard caps and bounded fragments.
- Provider, OAuth, MCP, hooks, shell, and external-agent tools must inspect existing owners before suggesting edits.
- No tool may silently rewrite tracking files or ADR status without preserving an audit trail.

## Review, Advice, And Challenge

This ADR is useful as an operations backlog, but it must not become a second product architecture. The safe framing is:

- Keep tools that inspect, summarize, enforce, or orchestrate existing owners.
- Require a separate ADR before any tool changes runtime provider behavior, public config, app-server APIs, credential storage, MCP protocol behavior, shell policy, or model context injection.
- Move release-management and final-answer automation out of this ADR because it does not naturally extend core project functionality.

## Second Challenge Findings

This ADR is still too broad to dispatch as written. The main challenge is not whether the ideas are useful; it is whether a low-capability agent can implement them without crossing runtime, model-visible, app-server, GitNexus, provider, MCP, shell, or context boundaries.

Current findings:

- Stage 0 is not implemented. Repository search found no `scripts/onto_memory_tools.py`, `onto_memory_status_digest`, `onto_tracking_count_left`, or `onto_diff_doc_link_check`.
- The 196-item candidate list must be treated as frozen backlog labels, not active work. The word "tool" in the catalog does not imply a model-visible tool, app-server API, Rust crate, or runtime feature.
- GitNexus wrappers are not approved by Stage 0. They must wait for a later task card and must align with `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md`: consume bounded evidence artifacts or GitNexus MCP/CLI output, never create a parallel graph store or parse GitNexus internals.
- Lean-ctx itself is not an Ontocode dependency target. This ADR may copy workflow patterns into small repository scripts, but it must not vendor lean-ctx internals, replicate lean-ctx caches, or add a second shell/read/search/context runtime.
- Existing repository scripts already provide packaging, formatting, redaction, readme generation, and test helpers. New Stage 0 code must be additive and should live in one script instead of creating a new framework.
- The current memory-bank index did not list this ADR, which makes it easy for later agents to miss the Stage 0 boundary. The index must include this ADR after challenge.

Corrected challenge result:

- Stage 0 remains the only approved repository-script implementation path.
- Stage 0 may add exactly one repository-only Python script and optional tests.
- A separate core-backbone path is accepted only for data contracts, state-backed records, evidence gates, and bounded summaries defined in the "Core Operational Backbone Scope" section.
- No GitNexus wrapper, manager loop, external-agent orchestration, provider/OAuth/MCP/shell/context diagnostic, model-visible tool, app-server API, or Rust code is approved until a backbone stage card names the exact owner, schema, tests, and compatibility gates.
- Any later slice must add a new task card to this ADR and must be reviewed as a separate implementation decision.

## Core Backbone Re-Challenge

The earlier Stage 0-only framing was safe but too defensive for the stated goal: putting the useful solution into Ontocode's core backbone. The corrected architecture is to promote operational invariants, not lean-ctx mechanics.

Challenge result:

- Accept an Ontocode Operational Backbone as a core-owned evidence and policy layer.
- Reject copying lean-ctx runtime behavior, caches, shell wrappers, read/search compression, session memory, or tool discovery into Ontocode.
- Reject adding 196 model-visible tools or app-server methods.
- Reuse existing state, memory, context-fragment, GitNexus evidence, tool-extension, app-server v2, and redaction owners.
- Keep Stage 0 scripts as bootstrap/reporting helpers only; they are not the backbone.
- Require separate implementation stage cards before any Rust crate, migration, context fragment, app-server API, or model-visible tool is added.

Backbone definition:

- `OperationalTaskCard`: durable unit of work with source ADR, owner surface, status, blocker, evidence references, and verification command summary.
- `OperationalEvidenceRecord`: bounded record for GitNexus context/impact/detect-change output, test commands, doc-link checks, redaction checks, and review decisions.
- `OperationalGateResult`: deterministic pass/fail/warn result with gate name, target surface, evidence ids, remediation, timestamp, and redaction status.
- `OperationalReadinessSummary`: compact derived report for "what is left", "ready for plenty providers", and "safe to close" answers.

Backbone non-goals:

- No direct dependency on lean-ctx as an Ontocode runtime crate.
- No direct dependency on GitNexus storage internals, LadybugDB, or `.gitnexus/lbug`.
- No second provider registry, MCP manager, shell runtime, credential store, redactor, context injection path, or app-server protocol family.
- No unbounded model context and no public API without a follow-up ADR.

## Hermes Agent Review Addendum

`ADR_HERMES_AGENT_TOOL_EXTENSIONS.md` originally proposed manager-loop and repository-operations points `181-200` and `381-400`. Those points are delegated here only as repo-only validators, prompt templates, tracking helpers, GitNexus wrappers, ADR challenge checklists, and low-agent task-card generators.

Delegated original points:

- `181-200`: delegation prompts, explicit-context rules, child result summaries, manager-loop checklists, failed-child redo rules, artifact handoff conventions, and low-agent prompt generation.
- `381-400`: affordability splitter, ADR challenge checklist, source-link verifier, candidate-count verifier, duplicate proposal detector, lefties router, implementation-label generator, tracking-file updater, GitNexus preflight template, test-command planner, blast-radius reporter, redaction/context/app-server/provider/MCP/shell/migration acceptance templates, manager dispatch prompt, and readiness gate.

Architecture decision:

- These remain memory-bank/project tooling ideas unless a separate ADR proves they must be model-visible, app-server-visible, or runtime-enforced.
- They must not replace GitNexus, create a second graph/index store, or modify runtime code paths.
- They may generate reports, task cards, and prompts that enforce existing architecture reuse rules.

## GBrain Review Addendum

`ADR_GBRAIN_TOOL_EXTENSIONS.md` originally proposed operational challenge/readiness points `381-400` and eval/diagnostic points `261-280`. Those points are delegated here only as memory-bank/project tooling that verifies ADR quality, source links, redaction requirements, owner mapping, and implementation readiness.

Delegated original points:

- `261-280`: retrieval/eval gate ideas only as optional project-level test-plan templates after a runtime owner accepts a feature.
- `381-400`: memory-bank validators, source-link verifier, candidate-count verifier, duplicate proposal detector, lefties router, implementation-label generator, tracking-file updater, GitNexus preflight template, test-command planner, blast-radius reporter, redaction/context/app-server/provider/MCP/shell/migration acceptance templates, manager dispatch prompt, and readiness gate.

Architecture decision:

- These remain repository-only helpers; they must not become model-visible tools, app-server APIs, or runtime gates without a separate ADR.
- They may enforce that GBrain-derived work stays in the accepted owner and does not duplicate memories, MCP, shell, provider, diagnostics, GitNexus, or external-agent migration.
- They may generate low-agent task cards from ADRs, but must preserve audit history and never silently rewrite tracking status.

## OpenClaw Review Addendum

`ADR_OPENCLAW_TOOL_EXTENSIONS.md` originally proposed diagnostics, conformance, ADR tracking, source-link, GitNexus, lefties, subagent prompt, completion-audit, and readiness-report points `241-260`, `341-360`, and `381-400`. Those points are delegated here only as repository-only validators, report templates, low-agent task cards, and challenge/checklist helpers.

Delegated original points:

- `241-260`: diagnostic section planning, telemetry field mapping, privacy guards, fixture generation, support-bundle size/secret tests, and live-probing rejection.
- `341-360`: conformance suite planning, snapshot requirements, feature-gate checks, no-runtime-execution tests, bounded-context tests, and GitNexus impact fixtures.
- `381-400`: challenge matrix, owner index, dependency graph, affordability/security scoring, test-plan generation, tracking updates, lefties export, GitNexus context recording, source-link checking, duplicate detection, reuse checks, public API/context/secret guards, stage splitting, subagent prompts, completion audit, detect-changes gate, and readiness reporting.

Architecture decision:

- These remain memory-bank/project tooling ideas unless a separate ADR proves they must be model-visible, app-server-visible, or runtime-enforced.
- They may enforce that OpenClaw-derived work stays in the accepted owner and does not duplicate gateway, plugin, provider, MCP, auth, browser, sandbox, channel, cron, node, diagnostics, or external-agent migration architecture.
- They may generate low-agent task cards and challenge reports, but must preserve audit history and never silently rewrite tracking status.

## OpenCode Review Addendum

`ADR_OPENCODE_TOOL_EXTENSIONS.md` originally proposed test/tooling points `301-320`, diagnostic/project-tooling points `361-380`, and ADR operations points `383-400`. Those points are delegated here only as repository-only validators, source-link checks, duplicate-proposal checks, owner-mapping reports, redaction templates, and low-agent task-card generators.

Delegated original points:

- `301-320`: recorded-test, fake-service, API exercise, snapshot, MCP OAuth, plugin, LSP, TUI, authorization, and benchmark ideas only as optional test-plan templates after a runtime owner accepts a feature.
- `361-380`: event, runtime flag, startup, skill, snapshot, provider, telemetry, usage, background-job, support-bundle, and diagnostics owner-matrix ideas only as report/checklist tooling around existing owners.
- `383-400`: compatibility matrices, lefties generator, source-link verifier, duplicate-proposal detector, implementation-label generator, tracking-file seeder, GitNexus preflight template, test-command planner, redaction acceptance template, affordability splitter, and readiness gate.

Architecture decision:

- These remain repository-only helpers; they must not become model-visible tools, app-server APIs, or runtime gates without a separate ADR.
- They may enforce that OpenCode-derived work stays in the accepted owner and does not duplicate agents, tools, context, MCP, shell, provider, app-server, diagnostics, GitNexus, or external-agent migration.
- They may generate task cards and acceptance templates only after the OpenCode ADR has been challenged and reduced to accepted labels.

## CliRelay Review Addendum

`ADR_CLIRELAY_TOOL_EXTENSIONS.md` originally proposed diagnostics/project-tooling/test points `341-360`, source-link/import tracking points `312-320`, and SDK/test/challenge points `381-400`. Those points are delegated here only as repository-only validators, source-link checks, duplicate-proposal checks, owner-mapping reports, redaction templates, fixture planners, and low-agent task-card generators.

Delegated original points:

- `312-320`: status matrix, source linker, duplicate detector, task-card generator, tracking seed, fixture pack, report schema, owner mapper, and lefties classifier.
- `341-360`: doctor summary ideas, redaction matrix, secret fixture suite, bounded output gate, no-model-context gate, advice catalog, source attribution, prior ADR linker, affordability report, and diagnostics lefties classifier.
- `381-400`: SDK abstraction reviews, example reviews, startup/executor/store test evidence, redaction tests, low-agent task split, and challenge checklist.

Architecture decision:

- These remain repository-only helpers; they must not become model-visible tools, app-server APIs, or runtime gates without a separate ADR.
- They may enforce that CliRelay-derived work stays in accepted owners and does not duplicate provider, auth, telemetry, network-proxy, app-server, diagnostics, external-agent migration, GitNexus, or storage architecture.
- They may generate task cards and acceptance templates only after the CliRelay ADR has been challenged and reduced to accepted labels.

GitNexus evidence used for this review:

- State/backbone storage owner: [runtime.rs](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime.rs:142), [memories.rs](/opt/demodb/_workfolder/ontocode/codex-rs/state/src/runtime/memories.rs:28), [local.rs](/opt/demodb/_workfolder/ontocode/codex-rs/agent-graph-store/src/local.rs:12)
- Model-visible tool planning owner: [spec_plan.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/tools/spec_plan.rs:548), [shell.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/tools/handlers/shell.rs:59), [events.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/tools/events.rs:143)
- Extension/plugin contribution owner: [registry.rs](/opt/demodb/_workfolder/ontocode/codex-rs/ext/extension-api/src/registry.rs:124), [extension.rs](/opt/demodb/_workfolder/ontocode/codex-rs/ext/skills/src/extension.rs:68), [extension.rs](/opt/demodb/_workfolder/ontocode/codex-rs/ext/web-search/src/extension.rs:128)
- Provider engine/descriptor owner: [descriptor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/descriptor.rs:7), [provider.rs](/opt/demodb/_workfolder/ontocode/codex-rs/model-provider/src/provider.rs:109)
- OAuth credential storage/import/redaction owners: [oauth.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/oauth.rs:58), [server.rs](/opt/demodb/_workfolder/ontocode/codex-rs/login/src/server.rs:643), [claude_oauth_import.rs](/opt/demodb/_workfolder/ontocode/codex-rs/external-agent-migration/src/claude_oauth_import.rs:10)
- MCP owner: [connection_manager.rs](/opt/demodb/_workfolder/ontocode/codex-rs/codex-mcp/src/connection_manager.rs:105), [mcp/mod.rs](/opt/demodb/_workfolder/ontocode/codex-rs/codex-mcp/src/mcp/mod.rs:318), [rmcp_client.rs](/opt/demodb/_workfolder/ontocode/codex-rs/rmcp-client/src/rmcp_client.rs:376)
- Context owner: [fragment.rs](/opt/demodb/_workfolder/ontocode/codex-rs/context-fragments/src/fragment.rs:46), [contextual_user_message.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/context/contextual_user_message.rs:46), [contributors.rs](/opt/demodb/_workfolder/ontocode/codex-rs/ext/extension-api/src/contributors.rs:100)
- Hooks/external-agent owner: [lib.rs](/opt/demodb/_workfolder/ontocode/codex-rs/external-agent-migration/src/lib.rs:110), [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:166), [hooks.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/tests/suite/hooks.rs:383)
- Shell/policy owner: [shell.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/tools/handlers/shell.rs:59), [unix_escalation.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/tools/runtimes/shell/unix_escalation.rs:102), [exec_policy.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/exec_policy.rs:631)
- App-server/config compatibility owner: [export.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server-protocol/src/export.rs:193), [config_processor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/config_processor.rs:148)
- External-agent config migration owner: [external_agent_config_processor.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/request_processors/external_agent_config_processor.rs:172), [external_agent_config.rs](/opt/demodb/_workfolder/ontocode/codex-rs/app-server/src/config/external_agent_config.rs:166)
- Diff/test owner examples: [turn_diff_tracker.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/src/turn_diff_tracker.rs:63), [rmcp_client.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/tests/suite/rmcp_client.rs:2002), [history_cell/tests.rs](/opt/demodb/_workfolder/ontocode/codex-rs/tui/src/history_cell/tests.rs:576), [common/lib.rs](/opt/demodb/_workfolder/ontocode/codex-rs/core/tests/common/lib.rs:48)

## Challenge Outcome

The ADR is not ready for implementation as written. It is acceptable as a reviewed backlog only after these constraints are treated as binding:

- P0: Do not add a new Codex tool registry for these items. Any model-visible tool must use the existing tool planning, handler, lifecycle, and extension registry surfaces.
- P0: Do not add the 196 operational tool labels to `codex-core`. Only the small Operational Backbone contracts may enter core/state code, and only through staged cards with tests.
- P0: Do not implement all 196 catalog labels as a project. The first core implementation must be the backbone contract slice, not a tool registry.
- P1: Split the ADR into surfaces before implementation: repository-only scripts, GitNexus wrappers, model-visible extension tools, app-server APIs, and runtime diagnostics.
- P1: Provider/OAuth/MCP/shell/context candidates are analysis gates only until their owning ADRs approve runtime changes.
- P1: External-agent orchestration must not duplicate the existing external-agent config import path or multi-agent runtime behavior.
- P1: GitNexus candidates are deferred behind a separate task card and must follow the code-graph-memory evidence-binary boundary; no direct LadybugDB or `.gitnexus/lbug` dependency is allowed.
- P2: Item names should be treated as working labels. Public names, command names, and tool schemas require a compatibility review before exposure.

## Required Surface Decision

Every candidate must be assigned to exactly one implementation surface before work starts:

| Surface | Allowed candidates | Existing owner | Constraint |
|---|---|---|---|
| Repository-only script | Memory-bank, tracking, GitNexus, doc-link, diff summaries | `.memory-bank`, scripts, GitNexus CLI/MCP | Must not become a model-visible tool or runtime dependency. |
| Core operational backbone | Task cards, evidence records, gate results, readiness summaries | `codex-rs/state`, `codex-rs/state/src/runtime/memories.rs`, `codex-rs/context-fragments`, GitNexus evidence boundary | Stores facts and gates only; must not execute provider/MCP/shell behavior or expose public APIs by default. |
| GitNexus wrapper | Impact, context, rename, detect-change, architecture reuse reports | GitNexus MCP/CLI | Must not persist inferred graph data as truth. |
| Codex model-visible tool | Only if the model must invoke it during a turn | Core tool spec/handlers and extension registry | Requires tool spec tests, lifecycle tests, and compatibility review. |
| App-server API | Only if external clients need it | app-server v2 protocol and processors | Requires ADR, schema generation, docs, and app-server protocol tests. |
| Runtime diagnostic | Provider/OAuth/MCP/shell/context diagnostics | Owning crate/module linked above | Must extend existing owner and include targeted tests. |
| Workflow prompt only | Manager-loop prompts, external-agent handoffs | Memory-bank and agent instructions | Must not be implemented in Rust unless justified by a later ADR. |

## Challenge Disposition Override

| Points | Previous disposition | Challenged disposition |
|---|---|---|
| 1-40 | Keep as new operational tooling | Keep only as repository-only scripts or memory-bank validators. Not core runtime. |
| 41-60 | Keep as GitNexus orchestration | Deferred except evidence-record import into the backbone. No duplicate graph/index store and no direct GitNexus storage dependency. |
| 61-80 | Keep as architecture guardrails | Keep as gate-result contracts first. Runtime enforcement requires owner-specific ADRs. |
| 81-100 | Keep as provider diagnostics | Keep as diagnostics only. Runtime provider changes must extend `ProviderEngine`/`ProviderKind`. |
| 101-120 | Keep as OAuth/security diagnostics | Keep as diagnostics/import validation only. No new credential broker. |
| 121-140 | Keep as MCP diagnostics | Keep as diagnostics around `McpConnectionManager`/RMCP. No alternate MCP manager. |
| 141-160 | Keep as hooks/shell/policy diagnostics | Keep as audit/test-planning only. Any permission behavior change requires impact analysis. |
| 161-167 | Keep as context diagnostics | Keep as bounded-fragment audits only. No side-channel context injection. |
| 168-180 | Keep as external-agent orchestration | Keep as workflow prompts unless a later ADR defines a runtime manager-loop owner. |
| 181-196 | Keep as test/diff readiness tooling | Keep as command planners/summarizers only. Do not replace `just`, insta, or existing test harnesses. |
| 197-200 | Move to lefties | Confirmed lefties. These remain outside this ADR. |

## Affordability Review

The ADR must be executable by the lowest-capability coding agent. That means the implementation plan must remove judgment-heavy decisions, avoid runtime code, and use mechanical checks.

Current affordability problems:

- The 196-item candidate list is too large to dispatch safely.
- The old first slice mixed memory-bank parsing, GitNexus enforcement, test planning, external-agent prompts, and manager-loop orchestration.
- Several candidate names sound like production tools even though most should remain repo-only scripts or checklists.
- A weak agent could accidentally add runtime code to `codex-core`, expose a model-visible tool, or create a parallel registry.
- The ADR did not define a tiny output contract, so completion would be subjective.

Affordability decision:

- Freeze the 196-item catalog as backlog labels only.
- Approve the Core Operational Backbone contract as the first core path.
- Approve only Stage 0 as the first repository-script implementation.
- Stage 0 must be repository-only, read-mostly, deterministic, and implemented outside `codex-rs`.
- Stage 0 must not call network services, not modify Rust code, not expose app-server APIs, and not register model-visible tools.
- Any later stage requires a new task card added to this ADR with exact input files, output files, commands, and pass/fail checks.

## Lowest-Agent Implementation Contract

A low-capability agent may implement only tasks that satisfy all of these rules:

- The task edits at most 2 files, excluding generated test fixtures.
- The task adds at most 250 changed lines.
- The task has one command for verification.
- The task has no architecture choice left open.
- The task does not require reading more than 5 source files.
- The task does not touch `codex-rs/core`, `codex-rs/app-server`, `codex-rs/model-provider`, `codex-rs/rmcp-client`, or `codex-rs/codex-mcp`.
- The task does not rename symbols.
- The task does not change public config, CLI behavior, app-server protocol, credential persistence, shell policy, MCP behavior, provider runtime behavior, or model context injection.
- If any rule is violated, stop and create a new ADR or senior-review task instead of coding.

## Core Operational Backbone Scope

This scope is consolidated into `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md`. Do not implement a separate Lean-CTX backbone schema from this ADR.

| Stage | Scope | Allowed owner | Required proof |
|---|---|---|---|
| B0 | Define task-card, evidence, gate, and readiness domains. | Consolidated `G1` operational evidence model in `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md`. | Unit tests for serialization, redaction status, deterministic ordering, domain filtering, and schema evolution notes. |
| B1 | Persist workflow records locally. | Existing `StateRuntime` through consolidated `operational_evidence_records`. | Migration tests, retention tests, redaction tests, and no public API exposure. |
| B2 | Import bounded workflow evidence from memory-bank, repository scripts, external lean-ctx-assisted workflows, GitNexus CLI/MCP output, and verification summaries. | Consolidated importer stages `G2` and `G2b` from `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md`. | Fixture-based import tests and rejection tests for oversized or secret-bearing evidence. |
| B3 | Produce deterministic readiness reports. | Internal reporting module over consolidated evidence records. | Golden-output tests for "what is left", owner readiness, stale evidence, blocked-task summaries, and evidence-domain filtering. |
| B4 | Optionally expose bounded context or app-server views. | Existing `ContextualUserFragment` path and app-server v2 only. | Separate ADR, hard caps, schema generation, protocol tests, cache-miss review, and dependency-policy check. |

Backbone implementation rules:

- Add no second runtime command executor, read/search system, memory store, graph store, provider registry, model catalog, OAuth broker, MCP manager, shell policy engine, or third-party dependency boundary.
- Keep model-visible summaries out of scope until B4; any B4 fragment must implement the existing `ContextualUserFragment` contract.
- Store evidence references and bounded summaries, not raw logs, raw GitNexus databases, raw credentials, lean-ctx cache/session data, compressed tool output bodies, or full source-file contents.
- Prefer deterministic, append-only records plus explicit supersession over silent mutation.
- Reuse existing redaction behavior for any security-sensitive diagnostic and test that secrets cannot appear in stored records or summaries.
- Any stage that touches Rust symbols must run GitNexus impact first and record the blast radius in the stage card.

Recommended first core slice:

1. Use `G1` from `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md` as the first core slice.
2. Define only the consolidated operational evidence data types and validation rules.
3. Add tests for serialization, deterministic ordering, domain filtering, max-size rejection, and redaction-state requirements.
4. Do not add migrations, APIs, context fragments, command execution, or lean-ctx invocation in the same slice.

## Stage 0 Approved Slice

Only these three tools are approved for first repository-script implementation:

| Tool | Type | Allowed implementation | Output |
|---|---|---|---|
| `onto_memory_status_digest` | Repository-only script | Read `.memory-bank/project_plan-current.md` and `.memory-bank/project_pending-tasks.md` | Print done, in-progress, pending, blocked, and next-task summary. |
| `onto_tracking_count_left` | Repository-only script | Read `.memory-bank/CLAUDE_CODE_APPROACHES_FOR_CODEBASE_TRACKING.md` | Print total tracked items and counts by status. |
| `onto_diff_doc_link_check` | Repository-only script | Read markdown files under `.memory-bank` | Print broken local markdown links and exit non-zero when broken links exist. |

Current implementation state:

- Not implemented as of 2026-06-08.
- Repository search found no approved Stage 0 command names and no `scripts/onto_memory_tools.py`.
- The next valid repository-script task is exactly this Stage 0 slice, not any GitNexus, provider, OAuth, MCP, shell, context, external-agent, or manager-loop candidate.
- The next valid core task is B0 from "Core Operational Backbone Scope", not Stage 0 script work.

Stage 0 non-goals:

- No GitNexus wrapper implementation.
- No provider/OAuth/MCP diagnostics.
- No shell or policy diagnostics.
- No external-agent manager loop.
- No model-visible tools.
- No app-server APIs.
- No Rust changes.
- No automatic tracking-file mutation.

Stage 0 file placement:

- Preferred location: `scripts/onto_memory_tools.py`
- Optional tests: `scripts/tests/test_onto_memory_tools.py`
- Add at most one production script for Stage 0.
- Do not add a new Rust crate.
- Do not add dependencies unless the standard library cannot perform the task.

Stage 0 command contract:

```bash
python3 scripts/onto_memory_tools.py status
python3 scripts/onto_memory_tools.py count-left
python3 scripts/onto_memory_tools.py check-links
```

Stage 0 acceptance criteria:

- Each command has deterministic output.
- `check-links` exits `0` when all local links resolve and non-zero otherwise.
- Missing input files produce a clear error and non-zero exit.
- Output never includes secrets or file contents, only counts, paths, and task labels.
- Tests, if added, use temporary markdown fixtures and do not depend on the live dirty worktree.

## Stage Gate For Any Later Slice

Before implementing Stage 1, B0, or beyond, add a small task card to this ADR with:

- exact tool names
- implementation surface
- files allowed to edit
- files allowed to read
- verification command
- expected output shape
- rollback plan
- GitNexus context or impact requirement if a code symbol will be edited
- storage, retention, and redaction rules if state records are added
- context cap and cache-impact review if model-visible fragments are added

No later slice may contain more than 3 tools unless a senior review explicitly raises the limit.

## Gemini CLI Review Delegation

`ADR_GEMINI_CLI_TOOL_EXTENSIONS.md` originally contained many generic tool/workflow proposals inspired by Gemini CLI. Those proposals are delegated here because they extend the existing lean-ctx-style operational-tool backlog rather than Gemini CLI interop.

Delegated original point ranges:

| Original points | Delegated concern | Existing owner/source |
|---|---|---|
| 41-50, 53-60 | settings/config diagnostics and migration reports | memory-bank scripts, config checks, external-agent import diagnostics |
| 61-72, 74-80 | context, memory, checkpoint, resume summaries | bounded context fragments and memory-bank tooling |
| 81-100 | generic tool inventory, tool schema, lifecycle, readiness | existing tool spec/handler/extension registry |
| 101-120 | file operation guards and reports | repository-only scripts and existing file/diff owners |
| 121-139 | shell, sandbox, permissions diagnostics | shell/policy owners, audit/test-planning only |
| 141-160 | MCP diagnostics, config import, schema/resource/status checks | existing MCP manager/RMCP owners |
| 161-167, 170-180 | slash-command/status/report concepts | repo-only workflow commands unless separately approved |
| 181-220 | plan mode, task tracking, subagent workflow | memory-bank/tracking/external-agent orchestration proposals |
| 225-234, 236-238, 240 | git/diff/CI diagnostics | GitNexus, diff, test-output summarizers |
| 241-254, 257-260 | telemetry/stat/test-duration reports | diagnostics only; privacy/opt-in required for telemetry |
| 261-265, 267-268, 275-280 | ACP/IDE/terminal diagnostics | integration diagnostics only; UI polish remains lefties |
| 281-320 | policy and web-search/fetch guards | existing policy owners and web-search extension |
| 321-330, 332-340 | skills/extensions/custom-command checks | extension registry, skill context caps |
| 341-360 | eval/test/quality planning | test-planning and fixture tooling |

Architecture decision:

- Treat these as backlog labels, not approved implementation tasks.
- Keep the Stage 0-only affordability rule in force.
- Do not add a model-visible tool, app-server API, runtime MCP manager, policy engine, or credential store from these delegated labels without a later task card and GitNexus impact evidence.

## Crush Review Delegation

`ADR_CRUSH_TOOL_EXTENSIONS.md` originally contained broad tool, workflow, diagnostics, protocol, safety, and test proposals inspired by Crush. Most are not Crush-specific and are delegated here as operational backlog labels or task-card inputs.

Delegated original point ranges:

| Original points | Delegated concern | Existing owner/source |
|---|---|---|
| 041-060 | MCP config/resource/status diagnostics | existing MCP manager, RMCP client, MCP status/resource paths |
| 061-080 | skill discovery/catalog/read diagnostics | existing skills extension and bounded context rules |
| 081-100 | LSP diagnostics/status/test planning | future owner-specific LSP task cards; no runtime changes from this ADR |
| 101-120 | hook decision/audit/input-rewrite checks | existing hooks runtime and integration tests |
| 121-140 | permission queue/grant/reporting checks | app-server permission protocol/tests |
| 141-160 | shell/background-job/safe-command diagnostics | shell handler, sandbox, exec-policy owners |
| 161-168, 170-180 | file/view/search/tracking/path utilities | existing tool handlers, path utilities, file tracker concepts |
| 181-220 | todo/session/workspace workflow ideas | memory-bank/task tracking and app-server task-card planning only |
| 241-280 | context and diagnostics/reporting ideas | bounded context fragments and redacted diagnostics |
| 281-288, 293-300 | web/fetch/download/network policy ideas | web-search extension, network policy, and source attribution checks |
| 321-335 | state/history/filetracker persistence ideas | state migration and test-planning task cards |
| 341-360 | app-server protocol/API ideas | app-server v2 ADR/schema/test process |
| 361-380 | safety/redaction/policy ideas | architecture reuse rules and redaction gates |
| 381-398, 400 | test harness and affordability ideas | test-planning and lowest-agent task-card rules |

Architecture decision:

- Treat delegated Crush labels as backlog evidence only.
- Do not implement runtime behavior from this ADR without creating a small owner-specific task card.
- Do not duplicate the existing tool registry, MCP manager, hook runner, permission system, shell runtime, app-server protocol, or context fragment pipeline.
- Items moved to `ADR_CRUSH_TOOL_EXTENSIONS_LEFTIES.md` stay out of this operational backlog unless a later product/UI ADR re-admits them.

## Per-Point Disposition

| Points | Disposition | Similar solution in core | Architecture decision |
|---|---|---|---|
| 1-20 | Keep as new operational tooling | Memory-bank docs, AGENTS/GEMINI rules, and repository docs; no Rust runtime owner | Implement as idempotent repository tooling that edits only `.memory-bank` and project docs with audit entries. |
| 21-40 | Keep as new operational tooling | ADR/tracking files; no Rust runtime owner | Implement as parsers/validators for tracking markdown, not as hidden state or autonomous status mutation. |
| 41-60 | Keep as GitNexus orchestration | GitNexus MCP/CLI is the existing solution; no duplicate graph engine in core | Wrap GitNexus calls and store evidence; do not reimplement symbol graph, impact, rename, or detect-change logic. |
| 61-80 | Keep as architecture guardrails | Existing owners are provider, OAuth, MCP, hooks, shell, context, app-server, and tests linked above | Implement as policy checks that route work to existing owners and block duplicate registries/factories/stores. |
| 81-94 | Keep as provider analysis extensions | `model-provider` already owns `ProviderEngine`, `ProviderDescriptor`, capabilities, and provider selection | Extend descriptor/engine diagnostics and test planning only; runtime provider behavior needs targeted implementation ADRs. |
| 95 | Keep, but ADR-gated | Existing provider runtime is in `model-provider`; no approved external adapter runtime owner | Tool may check prerequisites only. External adapter execution requires a separate security/runtime ADR. |
| 96-100 | Keep as provider diagnostics | `model-provider` descriptor/provider files and tests | Implement as reports that detect drift and readiness; do not mutate provider config automatically. |
| 101-120 | Keep as OAuth/security diagnostics | MCP OAuth storage, login redaction, and Claude import parser already exist | Extend existing storage/redaction/import boundaries; never create a parallel credential broker without ADR approval. |
| 121-140 | Keep as MCP diagnostics | `McpConnectionManager`, MCP status snapshot, RMCP client initialize/call paths | Extend existing MCP status and test harnesses; do not bypass connection manager ownership. |
| 141-159 | Keep as hooks/shell/policy diagnostics | External-agent migration hooks, shell handler/runtime, exec-policy tests | Implement as audit and test-planning tools; any runtime permission change requires impact analysis and tests. |
| 160 | Keep as cross-runtime report | Shell, policy, MCP, context, and app-server owners linked above | Use only for reporting affected surfaces from GitNexus/diff data. |
| 161-167 | Keep as context diagnostics | `ContextualUserFragment`, fragment registration, and extension contributors exist | Enforce hard caps and approved fragment types; do not inject new model context through side channels. |
| 168-180 | Keep as external-agent orchestration | External-agent config/import and sub-agent/hook migration paths exist | Implement as bounded handoff/result parsing around existing files; no autonomous merge without evidence gates. |
| 181-196 | Keep as test/diff readiness tooling | Existing `just test`, core suite, snapshot, and Bazel-lock workflows | Implement as command planners and output summarizers; do not replace repository test runners. |
| 197-200 | Move to lefties | No direct core owner; these are release-management/final-response helpers | Removed from this ADR and tracked in `.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS_LEFTIES.md`. |

## Architecture Decisions Required Before Implementation

- Memory/tracking tools need a small markdown-state parser with explicit schemas for task id, status, owner, evidence, blocker, and source ADR.
- Core-backbone work must start with B0 data contracts and validation only. Do not add migrations, app-server APIs, context fragments, or model-visible tools in the same slice.
- State-backed backbone records must extend existing `StateRuntime` ownership and migrations; do not create a second database root or external storage dependency.
- GitNexus tools are not approved in Stage 0. Later GitNexus task cards must use GitNexus MCP/CLI or the local evidence-binary contract from `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md`; they must persist raw evidence references, not inferred graph state.
- Provider tools must extend `model-provider` diagnostics and descriptor checks; new provider engines must be implemented behind existing `ProviderEngine` and `ProviderKind` seams.
- OAuth tools must use existing MCP OAuth storage and login redaction boundaries; generalized credential brokerage is out of scope.
- MCP tools must use `McpConnectionManager` and RMCP client paths as the source of truth for tool visibility, calls, status, retries, and OAuth persistence.
- Context tools must only inspect or package approved `ContextualUserFragment` paths with hard caps.
- External-agent tools must operate as manager-loop orchestration and must update memory-bank/tracking files before dispatch and after verified completion.

## Candidate Tools

### Memory Bank Tools

1. `onto_memory_bootstrap`: create the initial `.memory-bank` layout from project templates.
2. `onto_memory_read_required`: read the mandatory memory-bank entry files for a task.
3. `onto_memory_status_digest`: summarize project state, done count, pending count, and next task.
4. `onto_memory_plan_sync`: sync `project_plan-current.md` from authoritative tracking files.
5. `onto_memory_pending_sync`: sync `project_pending-tasks.md` from remaining tasks.
6. `onto_memory_arch_sync`: update `project_architecture.md` from approved ADR decisions.
7. `onto_memory_audit_append`: append a dated audit entry after meaningful work.
8. `onto_memory_secret_scan`: scan memory-bank files for tokens, cookies, and credentials.
9. `onto_memory_link_check`: validate internal markdown links after moving ADRs.
10. `onto_memory_move_project_docs`: move project planning markdown into `.memory-bank` safely.
11. `onto_memory_root_doc_guard`: report project markdown files left at the repository root.
12. `onto_memory_conflict_detector`: detect inconsistent status across memory-bank files.
13. `onto_memory_session_resume`: build a compact resume prompt from memory-bank state.
14. `onto_memory_agent_handoff`: create a sub-agent handoff package from plan and architecture files.
15. `onto_memory_decision_timeline`: list ADR decisions in date and dependency order.
16. `onto_memory_lefties_sync`: keep lefties files aligned with rejected or deferred scope.
17. `onto_memory_staleness_check`: flag memory-bank entries not updated after code changes.
18. `onto_memory_compact_export`: export a bounded project context for external agents.
19. `onto_memory_rule_audit`: check AGENTS/GEMINI rules against memory-bank reference rules.
20. `onto_memory_recovery_pack`: assemble the minimal files needed after a lost session.

### Tracking And ADR Tools

21. `onto_tracking_read`: parse the authoritative tracking file into structured tasks.
22. `onto_tracking_next`: select the next unblocked pending task.
23. `onto_tracking_mark_start`: mark a task in progress before dispatch.
24. `onto_tracking_mark_done`: mark a task done only after verification evidence exists.
25. `onto_tracking_mark_blocked`: mark blockers with owner, evidence, and retry path.
26. `onto_tracking_redo_queue`: collect failed tasks that need another implementation pass.
27. `onto_tracking_count_left`: report done, in-progress, pending, blocked, and lefties counts.
28. `onto_tracking_drift_check`: compare ADR scope against tracking-file tasks.
29. `onto_tracking_evidence_gate`: require command output or GitNexus evidence before closure.
30. `onto_tracking_scope_split`: split large tasks into reviewable sub-tasks.
31. `onto_tracking_dependency_graph`: derive task dependencies from ADR text.
32. `onto_tracking_owner_assign`: assign manager/sub-agent owner fields consistently.
33. `onto_tracking_status_table`: generate a compact status table for user answers.
34. `onto_tracking_unblock_advice`: propose senior-engineer unblock paths for stalled tasks.
35. `onto_tracking_done_audit`: verify done tasks have tests and change detection evidence.
36. `onto_tracking_adr_index`: build an index of ADRs, statuses, and implementation state.
37. `onto_tracking_adr_challenge`: challenge an ADR for missing risks, tests, and compatibility.
38. `onto_tracking_adr_update`: patch ADRs with accepted challenge feedback.
39. `onto_tracking_lefties_extract`: move unnatural scope out of core plans.
40. `onto_tracking_external_prompt`: create a bounded prompt for external agents to continue work.

### GitNexus Integration Tools

41. `onto_gitnexus_reindex`: run and summarize `npx gitnexus analyze`.
42. `onto_gitnexus_context_pack`: collect GitNexus context for a symbol or module.
43. `onto_gitnexus_impact_gate`: enforce impact analysis before symbol edits.
44. `onto_gitnexus_high_risk_warn`: stop and report HIGH or CRITICAL risk before edits.
45. `onto_gitnexus_detect_change_gate`: run detect_changes before task closure.
46. `onto_gitnexus_flow_summary`: summarize affected execution flows from changes.
47. `onto_gitnexus_symbol_owners`: identify likely module owners for a proposed change.
48. `onto_gitnexus_arch_reuse_check`: ensure a task extends existing owners instead of bypassing them.
49. `onto_gitnexus_rename_plan`: plan safe Ontocode renames with aliases and migration notes.
50. `onto_gitnexus_rename_gate`: block broad find-and-replace renames.
51. `onto_gitnexus_query_plan`: turn a concept query into candidate implementation files.
52. `onto_gitnexus_test_surface`: list tests directly related to affected symbols.
53. `onto_gitnexus_process_trace`: summarize process traces relevant to a bug or feature.
54. `onto_gitnexus_stale_index_guard`: detect stale-index warnings and request reindex.
55. `onto_gitnexus_change_report`: generate a concise change-risk report for final answers.
56. `onto_gitnexus_subagent_brief`: prepare scoped context for sub-agent implementation.
57. `onto_gitnexus_dependency_hotspots`: identify high-centrality symbols before refactoring.
58. `onto_gitnexus_compat_surface`: locate CLI, config, API, SDK, and persisted-state surfaces.
59. `onto_gitnexus_dead_branch_scan`: find duplicate provider/auth branches after refactors.
60. `onto_gitnexus_tracking_link`: attach impact/detect-change evidence to tracking tasks.

### Architecture Reuse Tools

61. `onto_arch_owner_lookup`: map work areas to existing architecture owners.
62. `onto_arch_duplicate_detector`: detect duplicate registries, factories, resolvers, or stores.
63. `onto_arch_module_size_guard`: flag modules near 500 or 800 line thresholds.
64. `onto_arch_public_api_guard`: identify new public APIs needing ADR and compatibility tests.
65. `onto_arch_config_schema_guard`: detect ConfigToml/schema changes and required commands.
66. `onto_arch_appserver_guard`: enforce app-server v2 payload and schema rules.
67. `onto_arch_context_bound_guard`: enforce bounded context fragment constraints.
68. `onto_arch_dependency_guard`: flag dependency changes needing Bazel lock updates.
69. `onto_arch_build_data_guard`: detect include_str/include_bytes compile data needs.
70. `onto_arch_trait_guard`: check new Rust traits for docs and RPITIT future shape.
71. `onto_arch_bool_param_guard`: detect ambiguous bool or Option positional APIs.
72. `onto_arch_private_module_guard`: recommend private modules and explicit exports.
73. `onto_arch_core_bloat_guard`: challenge unnecessary additions to `codex-core`.
74. `onto_arch_large_diff_splitter`: propose staging when changed lines exceed review limits.
75. `onto_arch_test_harness_finder`: locate existing fixtures before new test helpers are added.
76. `onto_arch_snapshot_guard`: enforce snapshot coverage for UI output changes.
77. `onto_arch_mcp_owner_guard`: route MCP mutations through existing MCP managers.
78. `onto_arch_hook_owner_guard`: route hook work through existing hook matchers and registries.
79. `onto_arch_shell_owner_guard`: route shell work through existing runtime and sandbox modules.
80. `onto_arch_provider_owner_guard`: route provider work through `model-provider` owners.

### Provider Extensibility Tools

81. `onto_provider_descriptor_audit`: summarize provider descriptors, engines, auth schemes, and capabilities.
82. `onto_provider_engine_gap`: compare desired providers against existing engine coverage.
83. `onto_provider_openai_compat_check`: decide whether a provider can use OpenAI-compatible config only.
84. `onto_provider_native_engine_plan`: plan native engines for Claude, Gemini, Copilot, or others.
85. `onto_provider_model_catalog_check`: inspect model catalogs for hard-coded exceptions.
86. `onto_provider_capability_matrix`: produce a provider/model capability matrix.
87. `onto_provider_selector_trace`: explain how a model/provider is selected.
88. `onto_provider_probe_plan`: define safe health and capability probes.
89. `onto_provider_header_profile`: isolate provider-specific headers into reusable profiles.
90. `onto_provider_stream_contract`: validate stream event translation contracts.
91. `onto_provider_tool_mapping`: compare internal tool calls to provider-specific schemas.
92. `onto_provider_error_mapping`: normalize provider error payloads without losing provenance.
93. `onto_provider_auth_boundary`: check that auth logic stays in provider/auth owners.
94. `onto_provider_config_compat`: verify public config changes have compatibility coverage.
95. `onto_provider_external_adapter_gate`: check external adapter ADR prerequisites.
96. `onto_provider_registry_drift`: detect provider-specific branches outside approved owners.
97. `onto_provider_fixture_generator`: generate redacted request/response fixtures for provider tests.
98. `onto_provider_model_alias_audit`: verify model aliases and defaults are explicit.
99. `onto_provider_migration_plan`: plan rollout from hard-coded provider paths to descriptors.
100. `onto_provider_readiness_report`: answer whether the codebase is ready for many providers.

### OAuth And Credential Tools

101. `onto_oauth_sample_validator`: validate redacted Claude/OAuth samples against parser contracts.
102. `onto_oauth_import_status`: summarize import outcomes as complete, partial, empty, or rejected.
103. `onto_oauth_store_mapping`: map imported records to existing OAuth token stores.
104. `onto_oauth_refresh_audit`: check refresh token semantics and expiry handling.
105. `onto_oauth_redaction_test`: assert no tokens, cookies, keys, or paths leak in diagnostics.
106. `onto_oauth_sensitive_query_guard`: scan login callback URLs for sensitive query keys.
107. `onto_oauth_keychain_boundary`: identify keychain, file, and environment credential boundaries.
108. `onto_oauth_overwrite_policy`: model overwrite, merge, and delete behavior for imported credentials.
109. `onto_oauth_provenance_report`: record origin, validation state, and import time without secrets.
110. `onto_oauth_live_runbook_check`: verify live-sample runbooks are complete before enabling import.
111. `onto_oauth_persistence_acceptance`: check persistence acceptance criteria against tests.
112. `onto_oauth_token_shape_diff`: compare credential shapes from multiple provider tools.
113. `onto_oauth_diagnostics_bundle`: produce redacted diagnostics for OAuth failures.
114. `onto_oauth_appserver_gate`: require ADR and compatibility tests for auth API surfaces.
115. `onto_oauth_mcp_integration_check`: verify imported tokens load through MCP OAuth runtime.
116. `onto_oauth_cli_import_plan`: plan safe CLI import UX and rollback behavior.
117. `onto_oauth_fixture_scrubber`: scrub fixture files and reject raw credential values.
118. `onto_oauth_env_leak_scan`: scan test output and logs for credential-looking values.
119. `onto_oauth_identity_matcher`: match imported credentials to provider/server identity.
120. `onto_oauth_readiness_report`: answer whether OAuth import is production-ready.

### MCP Reliability Tools

121. `onto_mcp_status_pipeline_audit`: trace MCP status events through existing processors.
122. `onto_mcp_auth_failure_classifier`: classify MCP auth failures into actionable categories.
123. `onto_mcp_tool_mutation_guard`: enforce use of existing MCP connection managers.
124. `onto_mcp_server_identity_report`: summarize MCP server identity and auth state.
125. `onto_mcp_oauth_persistence_trace`: trace save/load paths for MCP OAuth tokens.
126. `onto_mcp_redacted_status`: generate safe MCP status output for UI or logs.
127. `onto_mcp_retry_policy_audit`: check retry, backoff, and cancellation behavior.
128. `onto_mcp_config_compat_check`: verify MCP config migrations and compatibility tests.
129. `onto_mcp_fixture_generator`: generate MCP test fixtures without secrets.
130. `onto_mcp_diagnostics_bundle`: create bounded MCP support diagnostics.
131. `onto_mcp_tool_schema_diff`: compare MCP tool schemas before and after changes.
132. `onto_mcp_process_trace`: map MCP request lifecycle through GitNexus processes.
133. `onto_mcp_auth_store_reuse`: detect duplicate OAuth persistence logic.
134. `onto_mcp_e2e_test_plan`: propose focused MCP integration tests.
135. `onto_mcp_failure_replay`: replay captured redacted MCP failures in tests.
136. `onto_mcp_timeout_guard`: detect unbounded waits in MCP flows.
137. `onto_mcp_tool_visibility_audit`: explain why tools are visible or hidden.
138. `onto_mcp_transport_matrix`: summarize stdio, SSE, streamable HTTP, and auth support.
139. `onto_mcp_server_health_probe`: define safe server health checks.
140. `onto_mcp_readiness_report`: answer whether MCP changes are ready to close.

### Hooks, Shell, And Policy Tools

141. `onto_hook_registry_audit`: summarize existing hook matchers, registries, and execution flow.
142. `onto_hook_duplicate_guard`: detect new hook paths that bypass existing hook owners.
143. `onto_hook_security_review`: flag risky hook environment, command, and argument handling.
144. `onto_hook_test_plan`: produce integration tests for hook behavior changes.
145. `onto_shell_permission_audit`: trace shell permission parsing and policy evaluation.
146. `onto_shell_launcher_guard`: detect duplicate shell launchers or sandbox bypasses.
147. `onto_shell_sandbox_matrix`: summarize sandbox support and restrictions.
148. `onto_shell_command_redactor`: redact sensitive command arguments in diagnostics.
149. `onto_shell_test_failure_summarizer`: compress large shell/test outputs into actionable failures.
150. `onto_shell_long_running_monitor`: monitor Rust builds without killing processes by PID.
151. `onto_policy_eval_trace`: explain why an action was allowed, denied, or escalated.
152. `onto_policy_config_guard`: verify policy config compatibility and schema needs.
153. `onto_policy_fixture_generator`: create focused policy test fixtures.
154. `onto_policy_regression_scan`: compare behavior before and after policy changes.
155. `onto_sandbox_env_guard`: protect sandbox environment variable behavior from edits.
156. `onto_sandbox_seatbelt_trace`: trace Seatbelt child process environment assumptions.
157. `onto_shell_readiness_report`: answer whether shell/policy changes are ready.
158. `onto_hook_readiness_report`: answer whether hook changes are ready.
159. `onto_policy_readiness_report`: answer whether policy changes are ready.
160. `onto_runtime_surface_report`: summarize shared runtime surfaces affected by a change.

### Context And External-Agent Tools

161. `onto_context_fragment_audit`: list all context fragments and size caps.
162. `onto_context_cache_miss_guard`: identify context changes that cause avoidable cache misses.
163. `onto_context_large_item_detector`: flag individual context items over 1k and 10k token thresholds.
164. `onto_context_struct_guard`: ensure injected fragments use the approved context architecture.
165. `onto_context_budget_planner`: build task context within a fixed token budget.
166. `onto_context_delta_pack`: produce changed-line-only handoff context.
167. `onto_context_session_diff`: summarize what changed since the last memory-bank audit.
168. `onto_external_agent_prompt`: generate an external-agent continuation prompt.
169. `onto_external_agent_scope_guard`: prevent external agents from editing outside assigned scope.
170. `onto_external_agent_result_parser`: parse sub-agent reports into status and evidence.
171. `onto_external_agent_redo_router`: send failed tasks back through a redo loop.
172. `onto_external_agent_conflict_detector`: detect conflicting sub-agent changes.
173. `onto_external_agent_test_gate`: require scoped tests before accepting sub-agent output.
174. `onto_external_agent_memory_update`: update memory-bank after external-agent completion.
175. `onto_external_agent_gitnexus_gate`: require impact and detect-change evidence from agents.
176. `onto_external_agent_review_pack`: assemble a senior-review bundle for finished tasks.
177. `onto_external_agent_blocker_pack`: summarize blockers with exact next unblock step.
178. `onto_external_agent_parallel_plan`: split remaining tasks into independent work packages.
179. `onto_external_agent_merge_order`: define safe merge order for parallel results.
180. `onto_manager_loop`: run manager dispatch, wait, verify, redo, and close cycles.

### Test And Diff Tools

181. `onto_test_scope_planner`: choose `just test -p ...` commands from changed crates.
182. `onto_test_low_memory_runner`: run scoped Rust tests with controlled jobs and target dir.
183. `onto_test_failure_digest`: summarize failed tests without flooding context.
184. `onto_test_flake_classifier`: identify likely flakes versus deterministic failures.
185. `onto_test_fixture_reuse_check`: find existing fixtures before adding new ones.
186. `onto_test_integration_guard`: require integration tests for agent logic changes.
187. `onto_test_snapshot_guard`: find pending UI snapshots and acceptance commands.
188. `onto_test_command_history`: record verification commands and outcomes in memory-bank.
189. `onto_test_unrun_risk_report`: list residual risks when tests were not run.
190. `onto_test_bazel_lock_guard`: check dependency changes and lockfile commands.
191. `onto_diff_scope_summary`: summarize changed files grouped by owner and risk.
192. `onto_diff_unrelated_guard`: separate user/unrelated dirty worktree changes from agent changes.
193. `onto_diff_review_size_gate`: enforce 500/800-line review-size guidance.
194. `onto_diff_public_surface_scan`: find changed public APIs and compatibility surfaces.
195. `onto_diff_secret_scan`: scan diffs for secrets or credential material.
196. `onto_diff_doc_link_check`: validate documentation references after moves or renames.

Moved to lefties: original points 197-200 are tracked in `.memory-bank/ADR_LEAN_CTX_PROJECT_TOOL_EXTENSIONS_LEFTIES.md`.

## Prioritization

Consolidated core-backbone slice:

1. Use `G1 - Operational Evidence State Model` in `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md`.
2. Treat `OperationalTaskCard`, `OperationalEvidenceRecord`, `OperationalGateResult`, and `OperationalReadinessSummary` as workflow domains inside the unified operational evidence model.
3. Verify serialization, deterministic ordering, domain filtering, redaction-state handling, max-size rejection, and schema-evolution notes.

Approved first repository-script slice:

1. `onto_memory_status_digest`
2. `onto_tracking_count_left`
3. `onto_diff_doc_link_check`

These are the only repository tools approved by this ADR without another review. They are cheap, repository-only, and safe for a low-capability coding agent because they read markdown, print bounded summaries, and do not mutate runtime code.

Deferred from the previous first slice:

- `onto_tracking_next`
- `onto_tracking_mark_start`
- `onto_gitnexus_impact_gate`
- `onto_gitnexus_detect_change_gate`
- `onto_test_scope_planner`
- `onto_test_failure_digest`
- `onto_memory_plan_sync`
- `onto_memory_pending_sync`
- `onto_external_agent_prompt`
- `onto_manager_loop`

Reason: those tools require status mutation, GitNexus wrapper design, test-output summarization, or manager-loop behavior. They need separate task cards and senior review before implementation.

## Risks

- A large tool catalog can become shelfware unless it is converted into small tracked implementation slices.
- Calling this a core backbone can invite `codex-core` bloat; the preferred first home is the consolidated operational evidence model from `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md`.
- A backbone can become a second runtime if it executes tools instead of recording evidence and gate results.
- Some tools may overlap with native lean-ctx, GitNexus, or existing project scripts.
- Automation that edits tracking files can hide mistakes unless it records evidence.
- Security tools can give false confidence if they do not reuse the same redaction rules as production code.
- Provider/runtime tools can become architecture bypasses unless they are limited to analysis and orchestration.
- The catalog can be misread as approval to add model-visible tools or app-server APIs; every candidate outside Stage 0 or the consolidated `G1/G2b` path is blocked until a task card re-approves its surface.
- Lean-ctx and GitNexus are development/workflow dependencies here, not Ontocode runtime dependencies.
- GitNexus query quality can be degraded by stale or incomplete indexes; wrapper tools must surface index health instead of silently trusting results.

## Acceptance Criteria For Implementing Any Tool

- The tool is part of an approved stage card.
- The tool has one clear owner and does not duplicate an existing owner.
- The tool has tests or a documented manual verification path.
- The tool output is bounded and safe for model context.
- The tool records enough evidence for tracking-file updates.
- The tool is documented in memory-bank if it changes workflow behavior.
- The tool does not require a new third-party dependency unless the approved task card names it and explains why the standard library or existing repo tooling cannot do the job.
- Any third-party dependency exception must also satisfy the consolidation boundary in `ADR_GITNEXUS_CODE_GRAPH_ADOPTION.md`.

## Acceptance Criteria For Implementing Backbone Stages

- The stage card names exactly one owner and uses the consolidated operational evidence persistence boundary.
- The implementation stores bounded records and redaction status, not raw logs, raw secrets, raw GitNexus databases, lean-ctx cache/session data, compressed output bodies, or full source content.
- The implementation has tests for deterministic output, domain filtering, max-size rejection, redaction-state handling, and stale/superseded evidence.
- The implementation does not expose app-server APIs, model-visible context, public config, CLI behavior, or background workers unless the stage card explicitly approves that surface.
- GitNexus impact and detect-change evidence are recorded for any Rust symbol edit.
