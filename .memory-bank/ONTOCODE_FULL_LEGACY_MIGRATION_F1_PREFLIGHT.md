# Ontocode Full Legacy Migration F1 Preflight

Date: 2026-06-14
Task: F0-E-B
Status: preflight-only
Source plan: `ONTOCODE_FULL_LEGACY_MIGRATION_PROJECT_PLAN.md`
Inputs: Stage 0 review plus the layout, CLI/helpers, and packages/state/protocol matrices.

## Purpose

Prepare the narrow F1 path/layout implementation gate for moving the Rust workspace root from `ontocode-rs/` to `ontocode-rs/`.

This file does not approve implementation while the Stage 0 review remains `no-go-for-implementation`.

## Minimal F1 Scope

- Move the Rust workspace directory path: `ontocode-rs/` -> `ontocode-rs/`.
- Update active path references from `ontocode-rs` to `ontocode-rs`.
- Update active Bazel package-label prefixes from `//ontocode-rs/...` to `//ontocode-rs/...`.
- Update active manifest paths from `ontocode-rs/Cargo.toml` to `ontocode-rs/Cargo.toml`.
- Update active artifact roots from `ontocode-rs/target`, `ontocode-rs/dist`, and `ontocode-rs/unsigned-dist` to the matching `ontocode-rs/...` paths.
- Update active schema, snapshot, CI, package-staging, SDK, V8/native, and remote-test path roots only where they refer to the Rust workspace location.

## Non-Scope

- Public command behavior: `codex` vs `ontocode`.
- Helper executable names: `ontocode-exec`, `codex-linux-sandbox`, `ontocode-execve-wrapper`, app-server, exec-server, and related runtime helper names.
- Cargo package names, Rust crate names, npm package names, Python distribution names, Python import paths, TypeScript package names, native package names, and release asset names.
- `CODEX_*` env vars, `CODEX_HOME`, `.codex`, persisted state, cache, logs, rollout/session paths, and package layout names such as `codex-package.json`.
- Protocol, generated schema/model names, telemetry names, MCP/resource IDs, `.codex-plugin`, and app-server wire names.
- Bazel target names such as `:codex` unless only the containing package label path changes.
- Historical audit docs and migration history that accurately describe past `ontocode-rs` paths.
- OntoIndex repo label migration.
- A permanent `ontocode-rs -> ontocode-rs` compatibility symlink.

## Files And Path Classes Likely Touched

