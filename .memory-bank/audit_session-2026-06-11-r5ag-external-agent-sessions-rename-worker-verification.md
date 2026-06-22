# R5AG External-Agent Sessions Rename Worker Verification

Date: 2026-06-11

## Scope

- `codex-external-agent-sessions` -> `ontocode-external-agent-sessions`
- `codex_external_agent_sessions` -> `ontocode_external_agent_sessions`

## Model

- Fallback model used: `gpt-5.4-mini` after `gpt-5.3-codex-spark` usage-limit fallback.

## Result

- Identity-only package/lib/Bazel/import rename completed.
- External session scanning, recency limits, title selection, imported-history rollout construction, tool-call/tool-result bounded tags, token accounting, import ledger hashing/canonicalization, duplicate-import semantics, and app-server external-agent session APIs/processors were preserved.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-sessions --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server external_agent_config`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Repo-wide stale-reference search for `codex_external_agent_sessions|codex-external-agent-sessions`
- `cargo metadata --format-version 1 --no-deps`
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

## Residuals

- Active-source stale-reference search returned 0 matches for `codex_external_agent_sessions|codex-external-agent-sessions`.
- `cargo metadata --format-version 1 --no-deps` reports 40 remaining `codex-*` workspace packages.
- OntoIndex `detect-changes --repo codex` reported high risk because of the preexisting dirty tree, not this slice.
