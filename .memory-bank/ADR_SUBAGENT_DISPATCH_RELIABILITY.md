---
name: Sub-Agent Dispatch Reliability
description: Consolidated implementation ADR for manager-loop role dispatch, strict model selection, and namespace-safe sub-agent execution
type: adr
date: 2026-06-28
status: accepted
---

# ADR: Sub-Agent Dispatch Reliability

Authority:
- `ADR_CURRENT_SUB_AGENT_HANDLING.md`
- `ADR_CUSTOM_SUBAGENT_MODELS.md`
- `ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md`
- `ADR_CHAT_LOG_RECOMMENDATION_CLOSEOUT_AND_PROOF_ROUTING.md`

## Context

Recent review of session `019f08c4-9531-7ba1-9918-911b1bc33977` showed a bounded manager-style loop that should have used sub-agents, but the loop completed without actually dispatching the expected worker legs.

This is not a greenfield sub-agent problem. Current Ontocode owners already provide the narrow spawn/runtime pieces:

- `spawn_agent` already validates exact model ids through the shared model catalog.
- `spawn_agent.model` already supports `inherit` and `fast`.
- `spawn_agent` tool descriptions already expose bounded exact-id guidance.
- `Prompt::get_formatted_input()` already has regression coverage for restoring an unambiguous function-call namespace.
- focused tests already cover exact visible model-string preservation and `spawn_agent` namespace rendering.

The remaining defect is contract enforcement. The system still relies too much on free-form prompt compliance for:

- whether a bounded manager loop must dispatch required roles at all;
- whether an exact requested worker model may silently degrade to some other model;
- whether skipped dispatch is surfaced explicitly instead of looking like normal completion;
- whether replay/forwarding paths preserve the `spawn_agent` namespace everywhere, not only in the currently tested formatting path.

The fix must stay inside the current multi-agent, prompt, and closeout owners. Do not add a second task runtime, planner daemon, sub-agent registry, or model router.

## Decision

Implement the full reliability package below inside existing owners only.

### R1. Manager Loop Role-Contract Enforcement

Bounded manager loops must record and enforce the required worker legs:

- `senior-reviewer`
- `implementation-worker`
- `verification-worker`

Required behavior:

- If the active loop instructions call for one or more required roles, final closeout must report one of:
  - dispatched successfully, with the role and effective model made explicit;
  - not dispatched, with the exact blocking reason;
  - intentionally skipped, only when the loop contract explicitly allowed that skip.
- A loop must not read like successful delegated execution if none of the required sub-agent legs actually spawned.
- If no implementation-ready task exists, keep the accepted closeout rule from `ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md`: answer with the exact reopen gate or `nothing left in scope`.

Owners:

- `ontocode-rs/protocol/src/prompts/base_instructions/default.md`
- existing closeout/operational-evidence owners already used by the chat-log closeout ADR

### R2. Strict Requested-Model Behavior

An exact requested child model must fail closed unless fallback was explicitly authorized.

Required behavior:

- Exact model ids remain exact. No aliasing, truncation, or "closest available" substitution.
- If the caller asked for one exact model and that model is unavailable, `spawn_agent` must fail with a bounded error.
- Fallback is allowed only when the request explicitly supplies `model = "fast"`.
- `model = "inherit"` remains the explicit inherited-model selector.
- Manager closeout must not claim a worker used a requested model unless that exact model was actually effective.

Owners:

- `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`
- existing `multi_agents*_tests.rs`

### R3. Role-Aware Runtime Selection

Move the bounded manager-loop role policy out of prose-only behavior and into a small runtime helper reused by the current spawn path.

Required behavior:

- The bounded manager-loop roles resolve through one shared role-to-model preference policy.
- The policy must use exact current model ids, not shortened aliases.
- The policy may reuse `fast`-style ordered preference behavior, but only through the existing model catalog and current spawn resolver.
- If no preferred model for the requested role is available, the system must surface that exact reason instead of silently dispatching a different role/model combination.
- The role policy must remain bounded and local to current sub-agent spawn/dispatch owners.

Owners:

- existing multi-agent spawn/model-selection owners in `ontocode-rs/core/src/tools/handlers/multi_agents*`
- prompt owner only for declaring the contract, not for being the sole enforcement layer

### R4. Namespace Hardening Across Replay And Forwarding

Keep the current namespace-preservation fix, but harden every remaining replay/forwarding path that can emit a `spawn_agent` function call.

Required behavior:

- Round-tripped `spawn_agent` function calls keep the original namespace.
- Validation must fail clearly when a forwarded/replayed call loses `namespace`.
- Regression coverage must include replay/round-trip paths, not only the current formatted-input restoration case.
- Request-shape failures must be reported as request-shape failures, not explained away as model-quality issues.

Owners:

- current function-call formatting/replay owner
- `ontocode-rs/core/src/client_common_tests.rs`
- existing sub-agent spec/request-shape regression owners

## Implementation Order

Implement in this order:

1. `R1` role-contract enforcement and closeout reporting
2. `R2` strict exact-model behavior
3. `R3` role-aware runtime selection
4. `R4` replay/forwarding namespace hardening

This order is deliberate:

- `R1` makes missing dispatch visible instead of ambiguous.
- `R2` removes silent model substitution.
- `R3` gives the loop one explicit runtime path for bounded role selection.
- `R4` closes the remaining malformed-request escape hatch.

## Acceptance Criteria

- A bounded manager loop that requests required worker roles cannot finish with implied delegated success unless those roles were actually dispatched or explicitly reported as blocked/skipped.
- Exact requested child models either run as requested or fail with a bounded error.
- `fast` remains the only fallback mechanism in the current `spawn_agent.model` API.
- Effective dispatched role/model information is explicit in closeout.
- Replayed or forwarded `spawn_agent` calls preserve `namespace` or fail clearly before execution.
- No-dispatch loops still end with the exact reopen gate or `nothing left in scope`.

## Non-Goals

- Do not add a new planner, queue, scheduler, or parallel task runtime.
- Do not add a second model registry, alias grammar, or provider parser.
- Do not add a second sub-agent registry or role-definition system.
- Do not widen app-server or config surface for this ADR.
- Do not replace the current `spawn_agent` owner with a donor runtime.

## Phase-Two Gate

Do not build a separate structured loop executor unless the slices above land and a fresh failure still shows that prompt plus current runtime owners cannot reliably enforce:

- required worker-role execution;
- blocked-vs-skipped-vs-dispatched reporting; or
- exact reopen-gate closeout.

Until that proof exists, a new loop executor is rejected as a second system.
