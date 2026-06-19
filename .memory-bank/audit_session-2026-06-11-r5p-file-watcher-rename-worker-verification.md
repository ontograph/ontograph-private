# R5P File Watcher Rename Worker Verification

Date: 2026-06-11

## Scope

- Verified Cargo package rename `codex-file-watcher` -> `ontocode-file-watcher`.
- Verified Rust lib crate rename `codex_file_watcher` -> `ontocode_file_watcher`.
- Verified scoped workspace metadata, Bazel crate identity, `ontocode-app-server` dependency/import usage, Cargo lock metadata, and the existing `file-watcher` directory path for the file-watcher slice.

## Guardrails

- File-watch registration/unregistration semantics were not changed.
- Notify event filtering, debounce/throttle behavior, subscriber lifecycle, and path watch-count handling were preserved.
- App-server `fs_watch`, `skills_watcher`, and `thread_state` behavior, env/config/wire/generated names, telemetry/product strings, persisted state, public commands, runtime package layout, and protocol/generated crates were preserved.

## Verification

- Pre-edit OntoIndex evidence from the risk review remained LOW for `FileWatcher`, `FileWatcherSubscriber`, and `FileWatcher.add_subscriber`, with repoPath `/opt/demodb/_workfolder/ontocode`.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-file-watcher --no-tests=pass`: passed; 21 tests, bench-smoke completed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`: passed; command exited 0 after focused build/test execution and bench-smoke completed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active `ontocode-rs` source stale-reference search for `codex_file_watcher|codex-file-watcher`: zero matches.
- `git diff --check`: passed.
- OntoIndex CLI fallback `detect-changes --repo codex`: completed; reported broad dirty-tree context of 200 files, 320 symbols, 8 affected processes, high risk.
- Cargo metadata reports 57 remaining `codex-*` workspace packages after R5P.

## Notes

- The focused `ontocode-app-server` verification initially waited on a shared artifact-directory lock because another app-server nextest run was active in the dirty worktree; this run completed without reverting or interrupting unrelated work.
- Focused test output repeated known unrelated Windows sandbox duplicate-bin warnings.
- Bazel lock update repeated existing `rules_rs` crate-annotation warnings for `gio-sys`, `glib-sys`, `gobject-sys`, `libgit2-sys`, and `libssh2-sys`.
- Prior fallback context remained in effect: `gpt-5.3-codex-spark` hit the usage limit, `gpt-5.4-mini` hit capacity, and this worker verification completed on `gpt-5.4`.
