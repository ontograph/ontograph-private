# R3G CLI Rename Worker Verification

Date: 2026-06-10

## Scope

- Implemented identity-only Cargo package rename `codex-cli` -> `ontocode-cli`.
- Implemented identity-only Rust lib crate/Bazel crate rename `codex_cli` -> `ontocode_cli`.
- Updated direct workspace/dependent manifest keys, CLI imports, TUI cargo-shear dev-dep/import, active helper package selectors, `Cargo.lock`, and developer command examples that selected the removed `codex` binary.

## Preserved

- Public `ontocode` binary behavior; no `codex` binary was restored or added.
- Telemetry/product-client strings such as `codex_cli_rs`.
- SDK/package/runtime identities such as `openai-codex-cli-bin`, `codex_cli_bin`, `codex-package.json`, `codex-path`, `codex-resources`, and the `codex-cli` npm workspace.
- Protocol/wire/generated names, env/config semantics, app-server/TUI behavior, and persisted state.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli` passed: 261 tests, 0 skipped, plus bench smoke.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass` passed: 2772 tests, 4 skipped, plus bench smoke.
- `bash -n scripts/run_tui_with_exec_server.sh scripts/start-ontocode-exec.sh scripts/test-remote-env.sh` passed.
- `python3 -m py_compile ontocode-rs/windows-sandbox-rs/sandbox_smoketests.py` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- Active stale-reference search found no remaining Cargo package selectors, manifest package entries, Bazel crate names, or Rust imports for `codex-cli` / `codex_cli`.
- `git diff --check` passed through the lean-ctx wrapper.
- Scoped OntoIndex `gn_verify_diff` passed for the R3G changed-file scope.
