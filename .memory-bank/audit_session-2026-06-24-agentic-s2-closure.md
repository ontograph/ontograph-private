---
name: Agentic S2 Closure
description: Closure note for the picker-owned create-from-proposal agent definition flow
type: audit-session
date: 2026-06-24
status: accepted
---

# Agentic S2 Closure

Authority:
- `CLAUDE_CODE_AGENTIC_ENGINE_SOLUTIONS_REVIEW.md`
- `CLAUDE_CODE_AGENTIC_ENGINE_SOLUTIONS_TRACKING.md`
- `ADR_AGENT_SLASH_SUBAGENT_MANAGEMENT.md`

## Decision

`AGENTIC-S2` is complete.

The implemented scope stays narrow:
- `/agent` now includes `Create from proposal` beside the existing create action
- selecting it opens one freeform multiline prompt
- the first non-empty line becomes the role name
- remaining proposal text becomes both the generated `description` and `developer_instructions` source
- the write target stays the existing repo-local `.codex/agents/<slug>.toml` path
- generated fields are limited to `name`, `description`, and `developer_instructions`

The slice intentionally does not add:
- slash-dispatch changes
- runtime role mutation
- model or service-tier generation
- app-server APIs
- new registries
- history mining
- hot reload

## Verification

Exact checks passed:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui -- open_agent_picker_allows_create_action_when_no_threads_exist open_agent_picker_allows_create_from_proposal_action_when_no_threads_exist create_agent_definition_from_proposal_scaffold_writes_repo_local_role_file create_agent_definition_from_proposal_scaffold_requires_role_details open_agent_picker_create_from_proposal_row_snapshot`

Covered behaviors:
- the picker still preserves the existing create action and now exposes the proposal action
- a freeform proposal writes a repo-local scaffold with only the accepted fields
- proposal authoring rejects a prompt that lacks role details after the first-line name
- the new picker row has focused snapshot coverage

## Residual Risk

- Proposal parsing is intentionally simple: first line for the role name, remaining text for role details. This is deliberate scope control, not a template engine.
- OntoIndex `gn_verify_diff` stayed noisy because the worktree already had broad unrelated changes and snapshot drift; local diff inspection was used to confirm the intended write set.

## Follow-up

- Only the already-blocked `AGENTIC-S3/S4/S5` proposals remain on the `/agent` tracking line.
