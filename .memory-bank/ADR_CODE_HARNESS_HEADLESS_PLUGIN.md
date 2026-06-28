---
name: Code Harness Headless Plugin
description: ADR for a repo-owned headless harness and code-quality plugin derived from tmp/code-harness donor ideas
type: adr
date: 2026-06-28
status: challenged - package skeleton only, runtime gated
---

# ADR: Code Harness Headless Plugin

## Context

Donor source: [tmp/code-harness](../tmp/code-harness).

The donor documentation describes a broad CodeGraph integration stack with:

- lifecycle hooks that exchange JSON through stdin/stdout and inject bounded
  additional context;
- headless review and status commands with machine-readable output;
- best-effort post-commit refresh and review traces;
- LSP diagnostics for complexity, dead code, and security findings;
- plugin-driven workflow commands such as `review`, `audit`, `explain`,
  `update`, `status`, `next`, and `continue`.

Relevant donor evidence:

- [integrations/CLAUDE_CODE_GIT.html](../tmp/code-harness/codegraph.ru/docs/ru/integrations/CLAUDE_CODE_GIT.html)
- [integrations/OPENCODE_PLUGIN.html](../tmp/code-harness/codegraph.ru/docs/ru/integrations/OPENCODE_PLUGIN.html)
- [guides/CODE_REVIEW.html](../tmp/code-harness/codegraph.ru/docs/ru/guides/CODE_REVIEW.html)
- [guides/CODE_REVIEW_HOOKS.html](../tmp/code-harness/codegraph.ru/docs/ru/guides/CODE_REVIEW_HOOKS.html)
- [guides/CLI_GUIDE.html](../tmp/code-harness/codegraph.ru/docs/ru/guides/CLI_GUIDE.html)
- [guides/19-standards-check.html](../tmp/code-harness/codegraph.ru/docs/ru/guides/19-standards-check.html)
- [guides/07-test-coverage.html](../tmp/code-harness/codegraph.ru/docs/ru/guides/07-test-coverage.html)
- [guides/OPENCODE_QUICK_START.html](../tmp/code-harness/codegraph.ru/docs/ru/guides/OPENCODE_QUICK_START.html)

Current Ontocode owners already cover the architectural seams this proposal
needs:

- plugin manifest and interface metadata in
  [ontocode-rs/core-plugins/src/manifest.rs](../ontocode-rs/core-plugins/src/manifest.rs)
- plugin MCP loading in
  [ontocode-rs/core-plugins/src/loader.rs](../ontocode-rs/core-plugins/src/loader.rs)
- plugin cache, marketplace, and config loading in
  [ontocode-rs/core-plugins/src/manager.rs](../ontocode-rs/core-plugins/src/manager.rs)
- headless harness overrides in
  [ontocode-rs/core/src/config/mod.rs](../ontocode-rs/core/src/config/mod.rs)
  and [ontocode-rs/exec/src/lib.rs](../ontocode-rs/exec/src/lib.rs)
- plugin CLI surfaces in
  [ontocode-rs/cli/src/plugin_cmd.rs](../ontocode-rs/cli/src/plugin_cmd.rs)
- existing repo-local plugin packaging precedent in
  [plugins/ontocode-lean-ctx](../plugins/ontocode-lean-ctx)

The donor is useful as a headless workflow catalog. It is not approval to copy
its larger runtime architecture.

## Problem

Ontocode does not yet have a repo-owned plugin package dedicated to headless
harness workflows and code-quality checks that:

- works cleanly in non-interactive mode;
- reuses existing plugin, hook, MCP, and config owners;
- produces structured outputs suitable for CI, automation, and batch review;
- avoids introducing a second harness runtime, second API stack, or donor-owned
  service dependency.

## Decision

Pursue `repo-owned headless plugin package`, but do not dispatch runtime work
from this ADR yet.

Create a new plugin package named `ontocode-code-harness` under `plugins/`
that reuses current Ontocode owners. The first approved slice is packaging
only:

- plugin manifest and interface metadata
- README with headless workflow guidance
- optional `.mcp.json` only when it points at an already-existing repo-owned
  command or helper

Do not add plugin-bundled hooks, metrics, traces, SARIF, skills, new MCP tools,
or runtime helper scripts until a specific existing-owner gap is proven.

The plugin must not create a new core harness framework, plugin runtime,
frontend, API tier, or benchmark platform.

## Plugin Shape

Proposed package root:

