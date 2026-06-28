---
name: Manager Loop Native Iteration Tools Refactoring Project Plan
description: Refactoring plan for strict native manager-loop iteration helpers without adding a second scheduler or task runtime
type: project-plan
date: 2026-06-28
status: first-slice-landed
---

# Manager Loop Native Iteration Tools Refactoring Project Plan

## Purpose

Make "continue through all eligible open tasks until done" stricter and easier for the model to execute by adding small helper tools around the existing manager-loop and multi-agent owners.

This is not a plan for a new autonomous loop daemon. The manager remains the current session. The helpers should make the next legal action explicit, verify closeout, and prevent role/model drift.

## Current Architecture

Existing owners already cover the runtime pieces:

- `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs` defines `spawn_agent`, `wait_agent`, `followup_task`, `list_agents`, and `close_agent` tool specs.
- `ontocode-rs/core/src/tools/handlers/multi_agents_v2/*` implements the current v2 sub-agent tools.
- `ontocode-rs/core/src/agent/control.rs` owns agent spawn, list, and close behavior.
- `ontocode-rs/core/src/tools/handlers/agent_jobs.rs` and `ontocode-rs/state/src/runtime/agent_jobs.rs` already provide a durable batch-worker queue, but that queue is CSV/job oriented and should not become the manager-loop tracker.
- `.memory-bank/ADR_SUBAGENT_DISPATCH_RELIABILITY.md` and `.memory-bank/ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md` explicitly reject a second planner, scheduler, queue, or manager-loop daemon.

OntoIndex status during review: fresh index at `5edde24a78efe0f10bc710936dfa228427ab7fd1`, dirty worktree, medium confidence.

## Refactoring Rule

Add native support as a staged family of small, existing-owner read-only tools. The landed first slice is only `manager_loop_next`; role/model argument generation and closeout legality checks remain deferred until fresh evidence reopens them.

- read bounded tracking state from `.memory-bank/*.md`;
- return the next legal iteration decision;
- if reopened later, return exact role/model dispatch arguments;
- if reopened later, verify closeout legality.

Do not add:

- a background manager-loop service;
- a new task queue;
- a second role registry;
- a second model router;
- app-server or public config surface.

## Authoritative Structured Tracking Block

For this effort, this file is the authoritative tracking file. The first-slice helper reads one strict fenced YAML block and rejects prose/table fallback.

First-slice strict mode must not fall back to prose or legacy-table inference. If the structured block is absent, malformed, or not marked as authoritative, return `invalid_tracking`.

```yaml
manager_loop:
  authority: true
  active_next_task: null
  last_decision: dispatch
  reopen_gate: null
  required_roles:
    senior-reviewer:
      model: gemini-pro-agent
    implementation-worker:
      models:
        - gpt-5.3-codex-spark
        - gpt-5.4-mini
      reasoning_effort: high
    verification-worker:
      model: gpt-5.4-mini
      reasoning_effort: high
  tasks:
    - id: R1
      status: CLOSED
      classification: implementation-ready
      depends_on: []
      owner: existing multi-agent owners
      verification:
        - CARGO_TARGET_DIR=/tmp/ontocode-manager-loop-next-fix CARGO_BUILD_JOBS=8 just test -p ontocode-core manager_loop_next
        - CARGO_TARGET_DIR=/tmp/ontocode-manager-loop-spec CARGO_BUILD_JOBS=8 just test -p ontocode-core manager_loop_next_is_visible_and_registered_by_default
        - CARGO_TARGET_DIR=/tmp/ontocode-manager-loop-spec CARGO_BUILD_JOBS=8 just test -p ontocode-core environment_count_controls_environment_backed_tools
```

Allowed `classification` values:

- `implementation-ready`
- `docs/design-only`
- `proof-only`

Allowed `status` values:

- `OPEN`
- `CLOSED`
- `BLOCKED`

## Tool 1: `manager_loop_next`

