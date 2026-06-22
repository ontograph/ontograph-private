# R5AI Prompts Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-prompts` -> `ontocode-prompts`.
- Accepted `codex_prompts` -> `ontocode_prompts`.
- Scope was identity-only package/lib/Bazel/import rename.

## Preserved Surfaces

- Prompt text and template files.
- Permissions instructions/context-fragment behavior.
- Realtime, compaction, review, goal, and apply-patch prompt behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the `prompts` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-prompts --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core prompt_caching::gpt_5_tools_without_apply_patch_append_apply_patch_instructions`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core realtime_conversation::conversation_uses_default_realtime_backend_prompt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core compact::tests::process_compacted_history_inserts_context_before_last_real_user_message_only`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core goals::tests::goal_context_input_item_is_hidden_user_context`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core context::permissions_instructions --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core tasks::review --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_prompts|codex-prompts`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5AI.
- Active old refs are clean in `ontocode-rs`.
- Cargo metadata reports 38 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5AI-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