- `plugins/ontocode-code-harness/.codex-plugin/plugin.json`
- `plugins/ontocode-code-harness/README.md`
- optional `plugins/ontocode-code-harness/.mcp.json`

Manifest capabilities should stay minimal:

- `mcp` only if an existing command or helper already backs it

Like `ontocode-lean-ctx`, the plugin may install and load while the runtime
surface fails closed if required local prerequisites are missing.

## Runtime Contract

- plugin id: `ontocode-code-harness`
- ownership: Ontocode repo
- primary mode: headless and automation-first
- preferred MCP transport:
  - `stdio` first when the backend is local and short-lived
  - `http` only when a long-lived service is strictly required
- runtime ownership:
  - plugin manager loads config and metadata only
  - runtime helper processes are started by explicit repo-owned scripts or
    existing CLI commands, not by hidden plugin-manager background spawning
- failure mode:
  - plugin package load must fail closed on invalid manifest or invalid MCP
    config
  - optional hooks, if later approved, must fail open and produce no context
    when unavailable
  - any quality-check command, if later approved, must fail closed when it
    cannot produce a trustworthy result
  - status commands must report degraded or unavailable dependencies without
    pretending analysis ran

## Keep: Headless Ideas Worth Adopting

### 1. JSON hook contract

Keep as a possible later hook-owner extension, not phase-1 scope. The donor
pattern where hooks read JSON from stdin and return bounded JSON to stdout is
usable only if a current hook gap is reproduced.

Adopt:

- stdin JSON input
- bounded stdout JSON output
- empty result on no-op
- top-level exception trapping for best-effort hooks

### 2. Session and prompt enrichment

Keep as a possible later context-fragment extension, not phase-1 scope.
Lightweight session-start and prompt-submit enrichment is allowed only when it
remains bounded, optional, and implemented through current context-fragment
owners.

Future approved work may provide:

- project context summary
- index freshness status
- bounded entity lookup hints
- file or symbol-level risk hints before edits

This must stay inside current hook/context limits and must not inject large or
unbounded fragments.

### 3. Pre-edit quality gate

Keep as a possible later advisory check, not phase-1 scope. A bounded pre-edit
warning path may cover:

- high complexity
- high fan-out
- TODO or FIXME markers
- stale or risky interface surfaces

This should remain advisory. It must not become a new approval or policy engine.

### 4. Headless review command family

Keep the donor command pattern only as vocabulary for future wrappers around
existing Ontocode and OntoIndex owners:

- `review`
- `audit`
- `explain`
- `status`
- `update`
- `next`
- `continue`

Do not implement this family until each command names its current owner,
backing implementation, output cap, failure behavior, and validation command.

### 5. Structured output

Keep machine-readable headless outputs:

- markdown for humans
- JSON for automation
- SARIF only after JSON or markdown review output is stable and findings map
  cleanly to file or symbol locations

The command surface should carry clear exit semantics for CI:

- `0` for clean or advisory-only results
- non-zero for blocking or high-severity findings

### 6. Scope-aware filtering

Keep donor-style scope-aware downgrading when the analysis corpus is partial.

Examples:

- suppress missing-test claims when tests are known out of scope
- downgrade blast-radius claims when the graph or parse scope is incomplete
- keep method-local complexity and syntax findings unaffected

### 7. JSONL metrics and trace artifacts

Keep as possible later persistence only after the command surface exists.
Lightweight operational artifacts may include:

- JSONL metrics for hook or command execution
- per-run status snapshots
- optional watchable trace files for long-running review or update workflows

Before implementation, define path, retention, cleanup, concurrency behavior,
and redaction rules. These artifacts must stay local, bounded, and repo-owned.

### 8. Headless plan/act/review loop

Keep the donor’s core headless loop:

1. `status`
2. `update`
3. focused act step
4. targeted tests
5. `review` or diagnostics rerun

This is the right workflow shape, but it is not a new orchestrator. It does not
require a new runtime.

## Narrow Phase 1

The only approved phase-1 work is:

- scaffold `plugins/ontocode-code-harness`
- add `.codex-plugin/plugin.json`
- add README guidance for headless use
- optionally add `.mcp.json` only if backed by an existing repo-owned command
  or helper

One command may be added in the same slice only if it is `harness_status`, is
read-only, has bounded output, and is backed entirely by existing plugin,
headless config, or OntoIndex status paths.

Everything else is runtime work and needs a separate reopen gate.

