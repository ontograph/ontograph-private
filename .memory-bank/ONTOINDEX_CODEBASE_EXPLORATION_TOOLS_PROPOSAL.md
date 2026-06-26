---
name: OntoIndex Codebase Exploration Tools Proposal
description: Ultratink proposal for new OntoIndex tools that improve codebase exploration without duplicating existing helpers
type: proposal
date: 2026-06-21
status: proposed
last_review: 2026-06-21
---

# OntoIndex Codebase Exploration Tools Proposal

## Context

Current OntoIndex exploration is already strong: `gn_explore` finds concepts, `gn_explain_module` orients files, `gn_find_related` maps symbol neighborhoods, `gn_graph_walk` traverses the graph, `search` runs semantic/Cypher queries, `inspect` shows symbol context, and `impact` computes blast radius.

Agents still waste context on repeated manual stitching: reading files to confirm behavior ownership, grep-hunting tests, tracing property mutation through source, and building mental architecture maps from scratch.

Do not add thin wrappers. Add only tools that compose existing graph primitives into answers agents currently build manually — and tools that expose graph-native edges (ACCESSES, METHOD_OVERRIDES, METHOD_IMPLEMENTS) that zero current tools surface.

## Graph Schema Evidence

The OntoIndex schema stores relationships that zero current tools expose directly:

| Edge type | Meaning | Current exposure |
| --- | --- | --- |
| `ACCESSES` (read/write) | Which functions read/write which properties | None |
| `METHOD_OVERRIDES` | Polymorphic override chains (MRO) | None |
| `METHOD_IMPLEMENTS` | Interface/trait conformance | Indirect via `gn_find_related` |
| `IMPLEMENTS` | Type-level interface conformance | Indirect via `gn_find_related` |
| `MEMBER_OF` | Symbol-to-community membership | `gn_explore` returns cluster info |
| `STEP_IN_PROCESS` | Ordered execution flow steps | `read_mcp_resource` on process URIs |

## Proposed Tools

### Tier 1 — Build First (highest agent time savings)

**`gn_where_is_behavior`**
Input: natural language behavior, e.g. `"where is spawn_agent hidden for subagents"`.
Output: ranked owning files/functions, the guard condition that activates the behavior, callers that trigger it, explicit "not here" exclusions.
Composes: `gn_explore` (semantic search) + `gn_find_related` (neighborhood) + guard-condition extraction.
Why: saves 3-4 roundtrips that agents currently spend narrowing from broad semantic results to exact ownership.

**`gn_explain_flow`**
Input: `query` or `processId`.
Output: ordered narrative — entry point → step-by-step what each symbol does, key branches, side effects, exit — plus owning tests per step.
Composes: `read_mcp_resource` (process steps) + `gn_find_related` (per-symbol callers/callees) + test-file matching.
Why: processes today are symbol-name lists; agents read every symbol's source to understand the flow. This synthesizes the flow into a single readable answer.

**`gn_test_finder`**
Input: `symbol`, `filePath`, or behavior query.
Output: owning test file, relevant test functions, smallest run command, whether targeted coverage exists for the specific behavior.
Composes: test-file heuristic matching + `gn_find_related` (production→test CALLS edges) + test-name grep.
Why: `gn_test_gap` is post-edit; this is pre-edit exploration. Agents currently grep for test files and read them manually.

### Tier 2 — Prevent Common Mistakes

**`gn_arch_map`**
Input: `concept` or `filePath`.
Output: owner module, neighboring modules, allowed extension points, forbidden parallel owners (do not create a second X registry), and the architecture reuse rule for this area.
Composes: `gn_explain_module` + cluster membership + Leiden community cohesion scores.
Why: agents repeatedly create duplicate registries, factories, and side stacks because they don't know the architecture. This answers before they code.

**`gn_entrypoints`**
Input: `filePath`, `symbol`, or `concept`.
Output: CLI commands, RPC routes, TUI slash-command paths, background jobs, config keys, and hook events that reach this code — with the entrypoint file and exact trigger.
Composes: reverse `gn_find_related` (upstream to user-visible surface) + known entrypoint patterns.
Why: answers "how does user action reach this code?" which is more useful than raw callers.

**`gn_invariant_trace`**
Input: invariant text, e.g. `"coding sub-agents cannot spawn sub-agents"`.
Output: the guard code enforcing it (file:line), tests proving it (file:line), gaps where enforcement is missing, and confidence.
Composes: `gn_where_is_behavior` (find the guard) + `gn_test_finder` (find tests) + gap analysis.
Why: ADR and review work needs invariant evidence, not just symbol context. Saves reading guard code and tests separately.

**`gn_change_home`**
Input: intended change description.
Output: exact owner file/module, related config keys, related tests, "do not edit" areas (shared critical paths), and the recommended edit approach.
Composes: `gn_propose_location` (cluster match) + `gn_safe_edit_check` (risk) — but focused on modifying existing behavior, not adding files.
Why: prevents the "edited the wrong file" class of errors. `gn_propose_location` is for new files; this is for modifying existing behavior.

### Tier 3 — Graph-Native (schema edges no current tool exposes)

