# Gemini CLI Codegen Prompt Ideas Review

Date: 2026-06-26

Scope: review markdown prompt assets in [tmp/gemini-cli-main](../tmp/gemini-cli-main) and keep only ideas that improve Ontocode code-generation workflows without adding a parallel prompt runtime, skill runtime, agent runtime, scheduler, hook engine, or eval platform.

Status: review complete. This is proposal evidence only; no implementation slice is approved by this file alone.

## Donor Markdown Prompt Corpus

Prompt-bearing markdown found in the donor repo:

- [evals/README.md](../tmp/gemini-cli-main/evals/README.md:1): behavioral-eval philosophy, fail-first prompt/tool steering tests, flake policy, promotion, and regression checks.
- [.gemini/skills/behavioral-evals/SKILL.md](../tmp/gemini-cli-main/.gemini/skills/behavioral-evals/SKILL.md:1) and [references](../tmp/gemini-cli-main/.gemini/skills/behavioral-evals/references/creating.md:1): procedural eval creation, fixing, running, and promoting guidance.
- [docs/cli/system-prompt.md](../tmp/gemini-cli-main/docs/cli/system-prompt.md:1): custom system prompt override, prompt export, variable placeholders, and `system.md` versus `GEMINI.md` separation.
- [docs/cli/plan-mode.md](../tmp/gemini-cli-main/docs/cli/plan-mode.md:1): read-only planning, explicit approval, plan files, plan-mode tool restrictions, and skills during planning.
- [docs/cli/skills-best-practices.md](../tmp/gemini-cli-main/docs/cli/skills-best-practices.md:1), [docs/cli/creating-skills.md](../tmp/gemini-cli-main/docs/cli/creating-skills.md:1), and [docs/cli/using-agent-skills.md](../tmp/gemini-cli-main/docs/cli/using-agent-skills.md:1): skill discovery, progressive disclosure, focused resources, deterministic scripts, consent, and discovery precedence.
- [docs/cli/gemini-md.md](../tmp/gemini-cli-main/docs/cli/gemini-md.md:1): hierarchical project context, just-in-time context files, context reload, and imports.
- [docs/cli/token-caching.md](../tmp/gemini-cli-main/docs/cli/token-caching.md:1): stable context reuse and cache visibility.
- [docs/core/subagents.md](../tmp/gemini-cli-main/docs/core/subagents.md:1): focused subagents with isolated context windows and toolsets.
- [.gemini/commands/strict-development-rules.md](../tmp/gemini-cli-main/.gemini/commands/strict-development-rules.md:1): strict development/test rules and warning that prompt changes affect product quality.
- [tools/gemini-cli-bot/brain/interactive.md](../tmp/gemini-cli-main/tools/gemini-cli-bot/brain/interactive.md:1) and [scheduled.md](../tmp/gemini-cli-main/tools/gemini-cli-bot/brain/scheduled.md:1): one-thing-at-a-time automation, untrusted-context delimiters, and mandatory evidence gathering.
- Specialized review skills such as [code-reviewer](../tmp/gemini-cli-main/.gemini/skills/code-reviewer/SKILL.md:1), [async-pr-review](../tmp/gemini-cli-main/.gemini/skills/async-pr-review/SKILL.md:1), [review-duplication](../tmp/gemini-cli-main/.gemini/skills/review-duplication/SKILL.md:1), [string-reviewer](../tmp/gemini-cli-main/.gemini/skills/string-reviewer/SKILL.md:1), and [tui-tester](../tmp/gemini-cli-main/.gemini/skills/tui-tester/SKILL.md:1).

## OntoIndex Challenge Evidence

OntoIndex repo `codex` is fresh at commit `2e72a6d25e147f0619863e7721107b6f11a87fc2`, but the worktree is dirty with 293 files, so this review should not be treated as implementation authority without a fresh focused diff check.

Existing owner map:

