name: Oh My Pi Donor Keep Refactor Map Pre-Junior Project Plan
desc: Junior-safe, test-first first slice from ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md
type: project_plan
date: 2026-06-16
status: challenged

# Oh My Pi Donor Keep Refactor Map Pre-Junior Project Plan

## Goal

Implement only the first low-risk, test-first slice from
[ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md](ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md).

This is not a 127-item work order. Each stage must add or extend the smallest
test in the existing owner. If existing coverage already proves the behavior,
record that and do not change code.

## Challenge Review

- Do not dispatch a full stage to one pre-junior worker. Dispatch one ADR row, one owner, and one failing test at a time.
- Package names verified from `Cargo.toml`: `ontocode-core`, `ontocode-mcp`, `ontocode-hooks`, `ontocode-state`, and `ontocode-apply-patch`.
- `ontocode-rs/core/src/context/` exists; `ontocode-rs/core/src/ctx/` does not.
- Hot owner files are already large. Add or extend sibling tests first, and extract a helper only when the helper is reused by the same patch.
- Schema-cycle, redaction, cancellation, and compaction work must stay test-first. If a test cannot be expressed with existing owner APIs, stop for senior review.
- Claude Code donor rows parked as `NARROW`, `DEFER`, or `REJECT` are not pre-junior scope. This plan may keep only the narrower Oh My Pi accepted test rows and must not reopen the broader Claude MCP, hook, context/cache, command/debug, UI, release, eval, plugin, or agent-runtime ideas.

## Claude Code Consolidation Review

Do not dispatch these duplicate or broader Claude Code parked areas from this Oh My Pi pre-junior plan:

- MCP/resource/debug overlap: Claude rows 122, 123, 128-130, and 145-147.
- Context/compaction/prompt-cache overlap: Claude rows 073, 084, 089, 090, 094, 095, and 187.
- Hook overlap: Claude rows 097 and 101-104.
- Agent/job/session overlap: Claude rows 057-059 and 148-150.
- UI/release/eval/plugin overlap: Claude rows 161-180 and 181-200.

The retained Oh My Pi rows below are allowed only as test-first hardening of existing owners. If an implementation needs new runtime state, new prompt-cache behavior, a new MCP browser/debugger, a second hook registry, a new eval framework, or agent protocol/persistence changes, stop and move it back to the relevant ADR.

## Approved First Slice

1. MCP lifecycle hardening: ADR rows 121, 122, 124, 127, and 130.
2. Apply-patch safety: ADR rows 23, 26, 27, and 31-40.
3. Context cap and compaction safety: ADR rows 81-83, 85, 86, and 88-90.
4. Hook load, trust, and redaction: ADR rows 103, 104, and 111-120.
5. Agent job cleanup and structured results: ADR rows 141, 144-146, 148, and 150.

Row 128 is conditional. Do not implement MCP reconnect/backoff tests unless the
current MCP manager already exposes retry/backoff state.

## Non-Goals

- Do not import Oh My Pi code.
- Do not add public API, config, SDK, schema, persisted state, or new context fragments.
- Do not add DAP, browser control, notebook execution, virtual URI schemes, or a persistent language worker.
- Do not create a second MCP lifecycle, hook matcher, provider registry, memory service, tool runtime, shell runtime, or fixture framework.
- Do not grow hot files casually: `hooks/src/engine/discovery.rs`, `codex-mcp/src/connection_manager.rs`, `tui/src/app/session_lifecycle.rs`, and `core/src/tools/handlers/agent_jobs.rs`.

## Stage 0: Preflight

Read:

- [ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md](ADR_OH_MY_PI_DONOR_KEEP_REFACTOR_MAP.md)
- [OH_MY_PI_DONOR_200_SOLUTIONS_NON_KEEP.md](OH_MY_PI_DONOR_200_SOLUTIONS_NON_KEEP.md)
- [ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md](ADR_CLAUDE_CODE_DONOR_CORE_EXTENSION_REVIEW_DEFERRED_NARROW_REJECT.md)
- `ontocode-rs/core/tests/common/responses.rs`
- `ontocode-rs/core/tests/common/test_codex.rs`

Checks:

- Run OntoIndex context or impact on the exact owner symbol before editing Rust.
- Record the chosen owner file in the task note.
- Pick exactly one ADR row from the stage, not the whole stage.
- Stop if the task needs a new runtime concept.

