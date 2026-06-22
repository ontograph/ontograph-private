# R5BE Terminal Detection Rename Risk Review

Date: 2026-06-12

## Decision

- Approve exactly one residual slice: `codex-terminal-detection` -> `ontocode-terminal-detection`.
- Approve crate import rename: `codex_terminal_detection` -> `ontocode_terminal_detection`.
- Scope is identity-only: package metadata, library crate name, Bazel crate name, Cargo lock, and dependent imports.

## OntoIndex Impact

- Exact `terminal_info`: UNKNOWN graph risk, 0 impacted nodes.
- Exact `user_agent`: UNKNOWN graph risk, 0 impacted nodes.
- Exact `TerminalInfo`: UNKNOWN graph risk, 0 impacted nodes.
- Direct inventory is the controlling evidence for this slice.

## Direct Active References

- Root workspace dependency metadata.
- `terminal-detection` manifest and Bazel identity.
- CLI doctor and startup imports.
- Core session user-agent import.
- Login auth client user-agent import.
- MCP server test common user-agent import.
- Memories-write runtime user-agent import.
- TUI terminal info, notification, keymap, image protocol, resize/reflow, and test imports.
- Cargo lock entries.

## Guardrails

- Preserve terminal name and multiplexer detection.
- Preserve terminal info fields and defaults.
- Preserve user-agent suffix behavior.
- Preserve CLI doctor and startup behavior.
- Preserve core session user-agent behavior.
- Preserve login auth client user-agent behavior.
- Preserve MCP server test harness user-agent behavior.
- Preserve memories-write runtime user-agent behavior.
- Preserve TUI notification, image, keymap, render, and resize/reflow behavior.
- Preserve env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `terminal-detection` directory path.

## Required Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-terminal-detection --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-login --tests`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-mcp-server --tests`.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-write --no-tests=pass`.
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`.
- `CARGO_BUILD_JOBS=8 just fmt`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`.
- Scoped stale-reference search for `codex_terminal_detection|codex-terminal-detection`.
- Metadata residual count.
- `git diff --check`.
- OntoIndex `detect-changes --repo codex`.
