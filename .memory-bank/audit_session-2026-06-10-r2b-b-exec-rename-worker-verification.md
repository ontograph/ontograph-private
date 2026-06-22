# R2B-B Exec Rename Worker Verification

Date: 2026-06-10

Scope:
- Implemented identity-only rename `ontocode-exec` -> `ontocode-exec`.
- Renamed Rust lib crate identity `codex_exec` -> `ontocode_exec`.
- Preserved the existing `ontocode-exec` binary name.
- Preserved telemetry originator/client strings such as `codex_exec` as intentional non-scope behavior.

Changed surfaces:
- Cargo package/lib identity and root workspace dependency.
- Direct dependent `codex-cli` manifest/import/usages.
- Exec crate binary/test imports and active binary-name test argv/comments.
- Bazel `crate_name`.
- Cargo and Bazel lockfiles.

Verification:
- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec` passed: 122 passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli` passed: 261 passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec -E "test(/resume|review|output_schema|config/)"` passed: 30 passed.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Exact stale package/lib search found no active `ontocode-exec`/`codex_exec` package or crate references; remaining matches are intentional telemetry/originator strings and historical TUI snapshot text.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff` passed with the R2B-B file list and executed-test list; warning only noted the broader dirty worktree scan cap.

Decision:
- Worker verification is complete.
- Manager acceptance is still required before selecting the next R2B runtime path/package-layout crate.
