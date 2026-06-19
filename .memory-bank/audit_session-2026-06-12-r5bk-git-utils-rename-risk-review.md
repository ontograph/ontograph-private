# R5BK Git Utils Rename Risk Review

Date: 2026-06-12

## Slice

- Rename `codex-git-utils` -> `ontocode-git-utils`.
- Rename Rust crate refs `codex_git_utils` -> `ontocode_git_utils`.
- Identity-only scope: package metadata, library crate name, Bazel target/deps, imports, README references, and lockfiles.

## OntoIndex

- MCP `mcp__ontoindex` is still not wired to `/opt/demodb/_workfolder/ontocode`; use local OntoIndex CLI for this repo.
- `get_git_repo_root`: CRITICAL, 32 impacted nodes, 15 direct, 11 modules, 2 affected processes.
- `apply_git_patch`: CRITICAL, 10 impacted nodes, 8 direct, 5 modules.
- `extract_paths_from_patch`: CRITICAL, 13 impacted nodes, 4 direct, 5 modules.
- `stage_paths`: HIGH.
- `parse_git_apply_output`: HIGH.
- `canonicalize_git_remote_url`: HIGH.
- `merge_base_with_head`: HIGH.

## Guardrails

- Do not change git patch application, staging, path extraction, git-apply output parsing, baseline repository creation/reset/diff rendering, repo-root detection, remote URL canonicalization/hash behavior, merge-base selection, rollout/session metadata, analytics accepted-line repo hashes, CLI doctor/title behavior, TUI diff/status rendering, ChatGPT apply command, cloud task apply flows, secrets environment id, env/config/wire/generated names, telemetry/product strings, persisted state, or the existing `git-utils` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-git-utils --no-tests=pass`
- Focused analytics accepted-line or compile checks.
- ChatGPT apply command checks.
- Cloud-tasks-client compile or apply-path checks.
- CLI doctor/title compile or focused checks.
- Core repo-root/session/turn-metadata checks.
- Exec compile.
- Rollout/thread-store/secrets compile or focused checks.
- TUI diff/status checks.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_git_utils|codex-git-utils`.
- Cargo metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
