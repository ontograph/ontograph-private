# Qwen Sub-Agent Multi-Model Ideas Review

Date: 2026-06-26

Scope: review `tmp/qwen-code` via OntoIndex and extract only useful ideas for how a code agent should route sub-agents across different models/providers without importing Qwen's runtime wholesale.

Status: review complete. This file is donor evidence only; it does not approve implementation by itself.
Current queue state moved to [audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md](audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md), which closes `QSM-K5`, blocks `QSM-K2`/`QSM-K3`/`QSM-K4`, and leaves no implementation-ready task in this queue.

## Donor Surfaces Reviewed

- [packages/core/src/subagents/subagent-manager.ts](../tmp/qwen-code/packages/core/src/subagents/subagent-manager.ts:770)
- [packages/core/src/tools/agent/agent.ts](../tmp/qwen-code/packages/core/src/tools/agent/agent.ts:631)
- [packages/core/src/tools/agent/fork-subagent.ts](../tmp/qwen-code/packages/core/src/tools/agent/fork-subagent.ts:11)
- [packages/core/src/subagents/builtin-agents.ts](../tmp/qwen-code/packages/core/src/subagents/builtin-agents.ts:53)
- [packages/core/src/utils/modelId.ts](../tmp/qwen-code/packages/core/src/utils/modelId.ts:49)
- [packages/core/src/subagents/subagent-manager.test.ts](../tmp/qwen-code/packages/core/src/subagents/subagent-manager.test.ts:1793)
- [packages/core/src/utils/modelId.test.ts](../tmp/qwen-code/packages/core/src/utils/modelId.test.ts:40)
- [docs/users/features/sub-agents.md](../tmp/qwen-code/docs/users/features/sub-agents.md:150)
- [docs/design/fork-subagent/fork-subagent-design.md](../tmp/qwen-code/docs/design/fork-subagent/fork-subagent-design.md:7)
- [docs/design/telemetry-subagent-spans-design.md](../tmp/qwen-code/docs/design/telemetry-subagent-spans-design.md:155)
- [packages/core/src/telemetry/uiTelemetry.test.ts](../tmp/qwen-code/packages/core/src/telemetry/uiTelemetry.test.ts:404)
- [packages/core/src/agents/arena/ArenaManager.ts](../tmp/qwen-code/packages/core/src/agents/arena/ArenaManager.ts:906)

## Evidence Caveat

One donor design doc is stale: current code and user docs require explicit `subagent_type: "fork"`, while the older fork design doc still describes implicit fork behavior when the field is omitted. Use that design doc for cache-sharing ideas only, not current behavior. See [fork-subagent.ts](../tmp/qwen-code/packages/core/src/tools/agent/fork-subagent.ts:11), [sub-agents.md](../tmp/qwen-code/docs/users/features/sub-agents.md:196), and [fork-subagent-design.md](../tmp/qwen-code/docs/design/fork-subagent/fork-subagent-design.md:7).

## OntoIndex Challenge

OntoIndex for repo `codex` is stale against the current `HEAD`, so this challenge uses medium-confidence graph evidence cross-checked against current ADRs and source files. The current owner map is still clear enough:

- `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs` already owns tool exposure and schema for `spawn_agent`, `send_message`, `followup_task`, `resume_agent`, `wait_agent`, `list_agents`, and `close_agent`.
- `ontocode-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs` and the shared multi-agent helpers already own child spawn/runtime inheritance.
- `ontocode-rs/state/src/runtime/agent_jobs.rs` already owns persisted background/job state.
- `.memory-bank/ADR_CURRENT_SUB_AGENT_HANDLING.md` and `.memory-bank/ADR_CURRENT_SUB_AGENT_HANDLING_TRACKING.md` already closed the current read-only activity UI, bounded progress rendering, and agent-job export/status slices.
- `.memory-bank/ADR_CUSTOM_SUBAGENT_MODELS.md` already accepts exact-id child `model`, `reasoning_effort`, and `service_tier` overrides through the existing model catalog.

Challenge result: most Qwen ideas are either already covered in Ontocode or they would introduce a parallel daemon/session/subagent runtime. Keep only the ideas below.

## Keep Only: New Core Extensions

