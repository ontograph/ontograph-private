# R5BK Git Utils Rename Worker Verification

Date: 2026-06-12

## Scope

- Renamed `codex-git-utils` -> `ontocode-git-utils`.
- Renamed `codex_git_utils` -> `ontocode_git_utils`.
- Preserved git patch/root/baseline/remote/merge-base behavior, analytics accepted-line flow, ChatGPT apply command flow, cloud-tasks-client apply flow, CLI doctor/title flow, core repo-root/session/turn metadata, exec flow, rollout/thread-store/secrets flows, and TUI diff/status rendering.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-git-utils --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p codex-analytics --lib`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-chatgpt --lib`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cloud-tasks-client --lib`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --lib`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p codex-rollout --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-thread-store --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-secrets --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Exact old git-utils refs are clean.
- Cargo metadata residual count: 10 `codex-*` packages.
- `git diff --check` is clean.
- OntoIndex `detect-changes --repo codex` reports high risk on the known dirty tree baseline.
