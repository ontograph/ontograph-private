# R3G CLI Rename Closure

Date: 2026-06-10

## Scope

- Accepted `codex-cli` -> `ontocode-cli` and `codex_cli` -> `ontocode_cli` as an identity-only Cargo package/lib/Bazel/import rename.
- Preserved the existing public `ontocode` binary and did not restore a `codex` binary.
- Preserved telemetry/product-client strings, SDK/package/runtime identities, protocol/wire/generated names, env/config semantics, app-server/TUI behavior, and persisted state.

## Manager Fixes

- Updated `scripts/start-ontocode-exec.sh` user hint from removed `codex` command to `ontocode`.
- Updated `justfile` direct and Bazel CLI recipes from removed `codex` targets to `ontocode` targets.
- Updated Windows sandbox smoke-test binary resolution from `codex.exe` / `codex` to `ontocode.exe` / `ontocode`.
- Updated app-server test-client quickstart text to `ontocode app-server` while preserving the compatibility flag `--codex-bin`.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-cli` passed: 261 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass` passed: 2772 passed, 4 skipped.
- `bash -n` passed for `scripts/run_tui_with_exec_server.sh`, `scripts/start-ontocode-exec.sh`, and `scripts/test-remote-env.sh`.
- `python3 -m py_compile ontocode-rs/windows-sandbox-rs/sandbox_smoketests.py` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed in worker verification.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- Active stale-reference scan found no old `codex-cli` package selectors, `codex_cli` imports, `--bin codex`, `:codex` Bazel CLI targets, or `bazel-codex` recipes in the R3G scoped files.
- No pending `*.snap.new` files were found by `rg --files`.
- `git diff --check` passed.
- OntoIndex scoped `gn_verify_diff` passed.

## Stage Result

- Stage 3 CLI/app crate rename is complete.
- Next stage is R4 provider/auth/MCP support crates, gated by fresh senior risk review before any dispatch.
