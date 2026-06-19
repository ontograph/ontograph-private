# R5AI Prompts Rename Worker Verification

Date: 2026-06-12

Model fallback: `gpt-5.4-mini` after `gpt-5.3-codex-spark` usage-limit fallback.

## Outcome

- Renamed `codex-prompts` to `ontocode-prompts` and `codex_prompts` to `ontocode_prompts`.
- Preserved prompt text, template rendering, permissions instructions, realtime prompts, compaction prompt re-exports, review prompt usage, goal prompt usage, apply-patch instructions, env/config/wire/generated names, telemetry/product strings, persisted state, and the `prompts` directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-prompts --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core prompt_caching::gpt_5_tools_without_apply_patch_append_apply_patch_instructions`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core realtime_conversation::conversation_uses_default_realtime_backend_prompt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core realtime_conversation::conversation_uses_empty_instructions_for_null_or_empty_prompt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core compact::tests::process_compacted_history_inserts_context_before_last_real_user_message_only`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core goals::tests::goal_context_input_item_is_hidden_user_context`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core context::permissions_instructions --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core tasks::review --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- active-source stale-reference search for `codex_prompts|codex-prompts` returned 0 matches
- `cargo metadata --format-version 1 --no-deps` reports 38 remaining `codex-*` packages
- `git diff --check` passed
- OntoIndex CLI fallback `detect-changes --repo codex` reported a high-risk noisy tree due pre-existing unrelated changes
