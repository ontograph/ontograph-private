# R1M Home Dir Internal Crate Rename Risk Review

Date: 2026-06-10

Status: approved for one exact identity-only worker slice.

Decision:
- Approve `codex-utils-home-dir` -> `ontocode-utils-home-dir`.
- Scope is package name, crate/lib name, Bazel crate name, Rust imports, workspace dependency refs, lockfiles, and active direct references only.
- Do not rename `find_codex_home`, `ONTOCODE_HOME`, `CODEX_HOME`, home-directory precedence, fallback behavior, state/config paths, runtime package layout, public command names, telemetry, or persisted data.

Risk evidence:
- Direct Cargo dependents: `ontocode-app-server-daemon`, `codex-arg0`, `codex-core`, `codex-install-context`, `codex-network-proxy`, `codex-rmcp-client`, and `ontocode-tui`.
- OntoIndex `find_codex_home`: LOW, 0 impacted nodes due weak import graph coverage.
- OntoIndex `find_codex_home_with_overrides`: LOW, 10 impacted nodes, mostly crate-local tests.
- OntoIndex `resolve_home_override`: LOW, 11 impacted nodes, mostly crate-local tests.
- Direct text inventory is authoritative because the crate is an env/state boundary not fully represented by the graph.

Required verification:
- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-home-dir`
- `CARGO_BUILD_JOBS=8 just test -p codex-arg0`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-daemon`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p codex-install-context`
- `CARGO_BUILD_JOBS=8 just test -p codex-network-proxy`
- `CARGO_BUILD_JOBS=8 just test -p codex-rmcp-client`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui`
- focused home/env tests if package tests expose narrower targets.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- stale-reference search for `codex-utils-home-dir|codex_utils_home_dir` under active `ontocode-rs` sources.
- `git diff --check`
- OntoIndex `gn_verify_diff` covering changed files and executed tests.
