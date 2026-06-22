# R5BG Feedback Rename Risk Review

Date: 2026-06-12

## Scope

- Candidate: `codex-feedback` -> `ontocode-feedback`.
- Candidate: `codex_feedback` -> `ontocode_feedback`.
- Allowed change: package/lib/Bazel/import identity only.

## OntoIndex Evidence

- CLI status reports repo path `/opt/demodb/_workfolder/ontocode` and up-to-date index.
- MCP repo alias lookup failed for `codex` and exposed `OntoIndex`; local CLI was used because it reports the required repo path.
- Exact `FeedbackRequestTags` impact: CRITICAL, 10 impacted nodes, 7 direct, 6 modules, no affected processes.
- Exact `emit_feedback_request_tags_with_auth_env` impact: CRITICAL, 9 impacted nodes, 6 direct, 6 modules, no affected processes.
- `CodexFeedback` and `FeedbackDiagnostics` are ambiguous without UID disambiguation and therefore treated as UNKNOWN risk.

## Direct Inventory

- Root workspace metadata.
- Feedback manifest/Bazel identity.
- App-server-client feedback imports.
- App-server feedback imports, processors, and tests.
- Core feedback telemetry imports and tests.
- Exec feedback imports.
- Model-provider feedback telemetry imports.
- TUI feedback state, view, and tests.
- Cargo lock entries.
- Intentional compatibility strings for `codex-feedback-*` log filename stems and internal feedback URLs.

## Guardrails

- Preserve feedback request tag fields and auth-env tag emission.
- Preserve tracing metadata layer behavior.
- Preserve upload options, attachment paths, and doctor/windows-sandbox/diagnostics attachment filenames.
- Preserve TUI feedback UI copy and internal feedback URLs.
- Preserve app-server feedback processors.
- Preserve core/model-provider request telemetry tags.
- Preserve exec and app-server-client feedback wiring.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, intentional `codex-feedback-*` feedback log filename stems, and the existing `feedback` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-feedback --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- Focused app-server feedback tests if available.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-client --tests`
- Focused core feedback telemetry tests.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-exec --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-model-provider --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- Focused TUI feedback view tests if available.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search with intentional compatibility classification.
- Cargo metadata residual count.
- `git diff --check`
- OntoIndex `detect-changes --repo codex`
