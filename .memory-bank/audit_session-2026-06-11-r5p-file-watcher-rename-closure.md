# R5P File Watcher Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-file-watcher` -> `ontocode-file-watcher`.
- Accepted `codex_file_watcher` -> `ontocode_file_watcher`.
- Identity-only package/lib/Bazel/import rename.
- Preserved file-watch registration/unregistration, notify filtering, debounce/throttle behavior, subscriber lifecycle, path watch-count handling, app-server fs-watch/skills-watcher/thread-state behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `file-watcher` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-file-watcher --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`: passed.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active-source stale-reference search for `codex_file_watcher|codex-file-watcher`: clean.
- `git diff --check`: clean.
- Cargo metadata reports 57 remaining `codex-*` workspace packages.
- OntoIndex CLI fallback `detect-changes --repo codex` still reports the known broad dirty-tree high-risk context, not a scoped R5P-only blocker.

## Notes

- Worker verification completed on `gpt-5.4`.
- The prior model fallback context remains recorded: `gpt-5.3-codex-spark` hit usage limit and `gpt-5.4-mini` hit capacity earlier in this residual-package stage.
- R6 cleanup remains blocked while residual `codex-*` package identities remain.
