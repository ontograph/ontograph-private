---
name: Agentic S8 Unblock
description: Senior unblock note promoting repo-local definition copy as the only active `/agent` follow-up
type: audit-session
date: 2026-06-24
status: recorded
---

# Agentic S8 Unblock

## Scope

- Authority: `CLAUDE_CODE_AGENTIC_ENGINE_SOLUTIONS_TRACKING.md`
- Queue reviewed: `AGENTIC-S2` through `AGENTIC-S8`
- Goal: decide what is actually unblocked now without widening `/agent` into new runtime, registry, or profile surfaces

## OntoIndex Evidence

- `App.open_agent_picker` remains the accepted owner and has `MEDIUM` upstream impact concentrated in focused picker tests under `ontocode-rs/tui/src/app/tests.rs`.
- The repo-local definition authoring path already exists in code:
  - `AppEvent::OpenCreateAgentDefinitionPrompt`
  - `ChatWidget::show_create_agent_definition_prompt`
  - `AppEvent::CreateAgentDefinitionScaffold`
  - `App::create_agent_definition_scaffold`
- No new OntoIndex evidence justified reopening:
  - `AGENTIC-S2` `Create from proposal`, which still depends on the separate `AGDEF-S2` structured-field gate
  - `AGENTIC-S3` deterministic direct dispatch
  - `AGENTIC-S4` persistent profile/config writes
  - `AGENTIC-S5` expanded `agent_jobs` UX

## Decision

- Promote `AGENTIC-S8` to the only active next slice.
- Keep `AGENTIC-S8` narrow:
  - duplicate a repo-local `.codex/agents/<slug>.toml`
  - update only destination slug/path and internal `name = "..."`
  - stay inside the current picker, prompt, app event, and scaffold-writer owners
- Keep `AGENTIC-S2` blocked behind `AGDEF-S2`.
- Keep `AGENTIC-S3/S4/S5` blocked until fresh existing-owner evidence proves a real gap.

## Explicit Non-Scope

- No app-server API
- No direct tool/runtime dispatch
- No new registry or hot reload
- No persistent profile or config precedence work
- No history-mining or multi-chat proposal flow
