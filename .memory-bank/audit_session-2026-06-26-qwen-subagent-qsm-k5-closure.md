# Qwen Sub-Agent QSM-K5 Closure

Date: 2026-06-26

Source:
- `QWEN_SUBAGENT_MULTI_MODEL_IDEAS_REVIEW.md`
- `audit_session-2026-06-26-qwen-subagent-multimodel-loop.md`
- current source in `ontocode-rs/core` and `ontocode-rs/analytics`

## Scope

Close `QSM-K5` as the active next task from the bounded Qwen sub-agent multi-model loop.

Keep the implementation inside existing owners:

- `ontocode-rs/core/src/agent/control.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents*/spawn.rs`
- `ontocode-rs/analytics/src/*`

Do not widen the app-server or collaboration protocol.

## Gap Proven

Current source already emitted sub-agent start analytics via `emit_subagent_session_started(...)`, but it did not emit terminal sub-agent outcome telemetry after the completion watcher observed a final child status.

That left `QSM-K5` open because the retained donor requirement was explicit outcome telemetry for:

- requested model
- effective model
- invocation kind
- depth
- terminal status
- terminate reason
- duration
- result-summary presence

## Implemented

Added one new analytics-only completion fact and event family:

- `SubAgentThreadCompletedInput`
- `SubAgentInvocationKind`
- `SubAgentTerminalStatus`
- `codex_subagent_thread_completed`

The completion watcher now emits that fact when a thread-spawn child reaches a final status and inherited client metadata is available.

The watcher records:

- requested model from the existing spawn tool args when explicitly provided
- effective model from the child thread config snapshot
- invocation kind as `spawn` or `resume`
- thread-spawn depth from the existing session source
- terminal status and terminate reason
- elapsed watcher duration in milliseconds
- whether the completed status carried a non-empty final summary

This stayed owner-local:

- no new registry
- no new runtime state store
- no protocol widening
- no changes to `emit_subagent_session_started(...)`

## OntoIndex Impact

Fresh impact before edit:

- `maybe_start_completion_watcher`: LOW
- `handle_spawn_agent` in both v1 and v2: LOW
- `track_subagent_thread_started`: LOW
- `emit_subagent_session_started`: HIGH, so it was intentionally left unchanged
- `spawn_agent_internal`: HIGH
- `resume_single_agent_from_rollout`: HIGH

The two HIGH-risk control functions were limited to metadata pass-through only.

## Verification

Commands run:

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-analytics`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core`
- `git diff --check -- ontocode-rs/analytics/src/facts.rs ontocode-rs/analytics/src/events.rs ontocode-rs/analytics/src/client.rs ontocode-rs/analytics/src/lib.rs ontocode-rs/analytics/src/reducer.rs ontocode-rs/analytics/src/analytics_client_tests.rs ontocode-rs/core/src/agent/control.rs ontocode-rs/core/src/agent/control_tests.rs ontocode-rs/core/src/tools/handlers/multi_agents/spawn.rs ontocode-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs`
- OntoIndex `gn_verify_diff` PASS on the exact ten-file write set

Targeted evidence:

- `ontocode-analytics` passed, including new completion-event tests.
- Relevant `ontocode-core` agent-control tests passed inside the crate run:
  - `multi_agent_v2_completion_queues_message_for_direct_parent`
  - `completion_watcher_notifies_parent_when_child_is_missing`
  - `spawn_child_completion_notifies_parent_history`

Out-of-scope failures still exist in the full `ontocode-core` project test slice. They are unrelated truncation/config assertions outside this write set:

- `config::config_loader_tests::default_ontocode_home_profile_falls_back_to_legacy_codex_home`
- `config::tests::catalog_v2_allows_agents_max_threads_when_feature_disabled`
- `config::tests::multi_agent_v2_feature_rejects_agents_max_threads`
- several `shell_serialization`, `truncation`, and `user_shell_cmd` expectations around output summarization/truncation formatting

## Current Queue State

- `QSM-K1`: closed
- `QSM-K2`: blocked
- `QSM-K3`: blocked
- `QSM-K4`: blocked
- `QSM-K5`: closed
- `QSM-K6`: covered
- `QSM-K7`: covered
- `QSM-K8`: covered
- `QSM-K9`: covered

There is no remaining implementation-ready task in this queue.

## Exact Reopen Gate

If the user says `continue` after this note, the correct no-dispatch reply is:

- `QSM-K2` reopens only when provider catalog ids are canonical enough to add `provider:model` parsing without breaking exact ids.
- `QSM-K3` reopens only with concrete proof that the current spawn/runtime path allocates avoidable extra provider or runtime state when the effective child target is unchanged.
- `QSM-K4` reopens only after `R3` is unblocked and the existing role/config path proves it already owns default child-model policy data.

Without one of those new evidence sources, do not rewrite the tracking state again.
