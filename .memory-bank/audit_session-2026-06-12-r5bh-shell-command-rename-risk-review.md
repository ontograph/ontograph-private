# R5BH Shell Command Rename Risk Review

Date: 2026-06-12

## Scope

- Candidate: `codex-shell-command` -> `ontocode-shell-command`.
- Candidate: `codex_shell_command` -> `ontocode_shell_command`.
- Allowed change: package/lib/Bazel/import identity only.

## OntoIndex Evidence

- CLI status reports repo path `/opt/demodb/_workfolder/ontocode` and up-to-date index.
- Exact `parse_command` is ambiguous between the function and modules; treated as UNKNOWN risk.
- Exact `shlex_join` impact: CRITICAL, 33 impacted nodes, 14 direct, 8 modules, 2 affected processes.
- Exact `is_known_safe_command` impact: CRITICAL, 22 impacted nodes, 2 direct, 9 modules, no affected processes.
- Exact `extract_bash_command` impact: CRITICAL, 33 impacted nodes, 6 direct, 14 modules, 1 affected process.

## Direct Inventory

- Root workspace metadata.
- Shell-command manifest/Bazel identity.
- App-server-protocol command item imports.
- App-server bespoke event handling imports.
- Core command canonicalization, exec-policy, guardian, session, tool-runtime imports and tests.
- MCP server codex-tool imports.
- Memories-read usage imports.
- TUI command rendering and tests.
- Utils-cli resume imports.
- Cargo lock entries.

## Guardrails

- Preserve command parse AST/output shapes.
- Preserve `shlex_join` output.
- Preserve bash/zsh/PowerShell extraction behavior.
- Preserve safe-command and dangerous-command semantics.
- Preserve app-server protocol command display behavior.
- Preserve app-server bespoke command-event behavior.
- Preserve core exec-policy, guardian, shell, and unified-exec behavior.
- Preserve MCP codex-tool behavior, memories-read command usage classification, TUI command rendering, CLI resume behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `shell-command` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-shell-command --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server-protocol --tests`
- Focused app-server-protocol command item/thread-history checks if discoverable.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- Focused app-server bespoke command-event checks if discoverable.
- Focused core shell, exec-policy, guardian, and unified-exec checks.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-mcp-server --tests`
- Focused MCP codex-tool checks if discoverable.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-read --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- Focused TUI command rendering checks if discoverable.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-utils-cli --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex_shell_command|codex-shell-command`.
- Cargo metadata residual count.
- `git diff --check`
- OntoIndex `detect-changes --repo codex`
