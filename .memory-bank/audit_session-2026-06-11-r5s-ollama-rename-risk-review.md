# R5S Ollama Rename Risk Review

Date: 2026-06-11

## Decision

- Approved next residual slice: `codex-ollama` -> `ontocode-ollama`.
- Approved crate import rename: `codex_ollama` -> `ontocode_ollama`.
- Scope is identity-only package/lib/Bazel/import rename.

## Inventory

- Cargo metadata reports 55 remaining `codex-*` workspace packages before this slice.
- Direct reverse dependency: `ontocode-utils-oss`.
- Active refs: 9 refs across root workspace metadata, the Ollama manifest/Bazel identity, and OSS utility dependency/import/test usage.

## OntoIndex

- `Const:ontocode-rs/ollama/src/lib.rs:DEFAULT_OSS_MODEL`: LOW impact.
- `Function:ontocode-rs/ollama/src/lib.rs:ensure_oss_ready`: LOW impact.
- `Function:ontocode-rs/ollama/src/lib.rs:ensure_responses_supported`: CRITICAL impact.
- CRITICAL blast radius reaches `ensure_oss_provider_ready`, TUI `run_main`, exec `run_main`, CLI interactive TUI, and MCP server paths.

## Guardrails

- Preserve Ollama provider IDs.
- Preserve default model value.
- Preserve `ollama` command/process behavior.
- Preserve responses-version gate semantics.
- Preserve model loading/readiness behavior.
- Preserve OSS provider selection behavior.
- Preserve TUI/exec `--oss` startup behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `ollama` directory path.

## Verification Required

- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-ollama --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-oss --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-exec --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Active-source stale-reference classification for `codex_ollama|codex-ollama`.
- `git diff --check`.
- OntoIndex CLI fallback `detect-changes --repo codex`.
