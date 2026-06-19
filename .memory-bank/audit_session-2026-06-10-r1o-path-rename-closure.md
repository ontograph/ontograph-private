# R1O Path Internal Crate Rename Closure

Date: 2026-06-10

Status: accepted.

Scope:
- Renamed `codex-utils-path` to `ontocode-utils-path`.
- Renamed Rust crate imports from `codex_utils_path` to `ontocode_utils_path`.
- Updated workspace dependency refs, direct dependent manifests/imports, Cargo lockfile, Bazel crate name, and Bazel lockfile.
- Preserved path normalization, WSL behavior, symlink-write resolution, atomic-write behavior, public command names, config/schema shape, state layout, runtime package layout, telemetry, persisted data, and env/config semantics.

Verification:
- `cargo metadata --format-version 1 --no-deps`: passed; metadata exposes `ontocode-utils-path` and no `codex-utils-path`.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-path`: passed, 7 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`: passed, 261 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-config`: passed, 178 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-core`: passed, 2648 passed, 14 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-rollout`: passed, 69 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-thread-store`: passed, 76 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`: passed, 2772 passed, 4 skipped.
- Focused path tests: passed, 3 passed, 4 skipped.
- Focused config symlink write test: passed, 1 passed, 177 skipped.
- Focused rollout cwd tests: passed, 2 passed, 67 skipped.
- Focused thread-store cwd tests: passed, 3 passed, 73 skipped.
- Focused TUI resume/cwd tests: passed, 5 passed, 2771 skipped.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Stale-reference search for `codex-utils-path|codex_utils_path` under active `ontocode-rs` sources and lockfiles: passed, 0 matches.
- `git diff --check`: passed.
- Scoped OntoIndex `gn_verify_diff`: passed.