**`gn_property_access`**
Input: `property` name or qualified path.
Output: every function that reads this property, every function that writes this property, grouped by cluster with access type.
Uses: `ACCESSES` edges with reason `read` or `write`.
Why: schema stores this; zero tools expose it. Critical for understanding data mutation safety and field-level impact.

**`gn_override_chain`**
Input: `method` name.
Output: ordered override chain from base to most-derived, each with file:line, plus callers that dispatch through the base.
Uses: `METHOD_OVERRIDES` edges + MRO ordering.
Why: understanding polymorphic dispatch currently requires reading every override manually.

**`gn_interface_conformance`**
Input: `interface` or `trait` name.
Output: all concrete types implementing it, each implementation's file:line, and which methods are implemented vs defaulted.
Uses: `IMPLEMENTS` and `METHOD_IMPLEMENTS` edges.
Why: answers "who satisfies this contract?" — currently requires grep or indirect `gn_find_related`.

**`gn_config_trace`**
Input: config key path, e.g. `multi_agent_v2.max_concurrent_threads_per_session`.
Output: every function that reads this config key, the config struct path, default value location, and validation function.
Composes: `gn_property_access` (specialized to config structs) + config-key pattern matching.
Why: config tracing is a frequent exploration need; this is the config specialization of property access.

**`gn_data_flow`**
Input: source `symbol` and sink `symbol`.
Output: data propagation path — which properties/parameters carry data, which functions transform it, confidence per step.
Composes: `gn_call_path` (reachability) + `ACCESSES` edges (property reads/writes) + return-type matching.
Why: distinct from call-path — traces data through properties and return values, not just function calls.

### Tier 4 — Specialized, Build When Needed

**`gn_compare_implementations`**
Input: two `symbols` or `filePaths`.
Output: shared callees, divergent behavior patterns, which is canonical, which tests cover each, migration recommendation.
Composes: `gn_find_related` on both + callee-intersection + test-gap comparison.
Why: migration and donor-code reviews; avoids manual side-by-side reading.

**`gn_call_path`**
Input: `from` symbol and `to` symbol.
Output: shortest call path with per-step confidence, missing-edge notes, and alternative paths.
Composes: graph shortest-path + `CALLS` edges + confidence weighting.
Why: direct reachability question, lighter than stateful `gn_graph_walk`.

**`gn_dependency_graph`**
Input: `filePath` or `module` name.
Output: module-level dependency view — what this imports, what imports it, circular-dep warnings, and cluster-boundary crossings.
Composes: `IMPORTS` edges aggregated to module level + cluster membership.
Why: module-level view complements symbol-level `gn_find_related`.

**`gn_migration_path`**
Input: symbol to `rename` or `remove`.
Output: ordered checklist — every caller to update, every import to fix, every test to adjust, every config/doc reference — with confidence.
Composes: `gn_find_related` (full upstream) + doc/config reference scan + test-file matching.
Why: pre-computed migration checklist; currently agents build this manually from impact analysis.

**`gn_review_surface`**
Input: diff or commit range.
Output: changed behavior summary per symbol, existing test coverage per change, missing-test gaps, and risk by blast radius.
Composes: `gn_diff_impact` + `gn_test_finder` per changed symbol + behavioral synthesis.
Why: `gn_diff_impact` gives blast radius; this adds behavioral "what changed" and test-gap analysis.

**`gn_orientation_pack`**
Input: `concept` or repo root.
Output: one-page onboarding — core files, main flows, key tests, common traps, glossary of project-specific terms.
Composes: `gn_explore` (top processes) + cluster overview + `gn_explain_module` on top files + `gn_test_finder` on top symbols.
Why: reduces repeated agent startup cost on unfamiliar codebases.

**`gn_historical_context`**
Input: `symbol` or `filePath`.
Output: last meaningful commit, commit message, related PR if available, and co-changed files at that commit.
Composes: git-blame + commit-message extraction + co-change history.
Why: connects code to intent; currently requires manual `git log` and `git blame`.

## First Implementation Slice

Build only these first:

1. `gn_where_is_behavior` — most common agent question
2. `gn_explain_flow` — highest context savings
3. `gn_test_finder` — most frequent manual work

Add Tier 2 after these three prove stable. Add Tier 3 when graph-native edge exposure is needed. Add Tier 4 on demand.

## Rejected

- New storage, SQLite tables, or persistence layers.
- A separate exploration UI or dashboard.
- Dozens of one-purpose tool aliases.
- Tools that only rename existing `search`, `inspect`, or `gn_find_related`.
- A `gn_code_smells` tool — too heuristic, belongs in audit tools, not exploration.

## Implementation Notes

Each proposed tool is a super-function composing existing primitives. No new graph index features required for Tiers 1-2. Tiers 3-4 require exposing existing schema edges (ACCESSES, METHOD_OVERRIDES, METHOD_IMPLEMENTS) through the MCP surface — those edges already exist in the graph but have no callable tool.

All tools must:
- Return bounded output (default 25 items, max 100).
- Report stale-index warnings when the graph is behind HEAD.
- Support `repo` parameter for multi-repo sessions.
- Never require write access to the indexed repository.
