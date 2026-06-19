# R1M Home Dir Internal Crate Rename Closure

Date: 2026-06-10

Status: accepted.

Scope:
- Renamed `codex-utils-home-dir` to `ontocode-utils-home-dir`.
- Renamed Rust crate imports from `codex_utils_home_dir` to `ontocode_utils_home_dir`.
- Updated workspace dependency refs, direct dependent manifests/imports, Cargo lockfile, Bazel crate name, and Bazel lockfile.
- Preserved `find_codex_home`, `ONTOCODE_HOME`, `CODEX_HOME`, home-directory precedence, fallback behavior, state/config paths, runtime package layout, public command names, telemetry, and persisted data.

Verification:
- `cargo metadata --format-version 1 --no-deps`: passed; metadata exposes `ontocode-utils-home-dir` and no `codex-utils-home-dir`.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-home-dir`: passed, 8 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-arg0`: passed, 8 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-daemon`: passed, 27 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-core`: passed, 2648 passed, 14 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-install-context`: passed, 9 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-network-proxy`: passed, 165 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-rmcp-client`: passed, 64 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`: passed, 2772 passed, 4 skipped.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Stale-reference search for `codex-utils-home-dir|codex_utils_home_dir` under active `ontocode-rs` sources: passed, 0 matches.
- `git diff --check`: passed.
- Scoped OntoIndex `gn_verify_diff`: passed.
