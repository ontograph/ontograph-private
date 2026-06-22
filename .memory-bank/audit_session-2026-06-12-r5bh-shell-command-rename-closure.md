# R5BH Shell Command Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-shell-command` -> `ontocode-shell-command` and `codex_shell_command` -> `ontocode_shell_command`.
- Scope stayed identity-only: package, library crate, Bazel, imports, and lockfiles.
- Preserved command parsing, quoting, shell extraction, safety heuristics, app-server command display/events, core exec-policy/guardian/unified-exec behavior, MCP codex-tool behavior, memories-read usage classification, TUI command rendering, CLI resume behavior, and the existing `shell-command` directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-shell-command --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-protocol --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-core shell exec_policy guardian unified_exec`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-mcp-server --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-mcp-server codex_tool`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-read --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui shell_command`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-utils-cli --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Scoped stale-reference search found no `codex_shell_command` or `codex-shell-command` refs in `ontocode-rs`.
- `cargo metadata --no-deps --format-version 1` reports 13 remaining `codex-*` packages.
- `git diff --check` is clean.
- OntoIndex `detect-changes --repo codex` reports the known broad dirty-tree high-risk state.

## Notes

- Plato `019ebcd8-62f2-7432-b600-097e950f93b9` applied the scoped patch on fallback `gpt-5.4-mini`; the manager closed the still-running worker handle and completed verification.
