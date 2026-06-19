# R5BG Feedback Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-feedback` -> `ontocode-feedback`.
- Accepted `codex_feedback` -> `ontocode_feedback`.
- Scope remained package/lib/Bazel/import identity only.

## Risk

- OntoIndex exact `FeedbackRequestTags` impact was CRITICAL: 10 impacted nodes, 7 direct, 6 modules, no affected processes.
- OntoIndex exact `emit_feedback_request_tags_with_auth_env` impact was CRITICAL: 9 impacted nodes, 6 direct, 6 modules, no affected processes.
- `CodexFeedback` and `FeedbackDiagnostics` were ambiguous/UNKNOWN without UID disambiguation.
- Scope was accepted only because feedback upload, diagnostics, attachments, telemetry tags, tracing metadata, app-server processors, TUI feedback view/copy, and log filename behavior stayed unchanged.

## Manager Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-feedback --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server feedback`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-client --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core feedback`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-exec --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-model-provider --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui feedback`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Scoped stale-reference search for `codex_feedback|codex-feedback`
- Cargo metadata residual package count
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

## Result

- Noether applied the scoped patch, but the worker handle was closed while still running verification.
- Manager completed verification and accepted the slice.
- Active old crate refs are clean.
- Remaining `codex-feedback` refs are intentional compatibility strings for feedback log filename stems and internal feedback URLs.
- `git diff --check` is clean.
- Cargo metadata reports 14 remaining `codex-*` packages.
- OntoIndex `detect-changes --repo codex` reports the known broad high-risk dirty tree.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.
