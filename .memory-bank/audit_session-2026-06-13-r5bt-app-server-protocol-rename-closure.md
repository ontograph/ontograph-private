# R5BT App-Server Protocol Rename Closure

- Scope: manager acceptance for `ontocode-app-server-protocol` -> `ontocode-app-server-protocol` and `codex_app_server_protocol` -> `ontocode_app_server_protocol`.
- Outcome: accepted.
- Evidence:
  - `cargo metadata --no-deps` now reports only one remaining `codex-*` package: `codex-protocol`
  - workspace search shows no active `ontocode-app-server-protocol` refs in `ontocode-rs`
  - `git diff --check` passed
- Verification:
  - `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol`
  - `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
  - `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-client --tests`
  - `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-test-client --tests`
  - `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Known warnings:
  - duplicate compatibility bin-target warnings for old/new command aliases
  - pre-existing `TotalTokenUsageBreakdown` dead-code warning in `ontocode-core`
