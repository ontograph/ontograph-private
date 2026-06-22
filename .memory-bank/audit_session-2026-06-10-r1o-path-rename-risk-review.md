# R1O Path Internal Crate Rename Risk Review

Date: 2026-06-10

Status: approved for one exact identity-only worker slice.

Decision:
- Approve `codex-utils-path` -> `ontocode-utils-path`.
- Scope is package name, crate/lib name, Bazel crate name, Rust imports, workspace dependency refs, lockfiles, and active direct references only.
- Do not rename or change path normalization behavior, WSL behavior, symlink-write resolution, atomic-write behavior, public command names, config/schema shape, state layout, runtime package layout, telemetry, or persisted data.

Risk evidence:
- Direct Cargo dependents: `codex-cli`, `codex-config`, `codex-core`, `codex-rollout`, `codex-thread-store`, and `ontocode-tui`.
- OntoIndex `normalize_for_path_comparison`: CRITICAL, 26 impacted nodes, 7 direct callers, 12 modules.
- OntoIndex `resolve_symlink_write_paths`: HIGH, 10 impacted nodes, 4 direct callers, 3 modules.
- OntoIndex `write_atomically`: LOW, 0 impacted nodes, but direct text inventory shows config/plugin edit coupling.

Required verification:
- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-path`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli`
- `CARGO_BUILD_JOBS=8 just test -p codex-config`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p codex-rollout`
- `CARGO_BUILD_JOBS=8 just test -p codex-thread-store`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- focused path/config/rollout/thread-store/TUI resume tests if available.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- stale-reference search for `codex-utils-path|codex_utils_path` under active `ontocode-rs` sources.
- `git diff --check`
- OntoIndex `gn_verify_diff` covering changed files and executed tests.
