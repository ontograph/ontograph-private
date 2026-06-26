# ADR: Qwen Codegen Prompt Workflows

## Status

Implemented - Slice 1 complete, parked slices unchanged

## Date

2026-06-25

## Inputs

- Qwen donor repo guidance: [tmp/qwen-code/AGENTS.md](../tmp/qwen-code/AGENTS.md)
- Qwen batch skill: [tmp/qwen-code/packages/core/src/skills/bundled/batch/SKILL.md](../tmp/qwen-code/packages/core/src/skills/bundled/batch/SKILL.md)
- Qwen review skill: [tmp/qwen-code/packages/core/src/skills/bundled/review/SKILL.md](../tmp/qwen-code/packages/core/src/skills/bundled/review/SKILL.md)
- Qwen review design: [tmp/qwen-code/packages/core/src/skills/bundled/review/DESIGN.md](../tmp/qwen-code/packages/core/src/skills/bundled/review/DESIGN.md)
- Qwen simplify skill: [tmp/qwen-code/packages/core/src/skills/bundled/simplify/SKILL.md](../tmp/qwen-code/packages/core/src/skills/bundled/simplify/SKILL.md)
- Qwen new-app skill: [tmp/qwen-code/packages/core/src/skills/bundled/new-app/SKILL.md](../tmp/qwen-code/packages/core/src/skills/bundled/new-app/SKILL.md)
- Qwen markdown command example: [tmp/qwen-code/packages/cli/src/commands/extensions/examples/starter/commands/writing/polish.md](../tmp/qwen-code/packages/cli/src/commands/extensions/examples/starter/commands/writing/polish.md)
- Qwen prompt suggestion design: [tmp/qwen-code/docs/design/prompt-suggestion/prompt-suggestion-design.md](../tmp/qwen-code/docs/design/prompt-suggestion/prompt-suggestion-design.md)
- Qwen reduce-rounds via skill design: [tmp/qwen-code/docs/design/rt-optimization/reduce-rounds-via-skill-design.md](../tmp/qwen-code/docs/design/rt-optimization/reduce-rounds-via-skill-design.md)
- Qwen auto-skill design: [tmp/qwen-code/docs/design/skill-nudge/skill-nudge.md](../tmp/qwen-code/docs/design/skill-nudge/skill-nudge.md)
- Qwen declarative agent port notes: [tmp/qwen-code/docs/declarative-agents-port.md](../tmp/qwen-code/docs/declarative-agents-port.md)

Gate: keep only ideas that extend current Ontocode skill, prompt-fragment, tool-search, multi-agent, review, and test-evidence owners. Do not add a second skill runtime, command runtime, agent frontmatter runtime, speculative executor, or overlay filesystem.

## OntoIndex Challenge Evidence

OntoIndex repo `codex` is indexed at commit `2e72a6d25e147f0619863e7721107b6f11a87fc2` and reports a dirty worktree, so implementation must stay narrowly scoped and re-check affected symbols before edits.

- `SkillsManager.skills_for_config` in [ontocode-rs/core-skills/src/manager.rs](../ontocode-rs/core-skills/src/manager.rs) is the existing skill discovery/config owner. Codegen prompt workflows must reuse this surface instead of adding a Qwen-style parallel skill registry.
- `Session.build_initial_context` in [ontocode-rs/core/src/session/mod.rs](../ontocode-rs/core/src/session/mod.rs) already assembles skill metadata, prompt fragments, contextual user sections, plugins, apps, and developer instructions. New prompt guidance must enter through existing skill metadata or prompt-fragment paths.
- `tool_search`, multi-agent tools, agent job handlers, and review prompt templates already exist in Ontocode. Qwen's `/batch`, `/review`, and `/simplify` prompts are useful as workflow contracts, not as a runtime import.
- Current Ontocode skill frontmatter supports `name`, `description`, and `metadata.short-description`; richer Qwen fields such as `argument-hint` and `allowedTools` are donor evidence only, not accepted loader schema.

## Decision

Accept Slice 1 only. Park all other candidates behind concrete evidence gates.