- Prompt and context assembly already belongs to `Session.build_initial_context`, `core/context`, contextual user fragments, prompt fragments, and compaction context reinsertion.
- Skill discovery and rendering already belong to `core-skills` and repo/user skill loading. Donor skill guidance is useful as skill text discipline, not a reason to add a second loader schema.
- Planning behavior already has TUI collaboration-mode and plan-mode tests, including mode-switch and plan-nudge surfaces.
- External issue/PR context already has review/PR skills and GitHub helper scripts; any untrusted-context rule should extend those prompts or helpers, not create a new bot runtime.
- Prompt/tool steering reliability must use existing Rust integration tests, prompt-rendering tests, snapshot tests, `gn_verify_diff`, and `gn_test_gap` before any new eval platform is considered.

Challenge result: Gemini's best codegen value is process and test discipline around model-steering changes. Most runtime ideas are already covered by MiMo, Qwen, and Claude prompt reviews or belong to non-core product surfaces.

## Senior Challenge

This review should not open standalone work. The useful Gemini donor material is a set of gates to apply when a real prompt/tool/context change is already underway.

Challenge findings:

- `GCG-P1` is not an implementation slice. OntoIndex resolves prompt/context request-shape coverage to existing tests such as `build_initial_context_includes_prompt_fragments_from_extensions`, `build_initial_context_preserves_prompt_section_boundaries`, and `core/tests/suite/*` helpers using `ResponseMock::single_request`. Add a test only when a concrete prompt/tool behavior is being changed.
- `GCG-P2` is too broad if read as "add untrusted-context wrapping everywhere." OntoIndex points current external-context evidence to GitHub skill scripts such as `.codex/skills/babysit-pr/scripts/gh_pr_watch.py`, issue digest scripts, rollout/memory filtering, and `redact_secrets`. Reopen only for one named ingestion path that actually injects external text into model-visible context.
- `GCG-P3` should stay a checklist. The Prompt Cache Boundary ADR already owns stable/dynamic prompt separation; this file must not create a second prompt-governance layer.
- Rows marked `KEEP` mean "keep the principle for future reviews," not "dispatch now." Any future implementation still needs owner impact, a failing or missing test, and a focused validation command.
- The dirty worktree keeps OntoIndex scope confidence at medium. Before any code change, rerun `gn_ensure_fresh`, run impact on the exact symbol, and verify the current diff.

Challenge result: no open implementation tasks. The next valid action is only to apply these gates to the next concrete prompt/tool/context change.

## Best Code-Generation Ideas

