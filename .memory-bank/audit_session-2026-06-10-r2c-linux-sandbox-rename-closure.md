# R2C Linux Sandbox Rename Closure

Date: 2026-06-10

## Scope

- Implemented `codex-linux-sandbox` -> `ontocode-linux-sandbox` as an identity-only Cargo package/library crate/Bazel/import rename.
- Preserved the helper binary name `ontocode-linux-sandbox`.
- Preserved legacy `codex-linux-sandbox` arg0 compatibility and bwrap `--argv0` compatibility behavior.
- Preserved sandbox policy behavior, bubblewrap/Landlock/seccomp/proxy routing behavior, package-layout names, public commands, env/config semantics, protocol shape, telemetry, runtime layout, and persisted state.

## Verification

- `cargo metadata --format-version 1 --no-deps`: passed.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox`: passed, 116 tests.
- `CARGO_BUILD_JOBS=8 just test -p codex-arg0`: passed, 8 tests.
- `CARGO_BUILD_JOBS=8 just test -p codex-core`: passed, 2648 tests, 14 skipped, 1 unrelated realtime flaky retry passed.
- Focused arg0 dispatch tests: passed, 2 tests.
- Focused core sandbox tests: passed, 4 tests.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Stale package/lib reference search for `codex-linux-sandbox|codex_linux_sandbox` in metadata/import/lock surfaces: clean.
- `git diff --check`: passed.
- Scoped OntoIndex `gn_verify_diff`: passed for the R2C file slice; whole-worktree verification remains noisy because unrelated dirty files exceed the scan cap.

## Intentional Legacy Strings

- `codex-linux-sandbox` remains accepted by `codex-arg0` as a legacy helper arg0.
- `codex-linux-sandbox` remains the bwrap `--argv0` compatibility value.
- `codex-linux-sandbox-proxy-` remains the proxy socket directory prefix to avoid runtime-layout changes.
- Existing `codex_linux_sandbox_exe` runtime/config field names remain unchanged because they are not package/lib/import identities.
