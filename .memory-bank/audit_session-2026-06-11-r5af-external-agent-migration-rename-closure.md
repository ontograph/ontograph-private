# R5AF External-Agent Migration Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-external-agent-migration` -> `ontocode-external-agent-migration`.
- Accepted `codex_external_agent_migration` -> `ontocode_external_agent_migration`.
- Scope was identity-only package/lib/Bazel/import rename.

## Preserved Surfaces

- MCP config conversion.
- Hook/script migration, subagent import, command skill import, and AGENTS.md term rewriting.
- Claude OAuth import parsing, user-consent gating, token redaction, duplicate/rejection semantics, and debug redaction behavior.
- App-server external-agent config API behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and folder path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-migration --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server external_agent_config`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_external_agent_migration|codex-external-agent-migration`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5AF.
- Active old refs are clean in `ontocode-rs`.
- Cargo metadata reports 41 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5AF-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
