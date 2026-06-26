# Qwen Codegen Prompt Workflows Slice 1 Closure

Date: 2026-06-26

Scope:

- Implemented only Slice 1 from `ADR_QWEN_CODEGEN_PROMPT_WORKFLOWS.md`.
- Added repo-local `.codex/skills/simplify/SKILL.md`.
- Added focused `core-skills` coverage for repo-scope discovery, rendered availability, and prompt guardrails.

Kept closed:

- No skill loader schema changes.
- No Qwen `argument-hint` or `allowedTools` frontmatter support.
- No batch runtime, scheduler, file queue, telemetry platform, command runtime, or multi-agent change.

Verification:

- OntoIndex freshness checked for repo `codex`; index was current but worktree dirty.
- OntoIndex impact for `SkillsManager.skills_for_config` reported CRITICAL blast radius, so implementation avoided loader changes.
- Focused Rust verification belongs to `ontocode-rs/core-skills`.