## Deferred Tools

Do not expose plugin MCP tools directly in phase 1. A future tool must specify:

- current owner
- backing implementation
- output schema and hard cap
- failure behavior
- validation command

## Architecture Boundaries

### What stays in existing owners

- plugin loading and manifest parsing:
  `ontocode-rs/core-plugins`
- hook execution and matching:
  `ontocode-rs/hooks`
- headless configuration and override behavior:
  `ontocode-rs/core` and `ontocode-rs/exec`
- code intelligence and blast-radius analysis:
  OntoIndex and existing code-intelligence owners
- CLI or app-server user-facing command surfaces:
  existing CLI and app-server owners

### What the plugin owns

- package identity
- README workflow guidance
- optional thin config that points at existing owners
- documentation and install guidance

### What must not be added

- no second harness composition framework
- no second plugin runtime or plugin registry
- no second MCP runtime
- no second API server or dashboard
- no benchmark or evolution platform
- no hidden dependency on donor repo, donor checkout, donor daemon, or donor
  cloud service

## Rejected Donor Shapes

Reject the following donor-derived shapes for this ADR:

- dashboard-first architecture
- dedicated plugin-side API service as a default requirement
- second frontend or operator console
- benchmark, leaderboard, or RL loops
- meta-harness validator or replay subsystem as a new stack
- a second session system for continuity or memory
- plugin-managed hidden background service spawning

## Acceptance Criteria

- A repo-local plugin package can be installed and loaded through existing
  `ontocode-core-plugins` owners.
- Headless commands work without requiring interactive TUI flows.
- Optional hooks degrade gracefully and do not block unrelated work.
- Structured outputs are bounded and suitable for automation.
- The plugin does not introduce a required upstream checkout or third-party
  runtime dependency.
- The plugin manager is not taught to spawn background workers for this plugin.
- Existing CLI or app-server flows remain the authority for core behavior.
- Phase 1 does not introduce hooks, trace files, metrics, SARIF, skills, or new
  MCP tools unless the `harness_status` exception above is satisfied.

## Implementation Order

1. Scaffold the plugin package with manifest and README.
2. Add `.mcp.json` only if it is backed by an existing repo-owned command or
   helper.
3. Optionally add read-only `harness_status` only if the backend is already
   available and output is bounded.
4. Stop. Reopen runtime surfaces only with evidence.

## Reopen Gates

- Add hooks only after a specific current hook or context gap is reproduced.
- Add `quality_review` only after choosing one backend authority: CLI,
  app-server, or direct OntoIndex tooling.
- Add JSONL metrics or traces only after a long-running command exists and path,
  retention, cleanup, concurrency, and redaction rules are specified.
- Add SARIF only after stable JSON or markdown review output exists.
- Add HTTP runtime only after stdio or direct command execution is proven
  insufficient.
- Add skills only after the README and command/tool surface are stable enough to
  compose without inventing a second workflow runtime.

## TODO: Next Tools And Ideas

These are deferred candidate follow-ups from `tmp/code-harness`. They are not
approved runtime scope yet.

### Candidate tools

- `harness_status`
  - Minimal shape: read-only readiness report covering plugin load status,
    OntoIndex freshness, degraded dependencies, and exact next command.
  - Donor idea: `dogfood status`, `/status`, `codegraph_project_context_status`,
    and unified readiness output.
  - Reopen gate: prove the current plugin or OntoIndex status surfaces are not
    enough for one bounded status report.
- `harness_update`
  - Minimal shape: explicit freshness check or update wrapper over one existing
    owner, with structured output and no hidden background work.
  - Donor idea: `/update`, `codegraph_runtime_cpg_watch_run update`, explicit
    freshness fields like `commits_behind`, `needs_update`, and `next_command`.
  - Reopen gate: choose one current backend authority and prove status alone is
    insufficient for headless workflows.
- `quality_review`
  - Minimal shape: staged, diff, or explicit-file review with markdown or JSON
    output first.
  - Donor idea: CLI review over `--staged`, `--base-ref`, `--files`, optional
    security analysis, and scope-aware aggregation.
  - Reopen gate: pick one backend authority and define exit codes, output cap,
    and validation path before adding the command.
- `quality_explain`
  - Minimal shape: explain one file, symbol, or affected surface using existing
    code-intelligence owners only.
  - Donor idea: `/explain` helper and review fallback that surfaces affected
    methods, callers, and local risk.
  - Reopen gate: show that current `inspect`, `impact`, or review surfaces do
    not already answer the bounded explain case.
