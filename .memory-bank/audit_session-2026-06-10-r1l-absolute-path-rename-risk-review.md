# R1L Absolute Path Internal Crate Rename Risk Review

Date: 2026-06-10

Status: approved for one exact identity-only worker slice.

Decision:
- Approve `codex-utils-absolute-path` -> `ontocode-utils-absolute-path`.
- Scope is package name, crate/lib name, Bazel crate name, Rust imports, workspace dependency refs, lockfiles, and active direct references only.
- Do not rename path semantics, cwd/canonicalization behavior, sandbox policy behavior, protocol/schema shape, runtime package layout, public command names, state, telemetry, or `CODEX_*` surfaces.

Risk evidence:
- Direct Cargo dependents: 43 packages, including config, protocol, sandboxing, exec, TUI, app-server, apply-patch, core, plugin/skill loaders, install context, shell, and Windows/Linux sandbox crates.
- OntoIndex batch impact on all main exports timed out, confirming the surface is too broad for blind automation.
- OntoIndex `canonicalize_preserving_symlinks`: CRITICAL, 22 impacted nodes, 11 direct callers; affected modules include sandbox permission normalization, protocol file-system policy, core permission handlers, exec-server fs sandbox, and tests.
- OntoIndex `canonicalize_existing_preserving_symlinks`: HIGH, 11 impacted nodes, 4 direct callers; affected processes include `ontocode-rs/exec/src/lib.rs::run_main` and `ontocode-rs/tui/src/lib.rs::run_main`.
- OntoIndex struct impact for `AbsolutePathBuf` and `AbsolutePathBufGuard` is LOW due weak type graph coverage, so direct Cargo/text inventory must be treated as authoritative.

Required verification:
- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-absolute-path`
- `CARGO_BUILD_JOBS=8 just test -p codex-config`
- `CARGO_BUILD_JOBS=8 just test -p codex-protocol`
- `CARGO_BUILD_JOBS=8 just test -p codex-sandboxing`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server`
- `CARGO_BUILD_JOBS=8 just test -p codex-apply-patch`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- Representative direct dependent tests if changed imports indicate additional owners.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- stale-reference search for `codex-utils-absolute-path|codex_utils_absolute_path` under active `ontocode-rs` sources.
- `git diff --check`
- OntoIndex `gn_verify_diff` covering all changed files and executed tests.
