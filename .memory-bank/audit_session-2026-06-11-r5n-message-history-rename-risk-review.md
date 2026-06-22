# R5N Message History Rename Risk Review

Date: 2026-06-11

## Scope

- Rename `codex-message-history` to `ontocode-message-history`.
- Rename crate import identity `codex_message_history` to `ontocode_message_history`.
- Allowed files are limited to workspace metadata, `message-history` manifest/Bazel crate identity, TUI imports/usages, lockfiles, and factual tracking updates.
- Preserve the existing `message-history` directory path.

## Direct Inventory

- Active direct refs: 14.
- Direct reverse dependency: `ontocode-tui`.
- Ref homes: root workspace dependency metadata, `message-history/Cargo.toml`, `message-history/BUILD.bazel`, `tui/Cargo.toml`, `tui/src/app/thread_routing.rs`, and `tui/src/app_server_session.rs`.

## OntoIndex Evidence

- `HistoryConfig`: LOW, 0 impacted nodes, 0 affected modules, 0 affected processes.
- `append_entry`: LOW, 4 direct impacted callers/tests, 2 affected modules, 0 affected processes.
- `lookup`: LOW, 1 direct impacted TUI caller, 1 affected module, 0 affected processes.
- CLI fallback was used because the OntoIndex MCP facade has known repo/tool availability issues; all reported graph snapshots point to `/opt/demodb/_workfolder/ontocode`.

## Guardrails

- Do not change `history.jsonl` format.
- Do not change append, lookup, metadata, trimming, retry, or owner-only permission behavior.
- Do not rename `HistoryConfig::new` arguments or `codex_home` semantics.
- Do not change TUI thread routing, app-server session metadata behavior, env/config/wire/generated names, telemetry/product strings, or persisted state.
- Do not rename public command names, runtime package layout, protocol/generated crates, or historical ADR text.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-message-history --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_message_history|codex-message-history`
- `git diff --check`
- OntoIndex CLI fallback: `detect-changes --repo codex`
