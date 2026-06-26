# Prompt: Fix `AGENTS.md` Sub-Agent Instructions

Use this prompt with another coding agent when you want it to fix the repo's `AGENTS.md` guidance for sub-agent dispatch.

## Prompt

Review and fix `AGENTS.md` so the sub-agent instructions match the runtime constraints we already observed.

Scope:
- Edit `AGENTS.md` only unless one adjacent markdown file must be updated to remove an obvious contradiction.
- Keep the change narrow. Do not refactor unrelated instructions.

Required fixes:
- In the sub-agent model preference list, keep the exact model ids:
  - `gemini-3.5-flash-low`
  - `gemini-3-flash-agent`
  - `gemini-pro-agent`
  - `claude-sonnet-4-6`
  - `gpt-5.3-codex-spark`
  - `gpt-5.4-mini`
- State explicitly that `gemini-3.5-flash-low` must be spawned **without** `reasoning_effort`.
- State explicitly that if one exact model fails today, the same exact model should not be retried again until tomorrow; move to the next model in order.
- Preserve the existing requirement that exact model names must not be truncated.
- If the file contains any wording that implies a generic `reasoning_effort=medium` default for all models, remove or narrow it so it does not apply to `gemini-3.5-flash-low`.
- If sub-agent/tool call guidance is present, keep or strengthen the rule that the `spawn_agent` function-call namespace must be preserved.

Working rules:
- Read the existing `AGENTS.md` first and patch only the relevant section.
- Do not invent new workflow policy outside this issue.
- Keep wording operational and unambiguous.

Deliverables:
1. Patch to `AGENTS.md`.
2. Short summary of what changed.
3. Any remaining ambiguity or runtime limitation that still cannot be expressed cleanly in `AGENTS.md`.

Acceptance criteria:
- `AGENTS.md` no longer suggests using `reasoning_effort` with `gemini-3.5-flash-low`.
- The model roster uses the exact ids above.
- The retry/fallback behavior is explicit.
- The edit is small and limited to the relevant agent-dispatch guidance.

## Why this exists

We already hit a real launch failure because `gemini-3.5-flash-low` does not support the requested `reasoning_effort=medium`. The doc fix should prevent repeating that misconfiguration.
