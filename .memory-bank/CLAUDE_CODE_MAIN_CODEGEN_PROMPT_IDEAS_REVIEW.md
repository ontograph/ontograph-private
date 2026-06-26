# Claude Code Main Codegen Prompt Ideas Review

Date: 2026-06-26

Scope: review markdown prompt assets in [tmp/claude-code-main](../tmp/claude-code-main) and keep only ideas that improve code-generation workflows in Ontocode without adding a parallel command, plugin, agent, hook, or prompt runtime.

Implementation status: CCG-P2 is implemented as process guidance in this file. CCG-P1, CCG-P3, and CCG-P4 remain parked.

## Donor Markdown Prompt Corpus

Found useful prompt-bearing markdown in these areas:

- [prompts/00-overview.md](../tmp/claude-code-main/prompts/00-overview.md:1) through [prompts/16-testing.md](../tmp/claude-code-main/prompts/16-testing.md:1): dependency-ordered build-out prompts.
- [plugins/feature-dev/commands/feature-dev.md](../tmp/claude-code-main/plugins/feature-dev/commands/feature-dev.md:1): phased feature-development command prompt.
- [plugins/feature-dev/agents](../tmp/claude-code-main/plugins/feature-dev/agents/code-explorer.md:1): explorer, architect, and reviewer agent prompts.
- [plugins/pr-review-toolkit](../tmp/claude-code-main/plugins/pr-review-toolkit/commands/review-pr.md:1): aspect-specific PR review command and agents.
- [plugins/plugin-dev/skills](../tmp/claude-code-main/plugins/plugin-dev/skills/skill-development/SKILL.md:1): skill, command, agent, hook, MCP, and plugin prompt-design references.
- [plugins/hookify](../tmp/claude-code-main/plugins/hookify/commands/hookify.md:1): rule/hook generation prompts. Useful as guardrail inspiration only.

Prior donor tool inventory already found that polished descriptions required reading prompt files such as `src/tools/*/prompt.ts`, not only main implementation files. This review applies the same rule: treat prompt markdown and prompt references as first-class donor evidence.

## Current Ontocode Owner Gate

OntoIndex evidence for repo `codex` points to these existing owners:

- `Session.build_initial_context` already assembles model instructions, developer sections, contextual user sections, skill metadata, plugins, prompt fragments, and bounded context fragments.
- `SkillsManager.skills_for_config` and core-skills rendering already own skill discovery and prompt-visible skill metadata.
- Existing review, guardian, `gn_verify_diff`, and `gn_test_gap` surfaces already own change review, diff verification, and test-gap evidence.
- Multi-agent/sub-agent tooling already exists; donor agent prompts are useful workflow contracts, not justification for a second agent runtime.

Fresh OntoIndex challenge on 2026-06-26:

- The index is fresh at commit `2e72a6d25e147f0619863e7721107b6f11a87fc2`, but the worktree is dirty. Treat this file as proposal/review evidence, not implementation authority.
- `Session.build_initial_context` has broad caller coverage from compaction, resume, rollback, realtime context, prompt-fragment, skill-budget, and multi-agent tests. Any new prompt-assembly change is high-touch and must be reopened only for a concrete failing prompt/context case.
- `build_initial_context_includes_prompt_fragments_from_extensions` and `build_initial_context_preserves_prompt_section_boundaries` already cover the MiMo-style prompt-section boundary slice.
- `.codex/skills/simplify/SKILL.md` and `loads_repo_simplify_skill_without_loader_schema_extension` already implement the Qwen simplify/codegen-cleanup slice without loader schema changes.
- Review-aspect work already has existing homes: prompt review templates in `ontocode-rs/prompts`, guardian review session/prompt tests, `gn_test_gap`, `gn_verify_diff`, and code-review skills. Do not add another review checklist runtime or duplicate skill unless a missing behavior is proven.

Gate: keep prompt structure, checklists, and workflow contracts. Reject or defer donor runtime features such as markdown command execution, state-carrying command files, plugin marketplaces, new hook engines, or automatic agent definitions unless a separate ADR proves an existing-owner gap.

## Best Code-Generation Ideas

