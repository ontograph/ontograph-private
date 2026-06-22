# R5BE Terminal Detection Rename Worker Verification

Date: 2026-06-12

## Summary

- Renamed `codex-terminal-detection` to `ontocode-terminal-detection` and `codex_terminal_detection` to `ontocode_terminal_detection`.
- Preserved terminal detection, multiplexer detection, terminal info fields/defaults, user-agent suffix behavior, CLI doctor/startup behavior, core session user-agent behavior, login auth client user-agent behavior, MCP server test harness user-agent behavior, memories-write runtime user-agent behavior, TUI notification/image/keymap/render/resize-reflow behavior, env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `terminal-detection` directory path.
- Work completed on fallback `gpt-5.4-mini` after Spark usage-limit fallback.

## Verification

- `CARGO_BUILD_JOBS=8 just test -p ontocode-terminal-detection --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-cli --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-login --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-mcp-server --tests`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-memories-write --no-tests=pass`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tui --tests`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale search for `codex_terminal_detection|codex-terminal-detection` returned no matches.
- `cargo metadata --format-version 1 --no-deps` residual count: `16`.
- `git diff --check` passed.
- `OntoIndex detect-changes --repo codex` reported the known broad high-risk dirty tree.
