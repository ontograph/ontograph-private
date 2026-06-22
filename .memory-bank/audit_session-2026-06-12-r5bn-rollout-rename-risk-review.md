# R5BN Rollout Rename Risk Review

Date: 2026-06-12

Scope:
- `codex-rollout` -> `ontocode-rollout`.
- `codex_rollout` -> `ontocode_rollout`.
- Identity-only package/lib/Bazel/import rename; preserve the existing `rollout` directory path.

OntoIndex:
- Repo path verified through the OntoIndex CLI as `/opt/demodb/_workfolder/ontocode`.
- `RolloutConfigView`: CRITICAL, 17 impacted nodes, 1 direct, 6 modules, affected `search_threads`.
- `normalize_cwd_for_state_db`: CRITICAL, 19 impacted nodes, 5 direct, 7 modules.
- `spawn_rollout_compression_worker`: HIGH, 17 impacted nodes, 1 direct, 4 modules, affected multi-agent test and thread-manager sample processes.
- `plain_rollout_path`: HIGH, 20 impacted nodes, 5 direct, 4 modules, affected `search_threads`.
- `SessionIndexEntry`: LOW, 5 impacted nodes, 2 direct, 2 modules.
- `is_persisted_rollout_item`: LOW, 11 impacted nodes, 2 direct, 2 modules, affected app-server thread lifecycle resume.
- `should_persist_response_item`: LOW, 8 impacted nodes, 1 direct, 2 modules, affected app-server thread lifecycle resume.
- `RolloutRecorder`, `RolloutRecorderParams`, `RolloutConfig`, `ThreadItem`, and `ThreadsPage`: UNKNOWN due ambiguous symbol matches.

Guardrails:
- Do not change rollout file format, rollout path layout, compression/decompression behavior, recorder append/resume semantics, persisted item filtering, metadata builder semantics, list/search/cursor/sort behavior, session index entries, SQLite state DB schema/normalization/telemetry, thread-store/app-server/core resume/search behavior, env/config/wire/generated names, telemetry/product strings, persisted state, or folder path.

Verification required:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rollout --no-tests=pass`.
- Focused compression/recorder/list/search/metadata/session-index/state-db checks.
- Focused core resume/session/multi-agent rollout checks or `cargo check -p ontocode-core --tests`.
- Thread-store search/resume compile or focused checks.
- App-server thread lifecycle/read/search compile or focused checks.
- Memories-write rollout-summary checks if directly affected.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Stale-reference search for `codex-rollout|codex_rollout`.
- Cargo metadata residual count, expected 7 remaining `codex-*` packages after success.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`; known broad dirty-tree risk may remain outside this scoped rename.
