# R5BK Git Utils Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-git-utils` -> `ontocode-git-utils` and `codex_git_utils` -> `ontocode_git_utils`.
- Scope stayed identity-only: package metadata, library crate name, Bazel target/deps, imports, README references, and lockfiles.
- Preserved git patch application, staging, path extraction, git-apply output parsing, baseline repository creation/reset/diff rendering, repo-root detection, remote URL canonicalization/hash behavior, merge-base selection, rollout/session metadata, analytics accepted-line repo hashes, CLI doctor/title behavior, TUI diff/status rendering, ChatGPT apply command, cloud task apply flows, secrets environment id, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `git-utils` directory path.

## Verification

- Worker verification passed for `ontocode-git-utils`, analytics, ChatGPT, cloud-tasks-client, CLI, core, rollout, thread-store, secrets, TUI, fmt, Bazel lock update/check, stale-reference search, metadata count, diff check, and OntoIndex `detect-changes --repo codex`.
- Manager stale-reference search found no `codex_git_utils` or `codex-git-utils` refs in `ontocode-rs`.
- Manager metadata check reports 10 remaining `codex-*` packages.
- Manager `git diff --check` is clean.

## Notes

- Lorentz `019ebd3d-47e2-7003-ad43-a29a572ddbb9` completed the scoped patch and verification on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
