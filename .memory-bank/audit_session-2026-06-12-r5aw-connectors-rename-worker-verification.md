# R5AW Connectors Rename Worker Verification

## Summary

- Identity-only rename completed for `codex-connectors` -> `ontocode-connectors` and `codex_connectors` -> `ontocode_connectors`.
- Preserved connector cache keys, TTL, disk cache format, filtering, merge semantics, metadata normalization, install URL generation, display labels, mention slugs, duplicate handling, popup behavior, explicit app-id extraction, ChatGPT connector directory behavior, and app-server plugin behavior.
- Runtime model used for this worker run: `gpt-5.4-mini` after Spark usage-limit fallback.

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
- `cargo metadata --format-version 1 --no-deps` residual count: 24 `codex-*` packages
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

## Result

- Source refs to `codex_connectors` and `codex-connectors` are clean outside lock/history entries that now carry the new identity.
- OntoIndex detect-changes still reports the known broad high-risk dirty tree outside this slice.
