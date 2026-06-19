# R1L Absolute Path Internal Crate Rename Closure

Date: 2026-06-10

Status: accepted.

Scope:
- Renamed `codex-utils-absolute-path` to `ontocode-utils-absolute-path`.
- Renamed Rust crate imports from `codex_utils_absolute_path` to `ontocode_utils_absolute_path`.
- Updated workspace dependency refs, direct dependent manifests/imports, Bazel crate name, Cargo lockfile, and Bazel lockfile.
- Preserved path semantics, cwd/canonicalization behavior, sandbox policy behavior, protocol/schema shape, runtime package layout, public command names, state, telemetry, and `CODEX_*` surfaces.

Verification:
- `cargo metadata --format-version 1 --no-deps`: passed; metadata exposes `ontocode-utils-absolute-path` and no `codex-utils-absolute-path`.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-absolute-path`: passed, 25 passed, 1 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-config`: passed, 178 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-protocol`: passed, 231 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-sandboxing`: passed, 51 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`: passed, 122 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`: passed, 2772 passed, 4 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol`: passed, 224 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`: passed, 810 passed, 1 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-apply-patch`: passed, 84 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-core`: passed, 2648 passed, 14 skipped.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed after user-local Bazelisk was installed under `~/.local/bin`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Stale-reference search for `codex-utils-absolute-path|codex_utils_absolute_path` under active `ontocode-rs` sources: passed, 0 matches.
- `git diff --check`: passed.
- Scoped OntoIndex `gn_verify_diff`: passed.

Manager note:
- Whole-tree OntoIndex verification remains noisy because the repo contains hundreds of unrelated dirty files from prior accepted slices and the scan caps at 200 files.
