# R5W Image Generation Extension Rename Closure

Date: 2026-06-11

## Scope

- Accepted `codex-image-generation-extension` -> `ontocode-image-generation-extension`.
- Accepted `codex_image_generation_extension` -> `ontocode_image_generation_extension`.
- Kept the change identity-only: package, lib crate, Bazel crate identity, dependency/import wiring.

## Guardrails Preserved

- Image-generation tool namespace and tool names.
- Image request/response behavior.
- Model/provider selection behavior.
- Auth/provider behavior.
- Markdown description compile data.
- Tool schema behavior.
- Metrics behavior.
- App-server extension registration behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and `ext/image-generation` path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-image-generation-extension --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_image_generation_extension|codex-image-generation-extension`
- Cargo metadata residual count
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Result

- Focused verification passed.
- App-server suite passed: 810 passed, 1 skipped.
- Active old refs are clean in `ontocode-rs`.
- `git diff --check` is clean.
- Cargo metadata reports 50 remaining `codex-*` workspace packages.
- OntoIndex detect still reports the known broad dirty-tree high-risk context, not a scoped R5W blocker.

## Runtime Model

- Worker: Sartre `019eb82d-bf9a-7503-8a77-4f89ba0def79`.
- Model: `gpt-5.4-mini`, `high` reasoning, after Spark usage-limit fallback.
