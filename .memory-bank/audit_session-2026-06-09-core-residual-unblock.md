# Core Residual Unblock

Date: 2026-06-09

## Scope

- Closed the residual `codex-core` blockers left after the sandbox-helper/runtime unblock.
- Updated `ONTOCODE_INTERNAL_CRATE_RENAME_TRACKING.md`, `project_pending-tasks.md`, and `project_plan-current.md` to mark R1B-U1 done and R1 ready for the next exact manager-approved slice.

## Fixes

- Updated code-mode tests for current tool exposure: `exec`, `wait`, `web_search`, and deferred app image generation where applicable.
- Replaced model-visible 50k/90k raw-output assertions with bounded JSON metadata assertions while preserving runtime output-length/truncation validation.
- Updated the hidden dynamic tool test for namespaced `codex_app__hidden_dynamic_tool` wiring and a matching code-mode model fixture.
- Isolated the shell snapshot stdin probe from inherited `BASH_ENV`.
- Added test-local tempdir helpers for config/git/realtime tests so parent `/tmp` or workspace `.git`/`.codex` markers do not contaminate “outside repo” expectations.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt` passed.
- Focused code-mode and shell residual rerun passed: 10 tests passed.
- Focused config/git/realtime contamination rerun passed: 5 tests passed.
- Focused WebRTC sideband race rerun passed.
- Full `CARGO_BUILD_JOBS=8 just test -p codex-core` passed: 2648 passed, 1 flaky retry, 14 skipped.

## Remaining Gate

- R1 is no longer blocked by dependent core failures.
- Next implementation must still be an exact manager-approved slice with fresh OntoIndex impact; prior HIGH/CRITICAL notes on stream/parser and json-to-toml caller surfaces still require explicit acknowledgement before edits.
