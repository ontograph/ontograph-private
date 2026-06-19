# R4I Config Rename Worker Verification

Date: 2026-06-11

Scope:
- Renamed `codex-config` to `ontocode-config` for Cargo package/dependency identity.
- Renamed `codex_config` to `ontocode_config` for Rust crate/lib/import identity.
- Updated Bazel crate identity in `ontocode-rs/config/BUILD.bazel`.
- Preserved the existing `ontocode-rs/config` directory path.
- Preserved config behavior, schema/wire names, config/env keys, persisted state, telemetry/product strings, and `ConfigToml` shape/semantics.

Risk:
- R4I remained identity-only despite the approved CRITICAL `ConfigToml` impact surface: 82 impacted nodes, 71 direct, affected modules `Config`, `Bottom_pane`, `Loader`, and `Unified_exec`.

Verification:
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just fmt`
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-config --no-tests=pass` (178 passed)
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p codex-core config --no-tests=pass` (537 passed, 2125 skipped)
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-app-server config --no-tests=pass` (123 passed, 688 skipped)
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-cli config --no-tests=pass` (36 passed, 225 skipped)
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-tui config --no-tests=pass` (95 passed, 2681 skipped)
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider-info --no-tests=pass` (20 passed)
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-model-provider --no-tests=pass` (50 passed)
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-login --no-tests=pass` (118 passed)
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just test -p ontocode-mcp config --no-tests=pass` (6 passed, 65 skipped)
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-update` (no `MODULE.bazel.lock` diff)
- Passed: `cd ontocode-rs && CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Passed: `rg -n "\bcodex_config\b|\bcodex-config\b" ontocode-rs --glob "!target" || true`
- Passed: `git diff --check`
- Passed: scoped MCP `gn_verify_diff` with required test matrix; CLI `detect-changes --repo codex` completed with low risk but reported unrelated dirty-tree memory-bank noise, so staged-file cleanup was verified separately.

Remaining old-name refs:
- `ontocode-rs/config/src/state.rs`: `codex-config-tests` test temp directory stem; intentional compatibility/noise, not crate identity.
- `ontocode-rs/config/src/mcp_edit_tests.rs`: `codex-config-mcp-edit-test-*` and `codex-config-mcp-oauth-edit-test-*` test temp directory stems; intentional compatibility/noise, not crate identity.

Notes:
- `just` was blocked by the lean-ctx shell allowlist, so required `just` commands were run through the native terminal with `CARGO_BUILD_JOBS=8`.
- Temporary staged OntoIndex verification left no staged files afterward.