Acceptance:

- Owner file is named.
- ADR row is named.
- No code changed in Stage 0.

## Stage 1: MCP Lifecycle Hardening

Owner:

- `ontocode-rs/codex-mcp/src/connection_manager.rs`
- `ontocode-rs/core/src/mcp_tool_call_tests.rs`
- Existing RMCP/MCP fixture helpers

Task:

- Add tests for partial server failure, unified MCP error mapping, MCP/OAuth redaction, and schema cycle guards.
- Prefer test-only fixture servers.
- Do not add new MCP lifecycle state unless a failing test proves the current owner cannot express the case.
- Do not implement Claude parked MCP rows 122, 123, 128-130, or 145-147 here. No MCP source browser, teaching server, command debugger, explorer UI, or new metadata surface.
- Pre-junior dispatch limit: choose one of rows 121, 122, 124, 127, or 130 per patch.

Acceptance:

- One MCP server can fail without hiding healthy servers.
- Redacted diagnostics never expose tokens or OAuth material.
- Tool schema cycles fail safely.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

If touching `codex-mcp/` directly, run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-mcp
```

## Stage 2: Apply-Patch Safety

Owner:

- `ontocode-rs/apply-patch/`
- `ontocode-rs/core/tests/suite/apply_patch_cli.rs`
- `ontocode-rs/core/tests/suite/shell_serialization.rs`
- `ontocode-rs/core/tests/common/responses.rs`

Task:

- Add tests for stale-edit recovery, ambiguous line fallback, malformed patch rejection, create/update/move/delete failures, no-write parse failure, large output truncation, and exact changed-file verification.
- Reuse existing apply-patch harnesses and response helpers.
- Pre-junior dispatch limit: choose one row or one tightly coupled parser failure group per patch.

Acceptance:

- Malformed patches do not write files.
- Failure output is bounded and model-visible only where intended.
- Move/delete edge cases are deterministic.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

If touching `apply-patch/` directly, run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-apply-patch
```

## Stage 3: Context And Compaction Safety

Owner:

- `ontocode-rs/core/src/session/turn.rs`
- `ontocode-rs/core/src/compact.rs`
- `ontocode-rs/core/src/session/turn_context.rs`
- `ontocode-rs/core/tests/suite/compact.rs`
- `ontocode-rs/core/tests/suite/compact_remote.rs`

Task:

- Add tests for overflow retry, same-model retry loop guard, compaction request shape stability, bounded compaction failure events, hard context caps, existing prompt-cache stability, and reinjection after summary.
- Prefer existing compact snapshot tests.
- Do not change compaction behavior unless the failing test proves a real gap.
- Do not implement Claude parked context/cache rows 073, 084, 089, 090, 094, 095, or 187 here. No new context fragment, diagnostics-only context mutation, speculative cache, or golden-prompt/eval asset.
- Pre-junior dispatch limit: choose one compaction behavior per patch.

Acceptance:

- Tests do not require a live provider.
- Context items stay capped.
- Compaction failures are bounded and distinguishable.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

## Stage 4: Hook Load, Trust, And Redaction

Owner:

- `ontocode-rs/hooks/`
- `ontocode-rs/core/src/hook_runtime.rs`
- Existing hook tests and schema fixtures

Task:

- Add tests for invalid regex warning, glob-gated selectors, partial hook load errors, startup with one bad hook and one good hook, hook schema fixtures, hook timeout, hook order, output caps, trust prompts, config layer merge, and secret redaction.
- Prefer sibling tests. Do not grow `hooks/src/engine/discovery.rs` unless the edit is trivial.
- Do not implement Claude parked hook rows 097 or 101-104 here. No second hook registry, no new hook policy layer, and no model-context hook output beyond existing capped behavior.
- Pre-junior dispatch limit: choose one hook behavior per patch.

Acceptance:

- A bad hook does not disable all valid hooks.
- Hook output is bounded.
- Secrets are redacted.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