| Id | Donor evidence | Keep | Current Ontocode home | Challenge |
| --- | --- | --- | --- | --- |
| `QSM-K1` | [modelId.ts](../tmp/qwen-code/packages/core/src/utils/modelId.ts:49), [sub-agents.md](../tmp/qwen-code/docs/users/features/sub-agents.md:150) | Add symbolic child-model selectors on top of the existing `spawn_agent.model` field: `inherit` and `fast`. | `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`, `multi_agents_v2/spawn.rs`, existing model-catalog validation path | Keep narrow. Extend the current exact-id override contract; do not add a second model registry or free-form aliases. |
| `QSM-K2` | [modelId.ts](../tmp/qwen-code/packages/core/src/utils/modelId.ts:55), [modelId.test.ts](../tmp/qwen-code/packages/core/src/utils/modelId.test.ts:40) | Add a provider-qualified selector contract such as `provider:model` only after provider catalog ids are canonical. | existing sub-agent model override path plus the existing provider/model catalog owners | Keep as deferred core extension, not immediate implementation. Parser rules matter: reject malformed `provider:` input and do not break colon-bearing exact ids. |
| `QSM-K3` | [subagent-manager.ts](../tmp/qwen-code/packages/core/src/subagents/subagent-manager.ts:770), [subagent-manager.test.ts](../tmp/qwen-code/packages/core/src/subagents/subagent-manager.test.ts:1982) | Reuse the parent child-runtime path when no effective model/provider change occurs; allocate extra provider/runtime state only when an override actually changes the effective child target. | `multi_agents_common.rs`, `multi_agents_v2/spawn.rs`, `ThreadManager`, model-provider integration | Keep as an efficiency rule inside the current spawn/runtime owner. Do not fork a second sub-agent runtime. |
| `QSM-K4` | [builtin-agents.ts](../tmp/qwen-code/packages/core/src/subagents/builtin-agents.ts:53) | Allow existing sub-agent role/config owners to specify default model policy such as `fast` for selected roles. | existing role/config loading path plus `multi_agents_spec.rs` model guidance | Keep only when the current role/config path already owns that source data. Do not reopen blocked `R3` with a new registry or source-precedence layer. |
| `QSM-K5` | [telemetry-subagent-spans-design.md](../tmp/qwen-code/docs/design/telemetry-subagent-spans-design.md:155), [uiTelemetry.test.ts](../tmp/qwen-code/packages/core/src/telemetry/uiTelemetry.test.ts:404) | Add explicit sub-agent outcome telemetry: requested model, effective model, invocation kind, depth, terminal status, terminate reason, duration, and result-summary presence. | existing session/thread/sub-agent telemetry owners | Keep. This is new operational evidence and extends the existing core engine without changing user-facing protocol shape. |
| `QSM-K6` | [agent.ts](../tmp/qwen-code/packages/core/src/tools/agent/agent.ts:667), [fork-subagent-design.md](../tmp/qwen-code/docs/design/fork-subagent/fork-subagent-design.md:45) | Preserve a strict fork contract: full-history forks inherit parent identity/model/reasoning and must not accept child-model overrides. | already-ownered fork/spawn validation path | Keep only as reaffirmed guardrail. Ontocode already does this; treat it as a non-regression requirement, not a new feature. |
| `QSM-K7` | session review of thread `019efffd-3ba5-71e1-929a-c9ca64be45f2` (last-20-minute ping-pong pattern) | Add a manager-loop no-replan rule: once a queue has an `active_next_task` or `no-dispatch` result, a plain `continue` must either execute that next task or answer that nothing changed in scope. | existing manager-loop tracking files plus current sub-agent handling / ping-pong guardrail owners | Keep as a control-plane extension. This prevents sub-agent manager loops from re-ranking the same queue and producing new notes with no new evidence. |
| `QSM-K8` | session review of thread `019efffd-3ba5-71e1-929a-c9ca64be45f2` (repeated reopen/verify/note cycle) | Add a reopen threshold: blocked or no-dispatch rows can reopen only with new code evidence, a new failing test/fixture, explicit ADR acceptance, or explicit user scope change. | existing tracking/ADR owners, current sub-agent handling, and `ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md` | Keep. This is the smallest rule that stops manager/worker ping-pong without introducing a new scheduler or state store. |
| `QSM-K9` | session review of thread `019efffd-3ba5-71e1-929a-c9ca64be45f2` (docs churn on equivalent conclusions) | Add dedupe and docs-churn caps: if the conclusion is semantically unchanged, do not emit another audit note plus index update; answer briefly and stop. | current memory-bank/tracking owners and bounded manager-loop discipline | Keep as workflow hygiene that protects the core engine from fake progress loops and tracker spam. |

## Covered By Existing Ontocode Owners

These donor families are useful as confirmation only. They are not new core functionality here.

- Basic spawn validation, model/reasoning/service-tier fields, and bounded model guidance are already owned by `multi_agents_spec.rs` and the accepted custom-subagent-models ADR.
- Read-only activity UI, status/last-task display, deterministic role color, bounded progress rendering, and capped job export/status output are already closed under `ADR_CURRENT_SUB_AGENT_HANDLING_TRACKING.md` slices `R1`, `R2`, `R4`, and `R5`.
- Existing collaboration tool surfaces already own `spawn_agent`, `send_message`, `followup_task`, `resume_agent`, `wait_agent`, `list_agents`, and `close_agent`; the donor file should not restate those basics as new work.
- Shared-completion and manager-loop closeout rules already have an owner in `.memory-bank/ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md`; new ping-pong prevention should extend that accepted guardrail, not create a second loop-policy surface.

## Drop: Not New Or Wrong Owner

These donor families should not stay in the active keep list.

- Qwen daemon/session protocol ideas such as FIFO prompt queues, SSE replay rings, heartbeat, restore/load conflicts, recap routes, and session-close semantics. Those belong to Qwen's daemon runtime, not Ontocode's current sub-agent engine.
- Permission-mode and tool-allowlist runtime surfaces that would create a parallel sub-agent permission engine instead of extending the existing permission/profile owners.
- Nested daemon transcript tree mechanics tied to Qwen's `parentToolCallId`/`subagentType` reducer model. Reopen only if Ontocode's existing event/protocol owners prove a concrete missing lineage field.
- Arena-style competitive multi-model execution. That is a second orchestration product, not a narrow extension of current sub-agent handling.
- Built-in agent directory/registry/source-precedence ideas that would bypass the currently blocked `R3` constraints.

## Recommended Reuse Shape

The only durable Qwen carry-over is:

- a small child-model selector grammar over the existing `spawn_agent.model` field
- provider-aware parsing only when the provider catalog is canonical
- runtime reuse when the child effectively inherits the parent model/provider
- non-regression guardrails for full-history forks
- explicit sub-agent outcome telemetry inside the current core owners
- manager-loop no-replan, reopen-threshold, and dedupe rules so sub-agent queues do not bounce between the same blocked/no-dispatch outcomes

If this donor is reused later, extend the current Ontocode sub-agent owners named above. Do not add a second daemon, second task runtime, second permission engine, second agent registry, or second model router.
