---
name: Code Harness Chosen Tools And Algorithms
description: ADR selecting donor-derived tool families and bounded algorithms for the repo-owned code-harness headless plugin
type: adr
date: 2026-06-28
status: accepted - donor tool families selected, runtime remains gated
---

# ADR: Code Harness Chosen Tools And Algorithms

## Context

Parent decision:
[ADR_CODE_HARNESS_HEADLESS_PLUGIN.md](ADR_CODE_HARNESS_HEADLESS_PLUGIN.md).

Evidence basis for this ADR:

- OntoIndex review of the current `codex` repo to keep the plugin inside
  existing plugin, CLI, and MCP owners.
- Direct donor-source review under `tmp/code-harness/pypi-src/` for:
  - [code-review-graph](../tmp/code-harness/pypi-src/code_review_graph-2.3.6/README.md)
  - [codegraph-mcp-server](../tmp/code-harness/pypi-src/codegraph_mcp_server-0.8.0/README.md)
  - [cognitx-codegraph](../tmp/code-harness/pypi-src/cognitx_codegraph-0.1.118/README.md)

The donor-package trees were indexed, but exact donor-package repo targeting via
OntoIndex MCP was ambiguous in this session. Exact algorithm details below are
therefore taken from the unpacked donor source and README material, while
OntoIndex was used to keep the Ontocode-side owner alignment narrow.

This ADR chooses tool families and algorithm shapes only. It does not reopen
the parent ADR's runtime gates.

## Decision

For `plugins/ontocode-code-harness`, the accepted donor-derived tool families
are:

- `harness_status`
- `harness_update`
- `quality_explain`
- `quality_review`
- `quality_file_check`
- `quality_arch_check`

These are repo-owned tool contracts, not package adoptions.

They must follow five rules:

- OntoIndex-first when an existing repo-owned OntoIndex surface already
  satisfies the contract.
- Read-only by default; no write-capable graph mutation tools in the plugin
  surface.
- Explicit update only; no daemon, watch loop, or hidden background refresh.
- Bounded output only; every tool must support a small deterministic response.
- Small allowlisted surface only; the plugin must not expose every possible
  backend capability.

`quality_validate` is not selected as a separate first-cut tool. Its useful
parts belong inside `quality_arch_check` until a distinct validation contract
is justified.

## Chosen Tool Families

### `harness_status`

Purpose:
cheap, deterministic readiness and freshness summary for headless use.

Accepted algorithm:

1. Validate plugin manifest, local config, and backend selection without
   mutating anything.
2. Probe only read-only status sources:
   existing OntoIndex health or repo stats, plugin loadability, configured
   command availability, and any already-existing graph metadata.
3. Collect bounded repository-health facts:
   backend kind, index presence, freshness state, last update time when
   available, enabled tool subset, and degraded prerequisites.
4. Derive a single readiness state:
   `ready`, `degraded`, or `blocked`.
5. Emit machine-readable output with one exact next step such as
   `run harness_update` or `backend unavailable`.

Accepted output shape:

- summary state
- backend authority
- freshness state
- compact counts or stats when available
- enabled tools
- missing prerequisites
- exact next command

Primary donor ideas:

- `code-review-graph`: `status`, read-only `detect-changes --brief`, explicit
  distinction from `update --brief`
- `codegraph-mcp-server`: `stats` and `codegraph://stats`
- `cognitx-codegraph`: `stats`, read-only-by-default MCP stance

Implementation gate:
first runtime tool after package skeleton.

### `harness_update`

Purpose:
explicit refresh of the selected authority without introducing a background
service.

Accepted algorithm:

1. Require explicit invocation; never auto-run on save, commit, or session
   start.
2. Resolve update mode from bounded vocabulary:
   `full`, `incremental`, or `since-ref`.
3. Select changed files from git diff or explicit path input.
4. Prefer targeted refresh:
   donor vocabulary allows SHA-based incremental updates and diff-based refresh;
   Ontocode should reuse whichever existing owner can do this without adding a
   second runtime.
5. Recompute only the minimal post-processing needed by the selected tool
   surface.
