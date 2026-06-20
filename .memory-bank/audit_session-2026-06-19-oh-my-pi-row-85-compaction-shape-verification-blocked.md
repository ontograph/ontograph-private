# Oh My Pi Row 85 Compaction Shape Verification Blocked

Date: 2026-06-19

Row: 85

Status: blocked

Summary:

- Local compaction request-shape snapshots already pass in `ontocode-rs/core/tests/suite/compact.rs`.
- Remote request-shape verification still fails because the remote compact request is never emitted in the targeted cases, returning zero remote compact requests.
- The row 82 remote pre-turn blocker remains out of scope for this pass.

Focused tests:

- Passed: `snapshot_request_shape_mid_turn_continuation_compaction`, `snapshot_request_shape_pre_turn_compaction_including_incoming_user_message`, `snapshot_request_shape_pre_turn_compaction_strips_incoming_model_switch`, `snapshot_request_shape_pre_turn_compaction_context_window_exceeded`, `snapshot_request_shape_manual_compact_without_previous_user_messages`
- Blocked: `snapshot_request_shape_remote_manual_compact_restates_realtime_start`, `snapshot_request_shape_remote_mid_turn_compaction_does_not_restate_realtime_end`, `snapshot_request_shape_remote_compact_resume_restates_realtime_end`, `snapshot_request_shape_remote_mid_turn_compaction_summary_only_reinjects_context`, `snapshot_request_shape_remote_mid_turn_compaction_multi_summary_reinjects_above_last_summary`, `snapshot_request_shape_remote_manual_compact_without_previous_user_messages`, `remote_manual_compact_api_auth_omits_service_tier_and_reuses_prompt_cache_key`, `remote_manual_compact_chatgpt_auth_reuses_service_tier_and_prompt_cache_key`

Notes:

- Removed the generated `.snap.new` artifact after the failed remote snapshot run.
- No Rust source or snapshot content was changed in this pass.