| Path class | Likely files |
| --- | --- |
| Workspace root | `ontocode-rs/**` moved to `ontocode-rs/**` |
| Root task entrypoints | `justfile`, `package.json`, `pnpm-lock.yaml` |
| Bazel root metadata | `MODULE.bazel`, `MODULE.bazel.lock`, `BUILD.bazel`, `defs.bzl`, workspace launcher templates |
| Bazel package labels | all moved `ontocode-rs/**/BUILD.bazel`, `.github/scripts/test_run_bazel_with_buildbuddy.py`, `scripts/list-bazel-*.sh`, `.github/workflows/*.yml` |
| Cargo manifests and verifiers | `ontocode-rs/Cargo.toml`, `ontocode-rs/Cargo.lock`, nested `ontocode-rs/**/Cargo.toml`, `.github/scripts/verify_cargo_workspace_manifests.py`, `.github/scripts/verify_bazel_clippy_lints.py` |
| Package staging | `scripts/codex_package/cargo.py`, `scripts/codex_package/version.py`, `scripts/codex_package/v8.py`, `scripts/build_codex_package.py`, `scripts/codex_package/test_cargo.py`, `codex-cli/scripts/build_npm_package.py` |
| Formatting and helper scripts | `scripts/format.py`, `scripts/debug-codex.sh`, `scripts/test-remote-env.sh`, `scripts/run_tui_with_exec_server.sh`, `scripts/start-ontocode-exec.sh` |
| CI and release workflows | `.github/workflows/rust-ci*.yml`, `rust-release*.yml`, `bazel.yml`, `sdk.yml`, `cargo-deny.yml`, `v8-canary.yml`, `rusty-v8-release.yml`, `rust-release-prepare.yml`, `rust-release-zsh.yml` |
| CI helpers and metadata | `.github/scripts/**`, `.github/actions/**`, `.github/dependabot.yaml`, `.github/CODEOWNERS`, `.github/blob-size-allowlist.txt` |
| Argument-comment lint tooling | `tools/argument-comment-lint/run.py`, `run-prebuilt-linter.py`, `wrapper_common.py`, `test_wrapper_common.py`, `README.md`, `list-bazel-targets.sh`, `lint_aspect.bzl` |
| Tests and fixtures | `sdk/python/tests/test_artifact_workflow_and_binaries.py`, `sdk/typescript/tests/testCodex.ts`, `sdk/typescript/samples/helpers.ts`, `ontocode-rs/tui/tests/**`, `ontocode-rs/core/tests/common/lib.rs`, snapshots that render workspace paths |
| Schema roots | `ontocode-rs/app-server-protocol/**`, `ontocode-rs/core/config.schema.json`, root schema-copy scripts and workflows |
| SDK and binary resolver paths | `sdk/python/scripts/update_sdk_artifacts.py`, `sdk/typescript/**`, SDK workflow cache/build paths |
| Release package output roots | `codex-cli/scripts/**`, `scripts/stage_npm_packages.py`, release workflows that stage from `target`, `dist`, or `unsigned-dist` |
| V8/native inputs | `.github/workflows/v8-canary.yml`, `.github/workflows/rusty-v8-release.yml`, `.github/scripts/rusty_v8_bazel.py`, `scripts/codex_package/v8.py`, `ontocode-rs/v8-poc/**`, `ontocode-rs/code-mode/**` |
| Active docs and memory-bank paths | active operational references in `.memory-bank/**`, `AGENTS.md`, repo READMEs, `tools/**/README.md`, `scripts/**/README.md`; historical records stay classified, not rewritten by default |

## Required Pre-Edit OntoIndex Checks

- Confirm the implementation branch has a clean or explicitly checkpointed baseline; do not run F1 against the current broad dirty worktree.
- Do not run `ontoindex analyze` from more than one process; coordinate any refresh after the checkpoint before implementation starts.
- Run OntoIndex semantic/context search for `ontocode-rs ontocode-rs workspace layout justfile MODULE.bazel CODEX_RS_ROOT schema roots snapshot roots package staging`.
- Run exact context and upstream impact before editing any code symbol found in an owner file.
- At minimum, check these F1 path-sensitive symbols before editing their files:
  - `stage_sources` in `codex-cli/scripts/build_npm_package.py`
  - `stage_codex_sdk_sources` in `codex-cli/scripts/build_npm_package.py`
  - `generate_v2_all` in `sdk/python/scripts/update_sdk_artifacts.py`
  - `generate_types` in `sdk/python/scripts/update_sdk_artifacts.py`
  - `schema_bundle_path` in `sdk/python/scripts/update_sdk_artifacts.py`
  - `schema_root` in `ontocode-rs/app-server-protocol/src/export.rs`
  - `schema_root` in `ontocode-rs/app-server-protocol/tests/schema_fixtures.rs`
  - `configure_insta_workspace_root_for_snapshot_tests` in `ontocode-rs/core/tests/common/lib.rs`
  - `read_workspace_version` in `scripts/codex_package/version.py`
  - package root/constants in `scripts/codex_package/cargo.py`, including the `CODEX_RS_ROOT` path owner
- Disambiguate duplicate symbol names by `file_path`; `schema_root` has at least source and test-fixture owners.
- Record direct callers, affected processes, and risk for each edited symbol before edits.
- Warn the manager and stop for explicit approval if any required impact result is `HIGH` or `CRITICAL`.
- After the implementation diff is ready, run OntoIndex diff verification or pre-commit audit and confirm only expected path/layout files and symbols changed.

## Required Commands

Baseline before moving paths:

```bash
cd /opt/demodb/_workfolder/ontocode
CARGO_BUILD_JOBS=8 cargo metadata --manifest-path ontocode-rs/Cargo.toml --no-deps
```