6. Return refreshed counts, elapsed time, touched-file count, and the new
   status snapshot.

Accepted constraints:

- no watch mode
- no daemon
- no hidden hooks
- no write-through MCP tool exposed to arbitrary clients

Primary donor ideas:

- `code-review-graph`: `update`, `update --brief`, SHA-256 incremental
  vocabulary
- `cognitx-codegraph`: `--update` and `--since <git-ref>`
- `codegraph-mcp-server`: incremental reindex concept, but not its watch mode

Implementation gate:
deferred until the concrete backend authority is chosen.

### `quality_explain`

Purpose:
bounded structural explanation for a file, symbol, or changed area.

Accepted algorithm:

1. Resolve the target as file, symbol, or scoped diff.
2. Gather a bounded neighborhood:
   direct callers, direct callees, imports, tests, package or module role, and
   optionally one community-level summary when the backend already has it.
3. Rank neighbors by directness and relevance to the target.
4. Emit a compact explanation with:
   what this thing is, what depends on it, what it depends on, and what nearby
   tests or entry points matter.
5. Truncate aggressively rather than widening scope.

Accepted output shape:

- short human summary
- JSON neighborhood facts
- affected files or symbols
- optional compact metadata about why this path is smaller than a full-file
  read

Primary donor ideas:

- `codegraph-mcp-server`: `local_search`, `global_search`,
  `analyze_module_structure`
- `code-review-graph`: minimal review context and optional `context_savings`
  style metadata
- `cognitx-codegraph`: `describe_function`, `describe_schema`,
  callers/callees summaries

Implementation gate:
allowed after `harness_status`; lower risk than full review.

### `quality_review`

Purpose:
headless, scope-aware review of a diff or path set with graph-expanded impact.

Accepted algorithm:

1. Resolve scope from bounded inputs:
   `staged`, `unstaged`, `branch`, `since-ref`, or explicit paths.
2. Require an explicit freshness policy:
   inspect current status first, then either review against the current graph or
   run `harness_update` before continuing.
3. Seed the review set from changed files or symbols.
4. Expand the review set through bounded blast-radius traversal over calls,
   imports, inheritance, tests, and other already-supported dependency edges.
5. Run review heuristics only on the changed set plus impacted neighborhood.
6. Optionally attach architecture or file-check findings when they land inside
   the same bounded scope.
7. Emit Markdown and JSON outputs with severity, evidence location, and review
   scope summary.

Accepted review semantics:

- advisory by default
- non-zero exit only in explicit CI or strict-threshold mode
- findings must distinguish `directly changed` from `impact-only`
- optional token-savings metadata is allowed later, not required in the first
  slice

Primary donor ideas:

- `code-review-graph`: `detect-changes`, PR review flow, blast-radius BFS,
  risk-panel shape, optional `context_savings`
- `codegraph-mcp-server`: search plus neighborhood retrieval, but not prompt
  catalogs or shell execution
- `cognitx-codegraph`: scope-aware graph checks and confidence framing

Implementation gate:
deferred until `harness_update` and `quality_explain` are trustworthy.

### `quality_file_check`

Purpose:
fast advisory checks on one file or a small changed-file set.

Accepted algorithm:

1. Stay file-local by default.
2. Derive bounded structural signals:
   fan-out, import count, caller count, callee count, TODO or FIXME density,
   missing adjacent tests, obvious orphan risk, and registration or entry-point
   gaps when the backend already models them.
3. Attach a confidence label to each finding when the signal is inferred rather
   than directly observed.
4. Emit only actionable findings; do not dump raw graph state.

Accepted constraints:

- advisory only
- no repo-wide scan by default
- no duplicate overlap with full `quality_review` output unless explicitly asked

Primary donor ideas:

- `cognitx-codegraph`: confidence labels, orphan detection, stats and schema
  sanity framing
- `codegraph-mcp-server`: file structure and module-structure summaries
- `code-review-graph`: changed-area-first review discipline

Implementation gate:
after the review finding schema exists.

### `quality_arch_check`

