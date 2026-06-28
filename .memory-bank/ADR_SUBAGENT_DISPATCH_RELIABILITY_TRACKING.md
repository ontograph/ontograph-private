---
name: Sub-Agent Dispatch Reliability Tracking
description: Dispatch and verification ledger for ADR_SUBAGENT_DISPATCH_RELIABILITY
type: tracking
date: 2026-06-28
status: active
---

# Sub-Agent Dispatch Reliability Tracking

Authority:
- `ADR_SUBAGENT_DISPATCH_RELIABILITY.md`
- `ADR_CHAT_LOG_RECOMMENDATION_CLOSEOUT_AND_PROOF_ROUTING.md`
- `ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md`

## Manager Rules

- Update this file before starting each slice.
- Use OntoIndex before edits and record the impact result for touched production symbols.
- Keep changes inside the existing prompt, operational-evidence, and multi-agent owners.
- Do not add a second planner, queue, scheduler, role registry, or model router.
- If no `implementation-ready` task remains, close with the exact reopen gate or `nothing left in scope`.

## Senior Classification

| Slice | Status | Classification | Owner / Write Scope | Verification |
| --- | --- | --- | --- | --- |
| `SDR-R1` | closed | prompt-only | `ontocode-rs/protocol/src/prompts/base_instructions/default.md` | `CARGO_BUILD_JOBS=8 just test -p ontocode-protocol`; `CARGO_BUILD_JOBS=8 just fmt` |
| `SDR-R2` | closed | covered-by-current-source after ADR correction | existing spawn-model owners only | Existing `spawn_agent` resolver and tests already fail closed for exact ids and `fast` fallback; the ADR no longer claims an unimplemented ordered fallback-list API. |
| `SDR-R3` | closed | existing role-config seam unblocked | `.codex/agents/*`; existing role-spec tests | Requested manager-loop role names now exist as repo-local agent definitions and are covered by the role-spec rendering test. No second role-policy surface was added. |
| `SDR-R4` | pending | narrowed to request-builder inventory | current function-call formatting/replay owners only | Next step: add a failing test only if a replay/forwarding path outside current request builders bypasses `Prompt::get_formatted_input()`. |

## Event Log

- 2026-06-28: Tracking opened from fresh OntoIndex and direct-source review. OntoIndex is fresh at `5edde24a78efe0f10bc710936dfa228427ab7fd1` with dirty-worktree medium confidence. `evaluate_operational_evidence_task_closure` impact is MEDIUM through its direct runtime tests. `resolve_requested_spawn_agent_model` impact is LOW and current source already shows exact-id fail-closed behavior plus `inherit` and `fast`.
- 2026-06-28: Senior classification result: `SDR-R1` is the only active implementation-ready slice. `SDR-R2` is already covered in current source. `SDR-R3` and `SDR-R4` remain pending follow-ups and must not be reopened until `SDR-R1` lands and current source still shows a concrete gap.
- 2026-06-28: `SDR-R1` closed as a prompt-only first slice. The shared manager-loop prompt now requires per-role closeout reporting for required roles, forbids implied delegated execution when no worker spawned, and requires explicit unavailable-model reporting instead of silent fallback claims. Focused verification passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-protocol` and `CARGO_BUILD_JOBS=8 just fmt`. This verifies prompt integrity, not end-to-end role-closeout behavior.
- 2026-06-28: Senior re-review challenged the earlier closure wording. `SDR-R1` remains useful but is now recorded as prompt-only rather than full behavioral closure. `SDR-R3` and `SDR-R4` move from `blocked` to `pending` because the current evidence does not justify claiming they are dead ends; they are follow-up slices gated on narrower source audits and failing-test proof rather than broad redesign.
- 2026-06-28: Continued `SDR-R3` as the next pending slice using OntoIndex plus direct source reads. Current runtime already has a live role seam: `agent_type`/`agent_role` is carried on `SessionSource::SubAgent`, role files load through `config/agent_roles.rs`, and existing suite coverage proves a role can override the child model and reasoning settings. No implementation-ready code slice was opened because there is still no current structured manager-loop caller that emits those role names automatically; adding one now would risk inventing a second role-policy surface.
- 2026-06-28: Continued `SDR-R4` as a source inventory pass. Current request builders in `client.rs` and the Anthropic, Copilot, and Gemini native providers all route outgoing input through `Prompt::get_formatted_input()`, and the existing regression still proves namespace restoration for the unambiguous case. No implementation-ready code slice was opened because this pass did not find a replay/forwarding path that demonstrably bypasses that formatting step.
- 2026-06-28: Continued the unblock pass with OntoIndex fresh at `5edde24a78efe0f10bc710936dfa228427ab7fd1` and dirty-worktree medium confidence. `create_spawn_agent_tool_v1` and `create_spawn_agent_tool_v2` impact are MEDIUM through tool-spec tests; `apply_role_to_config` impact is HIGH, so no role-loader code was changed. Added repo-local role definitions for `senior-reviewer`, `implementation-worker`, and `verification-worker`, plus a focused `spawn_tool_spec_lists_manager_loop_roles` regression. Corrected `SDR-R2` ADR wording to remove ordered fallback-list claims because current `spawn_agent.model` supports only exact ids, `inherit`, and `fast`.
- 2026-06-28: Closed the session-019f08c4 prompt drift fix as `SDR-R1`/`SDR-R3` follow-through. The shared manager-loop prompt now requires exact `spawn_agent` arguments for bound roles, separate attempts for ordered fallback models, and fail-closed reporting instead of substituting generic `explorer`/`worker` roles or unordered model lists. Existing repo-local role files and `spawn_tool_spec_lists_manager_loop_roles` cover role visibility; no scheduler, router, or second role registry was added.
