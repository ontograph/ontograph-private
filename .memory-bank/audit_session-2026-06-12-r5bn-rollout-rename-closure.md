# R5BN Rollout Rename Closure

Date: 2026-06-12

Result:
- Accepted `codex-rollout` -> `ontocode-rollout`.
- Accepted `codex_rollout` -> `ontocode_rollout`.
- Identity-only package/lib/Bazel/import rename is complete.
- Residual `codex-*` Cargo package count is 7.

Verification:
- Worker passed `CARGO_BUILD_JOBS=8 just test -p ontocode-rollout --no-tests=pass`.
- Worker passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`.
- Worker passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-thread-store --tests`.
- Worker passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`.
- Worker passed `CARGO_BUILD_JOBS=8 cargo check -p ontocode-memories-write --tests`.
- Worker passed `CARGO_BUILD_JOBS=8 just fmt`.
- Worker passed `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- Worker passed `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Manager confirmed `git diff --check` is clean.
- Manager confirmed Cargo metadata now lists 7 remaining `codex-*` packages.
- Manager confirmed active old refs are clean outside memory-bank historical tracking.
- OntoIndex `detect-changes --repo codex` reports the known broad high-risk dirty tree.

Preserved:
- Rollout file format and rollout path layout.
- Compression/decompression behavior.
- Recorder append/resume semantics.
- Persisted item filtering.
- Metadata builder semantics.
- List/search/cursor/sort behavior.
- Session index entries.
- SQLite state DB schema/normalization/telemetry.
- Thread-store/app-server/core resume/search behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `rollout` directory path.

Notes:
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
