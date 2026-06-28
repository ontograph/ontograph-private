# Code Harness Classic Tool Donor Review

Date: 2026-06-28

## Scope

Review the unpacked Python donor packages under `tmp/code-harness/pypi-src/`
for classic tooling ideas that fit a coding-harness plugin:

- CI gates
- lint or format flows
- static analysis
- architecture validation
- dead-code and complexity checks
- benchmark or reproducibility checks

This note is additive to:

- [ADR_CODE_HARNESS_HEADLESS_PLUGIN.md](ADR_CODE_HARNESS_HEADLESS_PLUGIN.md)
- [ADR_CODE_HARNESS_CHOSEN_TOOLS_AND_ALGORITHMS.md](ADR_CODE_HARNESS_CHOSEN_TOOLS_AND_ALGORITHMS.md)

It does not widen the accepted runtime scope.

## OntoIndex Basis

OntoIndex was used to anchor this review in the current `codex` repo and the
existing headless-harness memory-bank decisions.

The unpacked donor package trees are indexed locally under their own
`.ontoindex/` directories, but current OntoIndex MCP and CLI repo selection in
this session did not reliably resolve most of those donor roots as selectable
repositories. Because of that, exact donor evidence below comes from direct
reads of the unpacked source trees and package metadata, not from donor-targeted
OntoIndex result sets.

That limitation affects retrieval only, not the donor evidence itself.

## Package Triage

### Strongest classic-tool donors

#### `cognitx-codegraph`

Best donor for old-school static-analysis and CI-gate patterns.

Keep-worthy ideas:

- `arch-check` as a first-class command with stable exit codes
- `validate` as a graph sanity check for counts, orphans, and schema
- concrete policy families:
  `import_cycles`, `cross_package`, `layer_bypass`, `coupling_ceiling`,
  `orphan_detection`
- suppression records that require a reason
- stale-suppression detection
- explicit incremental refresh modes:
  `--update` and `--since <git-ref>`
- CI wiring around architecture checks
- read-only-by-default MCP surface
- confidence labels on extracted relationships

Why it matters:

- this is the clearest donor for `quality_arch_check`
- parts of `validate` also fit a future `quality_file_check`
- it is the only donor with a real architecture-policy discipline instead of
  just search or visualization

Reject:

- Neo4j bootstrap
- interactive `init`
- hook install and watch flows
- write-capable MCP tools
- external-agent audit launcher

#### `code-review-graph`

Best donor for review-oriented checks that still feel like classic engineering
tools instead of agent theatre.

Keep-worthy ideas:

- explicit `status` versus `update`
- `detect-changes --brief` versus `update --brief`
- risk-scored changed-scope review
- architecture overview with coupling warnings
- framework-aware dead-code detection
- hotspot or hub detection
- reproducible benchmark discipline with pinned inputs and seeded community
  detection
- `--verify` style cross-checking for reported token-savings metrics
- tool allowlisting for a deliberately small MCP surface

Why it matters:

- strongest donor for `quality_review`
- useful donor for `harness_status` and `harness_update`
- good donor for a future bounded benchmark or verification mode

Reject:

- multi-repo daemon
- background watchers
- hook-managed background automation
- broad prompt catalogs
- refactor apply tool
- wiki and eval side stacks as product surface

#### `codegraphcontext`

Useful donor for classic static-analysis commands, but too heavy as a runtime.

Keep-worthy ideas:

- explicit CLI analysis verbs such as:
  `analyze callers`, `analyze complexity`, `analyze dead-code`
- optional higher-fidelity static indexing via SCIP-like external indexers
- strong dead-code and complexity framing

Why it matters:

- good donor for `quality_file_check`
- confirms that dead-code and complexity checks are expected in this tool
  family

Reject:

- multi-database runtime
- live watch as a default workflow
- backend sprawl
- setup wizard and service-heavy stack

#### `code-graph-rag`

Useful donor for conventional project-quality workflow shape, not for its
runtime.

Keep-worthy ideas:

- one-command developer quality gate:
  `make check = lint + typecheck + test`
- explicit split between unit, integration, and e2e checks
- pre-commit as a local gate, not a hidden runtime
- reference-guided quality or optimization using project standards docs

Why it matters:

- best donor for the shape of a thin CI-friendly check aggregator
- good reminder that harness quality output should compose existing checks
  instead of inventing a second lint universe

Reject:

- repository watch flow
- broad AI optimization runtime
- graph-service coupling

### Secondary donors

#### `codegraph-mcp-server`

Mostly a conventional Python dev-tooling package, not a strong classic analysis
donor by itself.

Useful evidence:

- normal dev stack:
  `pytest`, coverage, `ruff`, `mypy`
- file-structure and module-structure analysis
- self-contained `stats` surface

Why it matters:

- confirms the expected baseline hygiene for a Python tooling package
- minor donor for `quality_explain` and `harness_status`

Not enough by itself:

- no strong first-class CI or static-analysis command family beyond structure
  reading

Reject:

- `execute_shell_command`
- watch mode
- broad prompt surface

#### `codegraph`

Small but honest static-analysis donor.

Keep-worthy ideas:

- pure static parsing without executing target code
- dependency graph from lexical and syntax analysis only
- massive-object detection by lines of code
- simple package hygiene:
  `pytest`, coverage, `flake8`, `pre-commit`, `tox`, `twine`

