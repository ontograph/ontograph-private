# R5BN Rollout Rename Worker Verification

Date: 2026-06-12

Scope:
- `codex-rollout` -> `ontocode-rollout`.
- `codex_rollout` -> `ontocode_rollout`.
- Identity-only package/lib/Bazel/import rename; preserve the existing `rollout` directory path and rollout behavior.

Fallback:
- Work completed on `gpt-5.4-mini` after Spark usage-limit fallback.

Implementation:
- Renamed the rollout package/lib/Bazel/import identity surfaces.
- Left rollout recording/list/search/state-db behavior, compression/decompression behavior, recorder append/resume semantics, persisted item filtering, metadata builder semantics, session index entries, and rollout path layout unchanged.

Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rollout --no-tests=pass` passed.
- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests` passed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-thread-store --tests` passed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests` passed.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-memories-write --tests` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- `rg -n "codex_rollout|codex-rollout" ontocode-rs --glob '!Cargo.lock'` returned no matches.
- `cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name | select(startswith("codex-"))' | sort | wc -l` returned `7`.
- `git diff --check` passed.
- `OntoIndex detect-changes --repo codex` reported the known broad high-risk dirty-tree noise.

Residual refs:
- No code or docs references remain outside the historical memory-bank records.
