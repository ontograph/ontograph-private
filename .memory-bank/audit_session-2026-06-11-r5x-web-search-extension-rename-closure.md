# R5X Web Search Extension Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-web-search-extension` -> `ontocode-web-search-extension`.
- Accepted `codex_web_search_extension` -> `ontocode_web_search_extension`.
- Kept the change identity-only: package, lib crate, Bazel crate identity, dependency/import wiring.

## Guardrails Preserved

- Web-search tool namespace and tool names.
- `web_run` behavior.
- Search settings/config behavior.
- Model/provider capability behavior.
- Auth/provider behavior.
- Encrypted output behavior.
- Markdown description compile data.
- Tool schema behavior.
- Metrics behavior.
- App-server extension registration behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and `ext/web-search` path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-web-search-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_web_search_extension|codex-web-search-extension`
- Cargo metadata residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Focused verification passed.
- App-server suite passed: 810 passed, 1 skipped.
- Active old refs are clean in `ontocode-rs`.
- `git diff --check` is clean.
- Cargo metadata reports 49 remaining `codex-*` workspace packages.
- OntoIndex detect still reports the known broad dirty-tree high-risk context, not a scoped R5X blocker.

## Runtime Model

- Worker: Popper `019eb849-b110-7051-b049-9e61d4ad3aee`.
- Model: `gpt-5.4-mini`, `high` reasoning, after Spark usage-limit fallback.
