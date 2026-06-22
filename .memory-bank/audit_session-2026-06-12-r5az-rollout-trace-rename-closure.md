# R5AZ Rollout Trace Rename Closure

Date: 2026-06-12

Scope:
- Accepted `codex-rollout-trace` -> `ontocode-rollout-trace`.
- Accepted `codex_rollout_trace` -> `ontocode_rollout_trace`.
- Identity-only package/lib/Bazel/import/doc-comment/README rename; existing `rollout-trace` directory path is preserved.

Guardrails:
- Preserved trace event schema, writer behavior, inference/thread/tool-dispatch trace contexts, replay/reducer output, and reduced-state filename.
- Preserved CLI debug trace reduce behavior.
- Preserved core session/client/compact trace behavior and memories-write trace behavior.
- Preserved env/config/wire/generated names, telemetry/product strings, and persisted state.

Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-rollout-trace --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-write --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_rollout_trace|codex-rollout-trace`: clean.
- Cargo metadata residual `codex-*` package count: 21.
- `git diff --check`: clean.
- OntoIndex `detect-changes --repo codex`: known broad dirty-tree high-risk report remains; no new R5AZ-specific blocker found.

Notes:
- OntoIndex reported CRITICAL impact for `replay_bundle`; this slice was accepted only because changes stayed within crate/package identity and imports.
- Work completed on fallback `gpt-5.4-mini` after `gpt-5.3-codex-spark` usage-limit fallback.