| # | Donor source | Useful idea | Decision | Ontocode fit |
| --- | --- | --- | --- | --- |
| 1 | [00-overview.md](../tmp/claude-code-main/prompts/00-overview.md:1) | Dependency-ordered prompt pack with explicit prerequisites. | KEEP | Use as ADR/project-plan shape: each slice names dependencies and verification. |
| 2 | [00-overview.md](../tmp/claude-code-main/prompts/00-overview.md:33) | Mark independent prompts that can run in parallel while stopping on failed prerequisites. | NARROW | Apply to manager-loop dispatch ledgers, not a new scheduler. |
| 3 | [00-overview.md](../tmp/claude-code-main/prompts/00-overview.md:35) | Every prompt must be independently verifiable. | KEEP | Require each codegen slice to name focused `just test -p ...`, snapshot, or diff-verification evidence. |
| 4 | [07-tool-system.md](../tmp/claude-code-main/prompts/07-tool-system.md:3) | Start codegen prompts with `Context`, `Key files`, `Task`, and `Verification`. | KEEP | Good template for future ADR implementation prompts and worker dispatch prompts. |
| 5 | [07-tool-system.md](../tmp/claude-code-main/prompts/07-tool-system.md:14) | Split large feature work into Parts A-F: understand interface, audit registry, verify compile, fix blockers, smoke test, exclude unsupported internals. | KEEP | Translate into Rust owner-local steps for tools, providers, MCP, and skills. |
| 6 | [07-tool-system.md](../tmp/claude-code-main/prompts/07-tool-system.md:43) | Identify a small "core set" before exhaustive coverage. | NARROW | Use for first-slice smoke coverage, but do not leave non-core regressions untracked. |
| 7 | [07-tool-system.md](../tmp/claude-code-main/prompts/07-tool-system.md:95) | Unsupported/internal feature gates should be cleanly excluded, not half-loaded. | KEEP | Applies to plugin/skill/provider capability gates and disabled tools. |
| 8 | [09-query-engine.md](../tmp/claude-code-main/prompts/09-query-engine.md:29) | Map core LLM loop architecture before editing: public API, message flow, tool loop, streaming, retries. | KEEP | Existing-owner preflight for session/model-client/tool-loop work. |
| 9 | [09-query-engine.md](../tmp/claude-code-main/prompts/09-query-engine.md:47) | Distinguish essential blockers from optional services that can be stubbed or skipped. | KEEP | Useful for bounded test harness design and donor-port triage. |
| 10 | [09-query-engine.md](../tmp/claude-code-main/prompts/09-query-engine.md:60) | Minimal conversation smoke test for core loop. | NARROW | Use existing `core/suite`/responses helpers instead of new ad hoc scripts. |
| 11 | [09-query-engine.md](../tmp/claude-code-main/prompts/09-query-engine.md:106) | Document what works, what is stubbed, and what remains broken after a slice. | KEEP | Fits memory-bank closure notes and tracking ledgers. |
| 12 | [10-context-and-prompts.md](../tmp/claude-code-main/prompts/10-context-and-prompts.md:23) | Trace system prompt construction by sections, tools, model variations, and dynamic context. | KEEP | Already aligned with MiMo slice; use for future prompt-fragment tests only. |
| 13 | [10-context-and-prompts.md](../tmp/claude-code-main/prompts/10-context-and-prompts.md:52) | Add a prompt inspection path that catches `undefined` or unresolved macro output. | NARROW | Use focused Rust tests around `build_initial_context`, not a new public prompt-dump command. |
| 14 | [10-context-and-prompts.md](../tmp/claude-code-main/prompts/10-context-and-prompts.md:86) | Audit context modules for OS, shell, git, project, environment, and CI data. | DEFER | Useful only after a failing context owner case; avoid broad prompt bloat. |
| 15 | [16-testing.md](../tmp/claude-code-main/prompts/16-testing.md:1) | Treat testing as a final prompt slice with unit, smoke, integration, and build checks. | KEEP | Map to `just test -p`, snapshot acceptance, schema generation, and diff verification. |
| 16 | [feature-dev.md](../tmp/claude-code-main/plugins/feature-dev/commands/feature-dev.md:10) | Discovery before design; read code patterns before acting. | KEEP | Matches OntoIndex-first and memory-bank-first workflow. |
| 17 | [feature-dev.md](../tmp/claude-code-main/plugins/feature-dev/commands/feature-dev.md:12) | Ask clarifying questions before architecture/implementation when scope is underspecified. | KEEP | Keep as prompt rule for feature work; do not block simple bugfixes. |
| 18 | [feature-dev.md](../tmp/claude-code-main/plugins/feature-dev/commands/feature-dev.md:14) | If agents return key files, manager must read those files before designing. | KEEP | Important anti-hallucination rule for sub-agent manager loops. |
| 19 | [feature-dev.md](../tmp/claude-code-main/plugins/feature-dev/commands/feature-dev.md:41) | Use 2-3 exploration agents with distinct focus areas. | NARROW | Use only for large/ambiguous work; default to OntoIndex plus direct reads for small tasks. |
| 20 | [feature-dev.md](../tmp/claude-code-main/plugins/feature-dev/commands/feature-dev.md:73) | Architecture phase compares minimal, clean, and pragmatic approaches. | KEEP | Good ADR challenge format when paired with existing-owner constraints. |
| 21 | [feature-dev.md](../tmp/claude-code-main/plugins/feature-dev/commands/feature-dev.md:89) | Do not start implementation without approval after presenting architecture options. | NARROW | Applies to plan/ADR mode, not to direct "fix issues" requests. |
| 22 | [code-explorer.md](../tmp/claude-code-main/plugins/feature-dev/agents/code-explorer.md:16) | Explorer output must include entry points, call flow, data transformations, dependencies, and key files. | KEEP | Strong prompt contract for senior reviewer / exploration sub-agents. |
| 23 | [code-explorer.md](../tmp/claude-code-main/plugins/feature-dev/agents/code-explorer.md:43) | Require file:line references in exploration output. | KEEP | Matches OntoIndex/challenge evidence standard. |
| 24 | [code-architect.md](../tmp/claude-code-main/plugins/feature-dev/agents/code-architect.md:22) | Architecture blueprint should include patterns found, file map, data flow, build sequence, and critical details. | KEEP | Good ADR implementation-plan template. |
| 25 | [code-architect.md](../tmp/claude-code-main/plugins/feature-dev/agents/code-architect.md:16) | "Make decisive choices" instead of endless options. | NARROW | Manager may recommend one option, but user approval still governs broad implementation. |
| 26 | [code-reviewer.md](../tmp/claude-code-main/plugins/feature-dev/agents/code-reviewer.md:23) | Confidence-scored code review; report only issues above a high confidence threshold. | KEEP | Useful for review prompts; reduces false-positive churn. |
| 27 | [review-pr.md](../tmp/claude-code-main/plugins/pr-review-toolkit/commands/review-pr.md:20) | Aspect-specific reviews: comments, tests, errors, types, code, simplify. | KEEP | Already maps to existing code-review skills and test-gap/verify-diff tooling. |
| 28 | [review-pr.md](../tmp/claude-code-main/plugins/pr-review-toolkit/commands/review-pr.md:45) | Sequential review by default, parallel only on request. | KEEP | Good manager-loop default in dirty worktrees. |
| 29 | [pr-test-analyzer.md](../tmp/claude-code-main/plugins/pr-review-toolkit/agents/pr-test-analyzer.md:10) | Test review focuses on behavioral coverage and critical gaps, not coverage percentage. | KEEP | Matches repo test guidance; useful for codegen verification prompt. |
| 30 | [pr-test-analyzer.md](../tmp/claude-code-main/plugins/pr-review-toolkit/agents/pr-test-analyzer.md:27) | Rate test recommendations by criticality and explain regressions they prevent. | KEEP | Good addition to manager verification artifacts. |
| 31 | [silent-failure-hunter.md](../tmp/claude-code-main/plugins/pr-review-toolkit/agents/silent-failure-hunter.md:24) | Dedicated silent-failure audit over catch blocks, fallbacks, hidden defaults, and user feedback. | KEEP | Useful as review checklist for provider/MCP/session/tool error-handling changes. |
| 32 | [silent-failure-hunter.md](../tmp/claude-code-main/plugins/pr-review-toolkit/agents/silent-failure-hunter.md:99) | Error findings must include location, severity, hidden error, user impact, and concrete fix. | KEEP | Good strict output contract for reviewer workers. |
| 33 | [type-design-analyzer.md](../tmp/claude-code-main/plugins/pr-review-toolkit/agents/type-design-analyzer.md:13) | Type design review should identify invariants and rate encapsulation, expression, usefulness, and enforcement. | KEEP | Use for Rust API/type changes; do not make it mandatory for every diff. |
| 34 | [system-prompt-design.md](../tmp/claude-code-main/plugins/plugin-dev/skills/agent-development/references/system-prompt-design.md:5) | Standard prompt skeleton: responsibilities, process, quality standards, output format, edge cases. | KEEP | Best reusable pattern for codegen skill/worker prompts. |
| 35 | [system-prompt-design.md](../tmp/claude-code-main/plugins/plugin-dev/skills/agent-development/references/system-prompt-design.md:91) | Separate analysis, generation, validation, and orchestration prompt patterns. | KEEP | Useful for role-specific sub-agent prompts without new runtime. |
| 36 | [advanced-workflows.md](../tmp/claude-code-main/plugins/plugin-dev/skills/command-development/references/advanced-workflows.md:11) | Sequential workflow prompts with numbered steps and decision points. | NARROW | Keep as markdown planning pattern only; do not import command macro execution. |
| 37 | [advanced-workflows.md](../tmp/claude-code-main/plugins/plugin-dev/skills/command-development/references/advanced-workflows.md:64) | State-carrying workflow files enable resume. | DEFER | Ontocode already has rollout/state/memory owners; no new `.claude/*state*` files without ADR. |
| 38 | [testing-strategies.md](../tmp/claude-code-main/plugins/plugin-dev/skills/command-development/references/testing-strategies.md:9) | Validate prompt assets at multiple levels: syntax, metadata fields, invocation, argument matrix. | KEEP | Apply to skills/prompt assets if new ones are added. |
| 39 | [skill-development/SKILL.md](../tmp/claude-code-main/plugins/plugin-dev/skills/skill-development/SKILL.md:77) | Progressive disclosure: metadata always visible, body on trigger, references/scripts/assets on demand. | KEEP | Already compatible with Ontocode skills; keep future skill bodies lean. |
| 40 | [skill-development/SKILL.md](../tmp/claude-code-main/plugins/plugin-dev/skills/skill-development/SKILL.md:190) | Keep `SKILL.md` lean and move detailed patterns to references. | KEEP | Directly useful for any future codegen skill references. |

