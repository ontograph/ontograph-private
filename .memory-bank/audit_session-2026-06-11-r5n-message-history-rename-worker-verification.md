# R5N Message History Rename Worker Verification

Date: 2026-06-11

## Scope

- Verified Cargo package rename `codex-message-history` -> `ontocode-message-history`.
- Verified Rust lib crate rename `codex_message_history` -> `ontocode_message_history`.
- Verified scoped workspace metadata, Bazel crate identity, TUI dependency/import usage, and Cargo lock metadata for the message-history slice.

## Guardrails

- `history.jsonl` format was not changed.
- Append, lookup, history metadata, trimming, retry, and owner-only permission behavior were preserved.
- `HistoryConfig::new`, `codex_home` semantics, TUI thread-routing/session behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `message-history` directory path were preserved.

## Verification

- Pre-edit OntoIndex evidence from the risk review remained LOW for `HistoryConfig`, `append_entry`, and `lookup`, with repoPath `/opt/demodb/_workfolder/ontocode`.
- `CARGO_BUILD_JOBS=8 just fmt`: passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-message-history --no-tests=pass`: passed; 4 tests, bench-smoke completed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`: passed; build/test target completed, bench-smoke completed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`: passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`: passed.
- Active `ontocode-rs` source stale-reference search for `codex_message_history|codex-message-history`: zero matches.
- `git diff --check`: passed.
- OntoIndex CLI fallback `detect-changes --repo codex`: completed; reported broad dirty-tree context of 200 files, 320 symbols, 8 affected processes, high risk.
- Cargo metadata reports 59 remaining `codex-*` workspace packages after R5N.

## Notes

- The scoped production rename was already present in the dirty worktree when this `gpt-5.4` worker started; this run validated it, preserved unrelated edits, and added tracking only.
- Prior fallback workers closed before implementation: `gpt-5.3-codex-spark` hit the usage limit, then `gpt-5.4-mini` hit capacity; verification completed on `gpt-5.4`.
- Test output repeated known unrelated Windows sandbox duplicate-bin warnings.
- Bazel lock update repeated existing `rules_rs` crate-annotation warnings for `libssh2-sys`.
