---
name: Agent Definition S2 Closure
description: Closure note for the picker-owned optional-field follow-up on repo-local agent definition scaffolds
type: audit-session
date: 2026-06-24
status: accepted
---

# Agent Definition S2 Closure

Authority:
- `CLAUDE_CODE_AGENT_DEFINITION_UI_SOLUTIONS_REVIEW.md`
- `CLAUDE_CODE_AGENT_DEFINITION_UI_SOLUTIONS_TRACKING.md`

## Decision

`AGDEF-S2` is complete.

The implemented scope stays narrow:
- the existing create-flow name prompt now leads to one second multiline prompt
- that second prompt accepts only the optional role fields `model`, `model_reasoning_effort`, `service_tier`, and `nickname_candidates`
- the write target stays the same repo-local `.codex/agents/<slug>.toml` path
- fields left absent or commented are omitted from the scaffold
- invalid `nickname_candidates` fail before the file is written

The slice intentionally does not add:
- slash-dispatch changes
- runtime role application
- source precedence changes
- hot reload
- app-server APIs
- arbitrary file-open editor plumbing

## Verification

Exact checks passed:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui -- app::tests::create_agent_definition_scaffold_writes_repo_local_role_file app::tests::create_agent_definition_scaffold_writes_optional_fields_when_provided app::tests::create_agent_definition_scaffold_rejects_invalid_nickname_candidates`

Covered behaviors:
- the original scaffold path still writes the minimal file when no optional fields are provided
- provided optional fields are written in normalized TOML form
- duplicate `nickname_candidates` are rejected on the existing create path before write

## Residual Risk

- The second prompt uses a TOML-fragment template rather than a field-by-field widget. That keeps scope small, but it is still text entry.
- OntoIndex `gn_verify_diff` stayed noisy because the worktree already had broad unrelated changes and snapshot drift; local diff inspection was used to confirm the intended write set.

## Follow-up

- `AGENTIC-S2` is now the next queued `/agent` follow-up outside this review line.
