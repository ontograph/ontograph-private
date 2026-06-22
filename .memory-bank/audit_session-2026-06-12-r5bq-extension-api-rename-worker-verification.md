# R5BQ Extension API Rename Worker Verification

Date: 2026-06-12

## Scope

- Rename `codex-extension-api` to `ontocode-extension-api`.
- Rename `codex_extension_api` to `ontocode_extension_api`.
- Keep the existing `ext/extension-api` directory path.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name | select(startswith("codex-"))' | sort`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-extension-api --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-mcp-server --tests`
- `git diff --check`
- `/usr/bin/node /opt/demodb/_workfolder/OntoIndex/ontoindex/dist/cli/index.js detect-changes --repo codex`

## Result

- The workspace no longer contains `codex-extension-api` or `codex_extension_api` references outside this memory-bank history.
- `cargo metadata --no-deps` reports exactly four remaining `codex-*` packages: `ontocode-app-server-protocol`, `codex-protocol`, `codex-state`, and `codex-tools`.
- Build output only included pre-existing duplicate binary-target warnings and the known `TotalTokenUsageBreakdown` dead-code warning in `core`.
- Worker fallback model: `gpt-5.4-mini`.