| # | Donor source | Useful idea | Decision | Ontocode fit |
| --- | --- | --- | --- | --- |
| 1 | [evals/README.md](../tmp/gemini-cli-main/evals/README.md:3) | Treat model behavior as testable behavior, not just implementation correctness. | KEEP | For prompt/tool changes, add focused tests that assert the model-facing request shape, tool decision path, or transcript behavior. |
| 2 | [evals/README.md](../tmp/gemini-cli-main/evals/README.md:67) | Fail first before changing prompts or tool descriptions. | KEEP | Reopen prompt edits only with a reproduced failing fixture, snapshot, or transcript test. Do not accept "prompt polish" without before/after evidence. |
| 3 | [evals/README.md](../tmp/gemini-cli-main/evals/README.md:43) | Behavioral tests should use small realistic workspaces, not toy prompts. | KEEP | Good standard for core/suite and TUI integration tests: 2-3 realistic files where codegen behavior depends on local context. |
| 4 | [evals/README.md](../tmp/gemini-cli-main/evals/README.md:61) | Assertions must target concrete tool calls, modified files, AST/string facts, or request fields. | KEEP | Avoid brittle "expected prose" assertions; prefer structured `ResponsesRequest`, tool-call, snapshot, and file-content assertions. |
| 5 | [evals/README.md](../tmp/gemini-cli-main/evals/README.md:76) | Fewer realistic behavior tests beat many unit-like prompt tests. | NARROW | Use only for model-steering paths; normal Rust logic still needs ordinary unit/integration tests. |
| 6 | [evals/README.md](../tmp/gemini-cli-main/evals/README.md:105) | New non-deterministic evals start as non-blocking and graduate only after stability evidence. | DEFER | Useful policy if Ontocode later adds model behavioral eval infrastructure. For now, keep local deterministic tests blocking and do not add a nightly eval platform. |
| 7 | [evals/README.md](../tmp/gemini-cli-main/evals/README.md:247) | Compare regression against a dynamic baseline when model behavior is noisy. | DEFER | Too much infrastructure for current scope. Reopen only if model-behavior tests become flaky across providers. |
| 8 | [evals/README.md](../tmp/gemini-cli-main/evals/README.md:280) | Fix prompt/tool regressions with minimal targeted changes. | KEEP | Matches current owner-local rule: edit the smallest prompt/tool surface that owns the failed behavior. |
| 9 | [evals/README.md](../tmp/gemini-cli-main/evals/README.md:299) | Prefer positive prompt traits over negative prohibitions when possible. | KEEP | Useful for future stable system-prompt changes and skill text review; avoid adding long "do not" lists unless failure evidence requires them. |
| 10 | [docs/cli/system-prompt.md](../tmp/gemini-cli-main/docs/cli/system-prompt.md:105) | Separate stable system rules from project strategy/context. | KEEP | Reinforces Prompt Cache Boundary ADR and current AGENTS/memory-bank split. Stable prompt changes need stricter tests than project context changes. |
| 11 | [docs/cli/system-prompt.md](../tmp/gemini-cli-main/docs/cli/system-prompt.md:93) | Export/review the default prompt before overriding. | NARROW | Useful as internal review practice for prompt diffs; do not expose a new user-facing system-prompt override without separate product/security ADR. |
| 12 | [docs/cli/system-prompt.md](../tmp/gemini-cli-main/docs/cli/system-prompt.md:61) | Variable placeholders for tools/skills/subagents make custom prompts explicit. | REJECT for now | Ontocode already assembles tools, skills, plugins, apps, and prompt fragments. Macro substitution would add a new prompt language. |
| 13 | [docs/cli/plan-mode.md](../tmp/gemini-cli-main/docs/cli/plan-mode.md:3) | Planning should be read-only until strategy is agreed. | KEEP | Map to existing collaboration-mode discipline and review/ADR tasks; no new plan-mode runtime needed. |
| 14 | [docs/cli/plan-mode.md](../tmp/gemini-cli-main/docs/cli/plan-mode.md:57) | Plan workflow should research, discuss strategy, then approve/iterate/cancel. | NARROW | Applies to broad/ambiguous design work, not direct "fix issues" tasks where user already requested implementation. |
| 15 | [docs/cli/plan-mode.md](../tmp/gemini-cli-main/docs/cli/plan-mode.md:114) | Read-only mode should have explicit tool restrictions. | DEFER | Existing sandbox/mode policy owns this. Reopen only with a concrete plan-mode write leak. |
| 16 | [docs/cli/skills-best-practices.md](../tmp/gemini-cli-main/docs/cli/skills-best-practices.md:6) | Skill descriptions are the discovery contract. | KEEP | Future skills must use specific trigger keywords and avoid overlap; this is already compatible with existing skill loading. |
| 17 | [docs/cli/skills-best-practices.md](../tmp/gemini-cli-main/docs/cli/skills-best-practices.md:19) | Progressive disclosure keeps metadata always visible, body on trigger, references on demand. | KEEP | Already aligned with Qwen/Claude skill decisions; use it to reject giant always-loaded codegen prompts. |
| 18 | [docs/cli/skills-best-practices.md](../tmp/gemini-cli-main/docs/cli/skills-best-practices.md:32) | Match instruction specificity to task fragility. | KEEP | Fragile migrations may use scripts/templates; flexible reviews should stay text-based. |
| 19 | [docs/cli/skills-best-practices.md](../tmp/gemini-cli-main/docs/cli/skills-best-practices.md:43) | Bundle deterministic scripts for repeatable skill steps. | NARROW | Use only when a script replaces repeated manual error-prone work; do not add script folders to every skill. |
| 20 | [docs/cli/creating-skills.md](../tmp/gemini-cli-main/docs/cli/creating-skills.md:159) | Skill discovery tiers and workspace precedence are explicit. | NARROW | Ontocode already has its own skill/plugin precedence. Keep as comparison evidence, not schema import. |
| 21 | [docs/cli/using-agent-skills.md](../tmp/gemini-cli-main/docs/cli/using-agent-skills.md:71) | Remote/untrusted skills require consent and activation permission. | KEEP | Useful safety rule for plugin/skill install docs and future skill-install surfaces. |
| 22 | [docs/cli/gemini-md.md](../tmp/gemini-cli-main/docs/cli/gemini-md.md:11) | Hierarchical project context reduces repeated prompt boilerplate. | COVERED | Ontocode already uses AGENTS.md, memory-bank, skills, and prompt fragments. Do not add `GEMINI.md` hierarchy. |
| 23 | [docs/cli/gemini-md.md](../tmp/gemini-cli-main/docs/cli/gemini-md.md:28) | Just-in-time local context files can be loaded near accessed files. | DEFER | Potentially high prompt-injection and cache-churn risk. Reopen only with bounded ContextualUserFragment design and hard caps. |
| 24 | [docs/cli/token-caching.md](../tmp/gemini-cli-main/docs/cli/token-caching.md:3) | Stable instruction/context caching matters for cost and latency. | COVERED | Already handled by Prompt Cache Boundary ADR; avoid volatile data in stable prompt. |
| 25 | [docs/core/subagents.md](../tmp/gemini-cli-main/docs/core/subagents.md:13) | Specialized subagents should have focused context, tools, and isolated windows. | COVERED | Ontocode already has sub-agent/model guidance. Use as prompt-contract evidence only. |
| 26 | [docs/core/subagents.md](../tmp/gemini-cli-main/docs/core/subagents.md:95) | Delegate large-volume work to isolated workers to keep manager context small. | NARROW | Use for bounded manager loops; default remains direct OntoIndex/direct reads for small tasks. |
| 27 | [.gemini/commands/strict-development-rules.md](../tmp/gemini-cli-main/.gemini/commands/strict-development-rules.md:6) | Project-specific test rules should be explicit and close to the repo. | KEEP | Reinforces AGENTS.md and memory-bank instructions; do not import donor TypeScript/React rules. |
| 28 | [.gemini/commands/strict-development-rules.md](../tmp/gemini-cli-main/.gemini/commands/strict-development-rules.md:157) | Prompt changes can affect overall quality and need care. | KEEP | Add prompt changes to high-risk review gates: owner proof, focused prompt rendering test, and explicit residual risk. |
| 29 | [tools/gemini-cli-bot/brain/interactive.md](../tmp/gemini-cli-main/tools/gemini-cli-bot/brain/interactive.md:26) | Wrap external issue/PR/log content as untrusted data, never instructions. | KEEP | Strong new safety rule for GitHub issue/PR workflows, web excerpts, CI logs, and donor text ingestion. |
| 30 | [tools/gemini-cli-bot/brain/interactive.md](../tmp/gemini-cli-main/tools/gemini-cli-bot/brain/interactive.md:11) | One thing at a time: no drive-by refactors in automation/codegen runs. | KEEP | Already consistent with current user preferences and change-size guidance; keep it in future manager-loop prompts. |
| 31 | [tools/gemini-cli-bot/brain/scheduled.md](../tmp/gemini-cli-main/tools/gemini-cli-bot/brain/scheduled.md:53) | Form competing hypotheses before implementing a suspected fix. | NARROW | Useful for debugging/review tasks; not required for trivial edits. |
| 32 | [tools/gemini-cli-bot/brain/scheduled.md](../tmp/gemini-cli-main/tools/gemini-cli-bot/brain/scheduled.md:18) | Record lower-priority findings rather than bundling them into the current PR. | KEEP | Maps to memory-bank tracking and closure notes. |

