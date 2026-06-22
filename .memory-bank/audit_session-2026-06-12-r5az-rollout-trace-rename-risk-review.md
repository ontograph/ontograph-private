# R5AZ Rollout Trace Rename Risk Review

Date: 2026-06-12

Decision:
- Dispatch `codex-rollout-trace` -> `ontocode-rollout-trace`.
- Dispatch `codex_rollout_trace` -> `ontocode_rollout_trace`.
- Scope is identity-only package/lib/Bazel/import/doc-comment/README rename.

OntoIndex:
- `Function:ontocode-rs/rollout-trace/src/reducer/mod.rs:replay_bundle`: CRITICAL, 44 impacted nodes, 41 direct, 7 affected modules, no affected processes.
- Risk reasons: `direct_count>=30`, `total_count>=30`, `module_count>=5`.

Direct Inventory:
- Root workspace dependency metadata.
- `rollout-trace` manifest, Bazel crate identity, README, and protocol-event doc comments.
- CLI debug trace reduce imports.
- Core client/session/compact/thread-manager/tool-dispatch imports and focused tests.
- Memories-write runtime import.
- Cargo lock entries.

Guardrails:
- Preserve trace event schema, writer behavior, inference/thread/tool-dispatch trace contexts, replay/reducer output, and reduced-state filename.
- Preserve CLI debug trace reduce behavior.
- Preserve core session/client/compact trace behavior and memories-write trace behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `rollout-trace` directory path.

Required Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rollout-trace --no-tests=pass`
- Focused CLI debug trace reduce compile/tests or `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`.
- Focused core trace/client/tool-dispatch tests or `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-write --no-tests=pass` or focused memories-write trace/runtime tests.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_rollout_trace|codex-rollout-trace`.
- Cargo metadata residual count.
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

Model:
- Dispatch on `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.