Status: landed for the first implementation slice, with focused validation. Follow-up cleanup fixed the test helper so validation files are written under a temp workspace instead of `ontocode-rs/core`; strict task parsing now rejects unknown task fields while allowing `owner` and `verification` metadata; tracking input, exact-gate output, and required-role output are bounded.

Owner:

- `ontocode-rs/core/src/tools/handlers/manager_loop_next.rs` or a small submodule under existing tool handlers.
- Tool spec alongside other core tool specs, without creating a new runtime owner.

Input:

```json
{
  "tracking_path": ".memory-bank/ADR_SUBAGENT_DISPATCH_RELIABILITY_TRACKING.md",
  "mode": "strict"
}
```

Input constraints:

- `tracking_path` must be workspace-relative and under `.memory-bank/`.
- Only Markdown files are accepted.
- Output must be capped and must not echo arbitrary tracking-file body text.
- The tool is read-only and must not update tracking state.

Output:

```json
{
  "decision": "execute_active_next_task",
  "task_id": "R3",
  "reason": "active_next_task is set and classified implementation-ready",
  "required_roles": [
    {
      "role": "senior-reviewer",
      "required": true
    }
  ],
  "reopen_gate": null
}
```

Decision enum:

- `execute_active_next_task`
- `promote_next_open`
- `no_dispatch`
- `complete`
- `invalid_tracking`

Rules:

- Prefer `active_next_task` when present.
- Else if the last decision was `no-dispatch`, return `no_dispatch` with the exact reopen gate.
- Else select the first dependency-ready `OPEN` task with `classification: implementation-ready`.
- If no implementation-ready task remains, return `complete` with `nothing left in scope`.
- Never promote `proof-only`, `docs/design-only`, or non-`OPEN` tasks into implementation.
- Do not return generated worker prompts; return structured decision fields only.

Why first:

- It gives the model a deterministic next step without adding an executor loop.
- It is read-only, so blast radius stays low.

## Tool 2: `manager_loop_roles`

Status: deferred. Reconsider only after `manager_loop_next` lands and fresh evidence shows exact role/model drift still happens despite the existing `spawn_agent` metadata and prompt guardrails.

Purpose:

Return exact `spawn_agent` call arguments for the current manager-loop role contract.

Input:

```json
{
  "tracking_path": ".memory-bank/ADR_SUBAGENT_DISPATCH_RELIABILITY_TRACKING.md",
  "task_id": "R3"
}
```

Output:

```json
{
  "dispatch": [
    {
      "agent_type": "senior-reviewer",
      "model": "gemini-pro-agent"
    },
    {
      "agent_type": "implementation-worker",
      "model": "gpt-5.3-codex-spark",
      "reasoning_effort": "high"
    }
  ],
  "fallbacks": [
    {
      "agent_type": "implementation-worker",
      "model": "gpt-5.4-mini",
      "reasoning_effort": "high",
      "use_only_if": "gpt-5.3-codex-spark unavailable"
    }
  ],
  "unavailable": []
}
```

Rules:

- Use exact model ids only.
- Do not emit aliases.
- Do not emit ordered lists in the `model` field.
- If a model is not available, return `unavailable` with the exact role/model pair.
- Reuse the existing model catalog and role files; do not add a second role/model registry.

## Tool 3: `manager_loop_closeout_check`

Status: deferred. Reconsider only after read-only next-step decisions prove useful and a fresh closeout failure remains.

Purpose:

Verify whether the manager may legally stop or must continue.

Input:

```json
{
  "tracking_path": ".memory-bank/ADR_SUBAGENT_DISPATCH_RELIABILITY_TRACKING.md",
  "executed_roles": [
    {
      "role": "senior-reviewer",
      "model": "gemini-pro-agent",
      "status": "dispatched"
    }
  ],
  "executed_tests": [
    "CARGO_BUILD_JOBS=8 just test -p ontocode-core spawn_agent_tool"
  ]
}
```

Output:

```json
{
  "closeout_allowed": false,
  "reason": "implementation-worker required role is not accounted for",
  "missing_roles": ["implementation-worker"],
  "remaining_tasks": ["R3"],
  "required_next_action": "manager_loop_next"
}
```

