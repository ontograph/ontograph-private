# System Prompt Claude Manager-Loop Update

Date: 2026-06-26

## Outcome

Updated the stable base system prompt with the only Claude donor recommendation that belongs globally: manager loops must require explorer/reviewer workers to return key files or symbols, and the manager must read that evidence before accepting output.

## Scope

- Changed [default.md](../ontocode-rs/protocol/src/prompts/base_instructions/default.md) in the existing manager-loop section.
- Did not add a codegen prompt contract, review checklist runtime, prompt asset framework, agent runtime, command runtime, or scheduler.
- Kept implementation-specific Claude recommendations in [CLAUDE_CODE_MAIN_CODEGEN_PROMPT_IDEAS_REVIEW.md](CLAUDE_CODE_MAIN_CODEGEN_PROMPT_IDEAS_REVIEW.md).

## Verification

Docs/prompt text only. `git diff --check` should be sufficient.
