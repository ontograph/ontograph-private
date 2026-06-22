# R5AZ Rollout Trace Rename Worker Verification

Date: 2026-06-12

Model:
- `gpt-5.4-mini` fallback after `gpt-5.3-codex-spark` usage limit.

Summary:
- Renamed `codex-rollout-trace` -> `ontocode-rollout-trace`.
- Renamed `codex_rollout_trace` -> `ontocode_rollout_trace`.
- Preserved trace event schema, writer behavior, inference/thread/tool-dispatch trace contexts, replay/reducer output, reduced-state filename, CLI debug trace reduce behavior, core session/client/compact trace behavior, memories-write trace behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `rollout-trace` directory path.

Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rollout-trace --no-tests=pass` PASS
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests` PASS
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests` PASS
- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-write --no-tests=pass` PASS
- `CARGO_BUILD_JOBS=8 just fmt` PASS
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` PASS
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` PASS
- `rg -n 'codex_rollout_trace|codex-rollout-trace' ontocode-rs --glob '!target'` PASS, 0 source matches
- `cargo metadata --format-version 1 --no-deps` residual count: 21 `codex-*` packages
- `git diff --check` PASS
- `ontoindex detect-changes --repo codex` reported the known repository-wide high-risk dirty tree from unrelated changes

Intentional old-name refs:
- None in source for this slice.
