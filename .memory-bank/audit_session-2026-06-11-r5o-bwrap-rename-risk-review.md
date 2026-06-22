# R5O Bwrap Rename Risk Review

Date: 2026-06-11

## Scope

- Rename Cargo package `codex-bwrap` to `ontocode-bwrap`.
- Rename Bazel Rust crate identity `codex_bwrap` to `ontocode_bwrap`.
- Preserve the `bwrap` binary name and the existing `bwrap` directory path.

## Direct Inventory

- Active direct refs: 4.
- Direct reverse dependencies: 0.
- Ref homes: `bwrap/Cargo.toml`, `bwrap/BUILD.bazel`, and synthetic mount-target compatibility strings in `linux-sandbox/src/linux_run_main.rs` and `linux-sandbox/src/linux_run_main_tests.rs`.

## OntoIndex Evidence

- `bwrap/src/main.rs::main`: LOW, 0 impacted nodes, 0 affected modules, 0 affected processes.
- `bwrap/build.rs::main`: LOW, 0 impacted nodes, 0 affected modules, 0 affected processes.
- CLI fallback was used because the OntoIndex MCP facade has known repo/tool availability issues; graph snapshots point to `/opt/demodb/_workfolder/ontocode`.

## Guardrails

- Do not change the `bwrap` binary name.
- Do not change vendored bubblewrap/libcap FFI behavior, `bwrap_main` symbol mapping, build-script availability detection, or Bazel `bwrap-ffi` target wiring.
- Do not rename linux-sandbox synthetic mount-target compatibility strings unless a separate sandbox compatibility review approves it.
- Do not change sandbox/helper runtime behavior, env/config/wire/generated names, telemetry/product strings, persisted state, public commands, or runtime package layout.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-bwrap --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-linux-sandbox --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_bwrap|codex-bwrap` in `ontocode-rs`; classify any remaining refs.
- `git diff --check`
- OntoIndex CLI fallback: `detect-changes --repo codex`
