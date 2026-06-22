# R2D Windows Sandbox Rename Closure

Date: 2026-06-10

Scope completed:
- `ontocode-windows-sandbox` package identity renamed to `ontocode-windows-sandbox`.
- Rust crate name/import identity renamed from `codex_windows_sandbox` to `ontocode_windows_sandbox`.
- Workspace, direct dependent manifests/imports, Cargo lock, Bazel lock, and Bazel crate name updated.

Manager correction:
- Worker initially removed legacy Windows helper binary targets and added `ontocode-command-runner`.
- Manager restored legacy/current helper targets and removed `ontocode-command-runner` because helper executable names, setup helper names, and command-runner names are outside R2D scope.

Verification:
- Passed: `cargo metadata --format-version 1 --no-deps`.
- Passed: `CARGO_BUILD_JOBS=8 just fmt`.
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox` (10 passed).
- Passed by worker before manager takeover: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server` (810 passed, 1 skipped).
- Passed by worker before manager takeover: `CARGO_BUILD_JOBS=8 just test -p codex-cli` (261 passed).
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-core` (2648 passed, 14 skipped, 3 retry-passing flakes).
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-tui` (2772 passed, 4 skipped).
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-core windows_sandbox` (14 passed).
- Passed: `CARGO_BUILD_JOBS=8 just test -p codex-cli debug_sandbox` (7 passed).
- Passed: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server windows_sandbox` (12 passed).
- Passed: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- Passed: `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Passed: stale package/lib reference search. Remaining old-name hits are intentional helper/setup/package-builder runtime names.
- Passed: `git diff --check`.
- Passed: scoped OntoIndex `gn_verify_diff`.

Intentional legacy/runtime names preserved:
- `ontocode-windows-sandbox` bin target.
- `ontocode-windows-sandbox-setup` bin target, manifest, resource name, WFP service name, package-builder option/field names.
- `ontocode-command-runner` bin target.
- `ontocode-windows-sandbox*` test temp prefixes and package layout compatibility checks.

Decision:
- R2D accepted.
- Stage 2 helper crate slice is complete.
- R2B runtime path/package-layout crates remain gated by fresh OntoIndex impact/risk review.