Checks:

- Every required role is one of `dispatched`, `not_dispatched`, or `intentionally_skipped`.
- Generic substitutions such as `explorer` or `worker` are rejected when exact roles were required.
- No dependency-ready `OPEN` implementation task remains before final closeout.
- `no_dispatch` includes the exact reopen gate.
- Validation commands required by the task are reported or explicitly blocked.

## Optional Tool 4: `manager_loop_record`

Status: defer.

Purpose:

Write the structured `manager_loop` block back to the tracking file.

Reason to defer:

- Markdown mutation is higher risk than read-only decision helpers.
- Manual tracking updates already work.
- Add this only after read-only helpers prove useful and the remaining friction is stale tracking updates.

## Implementation Order

1. Add structured parsing types and `manager_loop_next` as read-only.
2. Add tests for active task, no-dispatch gate precedence, dependency-ready open task, no remaining implementation-ready task, invalid strict tracking, path rejection outside `.memory-bank/`, and no generated prompt field.
3. Add `manager_loop_roles` only if exact role/model drift remains after prompt and role-spec fixes.
4. Add `manager_loop_closeout_check` to prevent illegal final answers.
5. Reconsider `manager_loop_record` only with a fresh evidence-backed need.

## Expected Files

Likely first slice:

- `ontocode-rs/core/src/tools/handlers/manager_loop_next.rs`
- `ontocode-rs/core/src/tools/handlers/manager_loop_next_tests.rs`
- `ontocode-rs/core/src/tools/handlers/mod.rs`
- `ontocode-rs/core/src/tools/spec_plan.rs` or the current tool-planning owner that registers core tools

Avoid touching:

- `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`;
- app-server APIs;
- config schema;
- agent job database schema;
- model-provider owners;
- TUI unless there is a separate UX request.

## Validation Plan

For first slice:

- OntoIndex impact on the tool registration owner and new handler symbol before edits.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core manager_loop_next`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core multi_agent` if shared tool registration changes
- `CARGO_BUILD_JOBS=8 just fmt`
- `git diff --check -- <touched files>`
- `gn_verify_diff` scoped to expected files and executed tests.

If tool registration touches shared core surfaces, run the broader crate test:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-core`

Current first-slice closure evidence is focused only. Broader `multi_agent` and full `ontocode-core` validation were not run in this pass and remain optional confidence checks, not blockers for the narrowly landed read-only helper.

Ask before full workspace tests.

## Acceptance Criteria

- A manager can call one read-only tool to know whether to execute, promote, stop, or report no-dispatch.
- `active_next_task` wins first; absent that, `last_decision: no-dispatch` wins before any `OPEN` task promotion.
- The tool never promotes non-`OPEN`, proof-only, or docs/design-only tasks into implementation.
- Exact reopen gates are preserved.
- Strict mode rejects missing or malformed structured tracking instead of guessing from prose.
- Strict mode rejects structured blocks that do not set `authority: true`.
- Paths outside `.memory-bank/*.md` are rejected.
- No new scheduler, queue, daemon, role registry, or model router exists.

## Rejected Approaches

- Background auto-loop executor: rejected as a second scheduler.
- Reusing `agent_jobs` for manager-loop tracking: rejected because it is batch-worker CSV/job infrastructure, not ADR/tracking authority.
- Free-form prose parser only: rejected because strict mode needs a stable structured block.
- Prompt generator inside `manager_loop_next`: rejected because the helper should return decisions, not create a second prompt-policy owner.
- Role/model helper as first work: rejected because it risks duplicating existing `spawn_agent` role/model surfaces without fresh failure evidence.
- App-server API first: rejected because this is internal manager-loop ergonomics, not a public API requirement.

## Current Recommendation

Start with `manager_loop_next` only.

That is the smallest useful native support: it makes the next iteration deterministic while keeping the manager in the current session and preserving existing architecture boundaries.