Primary post-move build:

```bash
cd /opt/demodb/_workfolder/ontocode
CARGO_BUILD_JOBS=8 cargo build --manifest-path ontocode-rs/Cargo.toml -p ontocode-cli --bin ontocode
```

Expected post-move debug artifact:

```text
/opt/demodb/_workfolder/ontocode/ontocode-rs/target/debug/ontocode
```

Rust formatting and focused CLI test:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just fmt
CARGO_BUILD_JOBS=8 just test -p ontocode-cli
```

Path and workspace validation:

```bash
cd /opt/demodb/_workfolder/ontocode
CARGO_BUILD_JOBS=8 cargo metadata --manifest-path ontocode-rs/Cargo.toml --no-deps
rg -n --glob '!ontocode-rs/target/**' --glob '!node_modules/**' --glob '!bazel-*' 'ontocode-rs|//ontocode-rs|CODEX_RS_ROOT'
```

Bazel validation when Bazel labels or lock metadata change:

```bash
cd /opt/demodb/_workfolder/ontocode
CARGO_BUILD_JOBS=8 just bazel-lock-update
CARGO_BUILD_JOBS=8 just bazel-lock-check
bazel query --jobs=8 //ontocode-rs/...
bazel build --jobs=8 //ontocode-rs/cli:codex
```

Schema validation if schema roots or generators move:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just write-app-server-schema
CARGO_BUILD_JOBS=8 just write-config-schema
CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol
```

Snapshot validation if snapshot roots or rendered paths move:

```bash
cd /opt/demodb/_workfolder/ontocode/ontocode-rs
CARGO_BUILD_JOBS=8 just test -p ontocode-tui
cargo insta pending-snapshots -p ontocode-tui
```

Package/script validation when package staging or SDK path owners move:

```bash
cd /opt/demodb/_workfolder/ontocode
python3 -m unittest scripts.codex_package.test_cargo
python3 sdk/python/tests/test_artifact_workflow_and_binaries.py
```

Final whitespace check:

```bash
cd /opt/demodb/_workfolder/ontocode
git diff --check
```

## Rollback Plan

- Move `ontocode-rs/` back to `ontocode-rs/`.
- Revert every path-only reference changed in the same implementation slice.
- Restore `//ontocode-rs/...`, `ontocode-rs/Cargo.toml`, `ontocode-rs/target`, `ontocode-rs/dist`, and `ontocode-rs/unsigned-dist` references.
- Restore `CODEX_RS_ROOT` path values if the constant name is preserved for package-stage compatibility.
- Restore any regenerated Bazel lock, schema, or snapshot output that changed only because the path move failed.
- Remove any temporary compatibility symlink before rollback closure.
- Rerun the baseline command and the `rg` inventory to prove the old layout is consistently restored.

## Manager Go Criteria

- Stage 0 matrices remain accepted and this preflight is reviewed.
- Worktree is clean, or all unrelated dirty/untracked files are checkpointed and excluded from the F1 diff.
- F1 implementation diff is path/layout-only.
- Required OntoIndex context/impact has been run on every edited symbol or path-sensitive owner, with no unresolved `HIGH` or `CRITICAL` result.
- All required commands relevant to changed path classes pass, with exact command output retained in the worker notes.
- Remaining `ontocode-rs` matches are classified as historical, compatibility, generated, or deferred.
- No permanent compatibility symlink is left behind.

## Manager No-Go Criteria

- Worktree remains broadly dirty or unclassified.
- Any patch changes CLI behavior, helper names, package identities, env/state semantics, protocol/generated names, telemetry names, or MCP/wire IDs.
- Any edited symbol lacks fresh OntoIndex context and upstream impact.
- Any `HIGH` or `CRITICAL` impact result lacks explicit manager approval.
- Bazel labels, Cargo metadata, package staging, schema roots, or snapshot roots are changed without the matching validation command.
- Generated output changes include non-path identity churn.
- Rollback would require separating unrelated user/worker changes from the directory move.
