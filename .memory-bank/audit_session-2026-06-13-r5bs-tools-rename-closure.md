# R5BS Tools Rename Closure

Date: 2026-06-13

## Outcome

- Accepted `codex-tools` -> `ontocode-tools`.
- Accepted `codex_tools` -> `ontocode_tools`.
- Preserved the existing `tools` directory path.
- Residual `codex-*` Cargo package identities are now exactly `ontocode-app-server-protocol` and `codex-protocol`.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-tools --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-core --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-app-server --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-mcp-server --tests`
- `CARGO_BUILD_JOBS=8 cargo check -p ontocode-goal-extension --tests`
- `CARGO_BUILD_JOBS=8 just fix -p ontocode-tools`
- `cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name | select(startswith("codex-"))' | sort`
- `rg -n "codex-tools|codex_tools" ontocode-rs .config/nextest.toml justfile`
- `git diff --check`
- OntoIndex `detect-changes --repo codex`

## Notes

- Worker Kuhn applied the scoped patch on fallback `gpt-5.4-mini`, but the agent handle did not return a final message after repeated wait windows; manager closed the stale running handle and completed acceptance verification directly.
- `git diff --check` is clean.
- The scoped stale-reference search for `codex-tools|codex_tools` in active source/config surfaces is clean.
- OntoIndex still reports the known broad dirty-tree `high` risk, so its output is advisory rather than a scoped R5BS-only verdict.
