# R5V Memories Extension Rename Worker Verification

Date: 2026-06-11

## Outcome

- Implemented `codex-memories-extension` -> `ontocode-memories-extension`.
- Implemented `codex_memories_extension` -> `ontocode_memories_extension`.
- Preserved memory tool namespace/tool names, add/list/read/search behavior, local memories backend behavior, prompt/template content, metrics behavior, app-server extension registration behavior, telemetry, env/config/wire/generated names, persisted state, and the existing `ext/memories` directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_memories_extension|codex-memories-extension` in `ontocode-rs` passed with 0 matches.
- `git diff --check` passed.
- OntoIndex CLI fallback `detect-changes --repo codex` reported repo-wide high risk from unrelated dirty worktree changes.

## Residual Count

- Cargo metadata reports 51 remaining `codex-*` workspace packages after this slice.
