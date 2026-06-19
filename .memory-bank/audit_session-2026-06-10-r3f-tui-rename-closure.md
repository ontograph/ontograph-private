# R3F TUI Rename Closure

Date: 2026-06-10

## Scope

- Accepted `ontocode-tui` -> `ontocode-tui` as an identity-only Cargo package/lib/Bazel/import rename.
- Preserved the standalone `ontocode-tui` binary, telemetry/client-name strings, originator/tool gating strings, legacy snapshot namespace/files, log file name, env/config semantics, app-server startup behavior, CLI/MCP behavior, and persisted state.

## Manager Fixes

- Kept TUI dev-dependency aliasing valid by using the existing workspace `insta` dev-dependency and `extern crate insta as insta_crate`.
- Extended `tools/argument-comment-lint` first-party crate filtering to accept `ontocode_` crate prefixes after the TUI lib rename.
- Updated `scripts/run_tui_with_exec_server.sh` to launch the existing `ontocode` CLI binary instead of the removed `codex` binary.

## Verification

- `cargo fmt --check` in `tools/argument-comment-lint` passed after installing the missing nightly `rustfmt` component.
- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass` passed: 2772 passed, 4 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli config_overrides_from_interactive_preserves_global_options` passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-cloud-tasks --no-tests=pass` passed: 13 passed, 1 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server --no-tests=pass` passed: 14 passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- `bash -n scripts/run_tui_with_exec_server.sh` passed.
- No pending `*.snap.new` files were found by `rg --files`.
- `git diff --check` passed.
- OntoIndex scoped `gn_verify_diff` passed for the full R3F changed-file set.

## Residual

- `cargo test workspace_crate_filter_accepts_first_party_names_only` in `tools/argument-comment-lint` could not run because `dylint-link` is absent from PATH. This is an environment/tooling blocker, not an observed code failure.