## Rejected Or Parked Donor Runtime Ideas

| Donor idea | Decision | Reason |
| --- | --- | --- |
| Import the donor markdown command runtime, argument substitution, and command frontmatter. | REJECT for this review | Ontocode already has skills, prompts, slash commands, and plugin owners. Runtime import would create a second owner. |
| Add donor state-carrying command files such as `.claude/deployment-state.local.md`. | DEFER | Resume/state belongs to existing rollout, session, and memory-bank owners. Needs separate ADR and security model. |
| Add a new agent markdown/frontmatter runtime from donor agents. | REJECT | Sub-agent roles and model selection already exist. Use donor text as prompt contract only. |
| Add a hookify-style markdown rule engine. | DEFER | Existing hooks/guardian/tool permission owners must be extended if a real gap appears. |
| Port donor Bun/TypeScript build, shims, or Anthropic-internal gates. | REJECT | Donor-specific implementation detail, not code-generation prompt value for Rust Ontocode. |
| Add prompt inspection as a user-facing command. | DEFER | Current value is test coverage for prompt assembly; public command/API needs separate product proof. |
| Auto-generate skills/commands from repeated behavior. | DEFER | Needs permissions, overwrite protection, prompt-injection safety, and user control. |

## Proposed Implementation Slices

These are proposals only. After challenge, none are directly implementation-ready. Promote a slice only when it names a concrete current-owner gap that is not already covered by MiMo prompt-section tests, the Qwen simplify skill, existing review prompts, guardian review, `gn_test_gap`, or `gn_verify_diff`.