If touching `hooks/` directly, run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-hooks
```

## Stage 5: Agent Job Cleanup And Structured Results

Owner:

- `ontocode-rs/core/src/tools/handlers/agent_jobs.rs`
- `ontocode-rs/core/src/tools/handlers/multi_agents*`
- `ontocode-rs/core/src/agent/`
- `ontocode-rs/state/src/runtime/agent_jobs.rs`

Task:

- Add tests for typed subagent results, progress snapshots, job aggregation, cancellation, strict structured-result parsing, and orphan cleanup.
- Prefer sibling test files or extracted helpers. Do not grow `agent_jobs.rs` unless the edit is trivial.
- Pre-junior dispatch limit: choose one agent behavior per patch.
- Do not change agent protocol or persisted state shape.
- Do not implement Claude parked agent/session rows 057-059 or 148-150 here. No scheduler changes, parent/child job model, new session command behavior, or persisted-state expansion.

Acceptance:

- Canceled or failed subagents are cleaned up.
- Job progress remains deterministic.
- Structured-result parsing rejects prose when JSON is required.

Run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-core
```

If touching `state/` directly, run:

```sh
cd ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-state
```

## Closure Checklist

- Run `just fmt` in `ontocode-rs` after code changes.
- Run `CARGO_BUILD_JOBS=8 just fix -p <changed-package>` before finalizing Rust changes.
- Run the smallest relevant package test listed in the changed stage.
- If any TUI surface changes, also run `CARGO_BUILD_JOBS=8 just test -p ontocode-tui` and inspect snapshots.

## Dispatch Ledger

