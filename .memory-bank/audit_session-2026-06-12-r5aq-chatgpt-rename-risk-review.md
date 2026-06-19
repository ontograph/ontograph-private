# R5AQ ChatGPT Rename Risk Review

Date: 2026-06-12

## Decision

Dispatch R5AQ as an identity-only residual package rename:

- `codex-chatgpt` -> `ontocode-chatgpt`
- `codex_chatgpt` -> `ontocode_chatgpt`

## OntoIndex Impact

- `ApplyCommand`: LOW, no affected processes.
- `run_apply_command`: LOW, no affected processes.
- `apply_diff_from_task`: LOW, one impacted caller, no affected processes.
- `list_connectors`: LOW, no affected processes.
- `list_all_connectors_with_options`: LOW, 2 impacted app-server request-processor callers, no affected processes.
- `connectors_for_plugin_apps`: LOW, 9 impacted app-server/plugin callers, no affected processes.
- `codex_plugins_enabled_for_workspace`: LOW, 17 impacted app-server apps/plugins/catalog callers, no affected processes.

## Scope

Allowed:

- Rename Cargo package metadata, Rust lib crate name, Bazel crate identity, dependency entries, and Rust imports from old crate identity to Ontocode identity.
- Update lockfiles and generated Bazel lock metadata as required.

Forbidden:

- Do not rename ChatGPT product, connector, workspace, auth, originator, account, or backend strings.
- Do not rename compatibility originator values such as `codex_chatgpt_desktop`, `codex_chatgpt_android_remote`, or `codex_chatgpt_ios_remote`.
- Do not change apply-command behavior, task diff parsing, connector directory caching, plugin app filtering, workspace settings cache behavior, app-server request processors, CLI command behavior, env/config/wire/generated names, telemetry/product strings, persisted state, or folder path.
- Do not move the existing `ontocode-rs/chatgpt` directory.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-chatgpt --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- Focused app-server plugin/apps tests if available after the rename.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_chatgpt|codex-chatgpt`; classify preserved compatibility originator strings separately.
- Cargo metadata residual package count.
- `git diff --check`
- OntoIndex diff verification through `detect-changes --repo codex`

## Model Fallback

Dispatch uses `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.
