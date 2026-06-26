# ADR: MiMo Codegen Prompt Architecture

## Status

Challenged - keep only owner-local prompt architecture extensions

## Date

2026-06-25

## Inputs

- MiMo donor rules: [tmp/MiMo-Code/packages/web/src/content/docs/rules.mdx](../tmp/MiMo-Code/packages/web/src/content/docs/rules.mdx)
- MiMo donor agents: [tmp/MiMo-Code/packages/web/src/content/docs/agents.mdx](../tmp/MiMo-Code/packages/web/src/content/docs/agents.mdx)
- MiMo donor modes: [tmp/MiMo-Code/packages/web/src/content/docs/modes.mdx](../tmp/MiMo-Code/packages/web/src/content/docs/modes.mdx)
- MiMo donor commands: [tmp/MiMo-Code/packages/web/src/content/docs/commands.mdx](../tmp/MiMo-Code/packages/web/src/content/docs/commands.mdx)
- MiMo donor plugins: [tmp/MiMo-Code/packages/web/src/content/docs/plugins.mdx](../tmp/MiMo-Code/packages/web/src/content/docs/plugins.mdx)
- MiMo donor last-step handling: [tmp/MiMo-Code/docs/compose/reports/align-last-step-handling.md](../tmp/MiMo-Code/docs/compose/reports/align-last-step-handling.md)
- MiMo donor plan-mode backstop: [tmp/MiMo-Code/docs/compose/reports/plan-mode-edit-write-backstop.md](../tmp/MiMo-Code/docs/compose/reports/plan-mode-edit-write-backstop.md)
- MiMo donor freshness skill example: [tmp/MiMo-Code/packages/opencode/test/fixture/skills/agents-sdk/SKILL.md](../tmp/MiMo-Code/packages/opencode/test/fixture/skills/agents-sdk/SKILL.md)

Gate: keep only ideas that extend the current Ontocode prompt, session, skill, or plugin owners. Do not create a parallel prompt runtime.

## OntoIndex Challenge Evidence

OntoIndex repo `codex` resolves the current owners as:

- `Session.build_initial_context` in [ontocode-rs/core/src/session/mod.rs](../ontocode-rs/core/src/session/mod.rs) builds the initial prompt/context assembly path, injects developer and user instructions, attaches prompt fragments, enforces context-window budgets, and already has focused tests for prompt fragments and usage hints.
- `SkillsManager.skills_for_config` in [ontocode-rs/core-skills/src/manager.rs](../ontocode-rs/core-skills/src/manager.rs) is the current owner for skill discovery and config-layer skill loading.
- `PluginsManager.plugins_for_config_with_force_reload` in [ontocode-rs/core-plugins/src/manager.rs](../ontocode-rs/core-plugins/src/manager.rs) is the current owner for plugin configuration, layer-stack loading, and cached plugin enablement.

Challenge result: MiMo's useful ideas are architectural pressure on these owners, not justification for a second agent/mode/plugin stack. OntoIndex also shows that compaction and final-answer verification live on separate paths from initial prompt assembly, so those behaviors must not be bundled into the first implementation slice.

## Decision

Keep three active ideas and park two until a concrete current-owner gap is proven.

| Candidate | MiMo source | Decision | Existing owner | Boundary |
| --- | --- | --- | --- | --- |
| Layer prompt responsibilities instead of one monolithic prompt blob | rules.mdx, agents.mdx, modes.mdx | KEEP | session prompt/context assembly | Tighten and test the section ordering already present in `Session.build_initial_context`: developer sections, separate developer sections, contextual user sections, and prompt-fragment slots. Do not add a new prompt-section abstraction unless repeated code proves one is needed. |
| Lazy-loaded external instruction references | rules.mdx | KEEP | skills, prompt fragments, AGENTS/instruction loading | Reuse the current instruction and skill-loading paths so large optional guidance stays out of the default prompt unless explicitly needed. Do not add arbitrary remote prompt fetch or open-ended include chains. |
| Markdown/frontmatter prompt assets for agents, modes, or commands | agents.mdx, modes.mdx, commands.mdx | DEFER | skills first; command/agent owners only after proof | Active scope is only skills-backed markdown that already fits the skill owner. Agent, mode, and command frontmatter formats would create public product surface and need a separate ADR or reproduced owner-local gap. |
| Structured continuation/final-step contract | align-last-step-handling.md, plan-mode-edit-write-backstop.md | DEFER | compaction, request finalization, final-answer verifier | Useful invariant, but not part of prompt assembly. Reopen only after identifying the exact request/finalization owner and a failing case where a capped turn still has tools enabled or lacks a bounded final summary. |
| Freshness/retrieval preamble for volatile domains | SKILL.md | KEEP | skill guidance and prompt-fragment assembly | Add only as domain-specific skill or prompt-fragment guidance for volatile SDK/framework areas. Do not inject broad stale-warning prose into every turn. |