### Slice CCG-P1: Codegen Prompt Contract Reference

Goal: add a small prompt-reference document or skill reference that standardizes codegen worker prompts.

Challenge: PARK. The repo already has skill loading, prompt fragments, and a simplify skill. A generic "codegen prompt contract" would mostly duplicate existing AGENTS.md, skills, and ADR guidance unless it is attached to one concrete workflow that currently lacks instructions.

Steps:

1. Reuse existing skill/reference loading; do not change loader schema.
2. Add a compact reference with the donor-derived skeleton: Context, Key files, Existing owner, Task parts, Verification, Stop conditions.
3. Include role variants for exploration, architecture, generation, validation, and review.
4. Require file:line evidence for exploration/review output.
5. Test only discovery/rendering if this becomes a real skill asset.

Acceptance:

- No new command runtime, agent runtime, or public API.
- Prompt text points workers to OntoIndex and existing owner checks before edits.
- Verification commands remain project-specific and bounded.
- Reopen evidence names the exact workflow whose current prompts are insufficient.

### Slice CCG-P2: Manager-Loop Evidence Contract

Goal: improve manager loops so sub-agent outputs are not blindly trusted.

Challenge: IMPLEMENTED AS PROCESS ONLY. This is useful for manager-loop discipline, but it does not justify Rust code, schema, state, or runtime changes.

Steps:

1. Require explorer/reviewer workers to return 5-10 key files or symbols with line references.
2. Require manager to read the returned files before architecture or implementation.
3. Add tracking status fields: evidence read, owner confirmed, tests proposed, tests run, residual risk.
4. Keep dispatch sequential by default; allow parallel only when bundles are independent.
5. Close with works/stubbed/broken summary.

Acceptance:

