# R2A Shell Escalation Rename Risk Review

Date: 2026-06-10

## Decision

Approve one exact identity-only slice:

- `codex-shell-escalation` -> `ontocode-shell-escalation`

## OntoIndex Impact

- `EscalationPolicy`: HIGH, 20 impacted nodes, 3 direct, 2 affected processes, 3 modules.
- `run_shell_escalation_execve_wrapper`: CRITICAL, 12 impacted nodes, 2 direct, 5 modules.
- `main_execve_wrapper`: LOW, 0 impacted nodes.

## Direct Dependents

- `codex-arg0`
- `codex-core`

## Allowed Scope

- Cargo package rename.
- Rust library crate rename.
- Bazel crate-name metadata update.
- Workspace/dependent manifest updates.
- Rust import path updates.
- Active README/developer command references for this package.
- Lockfile updates.

## Non-Scope

- No execve wrapper binary-name changes.
- No zsh-fork protocol changes.
- No socket env var changes.
- No escalation policy, decision, permission, or execution behavior changes.
- No sandbox/unsandboxed execution behavior changes.
- No FD/socket handoff behavior changes.
- No public command, telemetry, env/config, protocol, runtime layout, or persisted-state changes.

## Required Verification

- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-shell-escalation`
- `CARGO_BUILD_JOBS=8 just test -p codex-arg0`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- Focused arg0 dispatch, shell escalation, zsh-fork, shell runtime, and unified-exec tests where available.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex-shell-escalation|codex_shell_escalation`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.
