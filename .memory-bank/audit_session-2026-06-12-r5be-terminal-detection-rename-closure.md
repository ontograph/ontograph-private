# R5BE Terminal Detection Rename Closure

Date: 2026-06-12

## Scope

- Accepted `codex-terminal-detection` -> `ontocode-terminal-detection`.
- Accepted `codex_terminal_detection` -> `ontocode_terminal_detection`.
- Scope remained identity-only: package, library, Bazel target, Cargo lock, and dependent imports.

## Guardrails Preserved

- Terminal name and multiplexer detection.
- Terminal info fields and defaults.
- User-agent suffix behavior.
- CLI doctor and startup behavior.
- Core session user-agent behavior.
- Login auth client user-agent behavior.
- MCP server test harness user-agent behavior.
- Memories-write runtime user-agent behavior.
- TUI notification, image, keymap, render, and resize/reflow behavior.
- Env/config/wire/generated names, telemetry/product strings, persisted state, and the existing `terminal-detection` directory path.

## Verification

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
- Scoped stale-reference search for `codex_terminal_detection|codex-terminal-detection`: clean in `ontocode-rs`.
- Cargo metadata residual count: 16 `codex-*` packages.
- `git diff --check`: clean.
- OntoIndex `detect-changes --repo codex`: completed with the known broad high-risk dirty-tree envelope.

## Result

- R5BE accepted.
- Work completed on `gpt-5.4-mini` after Spark usage-limit fallback.