| Candidate | Qwen source | Decision | Existing owner | Boundary |
| --- | --- | --- | --- | --- |
| Post-implementation simplify workflow | `simplify/SKILL.md` | KEEP ACTIVE | skills, review, diff verification, test-gap tools | Add or update one small Ontocode skill/workflow contract that reviews the current diff for reuse, code quality, and efficiency, then applies only safe local cleanups. No new cleanup engine and no loader/schema change. |
| Bounded batch codegen/edit workflow | `batch/SKILL.md` | PARK | multi-agent handlers, tool-search, existing edit/apply-patch/test owners | Keep as donor guidance only until a repeated mechanical multi-file task proves existing `spawn_agent`, tool-search, and apply-patch guidance is insufficient. Do not create a scheduler, file queue, or new batch runtime. |
| Skill-output completeness for codegen workflows | `reduce-rounds-via-skill-design.md` | PARK | skill metadata/rendering, prompt fragments, tool result/test evidence | Evidence-first only. Reopen for one named skill or prompt-fragment workflow after a concrete repeated immediate read/search follow-up is shown. No telemetry platform or generic output contract work in this ADR. |
| Deterministic checks before model review | `review/SKILL.md`, `review/DESIGN.md` | NARROW | existing review prompts, test-gap, verify-diff | Keep as rule text for accepted workflows: run deterministic checks first, then model review. Do not port Qwen's 9-agent review pipeline. |
| Markdown command `{{args}}` prompt assets | `polish.md` and command examples | DEFER | skills first; command owners only after proof | Useful format, but a command macro runtime is new product surface. Reopen only if existing slash-command/skill owners cannot express a concrete codegen command. |
| Next-step prompt suggestion filters | `prompt-suggestion-design.md` | DEFER | TUI/composer only after UX demand | Filter rules are useful evidence, but not codegen implementation. Reopen only with a current TUI follow-up-suggestion owner gap. |
| Auto-skill extraction | `skill-nudge.md` | DEFER | memories and skills only after safety proof | Potentially useful, but it writes project-local skills automatically. Requires separate ADR for ownership, protection, permissions, and user control. |
| Declarative agent markdown/frontmatter runtime | `declarative-agents-port.md` | REJECT for this ADR | agent role/config owners | Too much public surface for this goal. Use only as future schema evidence if a concrete agent-owner gap is reproduced. |
| Speculative execution / overlay filesystem | prompt suggestion implementation notes | REJECT | none in this ADR | High complexity and broad behavior risk. Not needed to improve code generation prompts. |

## Implementation Plan

Slice 1 is implemented. Keep parked slices closed unless separate evidence justifies reopening them.

### Slice 1: Simplify Skill Contract

Goal: make a small existing-loader codegen cleanup workflow that does less, safely.

Steps:

1. Add or update one Ontocode skill definition for a `simplify` workflow using the existing skill loader format.
2. The skill must inspect the current diff, identify low-risk reuse/quality/efficiency cleanups, and directly apply only straightforward edits.
3. It must skip uncertain, cross-owner, architectural, or broad refactor findings.
4. It must require focused validation for any changed code path and report skipped checks.
5. Tests should cover skill discovery/rendering and prompt text boundaries, not a new runtime.

Acceptance:

- The skill is discoverable through existing `SkillsManager` paths.
- No public config key, app-server API, or second command runtime is added.
- No new skill loader schema is added; Qwen frontmatter fields remain donor evidence only.
- The prompt says to prefer no change over speculative cleanup.

### Parked Slice 2: Bounded Batch Workflow

Goal: proposal-only evidence gate for mechanical codegen/edit work across multiple files.

Steps:

1. First prove a repeated task where existing `spawn_agent`, tool-search, and apply-patch guidance is insufficient.
2. If proven, add prompt-only guidance that discovers files, applies default exclusions, chunks work, and caps workers.
3. Default caps: no generated/vendor/build outputs, no files over a documented size cap, and no more than five workers without a separate approval.
4. Require per-chunk success/failed/skipped reporting and aggregate summary.
5. Add focused tests around prompt/tool-spec text or workflow parsing only where current owners already have test harnesses.

Acceptance:

- Reopen evidence names the repeated task and why current owners failed.
- Batch guidance cannot silently touch test/generated/vendor files unless explicitly requested.
- More than 100 candidate files remains a warning/stop condition, not automatic dispatch.
- No background queue, scheduler, or persistent batch state is introduced.

### Parked Slice 3: Skill Output Completeness

Goal: evidence-first check for reducing avoidable extra model rounds.

Steps:

1. Identify one existing skill or prompt-fragment workflow with a concrete repeated immediate read/search follow-up.
2. If found, extend only that skill's output contract or prompt text to return bounded read-only evidence.
3. Do not inline writes, approvals, deep analysis, or cross-skill chaining.
4. Keep token size bounded; if the skill body would grow too much, split guidance instead of stuffing the prompt.
5. Add one regression test that proves the selected skill metadata/body tells the model what evidence is returned.

Acceptance:

- The change improves an existing owner; it does not add a telemetry platform or new skill execution engine.
- Returned evidence is bounded and redacted through existing tool/session rules.
- If no concrete repeated follow-up is found, close this slice as no-dispatch.

## Rejected Runtime Imports

- Do not port Qwen's full `/review` nine-agent workflow. Ontocode already has review, test-gap, verify-diff, and multi-agent surfaces; use deterministic checks and focused reviewer prompts instead.
- Do not add Qwen-style markdown command macros in this ADR. Skills already cover the first useful prompt asset shape.
- Do not add auto-skill extraction until a separate ADR proves permissions, user control, and overwrite protection.
- Do not add speculative execution, cache-aware fork queries, copy-on-write overlay filesystems, or predictive tool execution.
- Do not add declarative agent frontmatter fields from Qwen or Claude Code as part of codegen prompt workflow work.

## Verification

For any implementation slice:

- Run OntoIndex impact on edited symbols before code changes.
- Run `CARGO_BUILD_JOBS=8 just fmt` after Rust changes.
- Run focused `just test -p <changed-crate>` tests for the touched owner.
- Run `gn_verify_diff` or note dirty-worktree limitations before closure.
- Update this ADR or add a closure note if a slice is completed, blocked, or narrowed.

## Current Next Step

No active implementation remains in this ADR. Keep Slice 2 and Slice 3 parked until separate evidence justifies reopening them.
