# Ontocode Chat Log Recommendation Implementation

status: implemented-narrow
date: 2026-06-27
scope: recommendation-layer follow-ups from chat-log review and sub-agent donor lefties

## Goal

Implement only the recommendation behavior that still adds value after the accepted ping-pong guardrails and current sub-agent closures.

Do not open a second planner, queue service, donor registry, or model-routing stack.

## Current Authority

- [ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md](ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md) already fixes the shared completion-boundary ping-pong and already requires `nothing left in scope` when no implementation-ready task remains.
- [audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md](audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md) already records the exact reopen-gate closeout pattern for the Qwen sub-agent loop family.
- [HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md](HARNESSX_SUBAGENT_MULTI_MODEL_2000_IDEAS_REVIEW.md), [OPENCLAW_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md](OPENCLAW_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md), and [CLAUDE_CODE_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md](CLAUDE_CODE_SUBAGENT_MULTI_MODEL_100_IDEAS_REVIEW.md) already say that nothing is implementation-ready without exact reopen evidence.

## Implementation Status

- `REC-R1`: already covered by the accepted prompt/closeout guardrail. No new code needed.
- `REC-R2`: implemented by adding top-of-file current-state pointers and explicit no-live-queue guidance to the three donor left-over files.
- `REC-R3`: already satisfied for the current sub-agent family by exact reopen-gate closure text; no additional generic backlog rewrite was added.
- `REC-R4`: intentionally not reopened. Current repo memory shows the rule and past fix pattern, but there is no new live reproduction to justify code changes now.

## Already Covered

- Shared task-completion ping-pong fix at the current-turn completion boundary.
- Manager-loop classification rule: `implementation-ready`, `docs/design-only`, `proof-only`, `blocked`, `closed`.
- No-dispatch closeout rule when no implementation-ready task remains.
- Existing owner shape for child model overrides, role/provider inheritance, and bounded model help text.
- Existing guidance that exact model ids and preserved `namespace` matter more than vague "model quality" explanations.

These should not be reopened as new implementation work unless current source regresses.

## Keep Only: Remaining Recommendation Slices

| Id | Recommendation | Owner | Smallest valid implementation | Reopen gate |
| --- | --- | --- | --- | --- |
| `REC-R1` | If a queue is already at `no-dispatch`, answer with the exact reopen gate immediately instead of asking the user to say `continue`. | existing prompt/tracking guardrail owners | Tighten prompt/tracker wording only where current surfaces still allow a stale `continue` handoff. | Reopen only if current prompt or task-completion output still emits a `continue` handoff after a no-dispatch closeout. |
| `REC-R2` | When a donor review file is not the live queue, point to the current closure artifact or tracker first. | memory-bank tracking owners | Add top-of-file current-state pointers in donor review artifacts that still read like live backlogs. | Reopen only if a donor review file still causes queue-state drift in a fresh review loop. |
| `REC-R3` | For evidence-gated rows, recommend the smallest missing proof artifact instead of restating the whole backlog. | existing tracking/closure-note owners | Standardize reopen text around one missing artifact: failing test, concrete UX transcript, canonical provider-id proof, role/config ownership proof, or runtime-overhead proof. | Reopen only if current closure notes still give vague "blocked" answers instead of one concrete missing proof. |
| `REC-R4` | Before blaming model choice, validate request shape: exact model id and preserved `namespace`. | existing sub-agent dispatch/request-shape owners | Add or tighten narrow validation where current forwarding/replay still accepts truncated model ids or loses `namespace`. | Reopen only with a current reproduction of alias fallback or dropped-namespace failure. |

## Lefties From The Three Donor Files

Nothing in the three donor challenge files is implementation-ready now.

- HarnessX lefties are still evidence-gated:
  - role/config child-default ownership
  - unchanged-target runtime-overhead proof
  - failing coverage proof
  - concrete requested-versus-effective output ambiguity
- OpenClaw lefties are still evidence-gated:
  - canonical provider-qualified ids
  - role/config child-default ownership
  - failing inheritance-path coverage
  - concrete selector-help ambiguity
  - concrete configured-versus-effective precedence confusion
- Claude lefties are still evidence-gated:
  - trustworthy family/tier metadata
  - canonical provider-qualified ids
  - role/config child-default ownership
  - failing inheritance-path coverage
  - concrete selector-help ambiguity
  - proven `fast` fallback failure

The correct recommendation behavior for those files is not "pick another row." It is "name the one missing proof that would reopen exactly one row."

## Do Not Implement

- No second planner or manager-loop runtime.
- No second model registry or provider parser.
- No donor-specific runtime alias, worker factory, or parallel orchestration layer.
- No broad protocol/API widening just to surface recommendation metadata.

## Recommended Order

1. Recheck current prompt/task-completion output against `REC-R1`.
2. If no stale `continue` handoff remains, do not write code.
3. Apply `REC-R2` only to donor files that still read like live queues.
4. Apply `REC-R3` only where a closure artifact still answers with generic blocked text.
5. Reopen `REC-R4` only on a current reproduction, not on historical memory alone.

## Stop Condition

If current source and current memory-bank artifacts already satisfy `REC-R1` through `REC-R4`, close with `nothing left in scope`.