| Date | Task | ADR row | Owner | Status | Agent model | Notes |
|---|---|---:|---|---|---|---|
| 2026-06-19 | MCP lifecycle hardening first slice | 121 | `ontocode-rs/codex-mcp/src/connection_manager.rs`; MCP tests | covered | `gpt-5.4-mini` fallback | Existing test `no_local_runtime_fails_local_stdio_but_keeps_local_http_server` already proves one server can fail without hiding healthy servers. No code change needed. |
| 2026-06-19 | MCP partial server failure fixture | 122 | `ontocode-rs/codex-mcp/src/connection_manager_tests.rs`; connection manager owner | covered | `gpt-5.4-mini` fallback | Existing test `no_local_runtime_fails_local_stdio_but_keeps_local_http_server` already proves one failing MCP server does not hide a healthy server, so no code change was needed. |
| 2026-06-19 | MCP error/status mapping fixture | 124 | `ontocode-rs/codex-mcp/src/connection_manager_tests.rs`; `required_startup_failures`/status owner | covered | `gpt-5.4-mini` fallback | Existing test `no_local_runtime_fails_local_stdio_but_keeps_local_http_server` already covers the current startup failure/status mapping by asserting `wait_for_server_ready("stdio", ...) == false` and `required_startup_failures(&["stdio".to_string()])` returns the expected local-stdio error. No code change needed. |
| 2026-06-19 | MCP OAuth/token redaction fixture | 127 | `ontocode-rs/rmcp-client/src/oauth.rs`; MCP/OAuth redaction tests | covered | `gpt-5.4-mini` fallback | Existing test `save_oauth_tokens_rejects_malformed_record_without_leaking_tokens` already calls `assert_error_redacts_oauth_tokens(&error, &["access-token", "refresh-token"])`, so row 127 is covered without code changes. `CARGO_BUILD_JOBS=8 just test -p ontocode-rmcp-client` passed. |
| 2026-06-19 | MCP tool schema cycle guard | 130 | `ontocode-rs/tools/src/mcp_tool_tests.rs` | covered | `gpt-5.4-mini` fallback | Added `parse_mcp_tool_handles_cyclic_local_refs` to prove recursive local `$ref`/`$defs` handling through the MCP wrapper. `CARGO_BUILD_JOBS=8 just test -p ontocode-tools` passed. |
| 2026-06-19 | Apply-patch stale-edit failure wording | 23 | `ontocode-rs/core/tests/suite/apply_patch_cli.rs`; `shell_serialization.rs` if needed | covered | `gpt-5.4-mini` fallback | Existing coverage already asserts the stale/missing-context wording: `apply_patch_cli_reports_missing_context` and `apply_patch_cli_missing_second_chunk_context_rejected` both require `Failed to find expected lines in`, and `shell_serialization.rs` already pins custom-tool failure output for missing-file cases. No code change needed. |
| 2026-06-19 | Apply-patch parser ambiguity rejection | 26 | `ontocode-rs/apply-patch/src/parser.rs`; `ontocode-rs/apply-patch/tests/suite/tool.rs` | covered | `gpt-5.4-mini` fallback | Existing parser and invocation tests already cover the row-26 ambiguity cases: invalid hunk header rejection, empty update hunk rejection, lenient heredoc mismatch/missing-closing rejection, missing-context rejection, and implicit-invocation ambiguity guards. `CARGO_BUILD_JOBS=8 just test -p ontocode-apply-patch` passed with 84/84 tests. |
| 2026-06-19 | Apply-patch malformed parser input | 27 | `ontocode-rs/apply-patch/src/parser.rs`; `ontocode-rs/apply-patch/tests/suite/tool.rs` | covered | `gpt-5.4-mini` fallback | Existing malformed-input coverage already exercises invalid patch boundaries, empty update hunks, invalid hunk headers, and empty environment IDs in `parser.rs`, with matching CLI coverage for malformed patch rejection in `tool.rs`. `CARGO_BUILD_JOBS=8 just test -p ontocode-apply-patch` passed in this session. |
| 2026-06-19 | Apply-patch custom tool call coverage | 31 | `ontocode-rs/core/tests/suite/shell_serialization.rs` | covered | `gpt-5.4-mini` fallback | Existing custom apply_patch coverage already proves create/update/failure output behavior in `apply_patch_custom_tool_call_creates_file` (lines 155-193), `apply_patch_custom_tool_call_updates_existing_file` (lines 196-233), and `apply_patch_custom_tool_call_reports_failure_output` (lines 236-265). Focused verification passed with `CARGO_BUILD_JOBS=8 just test -p ontocode-core apply_patch_custom_tool_call_creates_file apply_patch_custom_tool_call_updates_existing_file apply_patch_custom_tool_call_reports_failure_output`. |
| 2026-06-19 | Apply-patch failure matrix | 32 | `ontocode-rs/core/tests/suite/apply_patch_cli.rs` | covered | `gpt-5.4-mini` fallback | Existing `apply_patch_cli.rs` coverage already exercises the requested failure matrix: invalid hunk header, missing context, missing target file, delete missing file, empty patch, delete directory, path traversal, move path traversal, and verification-failure no-side-effects. Focused `CARGO_BUILD_JOBS=8 just test -p ontocode-core apply_patch_cli_` passed in this session. |
| 2026-06-19 | Apply-patch response helper reuse | 33 | `ontocode-rs/core/tests/common/responses.rs`; `ontocode-rs/core/tests/common/test_codex.rs` | covered | `gpt-5.4-mini` fallback | Existing structured helpers already cover the row: `core_test_support responses::tests::call_output_content_and_success_returns_only_single_text_content_item`, `core_test_support test_codex::tests::custom_tool_call_output_text_returns_output_text`, and `ontocode-core::all suite::apply_patch_cli::apply_patch_aggregates_diff_preserves_success_after_failure` all passed. No duplicate apply-patch harness was added. |
| 2026-06-19 | Apply-patch move overwrite rejection challenge | 34 | `ontocode-rs/core/tests/suite/apply_patch_cli.rs`; `ontocode-rs/apply-patch/tests/suite/tool.rs` | challenged | `gpt-5.4-mini` fallback | Focused tests passed and confirm overwrite-allowed behavior: `test_apply_patch_cli_move_overwrites_existing_destination` and `apply_patch_cli_move_overwrites_existing_destination` both expect the destination file to be replaced, so no runtime rejection gap is proven. |
| 2026-06-19 | Apply-patch move/new-dir behavior tests | 35 | `ontocode-rs/core/tests/suite/apply_patch_cli.rs`; `ontocode-rs/apply-patch/tests/suite/tool.rs` | covered | `gpt-5.4-mini` fallback | Existing tests already cover the row: `apply_patch_cli_moves_file_to_new_directory` and `test_apply_patch_cli_moves_file_to_new_directory` both pass. Verified with `CARGO_BUILD_JOBS=8 just test -p ontocode-core apply_patch_cli_moves_file_to_new_directory` and `CARGO_BUILD_JOBS=8 just test -p ontocode-apply-patch moves_file_to_new_directory`. |
| 2026-06-19 | Apply-patch failure output cap tests | 36 | `ontocode-rs/core/tests/suite/shell_serialization.rs` | covered | `gpt-5.4-mini` fallback | Added `apply_patch_custom_tool_call_truncates_failure_output_over_cap` alongside the existing exact failure test. Verified with `CARGO_BUILD_JOBS=8 just test -p ontocode-core apply_patch_custom_tool_call`. |
| 2026-06-19 | Apply-patch single harness constraint | 37 | `ontocode-rs/core/tests/suite/apply_patch_cli.rs` | covered | `gpt-5.4-mini` fallback | `apply_patch_cli.rs` exposes one reusable harness entrypoint (`apply_patch_harness`) plus one private configurator helper (`apply_patch_harness_with`); the other apply-patch helpers are mounts, not duplicate harness builders. Verified by grep/line inspection only; no tests run. |
| 2026-06-19 | Apply-patch parse-failure no-write test | 38 | `ontocode-rs/core/tests/suite/apply_patch_cli.rs` | covered | `gpt-5.4-mini` fallback | Added `apply_patch_cli_parse_failure_has_no_writes`; `CARGO_BUILD_JOBS=8 just test -p ontocode-core apply_patch_cli_parse_failure_has_no_writes` passed. The test seeds `sentinel.txt`, submits a malformed patch that hits `not a valid hunk header`, and asserts `sentinel.txt` stays `before\n` while `created.txt` is not created. |
| 2026-06-19 | Apply-patch large output truncation tests | 39 | `ontocode-rs/core/tests/suite/apply_patch_cli.rs`; `ontocode-rs/core/tests/suite/shell_serialization.rs` if custom-tool-specific | covered | `gpt-5.4-mini` fallback | Existing truncation coverage already spans the row: `apply_patch_custom_tool_call_truncates_failure_output_over_cap` asserts the custom-tool failure output contains `truncated` and is shorter than the raw failure, and `shell_serialization.rs` also includes shell output cap tests. Focused verification passed with `CARGO_BUILD_JOBS=8 just test -p ontocode-core apply_patch_custom_tool_call_truncates_failure_output_over_cap`. |
| 2026-06-19 | Apply-patch changed-file verification | 40 | `ontocode-rs/core/tests/suite/apply_patch_cli.rs`; apply-patch diff/event assertions | covered | `gpt-5.4-mini` fallback | Existing coverage already proves the changed-file reporting path: `apply_patch_cli_multiple_operations_integration` asserts `Success. Updated the following files:` plus `A nested/new.txt`, `M modify.txt`, and `D delete.txt`; `apply_patch_aggregates_diff_across_multiple_tool_calls` and `apply_patch_aggregates_diff_preserves_success_after_failure` keep the aggregated `TurnDiff` tied to the changed files and successful content across multiple apply-patch calls; `apply_patch_shell_command_heredoc_with_cd_emits_turn_diff` pins `PatchApplyEnd` success and `TurnDiff` emission. Verified with `CARGO_BUILD_JOBS=8 just test -p ontocode-core apply_patch_cli_multiple_operations_integration apply_patch_aggregates_diff_across_multiple_tool_calls apply_patch_aggregates_diff_preserves_success_after_failure apply_patch_shell_command_heredoc_with_cd_emits_turn_diff`. |
| 2026-06-19 | Context overflow retry tests | 81 | `ontocode-rs/core/src/session/turn.rs`; existing compact/session tests | covered | `gpt-5.4-mini` fallback | Existing `manual_compact_retries_after_context_window_error` already proves the bounded overflow retry path: it mounts a `context_length_exceeded` compact failure followed by a successful retry and asserts the retry drops exactly one history item. Focused verification passed with `CARGO_BUILD_JOBS=8 just test -p ontocode-core manual_compact_retries_after_context_window_error`. |
| 2026-06-19 | Compaction/session overflow fixtures | 82 | `ontocode-rs/core/tests/suite/compact.rs`; `ontocode-rs/core/tests/suite/compact_remote.rs` | blocked | `gpt-5.4-mini` fallback | Local overflow fixture coverage is proven by `manual_compact_retries_after_context_window_error` passing. Remote pre-turn compaction fixtures are currently failing before this row can close: `snapshot_request_shape_remote_pre_turn_compaction_context_window_exceeded`, `snapshot_request_shape_remote_pre_turn_compaction_including_incoming_user_message`, and `snapshot_request_shape_remote_pre_turn_compaction_strips_incoming_model_switch` all observe zero remote compact requests. No runtime code change accepted under row 82. |
| 2026-06-19 | Same-model retry loop guard tests | 83 | `ontocode-rs/core/tests/suite/compact.rs`; `ontocode-rs/core/tests/suite/compact_remote.rs`; `session/turn.rs` retry guards | covered | `gpt-5.4-mini` fallback | Existing owner-local coverage already proved the guard, and the new `pre_sampling_compact_does_not_run_for_same_model` test closes the same-model path explicitly. Focused verification passed with `CARGO_BUILD_JOBS=8 just test -p ontocode-core pre_sampling_compact_does_not_run_for_same_model pre_sampling_compact_runs_on_switch_to_smaller_context_model`. |
| 2026-06-19 | Compaction request shape snapshots | 85 | `ontocode-rs/core/tests/suite/compact.rs`; `ontocode-rs/core/tests/suite/compact_remote.rs` snapshots | blocked | `gpt-5.4-mini` fallback | Local request-shape snapshots pass (`snapshot_request_shape_mid_turn_continuation_compaction`, `snapshot_request_shape_pre_turn_compaction_including_incoming_user_message`, `snapshot_request_shape_pre_turn_compaction_strips_incoming_model_switch`, `snapshot_request_shape_pre_turn_compaction_context_window_exceeded`, `snapshot_request_shape_manual_compact_without_previous_user_messages`), but the remote request-shape set still fails with zero remote compact requests (`snapshot_request_shape_remote_manual_compact_restates_realtime_start`, `snapshot_request_shape_remote_mid_turn_compaction_does_not_restate_realtime_end`, `snapshot_request_shape_remote_compact_resume_restates_realtime_end`, `snapshot_request_shape_remote_mid_turn_compaction_summary_only_reinjects_context`, `snapshot_request_shape_remote_mid_turn_compaction_multi_summary_reinjects_above_last_summary`, `snapshot_request_shape_remote_manual_compact_without_previous_user_messages`, `remote_manual_compact_api_auth_omits_service_tier_and_reuses_prompt_cache_key`, `remote_manual_compact_chatgpt_auth_reuses_service_tier_and_prompt_cache_key`). Row 82 remains out of scope. |
| 2026-06-19 | Bounded compaction failure event tests | 86 | `ontocode-rs/core/tests/suite/compact.rs`; `ontocode-rs/core/tests/suite/compact_remote.rs`; compaction failure event paths | covered | `gpt-5.4-mini` fallback | Existing local pre-turn compaction context-window failure coverage already proves the bounded failure event path: `snapshot_request_shape_pre_turn_compaction_context_window_exceeded` waits for `EventMsg::Error`, then `TurnComplete`, and pins the failed compaction request shape. Verified with `TMPDIR="$(mktemp -d /var/tmp/ontocode-core.XXXXXX)" CARGO_BUILD_JOBS=8 just test -p ontocode-core snapshot_request_shape_pre_turn_compaction_context_window_exceeded`. Remote failure-event tests were not used as closure evidence because they currently fall through to an unmatched `/v1/responses` request after compact failure; row 82/85 remote compact blockers remain separate. |
| 2026-06-19 | Hard cap tests for context items | 88 | `ontocode-rs/core/src/context`; `ontocode-rs/core/src/context_manager`; existing context/core tests | covered | `gpt-5.4-mini` fallback | Existing tests already cover model-visible context hard caps: context fragment detection/invalid-source bounds, additional context truncation before model input, initial-context budget trimming, tool-output token caps, and original-detail image patch caps. Verified manager-side with `TMPDIR="$(mktemp -d /var/tmp/ontocode-core.XXXXXX)" CARGO_BUILD_JOBS=8 just test -p ontocode-core record_items_truncates_function_call_output_content record_items_truncates_custom_tool_call_output_content record_items_respects_custom_token_limit original_detail_images_are_capped_at_max_patch_count contextual_user_fragment_is_dyn_compatible`; worker also passed `detects_internal_model_context_fragment`, `additional_context_values_are_truncated_before_model_input`, and `build_initial_context_trims_skill_metadata_from_context_window_budget`. No code change needed. |
- Update this file status only after tests pass.

## Stop Conditions

Stop and ask for review if a task requires:

- new runtime architecture
- new public API/config/SDK/schema
- new persisted state
- new model-visible context fragment
- changes outside the listed owner files
- changes over roughly 300 lines