Purpose:
policy-driven structural checks for impacted scope first, whole repo only when
explicitly requested.

Accepted algorithm:

1. Load repo-owned policy configuration from Ontocode-owned files only.
2. Prefer impacted-scope evaluation for normal headless runs.
3. Run bounded checks such as:
   import cycles, cross-package violations, layer bypasses, coupling ceilings,
   orphan detection, and basic graph sanity counts.
4. Respect suppressions only when each suppression carries an explicit reason.
5. Emit JSON and Markdown suitable for CI, but default to advisory mode outside
   strict CI paths.

Accepted first-cut semantics:

- repo-owned policy config
- bounded checks only
- no live Cypher shell surface
- no direct donor database dependency

Primary donor ideas:

- `cognitx-codegraph`: `arch-check`, `validate`, suppression discipline,
  confidence-aware filtering
- `code-review-graph`: architecture overview and affected-scope framing
- `codegraph-mcp-server`: bounded stats and structure summaries

Implementation gate:
last selected tool family; depends on a stable finding format.

## Tool-Surface Policy

The plugin must expose an allowlisted surface only. The chosen tool families
above are the whole approved set for now.

Rejected from the public plugin surface even if a backend can do them:

- arbitrary shell execution
- graph wipe or file reindex mutation tools
- prompt catalogs as first-class runtime features
- background watch controls
- platform installer or rules-writer commands
- benchmark, wiki, diagram, or export commands

This rule is directly borrowed from the donor pattern where tool subsets are
explicitly configurable, but here it is stricter: the Ontocode plugin starts
small and stays small unless a later ADR widens it.

## Donor Mapping

### `code-review-graph`

Keep:

- explicit read-only `status` or `detect-changes` versus mutating `update`
- changed-set-first review flow
- bounded blast-radius expansion
- optional compact `context_savings` metadata
- allowlisted MCP surface

Do not keep:

- installer, daemon, watch loop, hook automation, wiki, eval, benchmark stack,
  or multi-platform integration writer

### `codegraph-mcp-server`

Keep:

- self-contained stdio-first MCP shape
- bounded `stats`, file structure, and neighborhood explanation patterns
- local versus global search distinction as a way to bound explanation scope

Do not keep:

- `execute_shell_command`
- SSE or HTTP-first runtime assumptions
- watch mode
- broad prompt catalog surface

### `cognitx-codegraph`

Keep:

- `arch-check` and `validate` ideas
- `--update` and `--since` refresh vocabulary
- read-only default surface
- confidence labels on extracted relationships
- suppression discipline for architecture findings

Do not keep:

- Neo4j bootstrap
- interactive `init`
- file watchers or git hooks
- write-enabled MCP tools
- external-agent `audit` launcher
- platform installation system

## Rejected Shapes

The following remain explicitly out of scope for this ADR:

- any required external database or graph service
- any new long-lived daemon or watcher
- any plugin-managed hidden subprocess lifecycle
- any write-capable MCP tool
- any broad audit-agent launcher or permission-bypass runtime
- any platform-wide installer or rules-file mutator
- any second review backend when OntoIndex or an existing owner already covers
  the need

## Implementation Order

The intended order remains narrow:

1. Package skeleton only in `plugins/ontocode-code-harness`
2. `harness_status`
3. `quality_explain`
4. `harness_update`
5. `quality_review`
6. `quality_file_check`
7. `quality_arch_check`

Only step 1 is already approved by the parent ADR. Every later step still needs
existing-owner proof at dispatch time.

## Reopen Gates

Do not widen this ADR unless one of these is proven:

- OntoIndex cannot satisfy the early `status` or `explain` contract inside the
  current owners.
- A separate `quality_validate` tool is needed because `quality_arch_check`
  would otherwise mix incompatible CI and advisory semantics.
- A concrete user workflow requires more than the six chosen tool families.
- A bounded strict-mode contract is needed for CI and cannot be expressed as
  thresholds on the selected tools.

Absent that evidence, this ADR stands as the narrow accepted tool set for the
headless code-harness plugin.
