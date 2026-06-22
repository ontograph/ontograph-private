# R5P File Watcher Rename Risk Review

Date: 2026-06-11

## Scope

- Rename Cargo package `codex-file-watcher` to `ontocode-file-watcher`.
- Rename Rust crate identity/import `codex_file_watcher` to `ontocode_file_watcher`.
- Preserve the existing `file-watcher` directory path.

## Direct Inventory

- Active direct refs: 17.
- Direct reverse dependency: `ontocode-app-server`.
- Ref homes: root workspace metadata, `file-watcher/Cargo.toml`, `file-watcher/BUILD.bazel`, `app-server/Cargo.toml`, `app-server/src/fs_watch.rs`, `app-server/src/skills_watcher.rs`, and `app-server/src/thread_state.rs`.

## OntoIndex Evidence

- `FileWatcher`: LOW, 0 impacted nodes, 0 affected modules, 0 affected processes.
- `FileWatcherSubscriber`: LOW, 1 direct same-crate caller, 1 affected module, 0 affected processes.
- `FileWatcher.add_subscriber`: LOW, 0 impacted nodes, 0 affected modules, 0 affected processes.
- CLI fallback was used because the OntoIndex MCP facade has known repo/tool availability issues; graph snapshots point to `/opt/demodb/_workfolder/ontocode`.

## Guardrails

- Do not change file-watch registration/unregistration semantics.
- Do not change notify event filtering, debounce/throttle behavior, subscriber lifecycle, path watch-count handling, or app-server fs-watch/skills-watcher/thread-state behavior.
- Do not change env/config/wire/generated names, telemetry/product strings, persisted state, public commands, runtime package layout, or protocol/generated crates.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-file-watcher --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_file_watcher|codex-file-watcher` in `ontocode-rs`; classify any remaining refs.
- `git diff --check`
- OntoIndex CLI fallback: `detect-changes --repo codex`
