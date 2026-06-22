# R5AX File System Rename Worker Verification

## Summary

- Identity-only rename completed for `codex-file-system` -> `ontocode-file-system` and `codex_file_system` -> `ontocode_file_system`.
- Preserved file read/write/copy/remove/create-dir/metadata/directory semantics, sandbox context conversion, permission profile/cwd handling, exec-server direct/sandboxed/remote filesystem behavior, config loader filesystem behavior, git-utils filesystem behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `file-system` directory path.
- Runtime model used for this worker run: `gpt-5.4-mini` after Spark usage-limit fallback.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-file-system --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-config loader`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec-server file_system`
- `CARGO_BUILD_JOBS=8 cargo check -p codex-git-utils --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_file_system|codex-file-system`
- `cargo metadata --format-version 1 --no-deps` residual count: 23 `codex-*` packages
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

## Result

- Source refs to `codex_file_system` and `codex-file-system` are clean outside lock/history entries that now carry the new identity.
- The requested `cargo check -p ontocode-git-utils --tests` target does not exist in the current workspace; the equivalent `codex-git-utils` check passed.
- OntoIndex detect-changes still reports the known broad high-risk dirty tree outside this slice.
