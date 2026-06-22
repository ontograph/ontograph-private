# R5N Message History Rename Closure

Date: 2026-06-11

## Result

- Accepted `codex-message-history` -> `ontocode-message-history`.
- Accepted `codex_message_history` -> `ontocode_message_history`.
- Preserved `history.jsonl` format, append/lookup/history metadata behavior, trimming/retry/owner-only permission behavior, `HistoryConfig::new`, `codex_home` semantics, TUI thread-routing/session behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `message-history` directory path.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-message-history --no-tests=pass`: passed, 4 tests.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`: passed, 2772 tests and 4 skipped.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active-source stale-reference search for `codex_message_history|codex-message-history` in `ontocode-rs`: zero matches.
- `git diff --check`: passed.
- Cargo metadata reports 59 remaining `codex-*` workspace packages.
- OntoIndex CLI fallback `detect-changes --repo codex`: completed with the known broad dirty-tree high-risk context, not an R5N-specific blocker.

## Model Fallback

- Initial worker on `gpt-5.3-codex-spark` hit the usage limit before completing work.
- Fallback worker on `gpt-5.4-mini` hit capacity before completing work.
- Verification completed on `gpt-5.4`.
