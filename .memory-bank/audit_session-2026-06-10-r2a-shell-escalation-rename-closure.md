# R2A Shell Escalation Rename Closure

Date: 2026-06-10

## Result

- Renamed Cargo package/library/import references from `codex-shell-escalation` / `codex_shell_escalation` to `ontocode-shell-escalation` / `ontocode_shell_escalation`.
- Updated root workspace dependency, `codex-arg0`, `codex-core`, shell-escalation manifest, Bazel crate name, Rust imports, README identity text, Cargo lock, and Bazel lock.
- Preserved `ontocode-execve-wrapper` binary name and did not change zsh-fork protocol, socket env var, escalation policy/decision behavior, sandbox/unsandboxed behavior, FD/socket handoff, public commands, telemetry, env/config semantics, protocol shape, runtime layout, or persisted state.

## Verification

- PASS: `cargo metadata --format-version 1 --no-deps`.
- PASS: `CARGO_BUILD_JOBS=8 just fmt`.
- PASS: `CARGO_BUILD_JOBS=8 just test -p ontocode-shell-escalation` (20 passed).
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-arg0` (8 passed).
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-core unix_escalation` (18 passed).
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-core zsh_fork` (13 passed).
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-core unified_exec` (93 passed).
- PASS: `CARGO_BUILD_JOBS=8 just test -p codex-core` (2648 passed, 14 skipped).
- PASS: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- PASS: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- PASS: active stale-reference search for `codex-shell-escalation|codex_shell_escalation`.
- PASS: `git diff --check`.
- PASS: scoped OntoIndex `gn_verify_diff`; unscoped verification remains noisy because the worktree contains many unrelated dirty files and the scan caps at 200 files.