- `quality_file_check`
  - Minimal shape: file-local advisory pass for complexity, fan-out, TODO or
    FIXME density, and risky interface registration gaps.
  - Donor idea: pre-edit warnings, registration checks, and practical dead-code
    filtering for changed files.
  - Reopen gate: prove one concrete file-local warning path is missing from
    current owners and keep the result advisory-only.
- `quality_test_gap`
  - Minimal shape: bounded changed-symbol test-gap summary with optional runtime
    coverage hints when approved evidence exists.
  - Donor idea: missing-test checks, hybrid coverage detection, and error-path
    test recommendations.
  - Reopen gate: current `gn_test_gap` or related test surfaces must prove
    insufficient for the desired headless report.
- `trace_status`
  - Minimal shape: inspect one local trace or status artifact for a long-running
    review or update task.
  - Donor idea: review-trace status, pending summaries, and watchable status
    snapshots.
  - Reopen gate: first land a long-running explicit command that already writes
    bounded local artifacts.

### Candidate ideas

- Scope-aware filtering
  - Keep the donor downgrade pattern for `dead_code`, `missing_test`, and
    blast-radius claims when analysis scope is partial.
  - Reopen gate: attach it to one specific review or test-gap command, not as a
    global policy layer.
- Structured output tiers
  - Keep markdown and JSON first; add SARIF only after a stable finding schema
    exists.
  - Reopen gate: one command must already emit stable file or symbol findings.
- Exact next-step guidance
  - Keep donor-style `next_step` or `next_command` fields for headless use.
  - Reopen gate: add only to commands that already have a clear owner and
    bounded failure matrix.
- Fail-open optional enrichment
  - Keep best-effort hook enrichment optional and empty on failure.
  - Reopen gate: reproduce one hook-driven context gap and implement through
    existing hook and context-fragment owners.
- Local metrics and trace artifacts
  - Keep JSONL metrics and status snapshots only as local operational evidence.
  - Reopen gate: define path, retention, cleanup, concurrency, and redaction
    before writing any file.
- Headless plan or act or review loop guidance
  - Keep the workflow shape `status -> update -> act -> test -> review` as
    documentation and command composition only.
  - Reopen gate: do not add a scheduler, queue, or second orchestrator.

## Downloaded PyPI Package Review

Reviewed package set under `tmp/code-harness/pypi-src/`:

- `code-review-graph`
- `codegraph-mcp-server`
- `cognitx-codegraph`
- `codegraphcontext`
- `code-graph-rag`
- `codegraph-agent`
- `deepcodegraph`
- `codegraph`
- `python-code-graph`
- `codepropertygraph`

The review basis was the indexed source trees plus package README and
entrypoint metadata. This section consolidates only donor ideas that are useful
for the headless harness ADR. It does not approve package adoption.

### Package triage

- `code-review-graph`
  - Strongest donor for local-first review context, explicit `status` or
    `detect-changes` versus `update`, MCP tool allowlisting, and token-savings
    reporting.
  - Reject its installer, daemon, hooks, watch mode, evaluation, wiki, and
    multi-platform wiring as parallel runtime or automation layers.
- `codegraph-mcp-server`
  - Strong donor for a self-contained stdio-first MCP shape with `index`,
    `stats`, `query`, and bounded structure-reading tools.
  - Reject SSE server, watch mode, and `execute_shell_command`.
- `cognitx-codegraph`
  - Strong donor for `arch-check`, `validate`, `stats`, explicit platform
    integration boundaries, and read-only graph or schema tools.
  - Reject Neo4j bootstrap, interactive `init`, file watchers, git hooks,
    write-capable MCP tools, and external-agent `audit` launching in bypass
    mode.
- `codegraphcontext`
  - Confirms the dual CLI plus MCP pattern and multi-backend graph storage as a
    recurring market shape.
  - Reject the graph-database stack, FastAPI or uvicorn surfaces, and live
    watch behavior for this ADR.
- `code-graph-rag`
  - Narrow donor only for AST-targeted edit previews and explicit MCP
    integration vocabulary.
  - Reject Memgraph, realtime updater, cloud-hosted path, broad agentic edit
    runtime, and external vector or graph service coupling.
- `codegraph-agent`
  - No clear harness-specific donor idea survived initial review; it reads as a
    broader agent framework rather than a bounded headless quality-tool donor.
