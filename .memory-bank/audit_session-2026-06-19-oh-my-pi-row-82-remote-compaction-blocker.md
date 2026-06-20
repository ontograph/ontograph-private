# Oh My Pi Row 82 Remote Compaction Blocker

Date: 2026-06-19

Row 82 remains blocked for the remote compaction/session overflow fixture slice.

Evidence:

- `CARGO_BUILD_JOBS=8 just test -p ontocode-core manual_compact_retries_after_context_window_error` passes and proves the local context-window retry fixture.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core snapshot_request_shape_remote_pre_turn_compaction_context_window_exceeded` fails because the expected remote compact request count is `0`, not `1`.
- Neighboring remote pre-turn compaction snapshots `snapshot_request_shape_remote_pre_turn_compaction_including_incoming_user_message` and `snapshot_request_shape_remote_pre_turn_compaction_strips_incoming_model_switch` fail the same way, indicating a broader existing remote pre-turn fixture/runtime mismatch.

Decision:

- Do not ignore or weaken the failing remote snapshot.
- Do not change remote compaction runtime behavior under this row.
- Continue later rows with row 82 recorded as blocked on the existing remote pre-turn compaction request path.
