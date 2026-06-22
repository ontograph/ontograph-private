---
name: R3B App Server Test Client Rename Risk Review
description: Senior unblock decision for the second CLI/app crate rename slice
type: audit_session
date: 2026-06-10
status: approved
---

# R3B App Server Test Client Rename Risk Review

## Decision

Approve only `ontocode-app-server-test-client` -> `ontocode-app-server-test-client` as the next R3 implementation slice.

## Scope

- Rename package/lib/Bazel/import identity for `ontocode-rs/app-server-test-client`.
- Update direct CLI references needed for compile/tests.
- Keep the slice separate from `ontocode-app-server-transport`, `ontocode-app-server-client`, `ontocode-app-server`, `codex-cli`, and `ontocode-tui`.

## Evidence

- Reverse dependency inventory: direct reach is through `codex-cli`, which is then used by `ontocode-tui`.
- Public API inventory: only `run` and `send_message_v2` are public async functions.
- OntoIndex impact: `run` is LOW with zero upstream impacted nodes.
- OntoIndex impact: `send_message_v2` is LOW and reaches `run_debug_app_server_command` -> `cli_main` -> `main`.
- Remaining app-server transport/client/server and CLI/TUI candidates have broader runtime dependency reach and stay gated.

## Non-Goals

- Do not rename app-server wire methods.
- Do not rename generated protocol model names.
- Do not change debug app-server command behavior.
- Do not change test-client runtime behavior.
- Do not change runtime temp directory compatibility unless an existing package-name test proves it must move.
- Do not change OTEL service-name semantics unless an existing package-name test proves it must move.
- Do not change env/config semantics.
- Do not change telemetry payload meaning.
- Do not change persisted state.

## Required Verification

- `CARGO_BUILD_JOBS=8 just fmt`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-test-client --no-tests=pass`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server`
- `CARGO_BUILD_JOBS=8 just test -p codex-cli debug`
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass` or a narrower TUI compile/test target if available
- `CARGO_BUILD_JOBS=8 just bazel-lock-update`
- `CARGO_BUILD_JOBS=8 just bazel-lock-check`
- Active stale-reference search for `ontocode-app-server-test-client` and `codex_app_server_test_client`, with intentional compatibility strings documented.
- `git diff --check`
- OntoIndex `gn_verify_diff` or `ontoindex detect-changes` scoped to the R3B slice.

## Next Gate

After R3B is accepted, run a fresh senior review before approving transport, client, server, CLI, or TUI rename work.
