# R1Q Output Truncation Rename Risk Review

Date: 2026-06-10

## Decision

Approve one exact identity-only slice:

- `codex-utils-output-truncation` -> `ontocode-utils-output-truncation`

## OntoIndex Impact

- `formatted_truncate_text`: CRITICAL, 47 impacted nodes, 4 direct, 7 modules.
- `truncate_text`: CRITICAL, 42 impacted nodes, 2 direct, 9 modules.
- `truncate_function_output_items_with_policy`: HIGH, 20 impacted nodes, 4 direct, 3 modules.
- `formatted_truncate_text_content_items_with_policy`: MEDIUM, 10 impacted nodes, 6 direct, 1 module.
- `approx_tokens_from_byte_count_i64`: LOW, 3 impacted nodes, 1 direct, 1 module.

## Direct Dependents

- `codex-core`
- `codex-core-skills`
- `codex-external-agent-sessions`
- `codex-hooks`
- `codex-memories-extension`
- `codex-memories-write`
- `codex-models-manager`
- `codex-tools`

## Allowed Scope

- Cargo package rename.
- Rust library crate rename.
- Bazel crate-name metadata update.
- Workspace/dependent manifest updates.
- Rust import path updates.
- Lockfile updates.

## Non-Scope

- No truncation behavior changes.
- No token estimate or byte-budget changes.
- No omitted-item text changes.
- No image/encrypted content preservation changes.
- No hook spill behavior changes.
- No model context accounting changes.
- No protocol, public command, telemetry, env/config, rollout, session, or persisted-state changes.

## Required Verification

- `cargo metadata --format-version 1 --no-deps`
- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-utils-output-truncation`
- `CARGO_BUILD_JOBS=8 just test -p codex-core`
- `CARGO_BUILD_JOBS=8 just test -p codex-core-skills`
- `CARGO_BUILD_JOBS=8 just test -p codex-external-agent-sessions`
- `CARGO_BUILD_JOBS=8 just test -p codex-hooks`
- `CARGO_BUILD_JOBS=8 just test -p codex-memories-extension`
- `CARGO_BUILD_JOBS=8 just test -p codex-memories-write`
- `CARGO_BUILD_JOBS=8 just test -p codex-models-manager`
- `CARGO_BUILD_JOBS=8 just test -p codex-tools`
- Focused truncation/history/code-mode/tool-output/hook-spill tests where available.
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `PATH=$HOME/.local/bin:$PATH CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Stale-reference search for `codex-utils-output-truncation|codex_utils_output_truncation`.
- `git diff --check`
- Scoped OntoIndex `gn_verify_diff`.