- No scheduler or persistent job queue changes.
- Tracking remains in existing `.memory-bank` or ADR tracking files.
- Dirty worktree caveats are explicit.
- If no tracking file is being updated for a concrete loop, no dispatch is needed.

### Slice CCG-P3: Review Aspect Checklist

Goal: improve codegen verification prompts by adding aspect-specific checklists.

Challenge: PARK. The useful checks already map to existing review/test owners. Reopen only for one missing checklist in one existing surface, not a new broad review asset.

Steps:

1. Reuse existing code-review/test-gap/verify-diff workflows.
2. Add checklist text for behavioral tests, silent failures, type invariants, and simplification.
3. Apply only to relevant diffs: type review for new/changed types, silent-failure review for error/fallback changes.
4. Use confidence threshold and criticality ratings to avoid noise.
5. Keep simplification scoped to recently changed code only.

Acceptance:

- Review findings include file:line, severity, reason, and concrete fix.
- Test-gap recommendations name the regression they prevent.
- No broad automated rewrite/simplification pass.
- Reopen evidence shows the existing review, guardian, simplify, test-gap, or verify-diff path failed to express the needed check.

### Slice CCG-P4: Prompt Asset Validation

Goal: if Ontocode adds more repo-local prompt assets, test them like code.

Challenge: CONDITIONAL ONLY. This becomes real only after a new prompt asset is accepted elsewhere. Do not create a standalone prompt-asset validation framework first.

Steps:

1. Validate markdown/frontmatter syntax for new prompt assets.
2. Assert required metadata fields through existing core-skills tests where applicable.
3. Add argument/invocation matrix tests only for assets with arguments.
4. Add prompt rendering tests for user-visible context changes.
5. Document unsupported metadata as donor evidence only.

Acceptance:

- Prompt assets fail tests when metadata is malformed or output is unbounded.
- No unsupported donor frontmatter fields become accepted schema by accident.
- Validation lives in the existing owner test harness for the accepted prompt asset.

## Recommended Next Step

No immediate implementation ADR. Apply the CCG-P2 contract below when running future donor/ADR loops. Reopen CCG-P1, CCG-P3, or CCG-P4 only with a named existing-owner failure and the smallest owner-local test that would prove the fix.

## Implemented CCG-P2 Manager-Loop Contract

Use this contract when a future donor/ADR manager loop dispatches reviewer, explorer, implementation, or verification workers.

### Manager Preflight

1. Read `.memory-bank/MEMORY.md` and the target ADR/review/tracking file.
2. Run or check OntoIndex freshness for repo `codex`.
3. Record dirty-worktree scope confidence before dispatch.
4. Identify the existing owner before asking a worker to design or edit.
5. If no owner-local gap is named, close as no-dispatch.

### Worker Prompt Requirements

Every explorer or reviewer worker prompt must request:

- 5-10 key files or symbols with line references.
- Existing owner and caller/callee or flow evidence when relevant.
- Proposed focused tests or explicit reason no test is needed.
- Works/stubbed/broken or keep/park/reject outcome.
- Residual risk and dirty-worktree caveats.

### Manager Acceptance Gate

Before accepting worker output, the manager must:

1. Read the key files or symbols returned by the worker.
2. Verify the proposed owner is still current.
3. Reject proposals that create a parallel owner, runtime, scheduler, registry, prompt engine, or state store.
4. Require a focused validation command for any implementation slice.
5. Update the tracking file before and after dispatch.

### Tracking Row Template

Use this compact row shape in existing tracking files when a manager loop needs structured evidence:

| Task | Status | Owner | Evidence read | Existing owner confirmed | Tests proposed | Tests run | Residual risk |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `<id>` | `pending/in_progress/review/done/blocked/no-dispatch` | `<manager/worker>` | `<files or symbols read>` | `<owner path or symbol>` | `<focused checks>` | `<actual commands or n/a>` | `<dirty tree, blocked proof, or none>` |

### Closure Template

Use this short closeout shape for docs-only or no-dispatch loops:

```text
Outcome: done / blocked / no-dispatch
OntoIndex: fresh/stale at <commit>; dirty worktree <yes/no>
Evidence read: <files/symbols>
Decision: <what changed or why nothing changed>
Verification: <commands run, diff check, or docs-only>
Residual risk: <remaining blocker or none>
```

### Non-Scope

- Do not add a scheduler, persistent queue, SQLite state, command runtime, or prompt asset runtime for this contract.
- Do not require sub-agents for small single-owner tasks where direct OntoIndex plus source reads are enough.
- Do not promote CCG-P1, CCG-P3, or CCG-P4 without a named existing-owner failure.