Why it matters:

- good donor for a minimal `quality_file_check` sub-check:
  large-object or large-file warnings
- good proof that the simple static-analysis baseline is still useful

Reject:

- visualization-first product shape as a harness surface

#### `deepcodegraph`

Mostly dev-tooling evidence, not a harness-tool donor.

Useful evidence:

- standard dev extras:
  `pytest`, coverage, `black`, `isort`, `mypy`, `pylint`

Why it matters:

- confirms conventional quality-stack expectations

Not enough by itself:

- no strong user-facing CI or static-analysis command family to borrow

### Weak or no classic-tool donor value

#### `codegraph_agent`

- broad agent framework
- test dependencies exist, but no clear old-school harness-tool contract stood
  out

#### `python-code-graph`

- evidence of basic project CI only
- no meaningful harness-tool idea beyond simple graph generation

#### `codepropertygraph`

- low-level graph substrate
- not a candidate donor for user-facing classic-tool contracts

## Consolidated Keep-Only Ideas

These are the classic-tool ideas that survive review and still fit the current
accepted harness shape.

### 1. Explicit check commands with stable exit codes

Keep:

- `status`
- `update`
- `arch-check`
- `validate`
- changed-scope review commands

Why:

- old-school tools win by being scriptable, deterministic, and easy to gate in
  CI

Fit to current ADRs:

- `harness_status`
- `harness_update`
- `quality_review`
- `quality_arch_check`

### 2. Architecture-policy checks

Keep:

- import cycles
- cross-package boundaries
- layer-bypass checks
- coupling ceilings
- orphan detection
- suppression records with required reason
- stale-suppression detection

Why:

- this is the strongest concrete static-analysis family across the donors

Fit to current ADRs:

- `quality_arch_check`
- some orphan or large-object signals can also feed `quality_file_check`

### 3. File-local static checks

Keep:

- dead-code suspicion
- cyclomatic complexity or complexity-threshold warnings
- large-object or large-file warnings
- obvious missing-test adjacency checks

Why:

- these are cheap, conventional, and understandable
- they do not require a second runtime owner if implemented as advisory checks

Fit to current ADRs:

- `quality_file_check`

### 4. CI-friendly check aggregation

Keep the shape, not the donor implementation:

- one explicit aggregate command that combines existing checks
- split outputs by category
- non-zero exit only in strict mode
- machine-readable JSON plus compact human summary

Why:

- `make check` style composition is better than inventing a new lint framework

Fit to current ADRs:

- preferably a mode on `quality_review` or `quality_arch_check`
- not a new top-level plugin subsystem

### 5. Reproducibility and benchmark discipline

Keep:

- pinned benchmark inputs
- seeded deterministic analysis where randomness exists
- explicit verification command or mode for claimed metrics

Why:

- useful for proving value of graph-assisted review without hand-wavy claims

Fit to current ADRs:

- optional later metadata or validation mode
- not an early product surface

### 6. Read-only first, explicit mutation second

Keep:

- read-only status and review surfaces by default
- explicit update only when the user asks for it

Why:

- matches classic CLI expectations
- reduces hidden state changes and surprise automation

Fit to current ADRs:

- `harness_status`
- `harness_update`

## What Is Only Dev Hygiene, Not a Product Feature

A lot of donor evidence is useful but should not be mistaken for a new harness
tool:

- `ruff`
- `mypy`
- `pylint`
- `flake8`
- `black`
- `isort`
- `pytest`
- coverage
- `tox`
- `pre-commit`

Conclusion:

- these are evidence of normal project discipline
- they do not justify adding a new plugin-owned lint engine
- if Ontocode later exposes a CI helper, it should compose existing repo checks
  instead of re-implementing them

## Rejections

Do not carry these donor shapes forward:

- background daemons
- live file watchers as default behavior
- git-hook-managed hidden updates
- external DB runtimes
- platform installers and rules-file writers
- write-capable MCP tools
- broad agent-driven audit launchers
- benchmark, visualization, wiki, or export stacks as first-class harness
  features
- a second lint or test framework owned by the plugin

## Recommended Mapping To Current Harness Plan

Use the already accepted tool families. Do not create new ones just to mirror
donor command names.

- `harness_status`
  - readiness, counts, freshness, degraded prerequisites
- `harness_update`
  - explicit refresh with `full` / `incremental` / `since-ref`
- `quality_review`
  - changed-scope review, impact expansion, strict CI mode later
- `quality_file_check`
  - complexity, dead-code suspicion, large-object warnings, local test hints
- `quality_arch_check`
  - cycles, package boundaries, coupling, orphan checks, suppression handling

Skipped:

- separate plugin-owned lint runner
- separate plugin-owned type checker
- separate benchmark product surface

Add when:

- the current owners cannot express those checks by composition

## Bottom Line

The strongest old-fashioned donor family is `cognitx-codegraph`, because it has
real architecture policies, validation, suppressions, and CI gating.

`code-review-graph` is the strongest review-era donor, because it turns graph
analysis into deterministic review checks with clear `status` versus `update`
semantics and reproducible metrics.

`codegraphcontext` adds the best cheap file-level static-analysis ideas:
callers, complexity, dead code.

Everything else is either:

- supporting evidence for standard dev hygiene, or
- runtime sprawl that should stay rejected.
