# R5AQ ChatGPT Rename Worker Verification

Date: 2026-06-12

Model fallback: `gpt-5.4-mini`

## Result

- Renamed `codex-chatgpt` -> `ontocode-chatgpt` and `codex_chatgpt` -> `ontocode_chatgpt`.
- Preserved ChatGPT product/backend/auth/connector semantics, compatibility originator strings (`codex_chatgpt_desktop`, `codex_chatgpt_android_remote`, `codex_chatgpt_ios_remote`), CLI/app-server behavior, and the existing `ontocode-rs/chatgpt` directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-chatgpt --no-tests=pass` passed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests` passed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests` passed.
- Focused app-server tests `plugin_list` and `app_list` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- Active-source stale-reference search found 5 remaining matches, all intentional compatibility strings.
- `cargo metadata --format-version 1 --no-deps` reports 30 remaining `codex-*` packages.
- `git diff --check` passed.
- OntoIndex `detect-changes --repo codex` reported high risk from the pre-existing dirty tree, not from this slice.
