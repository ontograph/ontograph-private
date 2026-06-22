# R2C Linux Sandbox Rename Risk Review

Date: 2026-06-10

## Decision

Approve one exact identity-only slice:

- `codex-linux-sandbox` -> `ontocode-linux-sandbox`

## OntoIndex Impact

- `run_main` in `ontocode-rs/linux-sandbox/src/lib.rs`: LOW, 0 impacted nodes.
- `run_main` in `ontocode-rs/linux-sandbox/src/linux_run_main.rs`: LOW, 0 impacted nodes.

Direct inventory overrides the weak graph result because this crate owns the Linux sandbox helper entrypoint and is called through `codex-arg0`.

## Direct Dependents

- `codex-arg0`

## Allowed Scope

- Cargo package rename.
- Rust library crate rename.
- Bazel crate-name metadata update.
- Workspace/dependent manifest updates.
- Rust import path updates.
- Active README/developer package references for this package.
- Lockfile updates.

## Non-Scope

- No binary name changes; `ontocode-linux-sandbox` remains the current helper binary.
- No removal of legacy `codex-linux-sandbox` arg0 compatibility.
- No sandbox policy behavior changes.
- No bubblewrap, landlock, seccomp, proxy routing, or filesystem isolation behavior changes.
- No package-layout names such as `codex-package.json`, `codex-path`, or `codex-resources`.
- No public command, telemetry, env/config, protocol, runtime layout, or persisted-state changes.

## Required Verification

- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox`
- `CARGO_BUILD_JOBS=8 just test -p codex-arg0`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- Focused arg0 sandbox-dispatch and core sandbox tests where available.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale package/lib reference search for `codex-linux-sandbox|codex_linux_sandbox`, while preserving intentional legacy arg0/doc compatibility strings.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.
