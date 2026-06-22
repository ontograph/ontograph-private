---
name: R3B App Server Test Client Rename Worker Verification
description: Worker implementation and verification for the app-server test-client identity-only crate rename
type: audit_session
date: 2026-06-10
status: review
---

# R3B App Server Test Client Rename Worker Verification

## Summary

- Implemented `ontocode-app-server-test-client` -> `ontocode-app-server-test-client`.
- Updated package/lib/Bazel/import identity, direct CLI dependency/call site, README package examples, `justfile` package helper, and `Cargo.lock`.
- Preserved app-server wire/protocol behavior, debug app-server command behavior, runtime temp directory compatibility, OTEL service-name meaning, env/config semantics, telemetry semantics, and persisted state.
- Kept intentional old-name compatibility strings only for `/tmp/ontocode-app-server-test-client` and `OTEL_SERVICE_NAME`.

## OntoIndex

- Pre-edit impact: `run` LOW, zero upstream impacted nodes.
- Pre-edit impact: `send_message_v2` LOW, reaches `run_debug_app_server_command` -> `cli_main` -> `main`.
- Pre-edit impact: app-server-test-client `main` LOW, zero upstream impacted nodes.
- Pre-edit impact: `run_debug_app_server_command` LOW, reaches `cli_main` -> `main`.
- Pre-edit impact: `live_elicitation_timeout_pause` LOW, reaches `run`.
- Pre-edit impact: `CodexClient.connect_websocket` LOW, reaches app-server-test-client client flows.
- Scoped `gn_verify_diff` passed for the R3B file/symbol/test set.

## Verification

- `CARGO_BUILD_JOBS=8 just fmt` passed.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-test-client --no-tests=pass` passed: zero package tests, bench smoke passed.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server` passed: 25 passed, 236 skipped.
- `CARGO_BUILD_JOBS=8 just test -p codex-cli debug` passed: 13 passed, 248 skipped.
- `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass` passed: 2772 passed, 4 skipped.
- `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed with existing rules_rs crate-annotation warnings.
- `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- Stale reference search for `ontocode-app-server-test-client|codex_app_server_test_client` found only intentional temp-dir and OTEL compatibility strings.
- `git diff --check` passed.
