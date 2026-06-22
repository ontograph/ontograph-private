# R5AQ ChatGPT Rename Closure

Date: 2026-06-12

## Closure

Accepted R5AQ: `codex-chatgpt` -> `ontocode-chatgpt` and `codex_chatgpt` -> `ontocode_chatgpt`.

The rename stayed identity-only across Cargo package metadata, Rust lib crate name, Bazel crate identity, workspace dependencies, and direct imports.

## Preserved Surfaces

- ChatGPT product/backend/auth/connector semantics.
- Compatibility originator and remote-client strings: `codex_chatgpt_desktop`, `codex_chatgpt_android_remote`, and `codex_chatgpt_ios_remote`.
- Apply-command behavior, task diff parsing, connector directory caching, plugin app filtering, workspace settings cache behavior, app-server request processors, CLI command behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and `ontocode-rs/chatgpt` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-chatgpt --no-tests=pass`: passed, 8 tests.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`: passed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server plugin_list`: passed, 34 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server app_list`: passed, 13 tests.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active-source stale-reference search for `codex_chatgpt|codex-chatgpt`: 5 matches, all intentional compatibility strings.
- Cargo metadata residual `codex-*` package count: 30.
- `git diff --check`: clean.
- OntoIndex `detect-changes --repo codex`: high risk from the known broad dirty tree, not a new ChatGPT-specific blocker.

## Model Fallback

Worker and manager recorded fallback use of `gpt-5.4-mini` because `gpt-5.3-codex-spark` was unavailable or usage-limited.
