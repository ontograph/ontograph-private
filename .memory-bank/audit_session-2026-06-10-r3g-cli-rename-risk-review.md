# R3G CLI Rename Risk Review

Date: 2026-06-10

## Decision

- Approve exactly one identity-only slice: `codex-cli` -> `ontocode-cli` and `codex_cli` -> `ontocode_cli`.
- Do not change public command behavior. The existing public binary remains `ontocode`; do not restore or rename the removed `codex` binary.

## OntoIndex Impact

- `cli_main` in `ontocode-rs/cli/src/main.rs`: LOW, direct upstream caller `main`.
- `main` in `ontocode-rs/cli/src/main.rs`: LOW, no upstream callers.
- Login stdin/status helpers and sandbox debug helpers: LOW, upstream impact through `cli_main` and `main`.

## Direct Inventory

- `ontocode-rs/cli/Cargo.toml` owns package `codex-cli` and lib crate `codex_cli`.
- `ontocode-rs/cli/BUILD.bazel` owns Bazel crate name `codex_cli`.
- `ontocode-rs/Cargo.toml` has the workspace dependency key `codex-cli`.
- `ontocode-rs/tui/Cargo.toml` dev-depends on `codex-cli`; `ontocode-rs/tui/tests/all.rs` imports `codex_cli` for cargo-shear.
- `scripts/run_tui_with_exec_server.sh` already uses `codex-cli` package selector with `--bin ontocode`.
- `scripts/start-ontocode-exec.sh` and `scripts/test-remote-env.sh` still reference `cargo build -p codex-cli --bin codex`; this slice should update package selectors to `ontocode-cli` and preserve the `ontocode` binary.

## Allowed Changes

- Cargo package name, lib crate name, workspace dependency key, dependent manifest keys, Rust imports, Bazel crate name, lockfiles, and active developer scripts/tests that select the old Cargo package.
- Internal tracing target filters may be updated only if needed to match the renamed Rust crate target.

## Forbidden Changes

- Public `ontocode` binary behavior.
- Telemetry/product-client strings such as `codex_cli_rs`.
- SDK/package/runtime identities such as `openai-codex-cli-bin`, `codex_cli_bin`, `codex-package.json`, `codex-path`, and `codex-resources`.
- Protocol/wire/generated names, app-server API names, env/config semantics, persisted state, or package-manager identities.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`
- Focused script syntax checks for touched shell scripts.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale-reference search distinguishing package/lib refs from intentional telemetry/SDK compatibility names.
- `git diff --check`
- OntoIndex scoped `gn_verify_diff`