## Rejected or Deferred Donor Ideas

| Donor idea | Why it does not carry over |
| --- | --- |
| Separate agent markdown runtime with its own model/tool registry | Ontocode already has agent, skill, and plugin owners. A second runtime would split ownership without adding core behavior. |
| Separate mode markdown runtime with independent prompt/tool routing | Existing collaboration mode, skills, and prompt fragments already own this surface. Keep extensions inline with those owners. |
| Slash-command macro system as a new prompt-language feature | Useful only if mapped onto existing command/config owners. A free-standing macro runtime would be new surface area, not a core-engine extension. |
| Plugin hooks that can fully replace the system prompt | This would bypass the current session/context assembly owner and weaken prompt invariants. Keep plugins as bounded contributors, not full prompt replacement owners. |
| Plugin continuation or compaction prompt replacement | Current plugin evidence only supports bounded loading/contribution through plugin owners. Full continuation or compaction prompt replacement belongs outside this ADR. |
| Broad marketplace or UI-driven prompt product features | Not core engine work. They add product/runtime surface without a reproduced owner-local gap. |

## First Implementation Slice

Smallest coherent first slice:

1. Add focused tests that lock down the existing `build_initial_context` ordering for skill metadata, prompt fragments, separate developer sections, and contextual user sections.
2. Keep optional heavy guidance owned by the existing skill-loading/rendering tests instead of adding a second proof path in session tests.
3. Add only tiny naming or helper cleanup inside `build_initial_context` if the tests expose unclear local ownership. Skip new abstractions until duplication proves they are needed.

This first slice extends the current core engine. It does not require new public config keys, new app-server API surface, a new prompt registry, or donor-style product UX.

Implementation status: first slice landed as focused `build_initial_context` regression coverage in [ontocode-rs/core/src/session/tests.rs](../ontocode-rs/core/src/session/tests.rs). No production prompt runtime or public prompt asset surface was added.

## Parked Follow-Up Slice

Final-step and continuation behavior is parked as a separate slice.

Reopen criteria:

1. Identify the exact owner that sends the capped final request or continuation prompt.
2. Show a current failing test or transcript where tools remain available when they should be disabled, or where the final summary is unbounded or missing.
3. Implement only in that owner, with a focused request-shape or transcript test.

## Acceptance Criteria

- Prompt assembly remains owned by the current session/context path.
- Optional heavy guidance remains owned by the current skill and instruction loading paths, not by a new prompt runtime.
- Initial implementation does not touch final-step, compaction, continuation, or final-answer verification paths.
- Any active markdown-defined prompt asset compiles into existing skill owners instead of creating a second runtime taxonomy.
- Tests cover the kept behavior at the existing session and skill owners.
- No new public config key, app-server API, SDK behavior, or user-facing prompt marketplace is introduced without a separate ADR and compatibility tests.

## Narrow Implementation Notes

- Start with session tests around `build_initial_context`.
- Reuse current skill metadata and prompt-fragment contribution paths for lazy-loaded guidance.
- If markdown/frontmatter prompt assets are added later, start with skills-backed markdown only. Agent, mode, and command prompt assets need separate approval.
- Keep final-step handling out of this first slice unless the parked follow-up reopen criteria are met.
- Treat MiMo's donor markdown as evidence for better boundaries and contracts, not as a framework to port.
