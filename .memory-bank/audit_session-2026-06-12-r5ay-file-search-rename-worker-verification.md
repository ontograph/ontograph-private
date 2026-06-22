# R5AY File Search Rename Worker Verification

Date: 2026-06-12

Model:
- `gpt-5.4-mini` fallback after `gpt-5.3-codex-spark` usage limit.

Summary:
- Renamed `ontocode-file-search` -> `ontocode-file-search`.
- Renamed `codex_file_search` -> `ontocode_file_search`.
- Preserved fuzzy matching, session update/completion semantics, cancellation, ignore/exclude, score/order, app-server fuzzy search API shapes, TUI mention/search behavior, rollout list behavior, generated schema descriptions, and the `file-search` directory path.

Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-file-search --no-tests=pass` PASS
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol` PASS
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server fuzzy_file_search` PASS
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests` PASS
- `CARGO_BUILD_JOBS=8 cargo check -p codex-rollout --tests` PASS
- `CARGO_BUILD_JOBS=8 just fmt` PASS
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` PASS
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` PASS
- `git diff --check` PASS
- `cargo metadata --format-version 1 --no-deps` residual count: 22 `codex-*` packages
- `rg -n 'codex_file_search|ontocode-file-search' ontocode-rs justfile --glob '!target'` PASS, 0 matches
- `ontoindex detect-changes --repo codex` reported the known repository-wide high-risk dirty tree from unrelated changes

Intentional old-name refs:
- None in source for this slice.
- No `ontocode-file-search` or `codex_file_search` refs remain in the scoped source search.
