# R5AW Connectors Rename Closure

## Scope

Accepted the identity-only residual crate rename:

- `codex-connectors` -> `ontocode-connectors`
- `codex_connectors` -> `ontocode_connectors`

## Preserved Behavior

- Connector directory cache keys, TTL, disk cache format, filtering, merge semantics, metadata normalization, install URL generation, display labels, mention slugs, plugin/app duplicate handling, TUI connector popup behavior, core session explicit app-id extraction, tool-suggest connector behavior, ChatGPT connector directory behavior, app-server plugin behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `connectors` directory path stayed unchanged.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-connectors --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-chatgpt connectors`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core connectors`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core collect_explicit_app_ids_from_skill_items`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_connectors|codex-connectors`
- Cargo metadata residual count: 24 `codex-*` packages
- `git diff --check`
- OntoIndex CLI fallback `detect-changes --repo codex`

## Notes

- Active old connectors identity refs are clean.
- OntoIndex reports the known broad high-risk dirty tree from the accumulated rename program.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
