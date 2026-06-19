# R5AG External-Agent Sessions Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-external-agent-sessions` -> `ontocode-external-agent-sessions`.
- Accepted `codex_external_agent_sessions` -> `ontocode_external_agent_sessions`.
- Scope was identity-only package/lib/Bazel/import rename.

## Preserved Surfaces

- External session scanning, recency limits, and title selection.
- Imported-history rollout construction, bounded tool-call/tool-result tags, and token accounting.
- Import ledger hashing/canonicalization and duplicate-import semantics.
- App-server external-agent session APIs/processors.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and folder path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-external-agent-sessions --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server external_agent_config`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_external_agent_sessions|codex-external-agent-sessions`
- `cargo metadata --format-version 1 --no-deps` residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Manager accepted R5AG.
- Active old refs are clean in `ontocode-rs`.
- Cargo metadata reports 40 remaining `codex-*` workspace packages.
- OntoIndex fallback still reports the known broad dirty-tree HIGH result rather than a scoped R5AG-only blocker.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