- `deepcodegraph`
  - Diagram and export focused. No headless harness-tool idea beats current
    owners.
- `codegraph`
  - Simple dependency-graph visualization. Useful as evidence that raw graph
    rendering is not enough for the retained ADR scope.
- `python-code-graph`
  - Basic Python graph generation only. No meaningful harness or quality-tool
    idea survived.
- `codepropertygraph`
  - Minimal CPG implementation, but too low-level for a plugin-surface donor by
    itself.

### Consolidated keep-only ideas

- Cheap read-only status versus explicit update
  - Repeated across `code-review-graph`, `codegraph-mcp-server`, and
    `cognitx-codegraph`.
  - Good fit for `harness_status` first and `harness_update` later.
- Tool-surface allowlisting
  - `code-review-graph` exposes a limited MCP tool subset instead of forcing
    every tool on every client.
  - Good fit for keeping the future plugin MCP surface intentionally small.
- Bounded repository statistics and schema introspection
  - `stats`, `describe_schema`, file structure, and package listing are strong
    patterns for read-only readiness and explain flows.
  - Good fit for `harness_status` and `quality_explain`.
- Architecture and validation checks
  - `cognitx-codegraph` contributes `arch-check`, `validate`, and confidence or
    scope framing ideas.
  - Good fit for a later advisory `quality_file_check` or repo-level
    `quality_review`.
- Token-savings or context-savings metadata
  - `code-review-graph` provides a concrete pattern for showing why the graph
    path is worth using.
  - Good fit only as optional metadata after the core command outputs exist.
- Incremental indexing vocabulary
  - Several packages distinguish full build, incremental update, and diff-based
    refresh.
  - Good fit only for explicit `harness_update`, not background automation.

### Consolidated deferred ideas

- File or package scoping and ignore-file support
  - Good future fit once a concrete review backend is chosen.
- Read-only graph prompts or query catalogs
  - Potentially useful as plugin-bundled guidance after the command surface is
    stable.
- Confidence or scope labels on findings
  - Good fit only after the finding schema exists and stays bounded.

### Consolidated rejections

- No external database, container, or hosted graph service requirement
  - Reject Neo4j, FalkorDB, KuzuDB, LadybugDB, Memgraph, and cloud-hosted graph
    paths for this ADR.
- No background daemon, watcher, or automatic re-index service
  - Reject `crg-daemon`, watch mode, realtime updaters, and persistent
    background refresh loops.
- No platform-wide installer or rules-writer runtime
  - Reject broad `install <platform>` flows that mutate many AI-tool configs as
    a default requirement.
- No write-capable MCP graph mutation or shell execution
  - Reject `wipe_graph`, `reindex_file`, and shell-command tools as donor scope
    for this plugin ADR.
- No external-agent self-audit launcher
  - Reject flows that spawn `claude`, `codex`, `gemini`, or other agents in
    bypass mode as part of the runtime contract.
- No benchmark, wiki, diagram, or cloud product side stacks
  - Reject evaluation boards, hosted dashboards, UML generators, and cloud
    deployment surfaces as unrelated runtime expansion.

### Updated package-driven summary

The market signal from these packages is consistent:

- a bounded headless surface benefits from explicit `status`, `update`,
  read-only stats, review-context, and architecture-check vocabulary;
- the attractive parts are mostly read-only, local, and small;
- the recurring overbuild is background sync, external databases, platform
  installers, and broad agent frameworks.

That reinforces the current ADR posture: package skeleton first, then at most a
read-only `harness_status`, and reopen everything heavier only with exact
owner-gap evidence.

## Open Questions

- Which exact current owner should serve as the primary backend for
  `quality_review`: CLI wrapper, app-server path, or direct OntoIndex tooling?
- Should phase 1 expose MCP tools only, or also ship plugin-bundled skills that
  compose those tools into named headless workflows?
- Which local artifact path is correct for trace and JSONL outputs?
- Is a repo-owned stdio helper enough, or is one explicit long-lived HTTP
  runtime actually necessary for any retained capability?

## Challenge Result

Keep the headless workflow ideas. Reject the donor runtime split. Challenge
phase 1 down to package skeleton plus, at most, one read-only status proof.

The correct Ontocode shape is a thin repo-owned plugin package that composes
existing plugin, hook, MCP, config, CLI, and OntoIndex owners into a
headless-first quality and harness workflow surface only after each runtime
surface earns its reopen gate.
