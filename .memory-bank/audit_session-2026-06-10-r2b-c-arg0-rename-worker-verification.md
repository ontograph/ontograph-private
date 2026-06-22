# R2B-C Arg0 Rename Worker Verification

Date: 2026-06-10

## Scope

- Renamed Cargo package `codex-arg0` to `ontocode-arg0`.
- Renamed Rust library crate `codex_arg0` to `ontocode_arg0`.
- Updated workspace and dependent manifests, Rust imports/usages, Bazel crate name, Cargo lock entries, and Bazel lock data.
- Preserved public binaries, argv[0] helper aliases, runtime helper file names, package-layout behavior, startup dispatch semantics, dotenv filtering, shell escalation dispatch, apply-patch aliases, Linux sandbox helper dispatch, telemetry, env/config semantics, protocol/schema, and persisted state.

## OntoIndex

- Pre-edit impact reported HIGH risk for `arg0_dispatch_or_else`, `arg0_dispatch`, and `linux_sandbox_exe_path` across CLI, TUI, app-server, MCP server, exec, test-binary-support, and helper dispatch flows.
- The HIGH impact matched the approved R2B-C gate; no unapproved HIGH/CRITICAL impact was introduced.

## Verification

- Passed: `cargo metadata --format-version 1 --no-deps`.
- Passed: `CARGO_BUILD_JOBS=8 just fmt`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-cli`.
- Passed with zero-test remediation: exact `CARGO_BUILD_JOBS=8 just test -p codex-core-api` compiled but exited 4 because the package has no tests; rerun with `--no-tests=pass` passed.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server`.
- Passed with zero-test remediation: exact `CARGO_BUILD_JOBS=8 just test -p codex-test-binary-support` compiled but exited 4 because the package has no tests; rerun with `--no-tests=pass` passed.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`.
- Passed focused startup/helper coverage for arg0 dispatch, package path preservation, apply-patch alias, shell escalation dispatch, and Linux sandbox alias compatibility.
- Passed: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- Passed: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Passed active stale-reference search: no active `codex_arg0` references remain; active `codex-arg0` references are limited to preserved runtime/test path strings.
- Passed: `git diff --check`.
- Passed: scoped OntoIndex `gn_verify_diff`.

## Intentional Old-Name References

- `ontocode-rs/arg0/src/lib.rs` keeps the runtime helper temp-dir prefix `codex-arg0`.
- `ontocode-rs/core/src/config/permissions_tests.rs` keeps test fixture path names containing `codex-arg0`.
- `ontocode-rs/linux-sandbox/src/linux_run_main_tests.rs` keeps Linux sandbox argv[0] compatibility fixture paths containing `codex-arg0`.
- Historical memory-bank inventory/risk-review references to `codex-arg0` and `codex_arg0` remain as audit history, not active source wiring.
