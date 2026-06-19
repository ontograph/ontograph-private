# R2B-E Sandboxing Rename Worker Verification

Date: 2026-06-10

## Scope

- Renamed Cargo package `codex-sandboxing` to `ontocode-sandboxing`.
- Renamed Rust library crate `codex_sandboxing` to `ontocode_sandboxing`.
- Updated workspace dependency keys, dependent manifests, Rust imports/usages, Bazel crate name, and lockfiles.

## Preserved Behavior

- Sandbox policy semantics, permission-profile transforms, platform sandbox selection, Linux sandbox argument generation, bubblewrap lookup/warnings, helper argv0 compatibility, network policy, managed MITM CA handling, sandbox env vars, public command names, protocol/schema surfaces, telemetry, env/config semantics, runtime layout, and persisted state were not intentionally changed.
- Helper binary and compatibility alias names were not renamed.
- Seatbelt tests are macOS-gated and were not available on this Linux verification host; `cargo nextest list -p ontocode-sandboxing | grep seatbelt` returned no local tests.

## Verification

- `cargo metadata --format-version 1 --no-deps` passed.
- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-sandboxing` passed: 51 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server` passed: 810 tests, 1 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli debug_sandbox` passed: 7 tests, 254 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-core` passed: 2648 tests, 14 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui` passed: 2772 tests, 4 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-arg0` passed: 8 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server` passed: 196 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox` passed: 116 tests.
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-sandboxing` passed after tests; no tests were rerun after `fix` per repo guidance.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update` passed.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- Active stale reference search for `codex-sandboxing` and `codex_sandboxing` returned zero matches.
- `git diff --check` passed.
- Scoped OntoIndex `gn_verify_diff` passed for the R2B-E file set.

## Focused Coverage

- Sandboxing crate tests covered manager transforms, policy transforms, Landlock command args, bwrap lookup/warnings, managed MITM CA readability, helper argv0 preservation, and network-policy behavior.
- App-server tests covered command exec, permission-profile command exec, runtime workspace roots, process exec, and sandbox policy validation.
- Core tests covered shell/unified exec, apply-patch sandbox safety, request-permissions grants, sandbox-denied shell command behavior, shell snapshot paths, and local sandbox verification.
- Arg0, exec-server, and linux-sandbox tests covered helper alias compatibility, sandboxed file-system behavior, Landlock/Linux sandbox behavior, bwrap lookup, and managed proxy routing.

## OntoIndex

- Pre-edit impact was CRITICAL for `SandboxManager.transform`, `get_platform_sandbox`, `create_seatbelt_command_args`, and `create_linux_sandbox_command_args_for_permission_profile`; this matched the approved R2B-E risk gate.
- Scoped post-edit OntoIndex `gn_verify_diff` passed with no unexpected changed files or missing tests.
