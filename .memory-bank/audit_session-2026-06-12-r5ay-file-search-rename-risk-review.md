# R5AY File Search Rename Risk Review

Date: 2026-06-12

Decision:
- Dispatch `ontocode-file-search` -> `ontocode-file-search`.
- Dispatch `codex_file_search` -> `ontocode_file_search`.
- Scope is identity-only package/lib/bin/Bazel/import/doc-comment/generated-schema/dev-helper rename.

OntoIndex:
- `Struct:ontocode-rs/file-search/src/lib.rs:FileMatch`: LOW, 1 impacted node, no affected processes.
- `Struct:ontocode-rs/file-search/src/lib.rs:FileSearchSession`: LOW, 6 impacted nodes, no affected processes, 2 affected modules.
- `Function:ontocode-rs/file-search/src/lib.rs:create_session`: LOW, 6 impacted nodes, 1 affected process through app-server initialized client request handling.
- `Function:ontocode-rs/file-search/src/lib.rs:run`: LOW, 1 impacted node, no affected processes.
- Generic `FileMatch` and `file_search` names are ambiguous in OntoIndex; direct inventory is the source of truth for non-exact refs.

Direct Inventory:
- Root workspace dependency metadata.
- `file-search` manifest, binary, library, Bazel crate identity, README, and main imports.
- `app-server-protocol` doc comments and generated schema descriptions for `FuzzyFileSearchResult`.
- `app-server` fuzzy search processor imports.
- `rollout` list search imports.
- `tui` file-search, mentions, popup, app-event, and chatwidget imports.
- Root `justfile` helper command for running the file-search binary.

Guardrails:
- Preserve fuzzy matching, session update/completion semantics, cancellation behavior, ignore/exclude behavior, and score/order behavior.
- Preserve app-server fuzzy search API method names, payload shapes, notification names, and wire field names.
- Preserve TUI mention/search behavior and rollout list behavior.
- Preserve env/config/wire field names, telemetry/product strings, persisted state, and the existing `file-search` directory path.

Required Verification:
- `CARGO_BUILD_JOBS=8 just test -p ontocode-file-search --no-tests=pass`
- App-server protocol schema regeneration/tests if doc comments change: `just write-app-server-schema` and `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-protocol`
- Focused app-server fuzzy-file-search tests or compile-only app-server checks if no focused test exists.
- TUI compile/focused file-search or mention checks.
- Rollout compile/list checks.
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active-source stale-reference search for `codex_file_search|ontocode-file-search`.
- Cargo metadata residual count.
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

Model:
- Dispatch on `gpt-5.4-mini` because `gpt-5.3-codex-spark` is unavailable or usage-limited.
