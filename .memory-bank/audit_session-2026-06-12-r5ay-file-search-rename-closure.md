# R5AY File Search Rename Closure

Date: 2026-06-12

Scope:
- Accepted `ontocode-file-search` -> `ontocode-file-search`.
- Accepted `codex_file_search` -> `ontocode_file_search`.
- Identity-only package/lib/bin/Bazel/import/doc-comment/generated-schema/dev-helper rename; existing `file-search` directory path is preserved.

Guardrails:
- Preserved fuzzy matching, session update/completion semantics, cancellation behavior, ignore/exclude behavior, and score/order behavior.
- Preserved app-server fuzzy search API method names, payload shapes, notifications, and wire field names.
- Preserved TUI mention/search behavior and rollout list behavior.
- Preserved env/config/wire field names, telemetry/product strings, persisted state, generated schema descriptions, and helper command wiring.

Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-file-search --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just write-app-server-schema`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server fuzzy_file_search`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p codex-rollout --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_file_search|ontocode-file-search`: clean.
- Cargo metadata residual `codex-*` package count: 22.
- `git diff --check`: clean.
- OntoIndex `detect-changes --repo codex`: known broad dirty-tree high-risk report remains; no new R5AY-specific blocker found.

Notes:
- `codex-rollout` is not renamed yet, so manager verification used the current rollout package target.
- Work completed on fallback `gpt-5.4-mini` after `gpt-5.3-codex-spark` usage-limit fallback.
