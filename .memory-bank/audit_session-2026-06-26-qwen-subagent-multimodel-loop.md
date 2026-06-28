# Qwen Sub-Agent Multi-Model Loop Closure

Superseded current-state note: see `audit_session-2026-06-26-qwen-subagent-qsm-k5-closure.md` for the post-`QSM-K5` queue state. `QSM-K3` is now blocked again and `QSM-K5` is closed.

Date: 2026-06-26

Source:
- `QWEN_SUBAGENT_MULTI_MODEL_IDEAS_REVIEW.md`
- `ADR_CUSTOM_SUBAGENT_MODELS.md`
- `ADR_TASK_COMPLETION_PING_PONG_GUARDRAILS.md`
- `ADR_CURRENT_SUB_AGENT_HANDLING.md`

## Scope

Run a bounded manager loop over retained rows `QSM-K1` through `QSM-K9` using current source plus OntoIndex impact/context evidence.

OntoIndex was stale against `HEAD`, so graph evidence stayed medium-confidence and source files were treated as authority.

## Senior Classification

- `QSM-K1`: closed
- `QSM-K2`: blocked
- `QSM-K3`: implementation-ready
- `QSM-K4`: blocked
- `QSM-K5`: implementation-ready
- `QSM-K6`: covered
- `QSM-K7`: covered
- `QSM-K8`: covered
- `QSM-K9`: covered

The sole active dispatch slice for this loop was `QSM-K1`.

## Implemented

`QSM-K1` is now closed in the existing custom-subagent-model owner:

- `spawn_agent.model = "inherit"` now behaves like omitting the model and preserves the parent model contract.
- `spawn_agent.model = "fast"` now resolves through the existing preferred worker-model order:
  `gemini-3.5-flash-low`, `gemini-3-flash-agent`, `gemini-pro-agent`, `claude-sonnet-4-6`, `gpt-5.3-codex-spark`, `gpt-5.4-mini`.
- If no preferred worker model is available, `spawn_agent` returns a bounded error instead of silently choosing an arbitrary model.
- Tool descriptions and suite expectations now document `inherit` and `fast`.

Implementation stayed inside:

- `ontocode-rs/core/src/tools/handlers/multi_agents_common.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs`
- existing spawn-agent test/suite files

No new registry, provider parser, config surface, or protocol shape was added.

## Verification

- OntoIndex impact before edit:
  - `apply_requested_spawn_agent_model_overrides`: LOW, direct callers are the v1/v2 spawn handlers
  - `find_spawn_agent_model_name` replacement path: LOW
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core spawn_agent`
- `git diff --check -- ontocode-rs/core/src/tools/handlers/multi_agents_common.rs ontocode-rs/core/src/tools/handlers/multi_agents_spec.rs ontocode-rs/core/src/tools/handlers/multi_agents_spec_tests.rs ontocode-rs/core/src/tools/handlers/multi_agents_tests.rs ontocode-rs/core/tests/suite/spawn_agent_description.rs`
- OntoIndex `gn_verify_diff` PASS on the exact five-file write set

## Reopen Gates

- `QSM-K2`: reopen only when provider catalog ids are canonical enough to add `provider:model` parsing without breaking exact ids.
- `QSM-K3`: reopen with a concrete proof that current spawn/runtime setup allocates avoidable extra provider/runtime state when the effective child target is unchanged.
- `QSM-K4`: reopen only after `R3` is unblocked and the existing role/config path proves it already owns default model-policy data.
- `QSM-K5`: reopen with a concrete telemetry gap scoped to existing session/thread/sub-agent telemetry owners and a no-protocol-widening implementation path.

## Notes

- Requested worker models were not fully usable on this host during the loop:
  - `gpt-5.3-codex-spark` rejected the active tool bundle
  - `gpt-5.4-mini` failed with local provider auth unavailability
- Manager completed `QSM-K1` locally after those environment failures.
