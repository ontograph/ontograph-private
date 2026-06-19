---
name: R3B App Server Test Client Rename Closure
description: Manager acceptance record for the app-server-test-client identity-only crate rename
type: audit_session
date: 2026-06-10
status: done
---

# R3B App Server Test Client Rename Closure

## Scope Accepted

- `ontocode-app-server-test-client` renamed to `ontocode-app-server-test-client`.
- Library crate identity renamed to `ontocode_app_server_test_client`.
- Direct CLI debug/app-server references updated.
- Active package command examples updated, including `justfile`.
- App-server wire/protocol behavior, debug app-server behavior, runtime temp-dir compatibility, OTEL service-name semantics, env/config semantics, telemetry payload meaning, and persisted state were preserved.

## Verification

- Worker: `CARGO_BUILD_JOBS=8 just fmt` passed.
- Worker and manager: `CARGO_BUILD_JOBS=8 just test -p ontocode-app-server-test-client --no-tests=pass` passed.
- Worker and manager: `CARGO_BUILD_JOBS=8 just test -p codex-cli app_server` passed.
- Worker and manager: `CARGO_BUILD_JOBS=8 just test -p codex-cli debug` passed.
- Worker: `CARGO_BUILD_JOBS=8 just test -p ontocode-tui --no-tests=pass` passed.
- Worker: `CARGO_BUILD_JOBS=8 just bazel-lock-update` passed with no final `MODULE.bazel.lock` diff for this slice.
- Worker and manager: `CARGO_BUILD_JOBS=8 just bazel-lock-check` passed.
- Worker and manager: `git diff --check` passed.
- Manager metadata check confirmed package/lib identity as `ontocode-app-server-test-client` / `ontocode_app_server_test_client`.
- Manager stale-reference search found old test-client names only in intentional temp-dir and OTEL compatibility strings.
- OntoIndex `gn_verify_diff` passed for the scoped R3B file set and required tests.

## Notes

- Whole worktree remains dirty from prior accepted rename and hard-cutover slices; R3B acceptance is scoped to the app-server-test-client file set.
- Remaining R3 crates still require fresh senior review before dispatch.
