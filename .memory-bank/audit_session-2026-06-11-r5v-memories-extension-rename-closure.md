# R5V Memories Extension Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-memories-extension` -> `ontocode-memories-extension`.
- Accepted `codex_memories_extension` -> `ontocode_memories_extension`.
- Kept the change identity-only: package, lib crate, Bazel crate identity, dependency/import wiring.

## Guardrails Preserved

- Memory tool namespace and tool names.
- Add/list/read/search behavior.
- Local memories backend behavior.
- Prompt/template content and metrics behavior.
- App-server extension registration behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and `ext/memories` path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_memories_extension|codex-memories-extension`
- Cargo metadata residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Focused verification passed.
- Active old refs are clean in `ontocode-rs`.
- `git diff --check` is clean.
- Cargo metadata reports 51 remaining `codex-*` workspace packages.
- OntoIndex detect still reports the known broad dirty-tree high-risk context, not a scoped R5V blocker.

## Runtime Model

- Worker: Boole `019eb815-091b-73a1-90d0-164ec7c446c4`.
- Model: `gpt-5.4-mini`, `high` reasoning, after Spark usage-limit fallback.
