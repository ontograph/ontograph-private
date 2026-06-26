# ADR: Task Completion Ping-Pong Guardrails

## Status

Accepted

## Date

2026-06-26

## Context

Session `019efffd-3ba5-71e1-929a-c9ca64be45f2` showed a manager-loop ping-pong failure:

- A bounded loop had reached a no-dispatch closure.
- A later `<subagent_notification>` was recorded as a user-role contextual message.
- The turn emitted `task_complete` with `last_agent_message: null`.
- The next user had to say `continue`, even though the assistant had already produced visible progress/finalization text.

The root defect is not that subagent notifications exist in history. They are already part of the current sub-agent/context architecture and have tests around preservation. The narrow failure is that the shared task-completion event can lose visible assistant text when the task runner returns `None`.

## Decision

Fix ping-pong at the shared task completion boundary.

`Session::on_task_finished` must preserve the explicit task-returned `last_agent_message` when present. If it is `None`, it may recover the latest visible assistant message from history, but only from items appended after the current turn started.

During that recovery scan:

- `<subagent_notification>` user-role messages are treated as contextual tail data and do not stop recovery.
- Normal user messages still stop recovery, so completion never borrows an answer from an earlier user turn.
- The existing assistant-message extraction helper remains the source of truth for stripping hidden markup and plan-only text.

The turn-start history length is stored in `TurnState` as the boundary. This is the smallest root-cause fix because every task completion already routes through `Session::on_task_finished`.

## Manager Loop Rule

Manager loops must not convert blocked, proof-only, or design-only rows into stale "unblock options" after senior review closes them.

For tracked task loops, classify every opened item before dispatch as:

- `implementation-ready`
- `docs/design-only`
- `proof-only`
- `blocked`
- `closed`

If no `implementation-ready` task remains, close explicitly with `nothing left in scope` and do not ask the user to continue.

## Non-Goals

- Do not introduce a new subagent side-channel event system in this decision.
- Do not remove or rewrite current `<subagent_notification>` history preservation.
- Do not add a new task runtime, manager-loop service, or queue owner.
- Do not infer final answers from earlier turns.
- Do not turn prompt guardrails into a general planning framework.

## Implementation

Implemented owner-local changes:

- `ontocode-rs/core/src/state/turn.rs` records `history_items_at_turn_start`.
- `ontocode-rs/core/src/tasks/mod.rs` recovers current-turn visible assistant text for `TurnComplete.last_agent_message` only when the task returned `None`.
- `ontocode-rs/core/src/session/tests.rs` covers recovery across a subagent-notification tail and the no-cross-user-turn guard.
- `ontocode-rs/protocol/src/prompts/base_instructions/default.md` adds manager-loop classification and no-continue closeout rules.

## Verification

Focused verification from the implementation pass:

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core task_finish_recovers_last_agent_message_before_subagent_notification task_finish_does_not_recover_last_agent_message_across_user_turn task_finish_emits_turn_item_lifecycle_for_leftover_pending_user_input`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-protocol`
- `git diff --check -- ontocode-rs/core/src/tasks/mod.rs ontocode-rs/core/src/state/turn.rs ontocode-rs/core/src/session/tests.rs ontocode-rs/protocol/src/prompts/base_instructions/default.md`

OntoIndex impact for `Session::on_task_finished` and `Session::start_task` was LOW. `gn_verify_diff` was noisy because the checkout already had many unrelated dirty files, but it reported no missing tests for this slice.

## Consequences

The completion event is now more robust without changing the subagent notification architecture.

Future work should revisit subagent notifications as a true side-channel only if there is a separate product/runtime requirement. Until then, this ADR intentionally keeps the fix at the shared completion boundary.
