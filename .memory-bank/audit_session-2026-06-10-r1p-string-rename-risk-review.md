# R1P String Internal Crate Rename Risk Review

Date: 2026-06-10

Status: approved for one exact identity-only worker slice.

Decision:
- Approve `codex-utils-string` -> `ontocode-utils-string`.
- Scope is package name, crate/lib name, Bazel crate name, Rust imports, workspace dependency refs, lockfiles, and active direct references only.
- Do not rename or change telemetry sanitization, UTF-8 boundary handling, markdown hash parsing, UUID parsing, token estimate/truncation behavior, protocol shape, tool-output semantics, public command names, telemetry keys, or persisted data.

Risk evidence:
- Direct Cargo dependents: `codex-context-fragments`, `codex-core`, `codex-otel`, `codex-protocol`, `codex-tools`, `ontocode-tui`, `codex-utils-output-truncation`, and `ontocode-windows-sandbox`.
- OntoIndex `sanitize_metric_tag_value`: CRITICAL, 23 impacted nodes, 7 direct callers, 9 modules; affected telemetry/metrics/windows-sandbox setup flows.
- OntoIndex `take_bytes_at_char_boundary`: HIGH, 20 impacted nodes, 3 direct callers, 3 affected Windows sandbox/tool-output processes.
- OntoIndex `normalize_markdown_hash_location_suffix`: LOW, TUI markdown link target path.
- OntoIndex `find_uuids`: LOW in graph, but crate-local tests remain required.

Required verification:
- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-string`
- `CARGO_BUILD_JOBS=8 just test -p codex-context-fragments`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p codex-otel`
- `CARGO_BUILD_JOBS=8 just test -p codex-protocol`
- `CARGO_BUILD_JOBS=8 just test -p codex-tools`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-windows-sandbox`
- `CARGO_BUILD_JOBS=8 just test -p codex-utils-output-truncation` or renamed equivalent if this slice changes the dependent manifest.
- focused telemetry/string/tool-output/windows-sandbox logging tests if available.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- stale-reference search for `codex-utils-string|codex_utils_string` under active `ontocode-rs` sources.
- `git diff --check`
- OntoIndex `gn_verify_diff` covering changed files and executed tests.
