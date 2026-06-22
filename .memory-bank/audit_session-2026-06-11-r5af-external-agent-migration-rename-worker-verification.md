# R5AF External-Agent Migration Rename Worker Verification

Date: 2026-06-11

Model: `gpt-5.4-mini` high, used after the requested Spark fallback was unavailable/usage-limited.

## Outcome

- `codex-external-agent-migration` -> `ontocode-external-agent-migration` and `codex_external_agent_migration` -> `ontocode_external_agent_migration` identity-only Cargo package/lib/Bazel/import rename is complete.
- Preserved MCP config conversion, hook/script migration, subagent import, command skill import, AGENTS.md term rewriting, Claude OAuth import parsing, user-consent gating, token redaction, duplicate/rejection semantics, app-server external-agent config API behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and folder path.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-migration --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server external_agent_config`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_external_agent_migration` and `codex-external-agent-migration`
- `cargo metadata --format-version 1 --no-deps` residual count: 41 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Residual References

- None in active `ontocode-rs` source paths; the folder path remains `external-agent-migration`.

## Notes

- `OntoIndex detect-changes` reported broad high risk because the worktree already contains unrelated dirty edits outside this slice.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