## Rejected Or Parked Runtime Ideas

| Donor idea | Decision | Reason |
| --- | --- | --- |
| Full `GEMINI_SYSTEM_MD` public override/import path. | REJECT for this review | This bypasses Ontocode's stable prompt/context owners and needs a security/product ADR if ever considered. |
| Prompt macro placeholders such as `${AgentSkills}` and tool-name variables. | REJECT | Ontocode already renders tools, skills, apps, plugins, and prompt fragments. A macro language would create a second prompt compiler. |
| New behavioral-eval platform with nightly flake tracking. | DEFER | Valuable but too broad. Start with deterministic Rust/request-shape tests for accepted prompt changes. |
| Plan-mode policy engine import. | REJECT | Existing collaboration modes and sandbox/tool policy own planning behavior. |
| Markdown command runtime for strict development rules. | REJECT | AGENTS.md and skills already carry repo rules. Commands would add a parallel prompt execution surface. |
| Hierarchical `GEMINI.md` loader and JIT file-near-context import. | DEFER | Already covered by AGENTS/memory/skills. JIT context has prompt-injection and cache-churn risk. |
| Agent runtime/config import from donor subagents. | REJECT | Sub-agent model ordering and multi-agent tooling already exist in Ontocode. |
| Scheduled bot/automation workflow import. | REJECT | Out of codegen prompt scope; would add scheduler/state/PR automation surface. |
| Specialized review skill bundle wholesale import. | REJECT | Existing code-review skills plus `gn_verify_diff`/`gn_test_gap` cover the base. Keep only narrow checklist ideas when a gap is proven. |

## Proposed Follow-Up Slices

These slices are not implementation approval. Promote one only after a concrete current-owner failure is named.

### GCG-P1: Prompt/Tool Steering Test Gate

Goal: require future prompt/tool-description changes to prove behavior with an owner-local failing or missing test.

Challenge: KEEP AS PROCESS ONLY. Do not implement this as a framework, command, eval harness, or generic validator. Use existing owner tests.

Steps:

1. Start only from an accepted prompt/tool-description change.
2. Name the failed behavior or missing invariant and the exact owner.
3. Add or update the smallest deterministic owner test: request-shape, tool-call, snapshot, transcript, or `core/suite` fixture.
4. Prefer realistic 2-3 file workspaces only when the behavior depends on workspace context.
5. Assert structured facts, not model prose.
6. Keep prompt edits minimal and positive where possible.

Acceptance:

- The test fails before the prompt/tool change or the closure note explains why fail-first is impossible.
- The test lives in the existing owner, not a new eval framework.
- Verification includes focused `just test -p <changed-crate>` or relevant snapshot command.
- No task is opened from this slice without a concrete current diff or failing behavior.

### GCG-P2: Untrusted External Context Prompt Contract

Goal: prevent external donor, issue, PR, CI, and web text from becoming instructions.

Challenge: CONDITIONAL ONLY. Do not add blanket prompt text, delimiters, or wrappers everywhere.

Steps:

1. Start from one named ingestion path that puts external text into model-visible context.
2. Confirm existing redaction/sanitization and bounded-size behavior first.
3. If the text is model-visible and currently ambiguous, label it as data with an explicit owner-local delimiter or heading.
4. Add one focused test proving external text is data, not instruction, for that path.
5. Add or reuse tests that secrets, authorization headers, cookies, and raw credential paths do not appear in output.

Acceptance:

- No new GitHub/bot runtime.
- Context injection remains bounded and owner-local.
- Tests fail if external content is treated as executable instruction or leaks sensitive data.
- No action is needed for workflows that only read donor markdown during human review and do not inject it into model context.

### GCG-P3: Prompt Boundary Review Checklist

Goal: standardize how prompt changes are reviewed.

Challenge: CHECKLIST ONLY. Do not add a new validator or governance file until multiple accepted prompt assets need it.

Steps:

1. Review whether a change belongs in stable base prompt, AGENTS/memory/project context, skill body, prompt fragment, or test fixture.
2. Reject volatile project-specific guidance from stable prompts.
3. Reject large always-loaded instructions when a skill/reference can carry them lazily.
4. Require cache-boundary and context-size reasoning for any always-loaded text.
5. Require focused tests for section ordering, unresolved placeholders, and prompt-visible output.

Acceptance:

- Prompt changes preserve stable/dynamic boundaries from the Prompt Cache Boundary ADR.
- New prompt text is bounded and owner-local.
- No new user-facing prompt override, macro, or command surface.

## Recommended Next Step

No immediate Rust implementation and no new task is open. Use this file as review evidence for future prompt/tool/context changes. If implementation is requested later, start from the concrete owner and failing behavior, not from this donor review.
